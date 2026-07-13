// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    StreamInfoVolatileFields, XmlCharacterData, XmlCharacterDataError, XmlCharacterDataLimit,
    XmlElementName, XmlElementNodeInput, XmlElementNodeValue, XmlElementTree, XmlElementTreeError,
    XmlElementTreeLimits, XmlLeafElement, XmlNameError, XmlNameLimit, XmlText, XmlTextError,
    XmlTextLimit,
};
use core::fmt;

const NODE_COUNT: usize = 12;
const ROOT_INDEX: usize = 0;
const ROOT_NAME: &str = "info";
const FIELD_NAMES: [&str; 11] = [
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

/// Caller-selected accepted XML bounds for one volatile stream-info projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamInfoVolatileXmlLimits {
    name: XmlNameLimit,
    text: XmlTextLimit,
    character_data: XmlCharacterDataLimit,
    tree: XmlElementTreeLimits,
}

impl StreamInfoVolatileXmlLimits {
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

/// One bounded owned `info` tree containing only eleven volatile field leaves.
///
/// The source accepted-data value remains borrowed. This local tree has no
/// provider, declaration, observed whitespace, complete-document, endpoint,
/// transport, runtime, or authority meaning.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoVolatileXml {
    limits: StreamInfoVolatileXmlLimits,
    tree: XmlElementTree,
}

impl StreamInfoVolatileXml {
    /// Projects one borrowed accepted volatile field set into a twelve-node tree.
    pub fn compose(
        fields: &StreamInfoVolatileFields,
        limits: StreamInfoVolatileXmlLimits,
    ) -> Result<Self, StreamInfoVolatileXmlError> {
        if limits.tree.max_nodes() < NODE_COUNT {
            return Err(StreamInfoVolatileXmlError::ElementTree(
                XmlElementTreeError::NodeLimitExceeded {
                    expected_max: limits.tree.max_nodes(),
                    actual: NODE_COUNT,
                },
            ));
        }

        let mut nodes = Vec::new();
        nodes.try_reserve_exact(NODE_COUNT).map_err(|_| {
            StreamInfoVolatileXmlError::NodeAllocationFailed {
                requested: NODE_COUNT,
            }
        })?;
        nodes.push(XmlElementNodeInput::new(
            None,
            XmlElementNodeValue::Container(copy_name(ROOT_INDEX, ROOT_NAME, limits.name)?),
        ));

        for (offset, ((role, name), expected_name)) in StreamInfoVolatileFields::roles()
            .iter()
            .copied()
            .zip(FIELD_NAMES)
            .zip(FIELD_NAMES)
            .enumerate()
        {
            debug_assert_eq!(name, expected_name);
            let node_index = offset + 1;
            let name = copy_name(node_index, name, limits.name)?;
            let text = copy_text(node_index, fields.field(role), limits.text)?;
            let character_data =
                XmlCharacterData::encode(limits.character_data, &text).map_err(|source| {
                    StreamInfoVolatileXmlError::CharacterData { node_index, source }
                })?;
            nodes.push(XmlElementNodeInput::new(
                Some(ROOT_INDEX),
                XmlElementNodeValue::Leaf(XmlLeafElement::new(name, character_data)),
            ));
        }

        let tree = XmlElementTree::new(limits.tree, nodes)
            .map_err(StreamInfoVolatileXmlError::ElementTree)?;
        Ok(Self { limits, tree })
    }

    /// Returns the component limits used for this projection.
    #[must_use]
    pub const fn limits(&self) -> StreamInfoVolatileXmlLimits {
        self.limits
    }

    /// Returns the bounded twelve-node element tree.
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

/// Deterministic rejection from volatile stream-info XML projection.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamInfoVolatileXmlError {
    /// The exact twelve-node arena reserve failed.
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
    /// The accepted XML text contract rejected a copied opaque value.
    Text {
        /// The leaf node being prepared.
        node_index: usize,
        /// The unchanged XML text error.
        source: XmlTextError,
    },
    /// The accepted character-data contract rejected an opaque value.
    CharacterData {
        /// The leaf node being prepared.
        node_index: usize,
        /// The unchanged character-data error.
        source: XmlCharacterDataError,
    },
    /// The accepted element-tree contract rejected the completed candidate.
    ElementTree(XmlElementTreeError),
}

impl fmt::Display for StreamInfoVolatileXmlError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "volatile stream-info XML rejected input: {self:?}"
        )
    }
}

impl std::error::Error for StreamInfoVolatileXmlError {}

fn copy_name(
    node_index: usize,
    source: &str,
    limit: XmlNameLimit,
) -> Result<XmlElementName, StreamInfoVolatileXmlError> {
    let copied = copy_string(source).map_err(|requested| {
        StreamInfoVolatileXmlError::NameAllocationFailed {
            node_index,
            requested,
        }
    })?;
    XmlElementName::new(limit, copied)
        .map_err(|source| StreamInfoVolatileXmlError::Name { node_index, source })
}

fn copy_text(
    node_index: usize,
    source: &str,
    limit: XmlTextLimit,
) -> Result<XmlText, StreamInfoVolatileXmlError> {
    let copied = copy_string(source).map_err(|requested| {
        StreamInfoVolatileXmlError::TextAllocationFailed {
            node_index,
            requested,
        }
    })?;
    XmlText::new(limit, copied)
        .map_err(|source| StreamInfoVolatileXmlError::Text { node_index, source })
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
        StreamInfoVolatileXml, StreamInfoVolatileXmlError, StreamInfoVolatileXmlLimits,
        FIELD_NAMES, NODE_COUNT,
    };
    use crate::{
        StreamInfoVolatileFieldInput, StreamInfoVolatileFieldLimits, StreamInfoVolatileFields,
        XmlCharacterDataLimit, XmlElementSerialization, XmlElementSerializationLimit,
        XmlElementTreeError, XmlElementTreeLimits, XmlNameLimit, XmlTextError, XmlTextLimit,
    };

    fn fields(values: [&str; 11]) -> StreamInfoVolatileFields {
        let [a, b, c, d, e, f, g, h, i, j, k] = values;
        StreamInfoVolatileFields::new(
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
        .unwrap()
    }

    fn limits(max_text: usize, max_bytes: usize, max_nodes: usize) -> StreamInfoVolatileXmlLimits {
        StreamInfoVolatileXmlLimits::new(
            XmlNameLimit::new(32).unwrap(),
            XmlTextLimit::new(max_text).unwrap(),
            XmlCharacterDataLimit::new(max_bytes).unwrap(),
            XmlElementTreeLimits::new(max_nodes, 2, 11, 1024).unwrap(),
        )
    }

    #[test]
    fn lslc_001p_exact_order_and_representation_are_compact_and_local() {
        let values = ["1.1", "<&>", "", "s", "h", "v4", "1", "2", "v6", "3", "4"];
        let source = fields(values);
        let projected =
            StreamInfoVolatileXml::compose(&source, limits(16, 32, NODE_COUNT)).unwrap();
        let serialized = XmlElementSerialization::serialize(
            XmlElementSerializationLimit::new(1024).unwrap(),
            projected.tree(),
        )
        .unwrap();

        let mut expected = String::from("<info>");
        for (name, value) in FIELD_NAMES.into_iter().zip(values) {
            let represented = value
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;");
            expected.push_str(&format!("<{name}>{represented}</{name}>"));
        }
        expected.push_str("</info>");
        assert_eq!(serialized.as_str(), expected);
    }

    #[test]
    fn lslc_001p_source_is_borrowed_unchanged_and_copies_are_distinct() {
        let values = ["v", "t", "u", "s", "h", "a", "1", "2", "b", "3", "4"];
        let source = fields(values);
        let pointers = StreamInfoVolatileFields::roles().map(|role| source.field(role).as_ptr());
        let projected = StreamInfoVolatileXml::compose(&source, limits(1, 1, NODE_COUNT)).unwrap();
        for ((node, expected), pointer) in projected.tree().nodes()[1..]
            .iter()
            .zip(values)
            .zip(pointers)
        {
            let leaf = match node.value() {
                crate::XmlElementNodeValue::Leaf(leaf) => leaf,
                crate::XmlElementNodeValue::Container(_) => {
                    panic!("volatile field became container")
                }
            };
            assert_eq!(leaf.character_data().as_str(), expected);
            assert_ne!(leaf.character_data().as_str().as_ptr(), pointer);
        }
        for (role, expected) in StreamInfoVolatileFields::roles()
            .iter()
            .copied()
            .zip(values)
        {
            assert_eq!(source.field(role), expected);
        }
    }

    #[test]
    fn lslc_001p_target_node_bound_rejects_before_projection_allocation() {
        let source = fields([""; 11]);
        assert_eq!(
            StreamInfoVolatileXml::compose(&source, limits(1, 1, NODE_COUNT - 1)),
            Err(StreamInfoVolatileXmlError::ElementTree(
                XmlElementTreeError::NodeLimitExceeded {
                    expected_max: 11,
                    actual: 12
                }
            ))
        );
    }

    #[test]
    fn lslc_001p_first_text_failure_retains_fixed_node_index() {
        let source = fields(["", "ab", "", "", "", "", "", "", "", "", ""]);
        assert_eq!(
            StreamInfoVolatileXml::compose(&source, limits(1, 8, NODE_COUNT)),
            Err(StreamInfoVolatileXmlError::Text {
                node_index: 2,
                source: XmlTextError::LimitExceeded {
                    expected_max: 1,
                    actual: 2
                },
            })
        );
    }
}
