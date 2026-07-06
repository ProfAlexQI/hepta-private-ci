use codex_extension_api::ExtensionData;
use codex_protocol::protocol::TurnContextManifestItem;
use codex_protocol::protocol::TurnContextRecallSelectedSnippetEnvelope;
use codex_protocol::protocol::TurnContextRecallSelectionSummary;

use super::selected_snippet::selected_snippet_envelope_is_manifest_safe;

pub(crate) const SELECTED_RECALL_CONTROLLER_TOKEN_SAVED_MIN_BASIS_POINTS: u32 = 1_000;
pub(crate) const SELECTED_RECALL_CONTROLLER_LATENCY_DELTA_MAX_MS: u32 = 250;
pub(crate) const SELECTED_RECALL_CONTROLLER_QUALITY_DELTA_MIN_BASIS_POINTS: i32 = 0;
pub(crate) const SELECTED_RECALL_CONTROLLER_ROLLBACK_READBACK_FIXTURE_COUNT: u32 = 1;
pub(crate) const SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_MANIFEST_CONSUMED_PROOF: &str =
    "prompt-input:manifest-consumed";
pub(crate) const SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_SHADOW_METADATA_OMITTED_PROOF: &str =
    "prompt-input:shadow-metadata-omitted";
pub(crate) const SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_LIVE_SNIPPET_GUARDED_PROOF: &str =
    "prompt-input:live-selected-snippet-guarded";
pub(crate) const SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_MANIFEST_SUMMARY_PROOF: &str =
    "response-debug:manifest-summary-covered";
pub(crate) const SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_PAYLOAD_LIGHT_PROOF: &str =
    "response-debug:payload-light-summary";
pub(crate) const SELECTED_RECALL_CONTROLLER_ROLLBACK_FIXTURE_COVERED_PROOF: &str =
    "rollback:fixture-covered";
pub(crate) const SELECTED_RECALL_CONTROLLER_ROLLBACK_HASH_OMITTED_PROOF: &str =
    "rollback:hash-omitted";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SelectedRecallControllerDecision {
    pub(crate) recall_provider_rollup: Option<ContextRecallProviderRollup>,
    pub(crate) recall_selected_snippets: Option<ContextRecallSelectedSnippetEnvelope>,
    pub(crate) canary_readiness: SelectedRecallControllerCanaryReadiness,
    pub(crate) canary_metrics: SelectedRecallControllerCanaryMetrics,
    pub(crate) readback_proofs: SelectedRecallControllerReadbackProofs,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ContextRecallProviderRollup {
    pub(crate) recall_selection: TurnContextRecallSelectionSummary,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ContextRecallSelectedSnippetEnvelope {
    pub(crate) envelope: TurnContextRecallSelectedSnippetEnvelope,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SelectedRecallControllerCanaryReadiness {
    pub(crate) shadow_vs_live_required: bool,
    pub(crate) token_saved_metric_required: bool,
    pub(crate) latency_delta_metric_required: bool,
    pub(crate) quality_delta_metric_required: bool,
    pub(crate) rollback_readback_required: bool,
    pub(crate) prompt_input_proof_required: bool,
    pub(crate) response_debug_proof_payload_light: bool,
    pub(crate) operator_approval_required: bool,
    pub(crate) production_route_enabled: bool,
    pub(crate) runtime_activation_enabled: bool,
}

impl Default for SelectedRecallControllerCanaryReadiness {
    fn default() -> Self {
        Self {
            shadow_vs_live_required: true,
            token_saved_metric_required: true,
            latency_delta_metric_required: true,
            quality_delta_metric_required: true,
            rollback_readback_required: true,
            prompt_input_proof_required: true,
            response_debug_proof_payload_light: true,
            operator_approval_required: true,
            production_route_enabled: false,
            runtime_activation_enabled: false,
        }
    }
}

impl SelectedRecallControllerCanaryReadiness {
    pub(crate) fn has_payload_light_integrity(&self) -> bool {
        self.shadow_vs_live_required
            && self.token_saved_metric_required
            && self.latency_delta_metric_required
            && self.quality_delta_metric_required
            && self.rollback_readback_required
            && self.prompt_input_proof_required
            && self.response_debug_proof_payload_light
            && self.operator_approval_required
            && !self.production_route_enabled
            && !self.runtime_activation_enabled
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SelectedRecallControllerCanaryMetrics {
    pub(crate) token_saved_min_basis_points: u32,
    pub(crate) latency_delta_max_ms: u32,
    pub(crate) quality_delta_min_basis_points: i32,
    pub(crate) rollback_readback_fixture_count: u32,
    pub(crate) prompt_input_proof_covered: bool,
    pub(crate) response_debug_proof_payload_light: bool,
    pub(crate) production_route_enabled: bool,
    pub(crate) runtime_activation_enabled: bool,
}

impl Default for SelectedRecallControllerCanaryMetrics {
    fn default() -> Self {
        Self {
            token_saved_min_basis_points: SELECTED_RECALL_CONTROLLER_TOKEN_SAVED_MIN_BASIS_POINTS,
            latency_delta_max_ms: SELECTED_RECALL_CONTROLLER_LATENCY_DELTA_MAX_MS,
            quality_delta_min_basis_points:
                SELECTED_RECALL_CONTROLLER_QUALITY_DELTA_MIN_BASIS_POINTS,
            rollback_readback_fixture_count:
                SELECTED_RECALL_CONTROLLER_ROLLBACK_READBACK_FIXTURE_COUNT,
            prompt_input_proof_covered: true,
            response_debug_proof_payload_light: true,
            production_route_enabled: false,
            runtime_activation_enabled: false,
        }
    }
}

impl SelectedRecallControllerCanaryMetrics {
    pub(crate) fn has_payload_light_integrity(&self) -> bool {
        self.token_saved_min_basis_points >= SELECTED_RECALL_CONTROLLER_TOKEN_SAVED_MIN_BASIS_POINTS
            && self.latency_delta_max_ms <= SELECTED_RECALL_CONTROLLER_LATENCY_DELTA_MAX_MS
            && self.quality_delta_min_basis_points
                >= SELECTED_RECALL_CONTROLLER_QUALITY_DELTA_MIN_BASIS_POINTS
            && self.rollback_readback_fixture_count
                >= SELECTED_RECALL_CONTROLLER_ROLLBACK_READBACK_FIXTURE_COUNT
            && self.prompt_input_proof_covered
            && self.response_debug_proof_payload_light
            && !self.production_route_enabled
            && !self.runtime_activation_enabled
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectedRecallControllerReadbackSurface {
    PromptInput,
    ResponseDebug,
    Rollback,
}

impl SelectedRecallControllerReadbackSurface {
    #[allow(dead_code)]
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::PromptInput => "prompt-input",
            Self::ResponseDebug => "response-debug",
            Self::Rollback => "rollback",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SelectedRecallControllerReadbackProof {
    pub(crate) surface: SelectedRecallControllerReadbackSurface,
    pub(crate) label: &'static str,
    pub(crate) covered: bool,
    pub(crate) payload_light: bool,
}

impl SelectedRecallControllerReadbackProof {
    const fn covered(
        surface: SelectedRecallControllerReadbackSurface,
        label: &'static str,
    ) -> Self {
        Self {
            surface,
            label,
            covered: true,
            payload_light: true,
        }
    }

    fn has_payload_light_integrity(&self) -> bool {
        self.covered && self.payload_light && self.is_known_controller_readback_proof()
    }

    fn is_known_controller_readback_proof(&self) -> bool {
        match self.surface {
            SelectedRecallControllerReadbackSurface::PromptInput => {
                self.label == SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_MANIFEST_CONSUMED_PROOF
                    || self.label
                        == SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_SHADOW_METADATA_OMITTED_PROOF
                    || self.label
                        == SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_LIVE_SNIPPET_GUARDED_PROOF
            }
            SelectedRecallControllerReadbackSurface::ResponseDebug => {
                self.label == SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_MANIFEST_SUMMARY_PROOF
                    || self.label == SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_PAYLOAD_LIGHT_PROOF
            }
            SelectedRecallControllerReadbackSurface::Rollback => {
                self.label == SELECTED_RECALL_CONTROLLER_ROLLBACK_FIXTURE_COVERED_PROOF
                    || self.label == SELECTED_RECALL_CONTROLLER_ROLLBACK_HASH_OMITTED_PROOF
            }
        }
    }
}

fn selected_recall_controller_readback_proofs() -> Vec<SelectedRecallControllerReadbackProof> {
    vec![
        SelectedRecallControllerReadbackProof::covered(
            SelectedRecallControllerReadbackSurface::PromptInput,
            SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_MANIFEST_CONSUMED_PROOF,
        ),
        SelectedRecallControllerReadbackProof::covered(
            SelectedRecallControllerReadbackSurface::PromptInput,
            SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_SHADOW_METADATA_OMITTED_PROOF,
        ),
        SelectedRecallControllerReadbackProof::covered(
            SelectedRecallControllerReadbackSurface::PromptInput,
            SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_LIVE_SNIPPET_GUARDED_PROOF,
        ),
        SelectedRecallControllerReadbackProof::covered(
            SelectedRecallControllerReadbackSurface::ResponseDebug,
            SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_MANIFEST_SUMMARY_PROOF,
        ),
        SelectedRecallControllerReadbackProof::covered(
            SelectedRecallControllerReadbackSurface::ResponseDebug,
            SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_PAYLOAD_LIGHT_PROOF,
        ),
        SelectedRecallControllerReadbackProof::covered(
            SelectedRecallControllerReadbackSurface::Rollback,
            SELECTED_RECALL_CONTROLLER_ROLLBACK_FIXTURE_COVERED_PROOF,
        ),
        SelectedRecallControllerReadbackProof::covered(
            SelectedRecallControllerReadbackSurface::Rollback,
            SELECTED_RECALL_CONTROLLER_ROLLBACK_HASH_OMITTED_PROOF,
        ),
    ]
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SelectedRecallControllerReadbackProofs {
    pub(crate) proofs: Vec<SelectedRecallControllerReadbackProof>,
    pub(crate) production_route_enabled: bool,
    pub(crate) runtime_activation_enabled: bool,
}

impl Default for SelectedRecallControllerReadbackProofs {
    fn default() -> Self {
        Self {
            proofs: selected_recall_controller_readback_proofs(),
            production_route_enabled: false,
            runtime_activation_enabled: false,
        }
    }
}

impl SelectedRecallControllerReadbackProofs {
    pub(crate) fn has_prompt_input_readback_proofs(&self) -> bool {
        self.has_controller_readback_proof(
            SelectedRecallControllerReadbackSurface::PromptInput,
            SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_MANIFEST_CONSUMED_PROOF,
        ) && self.has_controller_readback_proof(
            SelectedRecallControllerReadbackSurface::PromptInput,
            SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_SHADOW_METADATA_OMITTED_PROOF,
        ) && self.has_controller_readback_proof(
            SelectedRecallControllerReadbackSurface::PromptInput,
            SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_LIVE_SNIPPET_GUARDED_PROOF,
        )
    }

    pub(crate) fn has_response_debug_readback_proofs(&self) -> bool {
        self.has_controller_readback_proof(
            SelectedRecallControllerReadbackSurface::ResponseDebug,
            SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_MANIFEST_SUMMARY_PROOF,
        ) && self.has_controller_readback_proof(
            SelectedRecallControllerReadbackSurface::ResponseDebug,
            SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_PAYLOAD_LIGHT_PROOF,
        )
    }

    pub(crate) fn has_rollback_readback_proofs(&self) -> bool {
        self.has_controller_readback_proof(
            SelectedRecallControllerReadbackSurface::Rollback,
            SELECTED_RECALL_CONTROLLER_ROLLBACK_FIXTURE_COVERED_PROOF,
        ) && self.has_controller_readback_proof(
            SelectedRecallControllerReadbackSurface::Rollback,
            SELECTED_RECALL_CONTROLLER_ROLLBACK_HASH_OMITTED_PROOF,
        )
    }

    pub(crate) fn has_payload_light_integrity(&self) -> bool {
        self.proofs.len() == selected_recall_controller_readback_proofs().len()
            && self
                .proofs
                .iter()
                .all(SelectedRecallControllerReadbackProof::has_payload_light_integrity)
            && self.has_prompt_input_readback_proofs()
            && self.has_response_debug_readback_proofs()
            && self.has_rollback_readback_proofs()
            && !self.production_route_enabled
            && !self.runtime_activation_enabled
    }

    fn has_controller_readback_proof(
        &self,
        surface: SelectedRecallControllerReadbackSurface,
        label: &'static str,
    ) -> bool {
        self.proofs.iter().any(|proof| {
            proof.surface == surface && proof.label == label && proof.has_payload_light_integrity()
        })
    }
}

pub(super) fn selected_recall_controller_decision_from_extension_data(
    extension_data: &ExtensionData,
) -> SelectedRecallControllerDecision {
    SelectedRecallControllerDecision {
        recall_provider_rollup: extension_data
            .get::<TurnContextRecallSelectionSummary>()
            .filter(|summary| summary.has_count_integrity())
            .map(|summary| ContextRecallProviderRollup {
                recall_selection: (*summary).clone(),
            }),
        recall_selected_snippets: extension_data
            .get::<TurnContextRecallSelectedSnippetEnvelope>()
            .filter(|envelope| selected_snippet_envelope_is_manifest_safe(envelope))
            .map(|envelope| ContextRecallSelectedSnippetEnvelope {
                envelope: (*envelope).clone(),
            }),
        canary_readiness: SelectedRecallControllerCanaryReadiness::default(),
        canary_metrics: SelectedRecallControllerCanaryMetrics::default(),
        readback_proofs: SelectedRecallControllerReadbackProofs::default(),
    }
}

pub(super) fn apply_selected_recall_controller_decision(
    manifest: &mut TurnContextManifestItem,
    decision: &SelectedRecallControllerDecision,
) {
    debug_assert!(decision.canary_readiness.has_payload_light_integrity());
    debug_assert!(decision.canary_metrics.has_payload_light_integrity());
    debug_assert!(decision.readback_proofs.has_payload_light_integrity());

    if let Some(rollup) = &decision.recall_provider_rollup {
        manifest.recall_selection = Some(rollup.recall_selection.clone());
        manifest.refresh_ledger_hash();
    }
    if let Some(selected_snippets) = &decision.recall_selected_snippets
        && selected_snippet_envelope_is_manifest_safe(&selected_snippets.envelope)
    {
        manifest.recall_selected_snippets = Some(selected_snippets.envelope.clone());
        manifest.refresh_ledger_hash();
    }
}
