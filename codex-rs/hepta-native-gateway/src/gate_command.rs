use anyhow::Result;
use serde::Serialize;

use crate::gate_runner;
use crate::gate_spec::GateSpec;
use crate::gate_spec::ReceiptStateMachine;
use crate::route_manifest::ROUTE_EFFECT_GATE_MANIFEST_SCHEMA;
use crate::route_manifest::route_manifest_digest;
use crate::route_manifest::route_manifest_registry;
use crate::route_registry::CONTROL_UI_ROUTE_SPECS;
use crate::runtime_ingress::IngressEffectClass;
use crate::runtime_ingress::RUNTIME_INGRESS_REGISTRY_SCHEMA_VERSION;
use crate::runtime_ingress::runtime_ingress_lifecycle_registry_digest;

pub fn gate_command_json(raw_args: &[String]) -> Result<String> {
    match raw_args {
        [flag] if flag == "--list" => gate_registry_json(),
        [flag] if flag == "--shell-list" => gate_runner::shell_gate_catalog_json(),
        [flag] if flag == "--shell-snapshot" => gate_runner::shell_gate_snapshot_json(),
        [id] => gate_spec_json(id),
        [id, flag] if flag == "--json" => gate_spec_json(id),
        [id, flag] if flag == "--execute" => gate_runner::execute_gate(id),
        [id, flag] if flag == "--report" => gate_runner::execute_report(id),
        [] => anyhow::bail!(
            "usage: hepta gate <id> [--json|--execute|--report] | hepta gate [--list|--shell-list|--shell-snapshot]"
        ),
        _ => anyhow::bail!(
            "usage: hepta gate <id> [--json|--execute|--report] | hepta gate [--list|--shell-list|--shell-snapshot]"
        ),
    }
}

fn gate_registry_json() -> Result<String> {
    let route_manifest = route_manifest_registry();
    let route_manifest_digest = route_manifest_digest()?;
    let ingress_lifecycle_registry = route_manifest
        .iter()
        .map(|entry| entry.lifecycle)
        .collect::<Vec<_>>();
    let ingress_lifecycle_registry_digest = runtime_ingress_lifecycle_registry_digest()?;
    let quarantined_effect_route_count = route_manifest
        .iter()
        .filter(|entry| {
            entry.lifecycle.effect_class == IngressEffectClass::QuarantinedLegacyMutation
        })
        .count();
    let gates = CONTROL_UI_ROUTE_SPECS
        .iter()
        .map(|spec| {
            let route_manifest_entry = route_manifest.iter().find(|entry| {
                entry.lifecycle.method == spec.method
                    && entry.lifecycle.path_pattern == spec.pattern
            });
            serde_json::json!({
                "id": spec.capability,
                "method": spec.method,
                "pattern": spec.pattern,
                "source_command": spec.source_command,
                "side_effect_boundary": spec.side_effect_boundary,
                "read_only": spec.is_read_only(),
                "dry_run_only": spec.is_dry_run(),
                "guarded": spec.is_guarded(),
                "requires_confirmation": spec.requires_confirmation(),
                "receipt_state": spec.receipt_state(),
                "route_manifest_entry": route_manifest_entry,
            })
        })
        .collect::<Vec<_>>();

    Ok(json_or_error(&serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": "ready",
        "runner": "hepta gate",
        "mode": "declarative_registry_read_only",
        "gate_count": gates.len(),
        "route_count_source": "hepta_native_gateway::route_manifest",
        "route_effect_gate_manifest_schema_version": ROUTE_EFFECT_GATE_MANIFEST_SCHEMA,
        "route_effect_gate_manifest_sha256": route_manifest_digest,
        "route_effect_gate_manifest_count": route_manifest.len(),
        "ingress_lifecycle_registry_schema_version": RUNTIME_INGRESS_REGISTRY_SCHEMA_VERSION,
        "ingress_lifecycle_registry_sha256": ingress_lifecycle_registry_digest,
        "ingress_lifecycle_count": ingress_lifecycle_registry.len(),
        "ingress_lifecycle_registry": ingress_lifecycle_registry,
        "quarantined_effect_route_count": quarantined_effect_route_count,
        "release_dispatch_ready": quarantined_effect_route_count == 0,
        "route_effect_gate_manifest": route_manifest,
        "receipt_state_machine": ReceiptStateMachine::ORDERED_STATES,
        "report_execution_performed": false,
        "side_effect_free": true,
        "gates": gates,
    })))
}

fn gate_spec_json(id: &str) -> Result<String> {
    if let Some(spec) = CONTROL_UI_ROUTE_SPECS
        .iter()
        .find(|spec| gate_spec_matches_id(spec, id))
    {
        return Ok(json_or_error(&serde_json::json!({
            "product": "Hepta",
            "runtime": "hepta",
            "status": "ready",
            "runner": "hepta gate",
            "mode": "declarative_registry_read_only",
            "id": spec.capability,
            "method": spec.method,
            "pattern": spec.pattern,
            "source_command": spec.source_command,
            "side_effect_boundary": spec.side_effect_boundary,
            "read_only": spec.is_read_only(),
            "dry_run_only": spec.is_dry_run(),
            "guarded": spec.is_guarded(),
            "requires_confirmation": spec.requires_confirmation(),
            "receipt_state": spec.receipt_state(),
            "registered_route_count": CONTROL_UI_ROUTE_SPECS.len(),
            "report_execution_performed": false,
            "side_effect_free": true,
        })));
    }
    if let Some(spec_json) = gate_runner::migrated_pair_spec_json(id)? {
        return Ok(spec_json);
    }
    anyhow::bail!("unknown Hepta gate id: {id}")
}

fn gate_spec_matches_id(spec: &GateSpec, id: &str) -> bool {
    spec.capability == id || spec.pattern.strip_prefix("/api/") == Some(id)
}

fn json_or_error<T: Serialize>(value: &T) -> String {
    match serde_json::to_string(value) {
        Ok(json) => json,
        Err(err) => format!(r#"{{"error":"native gateway serialization failed: {err}"}}"#),
    }
}
