use std::sync::Arc;

use codex_app_server_protocol::HEPTA_EVIDENCE_SUMMARY_SCHEMA_VERSION;
use codex_app_server_protocol::HeptaChannelIngressEvidenceSummary;
use codex_app_server_protocol::HeptaEvidenceSummaryReadParams;
use codex_app_server_protocol::HeptaEvidenceSummaryReadResponse;
use codex_app_server_protocol::HeptaGovernanceEvidenceSummary;
use codex_app_server_protocol::HeptaProviderEvidenceSummary;
use codex_app_server_protocol::JSONRPCErrorError;
use codex_hepta_evidence::EvidenceSummary;
use codex_hepta_evidence::HeptaEvidenceStore;
use codex_rollout::StateDbHandle;

use crate::error_code::internal_error;
use crate::error_code::method_not_found;

pub(crate) struct HeptaEvidenceRequestProcessor {
    enabled: bool,
    state_db: Option<StateDbHandle>,
    evidence: tokio::sync::OnceCell<Result<Arc<HeptaEvidenceStore>, Arc<str>>>,
}

impl HeptaEvidenceRequestProcessor {
    pub(crate) fn new(enabled: bool, state_db: Option<StateDbHandle>) -> Self {
        Self {
            enabled,
            state_db,
            evidence: tokio::sync::OnceCell::new(),
        }
    }

    pub(crate) async fn summary_read(
        &self,
        _params: HeptaEvidenceSummaryReadParams,
    ) -> Result<HeptaEvidenceSummaryReadResponse, JSONRPCErrorError> {
        if !self.enabled {
            return Err(method_not_found("Hepta evidence is not enabled"));
        }
        let evidence = self.evidence().await?;
        let summary = evidence.summary().await.map_err(|error| {
            tracing::error!(detail = %error, "Hepta evidence summary failed");
            internal_error("Hepta evidence summary is unavailable")
        })?;
        Ok(summary_response(summary))
    }

    async fn evidence(&self) -> Result<Arc<HeptaEvidenceStore>, JSONRPCErrorError> {
        let evidence = self
            .evidence
            .get_or_init(|| async {
                match self.state_db.as_ref() {
                    Some(state_db) => HeptaEvidenceStore::open(state_db.sqlite())
                        .await
                        .map(Arc::new)
                        .map_err(|error| Arc::<str>::from(error.to_string())),
                    None => Err(Arc::from("Codex state runtime is unavailable")),
                }
            })
            .await;
        evidence.clone().map_err(|detail| {
            tracing::error!(%detail, "Hepta evidence store failed to open");
            internal_error("Hepta evidence store is unavailable")
        })
    }
}

fn summary_response(summary: EvidenceSummary) -> HeptaEvidenceSummaryReadResponse {
    HeptaEvidenceSummaryReadResponse {
        schema_version: HEPTA_EVIDENCE_SUMMARY_SCHEMA_VERSION,
        governance: HeptaGovernanceEvidenceSummary {
            decisions: summary.governance.decisions,
            receipts: summary.governance.receipts,
            pending_actions: summary.governance.pending_actions,
        },
        provider: HeptaProviderEvidenceSummary {
            intents: summary.provider.intents,
            receipts: summary.provider.receipts,
            pending_attempts: summary.provider.pending_attempts,
            indeterminate_attempts: summary.provider.indeterminate_attempts,
        },
        channel_ingress: HeptaChannelIngressEvidenceSummary {
            events: summary.channel_ingress.events,
            receipts: summary.channel_ingress.receipts,
            pending_events: summary.channel_ingress.pending_events,
            indeterminate_events: summary.channel_ingress.indeterminate_events,
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::error_code::INTERNAL_ERROR_CODE;
    use crate::error_code::METHOD_NOT_FOUND_ERROR_CODE;

    use super::*;

    #[tokio::test]
    async fn disabled_product_does_not_expose_hepta_evidence() {
        let processor = HeptaEvidenceRequestProcessor::new(false, None);
        let error = processor
            .summary_read(HeptaEvidenceSummaryReadParams {})
            .await
            .expect_err("ordinary Codex must not expose Hepta evidence");
        assert_eq!(error.code, METHOD_NOT_FOUND_ERROR_CODE);
    }

    #[tokio::test]
    async fn enabled_product_without_state_runtime_fails_closed() {
        let processor = HeptaEvidenceRequestProcessor::new(true, None);
        let error = processor
            .summary_read(HeptaEvidenceSummaryReadParams {})
            .await
            .expect_err("missing state runtime must fail closed");
        assert_eq!(error.code, INTERNAL_ERROR_CODE);
    }

    #[test]
    fn evidence_projection_maps_every_authoritative_count() {
        let response = summary_response(EvidenceSummary {
            governance: codex_hepta_evidence::GovernanceEvidenceSummary {
                decisions: 1,
                receipts: 2,
                pending_actions: 3,
            },
            provider: codex_hepta_evidence::ProviderEvidenceSummary {
                intents: 4,
                receipts: 5,
                pending_attempts: 6,
                indeterminate_attempts: 7,
            },
            channel_ingress: codex_hepta_evidence::ChannelIngressEvidenceSummary {
                events: 8,
                receipts: 9,
                pending_events: 10,
                indeterminate_events: 11,
            },
        });

        assert_eq!(response.schema_version, 1);
        assert_eq!(response.governance.decisions, 1);
        assert_eq!(response.governance.receipts, 2);
        assert_eq!(response.governance.pending_actions, 3);
        assert_eq!(response.provider.intents, 4);
        assert_eq!(response.provider.receipts, 5);
        assert_eq!(response.provider.pending_attempts, 6);
        assert_eq!(response.provider.indeterminate_attempts, 7);
        assert_eq!(response.channel_ingress.events, 8);
        assert_eq!(response.channel_ingress.receipts, 9);
        assert_eq!(response.channel_ingress.pending_events, 10);
        assert_eq!(response.channel_ingress.indeterminate_events, 11);
    }
}
