// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{ChannelFormat, MetadataTree, NominalSampleRate, StreamDefinition};

/// Identifies one descriptor-owned static stream-info field.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamInfoStaticFieldRole {
    /// The required stream name.
    Name,
    /// The effective content type, corresponding to the stream-info `type` role.
    Type,
    /// The validated channel count.
    ChannelCount,
    /// The caller-declared channel data format.
    ChannelFormat,
    /// The effective source correlation identifier.
    SourceId,
    /// The caller-declared nominal sample rate.
    NominalSrate,
}

impl StreamInfoStaticFieldRole {
    /// The fixed descriptor-owned static-field order observed by LSLC-001H.
    pub const ORDER: [Self; 6] = [
        Self::Name,
        Self::Type,
        Self::ChannelCount,
        Self::ChannelFormat,
        Self::SourceId,
        Self::NominalSrate,
    ];
}

/// A borrowed semantic view of one accepted stream definition's static fields.
///
/// This view allocates nothing, does not mutate or consume the definition, and
/// assigns no XML `desc` meaning to its generic extended metadata.
#[derive(Clone, Copy, Debug)]
pub struct StreamInfoStaticFields<'a> {
    definition: &'a StreamDefinition,
}

impl<'a> StreamInfoStaticFields<'a> {
    /// Borrows one accepted stream definition without allocating or validating.
    #[must_use]
    pub const fn new(definition: &'a StreamDefinition) -> Self {
        Self { definition }
    }

    /// Returns the fixed six-role static-field order.
    #[must_use]
    pub const fn roles() -> &'static [StreamInfoStaticFieldRole; 6] {
        &StreamInfoStaticFieldRole::ORDER
    }

    /// Returns the borrowed source definition.
    #[must_use]
    pub const fn definition(&self) -> &'a StreamDefinition {
        self.definition
    }

    /// Returns the unchanged required stream name.
    #[must_use]
    pub fn name(&self) -> &'a str {
        self.definition.descriptor().name()
    }

    /// Returns the unchanged original optional content-type form.
    #[must_use]
    pub fn content_type(&self) -> Option<&'a str> {
        self.definition.descriptor().content_type()
    }

    /// Returns the effective `type` value, mapping only absence to empty text.
    #[must_use]
    pub fn effective_type(&self) -> &'a str {
        self.content_type().unwrap_or("")
    }

    /// Returns the unchanged validated channel count.
    #[must_use]
    pub const fn channel_count(&self) -> usize {
        self.definition.descriptor().channel_count()
    }

    /// Returns the unchanged caller-declared channel data format.
    #[must_use]
    pub const fn channel_format(&self) -> ChannelFormat {
        self.definition.descriptor().channel_format()
    }

    /// Returns the exact observed static spelling for the channel format.
    #[must_use]
    pub const fn channel_format_spelling(&self) -> &'static str {
        match self.channel_format() {
            ChannelFormat::Float32 => "float32",
            ChannelFormat::Double64 => "double64",
            ChannelFormat::String => "string",
            ChannelFormat::Int32 => "int32",
            ChannelFormat::Int16 => "int16",
            ChannelFormat::Int8 => "int8",
            ChannelFormat::Int64 => "int64",
        }
    }

    /// Returns the unchanged original optional source-identifier form.
    #[must_use]
    pub fn source_id(&self) -> Option<&'a str> {
        self.definition.descriptor().source_id()
    }

    /// Returns the effective source identifier, mapping only absence to empty text.
    #[must_use]
    pub fn effective_source_id(&self) -> &'a str {
        self.source_id().unwrap_or("")
    }

    /// Returns the unchanged caller-declared nominal sample-rate form.
    #[must_use]
    pub const fn nominal_sample_rate(&self) -> NominalSampleRate {
        self.definition.descriptor().nominal_sample_rate()
    }

    /// Returns the effective numeric rate, mapping only irregular to positive zero.
    #[must_use]
    pub const fn effective_nominal_srate(&self) -> f64 {
        match self.nominal_sample_rate() {
            NominalSampleRate::Irregular => 0.0,
            NominalSampleRate::RegularHz(rate) => rate.hz(),
        }
    }

    /// Returns the unchanged generic extended metadata without assigning `desc` meaning.
    #[must_use]
    pub const fn extended_metadata(&self) -> &'a MetadataTree {
        self.definition.extended_metadata()
    }
}

#[cfg(test)]
mod tests {
    use super::{StreamInfoStaticFieldRole, StreamInfoStaticFields};
    use crate::{
        ChannelFormat, MetadataNodeInput, MetadataTree, MetadataTreeLimits, NominalSampleRate,
        StreamDefinition, StreamDescriptor, StreamDescriptorLimits,
    };

    fn definition(
        content_type: Option<&str>,
        source_id: Option<&str>,
        rate: NominalSampleRate,
        format: ChannelFormat,
    ) -> StreamDefinition {
        let descriptor = StreamDescriptor::new(
            StreamDescriptorLimits::new(32, 32, 32, 8).unwrap(),
            " name-Ω ".to_owned(),
            content_type.map(str::to_owned),
            source_id.map(str::to_owned),
            3,
            rate,
            format,
        )
        .unwrap();
        let metadata = MetadataTree::new(
            MetadataTreeLimits::new(3, 3, 2, 16, 16).unwrap(),
            vec![
                MetadataNodeInput::new(None, "generic-root".to_owned(), None),
                MetadataNodeInput::new(Some(0), "empty".to_owned(), Some(String::new())),
                MetadataNodeInput::new(Some(0), "value".to_owned(), Some(" unchanged ".to_owned())),
            ],
        )
        .unwrap();
        StreamDefinition::new(descriptor, metadata)
    }

    fn observed_definition(
        name: &str,
        content_type: &str,
        source_id: &str,
        channel_count: usize,
        rate: NominalSampleRate,
        format: ChannelFormat,
    ) -> StreamDefinition {
        let descriptor = StreamDescriptor::new(
            StreamDescriptorLimits::new(64, 64, 64, 8).unwrap(),
            name.to_owned(),
            Some(content_type.to_owned()),
            Some(source_id.to_owned()),
            channel_count,
            rate,
            format,
        )
        .unwrap();
        let metadata = MetadataTree::new(
            MetadataTreeLimits::new(1, 1, 1, 16, 16).unwrap(),
            vec![MetadataNodeInput::new(
                None,
                "generic-root".to_owned(),
                None,
            )],
        )
        .unwrap();
        StreamDefinition::new(descriptor, metadata)
    }

    #[test]
    fn lslc_001k_fixed_six_role_order_is_exact() {
        assert_eq!(
            StreamInfoStaticFields::roles(),
            &[
                StreamInfoStaticFieldRole::Name,
                StreamInfoStaticFieldRole::Type,
                StreamInfoStaticFieldRole::ChannelCount,
                StreamInfoStaticFieldRole::ChannelFormat,
                StreamInfoStaticFieldRole::SourceId,
                StreamInfoStaticFieldRole::NominalSrate,
            ]
        );
    }

    #[test]
    fn lslc_001k_option_forms_remain_distinct_from_effective_empty_values() {
        for (content_type, source_id, expected_type, expected_source) in [
            (None, None, "", ""),
            (Some(""), Some(""), "", ""),
            (Some(" type "), Some(" source "), " type ", " source "),
        ] {
            let definition = definition(
                content_type,
                source_id,
                NominalSampleRate::irregular(),
                ChannelFormat::Float32,
            );
            let fields = StreamInfoStaticFields::new(&definition);
            assert_eq!(fields.content_type(), content_type);
            assert_eq!(fields.source_id(), source_id);
            assert_eq!(fields.effective_type(), expected_type);
            assert_eq!(fields.effective_source_id(), expected_source);
        }
    }

    #[test]
    fn lslc_001k_all_channel_formats_map_exactly_and_totally() {
        for (format, spelling) in [
            (ChannelFormat::Float32, "float32"),
            (ChannelFormat::Double64, "double64"),
            (ChannelFormat::String, "string"),
            (ChannelFormat::Int32, "int32"),
            (ChannelFormat::Int16, "int16"),
            (ChannelFormat::Int8, "int8"),
            (ChannelFormat::Int64, "int64"),
        ] {
            let definition = definition(None, None, NominalSampleRate::irregular(), format);
            let fields = StreamInfoStaticFields::new(&definition);
            assert_eq!(fields.channel_format(), format);
            assert_eq!(fields.channel_format_spelling(), spelling);
        }
    }

    #[test]
    fn lslc_001k_seven_observed_semantic_cases_execute_exactly() {
        let cases = [
            (
                "neutral-float32",
                "",
                "",
                1,
                NominalSampleRate::irregular(),
                ChannelFormat::Float32,
                "float32",
            ),
            (
                "neutral-double64",
                "measurement",
                "source-double64",
                2,
                NominalSampleRate::regular_hz(100.0).unwrap(),
                ChannelFormat::Double64,
                "double64",
            ),
            (
                "unicode-Ω-中-&-<-greater->",
                "text-&-<-greater->-\"-'",
                "source-雪-&-<-greater->",
                3,
                NominalSampleRate::regular_hz(59.94).unwrap(),
                ChannelFormat::String,
                "string",
            ),
            (
                "neutral-int32",
                "integer",
                "source-int32",
                4,
                NominalSampleRate::regular_hz(1.0).unwrap(),
                ChannelFormat::Int32,
                "int32",
            ),
            (
                "neutral-int16",
                "integer",
                "source-int16",
                5,
                NominalSampleRate::regular_hz(256.5).unwrap(),
                ChannelFormat::Int16,
                "int16",
            ),
            (
                "neutral-int8",
                "integer",
                "source-int8",
                6,
                NominalSampleRate::irregular(),
                ChannelFormat::Int8,
                "int8",
            ),
            (
                "neutral-int64",
                "integer",
                "source-int64",
                7,
                NominalSampleRate::regular_hz(1_000_000.25).unwrap(),
                ChannelFormat::Int64,
                "int64",
            ),
        ];

        for (name, content_type, source_id, count, rate, format, spelling) in cases {
            let definition =
                observed_definition(name, content_type, source_id, count, rate, format);
            let fields = StreamInfoStaticFields::new(&definition);
            assert_eq!(fields.name(), name);
            assert_eq!(fields.content_type(), Some(content_type));
            assert_eq!(fields.effective_type(), content_type);
            assert_eq!(fields.channel_count(), count);
            assert_eq!(fields.channel_format(), format);
            assert_eq!(fields.channel_format_spelling(), spelling);
            assert_eq!(fields.source_id(), Some(source_id));
            assert_eq!(fields.effective_source_id(), source_id);
            assert_eq!(fields.nominal_sample_rate(), rate);
            let effective_bits = match rate {
                NominalSampleRate::Irregular => 0.0_f64.to_bits(),
                NominalSampleRate::RegularHz(value) => value.hz().to_bits(),
            };
            assert_eq!(fields.effective_nominal_srate().to_bits(), effective_bits);
            assert_eq!(fields.extended_metadata().nodes()[0].name(), "generic-root");
        }
    }

    #[test]
    fn lslc_001k_original_and_effective_rate_views_remain_separate() {
        let irregular = definition(
            None,
            None,
            NominalSampleRate::irregular(),
            ChannelFormat::Int8,
        );
        let irregular_fields = StreamInfoStaticFields::new(&irregular);
        assert_eq!(
            irregular_fields.nominal_sample_rate(),
            NominalSampleRate::Irregular
        );
        assert_eq!(irregular_fields.effective_nominal_srate().to_bits(), 0);

        let rate_bits = 0x405d_fae1_47ae_147b;
        let regular = definition(
            None,
            None,
            NominalSampleRate::regular_hz(f64::from_bits(rate_bits)).unwrap(),
            ChannelFormat::Double64,
        );
        let regular_fields = StreamInfoStaticFields::new(&regular);
        match regular_fields.nominal_sample_rate() {
            NominalSampleRate::RegularHz(rate) => assert_eq!(rate.hz().to_bits(), rate_bits),
            NominalSampleRate::Irregular => panic!("regular rate changed form"),
        }
        assert_eq!(
            regular_fields.effective_nominal_srate().to_bits(),
            rate_bits
        );
    }

    #[test]
    fn lslc_001k_borrowing_preserves_identity_and_generic_metadata() {
        let definition = definition(
            Some(" type "),
            Some(" source "),
            NominalSampleRate::irregular(),
            ChannelFormat::String,
        );
        let definition_pointer = &definition as *const StreamDefinition;
        let name_pointer = definition.descriptor().name().as_ptr();
        let type_pointer = definition.descriptor().content_type().unwrap().as_ptr();
        let source_pointer = definition.descriptor().source_id().unwrap().as_ptr();
        let root_pointer = definition.extended_metadata().nodes()[0].name().as_ptr();
        let value_pointer = definition.extended_metadata().nodes()[2]
            .value()
            .unwrap()
            .as_ptr();

        {
            let fields = StreamInfoStaticFields::new(&definition);
            assert_eq!(
                fields.definition() as *const StreamDefinition,
                definition_pointer
            );
            assert_eq!(fields.name().as_ptr(), name_pointer);
            assert_eq!(fields.content_type().unwrap().as_ptr(), type_pointer);
            assert_eq!(fields.effective_type().as_ptr(), type_pointer);
            assert_eq!(fields.source_id().unwrap().as_ptr(), source_pointer);
            assert_eq!(fields.effective_source_id().as_ptr(), source_pointer);
            assert_eq!(
                fields.extended_metadata().nodes()[0].name().as_ptr(),
                root_pointer
            );
            assert_eq!(
                fields.extended_metadata().nodes()[2]
                    .value()
                    .unwrap()
                    .as_ptr(),
                value_pointer
            );
            assert_eq!(fields.channel_count(), 3);
        }

        assert_eq!(definition.descriptor().name(), " name-Ω ");
        assert_eq!(definition.descriptor().content_type(), Some(" type "));
        assert_eq!(definition.descriptor().source_id(), Some(" source "));
        assert_eq!(
            definition.extended_metadata().nodes()[0].name(),
            "generic-root"
        );
        assert_eq!(definition.extended_metadata().nodes()[1].value(), Some(""));
        assert_eq!(
            definition.extended_metadata().nodes()[2].value(),
            Some(" unchanged ")
        );
    }
}
