// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{XmlElementNodeValue, XmlElementTree};
use core::fmt;

/// An explicit nonzero maximum for serialized UTF-8 output bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XmlElementSerializationLimit {
    max_output_bytes: usize,
}

impl XmlElementSerializationLimit {
    /// Creates an output limit that can accept at least one byte.
    pub fn new(max_output_bytes: usize) -> Result<Self, XmlElementSerializationError> {
        if max_output_bytes == 0 {
            return Err(XmlElementSerializationError::InvalidLimit {
                expected_min: 1,
                actual: max_output_bytes,
            });
        }
        Ok(Self { max_output_bytes })
    }

    /// Returns the maximum accepted serialized UTF-8 byte count.
    #[must_use]
    pub const fn max_output_bytes(self) -> usize {
        self.max_output_bytes
    }
}

/// One bounded owned serialization of a borrowed accepted element tree.
#[derive(Debug, Eq, PartialEq)]
pub struct XmlElementSerialization {
    limit: XmlElementSerializationLimit,
    output: String,
}

impl XmlElementSerialization {
    /// Serializes an accepted tree under the fixed local element spelling policy.
    ///
    /// Every node has an explicit start and end tag. Containers emit direct
    /// children depth-first in ascending arena-index order. Accepted character
    /// data is copied verbatim without decoding, revalidation, or re-escaping.
    /// Exact length and the output limit are checked before the single stack
    /// reservation and the single output reservation.
    pub fn serialize(
        limit: XmlElementSerializationLimit,
        source: &XmlElementTree,
    ) -> Result<Self, XmlElementSerializationError> {
        let required = exact_output_bytes(source.nodes())?;
        if required > limit.max_output_bytes {
            return Err(XmlElementSerializationError::OutputLimitExceeded {
                expected_max: limit.max_output_bytes,
                required,
            });
        }

        let requested_stack = source.nodes().len();
        let mut stack = Vec::new();
        reserve_traversal_stack(&mut stack, requested_stack)?;
        build_traversal_stack(source, &mut stack);

        let mut output = String::new();
        reserve_output(&mut output, required)?;
        let mut node_index = 0;
        loop {
            let node = &source.nodes()[node_index];
            let name = node_name(node.value());
            output.push('<');
            output.push_str(name);
            output.push('>');
            match node.value() {
                XmlElementNodeValue::Leaf(leaf) => {
                    output.push_str(leaf.character_data().as_str());
                    write_end_tag(&mut output, name);
                }
                XmlElementNodeValue::Container(_) => {
                    if let Some(first_child) = stack[node_index].first_child {
                        node_index = first_child;
                        continue;
                    }
                    write_end_tag(&mut output, name);
                }
            }

            loop {
                if node_index == 0 {
                    debug_assert_eq!(output.len(), required);
                    return Ok(Self { limit, output });
                }
                if let Some(next_sibling) = stack[node_index].next_sibling {
                    node_index = next_sibling;
                    break;
                }
                node_index = source.nodes()[node_index]
                    .parent_index()
                    .expect("accepted non-root XML nodes always have parents");
                write_end_tag(&mut output, node_name(source.nodes()[node_index].value()));
            }
        }
    }

    /// Returns the selected output-byte limit.
    #[must_use]
    pub const fn limit(&self) -> XmlElementSerializationLimit {
        self.limit
    }

    /// Returns the exact serialized UTF-8 text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.output
    }

    /// Returns the serialized string without replacing its allocation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.output
    }
}

/// Deterministic rejection from bounded element-tree serialization.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum XmlElementSerializationError {
    /// The configured maximum could not accept any output byte.
    InvalidLimit {
        /// The smallest accepted maximum.
        expected_min: usize,
        /// The caller-provided maximum.
        actual: usize,
    },
    /// Exact output-byte arithmetic overflowed at one accepted node.
    LengthOverflow {
        /// The zero-based arena index whose contribution overflowed.
        node_index: usize,
    },
    /// Exact output bytes exceeded the selected maximum.
    OutputLimitExceeded {
        /// The selected output-byte maximum.
        expected_max: usize,
        /// The exact required output-byte count.
        required: usize,
    },
    /// The bounded iterative traversal stack could not be reserved.
    TraversalStackAllocationFailed {
        /// The exact stack element count requested.
        requested: usize,
    },
    /// The exact serialized output allocation could not be reserved.
    OutputAllocationFailed {
        /// The exact output byte count requested.
        requested: usize,
    },
}

impl fmt::Display for XmlElementSerializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "XML element serialization rejected input: {self:?}"
        )
    }
}

impl std::error::Error for XmlElementSerializationError {}

#[derive(Clone, Copy, Default)]
struct TraversalFrame {
    first_child: Option<usize>,
    last_child: Option<usize>,
    next_sibling: Option<usize>,
}

fn node_name(value: &XmlElementNodeValue) -> &str {
    match value {
        XmlElementNodeValue::Container(name) => name.as_str(),
        XmlElementNodeValue::Leaf(leaf) => leaf.name().as_str(),
    }
}

fn exact_output_bytes(
    nodes: &[crate::XmlElementNodeInput],
) -> Result<usize, XmlElementSerializationError> {
    nodes
        .iter()
        .enumerate()
        .try_fold(0usize, |total, (node_index, node)| {
            let name_bytes = node_name(node.value()).len();
            let total = checked_output_sum(total, name_bytes, node_index)?;
            let total = checked_output_sum(total, name_bytes, node_index)?;
            let total = checked_output_sum(total, 5, node_index)?;
            match node.value() {
                XmlElementNodeValue::Container(_) => Ok(total),
                XmlElementNodeValue::Leaf(leaf) => {
                    checked_output_sum(total, leaf.character_data().as_str().len(), node_index)
                }
            }
        })
}

fn checked_output_sum(
    total: usize,
    addition: usize,
    node_index: usize,
) -> Result<usize, XmlElementSerializationError> {
    total
        .checked_add(addition)
        .ok_or(XmlElementSerializationError::LengthOverflow { node_index })
}

fn reserve_traversal_stack(
    stack: &mut Vec<TraversalFrame>,
    requested: usize,
) -> Result<(), XmlElementSerializationError> {
    stack
        .try_reserve_exact(requested)
        .map_err(|_| XmlElementSerializationError::TraversalStackAllocationFailed { requested })
}

fn build_traversal_stack(source: &XmlElementTree, stack: &mut Vec<TraversalFrame>) {
    stack.resize(source.nodes().len(), TraversalFrame::default());
    for child_index in 1..source.nodes().len() {
        let parent_index = source.nodes()[child_index]
            .parent_index()
            .expect("accepted non-root XML nodes always have parents");
        if let Some(last_child) = stack[parent_index].last_child {
            stack[last_child].next_sibling = Some(child_index);
        } else {
            stack[parent_index].first_child = Some(child_index);
        }
        stack[parent_index].last_child = Some(child_index);
    }
}

fn reserve_output(
    output: &mut String,
    requested: usize,
) -> Result<(), XmlElementSerializationError> {
    output
        .try_reserve_exact(requested)
        .map_err(|_| XmlElementSerializationError::OutputAllocationFailed { requested })
}

fn write_end_tag(output: &mut String, name: &str) {
    output.push_str("</");
    output.push_str(name);
    output.push('>');
}

#[cfg(test)]
mod tests {
    use super::{
        checked_output_sum, reserve_output, reserve_traversal_stack, TraversalFrame,
        XmlElementSerialization, XmlElementSerializationError, XmlElementSerializationLimit,
    };
    use crate::{
        project_metadata_tree_to_xml_element_tree, MetadataNodeInput, MetadataTree,
        MetadataTreeLimits, MetadataXmlProjectionLimits, XmlCharacterData, XmlCharacterDataLimit,
        XmlElementName, XmlElementNodeInput, XmlElementNodeValue, XmlElementTree,
        XmlElementTreeLimits, XmlLeafElement, XmlNameLimit, XmlText, XmlTextLimit,
    };

    fn name(value: &str) -> XmlElementName {
        XmlElementName::new(XmlNameLimit::new(32).unwrap(), value.to_owned()).unwrap()
    }

    fn leaf(value: &str, data: &str) -> XmlElementNodeValue {
        let text = XmlText::new(XmlTextLimit::new(64).unwrap(), data.to_owned()).unwrap();
        let data =
            XmlCharacterData::encode(XmlCharacterDataLimit::new(256).unwrap(), &text).unwrap();
        XmlElementNodeValue::Leaf(XmlLeafElement::new(name(value), data))
    }

    fn container(value: &str) -> XmlElementNodeValue {
        XmlElementNodeValue::Container(name(value))
    }

    fn node(parent: Option<usize>, value: XmlElementNodeValue) -> XmlElementNodeInput {
        XmlElementNodeInput::new(parent, value)
    }

    fn tree(nodes: Vec<XmlElementNodeInput>) -> XmlElementTree {
        XmlElementTree::new(XmlElementTreeLimits::new(64, 64, 64, 4096).unwrap(), nodes).unwrap()
    }

    fn serialize(source: &XmlElementTree, maximum: usize) -> XmlElementSerialization {
        XmlElementSerialization::serialize(
            XmlElementSerializationLimit::new(maximum).unwrap(),
            source,
        )
        .unwrap()
    }

    #[test]
    fn lslc_001g_leaf_root_and_empty_container_use_explicit_tags() {
        let leaf_root = tree(vec![node(None, leaf("value", ""))]);
        let empty_container = tree(vec![node(None, container("root"))]);
        let nested_empty_container = tree(vec![
            node(None, container("root")),
            node(Some(0), container("empty")),
        ]);

        assert_eq!(serialize(&leaf_root, 15).as_str(), "<value></value>");
        assert_eq!(serialize(&empty_container, 13).as_str(), "<root></root>");
        assert_eq!(
            serialize(&nested_empty_container, 32).as_str(),
            "<root><empty></empty></root>"
        );
    }

    #[test]
    fn lslc_001g_non_preorder_arena_serializes_depth_first_with_index_ordered_siblings() {
        let source = tree(vec![
            node(None, container("r")),
            node(Some(0), container("a")),
            node(Some(0), leaf("b", "B")),
            node(Some(1), leaf("x", "X")),
            node(Some(0), leaf("c", "C")),
        ]);

        assert_eq!(
            serialize(&source, 128).as_str(),
            "<r><a><x>X</x></a><b>B</b><c>C</c></r>"
        );
    }

    #[test]
    fn lslc_001g_unicode_colon_and_character_data_are_emitted_verbatim_once() {
        let source = tree(vec![node(None, leaf("p:é中", "&<>\"'"))]);
        let output = serialize(&source, 128);

        assert_eq!(output.as_str(), "<p:é中>&amp;&lt;&gt;\"'</p:é中>");
        assert!(!output.as_str().contains("&amp;amp;"));
    }

    #[test]
    fn lslc_001g_deep_tree_is_iterative_and_bounded() {
        const NODE_COUNT: usize = 16_384;
        let nodes = (0..NODE_COUNT)
            .map(|index| node(index.checked_sub(1), container("n")))
            .collect();
        let source = XmlElementTree::new(
            XmlElementTreeLimits::new(NODE_COUNT, NODE_COUNT, 1, NODE_COUNT).unwrap(),
            nodes,
        )
        .unwrap();
        let required = NODE_COUNT * 7;
        let output = serialize(&source, required);

        assert_eq!(output.as_str().len(), required);
        assert!(output.as_str().starts_with("<n><n><n>"));
        assert!(output.as_str().ends_with("</n></n></n>"));
    }

    #[test]
    fn lslc_001g_exact_limit_borrow_and_consuming_output_preserve_allocation() {
        let source = tree(vec![node(None, leaf("r", "é"))]);
        let output = serialize(&source, 9);
        let pointer = output.as_str().as_ptr();

        assert_eq!(output.limit().max_output_bytes(), 9);
        assert_eq!(output.as_str(), "<r>é</r>");
        let owned = output.into_string();
        assert_eq!(owned.as_ptr(), pointer);
    }

    #[test]
    fn lslc_001g_borrowed_serialization_preserves_source_components_and_allocations() {
        let source = tree(vec![
            node(None, container("root")),
            node(Some(0), leaf("leaf", "&")),
        ]);
        let root_pointer = source.nodes()[0]
            .value()
            .as_container()
            .unwrap()
            .as_str()
            .as_ptr();
        let data_pointer = source.nodes()[1]
            .value()
            .as_leaf()
            .unwrap()
            .character_data()
            .as_str()
            .as_ptr();

        assert_eq!(
            serialize(&source, 64).as_str(),
            "<root><leaf>&amp;</leaf></root>"
        );
        assert_eq!(
            source.nodes()[0]
                .value()
                .as_container()
                .unwrap()
                .as_str()
                .as_ptr(),
            root_pointer
        );
        assert_eq!(
            source.nodes()[1]
                .value()
                .as_leaf()
                .unwrap()
                .character_data()
                .as_str()
                .as_ptr(),
            data_pointer
        );
    }

    #[test]
    fn lslc_001g_core_004_through_lslc_001f_composes_into_serialization() {
        let metadata = MetadataTree::new(
            MetadataTreeLimits::new(4, 3, 2, 8, 8).unwrap(),
            vec![
                MetadataNodeInput::new(None, "root".to_owned(), None),
                MetadataNodeInput::new(Some(0), "group".to_owned(), None),
                MetadataNodeInput::new(Some(0), "z".to_owned(), Some(">".to_owned())),
                MetadataNodeInput::new(Some(1), "x".to_owned(), Some("&<".to_owned())),
            ],
        )
        .unwrap();
        let xml = project_metadata_tree_to_xml_element_tree(
            metadata,
            MetadataXmlProjectionLimits::new(
                XmlNameLimit::new(8).unwrap(),
                XmlTextLimit::new(8).unwrap(),
                XmlCharacterDataLimit::new(16).unwrap(),
                XmlElementTreeLimits::new(4, 3, 2, 64).unwrap(),
            ),
        )
        .unwrap();

        assert_eq!(
            serialize(&xml, 128).as_str(),
            "<root><group><x>&amp;&lt;</x></group><z>&gt;</z></root>"
        );
    }

    #[test]
    fn lslc_001g_zero_limit_has_stable_counts() {
        assert_eq!(
            XmlElementSerializationLimit::new(0),
            Err(XmlElementSerializationError::InvalidLimit {
                expected_min: 1,
                actual: 0,
            })
        );
    }

    #[test]
    fn lslc_001g_one_past_output_limit_reports_exact_required_bytes() {
        let source = tree(vec![node(None, leaf("r", "é"))]);
        assert_eq!(
            XmlElementSerialization::serialize(
                XmlElementSerializationLimit::new(8).unwrap(),
                &source,
            ),
            Err(XmlElementSerializationError::OutputLimitExceeded {
                expected_max: 8,
                required: 9,
            })
        );
        assert_eq!(source.nodes().len(), 1);
    }

    #[test]
    fn lslc_001g_checked_length_overflow_retains_node_index() {
        assert_eq!(
            checked_output_sum(usize::MAX, 1, 7),
            Err(XmlElementSerializationError::LengthOverflow { node_index: 7 })
        );
    }

    #[test]
    fn lslc_001g_traversal_stack_allocation_failure_retains_requested_count() {
        let mut stack: Vec<TraversalFrame> = Vec::new();
        assert_eq!(
            reserve_traversal_stack(&mut stack, usize::MAX),
            Err(
                XmlElementSerializationError::TraversalStackAllocationFailed {
                    requested: usize::MAX,
                }
            )
        );
    }

    #[test]
    fn lslc_001g_output_allocation_failure_retains_requested_bytes() {
        let mut output = String::new();
        assert_eq!(
            reserve_output(&mut output, usize::MAX),
            Err(XmlElementSerializationError::OutputAllocationFailed {
                requested: usize::MAX,
            })
        );
    }
}
