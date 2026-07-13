// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    StreamInfoStaticFields, StreamInfoStaticNumericSpellingError, StreamInfoStaticNumericSpellings,
    XmlCharacterData, XmlCharacterDataError, XmlCharacterDataLimit, XmlElementName,
    XmlElementNodeInput, XmlElementNodeValue, XmlElementTree, XmlElementTreeError,
    XmlElementTreeLimits, XmlLeafElement, XmlNameError, XmlNameLimit, XmlText, XmlTextError,
    XmlTextLimit,
};
use core::fmt;

const NODE_COUNT: usize = 7;
const ROOT_INDEX: usize = 0;
const ROOT_NAME: &str = "info";
const FIELD_NAMES: [&str; 6] = [
    "name",
    "type",
    "channel_count",
    "channel_format",
    "source_id",
    "nominal_srate",
];

/// Caller-selected accepted XML bounds for one static stream-info composition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamInfoStaticXmlLimits {
    name: XmlNameLimit,
    text: XmlTextLimit,
    character_data: XmlCharacterDataLimit,
    tree: XmlElementTreeLimits,
}

impl StreamInfoStaticXmlLimits {
    /// Groups existing accepted component limits without adding defaults.
    #[must_use]
    pub const fn new(
        name: XmlNameLimit,
        text: XmlTextLimit,
        character_data: XmlCharacterDataLimit,
        tree: XmlElementTreeLimits,
    ) -> Self {
        Self {
            name,
            text,
            character_data,
            tree,
        }
    }

    /// Returns the element-name bound.
    #[must_use]
    pub const fn name(self) -> XmlNameLimit {
        self.name
    }

    /// Returns the logical-text bound.
    #[must_use]
    pub const fn text(self) -> XmlTextLimit {
        self.text
    }

    /// Returns the represented-character-data bound.
    #[must_use]
    pub const fn character_data(self) -> XmlCharacterDataLimit {
        self.character_data
    }

    /// Returns the element-tree bounds.
    #[must_use]
    pub const fn tree(self) -> XmlElementTreeLimits {
        self.tree
    }
}

/// One bounded owned `info` element tree containing only six static fields.
///
/// The tree contains no declaration, whitespace nodes, `desc`, volatile field,
/// endpoint-document, protocol, transport, provider, or runtime meaning.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoStaticXml {
    limits: StreamInfoStaticXmlLimits,
    tree: XmlElementTree,
}

impl StreamInfoStaticXml {
    /// Projects one borrowed accepted static-field view into a seven-node tree.
    ///
    /// Numeric-domain validation precedes the exact node-arena reserve. Root
    /// and leaf names and all six values are copied through separate exact
    /// fallible reserves before delegation to the existing XML contracts.
    pub fn compose(
        fields: &StreamInfoStaticFields<'_>,
        limits: StreamInfoStaticXmlLimits,
    ) -> Result<Self, StreamInfoStaticXmlError> {
        let numeric = StreamInfoStaticNumericSpellings::new(fields)
            .map_err(StreamInfoStaticXmlError::NumericSpelling)?;

        let mut nodes = Vec::new();
        nodes.try_reserve_exact(NODE_COUNT).map_err(|_| {
            StreamInfoStaticXmlError::NodeAllocationFailed {
                requested: NODE_COUNT,
            }
        })?;

        nodes.push(XmlElementNodeInput::new(
            None,
            XmlElementNodeValue::Container(copy_name(ROOT_INDEX, ROOT_NAME, limits.name)?),
        ));

        let values = [
            fields.name(),
            fields.effective_type(),
            numeric.channel_count(),
            fields.channel_format_spelling(),
            fields.effective_source_id(),
            numeric.nominal_srate(),
        ];
        for (offset, (name, value)) in FIELD_NAMES.into_iter().zip(values).enumerate() {
            let node_index = offset + 1;
            let name = copy_name(node_index, name, limits.name)?;
            let text = copy_text(node_index, value, limits.text)?;
            let character_data = XmlCharacterData::encode(limits.character_data, &text)
                .map_err(|source| StreamInfoStaticXmlError::CharacterData { node_index, source })?;
            nodes.push(XmlElementNodeInput::new(
                Some(ROOT_INDEX),
                XmlElementNodeValue::Leaf(XmlLeafElement::new(name, character_data)),
            ));
        }

        let tree = XmlElementTree::new(limits.tree, nodes)
            .map_err(StreamInfoStaticXmlError::ElementTree)?;
        Ok(Self { limits, tree })
    }

    /// Returns the component limits used for this composition.
    #[must_use]
    pub const fn limits(&self) -> StreamInfoStaticXmlLimits {
        self.limits
    }

    /// Returns the bounded seven-node element tree.
    #[must_use]
    pub const fn tree(&self) -> &XmlElementTree {
        &self.tree
    }

    /// Returns the owned element tree without replacing its node arena.
    #[must_use]
    pub fn into_tree(self) -> XmlElementTree {
        self.tree
    }
}

/// Deterministic rejection from static stream-info XML composition.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamInfoStaticXmlError {
    /// The accepted LSLC-001L numeric domain or allocation rejected the input.
    NumericSpelling(StreamInfoStaticNumericSpellingError),
    /// The exact seven-node arena reserve failed.
    NodeAllocationFailed {
        /// The exact node capacity requested.
        requested: usize,
    },
    /// An exact copied-name allocation failed.
    NameAllocationFailed {
        /// The root or leaf node being prepared.
        node_index: usize,
        /// The exact UTF-8 byte capacity requested.
        requested: usize,
    },
    /// An accepted XML name contract rejected a copied fixed name.
    Name {
        /// The root or leaf node being prepared.
        node_index: usize,
        /// The unchanged XML name error.
        source: XmlNameError,
    },
    /// An exact copied-value allocation failed.
    TextAllocationFailed {
        /// The leaf node being prepared.
        node_index: usize,
        /// The exact UTF-8 byte capacity requested.
        requested: usize,
    },
    /// The accepted XML text contract rejected a copied static value.
    Text {
        /// The leaf node being prepared.
        node_index: usize,
        /// The unchanged XML text error.
        source: XmlTextError,
    },
    /// The accepted character-data contract rejected a static value.
    CharacterData {
        /// The leaf node being prepared.
        node_index: usize,
        /// The unchanged character-data error.
        source: XmlCharacterDataError,
    },
    /// The accepted element-tree contract rejected the completed candidate.
    ElementTree(XmlElementTreeError),
}

impl fmt::Display for StreamInfoStaticXmlError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "static stream-info XML rejected input: {self:?}")
    }
}

impl std::error::Error for StreamInfoStaticXmlError {}

fn copy_name(
    node_index: usize,
    source: &str,
    limit: XmlNameLimit,
) -> Result<XmlElementName, StreamInfoStaticXmlError> {
    let copied = copy_string(source).map_err(|requested| {
        StreamInfoStaticXmlError::NameAllocationFailed {
            node_index,
            requested,
        }
    })?;
    XmlElementName::new(limit, copied)
        .map_err(|source| StreamInfoStaticXmlError::Name { node_index, source })
}

fn copy_text(
    node_index: usize,
    source: &str,
    limit: XmlTextLimit,
) -> Result<XmlText, StreamInfoStaticXmlError> {
    let copied = copy_string(source).map_err(|requested| {
        StreamInfoStaticXmlError::TextAllocationFailed {
            node_index,
            requested,
        }
    })?;
    XmlText::new(limit, copied)
        .map_err(|source| StreamInfoStaticXmlError::Text { node_index, source })
}

fn copy_string(source: &str) -> Result<String, usize> {
    let requested = source.len();
    let mut copied = String::new();
    copied.try_reserve_exact(requested).map_err(|_| requested)?;
    copied.push_str(source);
    debug_assert_eq!(copied.len(), requested);
    Ok(copied)
}

#[cfg(test)]
mod tests {
    use super::{
        copy_string, StreamInfoStaticXml, StreamInfoStaticXmlError, StreamInfoStaticXmlLimits,
        FIELD_NAMES,
    };
    use crate::{
        ChannelFormat, MetadataNodeInput, MetadataTree, MetadataTreeLimits, NominalSampleRate,
        StreamDefinition, StreamDescriptor, StreamDescriptorLimits, StreamInfoStaticFields,
        StreamInfoStaticNumericSpellingError, XmlCharacterDataError, XmlCharacterDataLimit,
        XmlElementSerialization, XmlElementSerializationLimit, XmlElementTreeError,
        XmlElementTreeLimits, XmlNameError, XmlNameLimit, XmlTextError, XmlTextLimit,
    };

    fn definition(
        name: &str,
        content_type: Option<&str>,
        source_id: Option<&str>,
        channel_count: usize,
        rate: NominalSampleRate,
        format: ChannelFormat,
    ) -> StreamDefinition {
        let descriptor = StreamDescriptor::new(
            StreamDescriptorLimits::new(64, 64, 64, 8).unwrap(),
            name.to_owned(),
            content_type.map(str::to_owned),
            source_id.map(str::to_owned),
            channel_count,
            rate,
            format,
        )
        .unwrap();
        let metadata = MetadataTree::new(
            MetadataTreeLimits::new(1, 1, 1, 32, 32).unwrap(),
            vec![MetadataNodeInput::new(
                None,
                "generic-root".to_owned(),
                None,
            )],
        )
        .unwrap();
        StreamDefinition::new(descriptor, metadata)
    }

    fn limits() -> StreamInfoStaticXmlLimits {
        StreamInfoStaticXmlLimits::new(
            XmlNameLimit::new(32).unwrap(),
            XmlTextLimit::new(64).unwrap(),
            XmlCharacterDataLimit::new(256).unwrap(),
            XmlElementTreeLimits::new(7, 2, 6, 512).unwrap(),
        )
    }

    #[test]
    fn lslc_001m_seven_observed_static_cases_compose_and_serialize_exactly() {
        let cases = [
            ("neutral-float32", Some(""), Some(""), 1, NominalSampleRate::irregular(), ChannelFormat::Float32, "<info><name>neutral-float32</name><type></type><channel_count>1</channel_count><channel_format>float32</channel_format><source_id></source_id><nominal_srate>0.000000000000000</nominal_srate></info>"),
            ("neutral-double64", Some("measurement"), Some("source-double64"), 2, NominalSampleRate::regular_hz(100.0).unwrap(), ChannelFormat::Double64, "<info><name>neutral-double64</name><type>measurement</type><channel_count>2</channel_count><channel_format>double64</channel_format><source_id>source-double64</source_id><nominal_srate>100.0000000000000</nominal_srate></info>"),
            ("unicode-Ω-中-&-<-greater->", Some("text-&-<-greater->-\"-'"), Some("source-雪-&-<-greater->"), 3, NominalSampleRate::regular_hz(59.94).unwrap(), ChannelFormat::String, "<info><name>unicode-Ω-中-&amp;-&lt;-greater-&gt;</name><type>text-&amp;-&lt;-greater-&gt;-\"-'</type><channel_count>3</channel_count><channel_format>string</channel_format><source_id>source-雪-&amp;-&lt;-greater-&gt;</source_id><nominal_srate>59.94000000000000</nominal_srate></info>"),
            ("neutral-int32", Some("integer"), Some("source-int32"), 4, NominalSampleRate::regular_hz(1.0).unwrap(), ChannelFormat::Int32, "<info><name>neutral-int32</name><type>integer</type><channel_count>4</channel_count><channel_format>int32</channel_format><source_id>source-int32</source_id><nominal_srate>1.000000000000000</nominal_srate></info>"),
            ("neutral-int16", Some("integer"), Some("source-int16"), 5, NominalSampleRate::regular_hz(256.5).unwrap(), ChannelFormat::Int16, "<info><name>neutral-int16</name><type>integer</type><channel_count>5</channel_count><channel_format>int16</channel_format><source_id>source-int16</source_id><nominal_srate>256.5000000000000</nominal_srate></info>"),
            ("neutral-int8", Some("integer"), Some("source-int8"), 6, NominalSampleRate::irregular(), ChannelFormat::Int8, "<info><name>neutral-int8</name><type>integer</type><channel_count>6</channel_count><channel_format>int8</channel_format><source_id>source-int8</source_id><nominal_srate>0.000000000000000</nominal_srate></info>"),
            ("neutral-int64", Some("integer"), Some("source-int64"), 7, NominalSampleRate::regular_hz(1_000_000.25).unwrap(), ChannelFormat::Int64, "<info><name>neutral-int64</name><type>integer</type><channel_count>7</channel_count><channel_format>int64</channel_format><source_id>source-int64</source_id><nominal_srate>1000000.250000000</nominal_srate></info>"),
        ];

        for (name, kind, source_id, count, rate, format, expected) in cases {
            let definition = definition(name, kind, source_id, count, rate, format);
            let fields = StreamInfoStaticFields::new(&definition);
            let xml = StreamInfoStaticXml::compose(&fields, limits()).unwrap();
            assert_eq!(xml.tree().nodes().len(), 7);
            assert_eq!(
                xml.tree().nodes()[0]
                    .value()
                    .as_container()
                    .unwrap()
                    .as_str(),
                "info"
            );
            let actual_names: Vec<_> = xml.tree().nodes()[1..]
                .iter()
                .map(|node| node.value().as_leaf().unwrap().name().as_str())
                .collect();
            assert_eq!(actual_names, FIELD_NAMES);
            let serialized = XmlElementSerialization::serialize(
                XmlElementSerializationLimit::new(expected.len()).unwrap(),
                xml.tree(),
            )
            .unwrap();
            assert_eq!(serialized.as_str(), expected);
            assert_eq!(fields.definition() as *const _, &definition as *const _);
            assert_eq!(fields.extended_metadata().nodes()[0].name(), "generic-root");
        }
    }

    #[test]
    fn lslc_001m_absent_and_present_empty_optionals_share_only_effective_xml() {
        let absent = definition(
            "n",
            None,
            None,
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
        );
        let empty = definition(
            "n",
            Some(""),
            Some(""),
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
        );
        let absent_fields = StreamInfoStaticFields::new(&absent);
        let empty_fields = StreamInfoStaticFields::new(&empty);
        let absent_xml = StreamInfoStaticXml::compose(&absent_fields, limits()).unwrap();
        let empty_xml = StreamInfoStaticXml::compose(&empty_fields, limits()).unwrap();
        assert_eq!(absent_xml.tree(), empty_xml.tree());
        assert_eq!(absent_fields.content_type(), None);
        assert_eq!(empty_fields.content_type(), Some(""));
        assert_eq!(absent_fields.source_id(), None);
        assert_eq!(empty_fields.source_id(), Some(""));
    }

    #[test]
    fn lslc_001m_rejections_are_typed_and_ordered() {
        let unsupported = definition(
            "name-too-long",
            None,
            None,
            1,
            NominalSampleRate::regular_hz(2.0).unwrap(),
            ChannelFormat::Float32,
        );
        let unsupported_fields = StreamInfoStaticFields::new(&unsupported);
        assert_eq!(
            StreamInfoStaticXml::compose(
                &unsupported_fields,
                StreamInfoStaticXmlLimits::new(
                    XmlNameLimit::new(1).unwrap(),
                    XmlTextLimit::new(1).unwrap(),
                    XmlCharacterDataLimit::new(1).unwrap(),
                    XmlElementTreeLimits::new(1, 1, 1, 1).unwrap(),
                ),
            ),
            Err(StreamInfoStaticXmlError::NumericSpelling(
                StreamInfoStaticNumericSpellingError::UnsupportedRegularNominalSrate {
                    actual_bits: 2.0_f64.to_bits(),
                },
            ))
        );

        let accepted = definition(
            "A&B",
            None,
            None,
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
        );
        let fields = StreamInfoStaticFields::new(&accepted);
        assert_eq!(
            StreamInfoStaticXml::compose(
                &fields,
                StreamInfoStaticXmlLimits::new(
                    XmlNameLimit::new(3).unwrap(),
                    XmlTextLimit::new(2).unwrap(),
                    XmlCharacterDataLimit::new(4).unwrap(),
                    XmlElementTreeLimits::new(6, 1, 1, 1).unwrap(),
                ),
            ),
            Err(StreamInfoStaticXmlError::Name {
                node_index: 0,
                source: XmlNameError::LimitExceeded {
                    expected_max: 3,
                    actual: 4,
                },
            })
        );
        assert_eq!(
            StreamInfoStaticXml::compose(
                &fields,
                StreamInfoStaticXmlLimits::new(
                    XmlNameLimit::new(32).unwrap(),
                    XmlTextLimit::new(2).unwrap(),
                    XmlCharacterDataLimit::new(4).unwrap(),
                    XmlElementTreeLimits::new(6, 1, 1, 1).unwrap(),
                ),
            ),
            Err(StreamInfoStaticXmlError::Text {
                node_index: 1,
                source: XmlTextError::LimitExceeded {
                    expected_max: 2,
                    actual: 3,
                },
            })
        );
        assert_eq!(
            StreamInfoStaticXml::compose(
                &fields,
                StreamInfoStaticXmlLimits::new(
                    XmlNameLimit::new(32).unwrap(),
                    XmlTextLimit::new(64).unwrap(),
                    XmlCharacterDataLimit::new(4).unwrap(),
                    XmlElementTreeLimits::new(6, 1, 1, 1).unwrap(),
                ),
            ),
            Err(StreamInfoStaticXmlError::CharacterData {
                node_index: 1,
                source: XmlCharacterDataError::LimitExceeded {
                    expected_max: 4,
                    required: 7,
                },
            })
        );
        assert_eq!(
            StreamInfoStaticXml::compose(
                &fields,
                StreamInfoStaticXmlLimits::new(
                    XmlNameLimit::new(32).unwrap(),
                    XmlTextLimit::new(64).unwrap(),
                    XmlCharacterDataLimit::new(256).unwrap(),
                    XmlElementTreeLimits::new(6, 1, 1, 1).unwrap(),
                ),
            ),
            Err(StreamInfoStaticXmlError::ElementTree(
                XmlElementTreeError::NodeLimitExceeded {
                    expected_max: 6,
                    actual: 7,
                },
            ))
        );
    }

    #[test]
    fn lslc_001m_copy_and_consuming_access_preserve_owned_allocations() {
        let source = "owned-copy";
        let copied = copy_string(source).unwrap();
        assert_eq!(copied, source);
        assert_ne!(copied.as_ptr(), source.as_ptr());

        let definition = definition(
            "n",
            None,
            None,
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
        );
        let fields = StreamInfoStaticFields::new(&definition);
        let xml = StreamInfoStaticXml::compose(&fields, limits()).unwrap();
        let node_pointer = xml.tree().nodes().as_ptr();
        assert_eq!(xml.limits(), limits());
        let tree = xml.into_tree();
        assert_eq!(tree.nodes().as_ptr(), node_pointer);
    }

    #[test]
    fn lslc_001m_limit_accessors_are_exact() {
        let limits = limits();
        assert_eq!(limits.name(), XmlNameLimit::new(32).unwrap());
        assert_eq!(limits.text(), XmlTextLimit::new(64).unwrap());
        assert_eq!(
            limits.character_data(),
            XmlCharacterDataLimit::new(256).unwrap()
        );
        assert_eq!(
            limits.tree(),
            XmlElementTreeLimits::new(7, 2, 6, 512).unwrap()
        );
    }
}
