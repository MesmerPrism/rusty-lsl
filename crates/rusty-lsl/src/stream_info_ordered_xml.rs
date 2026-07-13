// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    StreamInfoDescriptionXml, StreamInfoVolatileXml, XmlElementNodeInput, XmlElementTree,
    XmlElementTreeError, XmlElementTreeLimits,
};
use core::fmt;

const INFO_ROOT_NAME: &str = "info";
const DESCRIPTION_ROOT_NAME: &str = "desc";
const STATIC_NODE_COUNT: usize = 7;
const DESCRIPTION_ROOT_INDEX: usize = 7;
const VOLATILE_NODE_COUNT: usize = 12;
const VOLATILE_LEAF_COUNT: usize = VOLATILE_NODE_COUNT - 1;
const STATIC_FIELD_NAMES: [&str; 6] = [
    "name",
    "type",
    "channel_count",
    "channel_format",
    "source_id",
    "nominal_srate",
];
const VOLATILE_FIELD_NAMES: [&str; 11] = [
    "version",
    "created_at",
    "uid",
    "session_id",
    "hostname",
    "v4address",
    "v4data_port",
    "v4service_port",
    "v6address",
    "v6data_port",
    "v6service_port",
];

/// One bounded local `info` element tree in static, volatile, then `desc` order.
///
/// This accepted element hierarchy has no XML declaration, observed whitespace,
/// self-closing, complete-document, provider, endpoint, runtime, or authority meaning.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoOrderedXml {
    limits: XmlElementTreeLimits,
    tree: XmlElementTree,
}

impl StreamInfoOrderedXml {
    /// Consumes accepted static-plus-description and volatile component trees.
    ///
    /// Fixed component shapes and the exact target node bound are checked before
    /// one exact merged-arena reserve. Component values move without cloning;
    /// only parents inside the description subtree receive the eleven-node offset.
    pub fn compose(
        static_description: StreamInfoDescriptionXml,
        volatile: StreamInfoVolatileXml,
        limits: XmlElementTreeLimits,
    ) -> Result<Self, StreamInfoOrderedXmlError> {
        validate_static_description_shape(static_description.tree())?;
        validate_volatile_shape(volatile.tree())?;

        let static_description_count = static_description.tree().nodes().len();
        let volatile_count = volatile.tree().nodes().len();
        let total = static_description_count
            .checked_add(volatile_count)
            .and_then(|sum| sum.checked_sub(1))
            .ok_or(StreamInfoOrderedXmlError::NodeCountOverflow {
                static_description_count,
                volatile_count,
            })?;
        if total > limits.max_nodes() {
            return Err(StreamInfoOrderedXmlError::ElementTree(
                XmlElementTreeError::NodeLimitExceeded {
                    expected_max: limits.max_nodes(),
                    actual: total,
                },
            ));
        }

        let mut merged = Vec::new();
        merged
            .try_reserve_exact(total)
            .map_err(|_| StreamInfoOrderedXmlError::MergedAllocationFailed { requested: total })?;

        let mut static_description_nodes = static_description.into_tree().into_nodes();
        let volatile_nodes = volatile.into_tree().into_nodes();
        merged.extend(static_description_nodes.drain(..STATIC_NODE_COUNT));
        merged.extend(volatile_nodes.into_iter().skip(1));

        for (node_index, node) in static_description_nodes.into_iter().enumerate() {
            let original_index = node_index + DESCRIPTION_ROOT_INDEX;
            let (parent_index, value) = node.into_parts();
            let remapped_parent = match parent_index {
                Some(0) => Some(0),
                Some(parent_index) => Some(parent_index.checked_add(VOLATILE_LEAF_COUNT).ok_or(
                    StreamInfoOrderedXmlError::ParentIndexOverflow {
                        node_index: original_index,
                        parent_index,
                        offset: VOLATILE_LEAF_COUNT,
                    },
                )?),
                None => None,
            };
            merged.push(XmlElementNodeInput::new(remapped_parent, value));
        }

        let tree =
            XmlElementTree::new(limits, merged).map_err(StreamInfoOrderedXmlError::ElementTree)?;
        Ok(Self { limits, tree })
    }

    /// Returns the final element-tree limits.
    #[must_use]
    pub const fn limits(&self) -> XmlElementTreeLimits {
        self.limits
    }

    /// Returns the ordered local element tree.
    #[must_use]
    pub const fn tree(&self) -> &XmlElementTree {
        &self.tree
    }

    /// Returns the ordered tree without replacing its node arena.
    #[must_use]
    pub fn into_tree(self) -> XmlElementTree {
        self.tree
    }
}

fn validate_static_description_shape(
    tree: &XmlElementTree,
) -> Result<(), StreamInfoOrderedXmlError> {
    let nodes = tree.nodes();
    if nodes.len() < STATIC_NODE_COUNT + 1 {
        return Err(StreamInfoOrderedXmlError::StaticDescriptionShape);
    }
    if nodes[0].parent_index().is_some()
        || nodes[0]
            .value()
            .as_container()
            .map_or(true, |name| name.as_str() != INFO_ROOT_NAME)
    {
        return Err(StreamInfoOrderedXmlError::StaticDescriptionShape);
    }
    for (node, expected_name) in nodes[1..STATIC_NODE_COUNT].iter().zip(STATIC_FIELD_NAMES) {
        if node.parent_index() != Some(0)
            || node
                .value()
                .as_leaf()
                .map_or(true, |leaf| leaf.name().as_str() != expected_name)
        {
            return Err(StreamInfoOrderedXmlError::StaticDescriptionShape);
        }
    }
    let description_root = &nodes[DESCRIPTION_ROOT_INDEX];
    if description_root.parent_index() != Some(0)
        || description_root
            .value()
            .as_container()
            .map_or(true, |name| name.as_str() != DESCRIPTION_ROOT_NAME)
    {
        return Err(StreamInfoOrderedXmlError::StaticDescriptionShape);
    }
    Ok(())
}

fn validate_volatile_shape(tree: &XmlElementTree) -> Result<(), StreamInfoOrderedXmlError> {
    let nodes = tree.nodes();
    if nodes.len() != VOLATILE_NODE_COUNT
        || nodes[0].parent_index().is_some()
        || nodes[0]
            .value()
            .as_container()
            .map_or(true, |name| name.as_str() != INFO_ROOT_NAME)
    {
        return Err(StreamInfoOrderedXmlError::VolatileShape);
    }
    for (node, expected_name) in nodes[1..].iter().zip(VOLATILE_FIELD_NAMES) {
        if node.parent_index() != Some(0)
            || node
                .value()
                .as_leaf()
                .map_or(true, |leaf| leaf.name().as_str() != expected_name)
        {
            return Err(StreamInfoOrderedXmlError::VolatileShape);
        }
    }
    Ok(())
}

/// Deterministic rejection from ordered accepted-element composition.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamInfoOrderedXmlError {
    /// The accepted static-plus-description component violated its fixed contract.
    StaticDescriptionShape,
    /// The accepted volatile component violated its fixed contract.
    VolatileShape,
    /// The two component node counts could not form one root-sharing total.
    NodeCountOverflow {
        /// Nodes in the static-plus-description component.
        static_description_count: usize,
        /// Nodes in the volatile component, including its discarded duplicate root.
        volatile_count: usize,
    },
    /// The exact merged node-arena reserve failed.
    MergedAllocationFailed {
        /// The exact requested node capacity.
        requested: usize,
    },
    /// Adding the fixed volatile offset to a description parent overflowed.
    ParentIndexOverflow {
        /// The node's index in the incoming static-plus-description tree.
        node_index: usize,
        /// The incoming parent index.
        parent_index: usize,
        /// The fixed volatile-leaf offset.
        offset: usize,
    },
    /// The final hierarchy contract rejected the merged tree.
    ElementTree(XmlElementTreeError),
}

impl fmt::Display for StreamInfoOrderedXmlError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "ordered stream-info XML rejected input: {self:?}"
        )
    }
}

impl std::error::Error for StreamInfoOrderedXmlError {}

#[cfg(test)]
mod tests {
    use super::{StreamInfoOrderedXml, StreamInfoOrderedXmlError, VOLATILE_LEAF_COUNT};
    use crate::{
        project_metadata_tree_to_xml_element_tree, ChannelFormat, MetadataNodeInput, MetadataTree,
        MetadataTreeLimits, MetadataXmlProjectionLimits, NominalSampleRate, StreamDefinition,
        StreamDescriptor, StreamDescriptorLimits, StreamInfoDescriptionXml, StreamInfoStaticFields,
        StreamInfoStaticXml, StreamInfoStaticXmlLimits, StreamInfoVolatileFieldInput,
        StreamInfoVolatileFieldLimits, StreamInfoVolatileFields, StreamInfoVolatileXml,
        StreamInfoVolatileXmlLimits, XmlCharacterDataLimit, XmlElementSerialization,
        XmlElementSerializationLimit, XmlElementTreeError, XmlElementTreeLimits, XmlNameLimit,
        XmlTextLimit,
    };

    fn static_description(
        name: &str,
        kind: &str,
        source_id: &str,
        channel_count: usize,
        rate: NominalSampleRate,
        format: ChannelFormat,
        nested: bool,
    ) -> StreamInfoDescriptionXml {
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
        let definition = StreamDefinition::new(descriptor, metadata);
        let fields = StreamInfoStaticFields::new(&definition);
        let static_xml = StreamInfoStaticXml::compose(
            &fields,
            StreamInfoStaticXmlLimits::new(
                XmlNameLimit::new(32).unwrap(),
                XmlTextLimit::new(64).unwrap(),
                XmlCharacterDataLimit::new(256).unwrap(),
                XmlElementTreeLimits::new(7, 2, 6, 512).unwrap(),
            ),
        )
        .unwrap();
        let metadata_nodes = if nested {
            vec![
                MetadataNodeInput::new(None, "desc".to_owned(), None),
                MetadataNodeInput::new(Some(0), "group".to_owned(), None),
                MetadataNodeInput::new(
                    Some(1),
                    "leaf".to_owned(),
                    Some("value-雪-&-<->".to_owned()),
                ),
            ]
        } else {
            vec![MetadataNodeInput::new(None, "desc".to_owned(), None)]
        };
        let description_count = metadata_nodes.len();
        let metadata = MetadataTree::new(
            MetadataTreeLimits::new(description_count, 4, 4, 32, 64).unwrap(),
            metadata_nodes,
        )
        .unwrap();
        let description = project_metadata_tree_to_xml_element_tree(
            metadata,
            MetadataXmlProjectionLimits::new(
                XmlNameLimit::new(32).unwrap(),
                XmlTextLimit::new(64).unwrap(),
                XmlCharacterDataLimit::new(256).unwrap(),
                XmlElementTreeLimits::new(description_count, 4, 4, 1024).unwrap(),
            ),
        )
        .unwrap();
        StreamInfoDescriptionXml::compose(
            static_xml,
            description,
            XmlElementTreeLimits::new(7 + description_count, 5, 7, 4096).unwrap(),
        )
        .unwrap()
    }

    fn volatile(values: [&str; 11]) -> StreamInfoVolatileXml {
        let [a, b, c, d, e, f, g, h, i, j, k] = values;
        let fields = StreamInfoVolatileFields::new(
            StreamInfoVolatileFieldLimits::new(64, 64, 64).unwrap(),
            StreamInfoVolatileFieldInput::new(
                a.to_owned(),
                b.to_owned(),
                c.to_owned(),
                d.to_owned(),
                e.to_owned(),
                f.to_owned(),
                g.to_owned(),
                h.to_owned(),
                i.to_owned(),
                j.to_owned(),
                k.to_owned(),
            ),
        )
        .unwrap();
        StreamInfoVolatileXml::compose(
            &fields,
            StreamInfoVolatileXmlLimits::new(
                XmlNameLimit::new(32).unwrap(),
                XmlTextLimit::new(64).unwrap(),
                XmlCharacterDataLimit::new(256).unwrap(),
                XmlElementTreeLimits::new(12, 2, 11, 2048).unwrap(),
            ),
        )
        .unwrap()
    }

    fn serialize(tree: &crate::XmlElementTree) -> String {
        XmlElementSerialization::serialize(XmlElementSerializationLimit::new(8192).unwrap(), tree)
            .unwrap()
            .into_string()
    }

    #[test]
    fn lslc_001q_seven_cases_preserve_exact_static_volatile_desc_order() {
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
            let static_description =
                static_description(name, kind, source, count, rate, format, case_index == 2);
            let volatile = volatile([
                "1.18",
                "12.5",
                "uid<&>",
                "session",
                "host",
                "127.0.0.1",
                "1",
                "2",
                "::1",
                "3",
                "4",
            ]);
            let static_description_text = serialize(static_description.tree());
            let volatile_text = serialize(volatile.tree());
            let volatile_inner = volatile_text
                .strip_prefix("<info>")
                .unwrap()
                .strip_suffix("</info>")
                .unwrap();
            let desc_offset = static_description_text.find("<desc>").unwrap();
            let expected = format!(
                "{}{}{}",
                &static_description_text[..desc_offset],
                volatile_inner,
                &static_description_text[desc_offset..]
            );
            let total = static_description.tree().nodes().len() + VOLATILE_LEAF_COUNT;
            let ordered = StreamInfoOrderedXml::compose(
                static_description,
                volatile,
                XmlElementTreeLimits::new(total, 6, 18, 8192).unwrap(),
            )
            .unwrap();
            assert_eq!(serialize(ordered.tree()), expected);
        }
    }

    #[test]
    fn lslc_001q_only_description_parents_receive_volatile_offset() {
        let static_description = static_description(
            "n",
            "",
            "",
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
            true,
        );
        let volatile = volatile([""; 11]);
        let total = static_description.tree().nodes().len() + VOLATILE_LEAF_COUNT;
        let ordered = StreamInfoOrderedXml::compose(
            static_description,
            volatile,
            XmlElementTreeLimits::new(total, 6, 18, 8192).unwrap(),
        )
        .unwrap();
        let nodes = ordered.tree().nodes();
        for node in &nodes[1..18] {
            assert_eq!(node.parent_index(), Some(0));
        }
        assert_eq!(nodes[18].parent_index(), Some(0));
        assert_eq!(nodes[19].parent_index(), Some(18));
        assert_eq!(nodes[20].parent_index(), Some(19));
    }

    #[test]
    fn lslc_001q_component_values_move_without_cloning() {
        let static_description = static_description(
            "n",
            "",
            "",
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
            true,
        );
        let static_pointer = static_description.tree().nodes()[1]
            .value()
            .as_leaf()
            .unwrap()
            .name()
            .as_str()
            .as_ptr();
        let desc_pointer = static_description.tree().nodes()[9]
            .value()
            .as_leaf()
            .unwrap()
            .character_data()
            .as_str()
            .as_ptr();
        let volatile = volatile(["v", "", "", "", "", "", "", "", "", "", ""]);
        let volatile_pointer = volatile.tree().nodes()[1]
            .value()
            .as_leaf()
            .unwrap()
            .character_data()
            .as_str()
            .as_ptr();
        let total = static_description.tree().nodes().len() + VOLATILE_LEAF_COUNT;
        let limits = XmlElementTreeLimits::new(total, 6, 18, 8192).unwrap();
        let ordered = StreamInfoOrderedXml::compose(static_description, volatile, limits).unwrap();
        assert_eq!(ordered.limits(), limits);
        assert_eq!(
            ordered.tree().nodes()[1]
                .value()
                .as_leaf()
                .unwrap()
                .name()
                .as_str()
                .as_ptr(),
            static_pointer
        );
        assert_eq!(
            ordered.tree().nodes()[7]
                .value()
                .as_leaf()
                .unwrap()
                .character_data()
                .as_str()
                .as_ptr(),
            volatile_pointer
        );
        assert_eq!(
            ordered.tree().nodes()[20]
                .value()
                .as_leaf()
                .unwrap()
                .character_data()
                .as_str()
                .as_ptr(),
            desc_pointer
        );
        let arena_pointer = ordered.tree().nodes().as_ptr();
        assert_eq!(ordered.into_tree().nodes().as_ptr(), arena_pointer);
    }

    #[test]
    fn lslc_001q_target_bound_rejects_before_merged_allocation() {
        let static_description = static_description(
            "n",
            "",
            "",
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
            false,
        );
        let volatile = volatile([""; 11]);
        assert_eq!(
            StreamInfoOrderedXml::compose(
                static_description,
                volatile,
                XmlElementTreeLimits::new(18, 3, 18, 8192).unwrap(),
            ),
            Err(StreamInfoOrderedXmlError::ElementTree(
                XmlElementTreeError::NodeLimitExceeded {
                    expected_max: 18,
                    actual: 19
                }
            ))
        );
    }
}
