use pretty_assertions::assert_eq;

use super::*;

#[test]
fn control_plane_is_read_only_and_short_id_addressable() {
    let report = hepta_work_graph_control_plane_status_frontier_report();

    assert_eq!(report.status, "ready");
    assert_eq!(report.status_node_count, report.status_nodes.len());
    assert_eq!(
        report.frontier_candidate_count,
        report.frontier_candidates.len()
    );
    assert_eq!(
        report.side_effects,
        WorkGraphControlPlaneSideEffects::none()
    );
    assert!(report.ready_for_read_only_workgraph_status_frontier);
    assert!(!report.ready_for_live_execution);
    assert!(
        report
            .status_nodes
            .iter()
            .all(|node| !node.work_node_id.is_empty()
                && !node.gate_id.is_empty()
                && !node.lineage_id.is_empty()
                && node.source_report_hash.starts_with("sha256:")
                && !node.frontier.is_empty()
                && !node.next_action.is_empty())
    );
}

#[test]
fn frontier_keeps_p3_before_enforcement_and_live() {
    let report = hepta_work_graph_control_plane_status_frontier_report();
    let frontier_ids: Vec<_> = report
        .frontier_candidates
        .iter()
        .map(|candidate| candidate.id)
        .collect();

    assert_eq!(
        frontier_ids,
        vec![
            "p1.workgraph_status_frontier_cli",
            "p2.schema_backed_fixture_generation",
            "p3.wal_precondition_readback",
            "p4.agent_jobs_task_board_dry_run_enforcement",
            "p5.agent_card_role_manifest_admission",
            "p6.live_canary_cutover",
        ]
    );
}

#[test]
fn inspect_finds_node_and_returns_next_action() {
    let inspection = work_graph_control_plane_inspect("wg.event_store.wal_precondition.v1");

    assert!(inspection.found);
    assert_eq!(inspection.target_kind, Some("status_node"));
    assert_eq!(
        inspection.status_node.map(|node| node.gate_id),
        Some(
            "hepta_work_graph_append_only_event_store_feature_gated_wal_precondition_gate"
                .to_string()
        )
    );
    assert!(
        inspection
            .next_action
            .is_some_and(|action| action.contains("WAL preconditions"))
    );
}
