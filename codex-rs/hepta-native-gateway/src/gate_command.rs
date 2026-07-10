use anyhow::Result;
use serde::Serialize;

use crate::gate_runner;
use crate::gate_spec::GateSpec;
use crate::gate_spec::ReceiptStateMachine;
use crate::route_registry::CONTROL_UI_ROUTE_SPECS;

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
    let gates = CONTROL_UI_ROUTE_SPECS
        .iter()
        .map(|spec| {
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
        "route_count_source": "CONTROL_UI_ROUTE_SPECS",
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
