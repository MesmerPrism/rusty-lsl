// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{XmlCharacterData, XmlElementName};

/// One accepted XML element name composed with accepted character data.
///
/// This leaf-only value assigns no tag spelling, namespace interpretation,
/// tree position, document role, or LSL field mapping to either component.
#[derive(Debug, Eq, PartialEq)]
pub struct XmlLeafElement {
    name: XmlElementName,
    character_data: XmlCharacterData,
}

impl XmlLeafElement {
    /// Moves two already accepted components directly into leaf-only state.
    #[must_use]
    pub fn new(name: XmlElementName, character_data: XmlCharacterData) -> Self {
        Self {
            name,
            character_data,
        }
    }

    /// Returns the unchanged accepted element name.
    #[must_use]
    pub const fn name(&self) -> &XmlElementName {
        &self.name
    }

    /// Returns the unchanged accepted character data.
    #[must_use]
    pub const fn character_data(&self) -> &XmlCharacterData {
        &self.character_data
    }

    /// Returns both unchanged owned components.
    #[must_use]
    pub fn into_parts(self) -> (XmlElementName, XmlCharacterData) {
        (self.name, self.character_data)
    }
}

#[cfg(test)]
mod tests {
    use super::XmlLeafElement;
    use crate::{
        XmlCharacterData, XmlCharacterDataError, XmlCharacterDataLimit, XmlElementName,
        XmlNameError, XmlNameLimit, XmlText, XmlTextLimit,
    };

    fn name(value: &str) -> XmlElementName {
        XmlElementName::new(
            XmlNameLimit::new(value.chars().count().max(1)).unwrap(),
            value.to_owned(),
        )
        .unwrap()
    }

    fn character_data(value: &str) -> XmlCharacterData {
        let text = XmlText::new(
            XmlTextLimit::new(value.chars().count().max(1)).unwrap(),
            value.to_owned(),
        )
        .unwrap();
        XmlCharacterData::encode(XmlCharacterDataLimit::new(256).unwrap(), &text).unwrap()
    }

    #[test]
    fn lslc_001d_empty_character_data_and_colon_name_are_preserved() {
        let name = name("prefix:leaf");
        let character_data = character_data("");
        let element = XmlLeafElement::new(name, character_data);

        assert_eq!(element.name().as_str(), "prefix:leaf");
        assert_eq!(element.character_data().as_str(), "");
    }

    #[test]
    fn lslc_001d_unicode_and_represented_reference_text_are_preserved() {
        let name_limit = XmlNameLimit::new(4).unwrap();
        let character_data_limit = XmlCharacterDataLimit::new(64).unwrap();
        let name = XmlElementName::new(name_limit, "叶𐀀".to_owned()).unwrap();
        let text = XmlText::new(XmlTextLimit::new(6).unwrap(), "é&amp;".to_owned()).unwrap();
        let character_data = XmlCharacterData::encode(character_data_limit, &text).unwrap();
        let element = XmlLeafElement::new(name, character_data);

        assert_eq!(element.name().limit(), name_limit);
        assert_eq!(element.name().as_str(), "叶𐀀");
        assert_eq!(element.character_data().limit(), character_data_limit);
        assert_eq!(element.character_data().as_str(), "é&amp;amp;");
    }

    #[test]
    fn lslc_001d_into_parts_preserves_both_owned_allocations() {
        let name = name("leaf");
        let character_data = character_data("A&B");
        let name_pointer = name.as_str().as_ptr();
        let character_data_pointer = character_data.as_str().as_ptr();

        let element = XmlLeafElement::new(name, character_data);
        assert_eq!(element.name().as_str().as_ptr(), name_pointer);
        assert_eq!(
            element.character_data().as_str().as_ptr(),
            character_data_pointer
        );

        let (name, character_data) = element.into_parts();
        let name_string = name.into_string();
        let character_data_string = character_data.into_string();
        assert_eq!(name_string.as_ptr(), name_pointer);
        assert_eq!(character_data_string.as_ptr(), character_data_pointer);
    }

    #[test]
    fn lslc_001d_damaged_name_remains_name_authority_rejection() {
        assert_eq!(
            XmlElementName::new(XmlNameLimit::new(1).unwrap(), "1".to_owned()),
            Err(XmlNameError::InvalidStart {
                index: 0,
                code_point: u32::from('1'),
            })
        );
    }

    #[test]
    fn lslc_001d_damaged_character_data_remains_representation_authority_rejection() {
        let text = XmlText::new(XmlTextLimit::new(1).unwrap(), "&".to_owned()).unwrap();
        assert_eq!(
            XmlCharacterData::encode(XmlCharacterDataLimit::new(4).unwrap(), &text),
            Err(XmlCharacterDataError::LimitExceeded {
                expected_max: 4,
                required: 5,
            })
        );
    }
}
