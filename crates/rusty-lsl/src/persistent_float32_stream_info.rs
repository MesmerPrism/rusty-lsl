// Copyright (C) 2026 Rusty LSL contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Structured canonical stream-info composition for persistent Float32 outlets.

use crate::{
    project_metadata_tree_to_xml_element_tree, ChannelFormat, MetadataTree,
    MetadataXmlProjectionError, MetadataXmlProjectionLimits, NominalSampleRate,
    PersistentFloat32Outlet, StreamDefinition, StreamDescriptor, StreamDescriptorError,
    StreamDescriptorLimits, StreamInfoDescriptionXml, StreamInfoDescriptionXmlError,
    StreamInfoObservedDocument, StreamInfoObservedDocumentError, StreamInfoObservedDocumentLimit,
    StreamInfoOrderedXml, StreamInfoOrderedXmlError, StreamInfoStaticFields, StreamInfoStaticXml,
    StreamInfoStaticXmlError, StreamInfoStaticXmlLimits, StreamInfoVolatileFieldError,
    StreamInfoVolatileFieldInput, StreamInfoVolatileFieldLimits, StreamInfoVolatileFields,
    StreamInfoVolatileXml, StreamInfoVolatileXmlError, StreamInfoVolatileXmlLimits,
    XmlElementTreeLimits,
};
use crate::{
    PersistentFloat32OutletServiceCreateError, PersistentFloat32OutletServiceIdentityRole,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const IMPLEMENTATION_VERSION: &str = "1.100000000000000";

/// Caller-selected limits for every existing structured composition stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PersistentFloat32StreamInfoLimits {
    descriptor: StreamDescriptorLimits,
    static_xml: StreamInfoStaticXmlLimits,
    metadata_xml: MetadataXmlProjectionLimits,
    description_xml: XmlElementTreeLimits,
    volatile_fields: StreamInfoVolatileFieldLimits,
    volatile_xml: StreamInfoVolatileXmlLimits,
    ordered_xml: XmlElementTreeLimits,
    document: StreamInfoObservedDocumentLimit,
}

impl PersistentFloat32StreamInfoLimits {
    /// Groups the existing independently validated stage limits.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        descriptor: StreamDescriptorLimits,
        static_xml: StreamInfoStaticXmlLimits,
        metadata_xml: MetadataXmlProjectionLimits,
        description_xml: XmlElementTreeLimits,
        volatile_fields: StreamInfoVolatileFieldLimits,
        volatile_xml: StreamInfoVolatileXmlLimits,
        ordered_xml: XmlElementTreeLimits,
        document: StreamInfoObservedDocumentLimit,
    ) -> Self {
        Self {
            descriptor,
            static_xml,
            metadata_xml,
            description_xml,
            volatile_fields,
            volatile_xml,
            ordered_xml,
            document,
        }
    }
}

/// Owned structured caller input excluding transport identity and addresses.
#[derive(Debug, PartialEq)]
pub struct PersistentFloat32StreamInfoInput {
    name: String,
    stream_type: String,
    nominal_sample_rate: NominalSampleRate,
    description: MetadataTree,
}

impl PersistentFloat32StreamInfoInput {
    /// Groups a stream name/type/rate with one already validated `desc` metadata tree.
    #[must_use]
    pub fn new(
        name: String,
        stream_type: String,
        nominal_sample_rate: NominalSampleRate,
        description: MetadataTree,
    ) -> Self {
        Self {
            name,
            stream_type,
            nominal_sample_rate,
            description,
        }
    }
}

/// Canonical observed stream-info document derived from a live outlet.
#[derive(Debug, Eq, PartialEq)]
pub struct PersistentFloat32StreamInfo {
    body: String,
    advertised_ipv4: Ipv4Addr,
    local_address: SocketAddr,
    channel_count: usize,
    uid: String,
    hostname: String,
    source_id: String,
    session_id: String,
}

impl PersistentFloat32StreamInfo {
    /// Composes canonical discovery metadata without caller-written XML.
    ///
    /// UID, source ID, session ID, host name, channel count, data/service port,
    /// and the source-clock creation time are derived from the accepted outlet.
    ///
    /// # Errors
    ///
    /// Returns the exact existing stage that rejected descriptor, metadata, volatile,
    /// ordered-tree, or canonical document composition.
    pub fn compose(
        outlet: &PersistentFloat32Outlet,
        advertised_ipv4: Ipv4Addr,
        input: PersistentFloat32StreamInfoInput,
        limits: PersistentFloat32StreamInfoLimits,
    ) -> Result<Self, PersistentFloat32StreamInfoError> {
        if advertised_ipv4.is_unspecified()
            || advertised_ipv4.is_multicast()
            || advertised_ipv4 == Ipv4Addr::BROADCAST
        {
            return Err(PersistentFloat32StreamInfoError::NonConcreteIpv4Interface);
        }
        let PersistentFloat32StreamInfoInput {
            name,
            stream_type,
            nominal_sample_rate,
            description,
        } = input;
        let identity = outlet.stream_identity();
        let descriptor = StreamDescriptor::new(
            limits.descriptor,
            name,
            Some(stream_type),
            Some(identity.source_id().to_owned()),
            outlet.channel_count(),
            nominal_sample_rate,
            ChannelFormat::Float32,
        )
        .map_err(PersistentFloat32StreamInfoError::Descriptor)?;
        let definition = StreamDefinition::new(descriptor, description);
        let static_fields = StreamInfoStaticFields::new(&definition);
        let static_xml = StreamInfoStaticXml::compose(&static_fields, limits.static_xml)
            .map_err(PersistentFloat32StreamInfoError::StaticXml)?;
        let (_, description) = definition.into_parts();
        let description =
            project_metadata_tree_to_xml_element_tree(description, limits.metadata_xml)
                .map_err(PersistentFloat32StreamInfoError::MetadataXml)?;
        let static_description =
            StreamInfoDescriptionXml::compose(static_xml, description, limits.description_xml)
                .map_err(PersistentFloat32StreamInfoError::DescriptionXml)?;

        let port = outlet.local_address().port().to_string();
        let volatile_fields = StreamInfoVolatileFields::new(
            limits.volatile_fields,
            StreamInfoVolatileFieldInput::new(
                IMPLEMENTATION_VERSION.to_owned(),
                crate::persistent_float32_local_clock().to_string(),
                identity.uid().to_owned(),
                identity.session_id().to_owned(),
                identity.hostname().to_owned(),
                advertised_ipv4.to_string(),
                port.clone(),
                port,
                String::new(),
                "0".to_owned(),
                "0".to_owned(),
            ),
        )
        .map_err(PersistentFloat32StreamInfoError::VolatileFields)?;
        let volatile_xml = StreamInfoVolatileXml::compose(&volatile_fields, limits.volatile_xml)
            .map_err(PersistentFloat32StreamInfoError::VolatileXml)?;
        let ordered =
            StreamInfoOrderedXml::compose(static_description, volatile_xml, limits.ordered_xml)
                .map_err(PersistentFloat32StreamInfoError::OrderedXml)?;
        let document = StreamInfoObservedDocument::project(limits.document, &ordered)
            .map_err(PersistentFloat32StreamInfoError::Document)?;
        Ok(Self {
            body: document.into_string(),
            advertised_ipv4,
            local_address: outlet.local_address(),
            channel_count: outlet.channel_count(),
            uid: identity.uid().to_owned(),
            hostname: identity.hostname().to_owned(),
            source_id: identity.source_id().to_owned(),
            session_id: identity.session_id().to_owned(),
        })
    }

    /// Canonical discovery body.
    #[must_use]
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Moves the canonical discovery body without copying it.
    #[must_use]
    pub fn into_body(self) -> String {
        self.body
    }

    pub(crate) fn validate_outlet(
        &self,
        advertised_ipv4: Ipv4Addr,
        outlet: &PersistentFloat32Outlet,
    ) -> Result<(), PersistentFloat32OutletServiceCreateError> {
        if self.advertised_ipv4 != advertised_ipv4 {
            return Err(PersistentFloat32OutletServiceCreateError::AdvertisedIpv4AddressMismatch);
        }
        if self.channel_count != outlet.channel_count() {
            return Err(
                PersistentFloat32OutletServiceCreateError::ChannelCountMismatch {
                    advertised: self.channel_count,
                    outlet: outlet.channel_count(),
                },
            );
        }
        let identity = outlet.stream_identity();
        for (role, expected, actual) in [
            (
                PersistentFloat32OutletServiceIdentityRole::Uid,
                self.uid.as_str(),
                identity.uid(),
            ),
            (
                PersistentFloat32OutletServiceIdentityRole::Hostname,
                self.hostname.as_str(),
                identity.hostname(),
            ),
            (
                PersistentFloat32OutletServiceIdentityRole::SourceId,
                self.source_id.as_str(),
                identity.source_id(),
            ),
            (
                PersistentFloat32OutletServiceIdentityRole::SessionId,
                self.session_id.as_str(),
                identity.session_id(),
            ),
        ] {
            if expected != actual {
                return Err(PersistentFloat32OutletServiceCreateError::IdentityMismatch(
                    role,
                ));
            }
        }
        let local = outlet.local_address();
        let IpAddr::V4(local_ipv4) = local.ip() else {
            return Err(PersistentFloat32OutletServiceCreateError::NonIpv4Outlet);
        };
        if !local_ipv4.is_unspecified() && local_ipv4 != advertised_ipv4 {
            return Err(PersistentFloat32OutletServiceCreateError::ListenerAddressMismatch);
        }
        if local.port() != self.local_address.port() {
            return Err(PersistentFloat32OutletServiceCreateError::AdvertisedDataPortMismatch);
        }
        Ok(())
    }
}

/// Deterministic structured stream-info composition failure.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PersistentFloat32StreamInfoError {
    /// The selected IPv4 address was unspecified, multicast, or broadcast.
    NonConcreteIpv4Interface,
    /// Descriptor validation failed.
    Descriptor(StreamDescriptorError),
    /// Static XML composition failed.
    StaticXml(StreamInfoStaticXmlError),
    /// Metadata projection failed.
    MetadataXml(MetadataXmlProjectionError),
    /// Static/description composition failed.
    DescriptionXml(StreamInfoDescriptionXmlError),
    /// Volatile field validation failed.
    VolatileFields(StreamInfoVolatileFieldError),
    /// Volatile XML composition failed.
    VolatileXml(StreamInfoVolatileXmlError),
    /// Ordered tree composition failed.
    OrderedXml(StreamInfoOrderedXmlError),
    /// Canonical document projection failed.
    Document(StreamInfoObservedDocumentError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_activation::test_capability;
    use crate::{
        MetadataNodeInput, MetadataTreeLimits, PersistentFloat32OutletActivation,
        PersistentFloat32OutletLimits, PersistentFloat32OutletRegistry,
        PersistentFloat32OutletRegistryLimits, PersistentFloat32OutletServiceLimits,
        ShortInfoQueryWireLimits, ShortInfoResponderActivation, ShortInfoResponseEnvelopeLimits,
        StreamHandshakeActivation, StreamHandshakeIdentity, StreamHandshakeLimits,
        StreamInfoObservedAdmissionLimits, StreamInfoObservedDocumentParseLimit,
        StreamInfoVolatileFieldLimits, TimestampedFloat32SampleActivation,
        TimestampedFloat32SampleLimits, XmlCharacterDataLimit, XmlNameLimit, XmlTextLimit,
    };
    use std::net::{TcpListener, UdpSocket};
    use std::time::Duration;

    fn handshake_limits() -> StreamHandshakeLimits {
        StreamHandshakeLimits::new(1024, 128, Duration::from_millis(5), Duration::from_secs(1))
            .unwrap()
    }

    fn outlet() -> PersistentFloat32Outlet {
        let activation = PersistentFloat32OutletActivation::new(
            test_capability(crate::RuntimeModule::PersistentFloat32Outlet),
            TimestampedFloat32SampleActivation::new(
                test_capability(crate::RuntimeModule::TimestampedFloat32Sample),
                StreamHandshakeActivation::new(test_capability(
                    crate::RuntimeModule::StreamHandshake,
                ))
                .unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        PersistentFloat32Outlet::new(
            activation,
            TcpListener::bind("127.0.0.1:0").unwrap(),
            StreamHandshakeIdentity::new(
                "72000000-0000-4000-8000-000000000001".into(),
                "polar-host".into(),
                "polar-source".into(),
                "polar-session".into(),
                handshake_limits(),
            )
            .unwrap(),
            handshake_limits(),
            TimestampedFloat32SampleLimits::new(Duration::from_millis(5), Duration::from_secs(1))
                .unwrap(),
            3,
            PersistentFloat32OutletLimits::new(256, 2).unwrap(),
        )
        .unwrap()
    }

    fn limits() -> PersistentFloat32StreamInfoLimits {
        let name = XmlNameLimit::new(32).unwrap();
        let text = XmlTextLimit::new(128).unwrap();
        let character_data = XmlCharacterDataLimit::new(512).unwrap();
        PersistentFloat32StreamInfoLimits::new(
            StreamDescriptorLimits::new(128, 128, 128, 8).unwrap(),
            StreamInfoStaticXmlLimits::new(
                name,
                text,
                character_data,
                XmlElementTreeLimits::new(7, 2, 6, 4096).unwrap(),
            ),
            MetadataXmlProjectionLimits::new(
                name,
                text,
                character_data,
                XmlElementTreeLimits::new(17, 4, 4, 4096).unwrap(),
            ),
            XmlElementTreeLimits::new(24, 5, 7, 8192).unwrap(),
            StreamInfoVolatileFieldLimits::new(64, 128, 128).unwrap(),
            StreamInfoVolatileXmlLimits::new(
                name,
                text,
                character_data,
                XmlElementTreeLimits::new(12, 2, 11, 4096).unwrap(),
            ),
            XmlElementTreeLimits::new(35, 5, 18, 16384).unwrap(),
            StreamInfoObservedDocumentLimit::new(16384).unwrap(),
        )
    }

    #[test]
    fn polar_001_stream_info_composes_canonical_acc_metadata_without_handwritten_xml() {
        let outlet = outlet();
        let description = MetadataTree::new(
            MetadataTreeLimits::new(17, 4, 4, 32, 64).unwrap(),
            vec![
                MetadataNodeInput::new(None, "desc".into(), None),
                MetadataNodeInput::new(Some(0), "manufacturer".into(), Some("Polar".into())),
                MetadataNodeInput::new(Some(0), "model".into(), Some("H10".into())),
                MetadataNodeInput::new(Some(0), "application".into(), Some("Polar Stream".into())),
                MetadataNodeInput::new(Some(0), "channels".into(), None),
                MetadataNodeInput::new(Some(4), "channel".into(), None),
                MetadataNodeInput::new(Some(5), "label".into(), Some("X".into())),
                MetadataNodeInput::new(Some(5), "unit".into(), Some("mg".into())),
                MetadataNodeInput::new(Some(5), "type".into(), Some("ACC".into())),
                MetadataNodeInput::new(Some(4), "channel".into(), None),
                MetadataNodeInput::new(Some(9), "label".into(), Some("Y".into())),
                MetadataNodeInput::new(Some(9), "unit".into(), Some("mg".into())),
                MetadataNodeInput::new(Some(9), "type".into(), Some("ACC".into())),
                MetadataNodeInput::new(Some(4), "channel".into(), None),
                MetadataNodeInput::new(Some(13), "label".into(), Some("Z".into())),
                MetadataNodeInput::new(Some(13), "unit".into(), Some("mg".into())),
                MetadataNodeInput::new(Some(13), "type".into(), Some("ACC".into())),
            ],
        )
        .unwrap();
        let stream_info = PersistentFloat32StreamInfo::compose(
            &outlet,
            Ipv4Addr::LOCALHOST,
            PersistentFloat32StreamInfoInput::new(
                "Polar H10 ACC".into(),
                "ACC".into(),
                NominalSampleRate::regular_hz(200.0).unwrap(),
                description,
            ),
            limits(),
        )
        .unwrap();
        assert!(stream_info
            .body()
            .starts_with("<?xml version=\"1.0\"?>\n<info>\n"));
        assert!(stream_info
            .body()
            .contains("\t<channel_count>3</channel_count>\n"));
        assert!(stream_info
            .body()
            .contains("\t<nominal_srate>200.0000000000000</nominal_srate>\n"));
        assert!(stream_info
            .body()
            .contains("\t<source_id>polar-source</source_id>\n"));
        assert!(stream_info.body().contains(&format!(
            "\t<v4service_port>{}</v4service_port>\n",
            outlet.local_address().port()
        )));
        assert!(stream_info.body().contains("\t\t\t<label>X</label>\n"));
        assert!(stream_info.body().contains("\t\t\t<label>Y</label>\n"));
        assert!(stream_info.body().contains("\t\t\t<label>Z</label>\n"));
        assert_eq!(
            stream_info
                .body()
                .matches("\t\t\t<unit>mg</unit>\n")
                .count(),
            3
        );
        assert_eq!(
            stream_info
                .body()
                .matches("\t\t\t<type>ACC</type>\n")
                .count(),
            3
        );
        assert!(stream_info.body().ends_with("\t</desc>\n</info>\n"));

        let body_len = stream_info.body().len();
        let service_limits = PersistentFloat32OutletServiceLimits::new(
            16384,
            StreamInfoObservedDocumentParseLimit::new(body_len).unwrap(),
            StreamInfoObservedAdmissionLimits::new(
                StreamDescriptorLimits::new(128, 128, 128, 8).unwrap(),
                MetadataTreeLimits::new(17, 4, 4, 32, 64).unwrap(),
                StreamInfoVolatileFieldLimits::new(64, 128, 128).unwrap(),
            ),
            ShortInfoQueryWireLimits::new(256, 512).unwrap(),
            ShortInfoResponseEnvelopeLimits::new(body_len, body_len + 64).unwrap(),
        )
        .unwrap();
        let mut registry = PersistentFloat32OutletRegistry::new_prebound(
            ShortInfoResponderActivation::new(test_capability(
                crate::RuntimeModule::ShortInfoDiscoveryResponder,
            ))
            .unwrap(),
            Ipv4Addr::LOCALHOST,
            UdpSocket::bind("127.0.0.1:0").unwrap(),
            PersistentFloat32OutletRegistryLimits::new(1, service_limits).unwrap(),
        )
        .unwrap();
        let id = registry.register_stream_info(outlet, stream_info).unwrap();
        assert_eq!(id.index(), 0);
        assert_eq!(registry.outlet_count(), 1);
    }
}
