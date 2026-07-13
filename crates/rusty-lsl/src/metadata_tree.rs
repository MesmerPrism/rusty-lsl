// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

/// Identifies one configured metadata-tree bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MetadataTreeBound {
    /// Maximum node count in the flat arena.
    Nodes,
    /// Maximum parent-chain depth, where the root has depth one.
    Depth,
    /// Maximum direct child count for any node.
    ChildrenPerNode,
    /// Maximum Unicode scalar-value count in a node name.
    NameCodePoints,
    /// Maximum Unicode scalar-value count in an optional node value.
    ValueCodePoints,
}

/// Identifies one bounded metadata-tree text member.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MetadataTreeTextRole {
    /// The required nonempty node name.
    Name,
    /// The optional node value.
    Value,
}

/// Explicit nonzero maxima for one flat metadata tree.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MetadataTreeLimits {
    max_nodes: usize,
    max_depth: usize,
    max_children_per_node: usize,
    max_name_code_points: usize,
    max_value_code_points: usize,
}

impl MetadataTreeLimits {
    /// Validates all maxima in the same order as the arguments.
    pub fn new(
        max_nodes: usize,
        max_depth: usize,
        max_children_per_node: usize,
        max_name_code_points: usize,
        max_value_code_points: usize,
    ) -> Result<Self, MetadataTreeError> {
        for (bound, actual) in [
            (MetadataTreeBound::Nodes, max_nodes),
            (MetadataTreeBound::Depth, max_depth),
            (MetadataTreeBound::ChildrenPerNode, max_children_per_node),
            (MetadataTreeBound::NameCodePoints, max_name_code_points),
            (MetadataTreeBound::ValueCodePoints, max_value_code_points),
        ] {
            if actual == 0 {
                return Err(MetadataTreeError::InvalidLimit {
                    bound,
                    expected_min: 1,
                    actual,
                });
            }
        }

        Ok(Self {
            max_nodes,
            max_depth,
            max_children_per_node,
            max_name_code_points,
            max_value_code_points,
        })
    }

    /// Returns the maximum accepted node count.
    #[must_use]
    pub const fn max_nodes(self) -> usize {
        self.max_nodes
    }

    /// Returns the maximum accepted depth, counting the root as depth one.
    #[must_use]
    pub const fn max_depth(self) -> usize {
        self.max_depth
    }

    /// Returns the maximum accepted direct child count for any node.
    #[must_use]
    pub const fn max_children_per_node(self) -> usize {
        self.max_children_per_node
    }

    /// Returns the maximum accepted node-name Unicode scalar count.
    #[must_use]
    pub const fn max_name_code_points(self) -> usize {
        self.max_name_code_points
    }

    /// Returns the maximum accepted optional-value Unicode scalar count.
    #[must_use]
    pub const fn max_value_code_points(self) -> usize {
        self.max_value_code_points
    }
}

/// One unvalidated node in a caller-ordered parent-before-child arena.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataNodeInput {
    parent_index: Option<usize>,
    name: String,
    value: Option<String>,
}

impl MetadataNodeInput {
    /// Creates an unvalidated flat node input without interpreting its text.
    #[must_use]
    pub const fn new(parent_index: Option<usize>, name: String, value: Option<String>) -> Self {
        Self {
            parent_index,
            name,
            value,
        }
    }

    /// Returns the caller-provided optional parent index.
    #[must_use]
    pub const fn parent_index(&self) -> Option<usize> {
        self.parent_index
    }

    /// Returns the caller-provided name without validation or alteration.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the caller-provided optional value without alteration.
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Returns all caller-provided input members unchanged.
    #[must_use]
    pub fn into_parts(self) -> (Option<usize>, String, Option<String>) {
        (self.parent_index, self.name, self.value)
    }
}

/// One validated node retained in its caller-provided flat arena position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataNode {
    parent_index: Option<usize>,
    name: String,
    value: Option<String>,
}

impl MetadataNode {
    /// Returns `None` for the root or the unchanged earlier parent index.
    #[must_use]
    pub const fn parent_index(&self) -> Option<usize> {
        self.parent_index
    }

    /// Returns the unchanged required node name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the unchanged optional node value.
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Returns the unchanged parent index, name, and optional value.
    #[must_use]
    pub fn into_parts(self) -> (Option<usize>, String, Option<String>) {
        (self.parent_index, self.name, self.value)
    }
}

/// A bounded parent-before-child metadata tree stored as one flat arena.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataTree {
    limits: MetadataTreeLimits,
    nodes: Vec<MetadataNode>,
}

impl MetadataTree {
    /// Validates the complete flat input before exposing accepted state.
    ///
    /// Index zero must be the only node without a parent. Every later node
    /// must name a strictly earlier parent. Depth and child counts are computed
    /// by one forward pass over the arena; text is counted in Unicode scalar
    /// values and is not trimmed, normalized, parsed, or otherwise changed.
    pub fn new(
        limits: MetadataTreeLimits,
        nodes: Vec<MetadataNodeInput>,
    ) -> Result<Self, MetadataTreeError> {
        if nodes.is_empty() {
            return Err(MetadataTreeError::EmptyArena);
        }
        if nodes.len() > limits.max_nodes {
            return Err(MetadataTreeError::NodeLimitExceeded {
                expected_max: limits.max_nodes,
                actual: nodes.len(),
            });
        }
        if let Some(parent_index) = nodes[0].parent_index {
            return Err(MetadataTreeError::RootHasParent { parent_index });
        }

        let mut depths = Vec::with_capacity(nodes.len());
        let mut child_counts = vec![0_usize; nodes.len()];
        for (node_index, node) in nodes.iter().enumerate() {
            let depth = if node_index == 0 {
                1
            } else {
                let parent_index = match node.parent_index {
                    Some(parent_index) => parent_index,
                    None => return Err(MetadataTreeError::ExtraRoot { node_index }),
                };
                if parent_index >= nodes.len() {
                    return Err(MetadataTreeError::ParentOutOfRange {
                        node_index,
                        parent_index,
                        node_count: nodes.len(),
                    });
                }
                if parent_index == node_index {
                    return Err(MetadataTreeError::ParentIsSelf {
                        node_index,
                        parent_index,
                    });
                }
                if parent_index > node_index {
                    return Err(MetadataTreeError::ParentIsForward {
                        node_index,
                        parent_index,
                    });
                }

                child_counts[parent_index] += 1;
                if child_counts[parent_index] > limits.max_children_per_node {
                    return Err(MetadataTreeError::ChildLimitExceeded {
                        node_index,
                        parent_index,
                        expected_max: limits.max_children_per_node,
                        actual: child_counts[parent_index],
                    });
                }
                depths[parent_index] + 1
            };

            if depth > limits.max_depth {
                return Err(MetadataTreeError::DepthLimitExceeded {
                    node_index,
                    expected_max: limits.max_depth,
                    actual: depth,
                });
            }
            if node.name.is_empty() {
                return Err(MetadataTreeError::EmptyName { node_index });
            }
            for (role, text, expected_max) in [
                (
                    MetadataTreeTextRole::Name,
                    Some(node.name.as_str()),
                    limits.max_name_code_points,
                ),
                (
                    MetadataTreeTextRole::Value,
                    node.value.as_deref(),
                    limits.max_value_code_points,
                ),
            ] {
                if let Some(text) = text {
                    let actual = text.chars().count();
                    if actual > expected_max {
                        return Err(MetadataTreeError::TextLimitExceeded {
                            node_index,
                            role,
                            expected_max,
                            actual,
                        });
                    }
                }
            }
            depths.push(depth);
        }

        let nodes = nodes
            .into_iter()
            .map(|node| MetadataNode {
                parent_index: node.parent_index,
                name: node.name,
                value: node.value,
            })
            .collect();
        Ok(Self { limits, nodes })
    }

    /// Returns the limits under which this tree was accepted.
    #[must_use]
    pub const fn limits(&self) -> MetadataTreeLimits {
        self.limits
    }

    /// Returns the unchanged parent-before-child node arena.
    #[must_use]
    pub fn nodes(&self) -> &[MetadataNode] {
        &self.nodes
    }

    /// Returns the unchanged owned parent-before-child node arena.
    #[must_use]
    pub fn into_nodes(self) -> Vec<MetadataNode> {
        self.nodes
    }
}

/// Deterministic rejection from metadata-tree limits or construction.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MetadataTreeError {
    /// A configured maximum cannot accept any value for the named bound.
    InvalidLimit {
        /// The malformed bound.
        bound: MetadataTreeBound,
        /// The smallest accepted configuration value.
        expected_min: usize,
        /// The caller-provided configuration value.
        actual: usize,
    },
    /// No root node was present.
    EmptyArena,
    /// The node arena exceeded its configured total-node maximum.
    NodeLimitExceeded {
        /// The configured maximum node count.
        expected_max: usize,
        /// The caller-provided node count.
        actual: usize,
    },
    /// Index zero incorrectly named a parent.
    RootHasParent {
        /// The invalid caller-provided parent index.
        parent_index: usize,
    },
    /// A later node omitted its required parent and would form another root.
    ExtraRoot {
        /// The zero-based location of the additional root.
        node_index: usize,
    },
    /// A node named a parent outside the supplied arena.
    ParentOutOfRange {
        /// The zero-based location of the rejected node.
        node_index: usize,
        /// The caller-provided parent index.
        parent_index: usize,
        /// The total caller-provided node count.
        node_count: usize,
    },
    /// A node named itself as parent.
    ParentIsSelf {
        /// The zero-based location of the rejected node.
        node_index: usize,
        /// The caller-provided parent index, equal to `node_index`.
        parent_index: usize,
    },
    /// A node named a later in-range node as parent.
    ParentIsForward {
        /// The zero-based location of the rejected node.
        node_index: usize,
        /// The caller-provided later parent index.
        parent_index: usize,
    },
    /// A node's computed parent-chain depth exceeded the configured maximum.
    DepthLimitExceeded {
        /// The zero-based location of the rejected node.
        node_index: usize,
        /// The configured maximum depth.
        expected_max: usize,
        /// The observed depth, counting the root as one.
        actual: usize,
    },
    /// Adding a node exceeded its parent's configured direct-child maximum.
    ChildLimitExceeded {
        /// The zero-based location of the child that exceeded the maximum.
        node_index: usize,
        /// The zero-based location of its parent.
        parent_index: usize,
        /// The configured maximum direct child count.
        expected_max: usize,
        /// The observed child count after adding the rejected node.
        actual: usize,
    },
    /// A required node name was empty.
    EmptyName {
        /// The zero-based location of the rejected node.
        node_index: usize,
    },
    /// One node text member exceeded its Unicode scalar-value maximum.
    TextLimitExceeded {
        /// The zero-based location of the rejected node.
        node_index: usize,
        /// The rejected text member.
        role: MetadataTreeTextRole,
        /// The configured Unicode scalar-value maximum.
        expected_max: usize,
        /// The observed Unicode scalar-value count.
        actual: usize,
    },
}

impl fmt::Display for MetadataTreeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "metadata tree rejected input: {self:?}")
    }
}

impl std::error::Error for MetadataTreeError {}

#[cfg(test)]
mod tests {
    use super::{
        MetadataNode, MetadataNodeInput, MetadataTree, MetadataTreeBound, MetadataTreeError,
        MetadataTreeLimits, MetadataTreeTextRole,
    };

    fn node(parent_index: Option<usize>, name: &str, value: Option<&str>) -> MetadataNodeInput {
        MetadataNodeInput::new(parent_index, name.to_owned(), value.map(str::to_owned))
    }

    fn raw(tree: MetadataTree) -> Vec<(Option<usize>, String, Option<String>)> {
        tree.into_nodes()
            .into_iter()
            .map(MetadataNode::into_parts)
            .collect()
    }

    fn roomy_limits() -> MetadataTreeLimits {
        MetadataTreeLimits::new(8, 8, 8, 8, 8).unwrap()
    }

    #[test]
    fn core_004_tree_exact_limits_preserve_flat_state() {
        let limits = MetadataTreeLimits::new(4, 3, 2, 5, 3).unwrap();
        let inputs = vec![
            node(None, " root", Some("v中 ")),
            node(Some(0), "left", None),
            node(Some(0), "右", Some("")),
            node(Some(1), "leaf", Some(" x ")),
        ];
        let expected: Vec<_> = inputs
            .clone()
            .into_iter()
            .map(MetadataNodeInput::into_parts)
            .collect();

        let tree = MetadataTree::new(limits, inputs).unwrap();

        assert_eq!(tree.limits(), limits);
        assert_eq!(tree.nodes()[0].parent_index(), None);
        assert_eq!(tree.nodes()[1].parent_index(), Some(0));
        assert_eq!(tree.nodes()[0].name(), " root");
        assert_eq!(tree.nodes()[0].value(), Some("v中 "));
        assert_eq!(raw(tree), expected);
    }

    #[test]
    fn core_004_absent_and_empty_values_remain_distinct() {
        let tree = MetadataTree::new(
            MetadataTreeLimits::new(2, 2, 1, 1, 1).unwrap(),
            vec![node(None, "r", None), node(Some(0), "c", Some(""))],
        )
        .unwrap();

        assert_eq!(tree.nodes()[0].value(), None);
        assert_eq!(tree.nodes()[1].value(), Some(""));
    }

    #[test]
    fn core_004_deep_parent_chain_at_depth_limit() {
        const NODE_COUNT: usize = 65_536;
        let inputs = (0_usize..NODE_COUNT)
            .map(|index| node(index.checked_sub(1), "n", None))
            .collect();
        let tree = MetadataTree::new(
            MetadataTreeLimits::new(NODE_COUNT, NODE_COUNT, 1, 1, 1).unwrap(),
            inputs,
        )
        .unwrap();

        assert_eq!(
            tree.nodes()[NODE_COUNT - 1].parent_index(),
            Some(NODE_COUNT - 2)
        );
    }

    #[test]
    fn core_004_child_fanout_at_limit() {
        let tree = MetadataTree::new(
            MetadataTreeLimits::new(5, 2, 4, 1, 1).unwrap(),
            vec![
                node(None, "r", None),
                node(Some(0), "a", None),
                node(Some(0), "b", None),
                node(Some(0), "c", None),
                node(Some(0), "d", None),
            ],
        )
        .unwrap();

        assert_eq!(tree.nodes().len(), 5);
    }

    #[test]
    fn core_004_unicode_scalar_counts_not_bytes() {
        let tree = MetadataTree::new(
            MetadataTreeLimits::new(1, 1, 1, 3, 3).unwrap(),
            vec![node(None, "é中🦀", Some("ß界🙂"))],
        )
        .unwrap();

        assert_eq!(tree.nodes()[0].name(), "é中🦀");
        assert_eq!(tree.nodes()[0].value(), Some("ß界🙂"));
    }

    #[test]
    fn core_004_zero_limits_reject_in_argument_order() {
        for (arguments, bound) in [
            ((0, 0, 0, 0, 0), MetadataTreeBound::Nodes),
            ((1, 0, 0, 0, 0), MetadataTreeBound::Depth),
            ((1, 1, 0, 0, 0), MetadataTreeBound::ChildrenPerNode),
            ((1, 1, 1, 0, 0), MetadataTreeBound::NameCodePoints),
            ((1, 1, 1, 1, 0), MetadataTreeBound::ValueCodePoints),
        ] {
            assert_eq!(
                MetadataTreeLimits::new(
                    arguments.0,
                    arguments.1,
                    arguments.2,
                    arguments.3,
                    arguments.4,
                ),
                Err(MetadataTreeError::InvalidLimit {
                    bound,
                    expected_min: 1,
                    actual: 0,
                })
            );
        }
    }

    #[test]
    fn core_004_empty_arena_has_stable_error() {
        assert_eq!(
            MetadataTree::new(roomy_limits(), vec![]),
            Err(MetadataTreeError::EmptyArena)
        );
    }

    #[test]
    fn core_004_invalid_root_parent_has_stable_error() {
        assert_eq!(
            MetadataTree::new(roomy_limits(), vec![node(Some(7), "r", None)]),
            Err(MetadataTreeError::RootHasParent { parent_index: 7 })
        );
    }

    #[test]
    fn core_004_extra_root_has_stable_indexed_error() {
        assert_eq!(
            MetadataTree::new(
                roomy_limits(),
                vec![node(None, "r", None), node(None, "x", None)],
            ),
            Err(MetadataTreeError::ExtraRoot { node_index: 1 })
        );
    }

    #[test]
    fn core_004_self_forward_and_out_of_range_parents_are_distinct() {
        for (parent_index, expected) in [
            (
                1,
                MetadataTreeError::ParentIsSelf {
                    node_index: 1,
                    parent_index: 1,
                },
            ),
            (
                2,
                MetadataTreeError::ParentIsForward {
                    node_index: 1,
                    parent_index: 2,
                },
            ),
            (
                3,
                MetadataTreeError::ParentOutOfRange {
                    node_index: 1,
                    parent_index: 3,
                    node_count: 3,
                },
            ),
        ] {
            assert_eq!(
                MetadataTree::new(
                    roomy_limits(),
                    vec![
                        node(None, "r", None),
                        node(Some(parent_index), "x", None),
                        node(Some(0), "y", None),
                    ],
                ),
                Err(expected)
            );
        }
    }

    #[test]
    fn core_004_one_past_node_limit_has_stable_error() {
        assert_eq!(
            MetadataTree::new(
                MetadataTreeLimits::new(1, 2, 1, 1, 1).unwrap(),
                vec![node(None, "r", None), node(Some(0), "c", None)],
            ),
            Err(MetadataTreeError::NodeLimitExceeded {
                expected_max: 1,
                actual: 2,
            })
        );
    }

    #[test]
    fn core_004_one_past_depth_limit_has_stable_indexed_error() {
        assert_eq!(
            MetadataTree::new(
                MetadataTreeLimits::new(3, 2, 1, 1, 1).unwrap(),
                vec![
                    node(None, "r", None),
                    node(Some(0), "a", None),
                    node(Some(1), "b", None),
                ],
            ),
            Err(MetadataTreeError::DepthLimitExceeded {
                node_index: 2,
                expected_max: 2,
                actual: 3,
            })
        );
    }

    #[test]
    fn core_004_one_past_child_limit_has_stable_indexed_error() {
        assert_eq!(
            MetadataTree::new(
                MetadataTreeLimits::new(3, 2, 1, 1, 1).unwrap(),
                vec![
                    node(None, "r", None),
                    node(Some(0), "a", None),
                    node(Some(0), "b", None),
                ],
            ),
            Err(MetadataTreeError::ChildLimitExceeded {
                node_index: 2,
                parent_index: 0,
                expected_max: 1,
                actual: 2,
            })
        );
    }

    #[test]
    fn core_004_one_past_text_limits_have_stable_indexed_errors() {
        for (name, value, role) in [
            ("é中", None, MetadataTreeTextRole::Name),
            ("n", Some("ß界"), MetadataTreeTextRole::Value),
        ] {
            assert_eq!(
                MetadataTree::new(
                    MetadataTreeLimits::new(1, 1, 1, 1, 1).unwrap(),
                    vec![node(None, name, value)],
                ),
                Err(MetadataTreeError::TextLimitExceeded {
                    node_index: 0,
                    role,
                    expected_max: 1,
                    actual: 2,
                })
            );
        }
    }

    #[test]
    fn core_004_empty_name_has_stable_indexed_error() {
        assert_eq!(
            MetadataTree::new(
                roomy_limits(),
                vec![node(None, "r", None), node(Some(0), "", None)],
            ),
            Err(MetadataTreeError::EmptyName { node_index: 1 })
        );
    }
}
