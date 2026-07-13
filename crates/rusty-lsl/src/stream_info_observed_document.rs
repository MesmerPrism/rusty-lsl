// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{StreamInfoOrderedXml, XmlElementNodeValue};
use core::fmt;

const DECLARATION: &str = "<?xml version=\"1.0\"?>\n";
const DESCRIPTION_ROOT_INDEX: usize = 18;
const DESCRIPTION_ROOT_NAME: &str = "desc";

/// An explicit nonzero maximum for one observed document-envelope projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamInfoObservedDocumentLimit {
    max_output_bytes: usize,
}

impl StreamInfoObservedDocumentLimit {
    /// Creates an output bound that accepts at least one UTF-8 byte.
    pub fn new(max_output_bytes: usize) -> Result<Self, StreamInfoObservedDocumentError> {
        if max_output_bytes == 0 {
            return Err(StreamInfoObservedDocumentError::InvalidLimit {
                expected_min: 1,
                actual: max_output_bytes,
            });
        }
        Ok(Self { max_output_bytes })
    }

    /// Returns the maximum accepted output byte count.
    #[must_use]
    pub const fn max_output_bytes(self) -> usize {
        self.max_output_bytes
    }
}

/// One bounded local projection using the LSLC-001H-observed document envelope.
///
/// This representation is separate from LSLC-001G compact serialization. It
/// proves no parser, endpoint, wire, provider, transport, or runtime behavior.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoObservedDocument {
    limit: StreamInfoObservedDocumentLimit,
    output: String,
}

impl StreamInfoObservedDocument {
    /// Borrows accepted Q state and applies the fixed observed envelope policy.
    ///
    /// The policy emits the exact XML declaration and LF, one tab for each
    /// depth below `info`, LF after every element line, `<desc />` only when the
    /// fixed description root is empty, and one final LF. Other childless
    /// containers fail closed because their empty spelling was not observed.
    pub fn project(
        limit: StreamInfoObservedDocumentLimit,
        source: &StreamInfoOrderedXml,
    ) -> Result<Self, StreamInfoObservedDocumentError> {
        let nodes = source.tree().nodes();
        let mut frames = Vec::new();
        frames.try_reserve_exact(nodes.len()).map_err(|_| {
            StreamInfoObservedDocumentError::TraversalFramesAllocationFailed {
                requested: nodes.len(),
            }
        })?;
        frames.resize(nodes.len(), TraversalFrame::default());
        build_frames(source, &mut frames);
        validate_empty_containers(source, &frames)?;

        let required = exact_output_bytes(source, &frames)?;
        if required > limit.max_output_bytes {
            return Err(StreamInfoObservedDocumentError::OutputLimitExceeded {
                expected_max: limit.max_output_bytes,
                required,
            });
        }

        let mut output = String::new();
        output.try_reserve_exact(required).map_err(|_| {
            StreamInfoObservedDocumentError::OutputAllocationFailed {
                requested: required,
            }
        })?;
        output.push_str(DECLARATION);
        write_document(source, &frames, &mut output);
        debug_assert_eq!(output.len(), required);
        Ok(Self { limit, output })
    }

    /// Returns the selected output byte limit.
    #[must_use]
    pub const fn limit(&self) -> StreamInfoObservedDocumentLimit {
        self.limit
    }

    /// Returns the exact local document-envelope text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.output
    }

    /// Returns the output string without replacing its allocation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.output
    }
}

/// Deterministic rejection from observed document-envelope projection.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamInfoObservedDocumentError {
    /// The configured bound could accept no output byte.
    InvalidLimit {
        /// The smallest accepted maximum.
        expected_min: usize,
        /// The malformed caller maximum.
        actual: usize,
    },
    /// Exact traversal-frame allocation failed.
    TraversalFramesAllocationFailed {
        /// The exact requested frame count.
        requested: usize,
    },
    /// A childless container outside the observed empty-desc role was present.
    UnsupportedEmptyContainer {
        /// The zero-based Q-tree node index.
        node_index: usize,
    },
    /// Exact output-byte arithmetic overflowed at one node.
    LengthOverflow {
        /// The zero-based Q-tree node index.
        node_index: usize,
    },
    /// The exact output exceeded the caller maximum.
    OutputLimitExceeded {
        /// The caller-selected maximum.
        expected_max: usize,
        /// The exact required byte count.
        required: usize,
    },
    /// Exact output allocation failed.
    OutputAllocationFailed {
        /// The exact requested byte capacity.
        requested: usize,
    },
}

impl fmt::Display for StreamInfoObservedDocumentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "observed stream-info document projection rejected input: {self:?}"
        )
    }
}

impl std::error::Error for StreamInfoObservedDocumentError {}

#[derive(Clone, Copy, Default)]
struct TraversalFrame {
    depth: usize,
    first_child: Option<usize>,
    last_child: Option<usize>,
    next_sibling: Option<usize>,
}

fn build_frames(source: &StreamInfoOrderedXml, frames: &mut [TraversalFrame]) {
    let nodes = source.tree().nodes();
    for child_index in 1..nodes.len() {
        let parent_index = nodes[child_index]
            .parent_index()
            .expect("accepted non-root Q node");
        frames[child_index].depth = frames[parent_index].depth + 1;
        if let Some(last_child) = frames[parent_index].last_child {
            frames[last_child].next_sibling = Some(child_index);
        } else {
            frames[parent_index].first_child = Some(child_index);
        }
        frames[parent_index].last_child = Some(child_index);
    }
}

fn validate_empty_containers(
    source: &StreamInfoOrderedXml,
    frames: &[TraversalFrame],
) -> Result<(), StreamInfoObservedDocumentError> {
    for (node_index, (node, frame)) in source.tree().nodes().iter().zip(frames).enumerate() {
        if let XmlElementNodeValue::Container(name) = node.value() {
            if frame.first_child.is_none()
                && (node_index != DESCRIPTION_ROOT_INDEX || name.as_str() != DESCRIPTION_ROOT_NAME)
            {
                return Err(StreamInfoObservedDocumentError::UnsupportedEmptyContainer {
                    node_index,
                });
            }
        }
    }
    Ok(())
}

fn exact_output_bytes(
    source: &StreamInfoOrderedXml,
    frames: &[TraversalFrame],
) -> Result<usize, StreamInfoObservedDocumentError> {
    source
        .tree()
        .nodes()
        .iter()
        .zip(frames)
        .enumerate()
        .try_fold(DECLARATION.len(), |total, (node_index, (node, frame))| {
            let name = node_name(node.value());
            let contribution =
                match node.value() {
                    XmlElementNodeValue::Leaf(leaf) => frame
                        .depth
                        .checked_add(name.len().checked_mul(2).ok_or(
                            StreamInfoObservedDocumentError::LengthOverflow { node_index },
                        )?)
                        .and_then(|value| value.checked_add(leaf.character_data().as_str().len()))
                        .and_then(|value| value.checked_add(6)),
                    XmlElementNodeValue::Container(_) if frame.first_child.is_none() => frame
                        .depth
                        .checked_add(name.len())
                        .and_then(|value| value.checked_add(5)),
                    XmlElementNodeValue::Container(_) => frame
                        .depth
                        .checked_mul(2)
                        .and_then(|value| value.checked_add(name.len().checked_mul(2)?))
                        .and_then(|value| value.checked_add(7)),
                }
                .ok_or(StreamInfoObservedDocumentError::LengthOverflow { node_index })?;
            total
                .checked_add(contribution)
                .ok_or(StreamInfoObservedDocumentError::LengthOverflow { node_index })
        })
}

fn write_document(source: &StreamInfoOrderedXml, frames: &[TraversalFrame], output: &mut String) {
    let nodes = source.tree().nodes();
    let mut node_index = 0;
    loop {
        let node = &nodes[node_index];
        let frame = frames[node_index];
        write_tabs(output, frame.depth);
        let name = node_name(node.value());
        output.push('<');
        output.push_str(name);
        match node.value() {
            XmlElementNodeValue::Leaf(leaf) => {
                output.push('>');
                output.push_str(leaf.character_data().as_str());
                write_end_tag(output, name, false);
            }
            XmlElementNodeValue::Container(_) if frame.first_child.is_none() => {
                output.push_str(" />\n");
            }
            XmlElementNodeValue::Container(_) => {
                output.push_str(">\n");
                node_index = frame.first_child.expect("validated nonempty container");
                continue;
            }
        }

        loop {
            if node_index == 0 {
                return;
            }
            if let Some(next_sibling) = frames[node_index].next_sibling {
                node_index = next_sibling;
                break;
            }
            node_index = nodes[node_index]
                .parent_index()
                .expect("accepted non-root Q node");
            write_tabs(output, frames[node_index].depth);
            write_end_tag(output, node_name(nodes[node_index].value()), true);
        }
    }
}

fn node_name(value: &XmlElementNodeValue) -> &str {
    match value {
        XmlElementNodeValue::Container(name) => name.as_str(),
        XmlElementNodeValue::Leaf(leaf) => leaf.name().as_str(),
    }
}

fn write_tabs(output: &mut String, count: usize) {
    for _ in 0..count {
        output.push('\t');
    }
}

fn write_end_tag(output: &mut String, name: &str, indented: bool) {
    output.push_str("</");
    output.push_str(name);
    output.push_str(">\n");
    if indented {
        debug_assert!(!name.is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::{
        StreamInfoObservedDocument, StreamInfoObservedDocumentError,
        StreamInfoObservedDocumentLimit,
    };
    use crate::{
        project_metadata_tree_to_xml_element_tree, ChannelFormat, MetadataNodeInput, MetadataTree,
        MetadataTreeLimits, MetadataXmlProjectionLimits, NominalSampleRate, StreamDefinition,
        StreamDescriptor, StreamDescriptorLimits, StreamInfoDescriptionXml, StreamInfoOrderedXml,
        StreamInfoStaticFields, StreamInfoStaticXml, StreamInfoStaticXmlLimits,
        StreamInfoVolatileFieldInput, StreamInfoVolatileFieldLimits, StreamInfoVolatileFields,
        StreamInfoVolatileXml, StreamInfoVolatileXmlLimits, XmlCharacterDataLimit,
        XmlElementTreeLimits, XmlNameLimit, XmlTextLimit,
    };

    const VOLATILE_NAMES: [&str; 11] = [
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
    const VOLATILE_VALUES: [&str; 11] = [
        "1.100000000000000",
        "NORMALIZED_CREATED_AT",
        "NORMALIZED_UID",
        "NORMALIZED_SESSION_ID",
        "NORMALIZED_HOSTNAME",
        "NORMALIZED_V4ADDRESS",
        "NORMALIZED_V4DATA_PORT",
        "NORMALIZED_V4SERVICE_PORT",
        "NORMALIZED_V6ADDRESS",
        "NORMALIZED_V6DATA_PORT",
        "NORMALIZED_V6SERVICE_PORT",
    ];

    fn ordered(
        name: &str,
        kind: &str,
        source_id: &str,
        channel_count: usize,
        rate: NominalSampleRate,
        format: ChannelFormat,
        description_nodes: Vec<MetadataNodeInput>,
    ) -> StreamInfoOrderedXml {
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
        let placeholder = MetadataTree::new(
            MetadataTreeLimits::new(1, 1, 1, 16, 16).unwrap(),
            vec![MetadataNodeInput::new(None, "unused".to_owned(), None)],
        )
        .unwrap();
        let definition = StreamDefinition::new(descriptor, placeholder);
        let static_fields = StreamInfoStaticFields::new(&definition);
        let static_xml = StreamInfoStaticXml::compose(
            &static_fields,
            StreamInfoStaticXmlLimits::new(
                XmlNameLimit::new(32).unwrap(),
                XmlTextLimit::new(64).unwrap(),
                XmlCharacterDataLimit::new(256).unwrap(),
                XmlElementTreeLimits::new(7, 2, 6, 1024).unwrap(),
            ),
        )
        .unwrap();
        let description_count = description_nodes.len();
        let metadata = MetadataTree::new(
            MetadataTreeLimits::new(description_count, 8, 8, 32, 64).unwrap(),
            description_nodes,
        )
        .unwrap();
        let description = project_metadata_tree_to_xml_element_tree(
            metadata,
            MetadataXmlProjectionLimits::new(
                XmlNameLimit::new(32).unwrap(),
                XmlTextLimit::new(64).unwrap(),
                XmlCharacterDataLimit::new(256).unwrap(),
                XmlElementTreeLimits::new(description_count, 8, 8, 4096).unwrap(),
            ),
        )
        .unwrap();
        let static_description = StreamInfoDescriptionXml::compose(
            static_xml,
            description,
            XmlElementTreeLimits::new(7 + description_count, 9, 7, 8192).unwrap(),
        )
        .unwrap();

        let [a, b, c, d, e, f, g, h, i, j, k] = VOLATILE_VALUES;
        let volatile_fields = StreamInfoVolatileFields::new(
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
        let volatile_xml = StreamInfoVolatileXml::compose(
            &volatile_fields,
            StreamInfoVolatileXmlLimits::new(
                XmlNameLimit::new(32).unwrap(),
                XmlTextLimit::new(64).unwrap(),
                XmlCharacterDataLimit::new(256).unwrap(),
                XmlElementTreeLimits::new(12, 2, 11, 4096).unwrap(),
            ),
        )
        .unwrap();
        let total = static_description.tree().nodes().len() + 11;
        StreamInfoOrderedXml::compose(
            static_description,
            volatile_xml,
            XmlElementTreeLimits::new(total, 10, 18, 16384).unwrap(),
        )
        .unwrap()
    }

    fn empty_desc() -> Vec<MetadataNodeInput> {
        vec![MetadataNodeInput::new(None, "desc".to_owned(), None)]
    }

    fn nested_desc() -> Vec<MetadataNodeInput> {
        vec![
            MetadataNodeInput::new(None, "desc".to_owned(), None),
            MetadataNodeInput::new(Some(0), "ordered".to_owned(), None),
            MetadataNodeInput::new(
                Some(1),
                "first".to_owned(),
                Some("alpha-α-&-<-greater->-\"-'".to_owned()),
            ),
            MetadataNodeInput::new(Some(1), "second".to_owned(), Some("beta-β-]]>".to_owned())),
            MetadataNodeInput::new(Some(1), "nested".to_owned(), None),
            MetadataNodeInput::new(
                Some(4),
                "third".to_owned(),
                Some("tail-尾-&-<-greater->".to_owned()),
            ),
        ]
    }

    fn represented(value: &str) -> String {
        value
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    }

    fn expected(
        name: &str,
        kind: &str,
        source_id: &str,
        count: usize,
        format: &str,
        rate: &str,
        nested: bool,
    ) -> String {
        let mut output = String::from("<?xml version=\"1.0\"?>\n<info>\n");
        for (field, value) in [
            ("name", represented(name)),
            ("type", represented(kind)),
            ("channel_count", count.to_string()),
            ("channel_format", format.to_owned()),
            ("source_id", represented(source_id)),
            ("nominal_srate", rate.to_owned()),
        ] {
            output.push_str(&format!("\t<{field}>{value}</{field}>\n"));
        }
        for (field, value) in VOLATILE_NAMES.into_iter().zip(VOLATILE_VALUES) {
            output.push_str(&format!("\t<{field}>{value}</{field}>\n"));
        }
        if nested {
            output.push_str("\t<desc>\n\t\t<ordered>\n\t\t\t<first>alpha-α-&amp;-&lt;-greater-&gt;-\"-'</first>\n\t\t\t<second>beta-β-]]&gt;</second>\n\t\t\t<nested>\n\t\t\t\t<third>tail-尾-&amp;-&lt;-greater-&gt;</third>\n\t\t\t</nested>\n\t\t</ordered>\n\t</desc>\n");
        } else {
            output.push_str("\t<desc />\n");
        }
        output.push_str("</info>\n");
        output
    }

    #[test]
    fn lslc_001r_seven_normalized_observations_match_exact_bytes() {
        let cases = [
            (
                "neutral-float32",
                "",
                "",
                1,
                NominalSampleRate::irregular(),
                ChannelFormat::Float32,
                "float32",
                "0.000000000000000",
                false,
            ),
            (
                "neutral-double64",
                "measurement",
                "source-double64",
                2,
                NominalSampleRate::regular_hz(100.0).unwrap(),
                ChannelFormat::Double64,
                "double64",
                "100.0000000000000",
                false,
            ),
            (
                "unicode-Ω-中-&-<-greater->",
                "text-&-<-greater->-\"-'",
                "source-雪-&-<-greater->",
                3,
                NominalSampleRate::regular_hz(59.94).unwrap(),
                ChannelFormat::String,
                "string",
                "59.94000000000000",
                true,
            ),
            (
                "neutral-int32",
                "integer",
                "source-int32",
                4,
                NominalSampleRate::regular_hz(1.0).unwrap(),
                ChannelFormat::Int32,
                "int32",
                "1.000000000000000",
                false,
            ),
            (
                "neutral-int16",
                "integer",
                "source-int16",
                5,
                NominalSampleRate::regular_hz(256.5).unwrap(),
                ChannelFormat::Int16,
                "int16",
                "256.5000000000000",
                false,
            ),
            (
                "neutral-int8",
                "integer",
                "source-int8",
                6,
                NominalSampleRate::irregular(),
                ChannelFormat::Int8,
                "int8",
                "0.000000000000000",
                false,
            ),
            (
                "neutral-int64",
                "integer",
                "source-int64",
                7,
                NominalSampleRate::regular_hz(1_000_000.25).unwrap(),
                ChannelFormat::Int64,
                "int64",
                "1000000.250000000",
                false,
            ),
        ];
        for (name, kind, source, count, rate, channel_format, spelling, rate_spelling, nested) in
            cases
        {
            let tree = ordered(
                name,
                kind,
                source,
                count,
                rate,
                channel_format,
                if nested { nested_desc() } else { empty_desc() },
            );
            let expected = expected(name, kind, source, count, spelling, rate_spelling, nested);
            let actual = StreamInfoObservedDocument::project(
                StreamInfoObservedDocumentLimit::new(expected.len()).unwrap(),
                &tree,
            )
            .unwrap();
            assert_eq!(actual.as_str().as_bytes(), expected.as_bytes());
        }
    }

    #[test]
    fn lslc_001r_exact_limit_borrow_and_consuming_output_preserve_allocations() {
        let tree = ordered(
            "n",
            "",
            "",
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
            empty_desc(),
        );
        let source_pointer = tree.tree().nodes()[1]
            .value()
            .as_leaf()
            .unwrap()
            .name()
            .as_str()
            .as_ptr();
        let required = expected("n", "", "", 1, "float32", "0.000000000000000", false).len();
        let limit = StreamInfoObservedDocumentLimit::new(required).unwrap();
        let document = StreamInfoObservedDocument::project(limit, &tree).unwrap();
        let output_pointer = document.as_str().as_ptr();
        assert_eq!(document.limit(), limit);
        assert_eq!(
            tree.tree().nodes()[1]
                .value()
                .as_leaf()
                .unwrap()
                .name()
                .as_str()
                .as_ptr(),
            source_pointer
        );
        let output = document.into_string();
        assert_eq!(output.as_ptr(), output_pointer);
    }

    #[test]
    fn lslc_001r_one_past_limit_reports_exact_required_bytes() {
        let tree = ordered(
            "n",
            "",
            "",
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
            empty_desc(),
        );
        let required = expected("n", "", "", 1, "float32", "0.000000000000000", false).len();
        assert_eq!(
            StreamInfoObservedDocument::project(
                StreamInfoObservedDocumentLimit::new(required - 1).unwrap(),
                &tree
            ),
            Err(StreamInfoObservedDocumentError::OutputLimitExceeded {
                expected_max: required - 1,
                required
            })
        );
        assert_eq!(
            StreamInfoObservedDocumentLimit::new(0),
            Err(StreamInfoObservedDocumentError::InvalidLimit {
                expected_min: 1,
                actual: 0
            })
        );
    }

    #[test]
    fn lslc_001r_childless_non_desc_container_fails_closed() {
        let tree = ordered(
            "n",
            "",
            "",
            1,
            NominalSampleRate::irregular(),
            ChannelFormat::Float32,
            vec![
                MetadataNodeInput::new(None, "desc".to_owned(), None),
                MetadataNodeInput::new(Some(0), "unobserved".to_owned(), None),
            ],
        );
        assert_eq!(
            StreamInfoObservedDocument::project(
                StreamInfoObservedDocumentLimit::new(4096).unwrap(),
                &tree
            ),
            Err(StreamInfoObservedDocumentError::UnsupportedEmptyContainer { node_index: 19 })
        );
    }
}
