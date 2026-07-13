// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

/// Identifies one configured metadata bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MetadataBound {
    /// Maximum number of descriptions in one metadata value.
    Descriptions,
    /// Maximum number of fields in one description.
    FieldsPerDescription,
    /// Maximum Unicode scalar-value count in one field name or value.
    TextCodePoints,
}

/// Identifies which text member violated a metadata bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MetadataTextRole {
    /// The field name.
    Name,
    /// The field value.
    Value,
}

/// Limits applied atomically when constructing [`BoundedMetadata`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MetadataLimits {
    max_descriptions: usize,
    max_fields_per_description: usize,
    max_text_code_points: usize,
}

impl MetadataLimits {
    /// Validates a complete metadata limit configuration.
    ///
    /// Every maximum must be at least one. The first invalid bound in argument
    /// order is returned as a stable [`MetadataError::InvalidLimit`].
    pub fn new(
        max_descriptions: usize,
        max_fields_per_description: usize,
        max_text_code_points: usize,
    ) -> Result<Self, MetadataError> {
        for (bound, actual) in [
            (MetadataBound::Descriptions, max_descriptions),
            (
                MetadataBound::FieldsPerDescription,
                max_fields_per_description,
            ),
            (MetadataBound::TextCodePoints, max_text_code_points),
        ] {
            if actual == 0 {
                return Err(MetadataError::InvalidLimit {
                    bound,
                    expected_min: 1,
                    actual,
                });
            }
        }

        Ok(Self {
            max_descriptions,
            max_fields_per_description,
            max_text_code_points,
        })
    }

    /// Returns the maximum accepted description count.
    #[must_use]
    pub const fn max_descriptions(self) -> usize {
        self.max_descriptions
    }

    /// Returns the maximum accepted field count per description.
    #[must_use]
    pub const fn max_fields_per_description(self) -> usize {
        self.max_fields_per_description
    }

    /// Returns the maximum accepted Unicode scalar-value count per text member.
    #[must_use]
    pub const fn max_text_code_points(self) -> usize {
        self.max_text_code_points
    }
}

/// One caller-owned metadata name/value pair.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataField {
    name: String,
    value: String,
}

impl MetadataField {
    /// Returns the unchanged field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the unchanged field value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns both unchanged owned strings.
    #[must_use]
    pub fn into_parts(self) -> (String, String) {
        (self.name, self.value)
    }
}

/// One caller-ordered metadata field collection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataDescription {
    fields: Vec<MetadataField>,
}

impl MetadataDescription {
    /// Returns the unchanged ordered fields.
    #[must_use]
    pub fn fields(&self) -> &[MetadataField] {
        &self.fields
    }

    /// Returns the unchanged owned fields.
    #[must_use]
    pub fn into_fields(self) -> Vec<MetadataField> {
        self.fields
    }
}

/// Metadata accepted atomically under explicit caller-visible limits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedMetadata {
    limits: MetadataLimits,
    descriptions: Vec<MetadataDescription>,
}

impl BoundedMetadata {
    /// Validates every collection and text bound before returning a value.
    ///
    /// Text sizes are counted in Unicode scalar values. Validation does not
    /// normalize, trim, reorder, or otherwise alter accepted strings.
    pub fn new(
        limits: MetadataLimits,
        descriptions: Vec<Vec<(String, String)>>,
    ) -> Result<Self, MetadataError> {
        if descriptions.len() > limits.max_descriptions {
            return Err(MetadataError::LimitExceeded {
                bound: MetadataBound::Descriptions,
                expected_max: limits.max_descriptions,
                actual: descriptions.len(),
                description_index: None,
                field_index: None,
                text_role: None,
            });
        }

        for (description_index, description) in descriptions.iter().enumerate() {
            if description.len() > limits.max_fields_per_description {
                return Err(MetadataError::LimitExceeded {
                    bound: MetadataBound::FieldsPerDescription,
                    expected_max: limits.max_fields_per_description,
                    actual: description.len(),
                    description_index: Some(description_index),
                    field_index: None,
                    text_role: None,
                });
            }

            for (field_index, field) in description.iter().enumerate() {
                for (text_role, text) in [
                    (MetadataTextRole::Name, field.0.as_str()),
                    (MetadataTextRole::Value, field.1.as_str()),
                ] {
                    let actual = text.chars().count();
                    if actual > limits.max_text_code_points {
                        return Err(MetadataError::LimitExceeded {
                            bound: MetadataBound::TextCodePoints,
                            expected_max: limits.max_text_code_points,
                            actual,
                            description_index: Some(description_index),
                            field_index: Some(field_index),
                            text_role: Some(text_role),
                        });
                    }
                }
            }
        }

        let descriptions = descriptions
            .into_iter()
            .map(|fields| MetadataDescription {
                fields: fields
                    .into_iter()
                    .map(|(name, value)| MetadataField { name, value })
                    .collect(),
            })
            .collect();

        Ok(Self {
            limits,
            descriptions,
        })
    }

    /// Returns the limits under which this value was accepted.
    #[must_use]
    pub const fn limits(&self) -> MetadataLimits {
        self.limits
    }

    /// Returns the unchanged ordered descriptions.
    #[must_use]
    pub fn descriptions(&self) -> &[MetadataDescription] {
        &self.descriptions
    }

    /// Returns the unchanged owned descriptions.
    #[must_use]
    pub fn into_descriptions(self) -> Vec<MetadataDescription> {
        self.descriptions
    }
}

/// Deterministic rejection from metadata limit configuration or construction.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MetadataError {
    /// A limit configuration cannot accept any value for the named bound.
    InvalidLimit {
        /// The malformed bound.
        bound: MetadataBound,
        /// The smallest accepted configuration value.
        expected_min: usize,
        /// The caller-provided configuration value.
        actual: usize,
    },
    /// Input exceeded one configured maximum.
    LimitExceeded {
        /// The violated bound.
        bound: MetadataBound,
        /// The configured maximum.
        expected_max: usize,
        /// The observed collection or Unicode scalar-value count.
        actual: usize,
        /// Zero-based description location when applicable.
        description_index: Option<usize>,
        /// Zero-based field location when applicable.
        field_index: Option<usize>,
        /// Text member location when applicable.
        text_role: Option<MetadataTextRole>,
    },
}

impl fmt::Display for MetadataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "metadata bounds rejected input: {self:?}")
    }
}

impl std::error::Error for MetadataError {}

#[cfg(test)]
mod tests {
    use super::{
        BoundedMetadata, MetadataBound, MetadataError, MetadataField, MetadataLimits,
        MetadataTextRole,
    };

    fn field(name: &str, value: &str) -> (String, String) {
        (name.to_owned(), value.to_owned())
    }

    fn into_raw(metadata: BoundedMetadata) -> Vec<Vec<(String, String)>> {
        metadata
            .into_descriptions()
            .into_iter()
            .map(|description| {
                description
                    .into_fields()
                    .into_iter()
                    .map(MetadataField::into_parts)
                    .collect()
            })
            .collect()
    }

    #[test]
    fn contract_metadata_bounds_exact_limit_and_values_unchanged() {
        let limits = MetadataLimits::new(2, 2, 3).unwrap();
        let expected = vec![
            vec![field("Aß中", " x "), field("two", "ééé")],
            vec![field("cat", "dog"), field("one", "two")],
        ];

        let accepted = BoundedMetadata::new(limits, expected.clone()).unwrap();

        assert_eq!(accepted.limits(), limits);
        assert_eq!(accepted.descriptions()[0].fields()[0].name(), "Aß中");
        assert_eq!(accepted.descriptions()[0].fields()[0].value(), " x ");
        assert_eq!(into_raw(accepted), expected);
    }

    #[test]
    fn contract_metadata_bounds_one_past_description_limit_rejected() {
        let limits = MetadataLimits::new(1, 1, 1).unwrap();
        let error = BoundedMetadata::new(limits, vec![vec![], vec![]]).unwrap_err();

        assert_eq!(
            error,
            MetadataError::LimitExceeded {
                bound: MetadataBound::Descriptions,
                expected_max: 1,
                actual: 2,
                description_index: None,
                field_index: None,
                text_role: None,
            }
        );
    }

    #[test]
    fn contract_metadata_bounds_one_past_field_limit_rejected() {
        let limits = MetadataLimits::new(1, 1, 1).unwrap();
        let error =
            BoundedMetadata::new(limits, vec![vec![field("a", "b"), field("c", "d")]]).unwrap_err();

        assert_eq!(
            error,
            MetadataError::LimitExceeded {
                bound: MetadataBound::FieldsPerDescription,
                expected_max: 1,
                actual: 2,
                description_index: Some(0),
                field_index: None,
                text_role: None,
            }
        );
    }

    #[test]
    fn contract_metadata_bounds_one_past_text_limit_rejected() {
        let limits = MetadataLimits::new(1, 1, 3).unwrap();
        let error = BoundedMetadata::new(limits, vec![vec![field("ok", "四五六七")]]).unwrap_err();

        assert_eq!(
            error,
            MetadataError::LimitExceeded {
                bound: MetadataBound::TextCodePoints,
                expected_max: 3,
                actual: 4,
                description_index: Some(0),
                field_index: Some(0),
                text_role: Some(MetadataTextRole::Value),
            }
        );
    }

    #[test]
    fn contract_metadata_bounds_zero_limits_rejected_deterministically() {
        for (values, expected_bound) in [
            ((0, 1, 1), MetadataBound::Descriptions),
            ((1, 0, 1), MetadataBound::FieldsPerDescription),
            ((1, 1, 0), MetadataBound::TextCodePoints),
        ] {
            assert_eq!(
                MetadataLimits::new(values.0, values.1, values.2),
                Err(MetadataError::InvalidLimit {
                    bound: expected_bound,
                    expected_min: 1,
                    actual: 0,
                })
            );
        }
    }
}
