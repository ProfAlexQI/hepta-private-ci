use super::*;

#[tokio::test]
async fn saves_and_loads_runtime_snapshot_across_instances() {
    let source = RuntimeKernel::new();
    source
        .run_demo_turn("hello persistence")
        .await
        .expect("plain turn should succeed");
    source
        .switch_model("mock-ollama/local-precise")
        .expect("model switch should succeed");
    source
        .approve_tool("read_file")
        .expect("approval should succeed");
    source
        .run_demo_turn(&architecture_foundation_read_intent())
        .await
        .expect("approved read turn should succeed");

    let snapshot_path = test_artifact_path(format!(
        "hepta-runtime-snapshot-{}.json",
        std::process::id()
    ));
    source
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot load should succeed");

    let selection = restored.model_selection().expect("selection should load");
    assert_eq!(selection.active.provider, "mock-ollama");
    assert_eq!(selection.active.model, "local-precise");

    let approvals = restored.approval_snapshot().expect("approvals should load");
    assert!(
        approvals
            .granted_tools
            .iter()
            .any(|tool| tool == "read_file")
    );

    let sessions = restored.sessions().expect("sessions should load");
    assert_eq!(sessions.len(), 1);
    let history = restored
        .history(Some("session-main"), 10)
        .expect("history should load");
    assert!(history.len() >= 2);
    let memories = restored.memory_snapshot(10).expect("memories should load");
    assert!(
        memories
            .iter()
            .any(|memory| memory.content.contains("hello persistence"))
    );

    let _ = std::fs::remove_file(snapshot_path);
}

#[tokio::test]
async fn saves_and_loads_runtime_snapshot_with_topic_sessions_and_graph_store() {
    let source = RuntimeKernel::new();
    source
        .switch_session("alpha")
        .expect("session switch should succeed");
    source
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("first turn should succeed");
    source
        .route_topics("alpha", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("first route should succeed");
    source
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("second turn should succeed");
    source
        .route_topics("alpha", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("second route should succeed");
    source
        .route_topics(
            "alpha",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("mixed route should succeed");

    let snapshot_path = test_artifact_path(format!(
        "hepta-runtime-topic-graph-snapshot-{}.json",
        std::process::id()
    ));
    source
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot load should succeed");

    let raw_topic_sessions = restored
        .topic_session_state
        .lock()
        .expect("topic session state lock should succeed")
        .sessions
        .clone();
    let raw_topic_graph_edges = restored
        .topic_graph_state
        .lock()
        .expect("topic graph state lock should succeed")
        .edges
        .clone();
    assert_eq!(raw_topic_sessions.len(), 2);
    assert!(raw_topic_graph_edges.iter().any(|record| {
        record.source_topic_session_id == "topic-session-bootstrap:alpha"
            && record.edge.target_topic_session_id
                == "topic-session-bootstrap:alpha:rust-worker-pipeline"
    }));

    let topic_sessions = restored
        .topic_sessions_for_surface("alpha")
        .expect("topic sessions should load");
    assert!(topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:alpha"
            && !topic_session.graph_edges.is_empty()
    }));

    let decision = restored
        .route_topics("alpha", Some("hello adaptive memory"), 8, 8, 8, 2)
        .expect("graph-expanded route should succeed");
    assert!(
        decision
            .active_topic_session_ids
            .iter()
            .any(|id| { id == "topic-session-bootstrap:alpha:rust-worker-pipeline" })
    );
    assert!(decision.activation_scores.iter().any(|score| {
        score.topic_id.0 == "topic-alpha-rust-worker-pipeline"
            && score
                .reason
                .as_deref()
                .is_some_and(|reason| reason.contains("stored co-activation edge"))
    }));

    let _ = std::fs::remove_file(snapshot_path);
}

#[tokio::test]
async fn loads_legacy_runtime_snapshot_missing_approvals_field() {
    let source = RuntimeKernel::new();
    source
        .run_demo_turn("legacy snapshot")
        .await
        .expect("plain turn should succeed");

    let snapshot_path = test_artifact_path(format!(
        "hepta-legacy-runtime-snapshot-{}.json",
        std::process::id()
    ));
    source
        .save_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("snapshot save should succeed");

    let mut snapshot_json: Value = serde_json::from_str(
        &fs::read_to_string(&snapshot_path).expect("snapshot should be readable"),
    )
    .expect("snapshot json should parse");
    let snapshot_object = snapshot_json
        .as_object_mut()
        .expect("snapshot json should be an object");
    snapshot_object.remove("approvals");
    snapshot_object.remove("topic_sessions");
    snapshot_object.remove("topic_graph_edges");
    fs::write(
        &snapshot_path,
        serde_json::to_string_pretty(&snapshot_json).expect("snapshot should serialize"),
    )
    .expect("legacy snapshot should be writable");

    let restored = RuntimeKernel::new();
    restored
        .load_snapshot(snapshot_path.to_str().expect("path should be utf8"))
        .expect("legacy snapshot load should succeed");

    let approvals = restored.approval_snapshot().expect("approvals should load");
    assert!(approvals.granted_tools.is_empty());
    assert!(approvals.pending.is_empty());

    let history = restored
        .history(Some("session-main"), 10)
        .expect("history should load");
    assert!(!history.is_empty());

    let _ = std::fs::remove_file(snapshot_path);
}

#[tokio::test]
async fn switches_active_session_and_persists_it() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("research-lab")
        .expect("session switch should succeed");
    runtime
        .run_demo_turn("hello switched session")
        .await
        .expect("turn should succeed");

    assert_eq!(
        runtime.active_session_id().expect("session id should load"),
        "research-lab"
    );
    let sessions = runtime.sessions().expect("sessions should load");
    let session = sessions
        .iter()
        .find(|session| session.session_id == "research-lab")
        .expect("research-lab session should exist");
    assert!(session.is_active);
    assert!(session.last_active_unix_ms >= session.created_at_unix_ms);
    let history = runtime
        .history(Some("research-lab"), 10)
        .expect("history should load");
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].input, "hello switched session");
}

#[tokio::test]
async fn can_rename_session_and_track_last_user_intent() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("research-lab")
        .expect("session switch should succeed");
    runtime
        .rename_active_session("Research planning")
        .expect("session rename should succeed");
    runtime
        .run_demo_turn("map out the next architecture milestone for Hepta")
        .await
        .expect("turn should succeed");
    runtime
        .route_topics(
            "research-lab",
            Some("map out the next architecture milestone for Hepta"),
            4,
            4,
            4,
            1,
        )
        .expect("topic route should succeed");

    let session = runtime
        .active_session_snapshot()
        .expect("active session snapshot should load");
    assert_eq!(session.title, "Research planning");
    assert_eq!(
        session.last_user_intent_summary.as_deref(),
        Some("map out the next architecture milestone for Hepta")
    );
    assert_eq!(session.topic_session_count, 1);
    assert_eq!(session.topic_graph_edge_count, 0);
}

#[tokio::test]
async fn can_run_in_specific_session_without_switching_active_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");

    let result = runtime
        .run_demo_turn_in_session("beta", "draft a beta session plan")
        .await
        .expect("beta run should succeed");

    assert_eq!(result.session_id, "beta");
    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "alpha"
    );

    let beta_history = runtime
        .history(Some("beta"), 10)
        .expect("beta history should load");
    assert_eq!(beta_history.len(), 1);
    assert_eq!(beta_history[0].input, "draft a beta session plan");

    let alpha_session = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "alpha")
        .expect("alpha session should exist");
    assert!(alpha_session.is_active);
}

#[test]
fn models_are_scoped_per_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .switch_model("mock-ollama/local-precise")
        .expect("alpha model switch should succeed");
    runtime
        .switch_model_in_session("beta", "demo/demo-creative")
        .expect("beta model switch should succeed");

    let alpha = runtime
        .model_selection_for_session("alpha")
        .expect("alpha model selection should load");
    assert_eq!(alpha.active.provider, "mock-ollama");
    assert_eq!(alpha.active.model, "local-precise");

    let beta = runtime
        .model_selection_for_session("beta")
        .expect("beta model selection should load");
    assert_eq!(beta.active.provider, "demo");
    assert_eq!(beta.active.model, "demo-creative");

    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "alpha"
    );

    let beta_session = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "beta")
        .expect("beta session should exist");
    assert_eq!(beta_session.model.provider, "demo");
    assert_eq!(beta_session.model.model, "demo-creative");
}

#[tokio::test]
async fn query_events_filters_by_kind_session_and_limit() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn("hello alpha")
        .await
        .expect("alpha turn should succeed");
    runtime
        .switch_session("beta")
        .expect("beta session switch should succeed");
    runtime
        .run_demo_turn("hello beta")
        .await
        .expect("beta turn should succeed");

    let beta_switch_events = runtime
        .query_events(25, Some(&EventKind::SessionSwitched), Some("beta"))
        .expect("filtered beta events should load");
    assert_eq!(beta_switch_events.len(), 1);
    assert_eq!(beta_switch_events[0].event.kind, EventKind::SessionSwitched);
    assert_eq!(
        beta_switch_events[0]
            .event
            .session_id
            .as_ref()
            .map(|session_id| session_id.0.as_str()),
        Some("beta")
    );

    let limited_switch_events = runtime
        .query_events(1, Some(&EventKind::SessionSwitched), None)
        .expect("limited switch events should load");
    assert_eq!(limited_switch_events.len(), 1);
    assert_eq!(
        limited_switch_events[0]
            .event
            .session_id
            .as_ref()
            .map(|session_id| session_id.0.as_str()),
        Some("beta")
    );
}

#[tokio::test]
async fn approvals_are_scoped_per_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .approve_tool("read_file")
        .expect("alpha approval should succeed");
    let alpha = runtime
        .approval_snapshot()
        .expect("alpha approvals should load");
    assert!(alpha.granted_tools.iter().any(|tool| tool == "read_file"));

    runtime
        .switch_session("beta")
        .expect("beta session switch should succeed");
    let beta = runtime
        .approval_snapshot()
        .expect("beta approvals should load");
    assert!(beta.granted_tools.is_empty());

    let blocked = runtime
        .run_demo_turn(&architecture_foundation_read_intent())
        .await
        .expect("beta read turn should return approval requirement");
    assert_eq!(blocked.approval_required.as_deref(), Some("read_file"));

    runtime
        .switch_session("alpha")
        .expect("switch back to alpha should succeed");
    let alpha_again = runtime
        .approval_snapshot()
        .expect("alpha approvals should still load");
    assert!(
        alpha_again
            .granted_tools
            .iter()
            .any(|tool| tool == "read_file")
    );
}

#[tokio::test]
async fn can_grant_and_inspect_approvals_for_non_active_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .approve_tool_in_session("beta", "read_file")
        .expect("beta approval should succeed");

    let alpha = runtime
        .approval_snapshot()
        .expect("alpha approvals should load");
    assert!(alpha.granted_tools.is_empty());

    let beta = runtime
        .approval_snapshot_for_session("beta")
        .expect("beta approvals should load");
    assert!(beta.granted_tools.iter().any(|tool| tool == "read_file"));

    let result = runtime
        .run_demo_turn_in_session("beta", &architecture_foundation_read_intent())
        .await
        .expect("beta read turn should succeed");
    assert_eq!(result.invoked_tool.as_deref(), Some("read_file"));
    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "alpha"
    );
}

#[tokio::test]
async fn archiving_active_session_switches_to_fallback() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn("keep alpha history")
        .await
        .expect("alpha turn should succeed");

    runtime
        .archive_session(None)
        .expect("archive should succeed");

    assert_ne!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "alpha"
    );
    let alpha = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "alpha")
        .expect("alpha session should exist");
    assert!(alpha.archived_at_unix_ms.is_some());
}

#[tokio::test]
async fn archiving_fresh_active_session_materializes_and_switches_to_fallback() {
    let runtime = RuntimeKernel::new();

    runtime
        .archive_session(None)
        .expect("archive should succeed for fresh active session");

    assert_ne!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "session-main"
    );
    let archived = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "session-main")
        .expect("session-main should exist");
    assert!(archived.archived_at_unix_ms.is_some());
}

#[tokio::test]
async fn deleting_session_removes_related_runtime_state() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn_in_session("beta", "beta note")
        .await
        .expect("beta turn should succeed");
    runtime
        .approve_tool_in_session("beta", "read_file")
        .expect("beta approval should succeed");
    runtime
        .switch_model_in_session("beta", "demo/demo-creative")
        .expect("beta model switch should succeed");
    runtime
        .run_demo_turn_in_session("beta", "hello adaptive memory")
        .await
        .expect("beta routed turn should succeed");
    runtime
        .route_topics("beta", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("beta route should succeed");

    runtime
        .delete_session("beta")
        .expect("delete should succeed");

    assert!(
        runtime
            .history(Some("beta"), 10)
            .expect("beta history should load")
            .is_empty()
    );
    assert!(
        runtime
            .approval_snapshot_for_session("beta")
            .expect("beta approvals should load")
            .granted_tools
            .is_empty()
    );
    assert!(
        runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .all(|session| session.session_id != "beta")
    );
    assert!(
        runtime
            .topic_sessions_for_surface("beta")
            .expect("beta topic sessions should load")
            .is_empty()
    );
    assert!(
        runtime
            .topic_graph_state
            .lock()
            .expect("topic graph state lock should succeed")
            .edges
            .iter()
            .all(|record| {
                !record.source_topic_session_id.contains("beta")
                    && !record.edge.target_topic_session_id.contains("beta")
            })
    );
}

#[tokio::test]
async fn prune_prefers_archived_sessions_and_keeps_active() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn("alpha work")
        .await
        .expect("alpha turn should succeed");
    runtime
        .run_demo_turn_in_session("beta", "beta work")
        .await
        .expect("beta turn should succeed");
    runtime
        .run_demo_turn_in_session("gamma", "gamma work")
        .await
        .expect("gamma turn should succeed");
    runtime
        .archive_session(Some("beta"))
        .expect("beta archive should succeed");

    let result = runtime.prune_sessions(2).expect("prune should succeed");
    assert!(result.contains("beta"));
    let sessions = runtime.sessions().expect("sessions should load");
    assert!(
        sessions
            .iter()
            .any(|session| session.session_id == "alpha" && session.is_active)
    );
    assert!(sessions.iter().all(|session| session.session_id != "beta"));
}

#[tokio::test]
async fn prune_sessions_counts_fresh_active_session() {
    let runtime = RuntimeKernel::new();

    let result = runtime
        .prune_sessions(1)
        .expect("prune should succeed for fresh runtime");

    assert_eq!(result, "no pruning needed, sessions=1 max=1");
    let sessions = runtime.sessions().expect("sessions should load");
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].session_id, "session-main");
    assert!(sessions[0].is_active);
}

#[tokio::test]
async fn exports_and_imports_single_session_package() {
    let source = RuntimeKernel::new();
    source
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    source
        .run_demo_turn_in_session("beta", "beta exported work")
        .await
        .expect("beta turn should succeed");
    source
        .rename_active_session("Alpha workspace")
        .expect("alpha rename should succeed");
    source
        .switch_model_in_session("beta", "demo/demo-creative")
        .expect("beta model switch should succeed");
    source
        .approve_tool_in_session("beta", "read_file")
        .expect("beta approval should succeed");
    source
        .archive_session(Some("beta"))
        .expect("beta archive should succeed");

    let export_path =
        test_artifact_path(format!("hepta-session-export-{}.json", std::process::id()));
    let export_report = source
        .export_session("beta", export_path.to_str().expect("path should be utf8"))
        .expect("beta export should succeed");
    assert_eq!(export_report.session_id, "beta");
    assert_eq!(export_report.title, "Hepta session beta");
    assert_eq!(export_report.model.model, "demo-creative");
    assert!(export_report.archived);
    assert_eq!(export_report.approvals_granted, 1);
    assert_eq!(export_report.history_entries, 1);
    assert_eq!(export_report.topic_session_count, 0);
    assert_eq!(export_report.topic_graph_edge_count, 0);

    let restored = RuntimeKernel::new();
    let import_report = restored
        .import_session(export_path.to_str().expect("path should be utf8"))
        .expect("beta import should succeed");
    assert_eq!(import_report.session_id, "beta");
    assert_eq!(import_report.imported_title, "Hepta session beta");
    assert_eq!(import_report.imported_model.model, "demo-creative");
    assert!(import_report.imported_archived);
    assert_eq!(import_report.approvals_granted, 1);
    assert_eq!(import_report.history_entries, 1);
    assert_eq!(import_report.topic_session_count, 0);
    assert_eq!(import_report.topic_graph_edge_count, 0);

    let beta = restored
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "beta")
        .expect("beta session should exist after import");
    assert_eq!(beta.model.provider, "demo");
    assert_eq!(beta.model.model, "demo-creative");
    assert!(beta.archived_at_unix_ms.is_some());
    assert_eq!(
        beta.last_user_intent_summary.as_deref(),
        Some("beta exported work")
    );

    let beta_approvals = restored
        .approval_snapshot_for_session("beta")
        .expect("beta approvals should load");
    assert!(
        beta_approvals
            .granted_tools
            .iter()
            .any(|tool| tool == "read_file")
    );

    let beta_history = restored
        .history(Some("beta"), 10)
        .expect("beta history should load");
    assert_eq!(beta_history.len(), 1);
    assert_eq!(beta_history[0].input, "beta exported work");

    let _ = std::fs::remove_file(export_path);
}

#[tokio::test]
async fn exports_and_imports_single_session_package_with_topic_graph_state() {
    let source = RuntimeKernel::new();
    source
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    source
        .run_demo_turn_in_session("beta", "hello adaptive memory")
        .await
        .expect("beta first turn should succeed");
    source
        .route_topics("beta", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("beta first route should succeed");
    source
        .run_demo_turn_in_session("beta", "rust worker pipeline")
        .await
        .expect("beta second turn should succeed");
    source
        .route_topics("beta", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("beta second route should succeed");
    source
        .route_topics(
            "beta",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("beta mixed route should succeed");

    let export_path = test_artifact_path(format!(
        "hepta-session-topic-graph-export-{}.json",
        std::process::id()
    ));
    let export_report = source
        .export_session("beta", export_path.to_str().expect("path should be utf8"))
        .expect("beta export should succeed");
    assert_eq!(export_report.topic_session_count, 2);
    assert_eq!(export_report.topic_graph_edge_count, 2);

    let restored = RuntimeKernel::new();
    let import_report = restored
        .import_session(export_path.to_str().expect("path should be utf8"))
        .expect("beta import should succeed");
    assert_eq!(import_report.topic_session_count, 2);
    assert_eq!(import_report.topic_graph_edge_count, 2);

    let raw_topic_graph_edges = restored
        .topic_graph_state
        .lock()
        .expect("topic graph state lock should succeed")
        .edges
        .clone();
    assert!(raw_topic_graph_edges.iter().any(|record| {
        record.source_topic_session_id == "topic-session-bootstrap:beta"
            && record.edge.target_topic_session_id
                == "topic-session-bootstrap:beta:rust-worker-pipeline"
    }));

    let decision = restored
        .route_topics("beta", Some("hello adaptive memory"), 8, 8, 8, 2)
        .expect("graph-expanded route should succeed");
    assert!(
        decision
            .active_topic_session_ids
            .iter()
            .any(|id| { id == "topic-session-bootstrap:beta:rust-worker-pipeline" })
    );

    let _ = std::fs::remove_file(export_path);
}

#[tokio::test]
async fn session_export_roundtrip_preserves_intelligence_learning_state() {
    let source = RuntimeKernel::new();
    source
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    for input in [
        "semantic router should learn from accepted feedback",
        "feedback calibration closes the loop into future intuition",
        "merge topic sessions then split them back into stable neurons",
        "aging neurons need refresh with transcript evidence",
    ] {
        source
            .run_demo_turn_in_session("beta", input)
            .await
            .expect("intelligence hardening turn should succeed");
    }

    let bundle = source
        .predict_intuition(
            "beta",
            "semantic router learned feedback should route topic memory",
            12,
            12,
            12,
            6,
            6,
            6,
        )
        .expect("intuition should produce a bundle");
    assert!(!bundle.topic_activation_scores.is_empty());
    assert!(!bundle.neuron_activations.is_empty());

    let skill_id = bundle
        .skill_decisions
        .first()
        .map(|decision| decision.skill_id.clone());
    let workflow_id = bundle
        .workflow_priors
        .first()
        .map(|prior| prior.workflow_id.clone());
    let source_topic_ids = bundle
        .topic_activation_scores
        .iter()
        .map(|score| score.topic_id.clone())
        .collect::<Vec<_>>();
    let source_neuron_ids = bundle
        .neuron_activations
        .iter()
        .map(|activation| activation.neuron_id.clone())
        .collect::<Vec<_>>();
    source
        .record_intuition_feedback(
            "beta",
            "semantic router learned feedback should route topic memory",
            IntuitionFeedbackOutcome::ExecutedSuccess,
            skill_id.as_deref(),
            workflow_id.as_deref(),
            source_topic_ids.clone(),
            source_neuron_ids,
            Some("release hardening accepted learned semantic router"),
        )
        .expect("feedback learning should be recorded");
    source
        .record_model_router_feedback(
            "beta",
            "semantic router learned feedback should route topic memory",
            ModelRef {
                provider: "demo".into(),
                model: "demo-chat".into(),
            },
            TopicAwareModelFeedbackOutcome::ExecutedSuccess,
            source_topic_ids.clone(),
            Some(1200),
            Some(0.03),
            Some(0.9),
            Some(0.8),
            Some("model-router feedback survived export"),
        )
        .expect("model-router feedback should be recorded");

    let before_route = source
        .route_topics(
            "beta",
            Some("semantic router learned feedback release hardening"),
            12,
            12,
            12,
            6,
        )
        .expect("learned router route should succeed before export");
    assert_eq!(
        before_route.router_id,
        "semantic-router:learned-feedback-v1"
    );
    assert!(before_route.learned_signal_count > 0);

    let before_calibration = source
        .intuition_calibration_overview("beta")
        .expect("calibration overview should load before export");
    assert!(before_calibration.closed_loop_ready);
    assert!(before_calibration.learned_topic_hint_count > 0);
    assert!(before_calibration.learned_neuron_update_count > 0);
    let before_model_calibration = source
        .model_router_feedback_summary("beta")
        .expect("model-router calibration should load before export");
    assert_eq!(before_model_calibration.len(), 1);
    assert!(before_model_calibration[0].success_rate > 0.0);

    let before_lifecycle = source
        .neuron_lifecycle_overview("beta")
        .expect("lifecycle overview should load before export");
    assert!(before_lifecycle.stored_neurons > 0);
    assert!(before_lifecycle.average_confidence > 0.0);

    let export_path = test_artifact_path(format!(
        "hepta-session-intelligence-export-{}.json",
        std::process::id()
    ));
    let export_report = source
        .export_session("beta", export_path.to_str().expect("path should be utf8"))
        .expect("beta intelligence export should succeed");
    assert_eq!(export_report.neuron_count, before_lifecycle.stored_neurons);
    assert_eq!(
        export_report.intuition_feedback_count,
        before_calibration.feedback_record_count
    );
    assert_eq!(
        export_report.model_router_feedback_count,
        before_model_calibration[0].record_count
    );

    let restored = RuntimeKernel::new();
    let import_report = restored
        .import_session(export_path.to_str().expect("path should be utf8"))
        .expect("beta intelligence import should succeed");
    assert_eq!(import_report.neuron_count, before_lifecycle.stored_neurons);
    assert_eq!(
        import_report.intuition_feedback_count,
        before_calibration.feedback_record_count
    );
    assert_eq!(
        import_report.model_router_feedback_count,
        before_model_calibration[0].record_count
    );

    let after_route = restored
        .route_topics(
            "beta",
            Some("semantic router learned feedback release hardening"),
            12,
            12,
            12,
            6,
        )
        .expect("learned router route should succeed after import");
    assert_eq!(after_route.router_id, "semantic-router:learned-feedback-v1");
    assert!(after_route.learned_signal_count >= before_route.learned_signal_count);

    let after_calibration = restored
        .intuition_calibration_overview("beta")
        .expect("calibration overview should load after import");
    assert_eq!(
        after_calibration.feedback_record_count,
        before_calibration.feedback_record_count
    );
    assert!(after_calibration.closed_loop_ready);
    assert_eq!(
        after_calibration.learned_neuron_update_count,
        before_calibration.learned_neuron_update_count
    );
    let after_model_calibration = restored
        .model_router_feedback_summary("beta")
        .expect("model-router calibration should load after import");
    assert_eq!(after_model_calibration, before_model_calibration);

    let after_lifecycle = restored
        .neuron_lifecycle_overview("beta")
        .expect("lifecycle overview should load after import");
    assert_eq!(
        after_lifecycle.stored_neurons,
        before_lifecycle.stored_neurons
    );
    assert!(after_lifecycle.average_confidence > 0.0);

    let _ = std::fs::remove_file(export_path);
}

#[tokio::test]
async fn imports_legacy_session_export_missing_approval_field() {
    let source = RuntimeKernel::new();
    source
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    source
        .run_demo_turn_in_session("beta", "legacy export")
        .await
        .expect("beta turn should succeed");

    let export_path = test_artifact_path(format!(
        "hepta-legacy-session-export-{}.json",
        std::process::id()
    ));
    source
        .export_session("beta", export_path.to_str().expect("path should be utf8"))
        .expect("beta export should succeed");

    let mut export_json: Value =
        serde_json::from_str(&fs::read_to_string(&export_path).expect("export should be readable"))
            .expect("export json should parse");
    let export_object = export_json
        .as_object_mut()
        .expect("export json should be an object");
    export_object.remove("approval");
    export_object.remove("topic_sessions");
    export_object.remove("topic_graph_edges");
    fs::write(
        &export_path,
        serde_json::to_string_pretty(&export_json).expect("export should serialize"),
    )
    .expect("legacy export should be writable");

    let restored = RuntimeKernel::new();
    let import_report = restored
        .import_session(export_path.to_str().expect("path should be utf8"))
        .expect("legacy export import should succeed");
    assert_eq!(import_report.topic_session_count, 0);
    assert_eq!(import_report.topic_graph_edge_count, 0);

    let beta_approvals = restored
        .approval_snapshot_for_session("beta")
        .expect("beta approvals should load");
    assert!(beta_approvals.granted_tools.is_empty());
    assert!(beta_approvals.pending.is_empty());

    let beta_history = restored
        .history(Some("beta"), 10)
        .expect("beta history should load");
    assert_eq!(beta_history.len(), 1);
    assert_eq!(beta_history[0].input, "legacy export");

    let _ = std::fs::remove_file(export_path);
}

#[tokio::test]
async fn forks_session_into_independent_branch() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn_in_session("beta", "beta base work")
        .await
        .expect("beta turn should succeed");
    runtime
        .switch_model_in_session("beta", "demo/demo-creative")
        .expect("beta model switch should succeed");
    runtime
        .approve_tool_in_session("beta", "read_file")
        .expect("beta approval should succeed");
    runtime
        .archive_session(Some("beta"))
        .expect("beta archive should succeed");

    let fork_report = runtime
        .fork_session("beta", "beta-fork")
        .expect("beta fork should succeed");
    assert_eq!(fork_report.source_session_id, "beta");
    assert_eq!(fork_report.target_session_id, "beta-fork");
    assert_eq!(fork_report.target_model.model, "demo-creative");
    assert!(!fork_report.target_archived);
    assert_eq!(fork_report.approvals_granted, 1);
    assert_eq!(fork_report.history_entries, 1);
    assert_eq!(fork_report.topic_session_count, 0);
    assert_eq!(fork_report.topic_graph_edge_count, 0);
    assert_eq!(fork_report.active_session_after_fork, "alpha");

    let fork = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "beta-fork")
        .expect("beta-fork session should exist");
    assert_eq!(fork.model.provider, "demo");
    assert_eq!(fork.model.model, "demo-creative");
    assert!(fork.archived_at_unix_ms.is_none());
    assert_eq!(
        fork.last_user_intent_summary.as_deref(),
        Some("beta base work")
    );
    assert!(fork.title.contains("(fork)"));

    let fork_approvals = runtime
        .approval_snapshot_for_session("beta-fork")
        .expect("fork approvals should load");
    assert!(
        fork_approvals
            .granted_tools
            .iter()
            .any(|tool| tool == "read_file")
    );

    let fork_history = runtime
        .history(Some("beta-fork"), 10)
        .expect("fork history should load");
    assert_eq!(fork_history.len(), 1);
    assert_eq!(fork_history[0].session_id, "beta-fork");
    assert_eq!(fork_history[0].input, "beta base work");

    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "alpha"
    );
}

#[tokio::test]
async fn fork_session_rejects_fresh_active_target_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .run_demo_turn_in_session("beta", "beta base work")
        .await
        .expect("beta turn should succeed");

    let err = runtime
        .fork_session("beta", "session-main")
        .expect_err("fresh active target should still be treated as existing");

    assert_eq!(err.0, "target session already exists: session-main");
    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "session-main"
    );
    assert!(
        runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .any(|session| session.session_id == "session-main" && session.is_active)
    );
}

#[tokio::test]
async fn fork_session_rebases_topic_sessions_and_graph_state() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn_in_session("beta", "hello adaptive memory")
        .await
        .expect("beta first turn should succeed");
    runtime
        .route_topics("beta", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("beta first route should succeed");
    runtime
        .run_demo_turn_in_session("beta", "rust worker pipeline")
        .await
        .expect("beta second turn should succeed");
    runtime
        .route_topics("beta", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("beta second route should succeed");
    runtime
        .route_topics(
            "beta",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("beta mixed route should succeed");

    let fork_report = runtime
        .fork_session("beta", "beta-fork")
        .expect("beta fork should succeed");
    assert_eq!(fork_report.topic_session_count, 2);
    assert_eq!(fork_report.topic_graph_edge_count, 2);

    let fork_topic_sessions = runtime
        .topic_sessions_for_surface("beta-fork")
        .expect("fork topic sessions should load");
    assert!(fork_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:beta-fork"
            && topic_session.topic_id.0 == "topic-beta-fork"
    }));
    assert!(fork_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:beta-fork:rust-worker-pipeline"
            && topic_session.topic_id.0 == "topic-beta-fork-rust-worker-pipeline"
            && !topic_session.graph_edges.is_empty()
    }));

    let decision = runtime
        .route_topics("beta-fork", Some("hello adaptive memory"), 8, 8, 8, 2)
        .expect("fork graph-expanded route should succeed");
    assert!(
        decision
            .active_topic_session_ids
            .iter()
            .any(|id| { id == "topic-session-bootstrap:beta-fork:rust-worker-pipeline" })
    );
}

#[tokio::test]
async fn merges_session_into_target_without_overwriting_target_model_or_title() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("mainline")
        .expect("mainline session switch should succeed");
    runtime
        .rename_active_session("Mainline workspace")
        .expect("mainline rename should succeed");
    runtime
        .switch_model("mock-ollama/local-precise")
        .expect("mainline model switch should succeed");
    runtime
        .run_demo_turn("mainline seed")
        .await
        .expect("mainline turn should succeed");
    runtime
        .run_demo_turn_in_session("beta-fork", "fork delta")
        .await
        .expect("fork turn should succeed");
    runtime
        .approve_tool_in_session("beta-fork", "read_file")
        .expect("fork approval should succeed");
    runtime
        .archive_session(Some("beta-fork"))
        .expect("fork archive should succeed");

    runtime
        .merge_session("beta-fork", "mainline", MergeOptions::default())
        .expect("merge should succeed");

    let mainline = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "mainline")
        .expect("mainline session should exist");
    assert_eq!(mainline.title, "Mainline workspace");
    assert_eq!(mainline.model.provider, "mock-ollama");
    assert_eq!(mainline.model.model, "local-precise");
    assert!(mainline.archived_at_unix_ms.is_none());
    assert_eq!(
        mainline.last_user_intent_summary.as_deref(),
        Some("fork delta")
    );

    let approvals = runtime
        .approval_snapshot_for_session("mainline")
        .expect("mainline approvals should load");
    assert!(
        approvals
            .granted_tools
            .iter()
            .any(|tool| tool == "read_file")
    );

    let history = runtime
        .history(Some("mainline"), 10)
        .expect("mainline history should load");
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].input, "fork delta");
    assert_eq!(history[1].input, "mainline seed");
}

#[tokio::test]
async fn diffs_sessions_semantically_without_treating_forked_history_as_all_different() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("alpha")
        .expect("alpha session switch should succeed");
    runtime
        .run_demo_turn_in_session("beta", "shared base")
        .await
        .expect("beta turn should succeed");
    runtime
        .approve_tool_in_session("beta", "read_file")
        .expect("beta approval should succeed");
    runtime
        .fork_session("beta", "beta-fork")
        .expect("beta fork should succeed");
    runtime
        .archive_session(Some("beta"))
        .expect("beta archive should succeed");
    runtime
        .switch_model_in_session("beta-fork", "demo/demo-creative")
        .expect("fork model switch should succeed");
    runtime
        .run_demo_turn_in_session("beta-fork", "fork-only delta")
        .await
        .expect("fork delta turn should succeed");

    let report = runtime
        .diff_sessions("beta", "beta-fork")
        .expect("diff should succeed");

    assert_eq!(report.left_session_id, "beta");
    assert_eq!(report.right_session_id, "beta-fork");
    assert_eq!(report.left_title, "Hepta session beta");
    assert_eq!(report.right_title, "Hepta session beta (fork)");
    assert_eq!(report.left_model.provider, "demo");
    assert_eq!(report.left_model.model, "demo-chat");
    assert_eq!(report.right_model.provider, "demo");
    assert_eq!(report.right_model.model, "demo-creative");
    assert!(report.left_archived);
    assert!(!report.right_archived);
    assert_eq!(report.left_history_count, 1);
    assert_eq!(report.right_history_count, 2);
    assert_eq!(report.shared_history_count, 1);
    assert!(report.approvals_only_left.is_empty());
    assert!(report.approvals_only_right.is_empty());
    assert!(report.history_only_left.is_empty());
    assert_eq!(report.history_only_right.len(), 1);
    assert!(report.history_only_right[0].contains("fork-only delta"));
    assert_eq!(
        report.left_last_user_intent_summary.as_deref(),
        Some("shared base")
    );
    assert_eq!(
        report.right_last_user_intent_summary.as_deref(),
        Some("fork-only delta")
    );
}

#[tokio::test]
async fn previews_deduplicating_merge_plan_for_forked_history() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("beta")
        .expect("beta session switch should succeed");
    runtime
        .run_demo_turn("shared base")
        .await
        .expect("beta base turn should succeed");
    runtime
        .approve_tool("read_file")
        .expect("beta approval should succeed");
    runtime
        .fork_session("beta", "beta-fork")
        .expect("beta fork should succeed");
    runtime
        .switch_model_in_session("beta-fork", "demo/demo-creative")
        .expect("fork model switch should succeed");
    runtime
        .run_demo_turn_in_session("beta-fork", "fork-only delta")
        .await
        .expect("fork delta turn should succeed");

    let report = runtime
        .preview_merge_session("beta-fork", "beta", MergeOptions::default())
        .expect("merge preview should succeed");

    assert_eq!(report.source_session_id, "beta-fork");
    assert_eq!(report.target_session_id, "beta");
    assert_eq!(report.target_title_before, "Hepta session beta");
    assert_eq!(report.target_title_after, "Hepta session beta");
    assert_eq!(report.target_model_before.provider, "demo");
    assert_eq!(report.target_model_before.model, "demo-chat");
    assert_eq!(report.target_model_after.provider, "demo");
    assert_eq!(report.target_model_after.model, "demo-chat");
    assert!(!report.target_archived_before);
    assert!(!report.target_archived_after);
    assert!(!report.source_deleted_after_merge);
    assert_eq!(report.source_history_count, 2);
    assert_eq!(report.target_history_count, 1);
    assert_eq!(report.history_entries_to_append, 1);
    assert_eq!(report.history_entries_skipped_as_duplicates, 1);
    assert_eq!(report.source_topic_session_count, 0);
    assert_eq!(report.target_topic_session_count_before, 0);
    assert_eq!(report.target_topic_session_count_after, 0);
    assert_eq!(report.source_topic_graph_edge_count, 0);
    assert_eq!(report.target_topic_graph_edge_count_before, 0);
    assert_eq!(report.target_topic_graph_edge_count_after, 0);
    assert!(report.approvals_added_to_target.is_empty());
    assert!(report.pending_added_to_target.is_empty());
    assert_eq!(report.new_history_entries_to_append.len(), 1);
    assert!(report.new_history_entries_to_append[0].contains("fork-only delta"));
    assert_eq!(report.duplicate_history_entries_skipped.len(), 1);
    assert!(report.duplicate_history_entries_skipped[0].contains("shared base"));
    assert_eq!(
        report.target_last_user_intent_summary_before.as_deref(),
        Some("shared base")
    );
    assert_eq!(
        report.source_last_user_intent_summary.as_deref(),
        Some("fork-only delta")
    );
    assert_eq!(
        report.merged_last_user_intent_summary.as_deref(),
        Some("fork-only delta")
    );
}

#[tokio::test]
async fn preview_merge_session_surfaces_topic_state_plan() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("mainline")
        .expect("mainline session switch should succeed");
    runtime
        .run_demo_turn("mainline planning")
        .await
        .expect("mainline turn should succeed");
    runtime
        .route_topics("mainline", Some("mainline planning"), 4, 4, 4, 1)
        .expect("mainline route should succeed");
    runtime
        .run_demo_turn_in_session("feature", "hello adaptive memory")
        .await
        .expect("feature first turn should succeed");
    runtime
        .route_topics("feature", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("feature first route should succeed");
    runtime
        .run_demo_turn_in_session("feature", "rust worker pipeline")
        .await
        .expect("feature second turn should succeed");
    runtime
        .route_topics("feature", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("feature second route should succeed");
    runtime
        .route_topics(
            "feature",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("feature mixed route should succeed");

    let report = runtime
        .preview_merge_session("feature", "mainline", MergeOptions::default())
        .expect("merge preview should succeed");

    assert_eq!(report.source_topic_session_count, 2);
    assert_eq!(report.target_topic_session_count_before, 1);
    assert_eq!(report.target_topic_session_count_after, 3);
    assert_eq!(report.source_topic_graph_edge_count, 2);
    assert_eq!(report.target_topic_graph_edge_count_before, 0);
    assert_eq!(report.target_topic_graph_edge_count_after, 2);
}

#[tokio::test]
async fn merge_session_deduplicates_shared_history_from_forked_source() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("beta")
        .expect("beta session switch should succeed");
    runtime
        .run_demo_turn("shared base")
        .await
        .expect("beta base turn should succeed");
    runtime
        .run_demo_turn("hello adaptive memory")
        .await
        .expect("beta topic turn should succeed");
    runtime
        .route_topics("beta", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("beta first route should succeed");
    runtime
        .run_demo_turn("rust worker pipeline")
        .await
        .expect("beta second topic turn should succeed");
    runtime
        .route_topics("beta", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("beta second route should succeed");
    runtime
        .route_topics(
            "beta",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("beta mixed route should succeed");
    runtime
        .fork_session("beta", "beta-fork")
        .expect("beta fork should succeed");
    runtime
        .run_demo_turn_in_session("beta-fork", "fork-only delta")
        .await
        .expect("fork delta turn should succeed");

    let merge_result = runtime
        .merge_session("beta-fork", "beta", MergeOptions::default())
        .expect("merge should succeed");
    assert_eq!(merge_result.appended_history_entries, 1);
    assert_eq!(merge_result.skipped_duplicate_history_entries, 3);
    assert_eq!(merge_result.target_session_id, "beta");
    assert_eq!(merge_result.target_title_after, "Hepta session beta");
    assert_eq!(merge_result.target_model_after.model, "demo-chat");
    assert_eq!(merge_result.source_topic_session_count, 2);
    assert_eq!(merge_result.target_topic_session_count_before, 2);
    assert_eq!(merge_result.target_topic_session_count_after, 2);
    assert_eq!(merge_result.source_topic_graph_edge_count, 2);
    assert_eq!(merge_result.target_topic_graph_edge_count_before, 2);
    assert_eq!(merge_result.target_topic_graph_edge_count_after, 2);

    let history = runtime
        .history(Some("beta"), 10)
        .expect("beta history should load");
    assert_eq!(history.len(), 4);
    assert_eq!(history[0].input, "fork-only delta");
    assert_eq!(history[1].input, "rust worker pipeline");
    assert_eq!(history[2].input, "hello adaptive memory");
    assert_eq!(history[3].input, "shared base");

    let beta_topic_sessions = runtime
        .topic_sessions_for_surface("beta")
        .expect("beta topic sessions should load");
    assert_eq!(beta_topic_sessions.len(), 2);
    assert!(beta_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:beta"
            && !topic_session.graph_edges.is_empty()
    }));
    assert!(beta_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:beta:rust-worker-pipeline"
    }));
}

#[tokio::test]
async fn merge_session_materializes_fresh_active_target_session() {
    let runtime = RuntimeKernel::new();
    runtime
        .run_demo_turn_in_session("feature", "feature base")
        .await
        .expect("feature base turn should succeed");

    let merge_result = runtime
        .merge_session("feature", "session-main", MergeOptions::default())
        .expect("merge into fresh active target should succeed");

    assert_eq!(merge_result.target_session_id, "session-main");
    assert_eq!(
        merge_result.target_title_after,
        "Hepta session session-main"
    );
    assert_eq!(merge_result.target_model_after.model, "demo-chat");
    assert_eq!(merge_result.appended_history_entries, 1);
    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "session-main"
    );

    let session_main = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "session-main")
        .expect("session-main should exist");
    assert_eq!(
        session_main.last_user_intent_summary.as_deref(),
        Some("feature base")
    );

    let history = runtime
        .history(Some("session-main"), 10)
        .expect("session-main history should load");
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].input, "feature base");
}

#[tokio::test]
async fn merge_session_rebases_unrelated_topic_graph_state_into_target_namespace() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("mainline")
        .expect("mainline session switch should succeed");
    runtime
        .run_demo_turn("mainline planning")
        .await
        .expect("mainline turn should succeed");
    runtime
        .route_topics("mainline", Some("mainline planning"), 4, 4, 4, 1)
        .expect("mainline route should succeed");
    runtime
        .run_demo_turn_in_session("feature", "hello adaptive memory")
        .await
        .expect("feature first turn should succeed");
    runtime
        .route_topics("feature", Some("hello adaptive memory"), 4, 4, 4, 1)
        .expect("feature first route should succeed");
    runtime
        .run_demo_turn_in_session("feature", "rust worker pipeline")
        .await
        .expect("feature second turn should succeed");
    runtime
        .route_topics("feature", Some("rust worker pipeline"), 6, 6, 6, 1)
        .expect("feature second route should succeed");
    runtime
        .route_topics(
            "feature",
            Some("hello adaptive memory and rust worker pipeline"),
            8,
            8,
            8,
            2,
        )
        .expect("feature mixed route should succeed");

    let merge_result = runtime
        .merge_session("feature", "mainline", MergeOptions::default())
        .expect("merge should succeed");
    assert_eq!(merge_result.source_topic_session_count, 2);
    assert_eq!(merge_result.target_topic_session_count_before, 1);
    assert_eq!(merge_result.target_topic_session_count_after, 3);
    assert_eq!(merge_result.source_topic_graph_edge_count, 2);
    assert_eq!(merge_result.target_topic_graph_edge_count_before, 0);
    assert_eq!(merge_result.target_topic_graph_edge_count_after, 2);

    let mainline_topic_sessions = runtime
        .topic_sessions_for_surface("mainline")
        .expect("mainline topic sessions should load");
    assert!(mainline_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:mainline"
            && topic_session.topic_id.0 == "topic-mainline"
    }));
    assert!(mainline_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id == "topic-session-bootstrap:mainline:feature"
            && topic_session.topic_id.0 == "topic-mainline-feature"
    }));
    assert!(mainline_topic_sessions.iter().any(|topic_session| {
        topic_session.topic_session_id
            == "topic-session-bootstrap:mainline:feature:rust-worker-pipeline"
            && topic_session.topic_id.0 == "topic-mainline-feature-rust-worker-pipeline"
    }));
    assert!(
        runtime
            .topic_graph_state
            .lock()
            .expect("topic graph state lock should succeed")
            .edges
            .iter()
            .any(|record| {
                record.source_topic_session_id == "topic-session-bootstrap:mainline:feature"
                    && record.edge.target_topic_session_id
                        == "topic-session-bootstrap:mainline:feature:rust-worker-pipeline"
            })
    );
}

#[tokio::test]
async fn merge_session_can_adopt_model_title_and_delete_source() {
    let runtime = RuntimeKernel::new();
    runtime
        .switch_session("mainline")
        .expect("mainline session switch should succeed");
    runtime
        .rename_active_session("Mainline workspace")
        .expect("mainline rename should succeed");
    runtime
        .run_demo_turn_in_session("feature", "feature base")
        .await
        .expect("feature base turn should succeed");
    runtime
        .switch_session("feature")
        .expect("feature session switch should succeed");
    runtime
        .rename_active_session("Feature workspace")
        .expect("feature rename should succeed");
    runtime
        .switch_model("demo/demo-creative")
        .expect("feature model switch should succeed");

    let preview = runtime
        .preview_merge_session(
            "feature",
            "mainline",
            MergeOptions {
                adopt_model: true,
                adopt_title: true,
                delete_source: true,
            },
        )
        .expect("merge preview should succeed");
    assert_eq!(preview.target_title_after, "Feature workspace");
    assert_eq!(preview.target_model_after.model, "demo-creative");
    assert!(preview.source_deleted_after_merge);

    let merge_result = runtime
        .merge_session(
            "feature",
            "mainline",
            MergeOptions {
                adopt_model: true,
                adopt_title: true,
                delete_source: true,
            },
        )
        .expect("merge should succeed");
    assert!(merge_result.options.adopt_title);
    assert!(merge_result.options.adopt_model);
    assert!(merge_result.options.delete_source);
    assert_eq!(merge_result.target_title_after, "Feature workspace");
    assert_eq!(merge_result.target_model_after.model, "demo-creative");
    assert!(merge_result.source_deleted_after_merge);

    let mainline = runtime
        .sessions()
        .expect("sessions should load")
        .into_iter()
        .find(|session| session.session_id == "mainline")
        .expect("mainline session should exist");
    assert_eq!(mainline.title, "Feature workspace");
    assert_eq!(mainline.model.model, "demo-creative");
    assert_eq!(
        runtime
            .active_session_id()
            .expect("active session should load"),
        "mainline"
    );
    assert!(
        runtime
            .sessions()
            .expect("sessions should load")
            .into_iter()
            .all(|session| session.session_id != "feature")
    );
}

mod architecture_v2_exact_safety_tests {
    use super::*;
    use crate::SafetyGateClient;

    include!("architecture_v2_exact_safety_support.rs");
    include!("architecture_v2_exact_safety.rs");
}

mod architecture_v2_execution_lease_tests {
    use super::*;
    use crate::ExecutionBus;
    use crate::SafetyGateClient;

    include!("architecture_v2_execution_lease.rs");
}

mod architecture_v2_outcome_receipt_tests {
    include!("architecture_v2_terminal_outcome_support.rs");
    include!("architecture_v2_terminal_outcome.rs");
}

mod architecture_v2_outcome_flow_tests {
    include!("architecture_v2_outcome_flow.rs");
}

mod architecture_v2_resource_reservation_tests {
    include!("architecture_v2_resource_reservation.rs");
}

mod architecture_v2_capability_descriptor_tests {
    include!("architecture_v2_capability_descriptor.rs");
}

mod architecture_v2_symlink_reservation_tests {
    use super::tempfile;

    include!("architecture_v2_symlink_reservation.rs");
}

mod architecture_v2_dispatch_selector_tests {
    include!("architecture_v2_dispatch_selector.rs");
}

mod architecture_v2_process_reservation_tests {
    include!("architecture_v2_process_reservation.rs");
}

mod architecture_v2_native_mutation_tests {
    use super::tempfile;

    include!("architecture_v2_native_mutation.rs");
}

mod architecture_v2_maintenance_mutation_tests {
    use super::tempfile;

    include!("architecture_v2_maintenance_mutation.rs");
}

mod architecture_v2_process_control_tests {
    use super::tempfile;

    include!("architecture_v2_process_control.rs");
}

mod architecture_v2_provider_idempotency_tests {
    include!("architecture_v2_provider_idempotency.rs");
}

mod architecture_v2_provider_effect_tests {
    use super::tempfile;

    include!("architecture_v2_provider_effect.rs");
}

mod architecture_v2_sealed_read_tests {
    use super::tempfile;

    include!("architecture_v2_sealed_read.rs");
}

#[cfg(unix)]
#[test]
fn durable_runtime_hydrates_session_state_on_reopen() {
    let root = tempfile::tempdir().expect("tempdir");
    let outcome_path = root.path().join("outcomes.sqlite3");
    let state_path = root.path().join("runtime-state.json");
    let runtime = RuntimeKernel::bootstrap_with_durable_outcomes_and_state(
        &outcome_path,
        hepta_memory::DurableIntegrityKey::from_bytes([11; 32]),
        &state_path,
        hepta_memory::DurableIntegrityKey::from_bytes([12; 32]),
    )
    .expect("bootstrap durable runtime");
    runtime
        .switch_session("durable-session")
        .expect("create durable session");
    drop(runtime);

    let recovered = RuntimeKernel::open_with_durable_outcomes_and_state(
        &outcome_path,
        hepta_memory::DurableIntegrityKey::from_bytes([11; 32]),
        &state_path,
        hepta_memory::DurableIntegrityKey::from_bytes([12; 32]),
    )
    .expect("open durable runtime");
    assert!(
        recovered
            .sessions()
            .expect("list recovered sessions")
            .iter()
            .any(|session| session.session_id == "durable-session")
    );
}
