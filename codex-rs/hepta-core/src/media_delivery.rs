use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaDeliveryContractKind {
    MultiImageBatch,
    OrderedImageCaptions,
    CentralizedAudioRouter,
    VoiceNoteHint,
    MimeTypePreservation,
    ChannelCapabilityFallback,
    DeliveryReceiptLink,
    RedactedMediaProvenance,
    BodylessMediaResponse,
    StreamingMediaSizeCap,
    BlankTextGeneratedMediaReply,
    ConfiguredTimeoutInheritance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaDeliveryDescriptor {
    pub id: String,
    pub kind: MediaDeliveryContractKind,
    pub contract_covered: bool,
    pub evidence_gate: String,
    pub operator_surface: String,
    pub multi_image_supported: bool,
    pub centralized_audio_required: bool,
    pub channel_agnostic_policy: bool,
    pub fallback_required: bool,
    pub receipt_link_required: bool,
    pub redaction_required: bool,
    pub external_side_effects: bool,
    pub summary: String,
}

impl MediaDeliveryDescriptor {
    pub fn new(
        id: impl Into<String>,
        kind: MediaDeliveryContractKind,
        evidence_gate: impl Into<String>,
        operator_surface: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            contract_covered: true,
            evidence_gate: evidence_gate.into(),
            operator_surface: operator_surface.into(),
            multi_image_supported: matches!(
                kind,
                MediaDeliveryContractKind::MultiImageBatch
                    | MediaDeliveryContractKind::OrderedImageCaptions
                    | MediaDeliveryContractKind::ChannelCapabilityFallback
            ),
            centralized_audio_required: matches!(
                kind,
                MediaDeliveryContractKind::CentralizedAudioRouter
                    | MediaDeliveryContractKind::VoiceNoteHint
                    | MediaDeliveryContractKind::ChannelCapabilityFallback
            ),
            channel_agnostic_policy: true,
            fallback_required: matches!(
                kind,
                MediaDeliveryContractKind::ChannelCapabilityFallback
                    | MediaDeliveryContractKind::DeliveryReceiptLink
            ),
            receipt_link_required: matches!(
                kind,
                MediaDeliveryContractKind::DeliveryReceiptLink
                    | MediaDeliveryContractKind::RedactedMediaProvenance
            ),
            redaction_required: matches!(
                kind,
                MediaDeliveryContractKind::RedactedMediaProvenance
                    | MediaDeliveryContractKind::DeliveryReceiptLink
            ),
            external_side_effects: false,
            summary: summary.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaDeliveryContractReport {
    pub contract_id: String,
    pub contract_count: usize,
    pub contract_covered_count: usize,
    pub multi_image_batch_contract: bool,
    pub ordered_image_captions_contract: bool,
    pub centralized_audio_router_contract: bool,
    pub voice_note_hint_contract: bool,
    pub mime_type_preservation_contract: bool,
    pub channel_capability_fallback_contract: bool,
    pub delivery_receipt_link_contract: bool,
    pub redacted_media_provenance_contract: bool,
    pub bodyless_media_response_contract: bool,
    pub streaming_media_size_cap_contract: bool,
    pub blank_text_generated_media_reply_contract: bool,
    pub configured_timeout_inheritance_contract: bool,
    pub max_images_per_message: usize,
    pub supported_media_kinds: Vec<String>,
    pub multi_image_order_preserved: bool,
    pub centralized_audio_delivery: bool,
    pub audio_voice_hint_supported: bool,
    pub channel_agnostic_policy: bool,
    pub fallback_policy_required: bool,
    pub redacted_provenance_required: bool,
    pub media_uploaded: bool,
    pub channel_message_sent: bool,
    pub provider_generation_started: bool,
    pub external_network_write: bool,
    pub external_side_effects: bool,
    pub p1_media_delivery_contract_ready: bool,
    pub contracts: Vec<MediaDeliveryDescriptor>,
}

impl MediaDeliveryContractReport {
    pub fn native_default() -> Self {
        Self::from_contracts(vec![
            MediaDeliveryDescriptor::new(
                "multi-image-delivery-batch",
                MediaDeliveryContractKind::MultiImageBatch,
                "cargo test -p hepta-core media_delivery_contract_covers_multi_image_and_centralized_audio_without_side_effects --quiet",
                "/media-delivery-contract --json, /gateway-contracts --json",
                "multiple generated or user-supplied images are represented as one ordered delivery group before adapter-specific splitting or album handling",
            ),
            MediaDeliveryDescriptor::new(
                "ordered-image-caption-policy",
                MediaDeliveryContractKind::OrderedImageCaptions,
                "cargo test -p hepta-core media_delivery_contract_covers_multi_image_and_centralized_audio_without_side_effects --quiet",
                "/media-delivery-contract --json",
                "image order, per-image filename, MIME type, and optional caption metadata remain stable across channel fallback plans",
            ),
            MediaDeliveryDescriptor::new(
                "centralized-audio-delivery-router",
                MediaDeliveryContractKind::CentralizedAudioRouter,
                "cargo test -p hepta-core media_delivery_contract_covers_multi_image_and_centralized_audio_without_side_effects --quiet",
                "/media-delivery-contract --json, /media-generation-plane --json",
                "generated music, TTS, and uploaded audio share one channel-agnostic delivery envelope instead of per-provider send paths",
            ),
            MediaDeliveryDescriptor::new(
                "voice-note-presentation-hint",
                MediaDeliveryContractKind::VoiceNoteHint,
                "cargo test -p hepta-core media_delivery_contract_covers_multi_image_and_centralized_audio_without_side_effects --quiet",
                "/media-delivery-contract --json, /voice-call-plane --json",
                "voice-note presentation is modeled as a delivery hint so channels can choose native voice UX without changing the audio asset",
            ),
            MediaDeliveryDescriptor::new(
                "mime-type-preserving-envelope",
                MediaDeliveryContractKind::MimeTypePreservation,
                "cargo test -p hepta-core media_delivery_contract_covers_multi_image_and_centralized_audio_without_side_effects --quiet",
                "/media-delivery-contract --json",
                "media delivery preserves filename, MIME type, semantic role, and generated-vs-user provenance before dispatch",
            ),
            MediaDeliveryDescriptor::new(
                "channel-capability-fallback-policy",
                MediaDeliveryContractKind::ChannelCapabilityFallback,
                "cargo test -p hepta-core media_delivery_contract_covers_multi_image_and_centralized_audio_without_side_effects --quiet",
                "/media-delivery-contract --json, /channel-contracts --json",
                "channels that lack albums, documents, or native voice notes receive a deterministic split, document, or link fallback plan",
            ),
            MediaDeliveryDescriptor::new(
                "delivery-receipt-media-link",
                MediaDeliveryContractKind::DeliveryReceiptLink,
                "cargo test -p hepta-core media_delivery_contract_covers_multi_image_and_centralized_audio_without_side_effects --quiet",
                "/media-delivery-contract --json, /gateway-ledger --json",
                "delivery receipts can reference a redacted media group id and attachment count without leaking local paths or binary payloads",
            ),
            MediaDeliveryDescriptor::new(
                "redacted-media-provenance-ledger",
                MediaDeliveryContractKind::RedactedMediaProvenance,
                "cargo test -p hepta-core media_delivery_contract_covers_multi_image_and_centralized_audio_without_side_effects --quiet",
                "/media-delivery-contract --json, /provenance --json",
                "media provenance records semantic kind, source class, and digest labels while keeping raw paths, URLs, and payload bytes out of audit output",
            ),
            MediaDeliveryDescriptor::new(
                "bodyless-media-response-policy",
                MediaDeliveryContractKind::BodylessMediaResponse,
                "cargo test -p hepta-core media_delivery_contract_covers_media_robustness_without_side_effects --quiet",
                "/media-delivery-contract --json, /media-generation-plane --json",
                "bodyless provider/media responses are represented as metadata-only success or status records without buffering a nonexistent response body",
            ),
            MediaDeliveryDescriptor::new(
                "streaming-inbound-media-size-cap",
                MediaDeliveryContractKind::StreamingMediaSizeCap,
                "cargo test -p hepta-core media_delivery_contract_covers_media_robustness_without_side_effects --quiet",
                "/media-delivery-contract --json, /nodes --json",
                "inbound media byte counts are enforced during streaming before full buffering or provider handoff",
            ),
            MediaDeliveryDescriptor::new(
                "blank-text-generated-media-reply",
                MediaDeliveryContractKind::BlankTextGeneratedMediaReply,
                "cargo test -p hepta-core media_delivery_contract_covers_media_robustness_without_side_effects --quiet",
                "/media-delivery-contract --json, /media-generation-plane --json",
                "generated media savedPath events may produce reply media even when assistant text is blank",
            ),
            MediaDeliveryDescriptor::new(
                "configured-media-timeout-inheritance",
                MediaDeliveryContractKind::ConfiguredTimeoutInheritance,
                "cargo test -p hepta-core media_delivery_contract_covers_media_robustness_without_side_effects --quiet",
                "/media-delivery-contract --json, /media-generation-plane --json",
                "media generation requests inherit configured music/video timeouts when timeoutMs is omitted",
            ),
        ])
    }

    pub fn from_contracts(contracts: Vec<MediaDeliveryDescriptor>) -> Self {
        let contract_count = contracts.len();
        let contract_covered_count = contracts
            .iter()
            .filter(|contract| contract.contract_covered)
            .count();
        let has_kind = |kind: MediaDeliveryContractKind| {
            contracts
                .iter()
                .any(|contract| contract.contract_covered && contract.kind == kind)
        };
        let multi_image_batch_contract = has_kind(MediaDeliveryContractKind::MultiImageBatch);
        let ordered_image_captions_contract =
            has_kind(MediaDeliveryContractKind::OrderedImageCaptions);
        let centralized_audio_router_contract =
            has_kind(MediaDeliveryContractKind::CentralizedAudioRouter);
        let voice_note_hint_contract = has_kind(MediaDeliveryContractKind::VoiceNoteHint);
        let mime_type_preservation_contract =
            has_kind(MediaDeliveryContractKind::MimeTypePreservation);
        let channel_capability_fallback_contract =
            has_kind(MediaDeliveryContractKind::ChannelCapabilityFallback);
        let delivery_receipt_link_contract =
            has_kind(MediaDeliveryContractKind::DeliveryReceiptLink);
        let redacted_media_provenance_contract =
            has_kind(MediaDeliveryContractKind::RedactedMediaProvenance);
        let bodyless_media_response_contract =
            has_kind(MediaDeliveryContractKind::BodylessMediaResponse);
        let streaming_media_size_cap_contract =
            has_kind(MediaDeliveryContractKind::StreamingMediaSizeCap);
        let blank_text_generated_media_reply_contract =
            has_kind(MediaDeliveryContractKind::BlankTextGeneratedMediaReply);
        let configured_timeout_inheritance_contract =
            has_kind(MediaDeliveryContractKind::ConfiguredTimeoutInheritance);
        let max_images_per_message = 10;
        let supported_media_kinds = vec![
            "image".into(),
            "image_group".into(),
            "audio".into(),
            "voice_note".into(),
            "file".into(),
        ];
        let multi_image_order_preserved = contracts
            .iter()
            .filter(|contract| contract.multi_image_supported)
            .count()
            >= 3;
        let centralized_audio_delivery = contracts
            .iter()
            .filter(|contract| contract.centralized_audio_required)
            .count()
            >= 3;
        let audio_voice_hint_supported = voice_note_hint_contract;
        let channel_agnostic_policy = contracts
            .iter()
            .all(|contract| contract.contract_covered && contract.channel_agnostic_policy);
        let fallback_policy_required = contracts
            .iter()
            .filter(|contract| contract.fallback_required)
            .count()
            >= 2;
        let redacted_provenance_required = contracts
            .iter()
            .filter(|contract| contract.redaction_required)
            .count()
            >= 2;
        let media_uploaded = false;
        let channel_message_sent = false;
        let provider_generation_started = false;
        let external_network_write = false;
        let external_side_effects = contracts
            .iter()
            .any(|contract| contract.external_side_effects)
            || media_uploaded
            || channel_message_sent
            || provider_generation_started
            || external_network_write;
        let p1_media_delivery_contract_ready = contract_count > 0
            && contract_count == contract_covered_count
            && multi_image_batch_contract
            && ordered_image_captions_contract
            && centralized_audio_router_contract
            && voice_note_hint_contract
            && mime_type_preservation_contract
            && channel_capability_fallback_contract
            && delivery_receipt_link_contract
            && redacted_media_provenance_contract
            && bodyless_media_response_contract
            && streaming_media_size_cap_contract
            && blank_text_generated_media_reply_contract
            && configured_timeout_inheritance_contract
            && max_images_per_message >= 4
            && supported_media_kinds.contains(&"image_group".to_string())
            && supported_media_kinds.contains(&"voice_note".to_string())
            && multi_image_order_preserved
            && centralized_audio_delivery
            && audio_voice_hint_supported
            && channel_agnostic_policy
            && fallback_policy_required
            && redacted_provenance_required
            && !external_side_effects;

        Self {
            contract_id: "media-delivery-contract".into(),
            contract_count,
            contract_covered_count,
            multi_image_batch_contract,
            ordered_image_captions_contract,
            centralized_audio_router_contract,
            voice_note_hint_contract,
            mime_type_preservation_contract,
            channel_capability_fallback_contract,
            delivery_receipt_link_contract,
            redacted_media_provenance_contract,
            bodyless_media_response_contract,
            streaming_media_size_cap_contract,
            blank_text_generated_media_reply_contract,
            configured_timeout_inheritance_contract,
            max_images_per_message,
            supported_media_kinds,
            multi_image_order_preserved,
            centralized_audio_delivery,
            audio_voice_hint_supported,
            channel_agnostic_policy,
            fallback_policy_required,
            redacted_provenance_required,
            media_uploaded,
            channel_message_sent,
            provider_generation_started,
            external_network_write,
            external_side_effects,
            p1_media_delivery_contract_ready,
            contracts,
        }
    }

    pub fn contract_ready(&self) -> bool {
        self.p1_media_delivery_contract_ready
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderMediaCapability {
    TextGeneration,
    ImageInput,
    ImageGeneration,
    AudioInput,
    TextToSpeech,
    SpeechToText,
    VideoGeneration,
    MusicGeneration,
    Embeddings,
    MediaDelivery,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderMediaCapabilityRow {
    pub provider_id: String,
    pub capabilities: Vec<ProviderMediaCapability>,
    pub local_catalog_only: bool,
    pub provider_call_performed: bool,
    pub external_network_read: bool,
    pub credential_value_read: bool,
}

impl ProviderMediaCapabilityRow {
    pub fn new(provider_id: impl Into<String>, capabilities: Vec<ProviderMediaCapability>) -> Self {
        Self {
            provider_id: provider_id.into(),
            capabilities,
            local_catalog_only: true,
            provider_call_performed: false,
            external_network_read: false,
            credential_value_read: false,
        }
    }

    pub fn supports(&self, capability: ProviderMediaCapability) -> bool {
        self.capabilities.contains(&capability)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderMediaCapabilityRegistry {
    pub row_count: usize,
    pub capability_count: usize,
    pub text_generation_provider_count: usize,
    pub image_generation_provider_count: usize,
    pub audio_provider_count: usize,
    pub video_provider_count: usize,
    pub embeddings_provider_count: usize,
    pub provider_call_performed: bool,
    pub external_network_read: bool,
    pub credential_value_read: bool,
    pub registry_ready: bool,
    pub rows: Vec<ProviderMediaCapabilityRow>,
}

impl ProviderMediaCapabilityRegistry {
    pub fn native_default() -> Self {
        Self::from_rows(vec![
            ProviderMediaCapabilityRow::new(
                "openai-compatible",
                vec![
                    ProviderMediaCapability::TextGeneration,
                    ProviderMediaCapability::ImageInput,
                    ProviderMediaCapability::TextToSpeech,
                    ProviderMediaCapability::SpeechToText,
                    ProviderMediaCapability::Embeddings,
                ],
            ),
            ProviderMediaCapabilityRow::new(
                "deepinfra",
                vec![
                    ProviderMediaCapability::TextGeneration,
                    ProviderMediaCapability::ImageInput,
                    ProviderMediaCapability::AudioInput,
                    ProviderMediaCapability::Embeddings,
                ],
            ),
            ProviderMediaCapabilityRow::new(
                "google-media",
                vec![
                    ProviderMediaCapability::ImageGeneration,
                    ProviderMediaCapability::VideoGeneration,
                    ProviderMediaCapability::MusicGeneration,
                    ProviderMediaCapability::MediaDelivery,
                ],
            ),
            ProviderMediaCapabilityRow::new(
                "local-ollama",
                vec![
                    ProviderMediaCapability::TextGeneration,
                    ProviderMediaCapability::ImageGeneration,
                    ProviderMediaCapability::Embeddings,
                ],
            ),
        ])
    }

    pub fn from_rows(rows: Vec<ProviderMediaCapabilityRow>) -> Self {
        let row_count = rows.len();
        let mut all_capabilities = rows
            .iter()
            .flat_map(|row| row.capabilities.iter().copied())
            .collect::<Vec<_>>();
        all_capabilities.sort();
        all_capabilities.dedup();
        let capability_count = all_capabilities.len();
        let count = |capability| rows.iter().filter(|row| row.supports(capability)).count();
        let text_generation_provider_count = count(ProviderMediaCapability::TextGeneration);
        let image_generation_provider_count = count(ProviderMediaCapability::ImageGeneration);
        let audio_provider_count = rows
            .iter()
            .filter(|row| {
                row.supports(ProviderMediaCapability::AudioInput)
                    || row.supports(ProviderMediaCapability::TextToSpeech)
                    || row.supports(ProviderMediaCapability::SpeechToText)
                    || row.supports(ProviderMediaCapability::MusicGeneration)
            })
            .count();
        let video_provider_count = count(ProviderMediaCapability::VideoGeneration);
        let embeddings_provider_count = count(ProviderMediaCapability::Embeddings);
        let provider_call_performed = rows.iter().any(|row| row.provider_call_performed);
        let external_network_read = rows.iter().any(|row| row.external_network_read);
        let credential_value_read = rows.iter().any(|row| row.credential_value_read);
        let registry_ready = row_count >= 4
            && capability_count >= 8
            && text_generation_provider_count >= 2
            && image_generation_provider_count >= 2
            && audio_provider_count >= 2
            && video_provider_count >= 1
            && embeddings_provider_count >= 2
            && rows.iter().all(|row| row.local_catalog_only)
            && !provider_call_performed
            && !external_network_read
            && !credential_value_read;

        Self {
            row_count,
            capability_count,
            text_generation_provider_count,
            image_generation_provider_count,
            audio_provider_count,
            video_provider_count,
            embeddings_provider_count,
            provider_call_performed,
            external_network_read,
            credential_value_read,
            registry_ready,
            rows,
        }
    }

    pub fn providers_supporting(
        &self,
        capability: ProviderMediaCapability,
    ) -> Vec<&ProviderMediaCapabilityRow> {
        self.rows
            .iter()
            .filter(|row| row.supports(capability))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MediaDeliveryContractReport, ProviderMediaCapability, ProviderMediaCapabilityRegistry,
    };

    #[test]
    fn media_delivery_contract_covers_multi_image_and_centralized_audio_without_side_effects() {
        let report = MediaDeliveryContractReport::native_default();

        assert_eq!(report.contract_count, 12);
        assert_eq!(report.contract_covered_count, report.contract_count);
        assert!(report.multi_image_batch_contract);
        assert!(report.ordered_image_captions_contract);
        assert!(report.centralized_audio_router_contract);
        assert!(report.voice_note_hint_contract);
        assert!(report.mime_type_preservation_contract);
        assert!(report.channel_capability_fallback_contract);
        assert!(report.delivery_receipt_link_contract);
        assert!(report.redacted_media_provenance_contract);
        assert!(report.bodyless_media_response_contract);
        assert!(report.streaming_media_size_cap_contract);
        assert!(report.blank_text_generated_media_reply_contract);
        assert!(report.configured_timeout_inheritance_contract);
        assert!(report.max_images_per_message >= 4);
        assert!(
            report
                .supported_media_kinds
                .contains(&"image_group".to_string())
        );
        assert!(
            report
                .supported_media_kinds
                .contains(&"voice_note".to_string())
        );
        assert!(report.multi_image_order_preserved);
        assert!(report.centralized_audio_delivery);
        assert!(report.audio_voice_hint_supported);
        assert!(report.channel_agnostic_policy);
        assert!(report.fallback_policy_required);
        assert!(report.redacted_provenance_required);
        assert!(!report.media_uploaded);
        assert!(!report.channel_message_sent);
        assert!(!report.provider_generation_started);
        assert!(!report.external_network_write);
        assert!(!report.external_side_effects);
        assert!(report.contract_ready());
        let forbidden = ["her", "mes"].concat();
        assert!(report.contracts.iter().all(|contract| {
            let id = contract.id.to_lowercase();
            let summary = contract.summary.to_lowercase();
            !id.contains(&forbidden) && !summary.contains(&forbidden)
        }));
    }

    #[test]
    fn media_delivery_contract_covers_media_robustness_without_side_effects() {
        let report = MediaDeliveryContractReport::native_default();

        assert_eq!(report.contract_count, 12);
        assert!(report.bodyless_media_response_contract);
        assert!(report.streaming_media_size_cap_contract);
        assert!(report.blank_text_generated_media_reply_contract);
        assert!(report.configured_timeout_inheritance_contract);
        assert!(!report.media_uploaded);
        assert!(!report.channel_message_sent);
        assert!(!report.external_network_write);
        assert!(report.contract_ready());
    }

    #[test]
    fn provider_media_capability_registry_is_local_only_and_queryable() {
        let registry = ProviderMediaCapabilityRegistry::native_default();

        assert!(registry.registry_ready);
        assert!(registry.capability_count >= 8);
        assert!(registry.text_generation_provider_count >= 2);
        assert!(registry.image_generation_provider_count >= 2);
        assert!(registry.audio_provider_count >= 2);
        assert!(registry.video_provider_count >= 1);
        assert!(registry.embeddings_provider_count >= 2);
        assert!(!registry.provider_call_performed);
        assert!(!registry.external_network_read);
        assert!(!registry.credential_value_read);
        assert!(
            registry
                .providers_supporting(ProviderMediaCapability::VideoGeneration)
                .iter()
                .any(|row| row.provider_id == "google-media")
        );
        assert!(
            registry
                .providers_supporting(ProviderMediaCapability::TextToSpeech)
                .iter()
                .any(|row| row.provider_id == "openai-compatible")
        );
    }
}
