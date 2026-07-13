// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    MetadataTree, XmlCharacterData, XmlCharacterDataError, XmlCharacterDataLimit, XmlElementName,
    XmlElementNodeInput, XmlElementNodeValue, XmlElementTree, XmlElementTreeError,
    XmlElementTreeLimits, XmlLeafElement, XmlNameError, XmlNameLimit, XmlText, XmlTextError,
    XmlTextLimit,
};
use core::fmt;

/// Caller-selected limits for the one-way metadata-to-element-tree projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MetadataXmlProjectionLimits {
    name: XmlNameLimit,
    text: XmlTextLimit,
    character_data: XmlCharacterDataLimit,
    element_tree: XmlElementTreeLimits,
}

impl MetadataXmlProjectionLimits {
    /// Composes already validated component and target-arena limits without defaults.
    #[must_use]
    pub const fn new(
        name: XmlNameLimit,
        text: XmlTextLimit,
        character_data: XmlCharacterDataLimit,
        element_tree: XmlElementTreeLimits,
    ) -> Self {
        Self {
            name,
            text,
            character_data,
            element_tree,
        }
    }

    /// Returns the XML element-name limit.
    #[must_use]
    pub const fn name(self) -> XmlNameLimit {
        self.name
    }

    /// Returns the XML text limit applied to each present metadata value.
    #[must_use]
    pub const fn text(self) -> XmlTextLimit {
        self.text
    }

    /// Returns the represented character-data byte limit applied to each leaf.
    #[must_use]
    pub const fn character_data(self) -> XmlCharacterDataLimit {
        self.character_data
    }

    /// Returns the final XML element-tree limits.
    #[must_use]
    pub const fn element_tree(self) -> XmlElementTreeLimits {
        self.element_tree
    }
}

/// Deterministic failure from the one-way metadata-to-element-tree projection.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MetadataXmlProjectionError {
    /// A child named a metadata node with a present value as its parent.
    ValueBearingParent {
        /// The value-bearing parent index.
        parent_index: usize,
        /// The first child in caller order that exposed the unsupported shape.
        first_child_index: usize,
    },
    /// The distinct output candidate arena could not be reserved exactly.
    OutputAllocationFailed {
        /// The exact candidate-node capacity requested.
        requested: usize,
    },
    /// One metadata name was rejected by the XML name contract.
    Name {
        /// The source and target node index.
        node_index: usize,
        /// The unchanged XML name rejection.
        error: XmlNameError,
    },
    /// One present metadata value was rejected by the XML text contract.
    Text {
        /// The source and target node index.
        node_index: usize,
        /// The unchanged XML text rejection.
        error: XmlTextError,
    },
    /// One accepted XML text value could not be represented as character data.
    CharacterData {
        /// The source and target node index.
        node_index: usize,
        /// The unchanged character-data rejection.
        error: XmlCharacterDataError,
    },
    /// The projected candidate arena was rejected by the target hierarchy contract.
    ElementTree(XmlElementTreeError),
}

impl fmt::Display for MetadataXmlProjectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "metadata XML projection rejected input: {self:?}"
        )
    }
}

impl std::error::Error for MetadataXmlProjectionError {}

/// Consumes an accepted generic metadata tree into a local XML element hierarchy.
///
/// An absent value becomes a container and every present value, including an
/// empty value, becomes a leaf. This is a one-way local classification: it has
/// no decoding, round-trip, document, serialization, stream-info, or LSL field
/// meaning.
pub fn project_metadata_tree_to_xml_element_tree(
    source: MetadataTree,
    limits: MetadataXmlProjectionLimits,
) -> Result<XmlElementTree, MetadataXmlProjectionError> {
    let node_count = source.nodes().len();
    if node_count > limits.element_tree.max_nodes() {
        return Err(MetadataXmlProjectionError::ElementTree(
            XmlElementTreeError::NodeLimitExceeded {
                expected_max: limits.element_tree.max_nodes(),
                actual: node_count,
            },
        ));
    }

    for (first_child_index, node) in source.nodes().iter().enumerate().skip(1) {
        if let Some(parent_index) = node.parent_index() {
            if source.nodes()[parent_index].value().is_some() {
                return Err(MetadataXmlProjectionError::ValueBearingParent {
                    parent_index,
                    first_child_index,
                });
            }
        }
    }

    let mut output = Vec::new();
    reserve_output_nodes(&mut output, node_count)?;
    for (node_index, node) in source.into_nodes().into_iter().enumerate() {
        let (parent_index, name, value) = node.into_parts();
        let name = XmlElementName::new(limits.name, name)
            .map_err(|error| MetadataXmlProjectionError::Name { node_index, error })?;
        let value = match value {
            None => XmlElementNodeValue::Container(name),
            Some(value) => {
                let text = XmlText::new(limits.text, value)
                    .map_err(|error| MetadataXmlProjectionError::Text { node_index, error })?;
                let character_data =
                    XmlCharacterData::encode(limits.character_data, &text).map_err(|error| {
                        MetadataXmlProjectionError::CharacterData { node_index, error }
                    })?;
                XmlElementNodeValue::Leaf(XmlLeafElement::new(name, character_data))
            }
        };
        output.push(XmlElementNodeInput::new(parent_index, value));
    }

    XmlElementTree::new(limits.element_tree, output)
        .map_err(MetadataXmlProjectionError::ElementTree)
}

fn reserve_output_nodes(
    output: &mut Vec<XmlElementNodeInput>,
    requested: usize,
) -> Result<(), MetadataXmlProjectionError> {
    output
        .try_reserve_exact(requested)
        .map_err(|_| MetadataXmlProjectionError::OutputAllocationFailed { requested })
}

#[cfg(test)]
mod tests {
    use super::{
        project_metadata_tree_to_xml_element_tree, reserve_output_nodes,
        MetadataXmlProjectionError, MetadataXmlProjectionLimits,
    };
    use crate::{
        MetadataNodeInput, MetadataTree, MetadataTreeLimits, XmlCharacterDataError,
        XmlCharacterDataLimit, XmlElementNodeInput, XmlElementNodeValue, XmlElementTreeError,
        XmlElementTreeLimits, XmlNameError, XmlNameLimit, XmlTextError, XmlTextLimit,
    };

    fn source(nodes: Vec<MetadataNodeInput>) -> MetadataTree {
        MetadataTree::new(MetadataTreeLimits::new(32, 32, 32, 32, 32).unwrap(), nodes).unwrap()
    }

    fn node(parent: Option<usize>, name: &str, value: Option<&str>) -> MetadataNodeInput {
        MetadataNodeInput::new(parent, name.to_owned(), value.map(str::to_owned))
    }

    fn limits(
        name: usize,
        text: usize,
        data: usize,
        nodes: usize,
        depth: usize,
        children: usize,
        retained: usize,
    ) -> MetadataXmlProjectionLimits {
        MetadataXmlProjectionLimits::new(
            XmlNameLimit::new(name).unwrap(),
            XmlTextLimit::new(text).unwrap(),
            XmlCharacterDataLimit::new(data).unwrap(),
            XmlElementTreeLimits::new(nodes, depth, children, retained).unwrap(),
        )
    }

    fn roomy() -> MetadataXmlProjectionLimits {
        limits(32, 32, 256, 32, 32, 32, 1024)
    }

    #[test]
    fn lslc_001f_none_and_some_including_empty_project_in_order() {
        let tree = project_metadata_tree_to_xml_element_tree(
            source(vec![
                node(None, "root", None),
                node(Some(0), "empty-container", None),
                node(Some(0), "empty-leaf", Some("")),
                node(Some(1), "leaf", Some("A&B")),
            ]),
            roomy(),
        )
        .unwrap();
        assert_eq!(tree.nodes().len(), 4);
        assert_eq!(tree.nodes()[1].parent_index(), Some(0));
        assert!(tree.nodes()[1].value().as_container().is_some());
        assert_eq!(
            tree.nodes()[2]
                .value()
                .as_leaf()
                .unwrap()
                .character_data()
                .as_str(),
            ""
        );
        assert_eq!(
            tree.nodes()[3]
                .value()
                .as_leaf()
                .unwrap()
                .character_data()
                .as_str(),
            "A&amp;B"
        );
    }

    #[test]
    fn lslc_001f_childless_container_and_valued_leaf_root_are_accepted() {
        let container = project_metadata_tree_to_xml_element_tree(
            source(vec![node(None, "root", None)]),
            roomy(),
        )
        .unwrap();
        assert!(container.nodes()[0].value().as_container().is_some());

        let leaf = project_metadata_tree_to_xml_element_tree(
            source(vec![node(None, "root", Some(""))]),
            roomy(),
        )
        .unwrap();
        assert!(leaf.nodes()[0].value().as_leaf().is_some());
    }

    #[test]
    fn lslc_001f_first_child_of_value_bearing_parent_rejects_before_components() {
        let error = project_metadata_tree_to_xml_element_tree(
            source(vec![
                node(None, "1bad", Some("value")),
                node(Some(0), "also bad", None),
                node(Some(0), "later", None),
            ]),
            limits(1, 1, 1, 3, 3, 3, 3),
        );
        assert_eq!(
            error,
            Err(MetadataXmlProjectionError::ValueBearingParent {
                parent_index: 0,
                first_child_index: 1,
            })
        );
    }

    #[test]
    fn lslc_001f_target_node_bound_precedes_shape_and_allocation() {
        let error = project_metadata_tree_to_xml_element_tree(
            source(vec![
                node(None, "root", Some("value")),
                node(Some(0), "child", None),
            ]),
            limits(32, 32, 32, 1, 1, 1, 1),
        );
        assert_eq!(
            error,
            Err(MetadataXmlProjectionError::ElementTree(
                XmlElementTreeError::NodeLimitExceeded {
                    expected_max: 1,
                    actual: 2,
                }
            ))
        );
    }

    #[test]
    fn lslc_001f_name_then_text_then_character_data_errors_are_indexed() {
        assert_eq!(
            project_metadata_tree_to_xml_element_tree(
                source(vec![node(None, "1", Some("too long"))]),
                limits(32, 1, 1, 1, 1, 1, 32),
            ),
            Err(MetadataXmlProjectionError::Name {
                node_index: 0,
                error: XmlNameError::InvalidStart {
                    index: 0,
                    code_point: u32::from('1'),
                },
            })
        );
        assert_eq!(
            project_metadata_tree_to_xml_element_tree(
                source(vec![node(None, "root", Some("ab"))]),
                limits(32, 1, 32, 1, 1, 1, 32),
            ),
            Err(MetadataXmlProjectionError::Text {
                node_index: 0,
                error: XmlTextError::LimitExceeded {
                    expected_max: 1,
                    actual: 2,
                },
            })
        );
        assert_eq!(
            project_metadata_tree_to_xml_element_tree(
                source(vec![node(None, "root", Some("\0"))]),
                limits(32, 32, 32, 1, 1, 1, 32),
            ),
            Err(MetadataXmlProjectionError::Text {
                node_index: 0,
                error: XmlTextError::IllegalCharacter {
                    index: 0,
                    code_point: 0,
                },
            })
        );
        assert_eq!(
            project_metadata_tree_to_xml_element_tree(
                source(vec![node(None, "root", Some("&"))]),
                limits(32, 32, 4, 1, 1, 1, 32),
            ),
            Err(MetadataXmlProjectionError::CharacterData {
                node_index: 0,
                error: XmlCharacterDataError::LimitExceeded {
                    expected_max: 4,
                    required: 5,
                },
            })
        );
    }

    #[test]
    fn lslc_001f_later_component_failure_preserves_caller_index() {
        assert!(matches!(
            project_metadata_tree_to_xml_element_tree(
                source(vec![
                    node(None, "root", None),
                    node(Some(0), "bad name", None)
                ]),
                roomy(),
            ),
            Err(MetadataXmlProjectionError::Name { node_index: 1, .. })
        ));
    }

    #[test]
    fn lslc_001f_target_hierarchy_errors_are_delegated_unchanged() {
        assert_eq!(
            project_metadata_tree_to_xml_element_tree(
                source(vec![node(None, "root", None), node(Some(0), "child", None)]),
                limits(32, 32, 32, 2, 1, 1, 32),
            ),
            Err(MetadataXmlProjectionError::ElementTree(
                XmlElementTreeError::DepthLimitExceeded {
                    node_index: 1,
                    expected_max: 1,
                    actual: 2,
                }
            ))
        );
    }

    #[test]
    fn lslc_001f_name_allocations_parent_identity_and_distinct_arena_are_preserved() {
        let root_name = "root".to_owned();
        let leaf_name = "leaf".to_owned();
        let raw_value = "&".to_owned();
        let root_pointer = root_name.as_ptr();
        let leaf_pointer = leaf_name.as_ptr();
        let raw_value_pointer = raw_value.as_ptr();
        let source = MetadataTree::new(
            MetadataTreeLimits::new(2, 2, 1, 8, 8).unwrap(),
            vec![
                MetadataNodeInput::new(None, root_name, None),
                MetadataNodeInput::new(Some(0), leaf_name, Some(raw_value)),
            ],
        )
        .unwrap();
        let source_arena_pointer = source.nodes().as_ptr();
        let tree = project_metadata_tree_to_xml_element_tree(source, roomy()).unwrap();
        assert_ne!(
            tree.nodes().as_ptr().cast::<u8>(),
            source_arena_pointer.cast::<u8>()
        );
        assert_eq!(
            tree.nodes()[0]
                .value()
                .as_container()
                .unwrap()
                .as_str()
                .as_ptr(),
            root_pointer
        );
        let leaf = tree.nodes()[1].value().as_leaf().unwrap();
        assert_eq!(leaf.name().as_str().as_ptr(), leaf_pointer);
        assert_eq!(leaf.character_data().as_str(), "&amp;");
        assert_ne!(leaf.character_data().as_str().as_ptr(), raw_value_pointer);
        assert_eq!(tree.nodes()[1].parent_index(), Some(0));
    }

    #[test]
    fn lslc_001f_limits_have_no_hidden_default_policy() {
        let selected = limits(2, 3, 4, 5, 6, 7, 8);
        assert_eq!(selected.name().max_code_points(), 2);
        assert_eq!(selected.text().max_code_points(), 3);
        assert_eq!(selected.character_data().max_encoded_bytes(), 4);
        assert_eq!(selected.element_tree().max_nodes(), 5);
        assert_eq!(selected.element_tree().max_depth(), 6);
        assert_eq!(selected.element_tree().max_children_per_container(), 7);
        assert_eq!(selected.element_tree().max_retained_bytes(), 8);
    }

    #[test]
    fn lslc_001f_output_allocation_helper_returns_typed_error_without_panicking() {
        let mut output: Vec<XmlElementNodeInput> = Vec::new();
        assert_eq!(
            reserve_output_nodes(&mut output, usize::MAX),
            Err(MetadataXmlProjectionError::OutputAllocationFailed {
                requested: usize::MAX,
            })
        );
    }

    #[test]
    fn lslc_001f_retained_bytes_delegation_sees_represented_data() {
        assert_eq!(
            project_metadata_tree_to_xml_element_tree(
                source(vec![node(None, "r", Some("&"))]),
                limits(8, 8, 8, 1, 1, 1, 5),
            ),
            Err(MetadataXmlProjectionError::ElementTree(
                XmlElementTreeError::RetainedBytesLimitExceeded {
                    expected_max: 5,
                    actual: 6,
                }
            ))
        );
    }

    #[test]
    fn lslc_001f_consuming_tree_exposes_only_accepted_target_values() {
        let tree = project_metadata_tree_to_xml_element_tree(
            source(vec![
                node(None, "root", None),
                node(Some(0), "leaf", Some("é>")),
            ]),
            roomy(),
        )
        .unwrap();
        let nodes = tree.into_nodes();
        let (_, value) = nodes.into_iter().nth(1).unwrap().into_parts();
        match value {
            XmlElementNodeValue::Leaf(leaf) => {
                let (name, data) = leaf.into_parts();
                assert_eq!(name.into_string(), "leaf");
                assert_eq!(data.into_string(), "é&gt;");
            }
            XmlElementNodeValue::Container(_) => unreachable!(),
        }
    }
}
