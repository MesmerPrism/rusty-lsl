// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{MetadataTree, StreamDescriptor};

/// One validated stream descriptor composed with one validated metadata tree.
///
/// This local aggregate assigns no XML or runtime meaning to either component.
/// In particular, the generic metadata-tree root is not interpreted as an LSL
/// `desc` element.
#[derive(Clone, Debug, PartialEq)]
pub struct StreamDefinition {
    descriptor: StreamDescriptor,
    extended_metadata: MetadataTree,
}

impl StreamDefinition {
    /// Moves two already validated components directly into accepted state.
    #[must_use]
    pub fn new(descriptor: StreamDescriptor, extended_metadata: MetadataTree) -> Self {
        Self {
            descriptor,
            extended_metadata,
        }
    }

    /// Returns the unchanged validated stream descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> &StreamDescriptor {
        &self.descriptor
    }

    /// Returns the unchanged validated generic extended-metadata tree.
    #[must_use]
    pub const fn extended_metadata(&self) -> &MetadataTree {
        &self.extended_metadata
    }

    /// Returns both unchanged owned components.
    #[must_use]
    pub fn into_parts(self) -> (StreamDescriptor, MetadataTree) {
        (self.descriptor, self.extended_metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::StreamDefinition;
    use crate::{
        ChannelFormat, MetadataNodeInput, MetadataTree, MetadataTreeLimits, NominalSampleRate,
        StreamDescriptor, StreamDescriptorLimits,
    };

    fn descriptor(
        limits: StreamDescriptorLimits,
        rate: NominalSampleRate,
        format: ChannelFormat,
    ) -> StreamDescriptor {
        StreamDescriptor::new(
            limits,
            " 脑电🦀 ".to_owned(),
            Some("".to_owned()),
            Some("来源-ß ".to_owned()),
            3,
            rate,
            format,
        )
        .unwrap()
    }

    fn extended_metadata(limits: MetadataTreeLimits) -> MetadataTree {
        MetadataTree::new(
            limits,
            vec![
                MetadataNodeInput::new(None, "根🦀".to_owned(), None),
                MetadataNodeInput::new(Some(0), "通道".to_owned(), Some("".to_owned())),
                MetadataNodeInput::new(Some(0), "设备".to_owned(), Some(" μV ".to_owned())),
                MetadataNodeInput::new(Some(1), "标签ß".to_owned(), Some("左🙂".to_owned())),
            ],
        )
        .unwrap()
    }

    #[test]
    fn core_008_borrow_access_preserves_all_component_values() {
        let descriptor_limits = StreamDescriptorLimits::new(6, 1, 6, 3).unwrap();
        let metadata_limits = MetadataTreeLimits::new(4, 3, 2, 4, 4).unwrap();
        let rate_bits = 0x405e_dccd_9e83_e426;
        let definition = StreamDefinition::new(
            descriptor(
                descriptor_limits,
                NominalSampleRate::regular_hz(f64::from_bits(rate_bits)).unwrap(),
                ChannelFormat::Double64,
            ),
            extended_metadata(metadata_limits),
        );

        let descriptor = definition.descriptor();
        assert_eq!(descriptor.limits(), descriptor_limits);
        assert_eq!(descriptor.name(), " 脑电🦀 ");
        assert_eq!(descriptor.content_type(), Some(""));
        assert_eq!(descriptor.source_id(), Some("来源-ß "));
        assert_eq!(descriptor.channel_count(), 3);
        assert_eq!(descriptor.channel_format(), ChannelFormat::Double64);
        match descriptor.nominal_sample_rate() {
            NominalSampleRate::RegularHz(rate) => assert_eq!(rate.hz().to_bits(), rate_bits),
            NominalSampleRate::Irregular => panic!("regular rate changed form"),
        }

        let metadata = definition.extended_metadata();
        assert_eq!(metadata.limits(), metadata_limits);
        assert_eq!(metadata.nodes().len(), 4);
        assert_eq!(metadata.nodes()[0].parent_index(), None);
        assert_eq!(metadata.nodes()[0].name(), "根🦀");
        assert_eq!(metadata.nodes()[0].value(), None);
        assert_eq!(metadata.nodes()[1].parent_index(), Some(0));
        assert_eq!(metadata.nodes()[1].value(), Some(""));
        assert_eq!(metadata.nodes()[2].parent_index(), Some(0));
        assert_eq!(metadata.nodes()[2].value(), Some(" μV "));
        assert_eq!(metadata.nodes()[3].parent_index(), Some(1));
        assert_eq!(metadata.nodes()[3].name(), "标签ß");
        assert_eq!(metadata.nodes()[3].value(), Some("左🙂"));
    }

    #[test]
    fn core_008_into_parts_preserves_irregular_descriptor_and_tree_order() {
        let descriptor_limits = StreamDescriptorLimits::new(6, 1, 6, 3).unwrap();
        let metadata_limits = MetadataTreeLimits::new(4, 3, 2, 4, 4).unwrap();
        let definition = StreamDefinition::new(
            descriptor(
                descriptor_limits,
                NominalSampleRate::irregular(),
                ChannelFormat::String,
            ),
            extended_metadata(metadata_limits),
        );

        let (descriptor, metadata) = definition.into_parts();
        assert_eq!(descriptor.limits(), descriptor_limits);
        assert_eq!(
            descriptor.nominal_sample_rate(),
            NominalSampleRate::Irregular
        );
        assert_eq!(descriptor.channel_format(), ChannelFormat::String);
        assert_eq!(metadata.limits(), metadata_limits);
        let parts: Vec<_> = metadata
            .into_nodes()
            .into_iter()
            .map(|node| node.into_parts())
            .collect();
        assert_eq!(
            parts,
            vec![
                (None, "根🦀".to_owned(), None),
                (Some(0), "通道".to_owned(), Some("".to_owned())),
                (Some(0), "设备".to_owned(), Some(" μV ".to_owned())),
                (Some(1), "标签ß".to_owned(), Some("左🙂".to_owned())),
            ]
        );
    }

    #[test]
    fn core_008_all_seven_channel_formats_survive_composition() {
        let formats = [
            ChannelFormat::Float32,
            ChannelFormat::Double64,
            ChannelFormat::String,
            ChannelFormat::Int32,
            ChannelFormat::Int16,
            ChannelFormat::Int8,
            ChannelFormat::Int64,
        ];

        for format in formats {
            let definition = StreamDefinition::new(
                descriptor(
                    StreamDescriptorLimits::new(6, 1, 6, 3).unwrap(),
                    NominalSampleRate::irregular(),
                    format,
                ),
                extended_metadata(MetadataTreeLimits::new(4, 3, 2, 4, 4).unwrap()),
            );
            assert_eq!(definition.descriptor().channel_format(), format);
            let (descriptor, _) = definition.into_parts();
            assert_eq!(descriptor.channel_format(), format);
        }
    }

    #[test]
    fn core_008_construction_moves_existing_allocations_unchanged() {
        let descriptor = descriptor(
            StreamDescriptorLimits::new(6, 1, 6, 3).unwrap(),
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
        );
        let metadata = extended_metadata(MetadataTreeLimits::new(4, 3, 2, 4, 4).unwrap());
        let descriptor_name_pointer = descriptor.name().as_ptr();
        let root_name_pointer = metadata.nodes()[0].name().as_ptr();
        let leaf_value_pointer = metadata.nodes()[3].value().unwrap().as_ptr();

        let definition = StreamDefinition::new(descriptor, metadata);
        assert_eq!(
            definition.descriptor().name().as_ptr(),
            descriptor_name_pointer
        );
        assert_eq!(
            definition.extended_metadata().nodes()[0].name().as_ptr(),
            root_name_pointer
        );
        assert_eq!(
            definition.extended_metadata().nodes()[3]
                .value()
                .unwrap()
                .as_ptr(),
            leaf_value_pointer
        );

        let (descriptor, metadata) = definition.into_parts();
        assert_eq!(descriptor.name().as_ptr(), descriptor_name_pointer);
        assert_eq!(metadata.nodes()[0].name().as_ptr(), root_name_pointer);
        assert_eq!(
            metadata.nodes()[3].value().unwrap().as_ptr(),
            leaf_value_pointer
        );
    }
}
