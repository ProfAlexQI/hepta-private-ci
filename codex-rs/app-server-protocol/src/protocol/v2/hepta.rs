use crate::JsonSchema;
use crate::TS;
use serde::Deserialize;
use serde::Serialize;

pub const HEPTA_EVIDENCE_SUMMARY_SCHEMA_VERSION: u32 = 1;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct HeptaEvidenceSummaryReadParams {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct HeptaGovernanceEvidenceSummary {
    pub decisions: u64,
    pub receipts: u64,
    pub pending_actions: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct HeptaProviderEvidenceSummary {
    pub intents: u64,
    pub receipts: u64,
    pub pending_attempts: u64,
    pub indeterminate_attempts: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct HeptaChannelIngressEvidenceSummary {
    pub events: u64,
    pub receipts: u64,
    pub pending_events: u64,
    pub indeterminate_events: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct HeptaEvidenceSummaryReadResponse {
    pub schema_version: u32,
    pub governance: HeptaGovernanceEvidenceSummary,
    pub provider: HeptaProviderEvidenceSummary,
    pub channel_ingress: HeptaChannelIngressEvidenceSummary,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn evidence_summary_uses_one_camel_case_typed_schema() {
        assert_eq!(
            serde_json::to_value(HeptaEvidenceSummaryReadParams::default())
                .expect("serialize empty evidence summary params"),
            json!({})
        );

        let response = HeptaEvidenceSummaryReadResponse {
            schema_version: HEPTA_EVIDENCE_SUMMARY_SCHEMA_VERSION,
            governance: HeptaGovernanceEvidenceSummary {
                decisions: 2,
                receipts: 1,
                pending_actions: 0,
            },
            provider: HeptaProviderEvidenceSummary {
                intents: 3,
                receipts: 2,
                pending_attempts: 1,
                indeterminate_attempts: 1,
            },
            channel_ingress: HeptaChannelIngressEvidenceSummary {
                events: 4,
                receipts: 3,
                pending_events: 1,
                indeterminate_events: 1,
            },
        };

        assert_eq!(
            serde_json::to_value(response).expect("serialize evidence summary"),
            json!({
                "schemaVersion": 1,
                "governance": {"decisions": 2, "receipts": 1, "pendingActions": 0},
                "provider": {
                    "intents": 3,
                    "receipts": 2,
                    "pendingAttempts": 1,
                    "indeterminateAttempts": 1
                },
                "channelIngress": {
                    "events": 4,
                    "receipts": 3,
                    "pendingEvents": 1,
                    "indeterminateEvents": 1
                }
            })
        );
    }
}
