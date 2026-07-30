use super::*;

#[tokio::test]
async fn activity_summary_respects_session_filter_and_limits() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("alpha first")
        .await
        .expect("alpha turn should succeed");
    runtime
        .run_demo_turn("alpha second")
        .await
        .expect("second alpha turn should succeed");
    runtime
        .run_demo_turn_in_session("beta", "beta only")
        .await
        .expect("beta turn should succeed");

    let summary = runtime
        .activity_summary(Some("alpha"), 1, 2)
        .expect("activity summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime activity: session alpha"));
    assert!(rendered.contains("- recent history entries: 1"));
    assert!(rendered.contains("- recent events: 2"));
    assert!(rendered.contains("alpha second"));
    assert!(!rendered.contains("beta only"));
}

#[tokio::test]
async fn activity_summary_includes_recent_history_and_event_lines() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("capture event line")
        .await
        .expect("turn should succeed");
    runtime
        .rename_active_session("Alpha workspace")
        .expect("rename should succeed");

    let summary = runtime
        .activity_summary(Some("alpha"), 2, 4)
        .expect("activity summary should succeed");
    let rendered = summary.join("\n");

    assert!(summary.iter().any(|line| line == "Recent history:"));
    assert!(summary.iter().any(|line| line == "Recent events:"));
    assert!(rendered.contains("capture event line"));
    assert!(rendered.contains("SessionRenamed"));
    assert!(rendered.contains("Alpha workspace"));
}

#[tokio::test]
async fn session_activity_summary_covers_multiple_sessions_and_status_flags() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("alpha planning")
        .await
        .expect("alpha turn should succeed");
    runtime
        .rename_active_session("Alpha workspace")
        .expect("rename should succeed");
    runtime
        .run_demo_turn_in_session("beta", "beta follow-up")
        .await
        .expect("beta turn should succeed");
    runtime
        .route_topics("alpha", Some("alpha planning"), 4, 4, 4, 1)
        .expect("alpha route should succeed");
    runtime
        .archive_session(Some("beta"))
        .expect("archive should succeed");

    let summary = runtime
        .session_activity_summary(1, 2)
        .expect("session activity summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime session activity:"));
    assert!(rendered.contains("- sessions: 2"));
    assert!(rendered.contains("- active sessions: 1"));
    assert!(rendered.contains("- archived sessions: 1"));
    assert!(rendered.contains("- sessions with recent history: 2"));
    assert!(rendered.contains("- sessions with recent events: 2"));
    assert!(rendered.contains("- sessions with topic state: 1"));
    assert!(rendered.contains("- total topic sessions: 1"));
    assert!(rendered.contains("- total topic graph edges: 0"));
    assert!(rendered.contains("alpha, active"));
    assert!(rendered.contains("title=\"Alpha workspace\""));
    assert!(rendered.contains("topic_sessions=1, topic_graph_edges=0"));
    assert!(rendered.contains("beta, archived"));
    assert!(rendered.contains("latest_user=\"beta follow-up\""));
    assert!(rendered.contains("latest_event=SessionArchived"));
}

#[tokio::test]
async fn session_activity_summary_applies_per_session_limits() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("alpha first")
        .await
        .expect("first alpha turn should succeed");
    runtime
        .run_demo_turn("alpha second")
        .await
        .expect("second alpha turn should succeed");
    runtime
        .rename_active_session("Alpha workspace")
        .expect("rename should succeed");

    let summary = runtime
        .session_activity_summary(1, 1)
        .expect("session activity summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("alpha, active"));
    assert!(rendered.contains("history=1, events=1"));
    assert!(rendered.contains("latest_user=\"alpha second\""));
    assert!(rendered.contains("latest_event=SessionRenamed"));
}

#[tokio::test]
async fn event_digest_summary_groups_recent_events_by_kind_and_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("alpha planning")
        .await
        .expect("alpha turn should succeed");
    runtime
        .rename_active_session("Alpha workspace")
        .expect("rename should succeed");
    runtime
        .run_demo_turn_in_session("beta", "beta follow-up")
        .await
        .expect("beta turn should succeed");

    let summary = runtime
        .event_digest_summary(0)
        .expect("event digest summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime event digest:"));
    assert!(rendered.contains("- limit: all available"));
    assert!(summary.iter().any(|line| line == "By kind:"));
    assert!(summary.iter().any(|line| line == "By session:"));
    assert!(summary.iter().any(|line| line == "Recent events:"));
    assert!(rendered.contains("SessionRenamed"));
    assert!(rendered.contains("bootstrap"));
    assert!(rendered.contains("alpha:"));
    assert!(rendered.contains("beta:"));
}

#[tokio::test]
async fn event_digest_summary_respects_recent_event_limit() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("alpha planning")
        .await
        .expect("alpha turn should succeed");
    runtime
        .rename_active_session("Alpha workspace")
        .expect("rename should succeed");

    let summary = runtime
        .event_digest_summary(1)
        .expect("event digest summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("- recent events: 1"));
    assert!(rendered.contains("- event kinds: 1"));
    assert!(rendered.contains("- session scopes: 1"));
    assert!(rendered.contains("latest=SessionRenamed"));
    assert!(rendered.contains("Alpha workspace"));
    assert!(!rendered.contains("bootstrap"));
}

#[tokio::test]
async fn transcript_query_summary_renders_hits_and_metadata() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("alpha transcript needle")
        .await
        .expect("alpha turn should succeed");
    runtime
        .run_demo_turn_in_session("beta", "beta transcript needle")
        .await
        .expect("beta turn should succeed");

    let summary = runtime
        .transcript_query_summary(Some("alpha"), "alpha transcript needle", 2)
        .expect("transcript query summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime transcript query: session alpha"));
    assert!(rendered.contains("- query: \"alpha transcript needle\""));
    assert!(!rendered.contains("- matched spans: 0"));
    assert!(!rendered.contains("- returned hits: 0"));
    assert!(rendered.contains("- matched sessions: 1"));
    assert!(rendered.contains("- returned transcript entries: 2"));
    assert!(rendered.contains("- truncated: no"));
    assert!(summary.iter().any(|line| line == "By session:"));
    assert!(summary.iter().any(|line| line == "Hits:"));
    assert!(rendered.contains("alpha: hits=2, entries=2"));
    assert!(rendered.contains("alpha transcript needle"));
    assert!(!rendered.contains("beta transcript needle"));
}

#[tokio::test]
async fn transcript_query_summary_handles_empty_results() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("alpha transcript needle")
        .await
        .expect("alpha turn should succeed");

    let summary = runtime
        .transcript_query_summary(None, "missing transcript needle", 3)
        .expect("transcript query summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime transcript query: all sessions"));
    assert!(rendered.contains("- query: \"missing transcript needle\""));
    assert!(rendered.contains("- matched spans: 0"));
    assert!(rendered.contains("- returned hits: 0"));
    assert!(rendered.contains("- matched sessions: 0"));
    assert!(rendered.contains("- returned transcript entries: 0"));
    assert!(rendered.contains("- truncated: no"));
    assert!(rendered.contains("- limit: 3"));
    assert!(summary.iter().any(|line| line == "By session:"));
    assert!(summary.iter().any(|line| line == "Hits:"));
    assert!(summary.iter().filter(|line| *line == "  - none").count() >= 2);
}

#[tokio::test]
async fn transcript_query_summary_groups_hits_by_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("shared transcript needle")
        .await
        .expect("alpha turn should succeed");
    runtime
        .run_demo_turn_in_session("beta", "shared transcript needle")
        .await
        .expect("beta turn should succeed");

    let summary = runtime
        .transcript_query_summary(None, "shared transcript needle", 10)
        .expect("transcript query summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime transcript query: all sessions"));
    assert!(rendered.contains("- matched sessions: 2"));
    assert!(summary.iter().any(|line| line == "By session:"));
    assert!(rendered.contains("alpha: hits=2, entries=2"));
    assert!(rendered.contains("beta: hits=2, entries=2"));
}

#[tokio::test]
async fn context_recall_summary_renders_recent_entries_and_hits() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("turn should succeed");

    let summary = runtime
        .context_recall_summary("alpha", Some("hello adaptive memory"), 4, 4, 4, true)
        .expect("context recall summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime context recall: session alpha"));
    assert!(rendered.contains("- query: \"hello adaptive memory\""));
    assert!(rendered.contains("- recent entries: 2"));
    assert!(rendered.contains("- transcript matches: 2"));
    assert!(rendered.contains("- transcript hits returned: 2"));
    assert!(rendered.contains("- durable memory hits: 1"));
    assert!(rendered.contains("- memory control omitted items: 0"));
    assert!(rendered.contains("- transcript evidence spans: "));
    assert!(rendered.contains("- omitted items: 0"));
    assert!(rendered.contains("- cross-session memory: allowed"));
    assert!(summary.iter().any(|line| line == "Recent window:"));
    assert!(summary.iter().any(|line| line == "Transcript hits:"));
    assert!(summary.iter().any(|line| line == "Durable memory hits:"));
    assert!(rendered.contains("role=user"));
    assert!(rendered.contains("role=assistant"));
    assert!(rendered.contains("excerpt=\"hello adaptive memory\""));
}

#[tokio::test]
async fn context_recall_summary_handles_missing_query_hits() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("turn should succeed");

    let summary = runtime
        .context_recall_summary("alpha", Some("missing recall string"), 2, 2, 2, false)
        .expect("context recall summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("- query: \"missing recall string\""));
    assert!(rendered.contains("- transcript matches: 0"));
    assert!(rendered.contains("- transcript hits returned: 0"));
    assert!(rendered.contains("- durable memory hits: 0"));
    assert!(rendered.contains("- summary hits: 0"));
    assert!(rendered.contains("- memory control omitted items: 0"));
    assert!(rendered.contains("- transcript evidence spans: 1"));
    assert!(rendered.contains("- omitted items: 0"));
    assert!(rendered.contains("- cross-session memory: disabled"));
    assert!(rendered.contains("hello adaptive memory"));
    assert!(summary.iter().any(|line| line == "Session summary hits:"));
    assert!(summary.iter().filter(|line| *line == "  - none").count() >= 3);
}

#[tokio::test]
async fn context_recall_summary_includes_active_topic_session_state() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("topic route should succeed");

    let summary = runtime
        .context_recall_summary("alpha", Some("hello adaptive memory"), 4, 4, 4, true)
        .expect("context recall summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("- active topic sessions: 1"));
    assert!(summary.iter().any(|line| line == "Active topic sessions:"));
    assert!(rendered.contains("topic-session-bootstrap:alpha"));
    assert!(rendered.contains("label=\"hello adaptive memory\""));
}

#[tokio::test]
async fn intuition_summary_renders_provenance_aware_top_level_output() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("turn should succeed");

    let summary = runtime
        .intuition_summary("alpha", "hello adaptive memory", 4, 4, 4, 2, 2, 2)
        .expect("intuition summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime intuition: session alpha"));
    assert!(rendered.contains("- user intent: \"hello adaptive memory\""));
    assert!(rendered.contains("- recent entries: 2"));
    assert!(rendered.contains("- transcript matches: 2"));
    assert!(rendered.contains("- durable memory hits: 1"));
    assert!(rendered.contains("- transcript evidence spans: "));
    assert!(rendered.contains("- foreground topic sessions: 1"));
    assert!(rendered.contains("- routed topics: 1"));
    assert!(rendered.contains("- returned neuron activations: 1"));
    assert!(rendered.contains("- suggested skills: 1"));
    assert!(rendered.contains("- workflow priors: 1"));
    assert!(
        summary
            .iter()
            .any(|line| line == "Topic activation scores:")
    );
    assert!(summary.iter().any(|line| line == "Neuron activations:"));
    assert!(summary.iter().any(|line| line == "Skill decisions:"));
    assert!(summary.iter().any(|line| line == "Workflow priors:"));
    assert!(rendered.contains("skill=skill-bootstrap:topic-alpha:followup"));
    assert!(rendered.contains("workflow=workflow:memory-review"));
    assert!(rendered.contains("registered=true action=prepare"));
    assert!(rendered.contains("bootstrap intuition synthesized"));
}

#[tokio::test]
async fn provenance_summary_renders_compact_provenance_health_lines() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("topic route should succeed");

    let summary = runtime
        .provenance_summary("alpha")
        .expect("provenance summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime provenance: session alpha"));
    assert!(rendered.contains("- last user intent summary: \"hello adaptive memory\""));
    assert!(rendered.contains("- total topic sessions: 1"));
    assert!(rendered.contains("- active topic sessions with transcript provenance: 1/1"));
    assert!(rendered.contains("- active topic sessions missing transcript provenance: 0"));
    assert!(rendered.contains("- recall transcript evidence spans: "));
    assert!(rendered.contains("- recall ranked items: "));
    assert!(rendered.contains("- recall low-trust ranked items: 0"));
    assert!(rendered.contains("- recall low-recency ranked items: 0"));
    assert!(rendered.contains("- recall memory control omitted items: 0"));
    assert!(rendered.contains("- recall omitted items: 0"));
    assert!(rendered.contains("- intuition transcript evidence spans: "));
    assert!(rendered.contains("- intuition foreground topic sessions: 1"));
}

#[tokio::test]
async fn intelligence_phase2_summary_renders_low_quality_recall_counts() {
    let runtime = RuntimeKernel::new();
    let summary = runtime
        .intelligence_phase2_summary("phase2")
        .await
        .expect("phase2 summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta intelligence phase2: complete"));
    assert!(rendered.contains("- recall ranked items: "));
    assert!(rendered.contains("- recall source count: "));
    assert!(rendered.contains("- recall low-trust ranked items: 0"));
    assert!(rendered.contains("- recall low-recency ranked items: 0"));
    assert!(rendered.contains("- recall memory control omitted items: 0"));
}

#[tokio::test]
async fn intelligence_eval_summary_renders_replay_quality_rollup() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    runtime
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");

    let summary = runtime
        .intelligence_eval_summary("alpha", 2, 6, 6, 6, 2, 2, 2)
        .expect("eval summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime intelligence eval: session alpha"));
    assert!(rendered.contains("- evaluated cases: 2"));
    assert!(rendered.contains("- passed cases: 2"));
    assert!(rendered.contains("- failed cases: 0"));
    assert!(rendered.contains("- total recall ranked items: "));
    assert!(rendered.contains("- total transcript evidence spans: "));
    assert!(rendered.contains("- total active neurons: "));
    assert!(rendered.contains("- total routed topics: "));
    assert!(rendered.contains("- total neuron activations: "));
    assert!(rendered.contains("- total suggested skills: "));
    assert!(rendered.contains("- total workflow priors: "));
    assert!(rendered.contains("- registered workflow priors: "));
    assert!(rendered.contains("- prepared workflow priors: "));
    assert!(rendered.contains("- gated workflow priors: "));
    assert!(rendered.contains("- feedback records: 0"));
    assert!(rendered.contains("- feedback net weight delta: +0.00"));
    assert!(rendered.contains("- calibrated skill targets: 0"));
    assert!(rendered.contains("- calibrated workflow targets: 0"));
    assert!(rendered.contains("- prepared skill decisions: "));
    assert!(rendered.contains("- gated skill decisions: "));
    assert!(rendered.contains("- semantic expectations: "));
    assert!(rendered.contains("- semantic score: 100"));
    assert!(summary.iter().any(|line| line == "Cases:"));
    assert!(rendered.contains("status=pass"));
    assert!(rendered.contains("active_neurons="));
    assert!(rendered.contains("activation_neurons="));
    assert!(rendered.contains("prepared_skills="));
    assert!(rendered.contains("gated_skills="));
    assert!(rendered.contains("registered_workflows="));
    assert!(rendered.contains("prepared_workflows="));
    assert!(rendered.contains("gated_workflows="));
    assert!(rendered.contains("score=100"));
    assert!(rendered.contains("hello adaptive memory"));
    assert!(rendered.contains("rust worker pipeline"));
}

#[tokio::test]
async fn knowledge_graph_dry_run_summary_renders_no_write_report() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_dry_run_summary()
        .expect("kg dry-run summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG dry-run: ready"));
    assert!(rendered.contains("- contract: hepta-intelligence-memory-kg-write-candidate-v0"));
    assert!(rendered.contains("- write candidates: "));
    assert!(rendered.contains("- live write enabled: 0"));
    assert!(rendered.contains("- external side effects enabled: 0"));
    assert!(rendered.contains("- all plans are dry-run: true"));
    assert!(rendered.contains("- no live write enabled: true"));
    assert!(rendered.contains("- no external side effects: true"));
    assert!(rendered.contains("Candidates:"));
}

#[tokio::test]
async fn knowledge_graph_adapter_dry_run_summary_renders_no_external_write_report() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_adapter_dry_run_summary()
        .expect("kg adapter dry-run summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG adapter dry-run: ready"));
    assert!(rendered.contains("- contract: hepta-kg-external-adapter-dry-run-v0"));
    assert!(rendered.contains("- supported adapters: 3"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- external writes enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- no network calls enabled: true"));
    assert!(rendered.contains("- no external writes enabled: true"));
    assert!(rendered.contains("- no live writes enabled: true"));
    assert!(rendered.contains("adapter=graphiti"));
    assert!(rendered.contains("adapter=neo4j"));
    assert!(rendered.contains("adapter=cocoindex"));
}

#[tokio::test]
async fn knowledge_graph_adapter_staging_gate_summary_renders_closed_gate_report() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_adapter_staging_gate_summary()
        .expect("kg adapter staging gate summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG adapter staging gate: ready"));
    assert!(rendered.contains("- contract: hepta-kg-external-adapter-staging-gate-v0"));
    assert!(rendered.contains("- supported adapters: 3"));
    assert!(rendered.contains("- staging ready: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- external writes enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- closed by default: true"));
    assert!(rendered.contains("- operator review required: true"));
    assert!(rendered.contains("- rollback plan required: true"));
    assert!(rendered.contains("- post-write validation required: true"));
    assert!(rendered.contains("gate=HEPTA_KG_GRAPHITI_STAGING"));
    assert!(rendered.contains("gate=HEPTA_KG_NEO4J_STAGING"));
    assert!(rendered.contains("gate=HEPTA_KG_COCOINDEX_STAGING"));
}

#[tokio::test]
async fn knowledge_graph_adapter_client_summary_renders_disabled_client_report() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_adapter_client_summary()
        .expect("kg adapter client summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG adapter clients: ready"));
    assert!(rendered.contains("- contract: hepta-kg-external-adapter-client-v0"));
    assert!(rendered.contains("- supported adapters: 3"));
    assert!(rendered.contains("- client audits: "));
    assert!(rendered.contains("- denied clients: "));
    assert!(rendered.contains("- network calls attempted: 0"));
    assert!(rendered.contains("- external writes attempted: 0"));
    assert!(rendered.contains("- live writes attempted: 0"));
    assert!(rendered.contains("- persisted records: 0"));
    assert!(rendered.contains("- denied by default: true"));
    assert!(rendered.contains("disabled-graphiti-adapter-client"));
    assert!(rendered.contains("disabled-neo4j-adapter-client"));
    assert!(rendered.contains("disabled-cocoindex-adapter-client"));
}

#[tokio::test]
async fn knowledge_graph_adapter_config_env_summary_renders_default_closed_report() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_adapter_config_env_summary()
        .expect("kg adapter config env summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG adapter config env: ready"));
    assert!(rendered.contains("- contract: hepta-kg-external-adapter-config-env-v0"));
    assert!(rendered.contains("- supported adapters: 3"));
    assert!(rendered.contains("- config reads: 3"));
    assert!(rendered.contains("- feature enabled: 0"));
    assert!(rendered.contains("- endpoints configured: 0"));
    assert!(rendered.contains("- credentials configured: 0"));
    assert!(rendered.contains("- network allowlisted: 0"));
    assert!(rendered.contains("- external write allowlisted: 0"));
    assert!(rendered.contains("- live writes requested: 0"));
    assert!(rendered.contains("- credential values captured: 0"));
    assert!(rendered.contains("- network calls attempted: 0"));
    assert!(rendered.contains("- external writes attempted: 0"));
    assert!(rendered.contains("- live writes attempted: 0"));
    assert!(rendered.contains("- configs closed by default: true"));
    assert!(rendered.contains("- no credential values captured: true"));
    assert!(rendered.contains("gate_key=HEPTA_KG_GRAPHITI_STAGING"));
    assert!(rendered.contains("credential_ref_key=HEPTA_KG_NEO4J_CREDENTIAL_REF"));
    assert!(rendered.contains("endpoint_key=HEPTA_KG_COCOINDEX_ENDPOINT"));
}

#[tokio::test]
async fn knowledge_graph_recall_plan_summary_renders_read_only_report() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_recall_plan_summary()
        .expect("kg recall plan summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG recall plan: ready"));
    assert!(rendered.contains("- contract: hepta-kg-read-recall-v0"));
    assert!(rendered.contains("- recall queries: 2"));
    assert!(rendered.contains("- entity matches: "));
    assert!(rendered.contains("- relation neighborhoods: "));
    assert!(rendered.contains("- timeline slices: "));
    assert!(rendered.contains("- evidence paths: "));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- all plans are read-only: true"));
    assert!(rendered.contains("- no external reads enabled: true"));
    assert!(rendered.contains("- no network calls enabled: true"));
    assert!(rendered.contains("- no live writes enabled: true"));
    assert!(rendered.contains("Recall plans:"));
    assert!(rendered.contains("Entity matches:"));
}

#[tokio::test]
async fn knowledge_graph_context_recall_bridge_summary_renders_no_injection_report() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_context_recall_bridge_summary()
        .expect("kg context recall bridge summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG context recall bridge: ready"));
    assert!(rendered.contains("- contract: hepta-intelligence-memory-kg-context-recall-bridge-v0"));
    assert!(rendered.contains("- kg recall contract: hepta-kg-read-recall-v0"));
    assert!(rendered.contains("- context recall items: "));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- model invoked: false"));
    assert!(rendered.contains("- context injection performed: false"));
    assert!(rendered.contains("- all items have KG source: true"));
    assert!(rendered.contains("- transcript provenance preserved: true"));
    assert!(rendered.contains("- no context injection performed: true"));
    assert!(rendered.contains("KG context recall items:"));
}

#[tokio::test]
async fn knowledge_graph_recall_evaluation_summary_renders_quality_gate_report() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_recall_evaluation_summary()
        .expect("kg recall evaluation summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG recall evaluation: ready"));
    assert!(rendered.contains("- contract: hepta-intelligence-memory-kg-recall-evaluation-v0"));
    assert!(rendered.contains("- kg recall contract: hepta-kg-read-recall-v0"));
    assert!(rendered.contains(
        "- kg context bridge contract: hepta-intelligence-memory-kg-context-recall-bridge-v0"
    ));
    assert!(rendered.contains("- failed cases: 0"));
    assert!(rendered.contains("- duplicate source memory ids: 0"));
    assert!(rendered.contains("- score order violations: 0"));
    assert!(rendered.contains("- coverage bp: 10000"));
    assert!(rendered.contains("- precision proxy bp: 10000"));
    assert!(rendered.contains("- score stability bp: 10000"));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- model invoked: false"));
    assert!(rendered.contains("- context injection performed: false"));
    assert!(rendered.contains("- source memory ids unique: true"));
    assert!(rendered.contains("- scores stably ordered: true"));
    assert!(rendered.contains("Evaluation cases:"));
}

#[tokio::test]
async fn knowledge_graph_context_injection_readiness_summary_renders_blocking_gate() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_context_injection_readiness_summary()
        .expect("kg context injection readiness summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG context injection readiness: blocked"));
    assert!(
        rendered
            .contains("- contract: hepta-intelligence-memory-kg-context-injection-readiness-v0")
    );
    assert!(rendered.contains(
        "- kg recall evaluation contract: hepta-intelligence-memory-kg-recall-evaluation-v0"
    ));
    assert!(rendered.contains("- failed cases: 0"));
    assert!(rendered.contains("- coverage bp: 10000"));
    assert!(rendered.contains("- quality threshold bp: 9000"));
    assert!(rendered.contains("- quality gate ready: true"));
    assert!(rendered.contains("- operator approved: false"));
    assert!(rendered.contains("- shadow rank enabled: false"));
    assert!(rendered.contains("- rollback plan ready: false"));
    assert!(rendered.contains("- kill switch ready: false"));
    assert!(rendered.contains("- context injection allowed: false"));
    assert!(rendered.contains("- context injection performed: false"));
    assert!(rendered.contains("- prompt preview rendered: false"));
    assert!(rendered.contains("- model invoked: false"));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- recall evaluation ready: true"));
    assert!(rendered.contains("- activation blocked without operator approval: true"));
    assert!(rendered.contains("- no context injection performed: true"));
    assert!(rendered.contains("Readiness blockers:"));
    assert!(rendered.contains("MissingOperatorApproval"));
    assert!(rendered.contains("ShadowRankNotEnabled"));
    assert!(rendered.contains("InjectionDisabledByDefault"));
}

#[tokio::test]
async fn knowledge_graph_shadow_rank_summary_renders_observation_report() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_shadow_rank_summary()
        .expect("kg shadow-rank summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG shadow rank: ready"));
    assert!(rendered.contains("- contract: hepta-intelligence-memory-kg-shadow-rank-v0"));
    assert!(rendered.contains(
        "- kg context injection readiness contract: hepta-intelligence-memory-kg-context-injection-readiness-v0"
    ));
    assert!(rendered.contains("- injection readiness status: blocked"));
    assert!(rendered.contains("- would enter prompt context: 0"));
    assert!(rendered.contains("- prompt preview rendered: false"));
    assert!(rendered.contains("- model invoked: false"));
    assert!(rendered.contains("- context injection performed: false"));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- injection readiness blocked: true"));
    assert!(rendered.contains("- all items observed only: true"));
    assert!(rendered.contains("- no items enter prompt context: true"));
    assert!(rendered.contains("- scores stably ordered: true"));
    assert!(rendered.contains("- no context injection performed: true"));
    assert!(rendered.contains("Shadow-rank items:"));
    assert!(rendered.contains("observed_only=true"));
    assert!(rendered.contains("enters_prompt=false"));
}

#[tokio::test]
async fn knowledge_graph_shadow_rank_comparison_summary_renders_local_baselines() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_shadow_rank_comparison_summary()
        .expect("kg shadow-rank comparison summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG shadow-rank comparison: ready"));
    assert!(
        rendered.contains("- contract: hepta-intelligence-memory-kg-shadow-rank-comparison-v0")
    );
    assert!(
        rendered.contains("- kg shadow rank contract: hepta-intelligence-memory-kg-shadow-rank-v0")
    );
    assert!(rendered.contains("- transcript baseline items:"));
    assert!(rendered.contains("- durable memory baseline items:"));
    assert!(rendered.contains("- comparison cases:"));
    assert!(rendered.contains("- prompt preview rendered: false"));
    assert!(rendered.contains("- model invoked: false"));
    assert!(rendered.contains("- context injection performed: false"));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- shadow rank ready: true"));
    assert!(rendered.contains("- baseline items nonzero: true"));
    assert!(rendered.contains("- comparison cases nonzero: true"));
    assert!(rendered.contains("- no kg items enter prompt context: true"));
    assert!(rendered.contains("- no baseline items enter prompt context: true"));
    assert!(rendered.contains("- no context injection performed: true"));
    assert!(rendered.contains("Comparison cases:"));
    assert!(rendered.contains("baseline=Transcript"));
    assert!(rendered.contains("baseline=DurableMemory"));
    assert!(rendered.contains("enters_prompt=false"));
    assert!(rendered.contains("baseline_enters_prompt=false"));
}

#[tokio::test]
async fn knowledge_graph_shadow_rank_drift_summary_renders_stable_regression_gate() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_shadow_rank_drift_summary()
        .expect("kg shadow-rank drift summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG shadow-rank drift: ready"));
    assert!(rendered.contains("- verdict: stable"));
    assert!(rendered.contains("- contract: hepta-intelligence-memory-kg-shadow-rank-drift-v0"));
    assert!(rendered.contains(
        "- kg shadow-rank comparison contract: hepta-intelligence-memory-kg-shadow-rank-comparison-v0"
    ));
    assert!(
        rendered.contains("- kg shadow rank contract: hepta-intelligence-memory-kg-shadow-rank-v0")
    );
    assert!(rendered.contains("- expected drift cases:"));
    assert!(rendered.contains("- stable cases:"));
    assert!(rendered.contains("- drifted cases: 0"));
    assert!(rendered.contains("- transcript cases:"));
    assert!(rendered.contains("- durable memory cases:"));
    assert!(rendered.contains("- prompt preview rendered: false"));
    assert!(rendered.contains("- model invoked: false"));
    assert!(rendered.contains("- context injection performed: false"));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- comparison ready: true"));
    assert!(rendered.contains("- top-n coverage complete: true"));
    assert!(rendered.contains("- baseline kind coverage stable: true"));
    assert!(rendered.contains("- rank order stable: true"));
    assert!(rendered.contains("- score delta within thresholds: true"));
    assert!(rendered.contains("- prompt flags stable: true"));
    assert!(rendered.contains("- no context injection performed: true"));
    assert!(rendered.contains("Drift cases:"));
    assert!(rendered.contains("stable=true"));
}

#[tokio::test]
async fn knowledge_graph_prompt_preview_approval_packet_summary_renders_blocked_packet() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_prompt_preview_approval_packet_summary()
        .expect("kg prompt-preview approval packet summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG prompt-preview approval packet: blocked"));
    assert!(
        rendered
            .contains("- contract: hepta-intelligence-memory-kg-prompt-preview-approval-packet-v0")
    );
    assert!(rendered.contains(
        "- kg shadow-rank drift contract: hepta-intelligence-memory-kg-shadow-rank-drift-v0"
    ));
    assert!(
        rendered.contains("- approval packet mode: draft_redacted_refs_only_no_prompt_preview")
    );
    assert!(rendered.contains("- drifted cases: 0"));
    assert!(rendered.contains("- operator approval recorded: false"));
    assert!(rendered.contains("- rollback plan ready: false"));
    assert!(rendered.contains("- kill switch ready: false"));
    assert!(rendered.contains("- approval packet accepted: false"));
    assert!(rendered.contains("- prompt preview allowed: false"));
    assert!(rendered.contains("- prompt preview rendered: false"));
    assert!(rendered.contains("- prompt payload materialized: false"));
    assert!(rendered.contains("- context injection allowed: false"));
    assert!(rendered.contains("- context injection performed: false"));
    assert!(rendered.contains("- model invoked: false"));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- drift gate stable: true"));
    assert!(rendered.contains("- approval items cover drift cases: true"));
    assert!(rendered.contains("- redacted refs present: true"));
    assert!(rendered.contains("- operator approval required: true"));
    assert!(rendered.contains("- prompt preview disabled by default: true"));
    assert!(rendered.contains("- prompt payload not materialized: true"));
    assert!(rendered.contains("- context injection disabled by default: true"));
    assert!(rendered.contains("- no context injection performed: true"));
    assert!(rendered.contains("- no model invoked: true"));
    assert!(rendered.contains("- no external reads enabled: true"));
    assert!(rendered.contains("- no network calls enabled: true"));
    assert!(rendered.contains("- no live writes enabled: true"));
    assert!(rendered.contains("Approval packet blockers:"));
    assert!(rendered.contains("MissingOperatorApproval"));
    assert!(rendered.contains("PromptPreviewDisabledByDefault"));
    assert!(rendered.contains("Approval packet items:"));
    assert!(rendered.contains("prompt_preview_included=false"));
    assert!(rendered.contains("injection_allowed=false"));
}

#[tokio::test]
async fn knowledge_graph_prompt_preview_operator_evidence_summary_renders_blocking_gate() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_prompt_preview_operator_evidence_summary()
        .expect("kg prompt-preview operator evidence summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG prompt-preview operator evidence: blocked"));
    assert!(
        rendered.contains(
            "- contract: hepta-intelligence-memory-kg-prompt-preview-operator-evidence-v0"
        )
    );
    assert!(rendered.contains(
        "- approval packet contract: hepta-intelligence-memory-kg-prompt-preview-approval-packet-v0"
    ));
    assert!(rendered.contains("- approval packet status: blocked"));
    assert!(
        rendered.contains(
            "- evidence gate mode: operator_evidence_requirements_only_no_prompt_preview"
        )
    );
    assert!(rendered.contains("- operator approval evidence present: false"));
    assert!(rendered.contains("- rollback plan evidence present: false"));
    assert!(rendered.contains("- kill switch evidence present: false"));
    assert!(rendered.contains("- reviewer identity present: false"));
    assert!(rendered.contains("- reviewer identity redacted: true"));
    assert!(rendered.contains("- approval timestamp present: false"));
    assert!(rendered.contains("- signed approval digest present: false"));
    assert!(rendered.contains("- bounded preview scope present: false"));
    assert!(rendered.contains("- required evidence: 7"));
    assert!(rendered.contains("- missing evidence: 7"));
    assert!(rendered.contains("- prompt preview allowed: false"));
    assert!(rendered.contains("- prompt preview rendered: false"));
    assert!(rendered.contains("- prompt payload materialized: false"));
    assert!(rendered.contains("- context injection allowed: false"));
    assert!(rendered.contains("- context injection performed: false"));
    assert!(rendered.contains("- model invoked: false"));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- approval packet checks ready: true"));
    assert!(rendered.contains("- approval packet not accepted: true"));
    assert!(rendered.contains("- evidence requirements all blocking: true"));
    assert!(rendered.contains("- operator approval evidence required: true"));
    assert!(rendered.contains("- signed approval digest required: true"));
    assert!(rendered.contains("- bounded preview scope required: true"));
    assert!(rendered.contains("- prompt preview disabled: true"));
    assert!(rendered.contains("- prompt payload not materialized: true"));
    assert!(rendered.contains("- context injection disabled: true"));
    assert!(rendered.contains("- no model invoked: true"));
    assert!(rendered.contains("- no context injection performed: true"));
    assert!(rendered.contains("- no external reads enabled: true"));
    assert!(rendered.contains("- no network calls enabled: true"));
    assert!(rendered.contains("- no live writes enabled: true"));
    assert!(rendered.contains("Operator evidence blockers:"));
    assert!(rendered.contains("ApprovalPacketNotAccepted"));
    assert!(rendered.contains("MissingOperatorApprovalEvidence"));
    assert!(rendered.contains("MissingSignedApprovalDigest"));
    assert!(rendered.contains("Operator evidence requirements:"));
    assert!(rendered.contains("name=operator_approval_record present=false"));
    assert!(rendered.contains("blocks_prompt_preview=true"));
}

#[tokio::test]
async fn knowledge_graph_prompt_preview_redaction_diff_summary_renders_redacted_only_gate() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_prompt_preview_redaction_diff_summary()
        .expect("kg prompt-preview redaction diff summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG prompt-preview redaction diff: blocked"));
    assert!(
        rendered
            .contains("- contract: hepta-intelligence-memory-kg-prompt-preview-redaction-diff-v0")
    );
    assert!(rendered.contains(
        "- operator evidence contract: hepta-intelligence-memory-kg-prompt-preview-operator-evidence-v0"
    ));
    assert!(rendered.contains("- operator evidence status: blocked"));
    assert!(
        rendered
            .contains("- redaction diff mode: redacted_requirement_refs_only_no_prompt_or_payload")
    );
    assert!(rendered.contains("- required evidence: 7"));
    assert!(rendered.contains("- missing evidence: 7"));
    assert!(rendered.contains("- diff items: 7"));
    assert!(rendered.contains("- redacted refs: 7"));
    assert!(rendered.contains("- raw prompt diffs: 0"));
    assert!(rendered.contains("- prompt text included: 0"));
    assert!(rendered.contains("- payload text included: 0"));
    assert!(rendered.contains("- redacted diff reported: true"));
    assert!(rendered.contains("- prompt preview allowed: false"));
    assert!(rendered.contains("- prompt preview rendered: false"));
    assert!(rendered.contains("- prompt payload materialized: false"));
    assert!(rendered.contains("- context injection allowed: false"));
    assert!(rendered.contains("- context injection performed: false"));
    assert!(rendered.contains("- model invoked: false"));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- operator evidence checks ready: true"));
    assert!(rendered.contains("- operator evidence missing requirements: true"));
    assert!(rendered.contains("- redacted diff items cover requirements: true"));
    assert!(rendered.contains("- raw prompt diff suppressed: true"));
    assert!(rendered.contains("- prompt text excluded: true"));
    assert!(rendered.contains("- payload text excluded: true"));
    assert!(rendered.contains("- prompt preview disabled: true"));
    assert!(rendered.contains("- prompt payload not materialized: true"));
    assert!(rendered.contains("- context injection disabled: true"));
    assert!(rendered.contains("- no model invoked: true"));
    assert!(rendered.contains("- no context injection performed: true"));
    assert!(rendered.contains("- no external reads enabled: true"));
    assert!(rendered.contains("- no network calls enabled: true"));
    assert!(rendered.contains("- no live writes enabled: true"));
    assert!(rendered.contains("Redaction diff blockers:"));
    assert!(rendered.contains("OperatorEvidenceIncomplete"));
    assert!(rendered.contains("RawPromptDiffSuppressed"));
    assert!(rendered.contains("Redaction diff items:"));
    assert!(rendered.contains("requirement=operator_approval_record"));
    assert!(rendered.contains("raw_before=false"));
    assert!(rendered.contains("raw_after=false"));
    assert!(rendered.contains("prompt_text=false"));
    assert!(rendered.contains("payload_text=false"));
}

#[tokio::test]
async fn knowledge_graph_prompt_preview_rollback_kill_switch_summary_renders_blocked_gate() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_prompt_preview_rollback_kill_switch_summary()
        .expect("kg prompt-preview rollback/kill-switch summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG prompt-preview rollback/kill-switch: blocked"));
    assert!(rendered.contains(
        "- contract: hepta-intelligence-memory-kg-prompt-preview-rollback-kill-switch-v0"
    ));
    assert!(rendered.contains(
        "- redaction diff contract: hepta-intelligence-memory-kg-prompt-preview-redaction-diff-v0"
    ));
    assert!(rendered.contains("- redaction diff status: blocked"));
    assert!(
        rendered
            .contains("- redaction diff mode: redacted_requirement_refs_only_no_prompt_or_payload")
    );
    assert!(rendered.contains("- required evidence: 7"));
    assert!(rendered.contains("- missing evidence: 7"));
    assert!(rendered.contains("- required controls: 4"));
    assert!(rendered.contains("- missing controls: 4"));
    assert!(rendered.contains("- rollback controls: 2"));
    assert!(rendered.contains("- kill switch controls: 2"));
    assert!(rendered.contains("- rollback plan ready: false"));
    assert!(rendered.contains("- rollback exercise ready: false"));
    assert!(rendered.contains("- kill switch ready: false"));
    assert!(rendered.contains("- kill switch dry run ready: false"));
    assert!(rendered.contains("- raw prompt diffs: 0"));
    assert!(rendered.contains("- prompt text included: 0"));
    assert!(rendered.contains("- payload text included: 0"));
    assert!(rendered.contains("- prompt preview allowed: false"));
    assert!(rendered.contains("- prompt preview rendered: false"));
    assert!(rendered.contains("- prompt payload materialized: false"));
    assert!(rendered.contains("- context injection allowed: false"));
    assert!(rendered.contains("- context injection performed: false"));
    assert!(rendered.contains("- model invoked: false"));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- redaction diff checks ready: true"));
    assert!(rendered.contains("- redaction diff blocked: true"));
    assert!(rendered.contains("- only redacted refs reported: true"));
    assert!(rendered.contains("- rollback controls nonzero: true"));
    assert!(rendered.contains("- kill switch controls nonzero: true"));
    assert!(rendered.contains("- controls all missing and blocking: true"));
    assert!(rendered.contains("- rollback plan required: true"));
    assert!(rendered.contains("- rollback exercise required: true"));
    assert!(rendered.contains("- kill switch required: true"));
    assert!(rendered.contains("- kill switch dry run required: true"));
    assert!(rendered.contains("- prompt preview disabled: true"));
    assert!(rendered.contains("- prompt payload not materialized: true"));
    assert!(rendered.contains("- context injection disabled: true"));
    assert!(rendered.contains("- no model invoked: true"));
    assert!(rendered.contains("- no context injection performed: true"));
    assert!(rendered.contains("- no external reads enabled: true"));
    assert!(rendered.contains("- no network calls enabled: true"));
    assert!(rendered.contains("- no live writes enabled: true"));
    assert!(rendered.contains("Rollback/kill-switch blockers:"));
    assert!(rendered.contains("RollbackPlanEvidenceMissing"));
    assert!(rendered.contains("KillSwitchEvidenceMissing"));
    assert!(rendered.contains("Rollback/kill-switch controls:"));
    assert!(rendered.contains("control=rollback_plan_record"));
    assert!(rendered.contains("kind=rollback"));
    assert!(rendered.contains("control=kill_switch_record"));
    assert!(rendered.contains("kind=kill_switch"));
    assert!(rendered.contains("present=false"));
    assert!(rendered.contains("blocks_prompt_preview=true"));
    assert!(rendered.contains("allows_context_injection=false"));
}

#[tokio::test]
async fn knowledge_graph_prompt_preview_context_handoff_summary_renders_blocked_gate() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_prompt_preview_context_handoff_summary()
        .expect("kg prompt-preview context-handoff summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG prompt-preview context handoff: blocked"));
    assert!(
        rendered
            .contains("- contract: hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0")
    );
    assert!(rendered.contains(
        "- safety gate contract: hepta-intelligence-memory-kg-prompt-preview-rollback-kill-switch-v0"
    ));
    assert!(rendered.contains("- safety gate status: blocked"));
    assert!(rendered.contains(
        "- redaction diff contract: hepta-intelligence-memory-kg-prompt-preview-redaction-diff-v0"
    ));
    assert!(rendered.contains("- required evidence: 7"));
    assert!(rendered.contains("- missing evidence: 7"));
    assert!(rendered.contains("- required controls: 4"));
    assert!(rendered.contains("- missing controls: 4"));
    assert!(rendered.contains("- handoff requirements: 6"));
    assert!(rendered.contains("- missing handoff requirements: 6"));
    assert!(rendered.contains("- redacted refs: 7"));
    assert!(rendered.contains("- raw prompt diffs: 0"));
    assert!(rendered.contains("- prompt text included: 0"));
    assert!(rendered.contains("- payload text included: 0"));
    assert!(rendered.contains("- redacted diff review present: false"));
    assert!(rendered.contains("- context handoff approval present: false"));
    assert!(rendered.contains("- prompt preview allowed: false"));
    assert!(rendered.contains("- prompt preview rendered: false"));
    assert!(rendered.contains("- prompt payload materialized: false"));
    assert!(rendered.contains("- context injection allowed: false"));
    assert!(rendered.contains("- context injection performed: false"));
    assert!(rendered.contains("- model invoked: false"));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- safety gate contract linked: true"));
    assert!(rendered.contains("- safety gate checks ready: true"));
    assert!(rendered.contains("- safety gate blocked: true"));
    assert!(rendered.contains("- operator evidence incomplete: true"));
    assert!(rendered.contains("- safety controls incomplete: true"));
    assert!(rendered.contains("- handoff requirements nonzero: true"));
    assert!(rendered.contains("- handoff requirements all missing and blocking: true"));
    assert!(rendered.contains("- redacted refs only: true"));
    assert!(rendered.contains("- redacted diff review required: true"));
    assert!(rendered.contains("- context handoff approval required: true"));
    assert!(rendered.contains("- prompt preview disabled: true"));
    assert!(rendered.contains("- prompt payload not materialized: true"));
    assert!(rendered.contains("- context injection disabled: true"));
    assert!(rendered.contains("- no model invoked: true"));
    assert!(rendered.contains("- no context injection performed: true"));
    assert!(rendered.contains("- no external reads enabled: true"));
    assert!(rendered.contains("- no network calls enabled: true"));
    assert!(rendered.contains("- no live writes enabled: true"));
    assert!(rendered.contains("Context-handoff blockers:"));
    assert!(rendered.contains("OperatorEvidenceIncomplete"));
    assert!(rendered.contains("SafetyControlsIncomplete"));
    assert!(rendered.contains("RedactedDiffReviewMissing"));
    assert!(rendered.contains("ContextHandoffApprovalMissing"));
    assert!(rendered.contains("Context-handoff requirements:"));
    assert!(rendered.contains("requirement=operator_evidence_packet"));
    assert!(rendered.contains("kind=operator_evidence"));
    assert!(rendered.contains("present=false"));
    assert!(rendered.contains("blocks_context_injection=true"));
}

#[tokio::test]
async fn knowledge_graph_prompt_preview_preflight_summary_renders_blocked_ci_gate() {
    let runtime = RuntimeKernel::new();

    let summary = runtime
        .knowledge_graph_prompt_preview_preflight_summary()
        .expect("kg prompt-preview preflight summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Hepta KG prompt-preview preflight: blocked"));
    assert!(
        rendered.contains("- contract: hepta-intelligence-memory-kg-prompt-preview-preflight-v0")
    );
    assert!(rendered.contains(
        "- context handoff contract: hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0"
    ));
    assert!(rendered.contains("- context handoff status: blocked"));
    assert!(rendered.contains("- source gates: 5"));
    assert!(rendered.contains("- ready source gates: 5"));
    assert!(rendered.contains("- blocked source gates: 5"));
    assert!(rendered.contains("- report-only source gates: 5"));
    assert!(rendered.contains("- required operator evidence: 7"));
    assert!(rendered.contains("- missing operator evidence: 7"));
    assert!(rendered.contains("- required safety controls: 4"));
    assert!(rendered.contains("- missing safety controls: 4"));
    assert!(rendered.contains("- required handoff requirements: 6"));
    assert!(rendered.contains("- missing handoff requirements: 6"));
    assert!(rendered.contains("- missing final review approval: 2"));
    assert!(rendered.contains("- required total preflight requirements: 19"));
    assert!(rendered.contains("- missing total preflight requirements: 19"));
    assert!(rendered.contains("- raw prompt diffs: 0"));
    assert!(rendered.contains("- prompt text included: 0"));
    assert!(rendered.contains("- payload text included: 0"));
    assert!(rendered.contains("- redacted diff review present: false"));
    assert!(rendered.contains("- context handoff approval present: false"));
    assert!(rendered.contains("- prompt preview allowed: false"));
    assert!(rendered.contains("- prompt preview rendered: false"));
    assert!(rendered.contains("- prompt payload materialized: false"));
    assert!(rendered.contains("- context injection allowed: false"));
    assert!(rendered.contains("- context injection performed: false"));
    assert!(rendered.contains("- model invoked: false"));
    assert!(rendered.contains("- CI promotion allowed: false"));
    assert!(rendered.contains("- preflight execution performed: false"));
    assert!(rendered.contains("- external reads enabled: 0"));
    assert!(rendered.contains("- network calls enabled: 0"));
    assert!(rendered.contains("- live writes enabled: 0"));
    assert!(rendered.contains("- source gates all linked: true"));
    assert!(rendered.contains("- source gates all checks ready: true"));
    assert!(rendered.contains("- source gates all blocked: true"));
    assert!(rendered.contains("- source gates all report-only: true"));
    assert!(rendered.contains("- context handoff contract linked: true"));
    assert!(rendered.contains("- context handoff checks ready: true"));
    assert!(rendered.contains("- context handoff blocked: true"));
    assert!(rendered.contains("- operator evidence incomplete: true"));
    assert!(rendered.contains("- safety controls incomplete: true"));
    assert!(rendered.contains("- handoff requirements incomplete: true"));
    assert!(rendered.contains("- redacted diff review required: true"));
    assert!(rendered.contains("- context handoff approval required: true"));
    assert!(rendered.contains("- prompt preview disabled: true"));
    assert!(rendered.contains("- prompt payload not materialized: true"));
    assert!(rendered.contains("- context injection disabled: true"));
    assert!(rendered.contains("- no model invoked: true"));
    assert!(rendered.contains("- no context injection performed: true"));
    assert!(rendered.contains("- no external reads enabled: true"));
    assert!(rendered.contains("- no network calls enabled: true"));
    assert!(rendered.contains("- no live writes enabled: true"));
    assert!(rendered.contains("- CI promotion disabled: true"));
    assert!(rendered.contains("- no preflight execution performed: true"));
    assert!(rendered.contains("Prompt-preview preflight blockers:"));
    assert!(rendered.contains("PromptPreviewGateChainBlocked"));
    assert!(rendered.contains("CiPromotionDisabled"));
    assert!(rendered.contains("Prompt-preview preflight source gates:"));
    assert!(rendered.contains("gate=approval_packet"));
    assert!(rendered.contains("gate=context_handoff"));
    assert!(rendered.contains("status=blocked"));
    assert!(rendered.contains("checks_ready=true"));
    assert!(rendered.contains("blocks_prompt_preview=true"));
    assert!(rendered.contains("blocks_context_injection=true"));
    assert!(rendered.contains("report_only=true"));
}

#[tokio::test]
async fn intuition_calibration_summary_renders_feedback_rollup() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("topic route should succeed");
    let overview = runtime
        .intuition_overview("alpha", "hello adaptive memory", 4, 4, 4, 1, 1, 1)
        .expect("intuition overview should succeed");
    let skill = overview.bundle.skill_decisions[0].clone();
    let workflow = overview.bundle.workflow_priors[0].clone();
    runtime
        .record_intuition_feedback(
            "alpha",
            "hello adaptive memory",
            hepta_core::IntuitionFeedbackOutcome::Accepted,
            Some(skill.skill_id.as_str()),
            Some(workflow.workflow_id.as_str()),
            skill.source_topic_ids,
            skill.source_neuron_ids,
            Some("accepted follow-up"),
        )
        .expect("feedback should record");

    let summary = runtime
        .intuition_calibration_summary("alpha")
        .expect("calibration summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Intuition calibration: session alpha"));
    assert!(rendered.contains("- feedback records: 1"));
    assert!(rendered.contains("- positive feedback: 1"));
    assert!(rendered.contains("- net weight delta: +0.12"));
    assert!(rendered.contains("Outcome counts:"));
    assert!(rendered.contains("  - accepted=1"));
    assert!(rendered.contains("Skill targets:"));
    assert!(rendered.contains("Workflow targets:"));
    assert!(rendered.contains("Recent feedback:"));
    assert!(rendered.contains("skill=skill-bootstrap:topic-alpha:followup"));
    assert!(rendered.contains("workflow=workflow:memory-review"));
    assert!(rendered.contains("outcome=accepted"));
}

#[tokio::test]
async fn neuron_lifecycle_summary_renders_health_findings() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("topic route should succeed");
    runtime
        .compress_active_topics_to_neurons("alpha", 2)
        .expect("compression should succeed");

    let summary = runtime
        .neuron_lifecycle_summary("alpha")
        .expect("lifecycle summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Neuron lifecycle: session alpha"));
    assert!(rendered.contains("- healthy: true"));
    assert!(rendered.contains("- stored neurons: 1"));
    assert!(rendered.contains("- neurons with transcript provenance: 1"));
    assert!(rendered.contains("- neurons with evidence digest: 1"));
    assert!(rendered.contains("- active topics without neurons: none"));
    assert!(summary.iter().any(|line| line == "Findings:"));
    assert!(rendered.contains("  - none"));
}

#[tokio::test]
async fn neuron_activation_summary_renders_direct_activation_metadata() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .rename_active_session("Alpha workspace")
        .expect("rename should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("turn should succeed");

    let summary = runtime
        .neuron_activation_summary("alpha", Some("hello adaptive memory"), 4, 4, 4, 2)
        .expect("neuron activation summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime neuron activation: session alpha"));
    assert!(rendered.contains("- query: \"hello adaptive memory\""));
    assert!(rendered.contains("- recent entries: 2"));
    assert!(rendered.contains("- transcript matches: 2"));
    assert!(rendered.contains("- durable memory hits: 1"));
    assert!(rendered.contains("- active topic sessions: 1"));
    assert!(rendered.contains("- routed topics: 1"));
    assert!(rendered.contains("- returned activations: 1"));
    assert!(summary.iter().any(|line| line == "Activations:"));
    assert!(rendered.contains("neuron=neuron-alpha"));
    assert!(rendered.contains("topic=topic-alpha"));
    assert!(rendered.contains("direct=0.90"));
    assert!(rendered.contains("propagated=0.00"));
    assert!(rendered.contains("inhibited=0.00"));
    assert!(rendered.contains("final=0.90"));
    assert!(rendered.contains("transcript_spans="));
    assert!(rendered.contains("via routed topic session"));
}

#[tokio::test]
async fn topic_routing_summary_renders_bootstrap_topic_decision() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .rename_active_session("Alpha workspace")
        .expect("rename should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("turn should succeed");

    let summary = runtime
        .topic_routing_summary("alpha", Some("hello adaptive memory"), 4, 4, 4, 2)
        .expect("topic routing summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime topic routing: session alpha"));
    assert!(rendered.contains("- query: \"hello adaptive memory\""));
    assert!(rendered.contains("- recent entries: 2"));
    assert!(rendered.contains("- transcript matches: 2"));
    assert!(rendered.contains("- durable memory hits: 1"));
    assert!(rendered.contains("- transcript evidence spans: "));
    assert!(rendered.contains("- active topic sessions: 1"));
    assert!(rendered.contains("- created topic sessions: 0"));
    assert!(rendered.contains("- multi-topic: no"));
    assert!(rendered.contains("- primary topic: topic-alpha"));
    assert!(summary.iter().any(|line| line == "Activation scores:"));
    assert!(summary.iter().any(|line| line == "Shift event:"));
    assert!(rendered.contains("topic=topic-alpha"));
    assert!(rendered.contains("label=\"hello adaptive memory\""));
    assert!(rendered.contains("matched_terms=hello|adaptive|memory"));
    assert!(rendered.contains("kind=Stayed"));
    assert!(rendered.contains("bootstrap topic routing anchored session"));
}

#[tokio::test]
async fn topic_session_summary_renders_bootstrap_topic_session_state() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("route topics should succeed");

    let summary = runtime
        .topic_session_summary("alpha")
        .expect("topic session summary should succeed");
    let rendered = summary.join("\n");

    assert!(rendered.contains("Runtime topic sessions: session alpha"));
    assert!(rendered.contains("- topic sessions: 1"));
    assert!(summary.iter().any(|line| line == "Sessions:"));
    assert!(rendered.contains("id=topic-session-bootstrap:alpha"));
    assert!(rendered.contains("topic=topic-alpha"));
    assert!(rendered.contains("label=\"hello adaptive memory\""));
    assert!(rendered.contains("status=Active"));
    assert!(rendered.contains("linked_surface_sessions=1"));
}

#[tokio::test]
async fn topic_routing_and_session_summaries_surface_shift_and_revive_state() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("first route should succeed");
    runtime
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");

    let shifted = runtime
        .topic_routing_summary("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("shift summary should succeed")
        .join("\n");
    assert!(shifted.contains("- created topic sessions: 1"));
    assert!(shifted.contains("- revived topic sessions: 0"));
    assert!(shifted.contains("kind=Shifted"));
    assert!(shifted.contains("topic-alpha-rust-worker-pipeline"));

    let revived = runtime
        .topic_routing_summary("alpha", Some("hello memory"), 6, 6, 6, 1)
        .expect("revive summary should succeed")
        .join("\n");
    assert!(revived.contains("- created topic sessions: 0"));
    assert!(revived.contains("- revived topic sessions: 1"));
    assert!(revived.contains("kind=Revived"));
    assert!(revived.contains("topic-alpha"));

    let session_summary = runtime
        .topic_session_summary("alpha")
        .expect("topic session summary should succeed")
        .join("\n");
    assert!(session_summary.contains("- topic sessions: 2"));
    assert!(session_summary.contains("id=topic-session-bootstrap:alpha topic=topic-alpha label=\"hello adaptive memory\" status=Active"));
    assert!(session_summary.contains("id=topic-session-bootstrap:alpha:rust-worker-pipeline topic=topic-alpha-rust-worker-pipeline label=\"rust worker pipeline\" status=Dormant"));
}

#[tokio::test]
async fn topic_routing_and_neuron_summaries_surface_multi_topic_coactivation() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("first route should succeed");
    runtime
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");
    runtime
        .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("second route should succeed");

    let routing_summary = runtime
        .topic_routing_summary(
            "alpha",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("routing summary should succeed")
        .join("\n");
    assert!(routing_summary.contains("- active topic sessions: 2"));
    assert!(routing_summary.contains("- created topic sessions: 0"));
    assert!(routing_summary.contains("- revived topic sessions: 1"));
    assert!(routing_summary.contains("- multi-topic: yes"));
    assert!(routing_summary.contains("kind=CoActivated"));
    assert!(routing_summary.contains("topic=topic-alpha"));
    assert!(routing_summary.contains("topic=topic-alpha-rust-worker-pipeline"));

    let neuron_summary = runtime
        .neuron_activation_summary(
            "alpha",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            3,
        )
        .expect("neuron summary should succeed")
        .join("\n");
    assert!(neuron_summary.contains("- active topic sessions: 2"));
    assert!(neuron_summary.contains("- routed topics: 2"));
    assert!(neuron_summary.contains("- returned activations: 2"));
    assert!(neuron_summary.contains("neuron=neuron-alpha"));
    assert!(neuron_summary.contains("neuron=neuron-alpha-rust-worker-pipeline"));
    assert!(neuron_summary.contains("links=1"));
    assert!(neuron_summary.contains("inhibited=0.00"));
    assert!(!neuron_summary.contains("propagated=0.00"));

    let session_summary = runtime
        .topic_session_summary("alpha")
        .expect("topic session summary should succeed")
        .join("\n");
    assert!(session_summary.contains("status=Active"));
    assert!(session_summary.contains("id=topic-session-bootstrap:alpha topic=topic-alpha label=\"hello adaptive memory\" status=Active"));
    assert!(session_summary.contains("id=topic-session-bootstrap:alpha:rust-worker-pipeline topic=topic-alpha-rust-worker-pipeline label=\"rust worker pipeline\" status=Active"));
}

#[tokio::test]
async fn topic_routing_summary_surfaces_implicit_multi_topic_detection_without_delimiters() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("first route should succeed");
    runtime
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");
    runtime
        .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("second route should succeed");

    let summary = runtime
        .topic_routing_summary(
            "alpha",
            Some("continue hello adaptive memory rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("routing summary should succeed")
        .join("\n");

    assert!(summary.contains("- active topic sessions: 2"));
    assert!(summary.contains("- created topic sessions: 0"));
    assert!(summary.contains("- revived topic sessions: 1"));
    assert!(summary.contains("- multi-topic: yes"));
    assert!(summary.contains("kind=CoActivated"));
    assert!(summary.contains("implicitly kept") || summary.contains("implicitly revived"));
}

#[tokio::test]
async fn topic_routing_summary_surfaces_semantic_mixed_turn_detection() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("first route should succeed");
    runtime
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");
    runtime
        .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("second route should succeed");

    let summary = runtime
        .topic_routing_summary(
            "alpha",
            Some("continue adaptive recall while checking executor flow"),
            8,
            8,
            8,
            2,
        )
        .expect("routing summary should succeed")
        .join("\n");

    assert!(summary.contains("- active topic sessions: 2"));
    assert!(summary.contains("- created topic sessions: 0"));
    assert!(summary.contains("- revived topic sessions: 1"));
    assert!(summary.contains("- multi-topic: yes"));
    assert!(summary.contains("kind=CoActivated"));
    assert!(summary.contains("semantic"));
    assert!(summary.contains("hello adaptive memory"));
    assert!(summary.contains("rust worker pipeline"));
}

#[tokio::test]
async fn topic_routing_and_session_summaries_surface_merge_and_split_state() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("first route should succeed");
    runtime
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");
    runtime
        .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("second route should succeed");

    let merged = runtime
        .topic_routing_summary(
            "alpha",
            Some("merge hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("merge summary should succeed")
        .join("\n");
    assert!(merged.contains("- active topic sessions: 1"));
    assert!(merged.contains("- created topic sessions: 1"));
    assert!(merged.contains("kind=Merged"));
    assert!(merged.contains("topic-alpha-hello-adaptive-memory-rust-worker-pipeline"));

    let split = runtime
        .topic_routing_summary(
            "alpha",
            Some("split hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("split summary should succeed")
        .join("\n");
    assert!(split.contains("- active topic sessions: 2"));
    assert!(split.contains("- created topic sessions: 0"));
    assert!(split.contains("- revived topic sessions: 2"));
    assert!(split.contains("kind=Split"));

    let session_summary = runtime
        .topic_session_summary("alpha")
        .expect("topic session summary should succeed")
        .join("\n");
    assert!(session_summary.contains("id=topic-session-bootstrap:alpha topic=topic-alpha label=\"hello adaptive memory\" status=Active"));
    assert!(session_summary.contains("id=topic-session-bootstrap:alpha:rust-worker-pipeline topic=topic-alpha-rust-worker-pipeline label=\"rust worker pipeline\" status=Active"));
    assert!(session_summary.contains("id=topic-session-bootstrap:alpha:hello-adaptive-memory-rust-worker-pipeline topic=topic-alpha-hello-adaptive-memory-rust-worker-pipeline label=\"hello adaptive memory + rust worker pipeline\" status=Dormant"));
}

#[tokio::test]
async fn topic_routing_summary_surfaces_graph_expansion_from_component_to_composite_topic() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("first route should succeed");
    runtime
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");
    runtime
        .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("second route should succeed");
    runtime
        .route_topics(
            "alpha",
            Some("merge hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("merge route should succeed");

    let summary = runtime
        .topic_routing_summary("alpha", Some("hello adaptive memory"), 8, 8, 8, 2)
        .expect("routing summary should succeed")
        .join("\n");

    assert!(summary.contains("- active topic sessions: 2"));
    assert!(summary.contains("- created topic sessions: 0"));
    assert!(summary.contains("- revived topic sessions: 1"));
    assert!(summary.contains("kind=CoActivated"));
    assert!(summary.contains("bootstrap topic graph expanded"));
    assert!(summary.contains("hello adaptive memory + rust worker pipeline"));
}

#[tokio::test]
async fn topic_routing_and_session_summaries_surface_stored_graph_edges() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("first route should succeed");
    runtime
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");
    runtime
        .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("second route should succeed");
    runtime
        .route_topics(
            "alpha",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("mixed route should succeed");

    let routing_summary = runtime
        .topic_routing_summary("alpha", Some("hello adaptive memory"), 8, 8, 8, 2)
        .expect("routing summary should succeed")
        .join("\n");
    assert!(routing_summary.contains("kind=CoActivated"));
    assert!(
        routing_summary.contains("topic graph expansion")
            || routing_summary.contains("bootstrap topic graph expanded")
    );

    let session_summary = runtime
        .topic_session_summary("alpha")
        .expect("topic session summary should succeed")
        .join("\n");
    assert!(session_summary.contains("graph_links=1"));
}

#[tokio::test]
async fn neuron_activation_summary_surfaces_inhibitory_suppression_for_contrast_query() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("switch should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    runtime
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("first route should succeed");
    runtime
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");
    runtime
        .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("second route should succeed");

    let summary = runtime
        .neuron_activation_summary(
            "alpha",
            Some("hello adaptive memory but not rust worker pipeline"),
            8,
            8,
            8,
            3,
        )
        .expect("neuron summary should succeed")
        .join("\n");

    assert!(summary.contains("neuron=neuron-alpha"));
    assert!(summary.contains("neuron=neuron-alpha-rust-worker-pipeline"));
    assert!(summary.contains("inhibited=0.00"));
    assert!(
        summary
            .lines()
            .any(|line| line.contains("inhibited=") && !line.contains("inhibited=0.00"))
    );
    assert!(summary.contains("links=1"));
}
