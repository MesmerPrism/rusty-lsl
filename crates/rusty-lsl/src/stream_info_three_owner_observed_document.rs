// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    StreamInfoDescriptionXml, StreamInfoObservedDocument, StreamInfoObservedDocumentError,
    StreamInfoObservedDocumentLimit, StreamInfoOrderedXml, StreamInfoOrderedXmlError,
    StreamInfoThreeOwnerEvidence, StreamInfoThreeOwnerSnapshot, StreamInfoVolatileXml,
    StreamInfoVolatileXmlError, StreamInfoVolatileXmlLimits, XmlElementTreeLimits,
};
use core::fmt;

/// Caller-selected bounds for the existing P, Q, and R composition stages.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamInfoThreeOwnerObservedDocumentLimits {
    volatile_xml: StreamInfoVolatileXmlLimits,
    ordered_xml: XmlElementTreeLimits,
    observed_document: StreamInfoObservedDocumentLimit,
}

impl StreamInfoThreeOwnerObservedDocumentLimits {
    /// Groups existing accepted component limits without adding defaults.
    #[must_use]
    pub const fn new(
        volatile_xml: StreamInfoVolatileXmlLimits,
        ordered_xml: XmlElementTreeLimits,
        observed_document: StreamInfoObservedDocumentLimit,
    ) -> Self {
        Self {
            volatile_xml,
            ordered_xml,
            observed_document,
        }
    }

    /// Returns the P volatile-XML limits.
    #[must_use]
    pub const fn volatile_xml(self) -> StreamInfoVolatileXmlLimits {
        self.volatile_xml
    }

    /// Returns the Q ordered-tree limits.
    #[must_use]
    pub const fn ordered_xml(self) -> XmlElementTreeLimits {
        self.ordered_xml
    }

    /// Returns the R observed-document limit.
    #[must_use]
    pub const fn observed_document(self) -> StreamInfoObservedDocumentLimit {
        self.observed_document
    }
}

/// One bounded observed document paired with three separate owner witnesses.
///
/// This local accepted state has no acquisition, parser, endpoint, wire,
/// runtime, freshness, authorization, activation, or Manifold authority meaning.
#[derive(Debug, Eq, PartialEq)]
pub struct StreamInfoThreeOwnerObservedDocument {
    evidence: StreamInfoThreeOwnerEvidence,
    document: StreamInfoObservedDocument,
}

impl StreamInfoThreeOwnerObservedDocument {
    /// Composes accepted N and X state through the existing P, Q, and R contracts.
    pub fn compose(
        static_description: StreamInfoDescriptionXml,
        snapshot: StreamInfoThreeOwnerSnapshot,
        limits: StreamInfoThreeOwnerObservedDocumentLimits,
    ) -> Result<Self, StreamInfoThreeOwnerObservedDocumentError> {
        let volatile =
            StreamInfoVolatileXml::compose(snapshot.snapshot().fields(), limits.volatile_xml)
                .map_err(StreamInfoThreeOwnerObservedDocumentError::VolatileXml)?;
        let ordered =
            StreamInfoOrderedXml::compose(static_description, volatile, limits.ordered_xml)
                .map_err(StreamInfoThreeOwnerObservedDocumentError::OrderedXml)?;
        let document = StreamInfoObservedDocument::project(limits.observed_document, &ordered)
            .map_err(StreamInfoThreeOwnerObservedDocumentError::ObservedDocument)?;
        let (evidence, _) = snapshot.into_parts();
        Ok(Self { evidence, document })
    }

    /// Returns the three unchanged, separately inspectable owner witnesses.
    #[must_use]
    pub const fn evidence(&self) -> &StreamInfoThreeOwnerEvidence {
        &self.evidence
    }

    /// Returns the bounded observed document.
    #[must_use]
    pub const fn document(&self) -> &StreamInfoObservedDocument {
        &self.document
    }

    /// Moves the separate evidence and document out unchanged.
    #[must_use]
    pub fn into_parts(self) -> (StreamInfoThreeOwnerEvidence, StreamInfoObservedDocument) {
        (self.evidence, self.document)
    }
}

/// Deterministic rejection from one existing P, Q, or R stage.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum StreamInfoThreeOwnerObservedDocumentError {
    /// P rejected the borrowed accepted volatile fields.
    VolatileXml(StreamInfoVolatileXmlError),
    /// Q rejected the accepted component trees.
    OrderedXml(StreamInfoOrderedXmlError),
    /// R rejected the accepted ordered tree or output bound.
    ObservedDocument(StreamInfoObservedDocumentError),
}

impl fmt::Display for StreamInfoThreeOwnerObservedDocumentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "three-owner observed document rejected input: {self:?}"
        )
    }
}

impl std::error::Error for StreamInfoThreeOwnerObservedDocumentError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        project_metadata_tree_to_xml_element_tree, ChannelFormat, MetadataNodeInput, MetadataTree,
        MetadataTreeLimits, MetadataXmlProjectionLimits, NominalSampleRate, StreamDefinition,
        StreamDescriptor, StreamDescriptorLimits, StreamInfoImplementationVersionAcquisition,
        StreamInfoImplementationVersionEvidenceLimit, StreamInfoImplementationVersionProvider,
        StreamInfoImplementationVersionProviderOutput, StreamInfoImplementationVersionWitness,
        StreamInfoRuntimeAcquisition, StreamInfoRuntimeEvidenceLimit, StreamInfoRuntimeProvider,
        StreamInfoRuntimeProviderOutput, StreamInfoRuntimeValues, StreamInfoRuntimeWitness,
        StreamInfoStaticFields, StreamInfoStaticXml, StreamInfoStaticXmlLimits,
        StreamInfoTransportAcquisition, StreamInfoTransportEvidenceLimit,
        StreamInfoTransportProvider, StreamInfoTransportProviderOutput, StreamInfoTransportValues,
        StreamInfoTransportWitness, StreamInfoVolatileFieldLimits, XmlCharacterDataLimit,
        XmlNameLimit, XmlTextLimit,
    };

    struct ImplementationProvider(Option<StreamInfoImplementationVersionProviderOutput>);
    impl StreamInfoImplementationVersionProvider for ImplementationProvider {
        type Error = ();
        fn acquire(
            &mut self,
        ) -> Result<StreamInfoImplementationVersionProviderOutput, Self::Error> {
            self.0.take().ok_or(())
        }
    }

    struct RuntimeProvider(Option<StreamInfoRuntimeProviderOutput>);
    impl StreamInfoRuntimeProvider for RuntimeProvider {
        type Error = ();
        fn acquire(&mut self) -> Result<StreamInfoRuntimeProviderOutput, Self::Error> {
            self.0.take().ok_or(())
        }
    }

    struct TransportProvider(Option<StreamInfoTransportProviderOutput>);
    impl StreamInfoTransportProvider for TransportProvider {
        type Error = ();
        fn acquire(&mut self) -> Result<StreamInfoTransportProviderOutput, Self::Error> {
            self.0.take().ok_or(())
        }
    }

    fn static_description() -> StreamInfoDescriptionXml {
        let descriptor = StreamDescriptor::new(
            StreamDescriptorLimits::new(32, 32, 32, 8).unwrap(),
            "name".into(),
            Some("type".into()),
            Some("source".into()),
            1,
            NominalSampleRate::Irregular,
            ChannelFormat::Float32,
        )
        .unwrap();
        let placeholder = MetadataTree::new(
            MetadataTreeLimits::new(1, 1, 1, 16, 16).unwrap(),
            vec![MetadataNodeInput::new(None, "unused".into(), None)],
        )
        .unwrap();
        let definition = StreamDefinition::new(descriptor, placeholder);
        let static_xml = StreamInfoStaticXml::compose(
            &StreamInfoStaticFields::new(&definition),
            StreamInfoStaticXmlLimits::new(
                XmlNameLimit::new(32).unwrap(),
                XmlTextLimit::new(32).unwrap(),
                XmlCharacterDataLimit::new(64).unwrap(),
                XmlElementTreeLimits::new(7, 2, 6, 512).unwrap(),
            ),
        )
        .unwrap();
        let description = project_metadata_tree_to_xml_element_tree(
            MetadataTree::new(
                MetadataTreeLimits::new(1, 1, 1, 16, 16).unwrap(),
                vec![MetadataNodeInput::new(None, "desc".into(), None)],
            )
            .unwrap(),
            MetadataXmlProjectionLimits::new(
                XmlNameLimit::new(16).unwrap(),
                XmlTextLimit::new(16).unwrap(),
                XmlCharacterDataLimit::new(16).unwrap(),
                XmlElementTreeLimits::new(1, 1, 1, 16).unwrap(),
            ),
        )
        .unwrap();
        StreamInfoDescriptionXml::compose(
            static_xml,
            description,
            XmlElementTreeLimits::new(8, 2, 7, 528).unwrap(),
        )
        .unwrap()
    }

    fn snapshot() -> StreamInfoThreeOwnerSnapshot {
        let field_limits = StreamInfoVolatileFieldLimits::new(32, 32, 32).unwrap();
        let implementation_witness = StreamInfoImplementationVersionWitness::new(
            StreamInfoImplementationVersionEvidenceLimit::new(32).unwrap(),
            "implementation-owner".into(),
            1,
            11,
        )
        .unwrap();
        let mut implementation =
            ImplementationProvider(Some(StreamInfoImplementationVersionProviderOutput::new(
                StreamInfoImplementationVersionWitness::new(
                    StreamInfoImplementationVersionEvidenceLimit::new(32).unwrap(),
                    "implementation-owner".into(),
                    1,
                    11,
                )
                .unwrap(),
                "version".into(),
            )));
        let implementation = StreamInfoImplementationVersionAcquisition::acquire(
            &mut implementation,
            &implementation_witness,
            field_limits,
        )
        .unwrap();

        let runtime_witness = StreamInfoRuntimeWitness::new(
            StreamInfoRuntimeEvidenceLimit::new(32).unwrap(),
            "runtime-owner".into(),
            2,
            22,
        )
        .unwrap();
        let mut runtime = RuntimeProvider(Some(StreamInfoRuntimeProviderOutput::new(
            StreamInfoRuntimeWitness::new(
                StreamInfoRuntimeEvidenceLimit::new(32).unwrap(),
                "runtime-owner".into(),
                2,
                22,
            )
            .unwrap(),
            StreamInfoRuntimeValues::new(
                "created".into(),
                "uid".into(),
                "session".into(),
                "host".into(),
            ),
        )));
        let runtime =
            StreamInfoRuntimeAcquisition::acquire(&mut runtime, &runtime_witness, field_limits)
                .unwrap();

        let transport_witness = StreamInfoTransportWitness::new(
            StreamInfoTransportEvidenceLimit::new(32).unwrap(),
            "transport-owner".into(),
            3,
            33,
        )
        .unwrap();
        let mut transport = TransportProvider(Some(StreamInfoTransportProviderOutput::new(
            StreamInfoTransportWitness::new(
                StreamInfoTransportEvidenceLimit::new(32).unwrap(),
                "transport-owner".into(),
                3,
                33,
            )
            .unwrap(),
            StreamInfoTransportValues::new(
                "v4".into(),
                "v4data".into(),
                "v4service".into(),
                "v6".into(),
                "v6data".into(),
                "v6service".into(),
            ),
        )));
        let transport = StreamInfoTransportAcquisition::acquire(
            &mut transport,
            &transport_witness,
            field_limits,
        )
        .unwrap();
        StreamInfoThreeOwnerSnapshot::new(field_limits, implementation, runtime, transport).unwrap()
    }

    fn limits(document_bytes: usize) -> StreamInfoThreeOwnerObservedDocumentLimits {
        StreamInfoThreeOwnerObservedDocumentLimits::new(
            StreamInfoVolatileXmlLimits::new(
                XmlNameLimit::new(32).unwrap(),
                XmlTextLimit::new(32).unwrap(),
                XmlCharacterDataLimit::new(64).unwrap(),
                XmlElementTreeLimits::new(12, 2, 11, 1024).unwrap(),
            ),
            XmlElementTreeLimits::new(19, 2, 18, 2048).unwrap(),
            StreamInfoObservedDocumentLimit::new(document_bytes).unwrap(),
        )
    }

    #[test]
    fn lslc_001z_one_call_produces_document_and_retains_three_witnesses() {
        let accepted = StreamInfoThreeOwnerObservedDocument::compose(
            static_description(),
            snapshot(),
            limits(4096),
        )
        .unwrap();
        assert!(accepted
            .document()
            .as_str()
            .starts_with("<?xml version=\"1.0\"?>\n<info>\n"));
        assert!(accepted
            .document()
            .as_str()
            .contains("\t<version>version</version>\n"));
        assert!(accepted
            .document()
            .as_str()
            .ends_with("\t<desc />\n</info>\n"));
        assert_eq!(accepted.evidence().implementation().revision(), 11);
        assert_eq!(accepted.evidence().runtime().revision(), 22);
        assert_eq!(accepted.evidence().transport().revision(), 33);
    }

    #[test]
    fn lslc_001z_r_rejection_remains_stage_typed() {
        assert!(matches!(
            StreamInfoThreeOwnerObservedDocument::compose(
                static_description(),
                snapshot(),
                limits(1),
            ),
            Err(StreamInfoThreeOwnerObservedDocumentError::ObservedDocument(
                _
            ))
        ));
    }
}
