// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    StreamInfoStaticXml, XmlElementNodeInput, XmlElementTree, XmlElementTreeError,
    XmlElementTreeLimits,
};
use core::fmt;

const INFO_ROOT_INDEX: usize = 0;
const STATIC_NODE_COUNT: usize = 7;
const DESCRIPTION_ROOT_NAME: &str = "desc";

/// One bounded `info` element tree containing static fields and an explicit description.
///
/// Construction accepts only a separately validated element tree whose root is
/// the container `desc`. It does not reinterpret arbitrary metadata roots or
/// add declaration, whitespace, volatile-field, endpoint, or runtime policy.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoDescriptionXml {
    limits: XmlElementTreeLimits,
    tree: XmlElementTree,
}

impl StreamInfoDescriptionXml {
    /// Moves accepted static and description trees into one target-limited tree.
    ///
    /// The explicit description-root contract and exact total node count are
    /// checked before one exact merged-arena reserve. Every description parent
    /// is remapped by the fixed static-node offset without recursion.
    pub fn compose(
        static_xml: StreamInfoStaticXml,
        description: XmlElementTree,
        limits: XmlElementTreeLimits,
    ) -> Result<Self, StreamInfoDescriptionXmlError> {
        let description_root = &description.nodes()[0];
        let Some(root_name) = description_root.value().as_container() else {
            return Err(StreamInfoDescriptionXmlError::DescriptionRootNotContainer);
        };
        if root_name.as_str() != DESCRIPTION_ROOT_NAME {
            return Err(StreamInfoDescriptionXmlError::DescriptionRootNameMismatch);
        }

        let static_count = static_xml.tree().nodes().len();
        debug_assert_eq!(static_count, STATIC_NODE_COUNT);
        let description_count = description.nodes().len();
        let total = static_count.checked_add(description_count).ok_or(
            StreamInfoDescriptionXmlError::NodeCountOverflow {
                static_count,
                description_count,
            },
        )?;
        if total > limits.max_nodes() {
            return Err(StreamInfoDescriptionXmlError::ElementTree(
                XmlElementTreeError::NodeLimitExceeded {
                    expected_max: limits.max_nodes(),
                    actual: total,
                },
            ));
        }

        let mut merged = Vec::new();
        merged.try_reserve_exact(total).map_err(|_| {
            StreamInfoDescriptionXmlError::MergedAllocationFailed { requested: total }
        })?;
        merged.extend(static_xml.into_tree().into_nodes());

        for (node_index, node) in description.into_nodes().into_iter().enumerate() {
            let (parent_index, value) = node.into_parts();
            let remapped_parent = if node_index == 0 {
                Some(INFO_ROOT_INDEX)
            } else {
                let parent_index = parent_index.expect("accepted non-root description node");
                Some(static_count.checked_add(parent_index).ok_or(
                    StreamInfoDescriptionXmlError::ParentIndexOverflow {
                        node_index,
                        parent_index,
                        offset: static_count,
                    },
                )?)
            };
            merged.push(XmlElementNodeInput::new(remapped_parent, value));
        }

        let tree = XmlElementTree::new(limits, merged)
            .map_err(StreamInfoDescriptionXmlError::ElementTree)?;
        Ok(Self { limits, tree })
    }

    /// Returns the final element-tree limits.
    #[must_use]
    pub const fn limits(&self) -> XmlElementTreeLimits {
        self.limits
    }

    /// Returns the composed element tree.
    #[must_use]
    pub const fn tree(&self) -> &XmlElementTree {
        &self.tree
    }

    /// Returns the composed tree without replacing its node arena.
    #[must_use]
    pub fn into_tree(self) -> XmlElementTree {
        self.tree
    }
}

/// Deterministic rejection from explicit description-tree composition.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamInfoDescriptionXmlError {
    /// The separately accepted description root was a leaf.
    DescriptionRootNotContainer,
    /// The separately accepted container root was not named exactly `desc`.
    DescriptionRootNameMismatch,
    /// Static and description node counts could not be added.
    NodeCountOverflow {
        /// The fixed accepted static-tree node count.
        static_count: usize,
        /// The accepted description-tree node count.
        description_count: usize,
    },
    /// The exact merged node-arena reserve failed.
    MergedAllocationFailed {
        /// The exact requested node capacity.
        requested: usize,
    },
    /// Adding the static-node offset to a description parent overflowed.
    ParentIndexOverflow {
        /// The description-local node index.
        node_index: usize,
        /// The description-local parent index.
        parent_index: usize,
        /// The fixed static-node offset.
        offset: usize,
    },
    /// The final accepted hierarchy contract rejected the merged tree.
    ElementTree(XmlElementTreeError),
}

impl fmt::Display for StreamInfoDescriptionXmlError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "stream-info description XML rejected input: {self:?}"
        )
    }
}

impl std::error::Error for StreamInfoDescriptionXmlError {}

#[cfg(test)]
mod tests {
    use super::{StreamInfoDescriptionXml, StreamInfoDescriptionXmlError, STATIC_NODE_COUNT};
    use crate::{
        project_metadata_tree_to_xml_element_tree, ChannelFormat, MetadataNodeInput, MetadataTree,
        MetadataTreeLimits, MetadataXmlProjectionLimits, NominalSampleRate, StreamDefinition,
        StreamDescriptor, StreamDescriptorLimits, StreamInfoStaticFields, StreamInfoStaticXml,
        StreamInfoStaticXmlLimits, XmlCharacterDataLimit, XmlElementSerialization,
        XmlElementSerializationLimit, XmlElementTreeError, XmlElementTreeLimits, XmlNameLimit,
        XmlTextLimit,
    };

    fn definition(
        name: &str,
        kind: &str,
        source_id: &str,
        channel_count: usize,
        rate: NominalSampleRate,
        format: ChannelFormat,
    ) -> StreamDefinition {
        let descriptor = StreamDescriptor::new(
            StreamDescriptorLimits::new(64, 64, 64, 8).unwrap(),
            name.to_owned(),
            Some(kind.to_owned()),
            Some(source_id.to_owned()),
            channel_count,
            rate,
            format,
        )
        .unwrap();
        let metadata = MetadataTree::new(
            MetadataTreeLimits::new(1, 1, 1, 16, 16).unwrap(),
            vec![MetadataNodeInput::new(None, "unused".to_owned(), None)],
        )
        .unwrap();
        StreamDefinition::new(descriptor, metadata)
    }

    fn static_xml(definition: &StreamDefinition) -> StreamInfoStaticXml {
        let fields = StreamInfoStaticFields::new(definition);
        StreamInfoStaticXml::compose(
            &fields,
            StreamInfoStaticXmlLimits::new(
                XmlNameLimit::new(32).unwrap(),
                XmlTextLimit::new(64).unwrap(),
                XmlCharacterDataLimit::new(256).unwrap(),
                XmlElementTreeLimits::new(7, 2, 6, 512).unwrap(),
            ),
        )
        .unwrap()
    }

    fn description(nodes: Vec<MetadataNodeInput>) -> crate::XmlElementTree {
        let count = nodes.len();
        let metadata =
            MetadataTree::new(MetadataTreeLimits::new(count, 8, 8, 32, 64).unwrap(), nodes)
                .unwrap();
        project_metadata_tree_to_xml_element_tree(
            metadata,
            MetadataXmlProjectionLimits::new(
                XmlNameLimit::new(32).unwrap(),
                XmlTextLimit::new(64).unwrap(),
                XmlCharacterDataLimit::new(256).unwrap(),
                XmlElementTreeLimits::new(count, 8, 8, 1024).unwrap(),
            ),
        )
        .unwrap()
    }

    fn empty_description() -> crate::XmlElementTree {
        description(vec![MetadataNodeInput::new(None, "desc".to_owned(), None)])
    }

    fn target_limits(nodes: usize) -> XmlElementTreeLimits {
        XmlElementTreeLimits::new(nodes, 8, 7, 4096).unwrap()
    }

    #[test]
    fn lslc_001n_seven_observed_cases_place_desc_after_static_fields_exactly() {
        let cases = [
            (
                "neutral-float32",
                "",
                "",
                1,
                NominalSampleRate::irregular(),
                ChannelFormat::Float32,
            ),
            (
                "neutral-double64",
                "measurement",
                "source-double64",
                2,
                NominalSampleRate::regular_hz(100.0).unwrap(),
                ChannelFormat::Double64,
            ),
            (
                "unicode-Ω-中-&-<-greater->",
                "text-&-<-greater->-\"-'",
                "source-雪-&-<-greater->",
                3,
                NominalSampleRate::regular_hz(59.94).unwrap(),
                ChannelFormat::String,
            ),
            (
                "neutral-int32",
                "integer",
                "source-int32",
                4,
                NominalSampleRate::regular_hz(1.0).unwrap(),
                ChannelFormat::Int32,
            ),
            (
                "neutral-int16",
                "integer",
                "source-int16",
                5,
                NominalSampleRate::regular_hz(256.5).unwrap(),
                ChannelFormat::Int16,
            ),
            (
                "neutral-int8",
                "integer",
                "source-int8",
                6,
                NominalSampleRate::irregular(),
                ChannelFormat::Int8,
            ),
            (
                "neutral-int64",
                "integer",
                "source-int64",
                7,
                NominalSampleRate::regular_hz(1_000_000.25).unwrap(),
                ChannelFormat::Int64,
            ),
        ];
        for (case_index, (name, kind, source, count, rate, format)) in cases.into_iter().enumerate()
        {
            let definition = definition(name, kind, source, count, rate, format);
            let static_xml = static_xml(&definition);
            let static_text = XmlElementSerialization::serialize(
                XmlElementSerializationLimit::new(1024).unwrap(),
                static_xml.tree(),
            )
            .unwrap()
            .into_string();
            let description = if case_index == 2 {
                description(vec![
                    MetadataNodeInput::new(None, "desc".to_owned(), None),
                    MetadataNodeInput::new(Some(0), "ordered".to_owned(), None),
                    MetadataNodeInput::new(
                        Some(1),
                        "first".to_owned(),
                        Some("alpha-α-&-<-greater->-\"-'".to_owned()),
                    ),
                    MetadataNodeInput::new(
                        Some(1),
                        "second".to_owned(),
                        Some("beta-β-]]>".to_owned()),
                    ),
                    MetadataNodeInput::new(Some(1), "nested".to_owned(), None),
                    MetadataNodeInput::new(
                        Some(4),
                        "third".to_owned(),
                        Some("tail-尾-&-<-greater->".to_owned()),
                    ),
                ])
            } else {
                empty_description()
            };
            let node_count = STATIC_NODE_COUNT + description.nodes().len();
            let composed = StreamInfoDescriptionXml::compose(
                static_xml,
                description,
                target_limits(node_count),
            )
            .unwrap();
            assert_eq!(composed.tree().nodes()[7].parent_index(), Some(0));
            assert_eq!(
                composed.tree().nodes()[7]
                    .value()
                    .as_container()
                    .unwrap()
                    .as_str(),
                "desc"
            );
            let actual = XmlElementSerialization::serialize(
                XmlElementSerializationLimit::new(2048).unwrap(),
                composed.tree(),
            )
            .unwrap();
            let desc = if case_index == 2 {
                "<desc><ordered><first>alpha-α-&amp;-&lt;-greater-&gt;-\"-'</first><second>beta-β-]]&gt;</second><nested><third>tail-尾-&amp;-&lt;-greater-&gt;</third></nested></ordered></desc>"
            } else {
                "<desc></desc>"
            };
            assert_eq!(
                actual.as_str(),
                static_text.replace("</info>", &format!("{desc}</info>"))
            );
        }
    }

    #[test]
    fn lslc_001n_root_contract_rejects_leaf_and_non_desc_container() {
        let definition = definition(
            "n",
            "",
            "",
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
        );
        let leaf = description(vec![MetadataNodeInput::new(
            None,
            "desc".to_owned(),
            Some(String::new()),
        )]);
        assert_eq!(
            StreamInfoDescriptionXml::compose(static_xml(&definition), leaf, target_limits(8)),
            Err(StreamInfoDescriptionXmlError::DescriptionRootNotContainer)
        );
        let other = description(vec![MetadataNodeInput::new(None, "other".to_owned(), None)]);
        assert_eq!(
            StreamInfoDescriptionXml::compose(static_xml(&definition), other, target_limits(8)),
            Err(StreamInfoDescriptionXmlError::DescriptionRootNameMismatch)
        );
    }

    #[test]
    fn lslc_001n_none_and_some_empty_survive_remap_with_exact_parent_order() {
        let definition = definition(
            "n",
            "",
            "",
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
        );
        let desc = description(vec![
            MetadataNodeInput::new(None, "desc".to_owned(), None),
            MetadataNodeInput::new(Some(0), "group".to_owned(), None),
            MetadataNodeInput::new(Some(1), "empty".to_owned(), Some(String::new())),
            MetadataNodeInput::new(Some(0), "tail".to_owned(), Some("v".to_owned())),
        ]);
        let composed =
            StreamInfoDescriptionXml::compose(static_xml(&definition), desc, target_limits(11))
                .unwrap();
        let nodes = composed.tree().nodes();
        assert_eq!(nodes[8].parent_index(), Some(7));
        assert!(nodes[8].value().as_container().is_some());
        assert_eq!(nodes[9].parent_index(), Some(8));
        assert_eq!(
            nodes[9]
                .value()
                .as_leaf()
                .unwrap()
                .character_data()
                .as_str(),
            ""
        );
        assert_eq!(nodes[10].parent_index(), Some(7));
        assert_eq!(
            nodes[10]
                .value()
                .as_leaf()
                .unwrap()
                .character_data()
                .as_str(),
            "v"
        );
    }

    #[test]
    fn lslc_001n_target_node_bound_rejects_before_merged_allocation() {
        let definition = definition(
            "n",
            "",
            "",
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
        );
        assert_eq!(
            StreamInfoDescriptionXml::compose(
                static_xml(&definition),
                empty_description(),
                target_limits(7)
            ),
            Err(StreamInfoDescriptionXmlError::ElementTree(
                XmlElementTreeError::NodeLimitExceeded {
                    expected_max: 7,
                    actual: 8
                }
            ))
        );
    }

    #[test]
    fn lslc_001n_component_allocations_move_and_consuming_tree_preserves_arena() {
        let definition = definition(
            "n",
            "",
            "",
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
        );
        let static_xml = static_xml(&definition);
        let static_name_pointer = static_xml.tree().nodes()[1]
            .value()
            .as_leaf()
            .unwrap()
            .name()
            .as_str()
            .as_ptr();
        let desc = description(vec![
            MetadataNodeInput::new(None, "desc".to_owned(), None),
            MetadataNodeInput::new(Some(0), "leaf".to_owned(), Some("value".to_owned())),
        ]);
        let desc_name_pointer = desc.nodes()[1]
            .value()
            .as_leaf()
            .unwrap()
            .name()
            .as_str()
            .as_ptr();
        let limits = target_limits(9);
        let composed = StreamInfoDescriptionXml::compose(static_xml, desc, limits).unwrap();
        assert_eq!(composed.limits(), limits);
        assert_eq!(
            composed.tree().nodes()[1]
                .value()
                .as_leaf()
                .unwrap()
                .name()
                .as_str()
                .as_ptr(),
            static_name_pointer
        );
        assert_eq!(
            composed.tree().nodes()[8]
                .value()
                .as_leaf()
                .unwrap()
                .name()
                .as_str()
                .as_ptr(),
            desc_name_pointer
        );
        let arena_pointer = composed.tree().nodes().as_ptr();
        let tree = composed.into_tree();
        assert_eq!(tree.nodes().as_ptr(), arena_pointer);
    }
}
