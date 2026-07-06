use super::*;
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
fn parse_csv_supports_quotes_and_commas() {
    let input = "id,name\n1,\"alpha, beta\"\n2,gamma\n";
    let (headers, rows) = parse_csv(input).expect("csv parse");
    assert_eq!(headers, vec!["id".to_string(), "name".to_string()]);
    assert_eq!(
        rows,
        vec![
            vec!["1".to_string(), "alpha, beta".to_string()],
            vec!["2".to_string(), "gamma".to_string()]
        ]
    );
}

#[test]
fn csv_escape_quotes_when_needed() {
    assert_eq!(csv_escape("simple"), "simple");
    assert_eq!(csv_escape("a,b"), "\"a,b\"");
    assert_eq!(csv_escape("a\"b"), "\"a\"\"b\"");
}

#[test]
fn render_instruction_template_expands_placeholders_and_escapes_braces() {
    let row = json!({
        "path": "src/lib.rs",
        "area": "test",
        "file path": "docs/readme.md",
    });
    let rendered = render_instruction_template(
        "Review {path} in {area}. Also see {file path}. Use {{literal}}.",
        &row,
    );
    assert_eq!(
        rendered,
        "Review src/lib.rs in test. Also see docs/readme.md. Use {literal}."
    );
}

#[test]
fn render_instruction_template_leaves_unknown_placeholders() {
    let row = json!({
        "path": "src/lib.rs",
    });
    let rendered = render_instruction_template("Check {path} then {missing}", &row);
    assert_eq!(rendered, "Check src/lib.rs then {missing}");
}

#[test]
fn ensure_unique_headers_rejects_duplicates() {
    let headers = vec!["path".to_string(), "path".to_string()];
    let Err(err) = ensure_unique_headers(headers.as_slice()) else {
        panic!("expected duplicate header error");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel("csv header path is duplicated".to_string())
    );
}

#[tokio::test]
async fn spawn_agents_on_csv_role_manifest_allows_task_result_worker() {
    let (_session, turn) = crate::session::tests::make_session_and_context().await;

    let decision = build_spawn_agents_on_csv_role_manifest_shadow_decision(
        4, &turn, /*output_schema_present*/ true,
    );

    assert_eq!(decision.decision, "allow_shadow_manifest_no_live_blocking");
    assert_eq!(decision.definition_source, "explicit_agent_card_manifest");
    assert_eq!(decision.manifest_version, "hepta.agent_card_manifest.v1");
    assert_eq!(
        decision.manifest_id,
        "agent-card:spawn_agents_on_csv:agent_job_worker"
    );
    assert_eq!(decision.role_name.as_deref(), Some("agent_job_worker"));
    assert_eq!(
        decision.allowed_tools,
        vec!["report_agent_job_result".to_string()]
    );
    assert_eq!(decision.attempted_tool, Some("report_agent_job_result"));
    assert_eq!(decision.tool_allowed, Some(true));
    assert_eq!(decision.lane, "agent_jobs");
    assert_eq!(decision.observed_lane, Some("agent_jobs"));
    assert_eq!(decision.lane_allowed, Some(true));
    assert!(decision.result_contract_present);
    assert!(decision.verifier_present);
    assert!(decision.reducer_present);
    assert!(!decision.live_blocking_enabled);
    assert!(!decision.live_cutover_enabled);
}
