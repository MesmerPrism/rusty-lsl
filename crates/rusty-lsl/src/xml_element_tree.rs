// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{XmlElementName, XmlLeafElement};
use core::fmt;

/// Identifies one configured XML element-tree resource bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum XmlElementTreeBound {
    /// Maximum number of nodes in the caller-owned arena.
    Nodes,
    /// Maximum parent-chain depth, where the root has depth one.
    Depth,
    /// Maximum direct child count for any container.
    ChildrenPerContainer,
    /// Maximum retained UTF-8 bytes across accepted component values.
    RetainedBytes,
}

/// Explicit nonzero resource maxima for one XML container/leaf hierarchy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XmlElementTreeLimits {
    max_nodes: usize,
    max_depth: usize,
    max_children_per_container: usize,
    max_retained_bytes: usize,
}

impl XmlElementTreeLimits {
    /// Validates all maxima in argument order.
    pub fn new(
        max_nodes: usize,
        max_depth: usize,
        max_children_per_container: usize,
        max_retained_bytes: usize,
    ) -> Result<Self, XmlElementTreeError> {
        for (bound, actual) in [
            (XmlElementTreeBound::Nodes, max_nodes),
            (XmlElementTreeBound::Depth, max_depth),
            (
                XmlElementTreeBound::ChildrenPerContainer,
                max_children_per_container,
            ),
            (XmlElementTreeBound::RetainedBytes, max_retained_bytes),
        ] {
            if actual == 0 {
                return Err(XmlElementTreeError::InvalidLimit {
                    bound,
                    expected_min: 1,
                    actual,
                });
            }
        }
        Ok(Self {
            max_nodes,
            max_depth,
            max_children_per_container,
            max_retained_bytes,
        })
    }

    /// Returns the maximum accepted node count.
    #[must_use]
    pub const fn max_nodes(self) -> usize {
        self.max_nodes
    }

    /// Returns the maximum accepted depth, counting the root as one.
    #[must_use]
    pub const fn max_depth(self) -> usize {
        self.max_depth
    }

    /// Returns the maximum accepted direct child count per container.
    #[must_use]
    pub const fn max_children_per_container(self) -> usize {
        self.max_children_per_container
    }

    /// Returns the maximum accepted retained component UTF-8 byte count.
    #[must_use]
    pub const fn max_retained_bytes(self) -> usize {
        self.max_retained_bytes
    }
}

/// The accepted component owned by one candidate hierarchy node.
#[derive(Debug, Eq, PartialEq)]
pub enum XmlElementNodeValue {
    /// A name-only element that may parent later nodes.
    Container(XmlElementName),
    /// A composed leaf element that cannot parent another node.
    Leaf(XmlLeafElement),
}

impl XmlElementNodeValue {
    /// Returns the container name, if this value is a container.
    #[must_use]
    pub const fn as_container(&self) -> Option<&XmlElementName> {
        match self {
            Self::Container(name) => Some(name),
            Self::Leaf(_) => None,
        }
    }

    /// Returns the leaf element, if this value is a leaf.
    #[must_use]
    pub const fn as_leaf(&self) -> Option<&XmlLeafElement> {
        match self {
            Self::Container(_) => None,
            Self::Leaf(leaf) => Some(leaf),
        }
    }
}

/// One caller-ordered candidate node containing only accepted components.
#[derive(Debug, Eq, PartialEq)]
pub struct XmlElementNodeInput {
    parent_index: Option<usize>,
    value: XmlElementNodeValue,
}

impl XmlElementNodeInput {
    /// Moves a parent index and accepted component value into candidate state.
    #[must_use]
    pub const fn new(parent_index: Option<usize>, value: XmlElementNodeValue) -> Self {
        Self {
            parent_index,
            value,
        }
    }

    /// Returns `None` for a candidate root or the caller-provided parent index.
    #[must_use]
    pub const fn parent_index(&self) -> Option<usize> {
        self.parent_index
    }

    /// Returns the unchanged accepted candidate value.
    #[must_use]
    pub const fn value(&self) -> &XmlElementNodeValue {
        &self.value
    }

    /// Returns the unchanged candidate members.
    #[must_use]
    pub fn into_parts(self) -> (Option<usize>, XmlElementNodeValue) {
        (self.parent_index, self.value)
    }
}

/// A bounded parent-before-child container/leaf hierarchy.
///
/// Caller order records parent identity only. It assigns no serialization,
/// document, stream-info, `info`, or `desc` meaning.
#[derive(Debug, Eq, PartialEq)]
pub struct XmlElementTree {
    limits: XmlElementTreeLimits,
    nodes: Vec<XmlElementNodeInput>,
}

impl XmlElementTree {
    /// Validates one caller-owned candidate arena without replacing it.
    ///
    /// Rejection precedence is empty arena, node bound, root parent, scratch
    /// reservation, each later node's parent identity, parent kind, depth and
    /// child bound, then retained-byte arithmetic and its arena bound.
    pub fn new(
        limits: XmlElementTreeLimits,
        nodes: Vec<XmlElementNodeInput>,
    ) -> Result<Self, XmlElementTreeError> {
        if nodes.is_empty() {
            return Err(XmlElementTreeError::EmptyArena);
        }
        if nodes.len() > limits.max_nodes {
            return Err(XmlElementTreeError::NodeLimitExceeded {
                expected_max: limits.max_nodes,
                actual: nodes.len(),
            });
        }
        if let Some(parent_index) = nodes[0].parent_index {
            return Err(XmlElementTreeError::RootHasParent { parent_index });
        }

        let mut scratch = Vec::new();
        reserve_scratch(&mut scratch, nodes.len())?;
        scratch.push(ScratchNode {
            depth: 1,
            child_count: 0,
        });

        for node_index in 1..nodes.len() {
            let parent_index = match nodes[node_index].parent_index {
                Some(parent_index) => parent_index,
                None => return Err(XmlElementTreeError::ExtraRoot { node_index }),
            };
            if parent_index >= nodes.len() {
                return Err(XmlElementTreeError::ParentOutOfRange {
                    node_index,
                    parent_index,
                    node_count: nodes.len(),
                });
            }
            if parent_index == node_index {
                return Err(XmlElementTreeError::ParentIsSelf {
                    node_index,
                    parent_index,
                });
            }
            if parent_index > node_index {
                return Err(XmlElementTreeError::ParentIsForward {
                    node_index,
                    parent_index,
                });
            }
            if matches!(nodes[parent_index].value, XmlElementNodeValue::Leaf(_)) {
                return Err(XmlElementTreeError::LeafParent {
                    node_index,
                    parent_index,
                });
            }

            let depth = scratch[parent_index].depth + 1;
            if depth > limits.max_depth {
                return Err(XmlElementTreeError::DepthLimitExceeded {
                    node_index,
                    expected_max: limits.max_depth,
                    actual: depth,
                });
            }
            let child_count = scratch[parent_index].child_count + 1;
            if child_count > limits.max_children_per_container {
                return Err(XmlElementTreeError::ChildLimitExceeded {
                    node_index,
                    parent_index,
                    expected_max: limits.max_children_per_container,
                    actual: child_count,
                });
            }
            scratch[parent_index].child_count = child_count;
            scratch.push(ScratchNode {
                depth,
                child_count: 0,
            });
        }

        let retained_bytes = retained_bytes(&nodes)?;
        if retained_bytes > limits.max_retained_bytes {
            return Err(XmlElementTreeError::RetainedBytesLimitExceeded {
                expected_max: limits.max_retained_bytes,
                actual: retained_bytes,
            });
        }

        Ok(Self { limits, nodes })
    }

    /// Returns the limits under which the hierarchy was accepted.
    #[must_use]
    pub const fn limits(&self) -> XmlElementTreeLimits {
        self.limits
    }

    /// Returns the original caller-ordered candidate-node arena.
    #[must_use]
    pub fn nodes(&self) -> &[XmlElementNodeInput] {
        &self.nodes
    }

    /// Returns the original caller-owned node vector without replacing its allocation.
    #[must_use]
    pub fn into_nodes(self) -> Vec<XmlElementNodeInput> {
        self.nodes
    }
}

#[derive(Clone, Copy)]
struct ScratchNode {
    depth: usize,
    child_count: usize,
}

/// Deterministic rejection from hierarchy limit configuration or validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum XmlElementTreeError {
    /// A configured maximum was zero.
    InvalidLimit {
        /// The malformed resource bound.
        bound: XmlElementTreeBound,
        /// The smallest accepted maximum.
        expected_min: usize,
        /// The caller-provided maximum.
        actual: usize,
    },
    /// No root node was present.
    EmptyArena,
    /// The caller arena exceeded its node maximum.
    NodeLimitExceeded {
        /// The configured maximum.
        expected_max: usize,
        /// The caller-provided node count.
        actual: usize,
    },
    /// Index zero incorrectly named a parent.
    RootHasParent {
        /// The invalid parent index.
        parent_index: usize,
    },
    /// Scratch state could not be reserved without panicking.
    ScratchAllocationFailed {
        /// The exact scratch element count requested.
        requested: usize,
    },
    /// A later node omitted its required parent.
    ExtraRoot {
        /// The rejected node index.
        node_index: usize,
    },
    /// A node named a parent outside the caller arena.
    ParentOutOfRange {
        /// The rejected node index.
        node_index: usize,
        /// The invalid parent index.
        parent_index: usize,
        /// The total caller-provided node count.
        node_count: usize,
    },
    /// A node named itself as parent.
    ParentIsSelf {
        /// The rejected node index.
        node_index: usize,
        /// The parent index, equal to the node index.
        parent_index: usize,
    },
    /// A node named a later in-range parent.
    ParentIsForward {
        /// The rejected node index.
        node_index: usize,
        /// The later parent index.
        parent_index: usize,
    },
    /// A node named an earlier leaf as its parent.
    LeafParent {
        /// The rejected child index.
        node_index: usize,
        /// The earlier leaf index.
        parent_index: usize,
    },
    /// A node exceeded the configured root-one depth.
    DepthLimitExceeded {
        /// The rejected node index.
        node_index: usize,
        /// The configured maximum.
        expected_max: usize,
        /// The computed depth.
        actual: usize,
    },
    /// A container exceeded its direct child maximum.
    ChildLimitExceeded {
        /// The rejected child index.
        node_index: usize,
        /// The parent container index.
        parent_index: usize,
        /// The configured maximum.
        expected_max: usize,
        /// The direct count including the rejected child.
        actual: usize,
    },
    /// Retained component UTF-8 bytes overflowed `usize` at one node.
    RetainedBytesOverflow {
        /// The node whose component addition overflowed.
        node_index: usize,
    },
    /// Retained component UTF-8 bytes exceeded the arena resource maximum.
    RetainedBytesLimitExceeded {
        /// The configured maximum.
        expected_max: usize,
        /// The exact retained byte count.
        actual: usize,
    },
}

impl fmt::Display for XmlElementTreeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "XML element tree rejected input: {self:?}")
    }
}

impl std::error::Error for XmlElementTreeError {}

fn reserve_scratch(
    scratch: &mut Vec<ScratchNode>,
    requested: usize,
) -> Result<(), XmlElementTreeError> {
    scratch
        .try_reserve_exact(requested)
        .map_err(|_| XmlElementTreeError::ScratchAllocationFailed { requested })
}

fn retained_bytes(nodes: &[XmlElementNodeInput]) -> Result<usize, XmlElementTreeError> {
    nodes
        .iter()
        .enumerate()
        .try_fold(0usize, |total, (node_index, node)| {
            let node_bytes = match &node.value {
                XmlElementNodeValue::Container(name) => name.as_str().len(),
                XmlElementNodeValue::Leaf(leaf) => leaf
                    .name()
                    .as_str()
                    .len()
                    .checked_add(leaf.character_data().as_str().len())
                    .ok_or(XmlElementTreeError::RetainedBytesOverflow { node_index })?,
            };
            checked_retained_sum(total, node_bytes, node_index)
        })
}

fn checked_retained_sum(
    total: usize,
    node_bytes: usize,
    node_index: usize,
) -> Result<usize, XmlElementTreeError> {
    total
        .checked_add(node_bytes)
        .ok_or(XmlElementTreeError::RetainedBytesOverflow { node_index })
}

#[cfg(test)]
mod tests {
    use super::{
        checked_retained_sum, reserve_scratch, retained_bytes, ScratchNode, XmlElementNodeInput,
        XmlElementNodeValue, XmlElementTree, XmlElementTreeBound, XmlElementTreeError,
        XmlElementTreeLimits,
    };
    use crate::{
        XmlCharacterData, XmlCharacterDataLimit, XmlElementName, XmlLeafElement, XmlNameLimit,
        XmlText, XmlTextLimit,
    };

    fn name(value: &str) -> XmlElementName {
        XmlElementName::new(
            XmlNameLimit::new(value.chars().count().max(1)).unwrap(),
            value.to_owned(),
        )
        .unwrap()
    }

    fn leaf(name_value: &str, text_value: &str) -> XmlLeafElement {
        let text = XmlText::new(
            XmlTextLimit::new(text_value.chars().count().max(1)).unwrap(),
            text_value.to_owned(),
        )
        .unwrap();
        let data =
            XmlCharacterData::encode(XmlCharacterDataLimit::new(256).unwrap(), &text).unwrap();
        XmlLeafElement::new(name(name_value), data)
    }

    fn container(parent: Option<usize>, value: &str) -> XmlElementNodeInput {
        XmlElementNodeInput::new(parent, XmlElementNodeValue::Container(name(value)))
    }

    fn leaf_node(parent: Option<usize>, name: &str, text: &str) -> XmlElementNodeInput {
        XmlElementNodeInput::new(parent, XmlElementNodeValue::Leaf(leaf(name, text)))
    }

    fn roomy() -> XmlElementTreeLimits {
        XmlElementTreeLimits::new(16, 16, 16, 1024).unwrap()
    }

    #[test]
    fn lslc_001e_exact_limits_preserve_hierarchy_and_component_values() {
        let nodes = vec![
            container(None, "根"),
            container(Some(0), "group"),
            leaf_node(Some(1), "leaf", "A&B"),
            leaf_node(Some(0), "末", "é"),
        ];
        let retained =
            "根".len() + "group".len() + "leaf".len() + "A&amp;B".len() + "末".len() + "é".len();
        let tree =
            XmlElementTree::new(XmlElementTreeLimits::new(4, 3, 2, retained).unwrap(), nodes)
                .unwrap();
        assert_eq!(tree.nodes().len(), 4);
        assert_eq!(tree.nodes()[2].parent_index(), Some(1));
        assert_eq!(
            tree.nodes()[2]
                .value()
                .as_leaf()
                .unwrap()
                .character_data()
                .as_str(),
            "A&amp;B"
        );
    }

    #[test]
    fn lslc_001e_root_may_be_a_leaf_but_a_leaf_cannot_parent() {
        assert!(XmlElementTree::new(roomy(), vec![leaf_node(None, "only", "")]).is_ok());
        assert_eq!(
            XmlElementTree::new(
                roomy(),
                vec![leaf_node(None, "root", ""), leaf_node(Some(0), "child", "")],
            ),
            Err(XmlElementTreeError::LeafParent {
                node_index: 1,
                parent_index: 0
            })
        );
    }

    #[test]
    fn lslc_001e_original_vector_and_component_allocations_are_preserved() {
        let root_name = name("root");
        let leaf = leaf("leaf", "A&B");
        let root_pointer = root_name.as_str().as_ptr();
        let leaf_name_pointer = leaf.name().as_str().as_ptr();
        let data_pointer = leaf.character_data().as_str().as_ptr();
        let mut nodes = Vec::with_capacity(8);
        nodes.push(XmlElementNodeInput::new(
            None,
            XmlElementNodeValue::Container(root_name),
        ));
        nodes.push(XmlElementNodeInput::new(
            Some(0),
            XmlElementNodeValue::Leaf(leaf),
        ));
        let vector_pointer = nodes.as_ptr();

        let tree = XmlElementTree::new(roomy(), nodes).unwrap();
        assert_eq!(tree.nodes().as_ptr(), vector_pointer);
        let nodes = tree.into_nodes();
        assert_eq!(nodes.as_ptr(), vector_pointer);
        let mut nodes = nodes.into_iter();
        let (_, root) = nodes.next().unwrap().into_parts();
        assert_eq!(root.as_container().unwrap().as_str().as_ptr(), root_pointer);
        let (_, value) = nodes.next().unwrap().into_parts();
        let (name, data) = match value {
            XmlElementNodeValue::Leaf(value) => value.into_parts(),
            XmlElementNodeValue::Container(_) => unreachable!(),
        };
        let name = name.into_string();
        let data = data.into_string();
        assert_eq!(name.as_ptr(), leaf_name_pointer);
        assert_eq!(data.as_ptr(), data_pointer);
    }

    #[test]
    fn lslc_001e_zero_limits_reject_in_argument_order() {
        for (args, bound) in [
            ((0, 0, 0, 0), XmlElementTreeBound::Nodes),
            ((1, 0, 0, 0), XmlElementTreeBound::Depth),
            ((1, 1, 0, 0), XmlElementTreeBound::ChildrenPerContainer),
            ((1, 1, 1, 0), XmlElementTreeBound::RetainedBytes),
        ] {
            assert_eq!(
                XmlElementTreeLimits::new(args.0, args.1, args.2, args.3),
                Err(XmlElementTreeError::InvalidLimit {
                    bound,
                    expected_min: 1,
                    actual: 0
                })
            );
        }
    }

    #[test]
    fn lslc_001e_empty_node_and_root_parent_precedence_is_stable() {
        assert_eq!(
            XmlElementTree::new(roomy(), vec![]),
            Err(XmlElementTreeError::EmptyArena)
        );
        assert_eq!(
            XmlElementTree::new(
                XmlElementTreeLimits::new(1, 1, 1, 1).unwrap(),
                vec![container(Some(9), "root"), container(None, "extra")],
            ),
            Err(XmlElementTreeError::NodeLimitExceeded {
                expected_max: 1,
                actual: 2
            })
        );
        assert_eq!(
            XmlElementTree::new(roomy(), vec![container(Some(9), "root")]),
            Err(XmlElementTreeError::RootHasParent { parent_index: 9 })
        );
    }

    #[test]
    fn lslc_001e_parent_identity_errors_are_distinct_and_indexed() {
        let cases = [
            (None, XmlElementTreeError::ExtraRoot { node_index: 1 }),
            (
                Some(1),
                XmlElementTreeError::ParentIsSelf {
                    node_index: 1,
                    parent_index: 1,
                },
            ),
            (
                Some(2),
                XmlElementTreeError::ParentIsForward {
                    node_index: 1,
                    parent_index: 2,
                },
            ),
            (
                Some(3),
                XmlElementTreeError::ParentOutOfRange {
                    node_index: 1,
                    parent_index: 3,
                    node_count: 3,
                },
            ),
        ];
        for (parent, expected) in cases {
            assert_eq!(
                XmlElementTree::new(
                    roomy(),
                    vec![
                        container(None, "r"),
                        container(parent, "x"),
                        container(Some(0), "y")
                    ]
                ),
                Err(expected)
            );
        }
    }

    #[test]
    fn lslc_001e_depth_and_child_one_past_errors_follow_parent_kind() {
        assert_eq!(
            XmlElementTree::new(
                XmlElementTreeLimits::new(3, 2, 2, 32).unwrap(),
                vec![
                    container(None, "r"),
                    container(Some(0), "a"),
                    leaf_node(Some(1), "b", "")
                ],
            ),
            Err(XmlElementTreeError::DepthLimitExceeded {
                node_index: 2,
                expected_max: 2,
                actual: 3
            })
        );
        assert_eq!(
            XmlElementTree::new(
                XmlElementTreeLimits::new(3, 2, 1, 32).unwrap(),
                vec![
                    container(None, "r"),
                    leaf_node(Some(0), "a", ""),
                    leaf_node(Some(0), "b", "")
                ],
            ),
            Err(XmlElementTreeError::ChildLimitExceeded {
                node_index: 2,
                parent_index: 0,
                expected_max: 1,
                actual: 2
            })
        );
    }

    #[test]
    fn lslc_001e_retained_utf8_byte_bound_is_exact_and_last() {
        let nodes = vec![container(None, "é"), leaf_node(Some(0), "叶", "&")];
        let required = "é".len() + "叶".len() + "&amp;".len();
        assert!(
            XmlElementTree::new(XmlElementTreeLimits::new(2, 2, 1, required).unwrap(), nodes)
                .is_ok()
        );
        assert_eq!(
            XmlElementTree::new(
                XmlElementTreeLimits::new(2, 2, 1, required - 1).unwrap(),
                vec![container(None, "é"), leaf_node(Some(0), "叶", "&")],
            ),
            Err(XmlElementTreeError::RetainedBytesLimitExceeded {
                expected_max: required - 1,
                actual: required
            })
        );
        assert_eq!(
            XmlElementTree::new(
                XmlElementTreeLimits::new(2, 1, 1, 1).unwrap(),
                vec![container(None, "root"), leaf_node(Some(0), "leaf", "long")],
            ),
            Err(XmlElementTreeError::DepthLimitExceeded {
                node_index: 1,
                expected_max: 1,
                actual: 2
            })
        );
    }

    #[test]
    fn lslc_001e_allocation_and_overflow_helpers_return_typed_errors() {
        let mut scratch: Vec<ScratchNode> = Vec::new();
        assert_eq!(
            reserve_scratch(&mut scratch, usize::MAX),
            Err(XmlElementTreeError::ScratchAllocationFailed {
                requested: usize::MAX
            })
        );
        let nodes = vec![container(None, "root")];
        assert_eq!(retained_bytes(&nodes).unwrap(), 4);
        assert_eq!(
            checked_retained_sum(usize::MAX, 1, 7),
            Err(XmlElementTreeError::RetainedBytesOverflow { node_index: 7 })
        );
    }
}
