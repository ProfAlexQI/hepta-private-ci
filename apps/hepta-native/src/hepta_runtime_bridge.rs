//! Local bridge from Hepta Native into the current `codex-rs/hepta-*` crates.
//!
//! This module intentionally stays read-only. It imports deterministic readiness
//! and capability reports from the current Rust crates and projects them into the
//! existing Matrix-shaped `m.hepta.*` fixture path. It does not call Gateway,
//! Matrix, Telegram, providers, process runners, or task mutation APIs.

use crate::hepta_bridge::HeptaBridgeEventInput;
use crate::hepta_event::HeptaEventStatus;

use hepta_core::hepta_native_absorption_report;
use hepta_runtime::{evaluate_runtime_readiness, live_adapter_activation_discipline_sample};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaCodexRuntimeBridgeSnapshot {
    pub source: &'static str,
    pub product: String,
    pub absorption_status: String,
    pub capability_count: usize,
    pub feature_count: usize,
    pub native_absorption_coverage_percent: u8,
    pub local_executable_coverage_percent: u8,
    pub external_boundary_count: usize,
    pub readiness_stage_label: String,
    pub readiness_product_ready: bool,
    pub readiness_warning: Option<String>,
    pub live_activation_sample_count: usize,
    pub live_activation_permitted_count: usize,
    pub dry_run_only_activation_count: usize,
    pub bridge_external_side_effect_performed: bool,
    pub gateway_called_by_bridge: bool,
    pub provider_invoked_by_bridge: bool,
    pub channel_delivery_performed_by_bridge: bool,
    pub process_spawned_by_bridge: bool,
}

impl HeptaCodexRuntimeBridgeSnapshot {
    pub fn summary_line(&self) -> String {
        format!(
            "{} · {} capabilities · readiness={} · product_ready={} · live_activation_permitted={}",
            self.source,
            self.capability_count,
            self.readiness_stage_label,
            self.readiness_product_ready,
            self.live_activation_permitted_count,
        )
    }

    pub fn as_payload_value(&self) -> Value {
        json!({
            "title": "Current codex-rs Hepta runtime bridge",
            "body": self.summary_line(),
            "bridge": self,
        })
    }
}

pub fn sample_current_codex_runtime_bridge_snapshot(
) -> Result<HeptaCodexRuntimeBridgeSnapshot, String> {
    let absorption = hepta_native_absorption_report();
    let readiness = evaluate_runtime_readiness(
        "hepta-native-current-codex-runtime-bridge",
        true,
        absorption.native_absorption_complete(),
        false,
        true,
        true,
        true,
    )
    .map_err(|err| err.to_string())?;
    let activation_reports =
        live_adapter_activation_discipline_sample().map_err(|err| err.to_string())?;
    let bridge_external_side_effect_performed = activation_reports.iter().any(|report| {
        report.live_side_effect_performed_by_gate
            || report.provider_invoked_by_gate
            || report.channel_delivery_performed_by_gate
            || report.node_invoked_by_gate
            || report.process_spawned_by_gate
    });

    Ok(HeptaCodexRuntimeBridgeSnapshot {
        source: "codex-rs/hepta-*",
        product: absorption.product.to_string(),
        absorption_status: absorption.absorption_status.to_string(),
        capability_count: absorption.capability_count,
        feature_count: absorption.feature_count,
        native_absorption_coverage_percent: absorption.native_absorption_coverage_percent,
        local_executable_coverage_percent: absorption.local_executable_coverage_percent,
        external_boundary_count: absorption.external_boundary_count,
        readiness_stage_label: readiness.stage.label().to_string(),
        readiness_product_ready: readiness.product_ready,
        readiness_warning: readiness.warning,
        live_activation_sample_count: activation_reports.len(),
        live_activation_permitted_count: activation_reports
            .iter()
            .filter(|report| report.activation_permitted)
            .count(),
        dry_run_only_activation_count: activation_reports
            .iter()
            .filter(|report| report.dry_run && !report.activation_permitted)
            .count(),
        bridge_external_side_effect_performed,
        gateway_called_by_bridge: false,
        provider_invoked_by_bridge: false,
        channel_delivery_performed_by_bridge: false,
        process_spawned_by_bridge: false,
    })
}

pub fn current_codex_runtime_bridge_event_input() -> Result<HeptaBridgeEventInput, String> {
    let snapshot = sample_current_codex_runtime_bridge_snapshot()?;
    let status = if snapshot.readiness_product_ready {
        HeptaEventStatus::Completed
    } else {
        HeptaEventStatus::Waiting
    };
    let mut input = HeptaBridgeEventInput::new(
        "runtime_event",
        "current-codex-runtime-bridge",
        status,
        snapshot.summary_line(),
    );
    input.conversation_id = Some("hepta-native-current-codex".to_string());
    input.payload = snapshot.as_payload_value();
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_bridge::HeptaBridgeMatrixEvent;
    use crate::hepta_event::{HeptaEventEnvelope, EVENT_RUNTIME_EVENT};

    #[test]
    fn snapshot_reads_current_codex_hepta_crates_without_live_side_effects() {
        let snapshot = sample_current_codex_runtime_bridge_snapshot().unwrap();

        assert_eq!(snapshot.source, "codex-rs/hepta-*");
        assert_eq!(snapshot.product, "Hepta");
        assert_eq!(snapshot.absorption_status, "complete");
        assert_eq!(snapshot.native_absorption_coverage_percent, 100);
        assert_eq!(snapshot.local_executable_coverage_percent, 100);
        assert_eq!(snapshot.readiness_stage_label, "M3-gated-live-adapter");
        assert!(!snapshot.readiness_product_ready);
        assert_eq!(snapshot.live_activation_sample_count, 4);
        assert_eq!(snapshot.live_activation_permitted_count, 0);
        assert_eq!(snapshot.dry_run_only_activation_count, 4);
        assert!(!snapshot.bridge_external_side_effect_performed);
        assert!(!snapshot.gateway_called_by_bridge);
        assert!(!snapshot.provider_invoked_by_bridge);
        assert!(!snapshot.channel_delivery_performed_by_bridge);
        assert!(!snapshot.process_spawned_by_bridge);
    }

    #[test]
    fn current_codex_runtime_snapshot_projects_to_matrix_event() {
        let input = current_codex_runtime_bridge_event_input().unwrap();
        let event = HeptaBridgeMatrixEvent::from_input(
            "!hepta-runtime-fixture:local",
            "@hepta-runtime:local",
            input,
        )
        .unwrap();

        assert_eq!(event.event_type, EVENT_RUNTIME_EVENT);
        let envelope = HeptaEventEnvelope::from_content_value(&event.content).unwrap();
        assert_eq!(envelope.event_kind, "runtime_event");
        assert_eq!(envelope.status, HeptaEventStatus::Waiting);
        assert_eq!(
            envelope
                .payload
                .pointer("/bridge/source")
                .and_then(Value::as_str),
            Some("codex-rs/hepta-*"),
        );
        assert_eq!(
            envelope
                .payload
                .pointer("/bridge/gateway_called_by_bridge")
                .and_then(Value::as_bool),
            Some(false),
        );
    }
}
