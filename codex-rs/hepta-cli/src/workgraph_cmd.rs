use serde::Serialize;

use hepta_runtime::WorkGraphControlPlaneInspection;
use hepta_runtime::WorkGraphControlPlaneReport;
use hepta_runtime::WorkGraphFrontierCandidate;
use hepta_runtime::WorkNode;
use hepta_runtime::hepta_work_graph_control_plane_status_frontier_report;
use hepta_runtime::work_graph_control_plane_inspect;

pub fn run(args: &[String]) -> anyhow::Result<()> {
    let command = WorkgraphCommand::parse(args)?;

    match command.action {
        WorkgraphAction::Status => {
            let report = hepta_work_graph_control_plane_status_frontier_report();
            if command.json {
                print_json(&report)?;
            } else {
                print_status(&report);
            }
        }
        WorkgraphAction::Frontier => {
            let report = hepta_work_graph_control_plane_status_frontier_report();
            if command.json {
                print_json(&report.frontier_candidates)?;
            } else {
                print_frontier(&report.frontier_candidates);
            }
        }
        WorkgraphAction::Inspect { id } => {
            let inspection = work_graph_control_plane_inspect(&id);
            if command.json {
                print_json(&inspection)?;
            } else {
                print_inspection(&inspection);
            }
        }
        WorkgraphAction::Help => print_help(),
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkgraphCommand {
    action: WorkgraphAction,
    json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum WorkgraphAction {
    Status,
    Frontier,
    Inspect { id: String },
    Help,
}

impl WorkgraphCommand {
    fn parse(args: &[String]) -> anyhow::Result<Self> {
        let mut json = false;
        let mut positional = Vec::new();

        for arg in args {
            match arg.as_str() {
                "--json" | "-j" => json = true,
                "--help" | "-h" | "help" => positional.push("help".to_string()),
                _ => positional.push(arg.clone()),
            }
        }

        let action = match positional.as_slice() {
            [] => WorkgraphAction::Status,
            [cmd] if cmd == "status" => WorkgraphAction::Status,
            [cmd] if cmd == "frontier" => WorkgraphAction::Frontier,
            [cmd] if cmd == "help" => WorkgraphAction::Help,
            [cmd, id] if cmd == "inspect" => WorkgraphAction::Inspect { id: id.clone() },
            [cmd] if cmd == "inspect" => {
                anyhow::bail!("hepta workgraph inspect requires a short ID or gate ID")
            }
            [cmd, ..] => anyhow::bail!("unknown hepta workgraph command: {cmd}"),
        };

        Ok(Self { action, json })
    }
}

fn print_status(report: &WorkGraphControlPlaneReport) {
    println!("Hepta WorkGraph control plane ({})", report.preview_mode);
    println!("gate: {}", report.gate);
    println!("status: {}", report.status);
    println!(
        "sources: {}, nodes: {}, frontier: {}, blockers: {}, required_prior: {}",
        report.source_report_count,
        report.status_node_count,
        report.frontier_candidate_count,
        report.blocker_count,
        report.required_prior_gate_count
    );
    println!("recommended_next_gate: {}", report.recommended_next_gate);
    println!(
        "boundaries: writes=false wal=false checkpoint=false replay=false enforcement=false live=false"
    );
    println!();
    println!("status nodes:");
    for node in &report.status_nodes {
        print_node_line(node);
    }
}

fn print_frontier(frontier: &[WorkGraphFrontierCandidate]) {
    println!("Hepta WorkGraph frontier candidates");
    for candidate in frontier {
        let blocked_by = if candidate.blocked_by.is_empty() {
            "none".to_string()
        } else {
            candidate.blocked_by.join(",")
        };
        println!(
            "{}. [{}] {} -> {}",
            candidate.priority,
            candidate.optimization_stage,
            candidate.id,
            candidate.recommended_gate
        );
        println!("   node: {}", candidate.work_node_id);
        println!("   next: {}", candidate.next_action);
        println!("   blocked_by: {blocked_by}");
    }
}

fn print_inspection(inspection: &WorkGraphControlPlaneInspection) {
    if !inspection.found {
        println!("not found: {}", inspection.query);
        println!("try: hepta workgraph status or hepta workgraph frontier");
        return;
    }

    println!("query: {}", inspection.query);
    if let Some(target_kind) = inspection.target_kind {
        println!("target_kind: {target_kind}");
    }
    if let Some(node) = &inspection.status_node {
        println!("work_node_id: {}", node.work_node_id);
        println!("gate_id: {}", node.gate_id);
        println!("lineage_id: {}", node.lineage_id);
        println!("source_report_hash: {}", node.source_report_hash);
        println!(
            "required_prior: {}",
            if node.required_prior_gate_ids.is_empty() {
                "none".to_string()
            } else {
                node.required_prior_gate_ids.join(",")
            }
        );
        println!("frontier: {}", node.frontier);
        println!("status: {}", node.status);
        println!("next_action: {}", node.next_action);
        return;
    }
    if let Some(candidate) = &inspection.frontier_candidate {
        println!("frontier_id: {}", candidate.id);
        println!("priority: {}", candidate.priority);
        println!("stage: {}", candidate.optimization_stage);
        println!("work_node_id: {}", candidate.work_node_id);
        println!("recommended_gate: {}", candidate.recommended_gate);
        println!("next_action: {}", candidate.next_action);
        return;
    }
    if let Some(source) = &inspection.source_report {
        println!("source_id: {}", source.source_id);
        println!("gate: {}", source.gate);
        println!("schema_version: {}", source.schema_version);
        println!("source_report_hash: {}", source.source_report_hash);
        println!("ready: {}", source.ready);
        println!("no_write_confirmed: {}", source.no_write_confirmed);
        return;
    }
    if let Some(blocker) = &inspection.blocker {
        println!("blocker_id: {}", blocker.id);
        println!("severity: {}", blocker.severity);
        println!("surface: {}", blocker.surface);
        println!("summary: {}", blocker.summary);
        println!("recommended_fix: {}", blocker.recommended_fix);
    }
}

fn print_node_line(node: &WorkNode) {
    println!(
        "- {} [{}] frontier={} gate={}",
        node.work_node_id, node.status, node.frontier, node.gate_id
    );
    println!("  next: {}", node.next_action);
}

fn print_help() {
    println!("Usage:");
    println!("  hepta workgraph status [--json]");
    println!("  hepta workgraph frontier [--json]");
    println!("  hepta workgraph inspect <short-id-or-gate-id> [--json]");
}

fn print_json<T: Serialize>(value: &T) -> anyhow::Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn parses_default_status() {
        let command = WorkgraphCommand::parse(&[]).expect("parse");

        assert_eq!(
            command,
            WorkgraphCommand {
                action: WorkgraphAction::Status,
                json: false
            }
        );
    }

    #[test]
    fn parses_frontier_json() {
        let args = vec!["frontier".to_string(), "--json".to_string()];
        let command = WorkgraphCommand::parse(&args).expect("parse");

        assert_eq!(
            command,
            WorkgraphCommand {
                action: WorkgraphAction::Frontier,
                json: true
            }
        );
    }

    #[test]
    fn parses_inspect_id() {
        let args = vec![
            "inspect".to_string(),
            "wg.event_store.wal_precondition.v1".to_string(),
        ];
        let command = WorkgraphCommand::parse(&args).expect("parse");

        assert_eq!(
            command,
            WorkgraphCommand {
                action: WorkgraphAction::Inspect {
                    id: "wg.event_store.wal_precondition.v1".to_string()
                },
                json: false
            }
        );
    }
}
