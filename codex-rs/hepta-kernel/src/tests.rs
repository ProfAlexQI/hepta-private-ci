use super::*;
use std::path::Path;
use std::path::PathBuf;

const TEST_NOW_MS: u64 = 1_000_000;

#[test]
fn kernel_native_post_route_specs_cover_real_handler_policy() {
    let specs = hepta_kernel_native_post_plan_route_specs();
    assert_eq!(specs.len(), 12);

    let action = specs
        .iter()
        .find(|spec| spec.pattern == "/api/actions/<action>")
        .expect("action route");
    assert_eq!(
        hepta_kernel_native_post_plan_parameter(action, "/api/actions/reload"),
        Some(Some("reload"))
    );
    assert_eq!(
        hepta_kernel_native_post_plan_parameter(action, "/api/actions/"),
        None
    );

    let task_publish = specs
        .iter()
        .find(|spec| spec.pattern == "/api/tasks/publish")
        .expect("task publish route");
    assert_eq!(
        hepta_kernel_native_post_plan_parameter(task_publish, "/api/tasks/publish"),
        Some(None)
    );
    assert!(task_publish.confirmation_required_for_real_mutation);
    assert!(!hepta_kernel_native_post_plan_kind_has_real_handler(
        task_publish.plan_kind
    ));
    assert!(!hepta_kernel_native_post_plan_kind_has_real_handler(
        "approval_apply"
    ));
    assert!(!hepta_kernel_native_post_plan_kind_has_real_handler(
        "readonly_command"
    ));
    assert_eq!(
        HEPTA_KERNEL_NATIVE_POST_COMPATIBILITY_HARNESS_PLAN_KINDS,
        ["approval_apply", "task_publish", "chat_send"]
    );
}

#[test]
fn kernel_native_post_body_schema_covers_real_handler_input_contracts() {
    let task_publish = hepta_kernel_native_post_body_schema("task_publish", true);
    assert_eq!(task_publish.schema_id, "hepta.post.task_publish.v1");
    assert!(task_publish.body_required_for_real_handler);
    assert_eq!(task_publish.content_type, "application/json");
    assert!(task_publish.required_fields.contains(&"task"));
    assert!(task_publish.required_fields.contains(&"confirm"));
    assert!(task_publish.optional_fields.contains(&"idempotency_key"));
    assert!(task_publish.body_read_during_plan);
    assert!(!task_publish.raw_body_exposed);
    assert!(!task_publish.raw_field_values_exposed);

    let readonly = hepta_kernel_native_post_body_schema("readonly_command", false);
    assert_eq!(readonly.schema_id, "hepta.post.readonly_command.v1");
    assert!(!readonly.body_required_for_real_handler);
    assert!(readonly.required_fields.is_empty());

    let unknown = hepta_kernel_native_post_body_schema("not_registered", false);
    assert_eq!(unknown.schema_id, "hepta.post.unknown.v1");
    assert!(!unknown.body_required_for_real_handler);
    assert_eq!(unknown.optional_fields, vec!["dry_run"]);
}

#[test]
fn kernel_native_post_body_admission_redacts_and_gates_real_handler_input() {
    let task_publish = hepta_kernel_native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let schema = hepta_kernel_native_post_body_schema(task_publish.plan_kind, true);
    let admission = hepta_kernel_native_post_body_admission(
        task_publish,
        &schema,
        Some(r#"{"task":"ship","confirm":"yes","dry_run":true,"idempotency_key":"same-key"}"#),
    );

    assert_eq!(admission.admission_status, "ready_for_real_handler");
    assert!(admission.ready_for_real_handler_input);
    assert!(admission.confirm_field_truthy);
    assert!(admission.dry_run_first_satisfied);
    assert!(admission.idempotency_key_present);
    assert!(
        admission
            .idempotency_key_fingerprint
            .as_deref()
            .is_some_and(|fingerprint| fingerprint.starts_with("sha256:"))
    );
    assert!(!admission.raw_body_exposed);
    assert!(!admission.raw_field_values_exposed);

    let missing_key = hepta_kernel_native_post_body_admission(
        task_publish,
        &schema,
        Some(r#"{"task":"ship","confirm":true,"dry_run":true}"#),
    );
    assert_eq!(missing_key.admission_status, "idempotency_key_missing");

    let plan_only = hepta_kernel_native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "readonly_command")
        .expect("readonly command spec");
    let plan_schema = hepta_kernel_native_post_body_schema(plan_only.plan_kind, true);
    let plan_admission = hepta_kernel_native_post_body_admission(
        plan_only,
        &plan_schema,
        Some(r#"{"command_args":["status"]}"#),
    );
    assert_eq!(plan_admission.admission_status, "validated_plan_input");
    assert!(!plan_admission.idempotency_key_required);
}

#[test]
fn kernel_native_post_evidence_contracts_gate_real_handler_readiness() {
    let chat_send = hepta_kernel_native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "chat_send")
        .expect("chat send spec");
    let schema = hepta_kernel_native_post_body_schema(chat_send.plan_kind, true);
    let admission = hepta_kernel_native_post_body_admission(
        chat_send,
        &schema,
        Some(
            r#"{"chat_id":"c1","message":"hello","confirm":true,"dry_run":true,"idempotency_key":"idem"}"#,
        ),
    );
    let confirmation = hepta_kernel_native_post_confirmation_contract(chat_send);
    let rollback = hepta_kernel_native_post_rollback_contract();
    let idempotency = hepta_kernel_native_post_idempotency_evidence(chat_send, &admission);
    let audit =
        hepta_kernel_native_post_audit_event_contract(chat_send, &schema, &admission, &idempotency);

    assert!(confirmation.real_mutation_requires_confirmation);
    assert_eq!(confirmation.accepted_confirmation_field, Some("confirm"));
    assert!(!confirmation.raw_confirmation_payload_exposed);
    assert!(rollback.current_plan_noop);
    assert!(rollback.real_handler_requires_rollback_contract);
    assert!(idempotency.required);
    assert!(idempotency.key_shape_valid);
    assert!(idempotency.duplicate_suppression_required);
    assert!(audit.required);
    assert!(audit.ready_for_real_handler);
    assert!(!audit.current_plan_emits_audit_event);
    assert!(!audit.raw_idempotency_key_exposed);
}

#[test]
fn kernel_native_post_execution_admission_requires_matching_scope() {
    let chat_send = hepta_kernel_native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "chat_send")
        .expect("chat send spec");
    let schema = hepta_kernel_native_post_body_schema(chat_send.plan_kind, true);
    let admission = hepta_kernel_native_post_body_admission(
        chat_send,
        &schema,
        Some(
            r#"{"chat_id":"c1","message":"hello","confirm":true,"dry_run":true,"idempotency_key":"idem"}"#,
        ),
    );
    let idempotency = hepta_kernel_native_post_idempotency_evidence(chat_send, &admission);
    let audit =
        hepta_kernel_native_post_audit_event_contract(chat_send, &schema, &admission, &idempotency);

    let mismatched = hepta_kernel_native_post_execution_admission_with_scope(
        chat_send,
        &admission,
        &idempotency,
        &audit,
        true,
        true,
        Some("task_publish"),
    );
    assert_eq!(mismatched.admission_status, "blocked");
    assert_eq!(mismatched.blocked_reason, "real_handler_not_wired");
    assert!(!mismatched.current_plan_executes_real_handler);
    assert!(!hepta_kernel_native_post_duplicate_check_required(
        &mismatched,
        &idempotency
    ));
    assert_eq!(
        mismatched.handler_scope_env,
        HEPTA_KERNEL_NATIVE_POST_REAL_HANDLER_SCOPE_ENV
    );

    let matched = hepta_kernel_native_post_execution_admission_with_scope(
        chat_send,
        &admission,
        &idempotency,
        &audit,
        true,
        true,
        Some("task_publish, chat_send"),
    );
    assert_eq!(matched.admission_status, "blocked");
    assert_eq!(matched.blocked_reason, "real_handler_not_wired");
    assert!(!matched.current_plan_executes_real_handler);
    assert!(matched.handler_scope_matches);
    assert!(!hepta_kernel_native_post_duplicate_check_required(
        &matched,
        &idempotency
    ));
    assert!(!hepta_kernel_native_post_rate_limit_check_required(
        &matched, true, false, None
    ));
    assert!(!hepta_kernel_native_post_rate_limit_check_required(
        &matched, true, true, None
    ));
    assert!(!hepta_kernel_native_post_rate_limit_check_required(
        &matched,
        true,
        false,
        Some("native_post_idempotency_check_failed")
    ));
    assert!(!hepta_kernel_native_post_store_capacity_check_required(
        &matched, true, false, None, false, None
    ));
    assert!(!hepta_kernel_native_post_store_capacity_check_required(
        &matched, true, false, None, true, None
    ));
    assert!(hepta_kernel_native_post_store_write_attempt_required(
        true, true, None
    ));
    assert!(!hepta_kernel_native_post_store_write_attempt_required(
        true, false, None
    ));

    let mut no_key_idempotency = idempotency.clone();
    no_key_idempotency.key_fingerprint = None;
    assert!(!hepta_kernel_native_post_duplicate_check_required(
        &matched,
        &no_key_idempotency
    ));
}

#[test]
fn kernel_native_post_real_handler_scope_selection_uses_kernel_registry() {
    let selected = hepta_kernel_native_post_real_handler_scope_selected_kinds(Some(
        "approval_apply chat_send",
    ));

    assert!(selected.is_empty());
    assert_eq!(
        hepta_kernel_native_post_real_handler_scope_single_selected_kind(Some("task_publish")),
        None
    );
    assert_eq!(
        hepta_kernel_native_post_real_handler_scope_single_selected_kind(Some(
            "approval_apply chat_send"
        )),
        None
    );
    assert_eq!(
        hepta_kernel_native_post_real_handler_scope_single_selected_kind(None),
        None
    );
    assert!(hepta_kernel_native_post_real_handler_scope_matches(
        "chat_send",
        Some("task_publish,chat_send")
    ));
    assert!(!hepta_kernel_native_post_real_handler_scope_matches(
        "approval_apply",
        Some("task_publish,chat_send")
    ));
}

#[test]
fn kernel_native_post_execution_readiness_report_stays_side_effect_free() {
    let report =
        hepta_kernel_native_post_execution_readiness_report(false, Some("task_publish chat_send"));

    assert_eq!(report.status, "ready");
    assert_eq!(
        report.endpoint,
        HEPTA_KERNEL_NATIVE_POST_EXECUTION_READINESS_ENDPOINT
    );
    assert_eq!(report.post_route_count, 12);
    assert_eq!(report.real_handler_candidate_count, 3);
    assert_eq!(report.real_handler_implemented_count, 0);
    assert_eq!(report.real_handler_ready_count, 0);
    assert_eq!(report.selected_handler_count, 0);
    assert!(report.all_real_handlers_blocked);
    assert!(!report.real_handler_gate_enabled);
    assert!(!report.external_side_effects);
    assert!(!report.gateway_mutation_performed);
    assert!(report.routes.iter().any(|route| {
        route.plan_kind == "task_publish"
            && !route.ready_for_real_handler_wiring
            && route.blocked_reason == "real_handler_not_wired"
    }));
}

#[test]
fn kernel_native_post_activation_plan_requires_dual_gate_and_single_scope() {
    let gated = hepta_kernel_native_post_activation_plan_report(
        false,
        false,
        Some("task_publish"),
        true,
        true,
        true,
        true,
    );
    assert_eq!(gated.status, "attention");
    assert_eq!(
        gated.endpoint,
        HEPTA_KERNEL_NATIVE_POST_ACTIVATION_PLAN_ENDPOINT
    );
    assert!(!gated.activation_preflight_ready);
    assert!(!gated.activation_currently_enabled);
    assert_eq!(
        gated.activation_blocked_reason,
        "real_handler_not_implemented"
    );
    assert!(!gated.rollback_ready);
    assert!(gated.selected_handler_kinds.is_empty());
    assert_eq!(gated.required_gates.len(), 3);
    assert!(!gated.external_side_effects);
    assert!(!gated.gateway_mutation_performed);

    let live_ready = hepta_kernel_native_post_activation_plan_report(
        true,
        true,
        Some("task_publish"),
        true,
        true,
        true,
        true,
    );
    assert!(!live_ready.activation_currently_enabled);
    assert_eq!(
        live_ready.activation_blocked_reason,
        "real_handler_not_implemented"
    );

    let ambiguous_scope = hepta_kernel_native_post_activation_plan_report(
        true,
        true,
        Some("task_publish chat_send"),
        true,
        true,
        true,
        true,
    );
    assert!(!ambiguous_scope.activation_currently_enabled);
    assert_eq!(
        ambiguous_scope.activation_blocked_reason,
        "real_handler_not_implemented"
    );
}

#[test]
fn kernel_native_post_execution_store_record_binds_redacted_evidence() {
    let task_publish = hepta_kernel_native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let schema = hepta_kernel_native_post_body_schema(task_publish.plan_kind, true);
    let admission = hepta_kernel_native_post_body_admission(
        task_publish,
        &schema,
        Some(r#"{"task":"secret","confirm":true,"dry_run":true,"idempotency_key":"secret-idem"}"#),
    );
    let idempotency = hepta_kernel_native_post_idempotency_evidence(task_publish, &admission);
    let audit = hepta_kernel_native_post_audit_event_contract(
        task_publish,
        &schema,
        &admission,
        &idempotency,
    );

    let record = hepta_kernel_native_post_execution_store_record(
        task_publish,
        &schema,
        &admission,
        &idempotency,
        &audit,
        true,
        42,
    );

    assert_eq!(record.schema_id, "hepta.post.execution_store_record.v1");
    assert_eq!(record.recorded_at_unix_ms, 42);
    assert_eq!(record.plan_kind, "task_publish");
    assert_eq!(record.body_schema_id, "hepta.post.task_publish.v1");
    assert!(record.idempotency_key_required);
    assert!(record.idempotency_key_present);
    assert!(record.idempotency_key_redacted);
    assert!(
        record
            .idempotency_key_fingerprint
            .as_deref()
            .is_some_and(|fingerprint| fingerprint.starts_with("sha256:"))
    );
    assert!(record.duplicate_suppression_required);
    assert!(record.audit_event_ready_for_real_handler);
    assert!(record.current_plan_executes_real_handler);
    assert!(!record.raw_request_body_exposed);
    assert!(!record.raw_idempotency_key_exposed);
}

#[test]
fn kernel_native_post_execution_store_record_json_line_serializes_redacted_record() {
    let task_publish = hepta_kernel_native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let schema = hepta_kernel_native_post_body_schema(task_publish.plan_kind, true);
    let admission = hepta_kernel_native_post_body_admission(
        task_publish,
        &schema,
        Some(r#"{"task":"secret","confirm":true,"idempotency_key":"secret-idem"}"#),
    );
    let idempotency = hepta_kernel_native_post_idempotency_evidence(task_publish, &admission);
    let audit = hepta_kernel_native_post_audit_event_contract(
        task_publish,
        &schema,
        &admission,
        &idempotency,
    );
    let record = hepta_kernel_native_post_execution_store_record(
        task_publish,
        &schema,
        &admission,
        &idempotency,
        &audit,
        true,
        42,
    );

    let line = hepta_kernel_native_post_execution_store_record_json_line(&record)
        .expect("record serializes");
    let projected_append_bytes =
        hepta_kernel_native_post_execution_store_record_projected_append_bytes(&record)
            .expect("project append bytes");
    let value = serde_json::from_str::<Value>(&line).expect("record JSON parses");

    assert_eq!(value["schema_id"], "hepta.post.execution_store_record.v1");
    assert_eq!(value["plan_kind"], "task_publish");
    assert_eq!(value["idempotency_key_redacted"], true);
    assert_eq!(value["raw_request_body_exposed"], false);
    assert_eq!(value["raw_idempotency_key_exposed"], false);
    assert!(
        value["idempotency_key_fingerprint"]
            .as_str()
            .is_some_and(|fingerprint| fingerprint.starts_with("sha256:"))
    );
    assert!(!line.contains("secret-idem"));
    assert_eq!(projected_append_bytes, line.len() as u64 + 1);
}

#[test]
fn kernel_native_post_real_handler_harness_summarizes_gateway_observations() {
    let task_publish = hepta_kernel_native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let schema = hepta_kernel_native_post_body_schema(task_publish.plan_kind, true);
    let admission = hepta_kernel_native_post_body_admission(
        task_publish,
        &schema,
        Some(r#"{"task":"ship","confirm":true,"dry_run":true,"idempotency_key":"idem-1"}"#),
    );
    let idempotency = hepta_kernel_native_post_idempotency_evidence(task_publish, &admission);
    let audit = hepta_kernel_native_post_audit_event_contract(
        task_publish,
        &schema,
        &admission,
        &idempotency,
    );
    let execution = hepta_kernel_native_post_execution_admission_with_scope(
        task_publish,
        &admission,
        &idempotency,
        &audit,
        true,
        true,
        Some("task_publish"),
    );
    let write_report = HeptaKernelNativePostExecutionStoreWriteReport {
        status: "persisted",
        root: ".hepta/native-post-execution".to_string(),
        written_file_count: 4,
        written_files: vec!["idempotency.jsonl".to_string()],
        raw_request_body_exposed: false,
        raw_field_values_exposed: false,
        raw_idempotency_key_exposed: false,
        raw_audit_payload_exposed: false,
    };

    let recorded = hepta_kernel_native_post_real_handler_harness_from_observation(
        task_publish,
        &execution,
        HeptaKernelNativePostRealHandlerObservation {
            duplicate_check_performed: true,
            duplicate_found: false,
            duplicate_check_error: None,
            rate_limit_check_performed: true,
            rate_limited: false,
            rate_limit_window_ms: 1_000,
            rate_limit_check_error: None,
            capacity_check_performed: true,
            store_capacity_ok: true,
            store_capacity_check_error: None,
            store_write_attempted: true,
            store_write_succeeded: true,
            store_write_report: Some(write_report),
            store_write_error: None,
        },
    );

    assert_eq!(recorded.status, "not_implemented");
    assert_eq!(recorded.handler_kind, "task_publish");
    assert!(recorded.dual_gate_satisfied);
    assert!(recorded.handler_scope_matches);
    assert!(!recorded.duplicate_check_performed);
    assert!(!recorded.rate_limit_check_performed);
    assert!(!recorded.capacity_check_performed);
    assert!(!recorded.store_write_attempted);
    assert!(!recorded.store_write_succeeded);
    assert!(recorded.store_write_report.is_none());
    assert!(!recorded.raw_request_body_exposed);
    assert!(!recorded.gateway_mutation_performed);

    let duplicate = hepta_kernel_native_post_real_handler_harness_from_observation(
        task_publish,
        &execution,
        HeptaKernelNativePostRealHandlerObservation {
            duplicate_check_performed: true,
            duplicate_found: true,
            duplicate_check_error: None,
            rate_limit_check_performed: false,
            rate_limited: false,
            rate_limit_window_ms: 1_000,
            rate_limit_check_error: None,
            capacity_check_performed: false,
            store_capacity_ok: true,
            store_capacity_check_error: None,
            store_write_attempted: false,
            store_write_succeeded: false,
            store_write_report: None,
            store_write_error: None,
        },
    );

    assert_eq!(duplicate.status, "not_implemented");
    assert!(!duplicate.duplicate_suppressed);
    assert!(!duplicate.duplicate_check_performed);
    assert!(!duplicate.store_write_attempted);
}

#[test]
fn kernel_native_post_plan_response_assembles_redacted_report() {
    let task_publish = hepta_kernel_native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let schema = hepta_kernel_native_post_body_schema(task_publish.plan_kind, true);
    let admission = hepta_kernel_native_post_body_admission(
        task_publish,
        &schema,
        Some(r#"{"task":"ship","confirm":true,"dry_run":true,"idempotency_key":"idem-1"}"#),
    );
    let idempotency = hepta_kernel_native_post_idempotency_evidence(task_publish, &admission);
    let audit = hepta_kernel_native_post_audit_event_contract(
        task_publish,
        &schema,
        &admission,
        &idempotency,
    );
    let execution = hepta_kernel_native_post_execution_admission_with_scope(
        task_publish,
        &admission,
        &idempotency,
        &audit,
        true,
        true,
        Some("task_publish"),
    );
    let harness = hepta_kernel_native_post_real_handler_harness(
        task_publish,
        &execution,
        true,
        false,
        None,
        true,
        false,
        1_000,
        None,
        true,
        true,
        None,
        true,
        true,
        None,
        None,
    );
    let store_effect_projection =
        hepta_kernel_native_post_store_effect_projection(idempotency, audit, &harness);

    let response = hepta_kernel_native_post_plan_response(
        task_publish,
        true,
        Some("redacted-param".len()),
        schema,
        admission,
        hepta_kernel_native_post_confirmation_contract(task_publish),
        hepta_kernel_native_post_rollback_contract(),
        store_effect_projection.idempotency_evidence,
        store_effect_projection.audit_event_contract,
        execution,
        harness,
    );

    assert_eq!(response.status, "confirm_required");
    assert_eq!(response.method, "POST");
    assert_eq!(response.pattern, "/api/tasks/publish");
    assert_eq!(response.parameter_length, Some("redacted-param".len()));
    assert!(response.parameter_redacted);
    assert!(response.side_effect_free);
    assert!(!response.real_handler_harness.store_write_attempted);
    assert!(!response.idempotency_evidence.current_plan_lookup_performed);
    assert!(!response.idempotency_evidence.current_plan_store_written);
    assert!(!response.audit_event_contract.current_plan_emits_audit_event);
    assert!(
        !response
            .audit_event_contract
            .current_plan_persists_audit_event
    );
    assert!(!response.raw_request_body_exposed);
    assert!(!response.raw_parameter_exposed);
}

#[test]
fn kernel_native_post_execution_stores_report_summarizes_file_statuses() {
    let stores = vec![
        HeptaKernelNativePostExecutionStoreFileStatus {
            store_kind: "idempotency",
            schema_id: "hepta.post.idempotency_entry.v1",
            filename: "idempotency.jsonl",
            path: ".hepta/native-post-execution/idempotency.jsonl".to_string(),
            exists: true,
            bytes: 10,
            max_bytes: 100,
            bytes_within_limit: true,
            append_only: true,
            jsonl: true,
            jsonl_readable: true,
            jsonl_valid: true,
            line_count: 1,
            max_lines: 10,
            line_count_within_limit: true,
            valid_json_line_count: 1,
            invalid_json_line_count: 0,
            raw_body_exposed: false,
            raw_field_values_exposed: false,
            raw_idempotency_key_exposed: false,
        },
        HeptaKernelNativePostExecutionStoreFileStatus {
            store_kind: "rollback",
            schema_id: "hepta.post.rollback_anchor.v1",
            filename: "rollback.jsonl",
            path: ".hepta/native-post-execution/rollback.jsonl".to_string(),
            exists: true,
            bytes: 12,
            max_bytes: 100,
            bytes_within_limit: true,
            append_only: true,
            jsonl: true,
            jsonl_readable: true,
            jsonl_valid: true,
            line_count: 2,
            max_lines: 10,
            line_count_within_limit: true,
            valid_json_line_count: 2,
            invalid_json_line_count: 0,
            raw_body_exposed: false,
            raw_field_values_exposed: false,
            raw_idempotency_key_exposed: false,
        },
    ];

    let report = hepta_kernel_native_post_execution_stores_report(
        ".hepta/native-post-execution".to_string(),
        true,
        true,
        100,
        10,
        stores,
    );

    assert_eq!(report.status, "ready");
    assert_eq!(
        report.endpoint,
        HEPTA_KERNEL_NATIVE_POST_EXECUTION_STORES_ENDPOINT
    );
    assert_eq!(report.store_file_count, 2);
    assert_eq!(report.existing_file_count, 2);
    assert_eq!(report.total_bytes, 22);
    assert_eq!(report.total_line_count, 3);
    assert!(report.store_jsonl_valid);
    assert!(report.store_capacity_ok);
    assert!(hepta_kernel_native_post_execution_store_contracts_ready(
        &report
    ));
    assert!(!report.raw_request_body_exposed);

    let mut blocked_report = report.clone();
    blocked_report.store_capacity_ok = false;
    assert!(!hepta_kernel_native_post_execution_store_contracts_ready(
        &blocked_report
    ));
}

#[test]
fn kernel_native_post_execution_store_limits_freeze_public_defaults() {
    let limits = HeptaKernelNativePostExecutionStoreLimits {
        max_store_bytes: DEFAULT_HEPTA_KERNEL_NATIVE_POST_STORE_MAX_BYTES,
        max_store_lines: DEFAULT_HEPTA_KERNEL_NATIVE_POST_STORE_MAX_LINES,
        rate_limit_window_ms: DEFAULT_HEPTA_KERNEL_NATIVE_POST_RATE_LIMIT_WINDOW_MS,
    };

    assert_eq!(
        HEPTA_KERNEL_NATIVE_POST_STORE_MAX_BYTES_ENV,
        "HEPTA_NATIVE_POST_STORE_MAX_BYTES"
    );
    assert_eq!(
        HEPTA_KERNEL_NATIVE_POST_STORE_MAX_LINES_ENV,
        "HEPTA_NATIVE_POST_STORE_MAX_LINES"
    );
    assert_eq!(
        HEPTA_KERNEL_NATIVE_POST_RATE_LIMIT_WINDOW_MS_ENV,
        "HEPTA_NATIVE_POST_RATE_LIMIT_WINDOW_MS"
    );
    assert_eq!(
        DEFAULT_HEPTA_KERNEL_NATIVE_POST_EXECUTION_STORE_DIR,
        ".hepta/native-post-execution"
    );
    assert_eq!(
        HEPTA_KERNEL_NATIVE_POST_EXECUTION_STORES_ENDPOINT,
        "/api/native-post-execution-stores"
    );
    assert_eq!(
        HEPTA_KERNEL_NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT,
        "/api/native-post-rollout-evidence"
    );
    assert_eq!(
        HEPTA_KERNEL_NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT,
        "/api/native-post-gray-release-evidence"
    );
    assert_eq!(limits.max_store_bytes, 10 * 1024 * 1024);
    assert_eq!(limits.max_store_lines, 100_000);
    assert_eq!(limits.rate_limit_window_ms, 1_000);
}

#[test]
fn kernel_native_post_execution_store_specs_freeze_store_files() {
    let specs = hepta_kernel_native_post_execution_store_specs();

    assert_eq!(specs.len(), 4);
    assert_eq!(specs[0].store_kind, "idempotency");
    assert_eq!(specs[0].schema_id, "hepta.post.idempotency_entry.v1");
    assert_eq!(specs[0].filename, "idempotency.jsonl");
    assert_eq!(specs[1].store_kind, "audit");
    assert_eq!(specs[1].schema_id, "hepta.post.execution_audit.v1");
    assert_eq!(specs[1].filename, "audit.jsonl");
    assert_eq!(specs[2].store_kind, "rollback");
    assert_eq!(specs[2].schema_id, "hepta.post.rollback_anchor.v1");
    assert_eq!(specs[2].filename, "rollback.jsonl");
    assert_eq!(specs[3].store_kind, "rate_limit");
    assert_eq!(specs[3].schema_id, "hepta.post.rate_limit_entry.v1");
    assert_eq!(specs[3].filename, "rate-limit.jsonl");
}

#[test]
fn kernel_native_post_execution_store_file_status_report_binds_capacity_and_redaction() {
    let spec = &hepta_kernel_native_post_execution_store_specs()[0];

    let status = hepta_kernel_native_post_execution_store_file_status_report(
        spec,
        ".hepta/native-post-execution/idempotency.jsonl".to_string(),
        true,
        99,
        100,
        3,
        true,
        4,
        3,
        1,
    );
    let observed = hepta_kernel_native_post_execution_store_file_status_from_observation(
        spec,
        HeptaKernelNativePostExecutionStoreFileObservation {
            path: ".hepta/native-post-execution/idempotency.jsonl".to_string(),
            exists: true,
            bytes: 99,
            max_bytes: 100,
            max_lines: 3,
            jsonl_observation: HeptaKernelNativePostStoreReadObservation {
                content: Some("{\"ok\":true}\n[1]\n{\"n\":2}\nnot-json\n".to_string()),
                missing: false,
                read_failed: false,
            },
        },
    );

    assert_eq!(observed, status);
    assert_eq!(status.store_kind, "idempotency");
    assert_eq!(status.schema_id, "hepta.post.idempotency_entry.v1");
    assert_eq!(status.filename, "idempotency.jsonl");
    assert!(status.exists);
    assert!(status.bytes_within_limit);
    assert!(!status.line_count_within_limit);
    assert!(status.jsonl_readable);
    assert!(!status.jsonl_valid);
    assert!(status.append_only);
    assert!(status.jsonl);
    assert!(!status.raw_body_exposed);
    assert!(!status.raw_field_values_exposed);
    assert!(!status.raw_idempotency_key_exposed);
}

#[test]
fn kernel_native_post_execution_store_jsonl_health_counts_content_and_default_paths() {
    let missing = hepta_kernel_native_post_execution_store_jsonl_health_missing();
    assert!(missing.jsonl_readable);
    assert_eq!(missing.line_count, 0);
    assert_eq!(missing.valid_json_line_count, 0);
    assert_eq!(missing.invalid_json_line_count, 0);

    let failed = hepta_kernel_native_post_execution_store_jsonl_health_read_failed();
    assert!(!failed.jsonl_readable);
    assert_eq!(failed.line_count, 0);
    assert_eq!(failed.valid_json_line_count, 0);
    assert_eq!(failed.invalid_json_line_count, 0);

    let health = hepta_kernel_native_post_execution_store_jsonl_health_from_content(
        "{\"ok\":true}\nnot-json\n[1,2,3]\n",
    );
    assert!(health.jsonl_readable);
    assert_eq!(health.line_count, 3);
    assert_eq!(health.valid_json_line_count, 2);
    assert_eq!(health.invalid_json_line_count, 1);

    let observed_health = hepta_kernel_native_post_execution_store_jsonl_health_from_observation(
        HeptaKernelNativePostStoreReadObservation {
            content: Some("{\"ok\":true}\nnot-json\n".to_string()),
            missing: false,
            read_failed: false,
        },
    );
    assert!(observed_health.jsonl_readable);
    assert_eq!(observed_health.line_count, 2);
    assert_eq!(observed_health.valid_json_line_count, 1);
    assert_eq!(observed_health.invalid_json_line_count, 1);

    let observed_missing = hepta_kernel_native_post_execution_store_jsonl_health_from_observation(
        HeptaKernelNativePostStoreReadObservation {
            content: None,
            missing: true,
            read_failed: false,
        },
    );
    assert_eq!(observed_missing, missing);

    let observed_failed = hepta_kernel_native_post_execution_store_jsonl_health_from_observation(
        HeptaKernelNativePostStoreReadObservation {
            content: None,
            missing: false,
            read_failed: true,
        },
    );
    assert_eq!(observed_failed, failed);
}

#[test]
fn kernel_native_post_execution_store_capacity_allows_append_projects_limits() {
    let spec = &hepta_kernel_native_post_execution_store_specs()[0];
    let ready = hepta_kernel_native_post_execution_store_file_status_report(
        spec,
        ".hepta/native-post-execution/idempotency.jsonl".to_string(),
        true,
        80,
        100,
        3,
        true,
        2,
        2,
        0,
    );

    assert!(
        hepta_kernel_native_post_execution_store_capacity_allows_append(
            &[ready.clone()],
            20,
            100,
            3
        )
    );
    assert!(
        !hepta_kernel_native_post_execution_store_capacity_allows_append(
            &[ready.clone()],
            21,
            100,
            3
        )
    );
    assert!(
        !hepta_kernel_native_post_execution_store_capacity_allows_append(
            &[ready.clone()],
            20,
            100,
            2
        )
    );

    let invalid = hepta_kernel_native_post_execution_store_file_status_report(
        spec,
        ".hepta/native-post-execution/idempotency.jsonl".to_string(),
        true,
        1,
        100,
        3,
        true,
        1,
        0,
        1,
    );
    assert!(
        !hepta_kernel_native_post_execution_store_capacity_allows_append(
            &[invalid.clone()],
            1,
            100,
            3
        )
    );
    assert!(hepta_kernel_native_post_execution_store_jsonl_valid(&[
        ready.clone()
    ]));
    assert!(hepta_kernel_native_post_execution_store_capacity_ok(&[
        ready.clone()
    ]));
    assert!(!hepta_kernel_native_post_execution_store_jsonl_valid(&[
        invalid.clone()
    ]));
    assert!(hepta_kernel_native_post_execution_store_capacity_ok(&[
        invalid.clone()
    ]));

    let full = hepta_kernel_native_post_execution_store_file_status_report(
        spec,
        ".hepta/native-post-execution/idempotency.jsonl".to_string(),
        true,
        101,
        100,
        3,
        true,
        4,
        4,
        0,
    );
    assert!(hepta_kernel_native_post_execution_store_jsonl_valid(&[
        full.clone()
    ]));
    assert!(!hepta_kernel_native_post_execution_store_capacity_ok(&[
        full
    ]));
}

#[test]
fn kernel_native_post_idempotency_duplicate_scan_uses_redacted_fingerprint() {
    let content =
        "{\"key_fingerprint\":\"sha256:abc123\"}\n{\"key_fingerprint\":\"sha256:def456\"}\n";

    assert!(
        hepta_kernel_native_post_idempotency_duplicate_present_in_content(
            content,
            Some("sha256:abc123")
        )
    );
    assert!(
        !hepta_kernel_native_post_idempotency_duplicate_present_in_content(
            content,
            Some("sha256:missing")
        )
    );
    assert!(!hepta_kernel_native_post_idempotency_duplicate_present_in_content(content, None));
    assert_eq!(
        hepta_kernel_native_post_idempotency_duplicate_present_from_observation(
            HeptaKernelNativePostStoreReadObservation {
                content: Some(content.to_string()),
                missing: false,
                read_failed: false,
            },
            Some("sha256:abc123"),
        ),
        Ok(true)
    );
    assert_eq!(
        hepta_kernel_native_post_idempotency_duplicate_present_from_observation(
            HeptaKernelNativePostStoreReadObservation {
                content: None,
                missing: true,
                read_failed: false,
            },
            Some("sha256:abc123"),
        ),
        Ok(false)
    );
    assert_eq!(
        hepta_kernel_native_post_idempotency_duplicate_present_from_observation(
            HeptaKernelNativePostStoreReadObservation {
                content: None,
                missing: false,
                read_failed: true,
            },
            Some("sha256:abc123"),
        ),
        Err("native_post_idempotency_check_failed")
    );
}

#[test]
fn kernel_native_post_rate_limit_scan_uses_bucket_window_and_now() {
    let content = "{\"rate_limit_bucket\":\"task_publish\",\"recorded_at_unix_ms\":900}\nnot-json\n{\"rate_limit_bucket\":\"chat_send\",\"recorded_at_unix_ms\":990}\n";

    assert!(
        hepta_kernel_native_post_rate_limit_recent_present_in_content(
            content,
            "task_publish",
            150,
            1_000,
        )
    );
    assert!(
        !hepta_kernel_native_post_rate_limit_recent_present_in_content(
            content,
            "task_publish",
            99,
            1_000,
        )
    );
    assert!(
        !hepta_kernel_native_post_rate_limit_recent_present_in_content(
            content, "missing", 1_000, 1_000,
        )
    );
    assert_eq!(
        hepta_kernel_native_post_rate_limit_recent_present_from_observation(
            HeptaKernelNativePostStoreReadObservation {
                content: Some(content.to_string()),
                missing: false,
                read_failed: false,
            },
            "task_publish",
            150,
            1_000,
        ),
        Ok(true)
    );
    assert_eq!(
        hepta_kernel_native_post_rate_limit_recent_present_from_observation(
            HeptaKernelNativePostStoreReadObservation {
                content: None,
                missing: true,
                read_failed: false,
            },
            "task_publish",
            150,
            1_000,
        ),
        Ok(false)
    );
    assert_eq!(
        hepta_kernel_native_post_rate_limit_recent_present_from_observation(
            HeptaKernelNativePostStoreReadObservation {
                content: None,
                missing: false,
                read_failed: true,
            },
            "task_publish",
            150,
            1_000,
        ),
        Err("native_post_rate_limit_check_failed")
    );
}

#[test]
fn kernel_native_post_execution_store_write_report_binds_files_and_redaction() {
    let report = hepta_kernel_native_post_execution_store_write_report(
        ".hepta/native-post-execution".to_string(),
        vec!["idempotency.jsonl".to_string(), "audit.jsonl".to_string()],
    );

    assert_eq!(report.status, "written");
    assert_eq!(report.root, ".hepta/native-post-execution");
    assert_eq!(report.written_file_count, 2);
    assert_eq!(report.written_files.len(), 2);
    assert!(!report.raw_request_body_exposed);
    assert!(!report.raw_field_values_exposed);
    assert!(!report.raw_idempotency_key_exposed);
    assert!(!report.raw_audit_payload_exposed);
}

#[test]
fn kernel_native_post_rollout_evidence_scan_summarizes_redacted_records() {
    let content = r#"{"recorded_at_unix_ms":1,"route_pattern":"/api/tasks/publish","capability":"task.publish","plan_kind":"task_publish","body_schema_id":"hepta.post.task_publish.v1","body_admission_status":"ready_for_real_handler","rollback_strategy":"pending_real_handler_rollback_anchor","rate_limit_bucket":"task_publish","current_plan_executes_real_handler":true,"idempotency_key_redacted":true,"idempotency_key_fingerprint":"sha256:abc","raw_request_body_exposed":false,"raw_field_values_exposed":false,"raw_idempotency_key_exposed":false,"raw_audit_payload_exposed":false}
not-json
{"recorded_at_unix_ms":2,"plan_kind":"chat_send","current_plan_executes_real_handler":false,"rollback_strategy":"pending_real_handler_rollback_anchor","raw_request_body_exposed":true}"#;

    let scan = hepta_kernel_native_post_rollout_evidence_scan_from_content(content);
    let observed_scan = hepta_kernel_native_post_rollout_evidence_scan_from_observation(
        HeptaKernelNativePostRolloutEvidenceFileObservation {
            content: Some(content.to_string()),
            missing: false,
            read_failed: false,
        },
    );

    assert!(scan.jsonl_readable);
    assert_eq!(scan.line_count, 3);
    assert_eq!(scan.valid_json_line_count, 2);
    assert_eq!(scan.invalid_json_line_count, 1);
    assert_eq!(scan.record_count, 2);
    assert_eq!(scan.dry_run_record_count, 1);
    assert_eq!(scan.rollback_anchor_count, 2);
    assert_eq!(scan.plan_kind_counts.len(), 2);
    assert!(scan.raw_request_body_exposed);
    let latest = scan.latest_record.expect("latest record");
    assert_eq!(latest.recorded_at_unix_ms, Some(2));
    assert_eq!(latest.plan_kind.as_deref(), Some("chat_send"));
    assert!(latest.raw_request_body_exposed);
    assert_eq!(observed_scan.record_count, 2);
    assert_eq!(observed_scan.dry_run_record_count, 1);

    let selected = hepta_kernel_native_post_selected_handler_rollout_evidence_from_content(
        Some("task_publish"),
        content,
    );
    let observed_selected =
        hepta_kernel_native_post_selected_handler_rollout_evidence_from_observation(
            Some("task_publish"),
            HeptaKernelNativePostRolloutEvidenceFileObservation {
                content: Some(content.to_string()),
                missing: false,
                read_failed: false,
            },
        );
    assert_eq!(
        selected.selected_handler_kind.as_deref(),
        Some("task_publish")
    );
    assert_eq!(selected.record_count, 1);
    assert!(selected.dry_run_record_present);
    assert!(selected.rollback_anchor_present);
    assert!(!selected.raw_request_body_exposed);
    assert_eq!(observed_selected.record_count, selected.record_count);

    let missing = hepta_kernel_native_post_rollout_evidence_scan_missing();
    assert!(missing.jsonl_readable);
    assert_eq!(missing.record_count, 0);
    let observed_missing = hepta_kernel_native_post_rollout_evidence_scan_from_observation(
        HeptaKernelNativePostRolloutEvidenceFileObservation {
            content: None,
            missing: true,
            read_failed: false,
        },
    );
    assert_eq!(observed_missing.record_count, 0);
    assert!(observed_missing.jsonl_readable);

    let read_failed = hepta_kernel_native_post_rollout_evidence_scan_read_failed();
    assert!(!read_failed.jsonl_readable);
    assert_eq!(read_failed.read_error, Some("rollback_store_read_failed"));

    let report = hepta_kernel_native_post_rollout_evidence_report(
        ".hepta/native-post-execution".to_string(),
        true,
        true,
        Some("task_publish"),
        hepta_kernel_native_post_rollout_evidence_scan_from_content(content),
    );
    assert_eq!(report.status, "attention");
    assert_eq!(
        report.endpoint,
        HEPTA_KERNEL_NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT
    );
    assert!(!report.single_handler_scope_ready);
    assert!(report.selected_handler_kinds.is_empty());
    assert!(report.rollback_anchor_present);
    assert!(report.dry_run_record_present);
    assert!(report.raw_request_body_exposed);
    assert!(!report.external_side_effects);
}

#[test]
fn kernel_native_post_gray_release_evidence_requires_scoped_rollout_evidence() {
    let empty_selected =
        hepta_kernel_native_post_selected_handler_rollout_evidence_missing(Some("task_publish"));
    let staged = hepta_kernel_native_post_gray_release_evidence_report(
        ".hepta/native-post-execution".to_string(),
        Some("task_publish"),
        true,
        true,
        true,
        true,
        true,
        false,
        false,
        false,
        false,
        empty_selected,
    );

    assert_eq!(staged.status, "attention");
    assert_eq!(staged.gray_release_phase, "activation_preflight_not_ready");
    assert!(!staged.activation_currently_enabled);
    assert!(!staged.gray_release_ready);

    let content = r#"{"recorded_at_unix_ms":1,"plan_kind":"task_publish","rollback_strategy":"pending_real_handler_rollback_anchor","current_plan_executes_real_handler":true,"idempotency_key_redacted":true,"idempotency_key_fingerprint":"sha256:abc","raw_request_body_exposed":false,"raw_field_values_exposed":false,"raw_idempotency_key_exposed":false,"raw_audit_payload_exposed":false}"#;
    let selected = hepta_kernel_native_post_selected_handler_rollout_evidence_from_content(
        Some("task_publish"),
        content,
    );
    let ready = hepta_kernel_native_post_gray_release_evidence_report(
        ".hepta/native-post-execution".to_string(),
        Some("task_publish"),
        true,
        true,
        true,
        true,
        true,
        false,
        false,
        false,
        false,
        selected,
    );

    assert_eq!(ready.status, "attention");
    assert_eq!(ready.gray_release_phase, "activation_preflight_not_ready");
    assert!(!ready.gray_release_evidence_ready);
    assert!(!ready.gray_release_ready);
    assert_eq!(
        ready.endpoint,
        HEPTA_KERNEL_NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT
    );
}

#[test]
fn kernel_turn_plan_makes_hepta_the_owner_and_codex_an_engine() {
    let plan = plan_hepta_kernel_turn(HeptaKernelTurnInput {
        channel: HeptaKernelTurnChannel::Telegram,
        user_message: "  解释一下融合架构  ",
        engine: HeptaKernelEngine::CodexEngine,
        hepta_intelligence_context: true,
        plugin_capability_context: true,
    })
    .expect("kernel plan");

    assert_eq!(plan.contract, HEPTA_KERNEL_CONTRACT);
    assert_eq!(plan.kernel_owner, HEPTA_KERNEL_OWNER);
    assert_eq!(plan.engine_id, CODEX_ENGINE_ID);
    assert!(!plan.codex_core_as_product_base);
    assert!(plan.hepta_owns_turn_loop);
    assert!(plan.hepta_intelligence_context);
    assert!(plan.plugin_capability_context);
    assert_eq!(plan.codex_tool_mention_sigil, '$');
    assert_eq!(plan.codex_plugin_mention_sigil, '@');
    assert_eq!(plan.agents_md_filename, "AGENTS.md");
    assert!(plan.prompt.contains("Hepta kernel owns the turn loop"));
    assert!(
        plan.prompt
            .contains("Codex is an internal execution engine")
    );
    assert!(
        plan.prompt
            .contains("Inbound Telegram user message:\n解释一下融合架构")
    );
}

#[test]
fn kernel_rejects_empty_turn_material() {
    let error = hepta_kernel_telegram_prompt("  ", true, true).expect_err("empty rejected");
    assert!(error.contains("non-empty"));
}

#[test]
fn kernel_runner_selection_prefers_hepta_kernel_over_mlx() {
    let plan = select_hepta_kernel_telegram_runner(
        Some("mlx-local/local-model"),
        Some("http://127.0.0.1:11436/v1"),
        Some(128),
        false,
        true,
    );

    assert_eq!(plan.runner_kind, HEPTA_KERNEL_TELEGRAM_RUNNER_KIND);
    assert!(plan.codex_core_runner_enabled);
    assert!(plan.in_process_runner_enabled);
    assert!(!plan.local_network_call);
    assert!(!plan.process_spawned_by_status);
    assert!(plan.hepta_intelligence_context_injected);
    assert!(plan.plugin_capability_context_injected);
    assert!(!plan.raw_prompt_text_exposed);
}

#[test]
fn kernel_session_bridge_plan_is_ready_and_redacted() {
    let runner = select_hepta_kernel_telegram_runner(
        Some("mlx-local/local-model"),
        Some(DEFAULT_TELEGRAM_MLX_BASE_URL),
        Some(128),
        false,
        true,
    );
    let plan = plan_hepta_kernel_telegram_session_bridge(Some(&runner));

    assert!(plan.bridge_plan_ready);
    assert_eq!(plan.runner_kind, HEPTA_KERNEL_TELEGRAM_RUNNER_KIND);
    assert_eq!(
        plan.runner_invocation_strategy,
        HEPTA_KERNEL_TELEGRAM_RUNNER_STRATEGY
    );
    assert!(
        plan.prompt_material_policy
            .contains("never serialized into status JSON")
    );
    assert!(
        plan.session_key_strategy
            .contains("without exposing raw chat ids")
    );
    assert!(plan.duplicate_policy.contains("before any model turn"));
    assert!(plan.cursor_commit_policy.contains("after model output"));
    assert!(
        plan.response_delivery_policy
            .contains("HEPTA_NATIVE_TELEGRAM_SEND")
    );
    assert!(!plan.process_spawned_by_status);
    assert!(!plan.raw_prompt_text_exposed);
    assert!(!plan.raw_chat_id_exposed);
    assert!(!plan.raw_sender_id_exposed);
    assert!(!plan.raw_message_id_exposed);

    let disabled = plan_hepta_kernel_telegram_session_bridge(None);
    assert!(!disabled.bridge_plan_ready);
    assert_eq!(disabled.runner_kind, "disabled");
}

fn telegram_kernel_gates(
    delivery: bool,
    live_read: bool,
    model_turn: bool,
    send: bool,
) -> HeptaKernelTelegramGatewayGateSummary {
    build_hepta_kernel_telegram_gateway_gate_summary(HeptaKernelTelegramGatewayGateSummaryInput {
        delivery_approval_gate_env: "HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED",
        delivery_approval_gate_enabled: delivery,
        live_read_gate_env: "HEPTA_NATIVE_TELEGRAM_LIVE_READ",
        live_read_gate_enabled: live_read,
        model_turn_gate_env: "HEPTA_NATIVE_TELEGRAM_MODEL_TURN",
        model_turn_gate_enabled: model_turn,
        send_gate_env: "HEPTA_NATIVE_TELEGRAM_SEND",
        send_gate_enabled: send,
    })
}

fn ready_telegram_config() -> HeptaKernelTelegramConfigStatus {
    HeptaKernelTelegramConfigStatus {
        config_path: Some("private/config/openclaw.json".to_string()),
        config_found: true,
        enabled: true,
        dm_policy: "trusted".to_string(),
        group_policy: "deny".to_string(),
        allow_from_count: 1,
        group_count: 0,
        token_source: "secret_file",
        token_secret_ref_present: true,
        token_secret_provider: Some("telegram_bot".to_string()),
        token_secret_id_present: true,
        token_file_present: true,
        token_file_mode_0600: true,
        token_file_security_ready: true,
        token_shape_ok: true,
        raw_token_exposed: false,
        binding_ready: true,
        error: None,
    }
}

fn ready_kernel_poll_loop_status() -> HeptaKernelTelegramPollLoopStatus {
    HeptaKernelTelegramPollLoopStatus {
        product: "Hepta",
        runtime: "hepta",
        requested: true,
        status: "armed",
        poll_loop_gate_env: "HEPTA_NATIVE_TELEGRAM_POLL_LOOP",
        poll_loop_gate_enabled: true,
        delivery_approval_gate_env: "HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED",
        delivery_approval_gate_enabled: true,
        poll_ms: 1500,
        drain_once_endpoint: "/api/telegram-drain-once",
        worker_spawned_by_status: false,
        loop_invokes_drain_once: true,
        requires_live_read_gate: "HEPTA_NATIVE_TELEGRAM_LIVE_READ",
        requires_model_turn_gate: "HEPTA_NATIVE_TELEGRAM_MODEL_TURN",
        requires_send_gate: "HEPTA_NATIVE_TELEGRAM_SEND",
        requires_delivery_approval_gate: "HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED",
        external_network_read_by_status: false,
        external_send_by_status: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        raw_token_exposed: false,
        next_migration_slice: "test",
    }
}

fn ready_kernel_cursor_status() -> HeptaKernelTelegramCursorStatus {
    HeptaKernelTelegramCursorStatus {
        product: "Hepta",
        runtime: "hepta",
        requested: true,
        status: "ready",
        cursor_path: ".hepta/telegram/ingress-drain-cursor.json",
        cursor_file_present: true,
        cursor_parse_ok: true,
        next_update_offset: Some(917025970),
        cursor_updated_at_unix_ms: Some(TEST_NOW_MS),
        last_delivered_next_update_offset: Some(917025970),
        durable_cursor_evidence_present: true,
        cursor_represents_next_update_offset: true,
        duplicate_suppression_rule_valid: true,
        cursor_write_policy: "write only after model output is delivered or duplicate suppression is recorded",
        cursor_written: false,
        raw_update_payload_persisted: false,
        error: None,
        next_migration_slice: "test",
    }
}

fn ready_kernel_delivery_ledger_status() -> HeptaKernelTelegramDeliveryLedgerStatus {
    HeptaKernelTelegramDeliveryLedgerStatus {
        product: "Hepta",
        runtime: "hepta",
        requested: true,
        status: "ready",
        ledger_path: ".hepta/telegram/delivery-ledger.jsonl",
        ledger_file_present: true,
        jsonl_readable: true,
        jsonl_valid: true,
        line_count: 2,
        valid_json_line_count: 2,
        invalid_json_line_count: 0,
        acked_count: 1,
        failed_count: 0,
        latest_stage: Some("acked".to_string()),
        latest_created_unix_seconds: Some(TEST_NOW_MS / 1_000),
        latest_acked_created_unix_seconds: Some(TEST_NOW_MS / 1_000),
        ledger_updated_at_unix_ms: Some(TEST_NOW_MS),
        provider_message_id_present: true,
        durable_delivery_evidence_present: true,
        raw_response_text_logged: false,
        raw_chat_id_logged: false,
        raw_message_id_logged: false,
        raw_token_logged: false,
        error: None,
        next_migration_slice: "test",
    }
}

fn ready_kernel_production_guards() -> HeptaKernelTelegramProductionGuardStatus {
    HeptaKernelTelegramProductionGuardStatus {
        read_max_attempts_env: "HEPTA_NATIVE_TELEGRAM_READ_MAX_ATTEMPTS",
        read_max_attempts: 3,
        read_retry_backoff_env: "HEPTA_NATIVE_TELEGRAM_READ_RETRY_BACKOFF_MS",
        read_retry_backoff_ms: 700,
        retry_transient_read_errors: true,
        typing_keepalive_env: "HEPTA_NATIVE_TELEGRAM_TYPING_KEEPALIVE",
        typing_keepalive_enabled: true,
        typing_keepalive_interval_ms: 4000,
        model_timeout_env: "HEPTA_NATIVE_TELEGRAM_MODEL_TIMEOUT_MS",
        model_timeout_ms: 120000,
        model_failure_fallback_env: "HEPTA_NATIVE_TELEGRAM_MODEL_FAILURE_FALLBACK",
        model_failure_fallback_enabled: true,
        send_min_interval_env: "HEPTA_NATIVE_TELEGRAM_SEND_MIN_INTERVAL_MS",
        send_min_interval_ms: 1200,
        send_max_attempts_env: "HEPTA_NATIVE_TELEGRAM_SEND_MAX_ATTEMPTS",
        send_max_attempts: 3,
        send_retry_backoff_env: "HEPTA_NATIVE_TELEGRAM_SEND_RETRY_BACKOFF_MS",
        send_retry_backoff_ms: 700,
        retry_transient_send_errors: true,
        rate_limit_scope: "in-process per chat id; reset on gateway restart",
        raw_token_exposed: false,
    }
}

fn kernel_live_soak_observation(
    poll_iterations: u64,
    attention_count: u64,
    last_status: Option<&str>,
    last_bot_api_ok: Option<bool>,
) -> HeptaKernelTelegramLiveSoakObservationReport {
    HeptaKernelTelegramLiveSoakObservationReport {
        poll_iterations,
        drained_count: 0,
        busy_count: 0,
        attention_count,
        empty_read_count: poll_iterations.saturating_sub(attention_count),
        model_turn_started_count: 0,
        send_started_count: 0,
        cursor_written_count: 0,
        external_send_count: 0,
        last_drained_at_unix_ms: None,
        last_drained_next_update_offset: None,
        last_observed_at_unix_ms: Some(TEST_NOW_MS),
        last_status: last_status.map(str::to_string),
        last_error: None,
        last_bot_api_ok,
        last_get_updates_offset: Some(917025970),
        last_local_next_update_offset: None,
        last_update_count: 0,
        last_allowed_update_count: 0,
        last_model_turn_started: false,
        last_send_started: false,
        last_cursor_written: false,
        last_external_send: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        raw_token_exposed: false,
    }
}

#[test]
fn kernel_model_bridge_status_is_gated_and_side_effect_free() {
    let runner = select_hepta_kernel_telegram_runner(
        Some("mlx-local/local-model"),
        Some(DEFAULT_TELEGRAM_MLX_BASE_URL),
        Some(128),
        false,
        true,
    );
    let status = build_hepta_kernel_telegram_model_bridge_status(
        HeptaKernelTelegramModelBridgeStatusInput {
            requested: true,
            config: ready_telegram_config(),
            model_turn_gate_env: "HEPTA_NATIVE_TELEGRAM_MODEL_TURN",
            model_turn_gate_enabled: false,
            send_gate_env: "HEPTA_NATIVE_TELEGRAM_SEND",
            model_runner_plan: &runner,
        },
    );

    assert_eq!(status.status, "gated");
    assert_eq!(
        status.model_turn_gate_env,
        "HEPTA_NATIVE_TELEGRAM_MODEL_TURN"
    );
    assert!(!status.model_turn_bridge_ready);
    assert!(!status.model_turn_started);
    assert!(!status.session_runner_invoked);
    assert!(!status.local_process_spawned);
    assert!(!status.external_network_read);
    assert!(!status.external_send);
    assert!(!status.cursor_written);
    assert!(!status.raw_update_payload_exposed);
    assert!(!status.raw_prompt_text_exposed);
    assert!(status.cursor_plan.duplicate_suppression_ready);
    assert!(status.model_turn_plan.planner_ready);
    assert_eq!(
        status.invocation_request.duplicate_decision,
        "no_model_candidate"
    );
    assert_eq!(status.model_execution.status, "gated");
    assert!(status.bridge_plan.bridge_plan_ready);
    assert!(!status.bridge_plan.process_spawned_by_status);
    assert!(
        status
            .error
            .unwrap()
            .contains("HEPTA_NATIVE_TELEGRAM_MODEL_TURN")
    );
}

#[test]
fn kernel_plugin_status_reports_native_supervisor_without_side_effects() {
    let status = build_hepta_kernel_telegram_plugin_status(HeptaKernelTelegramPluginStatusInput {
        requested: true,
        poll_ms: 1_500,
        allowed_updates: HEPTA_KERNEL_TELEGRAM_ALLOWED_UPDATES,
        config: ready_telegram_config(),
        gates: telegram_kernel_gates(true, true, true, true),
        poll_loop_gate_enabled: true,
    });

    assert_eq!(status.status, "native_supervisor_ready");
    assert!(status.in_process_supervisor_ready);
    assert!(status.in_process_reply_loop_ready);
    assert!(status.model_turn_bridge_ready);
    assert!(status.bot_api_poll_ready);
    assert!(status.bot_api_send_ready);
    assert!(!status.openclaw_gateway_runtime_dependency);
    assert!(!status.external_network_read);
    assert!(!status.external_send);
    assert!(status.transport_plan.bot_api_transport_plan_ready);
    assert!(status.ingress_parser.parser_ready);
    assert_eq!(status.ingress_parser.update_count, 0);
    assert!(status.cursor_plan.duplicate_suppression_ready);
    assert!(status.model_turn_plan.planner_ready);
    assert!(status.migration_blocker.is_none());

    let disabled =
        build_hepta_kernel_telegram_plugin_status(HeptaKernelTelegramPluginStatusInput {
            requested: false,
            poll_ms: 1_500,
            allowed_updates: HEPTA_KERNEL_TELEGRAM_ALLOWED_UPDATES,
            config: ready_telegram_config(),
            gates: telegram_kernel_gates(false, false, false, false),
            poll_loop_gate_enabled: false,
        });
    assert_eq!(disabled.status, "disabled");
    assert!(!disabled.transport_plan.bot_api_transport_plan_ready);
    assert!(!disabled.cursor_plan.duplicate_suppression_ready);
    assert!(!disabled.model_turn_plan.planner_ready);
}

#[test]
fn kernel_model_turn_plan_status_is_planned_and_side_effect_free() {
    let status = build_hepta_kernel_telegram_model_turn_plan_status(
        HeptaKernelTelegramModelTurnPlanStatusInput {
            requested: true,
            config: ready_telegram_config(),
        },
    );

    assert_eq!(status.status, "planned");
    assert!(!status.model_turn_bridge_ready);
    assert!(!status.model_turn_started);
    assert!(!status.session_runner_invoked);
    assert!(!status.external_send);
    assert!(!status.cursor_written);
    assert!(!status.raw_update_payload_exposed);
    assert!(!status.raw_prompt_text_exposed);
    assert!(!status.raw_chat_id_exposed);
    assert!(!status.raw_sender_id_exposed);
    assert!(!status.raw_message_id_exposed);
    assert!(status.cursor_plan.duplicate_suppression_ready);
    assert!(status.inspection.parser_ready);
    assert_eq!(status.inspection.update_count, 0);
    assert!(status.model_turn_plan.planner_ready);
    assert_eq!(status.model_turn_plan.candidate_count, 0);
    assert!(status.error.is_none());

    let disabled = build_hepta_kernel_telegram_model_turn_plan_status(
        HeptaKernelTelegramModelTurnPlanStatusInput {
            requested: false,
            config: ready_telegram_config(),
        },
    );
    assert_eq!(disabled.status, "disabled");
    assert!(!disabled.cursor_plan.duplicate_suppression_ready);
    assert!(!disabled.model_turn_plan.planner_ready);
}

#[test]
fn kernel_send_plan_status_is_gated_and_side_effect_free() {
    let status =
        build_hepta_kernel_telegram_send_plan_status(HeptaKernelTelegramSendPlanStatusInput {
            requested: true,
            config: ready_telegram_config(),
            send_gate_env: "HEPTA_NATIVE_TELEGRAM_SEND",
            send_gate_enabled: false,
        });

    assert_eq!(status.status, "gated");
    assert_eq!(status.send_gate_env, "HEPTA_NATIVE_TELEGRAM_SEND");
    assert!(!status.send_gate_enabled);
    assert!(!status.bot_api_send_ready);
    assert!(!status.external_network_write);
    assert!(!status.external_send);
    assert!(!status.cursor_written);
    assert!(!status.raw_response_text_exposed);
    assert!(!status.raw_chat_id_exposed);
    assert!(!status.raw_message_id_exposed);
    assert!(!status.raw_token_exposed);
    assert!(status.transport_plan.bot_api_transport_plan_ready);
    assert!(status.send_plan.send_plan_ready);
    assert!(status.send_request.request_builder_ready);
    assert!(!status.send_request.send_allowed);
    assert!(status.error.unwrap().contains("HEPTA_NATIVE_TELEGRAM_SEND"));
}

#[test]
fn kernel_gateway_gate_summary_is_side_effect_free() {
    let summary = telegram_kernel_gates(true, false, true, false);

    assert!(summary.delivery_approval_gate_enabled);
    assert!(!summary.live_read_gate_enabled);
    assert!(summary.model_turn_gate_enabled);
    assert!(!summary.send_gate_enabled);
    assert!(!summary.readiness_summary_performs_live_read);
    assert!(!summary.readiness_summary_invokes_model);
    assert!(!summary.readiness_summary_sends_message);
}

#[test]
fn kernel_receive_once_preflight_reports_gate_without_side_effects() {
    let config = ready_telegram_config();
    let transport_plan = HeptaKernelTelegramTransportPlan::for_config_state(true, true, true);
    let cursor_plan = HeptaKernelTelegramCursorPlan::ready();

    let report = plan_hepta_kernel_telegram_receive_once_preflight_status(
        HeptaKernelTelegramReceiveOncePreflightInput {
            requested: true,
            live_read_gate_env: "HEPTA_NATIVE_TELEGRAM_LIVE_READ",
            live_read_gate_enabled: false,
            limit: 99,
            config: &config,
            transport_plan: &transport_plan,
            cursor_plan: &cursor_plan,
        },
    )
    .expect("missing live-read gate should produce a status report");

    assert_eq!(report.status, "gated");
    assert_eq!(report.limit, 99);
    assert!(!report.external_network_read);
    assert!(!report.external_send);
    assert!(!report.cursor_written);
    assert!(!report.raw_token_exposed);
    assert_eq!(
        report.error.as_deref(),
        Some(
            "live Telegram receive is gated; set HEPTA_NATIVE_TELEGRAM_LIVE_READ=1 to run one redacted getUpdates read"
        )
    );

    assert!(
        plan_hepta_kernel_telegram_receive_once_preflight_status(
            HeptaKernelTelegramReceiveOncePreflightInput {
                requested: true,
                live_read_gate_env: "HEPTA_NATIVE_TELEGRAM_LIVE_READ",
                live_read_gate_enabled: true,
                limit: 20,
                config: &config,
                transport_plan: &transport_plan,
                cursor_plan: &cursor_plan,
            },
        )
        .is_none()
    );
}

#[test]
fn kernel_receive_once_api_result_redacts_and_preserves_candidate_plan() {
    let api = json!({
        "ok": true,
        "result": [{
            "update_id": 41,
            "message": {
                "message_id": 9,
                "text": "private user prompt",
                "chat": { "id": 6476198178_i64, "type": "private" },
                "from": { "id": 6476198178_i64, "username": "private_user" }
            }
        }]
    });

    let report = build_hepta_kernel_telegram_receive_once_status_from_api_result(
        HeptaKernelTelegramReceiveOnceApiResultInput {
            requested: true,
            live_read_gate_env: "HEPTA_NATIVE_TELEGRAM_LIVE_READ",
            live_read_gate_enabled: true,
            external_network_read: true,
            limit: 20,
            config: ready_telegram_config(),
            transport_plan: HeptaKernelTelegramTransportPlan::for_config_state(true, true, true),
            cursor_plan: HeptaKernelTelegramCursorPlan::ready(),
            get_updates_offset: Some(40),
            api_result: Ok(&api),
        },
    );

    assert_eq!(report.status, "ready");
    assert_eq!(report.bot_api_ok, Some(true));
    assert_eq!(report.get_updates_offset, Some(40));
    assert_eq!(report.local_next_update_offset, Some(42));
    assert_eq!(report.inspection.allowed_update_count, 1);
    assert_eq!(report.model_turn_plan.text_candidate_count, 1);
    assert!(!report.raw_update_payload_exposed);
    assert!(!report.raw_token_exposed);
    assert!(report.error.is_none());

    let ok_false = json!({
        "ok": false,
        "description": "Unauthorized 123456789:abcdefghijklmnopqrstuvwxyz token rejected"
    });
    let attention = build_hepta_kernel_telegram_receive_once_status_from_api_result(
        HeptaKernelTelegramReceiveOnceApiResultInput {
            requested: true,
            live_read_gate_env: "HEPTA_NATIVE_TELEGRAM_LIVE_READ",
            live_read_gate_enabled: true,
            external_network_read: true,
            limit: 1,
            config: ready_telegram_config(),
            transport_plan: HeptaKernelTelegramTransportPlan::for_config_state(true, true, true),
            cursor_plan: HeptaKernelTelegramCursorPlan::ready(),
            get_updates_offset: Some(7),
            api_result: Ok(&ok_false),
        },
    );
    assert_eq!(attention.status, "attention");
    assert_eq!(
        attention.error.as_deref(),
        Some("Unauthorized [redacted-telegram-token] token rejected")
    );

    let conflict = build_hepta_kernel_telegram_receive_once_status_from_api_result(
        HeptaKernelTelegramReceiveOnceApiResultInput {
            requested: true,
            live_read_gate_env: "HEPTA_NATIVE_TELEGRAM_LIVE_READ",
            live_read_gate_enabled: true,
            external_network_read: true,
            limit: 20,
            config: ready_telegram_config(),
            transport_plan: HeptaKernelTelegramTransportPlan::for_config_state(true, true, true),
            cursor_plan: HeptaKernelTelegramCursorPlan::ready(),
            get_updates_offset: Some(9),
            api_result: Err(
                "Telegram Bot API getUpdates HTTP status 409; description=Conflict: terminated by other getUpdates request",
            ),
        },
    );
    assert_eq!(conflict.status, "busy");
    assert_eq!(conflict.inspection.update_count, 0);
    assert!(
        conflict
            .error
            .as_deref()
            .unwrap_or_default()
            .contains("409")
    );
}

#[test]
fn kernel_drain_execution_plan_preserves_gate_order_and_probe_boundary() {
    assert_eq!(
        hepta_kernel_telegram_drain_first_missing_gate(&telegram_kernel_gates(
            false, false, false, false
        )),
        Some("HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED")
    );
    assert_eq!(
        hepta_kernel_telegram_drain_first_missing_gate(&telegram_kernel_gates(
            true, false, false, false
        )),
        Some("HEPTA_NATIVE_TELEGRAM_LIVE_READ")
    );
    assert_eq!(
        hepta_kernel_telegram_drain_first_missing_gate(&telegram_kernel_gates(
            true, true, false, false
        )),
        Some("HEPTA_NATIVE_TELEGRAM_MODEL_TURN")
    );
    assert_eq!(
        hepta_kernel_telegram_drain_first_missing_gate(&telegram_kernel_gates(
            true, true, true, false
        )),
        Some("HEPTA_NATIVE_TELEGRAM_SEND")
    );

    let gates = telegram_kernel_gates(true, true, true, true);
    let plan = hepta_kernel_telegram_drain_execution_plan(true, &gates);

    assert!(plan.execution_plan_ready);
    assert_eq!(plan.stages, HEPTA_KERNEL_TELEGRAM_DRAIN_ONCE_STAGES);
    assert!(plan.all_required_gates_enabled);
    assert_eq!(plan.first_missing_gate, None);
    assert!(plan.receive_before_model);
    assert!(plan.send_after_model_success);
    assert!(plan.cursor_commit_after_delivery);
    assert!(plan.status_probe_executes_pipeline);
    assert!(hepta_kernel_telegram_drain_status_probe_executes_pipeline(
        true,
        &telegram_kernel_gates(true, true, false, false)
    ));
    assert!(!hepta_kernel_telegram_drain_status_probe_executes_pipeline(
        true,
        &telegram_kernel_gates(true, false, true, true)
    ));
}

#[test]
fn kernel_model_turn_plan_defaults_keep_private_fields_redacted() {
    let disabled = HeptaKernelTelegramModelTurnPlan::disabled();
    assert!(!disabled.planner_ready);
    assert_eq!(disabled.prompt_material_policy, "disabled");
    assert!(!disabled.raw_message_text_exposed);
    assert!(!disabled.raw_callback_data_exposed);
    assert!(!disabled.raw_chat_id_exposed);
    assert!(!disabled.raw_sender_id_exposed);
    assert!(!disabled.raw_message_id_exposed);

    let mut ready = HeptaKernelTelegramModelTurnPlan::ready();
    ready.candidate_count = 2;
    ready.text_candidate_count = 1;
    ready.callback_candidate_count = 1;
    ready.reply_target_count = 2;
    ready.candidate_kinds.push("message:text".to_string());
    ready
        .candidate_kinds
        .push("callback_query:redacted".to_string());

    assert!(ready.planner_ready);
    assert!(
        ready
            .prompt_material_policy
            .contains("never expose it in readiness JSON")
    );
    assert!(ready.session_key_strategy.contains("redacted"));
    assert_eq!(ready.candidate_count, 2);
    let serialized = serde_json::to_string(&ready).expect("serialize");
    assert!(serialized.contains("callback_query:redacted"));
    assert!(!serialized.contains("private prompt text"));
    assert!(!serialized.contains("button_secret_payload"));
    assert!(!serialized.contains("6476198178"));
    assert!(!ready.raw_message_text_exposed);
    assert!(!ready.raw_callback_data_exposed);
    assert!(!ready.raw_chat_id_exposed);
    assert!(!ready.raw_sender_id_exposed);
    assert!(!ready.raw_message_id_exposed);
}

#[test]
fn kernel_model_turn_plan_aggregates_candidates_without_serializing_private_material() {
    let candidates = vec![
        HeptaKernelTelegramCandidateMaterial {
            update_id: Some(42),
            kind: "message:text".to_string(),
            prompt_text: Some("private prompt text".to_string()),
            has_reply_target: true,
            reply_target: Some(HeptaKernelTelegramReplyTargetMaterial {
                chat_id: 6476198178,
                reply_to_message_id: Some(7),
                raw_identifiers_exposed: false,
            }),
            requires_model: true,
            raw_identifiers_exposed: false,
        },
        HeptaKernelTelegramCandidateMaterial {
            update_id: Some(43),
            kind: "callback_query:redacted".to_string(),
            prompt_text: Some("button_secret_payload".to_string()),
            has_reply_target: true,
            reply_target: Some(HeptaKernelTelegramReplyTargetMaterial {
                chat_id: 6476198178,
                reply_to_message_id: Some(8),
                raw_identifiers_exposed: false,
            }),
            requires_model: true,
            raw_identifiers_exposed: false,
        },
        HeptaKernelTelegramCandidateMaterial {
            update_id: Some(44),
            kind: "message_reaction:redacted".to_string(),
            prompt_text: None,
            has_reply_target: false,
            reply_target: None,
            requires_model: false,
            raw_identifiers_exposed: false,
        },
    ];

    let plan = hepta_kernel_telegram_model_turn_plan_from_candidates(&candidates);

    assert!(plan.planner_ready);
    assert_eq!(plan.candidate_count, 3);
    assert_eq!(plan.text_candidate_count, 1);
    assert_eq!(plan.callback_candidate_count, 1);
    assert_eq!(plan.reaction_candidate_count, 1);
    assert_eq!(plan.reply_target_count, 2);
    assert_eq!(
        plan.candidate_kinds,
        vec![
            "message:text".to_string(),
            "callback_query:redacted".to_string(),
            "message_reaction:redacted".to_string(),
        ]
    );

    let serialized = serde_json::to_string(&plan).expect("serialize");
    assert!(!serialized.contains("private prompt text"));
    assert!(!serialized.contains("button_secret_payload"));
    assert!(!serialized.contains("6476198178"));
    assert!(!plan.raw_message_text_exposed);
    assert!(!plan.raw_callback_data_exposed);
    assert!(!plan.raw_chat_id_exposed);
    assert!(!plan.raw_sender_id_exposed);
    assert!(!plan.raw_message_id_exposed);
}

#[test]
fn kernel_ingress_parser_extracts_updates_without_serializing_private_material() {
    let updates = vec![
        json!({
            "update_id": 50,
            "message": {
                "message_id": 12,
                "text": "private message prompt",
                "chat": { "id": 6476198178_i64, "type": "private" },
                "from": { "id": 6476198178_i64, "username": "private_user" }
            }
        }),
        json!({
            "update_id": 51,
            "callback_query": {
                "id": "opaque-callback-id",
                "data": "button_secret_payload",
                "message": {
                    "message_id": 13,
                    "chat": { "id": 6476198178_i64, "type": "private" }
                }
            }
        }),
        json!({
            "update_id": 52,
            "message_reaction": {
                "chat": { "id": 6476198178_i64, "type": "private" }
            }
        }),
    ];

    let candidate =
        extract_hepta_kernel_telegram_candidate_material(&updates[0]).expect("candidate");
    assert_eq!(candidate.kind, "message:text");
    assert_eq!(
        candidate.prompt_text.as_deref(),
        Some("private message prompt")
    );
    assert!(candidate.has_reply_target);
    assert!(!candidate.raw_identifiers_exposed);

    let plan = hepta_kernel_telegram_model_turn_plan_for_updates(&updates);
    assert_eq!(plan.candidate_count, 3);
    assert_eq!(plan.text_candidate_count, 1);
    assert_eq!(plan.callback_candidate_count, 1);
    assert_eq!(plan.reaction_candidate_count, 1);
    assert_eq!(plan.reply_target_count, 2);

    let request = hepta_kernel_telegram_model_invocation_request_plan_for_updates(
        &updates,
        Some(50),
        "HEPTA_NATIVE_TELEGRAM_MODEL_TURN",
        true,
    );
    assert_eq!(request.duplicate_decision, "model_candidate");
    assert_eq!(request.candidate_kind.as_deref(), Some("message:text"));
    assert!(request.prompt_material_in_memory);
    assert!(!request.prompt_material_serialized);
    assert!(!request.raw_prompt_text_exposed);

    let inspection = inspect_hepta_kernel_telegram_updates(&updates);
    assert_eq!(inspection.update_count, 3);
    assert_eq!(inspection.allowed_update_count, 3);
    assert_eq!(inspection.latest_allowed_next_update_offset, Some(53));
    assert!(inspection.latest_allowed_text_present);

    let serialized_plan = serde_json::to_string(&plan).expect("serialize plan");
    let serialized_request = serde_json::to_string(&request).expect("serialize request");
    let serialized_inspection = serde_json::to_string(&inspection).expect("serialize inspection");
    for serialized in [serialized_plan, serialized_request, serialized_inspection] {
        assert!(!serialized.contains("private message prompt"));
        assert!(!serialized.contains("button_secret_payload"));
        assert!(!serialized.contains("opaque-callback-id"));
        assert!(!serialized.contains("6476198178"));
        assert!(!serialized.contains("private_user"));
    }
}

#[test]
fn kernel_first_model_candidate_selects_duplicate_policy_without_raw_payload() {
    let candidates = vec![
        HeptaKernelTelegramCandidateMaterial {
            update_id: Some(40),
            kind: "message_reaction:redacted".to_string(),
            prompt_text: None,
            has_reply_target: false,
            reply_target: None,
            requires_model: false,
            raw_identifiers_exposed: false,
        },
        HeptaKernelTelegramCandidateMaterial {
            update_id: Some(42),
            kind: "message:text".to_string(),
            prompt_text: Some("private prompt text".to_string()),
            has_reply_target: true,
            reply_target: Some(HeptaKernelTelegramReplyTargetMaterial {
                chat_id: 6476198178,
                reply_to_message_id: Some(7),
                raw_identifiers_exposed: false,
            }),
            requires_model: true,
            raw_identifiers_exposed: false,
        },
    ];

    let (candidate, decision, request) =
        hepta_kernel_telegram_first_model_candidate_with_duplicate_decision(
            &candidates,
            Some(42),
            "HEPTA_NATIVE_TELEGRAM_MODEL_TURN",
            true,
        );

    assert_eq!(
        candidate.as_ref().map(|candidate| candidate.kind.as_str()),
        Some("message:text")
    );
    assert_eq!(
        decision.as_ref().map(|decision| decision.decision),
        Some("model_candidate")
    );
    assert_eq!(request.duplicate_decision, "model_candidate");
    assert!(request.should_invoke_model);
    assert!(request.runner_invocation_allowed);
    assert_eq!(request.candidate_next_update_offset, Some(43));
    assert!(!request.raw_update_payload_exposed);
    assert!(!request.raw_prompt_text_exposed);
    assert!(!request.raw_chat_id_exposed);
    assert!(
        !serde_json::to_string(&request)
            .expect("serialize")
            .contains("private prompt text")
    );
}

#[test]
fn kernel_first_model_candidate_reports_missing_update_id_and_empty_queue() {
    let missing_update_id = vec![HeptaKernelTelegramCandidateMaterial {
        update_id: None,
        kind: "message:text".to_string(),
        prompt_text: Some("private prompt text".to_string()),
        has_reply_target: true,
        reply_target: Some(HeptaKernelTelegramReplyTargetMaterial {
            chat_id: 6476198178,
            reply_to_message_id: Some(7),
            raw_identifiers_exposed: false,
        }),
        requires_model: true,
        raw_identifiers_exposed: false,
    }];

    let (candidate, decision, request) =
        hepta_kernel_telegram_first_model_candidate_with_duplicate_decision(
            &missing_update_id,
            Some(42),
            "HEPTA_NATIVE_TELEGRAM_MODEL_TURN",
            true,
        );

    assert!(candidate.is_some());
    assert!(decision.is_none());
    assert_eq!(request.duplicate_decision, "missing_update_id");
    assert!(!request.should_invoke_model);
    assert!(!request.runner_invocation_allowed);
    assert!(!request.raw_prompt_text_exposed);

    let (_, empty_decision, empty_request) =
        hepta_kernel_telegram_first_model_candidate_with_duplicate_decision(
            &[],
            Some(42),
            "HEPTA_NATIVE_TELEGRAM_MODEL_TURN",
            true,
        );
    assert!(empty_decision.is_none());
    assert_eq!(empty_request.duplicate_decision, "no_model_candidate");
    assert!(!empty_request.candidate_present);
}

#[test]
fn kernel_runner_selection_preserves_mlx_and_child_fallbacks() {
    let mlx = select_hepta_kernel_telegram_runner(
        Some(" mlx-local/froggeric/Qwen3.6-35B-A3B-Uncensored-Heretic-MLX-4bit "),
        Some(" http://127.0.0.1:11436/v1/ "),
        Some(8_000),
        true,
        false,
    );
    assert_eq!(mlx.runner_kind, MLX_LOCAL_CHAT_COMPLETIONS_RUNNER_KIND);
    assert_eq!(
        mlx.mlx_model.as_deref(),
        Some("froggeric/Qwen3.6-35B-A3B-Uncensored-Heretic-MLX-4bit")
    );
    assert_eq!(
        mlx.mlx_base_url.as_deref(),
        Some(DEFAULT_TELEGRAM_MLX_BASE_URL)
    );
    assert_eq!(mlx.mlx_max_tokens, Some(MAX_TELEGRAM_MLX_MAX_TOKENS));
    assert!(mlx.local_network_call);
    assert!(!mlx.process_spawned_by_status);

    let child = select_hepta_kernel_telegram_runner(None, None, None, false, false);
    assert_eq!(child.runner_kind, HEPTA_EXEC_CHILD_RUNNER_KIND);
    assert!(!child.in_process_runner_enabled);
    assert!(child.process_spawned_by_status);
}

#[test]
fn kernel_runner_invocation_trims_output_and_redacts_errors() {
    let child = select_hepta_kernel_telegram_runner(None, None, None, false, false);
    let completed = invoke_hepta_kernel_telegram_runner_with_plan(
        &child,
        " private prompt ",
        |_, _| panic!("mlx runner must not be selected"),
        |_| panic!("in-process runner must not be selected"),
        |prompt| {
            assert_eq!(prompt, "private prompt");
            Ok(" child reply \n".to_string())
        },
    );
    assert_eq!(completed.status, "completed");
    assert!(completed.runner_invoked);
    assert!(completed.local_process_spawned);
    assert_eq!(
        completed.into_result().expect("model output"),
        "child reply"
    );

    let mlx = select_hepta_kernel_telegram_runner(
        Some("mlx-local/local-model"),
        Some(DEFAULT_TELEGRAM_MLX_BASE_URL),
        Some(128),
        false,
        false,
    );
    let failed = invoke_hepta_kernel_telegram_runner_with_plan(
        &mlx,
        "private prompt",
        |_, _| {
            Err(
                "local MLX chat-completions HTTP status 500; token 123456:ABCDEFGHIJKLMNOPQRSTUVWX"
                    .to_string(),
            )
        },
        |_| panic!("in-process runner must not be selected"),
        |_| panic!("child runner must not be selected"),
    );
    assert_eq!(failed.status, "attention");
    assert!(failed.runner_invoked);
    assert!(failed.local_network_call);
    assert_eq!(failed.error_kind, Some("local_mlx_http_status"));
    let error = failed.error.expect("redacted error");
    assert!(error.contains("telegram_model_runner_error[local_mlx_http_status]"));
    assert!(error.contains("[redacted-telegram-token]"));
    assert!(!error.contains("ABCDEFGHIJKLMNOPQRSTUVWX"));
}

#[test]
fn kernel_runner_invocation_rejects_empty_before_runner() {
    let plan = select_hepta_kernel_telegram_runner(None, None, None, true, false);
    let outcome = invoke_hepta_kernel_telegram_runner_with_plan(
        &plan,
        " \n ",
        |_, _| panic!("mlx runner must not run for empty prompt"),
        |_| panic!("in-process runner must not run for empty prompt"),
        |_| panic!("child runner must not run for empty prompt"),
    );

    assert_eq!(outcome.status, "attention");
    assert!(!outcome.runner_invoked);
    assert_eq!(outcome.error_kind, Some("empty_prompt"));
}

#[test]
fn kernel_mlx_chat_completion_body_is_bounded_and_openai_compatible() {
    let body = hepta_kernel_mlx_chat_completion_body("local-model", " private prompt ", 999_999)
        .expect("request body");

    assert_eq!(body["model"], "local-model");
    assert_eq!(body["messages"][0]["role"], "system");
    assert_eq!(body["messages"][1]["role"], "user");
    assert_eq!(body["messages"][1]["content"], "private prompt");
    assert_eq!(body["max_tokens"], MAX_TELEGRAM_MLX_MAX_TOKENS);
    assert_eq!(body["stream"], false);
    assert_eq!(body["strip_thinking"], true);

    assert!(
        hepta_kernel_mlx_chat_completion_body("   ", "prompt", 12)
            .expect_err("empty model rejected")
            .contains("selected model")
    );
    assert!(
        hepta_kernel_mlx_chat_completion_body("model", "   ", 12)
            .expect_err("empty prompt rejected")
            .contains("non-empty prompt")
    );
}

#[test]
fn kernel_openai_chat_completion_text_extractor_accepts_message_or_text() {
    let chat = json!({
        "choices": [{
            "message": { "role": "assistant", "content": "  local reply  " }
        }]
    });
    assert_eq!(
        extract_hepta_kernel_openai_chat_completion_text(&chat).expect("chat content"),
        "local reply"
    );

    let completion = json!({
        "choices": [{ "text": "  completion reply  " }]
    });
    assert_eq!(
        extract_hepta_kernel_openai_chat_completion_text(&completion).expect("completion text"),
        "completion reply"
    );

    let missing = json!({ "choices": [{ "message": { "content": "   " }}]});
    assert!(
        extract_hepta_kernel_openai_chat_completion_text(&missing)
            .expect_err("empty text rejected")
            .contains("did not include text")
    );
}

#[test]
fn kernel_poll_loop_and_receive_limit_policies_are_bounded() {
    assert!(hepta_kernel_telegram_poll_loop_should_spawn(
        true, true, true
    ));
    assert!(!hepta_kernel_telegram_poll_loop_should_spawn(
        false, true, true
    ));
    assert!(!hepta_kernel_telegram_poll_loop_should_spawn(
        true, false, true
    ));
    assert!(!hepta_kernel_telegram_poll_loop_should_spawn(
        true, true, false
    ));
    assert_eq!(
        hepta_kernel_telegram_poll_loop_interval_ms_policy(1),
        MIN_TELEGRAM_POLL_LOOP_INTERVAL_MS
    );
    assert_eq!(
        hepta_kernel_telegram_poll_loop_interval_ms_policy(1_500),
        1_500
    );
    assert_eq!(
        hepta_kernel_telegram_poll_loop_interval_ms_policy(999_999),
        MAX_TELEGRAM_POLL_LOOP_INTERVAL_MS
    );
    assert_eq!(hepta_kernel_telegram_receive_limit_policy(0), 1);
    assert_eq!(hepta_kernel_telegram_receive_limit_policy(7), 7);
    assert_eq!(hepta_kernel_telegram_receive_limit_policy(999), 20);
}

#[test]
fn kernel_telegram_poll_loop_status_is_side_effect_free() {
    let disabled =
        build_hepta_kernel_telegram_poll_loop_status(HeptaKernelTelegramPollLoopStatusInput {
            requested: false,
            poll_ms: 500,
            poll_loop_gate_env: "POLL",
            poll_loop_gate_enabled: true,
            delivery_approval_gate_env: "APPROVAL",
            delivery_approval_gate_enabled: true,
            live_read_gate_env: "READ",
            model_turn_gate_env: "MODEL",
            send_gate_env: "SEND",
        });
    assert_eq!(disabled.status, "disabled");
    assert!(!disabled.loop_invokes_drain_once);

    let approval_required =
        build_hepta_kernel_telegram_poll_loop_status(HeptaKernelTelegramPollLoopStatusInput {
            requested: true,
            poll_ms: 500,
            poll_loop_gate_env: "POLL",
            poll_loop_gate_enabled: true,
            delivery_approval_gate_env: "APPROVAL",
            delivery_approval_gate_enabled: false,
            live_read_gate_env: "READ",
            model_turn_gate_env: "MODEL",
            send_gate_env: "SEND",
        });
    assert_eq!(approval_required.status, "approval_required");
    assert!(!approval_required.worker_spawned_by_status);

    let armed =
        build_hepta_kernel_telegram_poll_loop_status(HeptaKernelTelegramPollLoopStatusInput {
            requested: true,
            poll_ms: 1_000,
            poll_loop_gate_env: "POLL",
            poll_loop_gate_enabled: true,
            delivery_approval_gate_env: "APPROVAL",
            delivery_approval_gate_enabled: true,
            live_read_gate_env: "READ",
            model_turn_gate_env: "MODEL",
            send_gate_env: "SEND",
        });
    assert_eq!(armed.status, "armed");
    assert!(armed.loop_invokes_drain_once);
    assert!(!armed.external_network_read_by_status);
    assert!(!armed.external_send_by_status);
    assert!(!armed.raw_token_exposed);
}

#[test]
fn kernel_soak_and_time_policies_clamp_and_default() {
    assert_eq!(
        hepta_kernel_telegram_soak_min_poll_iterations_policy(None),
        DEFAULT_TELEGRAM_SOAK_MIN_POLLS
    );
    assert_eq!(
        hepta_kernel_telegram_soak_min_poll_iterations_policy(Some(0)),
        1
    );
    assert_eq!(
        hepta_kernel_telegram_soak_min_poll_iterations_policy(Some(999_999)),
        MAX_TELEGRAM_SOAK_MIN_POLLS
    );
    assert_eq!(
        hepta_kernel_telegram_soak_max_attention_count_policy(None),
        DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION
    );
    assert_eq!(
        hepta_kernel_telegram_soak_max_attention_count_policy(Some(999_999)),
        MAX_TELEGRAM_SOAK_MAX_ATTENTION
    );
    assert_eq!(
        hepta_kernel_telegram_soak_max_observed_age_ms_policy(None),
        DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS
    );
    assert_eq!(
        hepta_kernel_telegram_soak_max_observed_age_ms_policy(Some(1)),
        1_000
    );
    assert_eq!(
        hepta_kernel_telegram_soak_max_observed_age_ms_policy(Some(999_999_999)),
        MAX_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS
    );
    assert_eq!(hepta_kernel_telegram_system_time_unix_ms(UNIX_EPOCH), 0);
    assert_eq!(
        hepta_kernel_telegram_system_time_unix_ms(UNIX_EPOCH + Duration::from_millis(42)),
        42
    );
    assert_eq!(
        hepta_kernel_telegram_system_time_unix_ms(UNIX_EPOCH - Duration::from_millis(1)),
        0
    );
}

#[test]
fn kernel_telegram_delivery_lifecycle_policy_redacts_and_classifies_retry() {
    let record = hepta_kernel_telegram_delivery_lifecycle_record(
        "failed",
        Some(42),
        true,
        true,
        Some(false),
        false,
        Some("transient token=123456789:abcdefghijklmnopqrstuvwxyz timeout"),
        1_777_777,
    );

    assert_eq!(
        record["store_identifier"],
        HEPTA_KERNEL_TELEGRAM_DELIVERY_STORE_IDENTIFIER
    );
    assert_eq!(record["entry_id"], "telegram:next-offset:42");
    assert_eq!(record["idempotency_key"], "telegram:next-offset:42");
    assert_eq!(record["created_unix_seconds"], 1_777_777);
    assert_eq!(record["payload_count"], 1);
    assert_eq!(record["payload_text_chunk_count"], 1);
    assert_eq!(record["failed"], true);
    assert_eq!(record["acked"], false);
    assert_eq!(record["retry_scheduled"], true);
    assert_eq!(record["next_retry_count"], 1);
    assert_eq!(record["next_retry_backoff_ms"], 5_000);
    assert_eq!(
        record["max_retries"],
        HEPTA_KERNEL_TELEGRAM_DELIVERY_MAX_RETRIES
    );
    assert_eq!(record["raw_chat_id_logged"], false);
    assert_eq!(record["raw_message_id_logged"], false);
    assert_eq!(record["raw_token_logged"], false);
    assert_eq!(
        record["error"],
        "transient [redacted-telegram-token] timeout"
    );
}

#[test]
fn kernel_telegram_delivery_ledger_status_summarizes_redacted_ack_evidence() {
    let raw = concat!(
        r#"{"stage":"enqueued","created_unix_seconds":1,"provider_message_id_present":false,"content_logged":false,"raw_chat_id_logged":false,"raw_message_id_logged":false,"raw_token_logged":false}"#,
        "\n",
        r#"{"stage":"acked","created_unix_seconds":2,"provider_message_id_present":true,"content_logged":false,"raw_chat_id_logged":false,"raw_message_id_logged":false,"raw_token_logged":false}"#,
        "\n",
    );

    let status = build_hepta_kernel_telegram_delivery_ledger_status(
        HeptaKernelTelegramDeliveryLedgerStatusInput {
            requested: true,
            ledger_path: ".hepta/telegram/delivery-ledger.jsonl",
            ledger_file_present: true,
            ledger_updated_at_unix_ms: Some(42),
            raw_jsonl: Some(raw),
            read_error: None,
        },
    );

    assert_eq!(status.status, "ready");
    assert_eq!(status.ledger_updated_at_unix_ms, Some(42));
    assert_eq!(status.line_count, 2);
    assert_eq!(status.valid_json_line_count, 2);
    assert_eq!(status.acked_count, 1);
    assert_eq!(status.failed_count, 0);
    assert_eq!(status.latest_stage.as_deref(), Some("acked"));
    assert_eq!(status.latest_created_unix_seconds, Some(2));
    assert_eq!(status.latest_acked_created_unix_seconds, Some(2));
    assert!(status.provider_message_id_present);
    assert!(status.durable_delivery_evidence_present);
    assert!(!status.raw_response_text_logged);
    assert!(!status.raw_chat_id_logged);
    assert!(!status.raw_message_id_logged);
    assert!(!status.raw_token_logged);
}

#[test]
fn kernel_telegram_delivery_ledger_status_flags_invalid_or_raw_logging() {
    let raw = concat!(
        r#"{"stage":"acked","created_unix_seconds":2,"provider_message_id_present":true,"content_logged":true}"#,
        "\n",
        "not-json",
        "\n",
    );

    let status = build_hepta_kernel_telegram_delivery_ledger_status(
        HeptaKernelTelegramDeliveryLedgerStatusInput {
            requested: true,
            ledger_path: ".hepta/telegram/delivery-ledger.jsonl",
            ledger_file_present: true,
            ledger_updated_at_unix_ms: None,
            raw_jsonl: Some(raw),
            read_error: None,
        },
    );

    assert_eq!(status.status, "attention");
    assert!(!status.jsonl_valid);
    assert_eq!(status.invalid_json_line_count, 1);
    assert!(status.raw_response_text_logged);
}

#[test]
fn kernel_telegram_delivery_ledger_status_handles_disabled_missing_and_read_error() {
    let disabled = build_hepta_kernel_telegram_delivery_ledger_status(
        HeptaKernelTelegramDeliveryLedgerStatusInput {
            requested: false,
            ledger_path: ".hepta/telegram/delivery-ledger.jsonl",
            ledger_file_present: true,
            ledger_updated_at_unix_ms: Some(42),
            raw_jsonl: Some("ignored"),
            read_error: None,
        },
    );
    assert_eq!(disabled.status, "disabled");
    assert!(!disabled.ledger_file_present);
    assert_eq!(disabled.line_count, 0);

    let missing = build_hepta_kernel_telegram_delivery_ledger_status(
        HeptaKernelTelegramDeliveryLedgerStatusInput {
            requested: true,
            ledger_path: ".hepta/telegram/delivery-ledger.jsonl",
            ledger_file_present: false,
            ledger_updated_at_unix_ms: None,
            raw_jsonl: None,
            read_error: None,
        },
    );
    assert_eq!(missing.status, "missing");

    let read_error = build_hepta_kernel_telegram_delivery_ledger_status(
        HeptaKernelTelegramDeliveryLedgerStatusInput {
            requested: true,
            ledger_path: ".hepta/telegram/delivery-ledger.jsonl",
            ledger_file_present: true,
            ledger_updated_at_unix_ms: Some(7),
            raw_jsonl: None,
            read_error: Some("failed token=123456789:abcdefghijklmnopqrstuvwxyz"),
        },
    );
    assert_eq!(read_error.status, "attention");
    assert_eq!(
        read_error.error.as_deref(),
        Some("failed [redacted-telegram-token]")
    );
}

#[test]
fn kernel_telegram_delivery_error_classification_and_backoff_are_stable() {
    assert!(hepta_kernel_telegram_delivery_error_is_permanent(Some(
        "Forbidden: bot was blocked by the user"
    )));
    assert!(hepta_kernel_telegram_delivery_error_is_permanent(Some(
        "Bad Request: chat not found"
    )));
    assert!(!hepta_kernel_telegram_delivery_error_is_permanent(Some(
        "Too Many Requests: retry after 1"
    )));
    assert_eq!(hepta_kernel_telegram_delivery_backoff_ms(0), 0);
    assert_eq!(hepta_kernel_telegram_delivery_backoff_ms(1), 5_000);
    assert_eq!(hepta_kernel_telegram_delivery_backoff_ms(2), 25_000);
    assert_eq!(hepta_kernel_telegram_delivery_backoff_ms(3), 120_000);
    assert_eq!(hepta_kernel_telegram_delivery_backoff_ms(4), 600_000);
}

#[test]
fn kernel_telegram_send_plan_is_side_effect_free() {
    let disabled = HeptaKernelTelegramSendPlan::disabled();
    assert!(!disabled.send_plan_ready);
    assert_eq!(disabled.method, "disabled");
    assert!(!disabled.delivery_performed_by_status);
    assert!(!disabled.raw_token_exposed);

    let ready = HeptaKernelTelegramSendPlan::ready();
    assert!(ready.send_plan_ready);
    assert_eq!(ready.method, "sendMessage");
    assert!(!ready.request_body_materialized_by_status);
    assert!(!ready.delivery_performed_by_status);
    assert!(!ready.raw_response_text_exposed);
    assert!(!ready.raw_chat_id_exposed);
    assert!(!ready.raw_message_id_exposed);
    assert!(!ready.raw_token_exposed);
}

#[test]
fn kernel_telegram_receive_shell_readiness_redacts_and_blocks_before_bot_api() {
    let token_block = plan_hepta_kernel_telegram_receive_once_shell_readiness(
        HeptaKernelTelegramReceiveOnceShellReadinessInput {
            token_error: Some("bad token 123456789:abcdefghijklmnopqrstuvwxyz"),
            cursor_file_present: false,
            cursor_parse_ok: true,
            cursor_error: None,
        },
    );

    assert_eq!(token_block.status, "attention");
    assert!(!token_block.may_call_bot_api);
    let error = token_block.error.expect("redacted token error");
    assert!(error.contains("[redacted-telegram-token]"));
    assert!(!error.contains("abcdefghijklmnopqrstuvwxyz"));

    let cursor_block = plan_hepta_kernel_telegram_receive_once_shell_readiness(
        HeptaKernelTelegramReceiveOnceShellReadinessInput {
            token_error: None,
            cursor_file_present: true,
            cursor_parse_ok: false,
            cursor_error: None,
        },
    );
    assert_eq!(cursor_block.status, "attention");
    assert!(!cursor_block.may_call_bot_api);
    assert_eq!(
        cursor_block.error.as_deref(),
        Some("Telegram cursor state is not readable")
    );

    let ready = plan_hepta_kernel_telegram_receive_once_shell_readiness(
        HeptaKernelTelegramReceiveOnceShellReadinessInput {
            token_error: None,
            cursor_file_present: true,
            cursor_parse_ok: true,
            cursor_error: None,
        },
    );
    assert_eq!(ready.status, "planned");
    assert!(ready.error.is_none());
    assert!(ready.may_call_bot_api);
}

#[test]
fn kernel_telegram_drain_shell_readiness_preserves_failure_order() {
    let cursor_block = plan_hepta_kernel_telegram_drain_once_shell_readiness(
        HeptaKernelTelegramDrainOnceShellReadinessInput {
            cursor_file_present: true,
            cursor_parse_ok: false,
            cursor_error: Some("cursor JSON is malformed"),
            config_ready: false,
            token_error: Some("bad token 123456789:abcdefghijklmnopqrstuvwxyz"),
        },
    );
    assert_eq!(cursor_block.status, "attention");
    assert!(!cursor_block.may_call_bot_api);
    assert_eq!(
        cursor_block.error.as_deref(),
        Some("cursor JSON is malformed")
    );

    let config_block = plan_hepta_kernel_telegram_drain_once_shell_readiness(
        HeptaKernelTelegramDrainOnceShellReadinessInput {
            cursor_file_present: false,
            cursor_parse_ok: true,
            cursor_error: None,
            config_ready: false,
            token_error: Some("bad token 123456789:abcdefghijklmnopqrstuvwxyz"),
        },
    );
    assert_eq!(config_block.status, "attention");
    assert!(!config_block.may_call_bot_api);
    assert_eq!(
        config_block.error.as_deref(),
        Some("Telegram config, token shape, or binding is not ready")
    );

    let token_block = plan_hepta_kernel_telegram_drain_once_shell_readiness(
        HeptaKernelTelegramDrainOnceShellReadinessInput {
            cursor_file_present: false,
            cursor_parse_ok: true,
            cursor_error: None,
            config_ready: true,
            token_error: Some("bad token 123456789:abcdefghijklmnopqrstuvwxyz"),
        },
    );
    assert_eq!(token_block.status, "attention");
    assert!(!token_block.may_call_bot_api);
    let error = token_block.error.expect("redacted token error");
    assert!(error.contains("[redacted-telegram-token]"));
    assert!(!error.contains("abcdefghijklmnopqrstuvwxyz"));

    let ready = plan_hepta_kernel_telegram_drain_once_shell_readiness(
        HeptaKernelTelegramDrainOnceShellReadinessInput {
            cursor_file_present: true,
            cursor_parse_ok: true,
            cursor_error: None,
            config_ready: true,
            token_error: None,
        },
    );
    assert_eq!(ready.status, "planned");
    assert!(ready.error.is_none());
    assert!(ready.may_call_bot_api);
}

#[test]
fn kernel_telegram_drain_once_preflight_plans_pipeline_without_side_effects() {
    let gated = telegram_kernel_gates(true, true, false, true);
    let gated_plan = plan_hepta_kernel_telegram_drain_once_preflight(
        HeptaKernelTelegramDrainOncePreflightInput {
            requested: true,
            gates: &gated,
        },
    );

    assert_eq!(gated_plan.status, "gated");
    assert!(
        gated_plan
            .error
            .as_deref()
            .unwrap_or_default()
            .contains("HEPTA_NATIVE_TELEGRAM_MODEL_TURN")
    );
    assert!(gated_plan.cursor_plan.duplicate_suppression_ready);
    assert_eq!(gated_plan.inspection.update_count, 0);
    assert_eq!(gated_plan.model_turn_plan.candidate_count, 0);
    assert!(gated_plan.invocation_request.request_builder_ready);
    assert!(!gated_plan.invocation_request.runner_invocation_allowed);
    assert_eq!(gated_plan.model_execution.status, "gated");
    assert!(gated_plan.send_plan.send_plan_ready);
    assert_eq!(gated_plan.send_execution.status, "waiting_model_output");
    assert!(gated_plan.status_probe_executes_pipeline);
    assert!(!gated_plan.send_execution.external_send);
    assert!(!gated_plan.send_execution.cursor_written);

    let ready = telegram_kernel_gates(true, true, true, true);
    let ready_plan = plan_hepta_kernel_telegram_drain_once_preflight(
        HeptaKernelTelegramDrainOncePreflightInput {
            requested: true,
            gates: &ready,
        },
    );
    assert_eq!(ready_plan.status, "planned");
    assert!(ready_plan.error.is_none());
    assert!(ready_plan.status_probe_executes_pipeline);
    assert_eq!(ready_plan.model_execution.status, "waiting_candidate");
}

#[test]
fn kernel_telegram_drain_once_api_result_redacts_and_preserves_candidate_plan() {
    let gates = telegram_kernel_gates(true, true, true, true);
    let api = json!({
        "ok": true,
        "result": [{
            "update_id": 47,
            "message": {
                "message_id": 9,
                "chat": { "id": 6476198178i64 },
                "text": "private prompt"
            }
        }]
    });

    let plan = plan_hepta_kernel_telegram_drain_once_api_result(
        HeptaKernelTelegramDrainOnceApiResultInput {
            requested: true,
            gates: &gates,
            next_update_offset: Some(47),
            api_result: Ok(&api),
        },
    );

    assert_eq!(plan.status, "planned");
    assert!(plan.should_execute_pipeline);
    assert_eq!(plan.bot_api_ok, Some(true));
    assert_eq!(plan.local_next_update_offset, Some(48));
    assert_eq!(plan.inspection.allowed_update_count, 1);
    assert_eq!(plan.model_turn_plan.text_candidate_count, 1);
    assert!(plan.invocation_request.candidate_present);
    assert_eq!(
        plan.invocation_request.duplicate_decision,
        "model_candidate"
    );
    assert!(plan.invocation_request.prompt_material_in_memory);
    assert!(!plan.invocation_request.prompt_material_serialized);
    assert!(!plan.invocation_request.raw_prompt_text_exposed);

    let ok_false = json!({
        "ok": false,
        "description": "bad token 123456789:abcdefghijklmnopqrstuvwxyz"
    });
    let blocked = plan_hepta_kernel_telegram_drain_once_api_result(
        HeptaKernelTelegramDrainOnceApiResultInput {
            requested: true,
            gates: &gates,
            next_update_offset: Some(47),
            api_result: Ok(&ok_false),
        },
    );
    assert_eq!(blocked.status, "attention");
    assert!(!blocked.should_execute_pipeline);
    let error = blocked.error.expect("redacted error");
    assert!(error.contains("[redacted-telegram-token]"));
    assert!(!error.contains("abcdefghijklmnopqrstuvwxyz"));

    let conflict = plan_hepta_kernel_telegram_drain_once_api_result(
        HeptaKernelTelegramDrainOnceApiResultInput {
            requested: true,
            gates: &gates,
            next_update_offset: Some(47),
            api_result: Err(
                "Telegram Bot API getUpdates HTTP status 409; description=Conflict: terminated by other getUpdates request",
            ),
        },
    );
    assert_eq!(conflict.status, "busy");
    assert!(!conflict.should_execute_pipeline);
    assert!(!conflict.invocation_request.candidate_present);
}

#[test]
fn kernel_telegram_drain_once_status_summarizes_pipeline_without_payload_leaks() {
    let gates = telegram_kernel_gates(true, true, true, true);
    let plan = plan_hepta_kernel_telegram_drain_once_preflight(
        HeptaKernelTelegramDrainOncePreflightInput {
            requested: true,
            gates: &gates,
        },
    );
    let mut model_execution = plan.model_execution.clone();
    model_execution.session_runner_invoked = true;
    let mut send_execution = plan.send_execution.clone();
    send_execution.send_attempted = true;
    send_execution.cursor_written = true;
    send_execution.external_network_write = true;
    send_execution.external_send = true;

    let status =
        build_hepta_kernel_telegram_drain_once_status(HeptaKernelTelegramDrainOnceStatusInput {
            requested: true,
            status: "drained",
            gates,
            config: ready_telegram_config(),
            execution_plan: plan.execution_plan,
            cursor_plan: plan.cursor_plan,
            inspection: plan.inspection,
            model_turn_plan: plan.model_turn_plan,
            invocation_request: plan.invocation_request,
            model_execution,
            send_plan: plan.send_plan,
            send_request: plan.send_request,
            send_execution,
            bot_api_ok: Some(true),
            local_next_update_offset: Some(48),
            get_updates_offset: Some(47),
            live_read_started: true,
            external_network_read: true,
            error: None,
        });

    assert_eq!(status.status, "drained");
    assert!(status.model_turn_started);
    assert!(status.send_started);
    assert!(status.cursor_written);
    assert!(status.external_network_read);
    assert!(status.external_network_write);
    assert!(status.external_send);
    assert_eq!(status.bot_api_ok, Some(true));
    assert_eq!(status.local_next_update_offset, Some(48));
    assert!(!status.raw_update_payload_exposed);
    assert!(!status.raw_prompt_text_exposed);
    assert!(!status.raw_response_text_exposed);
    assert!(!status.raw_token_exposed);
}

#[test]
fn kernel_live_soak_observation_state_accumulates_redacted_report() {
    let gates = telegram_kernel_gates(true, true, true, true);
    let plan = plan_hepta_kernel_telegram_drain_once_preflight(
        HeptaKernelTelegramDrainOncePreflightInput {
            requested: true,
            gates: &gates,
        },
    );
    let status =
        build_hepta_kernel_telegram_drain_once_status(HeptaKernelTelegramDrainOnceStatusInput {
            requested: true,
            status: "attention",
            gates,
            config: ready_telegram_config(),
            execution_plan: plan.execution_plan,
            cursor_plan: plan.cursor_plan,
            inspection: plan.inspection,
            model_turn_plan: plan.model_turn_plan,
            invocation_request: plan.invocation_request,
            model_execution: plan.model_execution,
            send_plan: plan.send_plan,
            send_request: plan.send_request,
            send_execution: plan.send_execution,
            bot_api_ok: Some(false),
            local_next_update_offset: Some(48),
            get_updates_offset: Some(47),
            live_read_started: true,
            external_network_read: true,
            error: Some("bad token 123456789:abcdefghijklmnopqrstuvwxyz".to_string()),
        });

    let mut state = HeptaKernelTelegramLiveSoakObservationState::default();
    state.observe(&status, 1_000_500);
    let report = state.report();

    assert_eq!(report.poll_iterations, 1);
    assert_eq!(report.attention_count, 1);
    assert_eq!(report.last_status.as_deref(), Some("attention"));
    assert_eq!(report.last_bot_api_ok, Some(false));
    assert_eq!(report.last_get_updates_offset, Some(47));
    assert_eq!(report.last_local_next_update_offset, Some(48));
    let error = report.last_error.expect("redacted observation error");
    assert!(error.contains("[redacted-telegram-token]"));
    assert!(!error.contains("abcdefghijklmnopqrstuvwxyz"));
    assert!(!report.raw_update_payload_exposed);
    assert!(!report.raw_prompt_text_exposed);
    assert!(!report.raw_response_text_exposed);
    assert!(!report.raw_token_exposed);
}

#[test]
fn kernel_telegram_production_readiness_is_ready_after_clean_guarded_soak() {
    let poll_loop = ready_kernel_poll_loop_status();
    let cursor = ready_kernel_cursor_status();
    let delivery_ledger = ready_kernel_delivery_ledger_status();
    let guards = ready_kernel_production_guards();
    let observation = kernel_live_soak_observation(
        DEFAULT_TELEGRAM_SOAK_MIN_POLLS,
        0,
        Some("planned"),
        Some(true),
    );

    let readiness = build_hepta_kernel_telegram_production_readiness_status(
        HeptaKernelTelegramProductionReadinessInput {
            requested: true,
            poll_loop_status: &poll_loop,
            cursor_status: &cursor,
            delivery_ledger_status: &delivery_ledger,
            production_guards: &guards,
            observation: &observation,
            min_poll_iterations_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MIN_POLLS",
            min_poll_iterations: DEFAULT_TELEGRAM_SOAK_MIN_POLLS,
            max_attention_count_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_ATTENTION",
            max_attention_count: DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION,
            max_observed_age_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS",
            max_observed_age_ms: DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS,
            now_unix_ms: TEST_NOW_MS,
        },
    );

    assert!(readiness.ready);
    assert_eq!(readiness.status, "ready");
    assert!(readiness.poll_loop_armed);
    assert!(readiness.cursor_ready);
    assert!(readiness.production_guards_ready);
    assert!(readiness.observation_ready);
    assert!(readiness.observation_fresh);
    assert!(readiness.durable_cursor_evidence_present);
    assert!(!readiness.durable_delivery_evidence_required);
    assert!(readiness.durable_delivery_evidence_fresh);
    assert!(readiness.delivery_ledger_ready);
    assert!(readiness.attention_budget_ok);
    assert!(readiness.recent_bot_api_ok);
    assert!(readiness.redaction_guards_ok);
    assert!(readiness.readiness_blockers.is_empty());
    assert!(readiness.readiness_warnings.is_empty());
    assert!(!readiness.raw_update_payload_exposed);
    assert!(!readiness.raw_prompt_text_exposed);
    assert!(!readiness.raw_response_text_exposed);
    assert!(!readiness.raw_token_exposed);
}

#[test]
fn kernel_telegram_production_readiness_blocks_missing_delivery_evidence_after_send() {
    let poll_loop = ready_kernel_poll_loop_status();
    let cursor = ready_kernel_cursor_status();
    let mut delivery_ledger = ready_kernel_delivery_ledger_status();
    delivery_ledger.status = "empty";
    delivery_ledger.acked_count = 0;
    delivery_ledger.provider_message_id_present = false;
    delivery_ledger.durable_delivery_evidence_present = false;
    delivery_ledger.latest_acked_created_unix_seconds = None;
    let guards = ready_kernel_production_guards();
    let mut observation = kernel_live_soak_observation(
        DEFAULT_TELEGRAM_SOAK_MIN_POLLS,
        0,
        Some("drained"),
        Some(true),
    );
    observation.drained_count = 1;
    observation.send_started_count = 1;
    observation.cursor_written_count = 1;
    observation.external_send_count = 1;
    observation.last_send_started = true;
    observation.last_cursor_written = true;
    observation.last_external_send = true;

    let readiness = build_hepta_kernel_telegram_production_readiness_status(
        HeptaKernelTelegramProductionReadinessInput {
            requested: true,
            poll_loop_status: &poll_loop,
            cursor_status: &cursor,
            delivery_ledger_status: &delivery_ledger,
            production_guards: &guards,
            observation: &observation,
            min_poll_iterations_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MIN_POLLS",
            min_poll_iterations: DEFAULT_TELEGRAM_SOAK_MIN_POLLS,
            max_attention_count_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_ATTENTION",
            max_attention_count: DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION,
            max_observed_age_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS",
            max_observed_age_ms: DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS,
            now_unix_ms: TEST_NOW_MS,
        },
    );

    assert!(!readiness.ready);
    assert_eq!(readiness.status, "attention");
    assert!(readiness.durable_delivery_evidence_required);
    assert!(!readiness.durable_delivery_evidence_present);
    assert!(!readiness.delivery_ledger_ready);
    assert!(
        readiness
            .readiness_blockers
            .contains(&"durable_delivery_evidence_missing")
    );
}

#[test]
fn kernel_telegram_live_soak_status_reports_soaking_after_ready_readiness() {
    let poll_loop = ready_kernel_poll_loop_status();
    let cursor = ready_kernel_cursor_status();
    let delivery_ledger = ready_kernel_delivery_ledger_status();
    let guards = ready_kernel_production_guards();
    let observation = kernel_live_soak_observation(
        DEFAULT_TELEGRAM_SOAK_MIN_POLLS,
        0,
        Some("planned"),
        Some(true),
    );
    let readiness = build_hepta_kernel_telegram_production_readiness_status(
        HeptaKernelTelegramProductionReadinessInput {
            requested: true,
            poll_loop_status: &poll_loop,
            cursor_status: &cursor,
            delivery_ledger_status: &delivery_ledger,
            production_guards: &guards,
            observation: &observation,
            min_poll_iterations_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MIN_POLLS",
            min_poll_iterations: DEFAULT_TELEGRAM_SOAK_MIN_POLLS,
            max_attention_count_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_ATTENTION",
            max_attention_count: DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION,
            max_observed_age_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS",
            max_observed_age_ms: DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS,
            now_unix_ms: TEST_NOW_MS,
        },
    );

    let status =
        build_hepta_kernel_telegram_live_soak_status(HeptaKernelTelegramLiveSoakStatusInput {
            requested: true,
            poll_loop_status: poll_loop,
            cursor_status: cursor,
            delivery_ledger_status: delivery_ledger,
            production_guards: guards,
            production_readiness: readiness,
            observation,
        });

    assert_eq!(status.status, "soaking");
    assert_eq!(status.endpoint, "/api/telegram-live-soak");
    assert!(status.health_ready);
    assert!(status.side_effect_free);
    assert!(!status.raw_update_payload_exposed);
    assert!(!status.raw_prompt_text_exposed);
    assert!(!status.raw_response_text_exposed);
    assert!(!status.raw_token_exposed);
}

#[test]
fn kernel_telegram_live_soak_allows_bounded_recovered_attention_history() {
    let poll_loop = ready_kernel_poll_loop_status();
    let cursor = ready_kernel_cursor_status();
    let delivery_ledger = ready_kernel_delivery_ledger_status();
    let guards = ready_kernel_production_guards();
    let observation = kernel_live_soak_observation(
        DEFAULT_TELEGRAM_SOAK_MIN_POLLS.saturating_add(10),
        DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION,
        Some("planned"),
        Some(true),
    );

    let readiness = build_hepta_kernel_telegram_production_readiness_status(
        HeptaKernelTelegramProductionReadinessInput {
            requested: true,
            poll_loop_status: &poll_loop,
            cursor_status: &cursor,
            delivery_ledger_status: &delivery_ledger,
            production_guards: &guards,
            observation: &observation,
            min_poll_iterations_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MIN_POLLS",
            min_poll_iterations: DEFAULT_TELEGRAM_SOAK_MIN_POLLS,
            max_attention_count_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_ATTENTION",
            max_attention_count: DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION,
            max_observed_age_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS",
            max_observed_age_ms: DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS,
            now_unix_ms: TEST_NOW_MS,
        },
    );

    assert!(readiness.ready);
    assert_eq!(readiness.status, "ready");
    assert!(readiness.attention_budget_ok);
    assert!(
        !readiness
            .readiness_blockers
            .contains(&"attention_budget_exceeded")
    );
}

#[test]
fn kernel_telegram_live_soak_status_surfaces_attention_observations() {
    let poll_loop = ready_kernel_poll_loop_status();
    let cursor = ready_kernel_cursor_status();
    let delivery_ledger = ready_kernel_delivery_ledger_status();
    let guards = ready_kernel_production_guards();
    let observation = kernel_live_soak_observation(
        DEFAULT_TELEGRAM_SOAK_MIN_POLLS,
        1,
        Some("attention"),
        Some(false),
    );
    let readiness = build_hepta_kernel_telegram_production_readiness_status(
        HeptaKernelTelegramProductionReadinessInput {
            requested: true,
            poll_loop_status: &poll_loop,
            cursor_status: &cursor,
            delivery_ledger_status: &delivery_ledger,
            production_guards: &guards,
            observation: &observation,
            min_poll_iterations_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MIN_POLLS",
            min_poll_iterations: DEFAULT_TELEGRAM_SOAK_MIN_POLLS,
            max_attention_count_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_ATTENTION",
            max_attention_count: DEFAULT_TELEGRAM_SOAK_MAX_ATTENTION,
            max_observed_age_env: "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS",
            max_observed_age_ms: DEFAULT_TELEGRAM_SOAK_MAX_OBSERVED_AGE_MS,
            now_unix_ms: TEST_NOW_MS,
        },
    );

    let status =
        build_hepta_kernel_telegram_live_soak_status(HeptaKernelTelegramLiveSoakStatusInput {
            requested: true,
            poll_loop_status: poll_loop,
            cursor_status: cursor,
            delivery_ledger_status: delivery_ledger,
            production_guards: guards,
            production_readiness: readiness,
            observation,
        });

    assert_eq!(status.status, "attention");
    assert!(!status.health_ready);
    assert!(!status.production_readiness.attention_budget_ok);
    assert_eq!(status.observation.last_status.as_deref(), Some("attention"));
}

#[test]
fn kernel_exec_child_args_are_ephemeral_read_only_and_capture_last_message() {
    let args =
        hepta_kernel_exec_child_args("/tmp/hepta-telegram-last-message.txt", "private prompt");

    assert_eq!(args[0], "-c");
    assert_eq!(args[1], "approval_policy=\"never\"");
    assert_eq!(args[2], "exec");
    assert!(args.contains(&"--skip-git-repo-check".to_string()));
    assert!(args.contains(&"--ephemeral".to_string()));
    assert!(args.contains(&"--ignore-rules".to_string()));
    assert_eq!(
        args.windows(2)
            .find(|pair| pair[0] == "--sandbox")
            .map(|pair| pair[1].as_str()),
        Some("read-only")
    );
    assert_eq!(
        args.windows(2)
            .find(|pair| pair[0] == "--output-last-message")
            .map(|pair| pair[1].as_str()),
        Some("/tmp/hepta-telegram-last-message.txt")
    );
    assert_eq!(args.last().map(String::as_str), Some("private prompt"));
}

#[test]
fn kernel_exec_child_final_message_extractor_trims_and_rejects_empty() {
    assert_eq!(
        extract_hepta_kernel_exec_child_final_message("  final answer \n").expect("final message"),
        "final answer"
    );
    assert!(
        extract_hepta_kernel_exec_child_final_message(" \n\t ")
            .expect_err("empty output rejected")
            .contains("empty final message")
    );
}

#[test]
fn kernel_exec_child_status_policy_reports_exit_code_or_signal() {
    assert_eq!(hepta_kernel_exec_child_status_error(true, Some(0)), None);
    assert!(
        hepta_kernel_exec_child_status_error(false, Some(7))
            .expect("nonzero status")
            .contains("7")
    );
    assert!(
        hepta_kernel_exec_child_status_error(false, None)
            .expect("signal status")
            .contains("signal")
    );
}

#[test]
fn kernel_model_failure_fallback_policy_requires_safe_delivery_context() {
    assert!(hepta_kernel_telegram_model_failure_fallback_allowed(
        true,
        true,
        "attention",
        true,
        true
    ));
    assert!(!hepta_kernel_telegram_model_failure_fallback_allowed(
        false,
        true,
        "attention",
        true,
        true
    ));
    assert!(!hepta_kernel_telegram_model_failure_fallback_allowed(
        true,
        false,
        "attention",
        true,
        true
    ));
    assert!(!hepta_kernel_telegram_model_failure_fallback_allowed(
        true,
        true,
        "completed",
        true,
        true
    ));
    assert!(!hepta_kernel_telegram_model_failure_fallback_allowed(
        true,
        true,
        "attention",
        false,
        true
    ));
    assert!(!hepta_kernel_telegram_model_failure_fallback_allowed(
        true,
        true,
        "attention",
        true,
        false
    ));
    assert!(
        HEPTA_KERNEL_TELEGRAM_MODEL_FAILURE_FALLBACK_MESSAGE
            .contains("本地模型这次响应超时或失败了")
    );
}

#[test]
fn kernel_drain_pipeline_delivery_plan_uses_model_output_first() {
    let plan = plan_hepta_kernel_telegram_drain_pipeline_delivery(
        HeptaKernelTelegramDrainPipelineDeliveryInput {
            model_output_present: true,
            model_failure_fallback_enabled: true,
            model_execution_session_runner_invoked: true,
            model_execution_status: "completed",
            reply_target_available: true,
            candidate_next_update_offset: Some(43),
            send_gate_env: "HEPTA_NATIVE_TELEGRAM_SEND",
            send_gate_enabled: true,
        },
    );

    assert!(!plan.model_failure_fallback_allowed);
    assert!(plan.delivery_output_present);
    assert!(plan.send_request.send_allowed);
    assert!(plan.send_request.model_output_present);
    assert!(plan.send_request.cursor_commit_allowed_after_delivery);
    assert!(!plan.send_request.raw_response_text_exposed);
    assert!(!plan.send_request.raw_chat_id_exposed);
    assert!(!plan.send_request.raw_token_exposed);
}

#[test]
fn kernel_drain_pipeline_delivery_plan_allows_bounded_fallback_only_when_safe() {
    let fallback = plan_hepta_kernel_telegram_drain_pipeline_delivery(
        HeptaKernelTelegramDrainPipelineDeliveryInput {
            model_output_present: false,
            model_failure_fallback_enabled: true,
            model_execution_session_runner_invoked: true,
            model_execution_status: "attention",
            reply_target_available: true,
            candidate_next_update_offset: Some(43),
            send_gate_env: "HEPTA_NATIVE_TELEGRAM_SEND",
            send_gate_enabled: true,
        },
    );
    assert!(fallback.model_failure_fallback_allowed);
    assert!(fallback.delivery_output_present);
    assert!(fallback.send_request.send_allowed);

    let unsafe_missing_reply = plan_hepta_kernel_telegram_drain_pipeline_delivery(
        HeptaKernelTelegramDrainPipelineDeliveryInput {
            model_output_present: false,
            model_failure_fallback_enabled: true,
            model_execution_session_runner_invoked: true,
            model_execution_status: "attention",
            reply_target_available: false,
            candidate_next_update_offset: Some(43),
            send_gate_env: "HEPTA_NATIVE_TELEGRAM_SEND",
            send_gate_enabled: true,
        },
    );
    assert!(!unsafe_missing_reply.model_failure_fallback_allowed);
    assert!(!unsafe_missing_reply.delivery_output_present);
    assert!(!unsafe_missing_reply.send_request.send_allowed);
    assert_eq!(
        HeptaKernelTelegramSendExecutionReport::from_send_request(
            &unsafe_missing_reply.send_request
        )
        .status,
        "waiting_model_output"
    );
}

#[test]
fn kernel_drain_final_status_prefers_delivery_then_model_then_previous() {
    let delivered = hepta_kernel_telegram_drain_final_status(
        true,
        true,
        "delivered",
        Some("ignored-send-error"),
        "attention",
        Some("ignored-model-error"),
        "planned",
        Some("ignored-previous-error"),
    );
    assert_eq!(delivered.status, "drained");
    assert_eq!(delivered.error, None);
    assert!(delivered.local_process_spawned);

    let send_attention = hepta_kernel_telegram_drain_final_status(
        false,
        true,
        "attention",
        Some("send failed"),
        "completed",
        None,
        "planned",
        None,
    );
    assert_eq!(send_attention.status, "attention");
    assert_eq!(send_attention.error.as_deref(), Some("send failed"));
    assert!(!send_attention.local_process_spawned);

    let model_attention = hepta_kernel_telegram_drain_final_status(
        true,
        false,
        "gated",
        None,
        "attention",
        Some("model failed"),
        "planned",
        None,
    );
    assert_eq!(model_attention.status, "attention");
    assert_eq!(model_attention.error.as_deref(), Some("model failed"));
    assert!(!model_attention.local_process_spawned);

    let previous = hepta_kernel_telegram_drain_final_status(
        false,
        false,
        "gated",
        None,
        "skipped",
        None,
        "planned",
        Some("previous error"),
    );
    assert_eq!(previous.status, "planned");
    assert_eq!(previous.error.as_deref(), Some("previous error"));
    assert!(!previous.local_process_spawned);
}

#[test]
fn kernel_drain_pipeline_finalizer_updates_process_and_error_precedence() {
    let invocation_request =
        HeptaKernelTelegramModelInvocationRequestPlan::disabled("MODEL_GATE", true);
    let mut model_execution =
        HeptaKernelTelegramModelExecutionReport::from_invocation_request(&invocation_request);
    model_execution.session_runner_invoked = true;
    let send_request = HeptaKernelTelegramSendRequestPlan::from_model_output(
        Some("private response text"),
        true,
        Some(43),
        "SEND_GATE",
        true,
    );
    let mut send_execution =
        HeptaKernelTelegramSendExecutionReport::from_send_request(&send_request);
    send_execution.status = "delivered";
    send_execution.error = Some("ignored stale send error".to_string());
    let delivered = finalize_hepta_kernel_telegram_drain_pipeline_status(
        HeptaKernelTelegramDrainPipelineOutcome {
            invocation_request,
            model_execution,
            send_request,
            send_execution,
        },
        true,
        "planned",
        Some("previous error".to_string()),
    );

    assert_eq!(delivered.status, "drained");
    assert_eq!(delivered.error, None);
    assert!(delivered.outcome.model_execution.local_process_spawned);

    let invocation_request =
        HeptaKernelTelegramModelInvocationRequestPlan::disabled("MODEL_GATE", true);
    let mut model_execution =
        HeptaKernelTelegramModelExecutionReport::from_invocation_request(&invocation_request);
    model_execution.status = "attention";
    model_execution.error = Some("model failed".to_string());
    let send_request = HeptaKernelTelegramSendRequestPlan::from_model_output(
        None,
        true,
        Some(43),
        "SEND_GATE",
        true,
    );
    let mut send_execution =
        HeptaKernelTelegramSendExecutionReport::from_send_request(&send_request);
    send_execution.status = "attention";
    send_execution.error = Some("send failed".to_string());
    let attention = finalize_hepta_kernel_telegram_drain_pipeline_status(
        HeptaKernelTelegramDrainPipelineOutcome {
            invocation_request,
            model_execution,
            send_request,
            send_execution,
        },
        false,
        "planned",
        None,
    );

    assert_eq!(attention.status, "attention");
    assert_eq!(attention.error.as_deref(), Some("send failed"));
    assert!(!attention.outcome.model_execution.local_process_spawned);
}

#[test]
fn kernel_send_request_and_execution_report_preserve_delivery_gates() {
    let disabled =
        HeptaKernelTelegramSendRequestPlan::disabled("HEPTA_NATIVE_TELEGRAM_SEND", false);
    assert!(!disabled.request_builder_ready);
    assert!(!disabled.send_allowed);
    assert_eq!(
        HeptaKernelTelegramSendExecutionReport::from_send_request(&disabled).status,
        "disabled"
    );

    let gated = HeptaKernelTelegramSendRequestPlan::from_model_output(
        Some("private model response text"),
        true,
        Some(43),
        "HEPTA_NATIVE_TELEGRAM_SEND",
        false,
    );
    assert!(gated.request_builder_ready);
    assert!(gated.model_output_present);
    assert!(gated.reply_target_available);
    assert_eq!(gated.candidate_next_update_offset, Some(43));
    assert!(!gated.request_body_materialized_by_status);
    assert!(!gated.delivery_performed_by_status);
    assert!(!gated.cursor_commit_allowed_after_delivery);
    assert!(!gated.raw_response_text_exposed);
    assert!(!gated.raw_chat_id_exposed);
    assert!(!gated.raw_message_id_exposed);
    assert!(!gated.raw_token_exposed);
    assert!(!gated.send_allowed);
    assert!(
        !serde_json::to_string(&gated)
            .expect("serialize")
            .contains("private model response text")
    );
    assert_eq!(
        HeptaKernelTelegramSendExecutionReport::from_send_request(&gated).status,
        "gated"
    );

    let ready = HeptaKernelTelegramSendRequestPlan::from_model_output(
        Some(" hello "),
        true,
        Some(43),
        "HEPTA_NATIVE_TELEGRAM_SEND",
        true,
    );
    assert!(ready.send_allowed);
    assert!(ready.cursor_commit_allowed_after_delivery);
    let report = HeptaKernelTelegramSendExecutionReport::from_send_request(&ready);
    assert_eq!(report.status, "ready");
    assert!(report.execution_ready);
    assert!(!report.external_send);
    assert!(!report.cursor_written);

    let without_reply_target = HeptaKernelTelegramSendRequestPlan::from_model_output(
        Some("private model response text"),
        false,
        Some(43),
        "HEPTA_NATIVE_TELEGRAM_SEND",
        true,
    );
    assert!(without_reply_target.model_output_present);
    assert!(without_reply_target.send_gate_enabled);
    assert!(!without_reply_target.reply_target_available);
    assert!(!without_reply_target.send_allowed);
    assert!(!without_reply_target.cursor_commit_allowed_after_delivery);

    let without_offset = HeptaKernelTelegramSendRequestPlan::from_model_output(
        Some("private model response text"),
        true,
        None,
        "HEPTA_NATIVE_TELEGRAM_SEND",
        true,
    );
    assert!(without_offset.model_output_present);
    assert!(without_offset.reply_target_available);
    assert!(!without_offset.send_allowed);
    assert!(!without_offset.cursor_commit_allowed_after_delivery);
}

#[test]
fn kernel_send_execution_preflight_reports_readiness_without_side_effects() {
    let ready = plan_hepta_kernel_telegram_send_execution_preflight(
        HeptaKernelTelegramSendExecutionPreflightInput {
            model_output_present: true,
            reply_target_available: true,
            candidate_next_update_offset: Some(43),
            token_shape_ok: true,
            send_gate_env: "HEPTA_NATIVE_TELEGRAM_SEND",
            send_gate_enabled: true,
        },
    );

    assert!(ready.execution_can_attempt_send);
    assert_eq!(ready.report.status, "ready");
    assert!(ready.request.send_allowed);
    assert!(!ready.report.send_attempted);
    assert!(!ready.report.delivery_ledger_write_attempted);
    assert!(!ready.report.cursor_commit_attempted);
    assert!(!ready.report.external_network_write);
    assert!(!ready.report.external_send);
    assert!(!ready.report.raw_response_text_exposed);
    assert!(!ready.report.raw_chat_id_exposed);
    assert!(!ready.report.raw_message_id_exposed);
    assert!(!ready.report.raw_token_exposed);
}

#[test]
fn kernel_send_execution_preflight_blocks_missing_token_shape() {
    let blocked = plan_hepta_kernel_telegram_send_execution_preflight(
        HeptaKernelTelegramSendExecutionPreflightInput {
            model_output_present: true,
            reply_target_available: true,
            candidate_next_update_offset: Some(43),
            token_shape_ok: false,
            send_gate_env: "HEPTA_NATIVE_TELEGRAM_SEND",
            send_gate_enabled: true,
        },
    );

    assert!(!blocked.execution_can_attempt_send);
    assert_eq!(blocked.report.status, "attention");
    assert_eq!(
        blocked.report.error.as_deref(),
        Some("Telegram send execution requires a valid Bot API token")
    );
    assert!(!blocked.report.send_attempted);
    assert!(!blocked.report.delivery_ledger_write_attempted);
    assert!(!blocked.report.cursor_written);
    assert!(!blocked.report.external_send);
    assert!(!blocked.report.raw_token_exposed);
}

#[test]
fn kernel_send_execution_report_transitions_preserve_redaction_boundary() {
    let request = HeptaKernelTelegramSendRequestPlan::from_model_output(
        Some("private model response text"),
        true,
        Some(43),
        "HEPTA_NATIVE_TELEGRAM_SEND",
        true,
    );

    let report = HeptaKernelTelegramSendExecutionReport::from_send_request(&request)
        .with_delivery_ledger_write_attempted()
        .with_delivery_ledger_written("enqueued")
        .with_sending_attempt_started()
        .with_bot_api_ack(Some(true))
        .with_external_send(true)
        .with_delivery_ledger_written("acked")
        .with_cursor_commit_attempted()
        .with_cursor_written();

    assert_eq!(report.status, "delivered");
    assert!(report.delivery_ledger_write_attempted);
    assert_eq!(report.delivery_ledger_written_count, 2);
    assert_eq!(
        report.latest_delivery_ledger_stage.as_deref(),
        Some("acked")
    );
    assert!(report.send_attempted);
    assert_eq!(report.bot_api_ack, Some(true));
    assert!(report.external_network_write);
    assert!(report.external_send);
    assert!(report.cursor_commit_attempted);
    assert!(report.cursor_written);
    assert!(!report.raw_response_text_exposed);
    assert!(!report.raw_token_exposed);

    let attention = report
        .clone()
        .with_redacted_attention_error("failed 123456789:abcdefghijklmnopqrstuvwxyz");
    assert_eq!(attention.status, "attention");
    assert_eq!(
        attention.error.as_deref(),
        Some("failed [redacted-telegram-token]")
    );
    assert!(!attention.raw_token_exposed);
}

#[test]
fn kernel_duplicate_policy_treats_cursor_as_next_update_offset() {
    assert!(hepta_kernel_telegram_update_already_drained(41, Some(42)));
    assert!(!hepta_kernel_telegram_update_already_drained(42, Some(42)));
    assert!(hepta_kernel_telegram_cursor_duplicate_rule_valid());
    assert_eq!(hepta_kernel_telegram_next_update_offset(42), Some(43));
    assert_eq!(hepta_kernel_telegram_next_update_offset(i64::MAX), None);

    let duplicate = hepta_kernel_telegram_duplicate_decision(41, Some(42));
    assert_eq!(duplicate.decision, "skip_already_drained");
    assert!(duplicate.already_drained);
    assert!(!duplicate.should_invoke_model);
    assert!(duplicate.should_record_duplicate);
    assert!(!duplicate.cursor_write_allowed_after_delivery);
    assert_eq!(duplicate.candidate_next_update_offset, Some(42));
    assert!(!duplicate.raw_update_payload_exposed);

    let candidate = hepta_kernel_telegram_duplicate_decision(42, Some(42));
    assert_eq!(candidate.decision, "model_candidate");
    assert!(!candidate.already_drained);
    assert!(candidate.should_invoke_model);
    assert!(!candidate.should_record_duplicate);
    assert!(candidate.cursor_write_allowed_after_delivery);
    assert_eq!(candidate.candidate_next_update_offset, Some(43));
    assert!(!candidate.raw_update_payload_exposed);
}

#[test]
fn kernel_telegram_config_parser_helpers_are_trimmed_and_bounded() {
    assert_eq!(
        hepta_kernel_telegram_normalize_binding_id(" telegram:6476198178 "),
        "6476198178"
    );
    assert_eq!(
        hepta_kernel_telegram_normalize_binding_id("tg:6476198178"),
        "6476198178"
    );
    assert_eq!(
        hepta_kernel_telegram_normalize_binding_id("6476198178"),
        "6476198178"
    );
    assert_eq!(hepta_kernel_telegram_normalize_binding_id(" tg: "), "");

    assert!(hepta_kernel_telegram_env_truthy_value(" YES "));
    assert!(hepta_kernel_telegram_env_truthy_value("true"));
    assert!(hepta_kernel_telegram_env_truthy_value("1"));
    assert!(hepta_kernel_telegram_env_truthy_value("on"));
    assert!(!hepta_kernel_telegram_env_truthy_value("off"));
    assert!(!hepta_kernel_telegram_env_truthy_value("0"));

    assert_eq!(hepta_kernel_telegram_env_u64_value(" 42 "), Some(42));
    assert_eq!(hepta_kernel_telegram_env_u64_value("not-a-number"), None);
    assert_eq!(hepta_kernel_telegram_env_u64_value("-1"), None);
}

#[test]
fn kernel_telegram_config_status_derives_binding_without_exposing_tokens() {
    let status = build_hepta_kernel_telegram_config_status(HeptaKernelTelegramConfigStatusInput {
        config_path: Some("private/config/openclaw.json".to_string()),
        config_found: true,
        enabled: true,
        dm_policy: " Trusted ".to_string(),
        group_policy: "Deny".to_string(),
        allow_from_count: 1,
        group_count: 0,
        token_source: "secret_file",
        token_secret_ref_present: true,
        token_secret_provider: Some("telegram_bot".to_string()),
        token_secret_id_present: true,
        token_file_present: true,
        token_file_mode_0600: true,
        token_file_security_ready: true,
        token_shape_ok: true,
        error: None,
    });

    assert!(status.binding_ready);
    assert!(status.config_ready());
    assert_eq!(status.dm_policy, "trusted");
    assert_eq!(status.group_policy, "deny");
    assert!(!status.raw_token_exposed);

    let missing = HeptaKernelTelegramConfigStatus::missing("missing config".to_string());
    assert_eq!(missing.token_source, "missing");
    assert_eq!(missing.error.as_deref(), Some("missing config"));
    assert!(!missing.config_ready());
}

#[test]
fn kernel_telegram_config_status_requires_binding_scope() {
    let status = build_hepta_kernel_telegram_config_status(HeptaKernelTelegramConfigStatusInput {
        config_path: Some("private/config/openclaw.json".to_string()),
        config_found: true,
        enabled: true,
        dm_policy: "deny".to_string(),
        group_policy: "deny".to_string(),
        allow_from_count: 0,
        group_count: 0,
        token_source: "env",
        token_secret_ref_present: false,
        token_secret_provider: None,
        token_secret_id_present: false,
        token_file_present: false,
        token_file_mode_0600: false,
        token_file_security_ready: false,
        token_shape_ok: true,
        error: None,
    });

    assert!(!status.binding_ready);
    assert!(!status.config_ready());
}

#[test]
fn kernel_telegram_config_status_requires_secure_file_admission() {
    let insecure =
        build_hepta_kernel_telegram_config_status(HeptaKernelTelegramConfigStatusInput {
            config_path: Some("private/config/openclaw.json".to_string()),
            config_found: true,
            enabled: true,
            dm_policy: "trusted".to_string(),
            group_policy: "deny".to_string(),
            allow_from_count: 1,
            group_count: 0,
            token_source: "secret_file",
            token_secret_ref_present: true,
            token_secret_provider: Some("telegram_bot".to_string()),
            token_secret_id_present: true,
            token_file_present: true,
            token_file_mode_0600: true,
            token_file_security_ready: false,
            token_shape_ok: true,
            error: Some("unsafe secret file".to_string()),
        });
    assert!(insecure.binding_ready);
    assert!(!insecure.config_ready());
    assert!(
        !hepta_kernel_telegram_transport_plan_for_config_status(&insecure)
            .bot_api_transport_plan_ready
    );

    let mut secure = insecure;
    secure.token_file_security_ready = true;
    secure.error = None;
    assert!(secure.config_ready());
    assert!(
        hepta_kernel_telegram_transport_plan_for_config_status(&secure)
            .bot_api_transport_plan_ready
    );
}

#[test]
fn kernel_telegram_config_status_rejects_inline_token_without_legacy_override() {
    let mut status =
        build_hepta_kernel_telegram_config_status(HeptaKernelTelegramConfigStatusInput {
            config_path: Some("private/config/openclaw.json".to_string()),
            config_found: true,
            enabled: true,
            dm_policy: "trusted".to_string(),
            group_policy: "deny".to_string(),
            allow_from_count: 1,
            group_count: 0,
            token_source: "inline_config_rejected",
            token_secret_ref_present: false,
            token_secret_provider: None,
            token_secret_id_present: false,
            token_file_present: false,
            token_file_mode_0600: false,
            token_file_security_ready: false,
            token_shape_ok: true,
            error: Some("inline token rejected".to_string()),
        });
    assert!(status.binding_ready);
    assert!(!status.config_ready());

    status.token_source = "inline_config_legacy_override";
    status.error = None;
    assert!(status.config_ready());
}

#[test]
fn kernel_telegram_token_observation_prefers_safe_sources() {
    let env = hepta_kernel_telegram_token_observation(HeptaKernelTelegramTokenObservationInput {
        env_token_present: true,
        env_token_shape_ok: true,
        file_token_present: true,
        file_token_shape_ok: true,
        inline_token_present: true,
        inline_token_shape_ok: true,
        token_secret_ref_present: true,
    });
    assert_eq!(env.token_source, "env");
    assert!(env.token_shape_ok);

    let file = hepta_kernel_telegram_token_observation(HeptaKernelTelegramTokenObservationInput {
        env_token_present: false,
        env_token_shape_ok: false,
        file_token_present: true,
        file_token_shape_ok: false,
        inline_token_present: true,
        inline_token_shape_ok: true,
        token_secret_ref_present: true,
    });
    assert_eq!(file.token_source, "secret_file");
    assert!(!file.token_shape_ok);

    let secret_missing =
        hepta_kernel_telegram_token_observation(HeptaKernelTelegramTokenObservationInput {
            env_token_present: false,
            env_token_shape_ok: false,
            file_token_present: false,
            file_token_shape_ok: false,
            inline_token_present: false,
            inline_token_shape_ok: false,
            token_secret_ref_present: true,
        });
    assert_eq!(secret_missing.token_source, "secret_file_missing");
    assert!(!secret_missing.token_shape_ok);
}

#[test]
fn kernel_telegram_config_metadata_extracts_non_secret_fields() {
    let config = json!({
        "secrets": {
            "providers": {
                "telegram_bot": {
                    "path": "../secrets/telegram-token"
                }
            }
        },
        "channels": {
            "telegram": {
                "enabled": true,
                "dmPolicy": " Trusted ",
                "groupPolicy": "Mention",
                "allowFrom": ["telegram:6476198178", " tg:42 ", ""],
                "groups": {
                    "ops": { "id": "-1001" },
                    "dev": { "id": "-1002" }
                },
                "botToken": {
                    "source": "file",
                    "provider": "telegram_bot",
                    "id": " bot-token "
                }
            }
        }
    });

    let metadata = extract_hepta_kernel_telegram_config_metadata(
        Path::new("/tmp/hepta/private/config/openclaw.json"),
        &config,
    )
    .expect("metadata");

    assert!(metadata.enabled);
    assert_eq!(metadata.dm_policy, "trusted");
    assert_eq!(metadata.group_policy, "mention");
    assert_eq!(metadata.allow_from_count, 2);
    assert_eq!(metadata.group_count, 2);
    assert!(metadata.token_secret_ref_present);
    assert_eq!(
        metadata.token_secret_provider.as_deref(),
        Some("telegram_bot")
    );
    assert!(metadata.token_secret_id_present);
    assert_eq!(
        metadata.token_secret_path,
        Some(PathBuf::from(
            "/tmp/hepta/private/config/../secrets/telegram-token"
        ))
    );
    assert!(!metadata.inline_token_present);
}

#[test]
fn kernel_telegram_config_metadata_does_not_resolve_provider_without_file_source() {
    let config = json!({
        "secrets": {
            "providers": {
                "telegram_bot": { "path": "../secrets/telegram-token" }
            }
        },
        "channels": {
            "telegram": {
                "enabled": true,
                "botToken": {
                    "source": "unsupported",
                    "provider": "telegram_bot",
                    "id": "bot-token"
                }
            }
        }
    });

    let metadata = extract_hepta_kernel_telegram_config_metadata(
        Path::new("/tmp/hepta/private/config/openclaw.json"),
        &config,
    )
    .expect("metadata");
    assert!(!metadata.token_secret_ref_present);
    assert!(metadata.token_secret_path.is_none());
}

#[test]
fn kernel_telegram_secret_provider_path_resolves_against_config_parent() {
    let config = json!({
        "secrets": {
            "providers": {
                "telegram_bot": {
                    "path": "../secrets/telegram-token"
                },
                "absolute": {
                    "path": "/private/tmp/telegram-token"
                }
            }
        }
    });

    assert_eq!(
        resolve_hepta_kernel_telegram_secret_provider_path(
            Path::new("/tmp/hepta/private/config/openclaw.json"),
            &config,
            "telegram_bot",
        ),
        Some(PathBuf::from(
            "/tmp/hepta/private/config/../secrets/telegram-token"
        ))
    );
    assert_eq!(
        resolve_hepta_kernel_telegram_secret_provider_path(
            Path::new("/tmp/hepta/private/config/openclaw.json"),
            &config,
            "absolute",
        ),
        Some(PathBuf::from("/private/tmp/telegram-token"))
    );
    assert!(
        resolve_hepta_kernel_telegram_secret_provider_path(
            Path::new("/tmp/hepta/private/config/openclaw.json"),
            &config,
            "missing",
        )
        .is_none()
    );
}

#[test]
fn kernel_model_invocation_request_preserves_prompt_privacy_and_gates() {
    let candidate = HeptaKernelTelegramCandidateMaterial {
        update_id: Some(42),
        kind: "message:text".to_string(),
        prompt_text: Some("private prompt text".to_string()),
        has_reply_target: true,
        reply_target: Some(HeptaKernelTelegramReplyTargetMaterial {
            chat_id: 123,
            reply_to_message_id: Some(456),
            raw_identifiers_exposed: false,
        }),
        requires_model: true,
        raw_identifiers_exposed: false,
    };
    let decision = hepta_kernel_telegram_duplicate_decision(42, Some(42));
    let request = HeptaKernelTelegramModelInvocationRequestPlan::from_candidate(
        candidate,
        decision,
        "HEPTA_NATIVE_TELEGRAM_MODEL_TURN",
        true,
    );

    assert!(request.request_builder_ready);
    assert!(request.candidate_present);
    assert_eq!(request.candidate_kind.as_deref(), Some("message:text"));
    assert_eq!(request.duplicate_decision, "model_candidate");
    assert!(request.prompt_material_in_memory);
    assert!(!request.prompt_material_serialized);
    assert!(request.reply_target_available);
    assert!(request.stable_session_key_ready);
    assert!(request.should_invoke_model);
    assert!(!request.should_record_duplicate);
    assert_eq!(request.candidate_next_update_offset, Some(43));
    assert!(request.runner_invocation_allowed);
    assert!(!request.session_runner_invoked);
    assert!(!request.local_process_spawned);
    assert!(!request.external_send);
    assert!(!request.cursor_written);
    assert!(!request.raw_prompt_text_exposed);
    assert!(!request.raw_chat_id_exposed);
    assert!(!request.raw_sender_id_exposed);
    assert!(!request.raw_message_id_exposed);
    assert!(
        !serde_json::to_string(&request)
            .expect("serialize")
            .contains("private prompt text")
    );
}

#[test]
fn kernel_model_execution_report_maps_request_statuses() {
    let disabled = HeptaKernelTelegramModelInvocationRequestPlan::disabled("MODEL_GATE", false);
    assert_eq!(
        HeptaKernelTelegramModelExecutionReport::from_invocation_request(&disabled).status,
        "disabled"
    );

    let empty_gated = HeptaKernelTelegramModelInvocationRequestPlan::empty("MODEL_GATE", false);
    assert_eq!(
        HeptaKernelTelegramModelExecutionReport::from_invocation_request(&empty_gated).status,
        "gated"
    );

    let waiting_candidate =
        HeptaKernelTelegramModelInvocationRequestPlan::empty("MODEL_GATE", true);
    assert_eq!(
        HeptaKernelTelegramModelExecutionReport::from_invocation_request(&waiting_candidate).status,
        "waiting_candidate"
    );

    let duplicate_candidate = HeptaKernelTelegramCandidateMaterial {
        update_id: Some(41),
        kind: "message:text".to_string(),
        prompt_text: Some("private prompt text".to_string()),
        has_reply_target: true,
        reply_target: Some(HeptaKernelTelegramReplyTargetMaterial {
            chat_id: 123,
            reply_to_message_id: Some(456),
            raw_identifiers_exposed: false,
        }),
        requires_model: true,
        raw_identifiers_exposed: false,
    };
    let duplicate_request = HeptaKernelTelegramModelInvocationRequestPlan::from_candidate(
        duplicate_candidate,
        hepta_kernel_telegram_duplicate_decision(41, Some(42)),
        "MODEL_GATE",
        true,
    );
    assert_eq!(
        HeptaKernelTelegramModelExecutionReport::from_invocation_request(&duplicate_request).status,
        "duplicate_suppressed"
    );

    let waiting_prompt_candidate = HeptaKernelTelegramCandidateMaterial {
        update_id: Some(44),
        kind: "message_reaction:redacted".to_string(),
        prompt_text: None,
        has_reply_target: false,
        reply_target: None,
        requires_model: true,
        raw_identifiers_exposed: false,
    };
    let waiting_prompt_request = HeptaKernelTelegramModelInvocationRequestPlan::from_candidate(
        waiting_prompt_candidate,
        hepta_kernel_telegram_duplicate_decision(44, Some(44)),
        "MODEL_GATE",
        true,
    );
    assert_eq!(
        HeptaKernelTelegramModelExecutionReport::from_invocation_request(&waiting_prompt_request)
            .status,
        "waiting_prompt"
    );

    let ready_candidate = HeptaKernelTelegramCandidateMaterial {
        update_id: Some(45),
        kind: "message:text".to_string(),
        prompt_text: Some("private prompt text".to_string()),
        has_reply_target: true,
        reply_target: Some(HeptaKernelTelegramReplyTargetMaterial {
            chat_id: 123,
            reply_to_message_id: Some(456),
            raw_identifiers_exposed: false,
        }),
        requires_model: true,
        raw_identifiers_exposed: false,
    };
    let ready_request = HeptaKernelTelegramModelInvocationRequestPlan::from_candidate(
        ready_candidate,
        hepta_kernel_telegram_duplicate_decision(45, Some(45)),
        "MODEL_GATE",
        true,
    );
    let ready_report =
        HeptaKernelTelegramModelExecutionReport::from_invocation_request(&ready_request);
    assert_eq!(ready_report.status, "ready");
    assert!(ready_report.execution_ready);
    assert!(ready_report.runner_invocation_allowed);
    assert!(!ready_report.session_runner_invoked);
    assert!(!ready_report.external_send);
    assert!(!ready_report.cursor_written);
    assert!(!ready_report.raw_response_text_exposed);

    let missing_id_candidate = HeptaKernelTelegramCandidateMaterial {
        update_id: None,
        kind: "message:text".to_string(),
        prompt_text: Some("private prompt text".to_string()),
        has_reply_target: true,
        reply_target: Some(HeptaKernelTelegramReplyTargetMaterial {
            chat_id: 123,
            reply_to_message_id: Some(456),
            raw_identifiers_exposed: false,
        }),
        requires_model: true,
        raw_identifiers_exposed: false,
    };
    let missing_id_request = HeptaKernelTelegramModelInvocationRequestPlan::attention(
        missing_id_candidate,
        "missing_update_id",
        None,
        "MODEL_GATE",
        true,
    );
    let missing_id_outcome = build_hepta_kernel_telegram_model_execution_outcome_without_runner(
        missing_id_request,
        None,
    );
    assert_eq!(missing_id_outcome.report.status, "attention");
    assert_eq!(
        missing_id_outcome.report.error.as_deref(),
        Some("Telegram model execution requires an update id for cursor safety")
    );
}

#[test]
fn kernel_model_execution_runs_runner_without_serializing_private_material() {
    let candidate = HeptaKernelTelegramCandidateMaterial {
        update_id: Some(48),
        kind: "message:text".to_string(),
        prompt_text: Some("private model prompt".to_string()),
        has_reply_target: true,
        reply_target: Some(HeptaKernelTelegramReplyTargetMaterial {
            chat_id: 6476198178,
            reply_to_message_id: Some(13),
            raw_identifiers_exposed: false,
        }),
        requires_model: true,
        raw_identifiers_exposed: false,
    };
    let decision = hepta_kernel_telegram_duplicate_decision(48, Some(48));

    let outcome = execute_hepta_kernel_telegram_model_turn_after_candidate(
        HeptaKernelTelegramModelExecutionInput {
            candidate: Some(candidate),
            duplicate_decision: Some(decision),
            model_turn_gate_env: "MODEL_GATE",
            model_turn_gate_enabled: true,
        },
        |prompt| {
            assert_eq!(prompt, "private model prompt");
            Ok(" private model response text ".to_string())
        },
    );

    assert_eq!(outcome.report.status, "completed");
    assert!(outcome.report.execution_ready);
    assert!(outcome.report.runner_invocation_allowed);
    assert!(outcome.report.session_runner_invoked);
    assert!(outcome.report.model_output_present);
    assert_eq!(outcome.candidate_next_update_offset, Some(49));
    assert_eq!(
        outcome.model_output.as_deref(),
        Some("private model response text")
    );
    assert!(outcome.reply_target.is_some());

    let serialized = serde_json::to_string(&outcome.report).expect("serialize report");
    assert!(!serialized.contains("private model prompt"));
    assert!(!serialized.contains("private model response text"));
    assert!(!serialized.contains("6476198178"));
}

#[test]
fn kernel_model_execution_respects_gate_before_runner() {
    let candidate = HeptaKernelTelegramCandidateMaterial {
        update_id: Some(48),
        kind: "message:text".to_string(),
        prompt_text: Some("private model prompt".to_string()),
        has_reply_target: true,
        reply_target: Some(HeptaKernelTelegramReplyTargetMaterial {
            chat_id: 6476198178,
            reply_to_message_id: Some(13),
            raw_identifiers_exposed: false,
        }),
        requires_model: true,
        raw_identifiers_exposed: false,
    };
    let decision = hepta_kernel_telegram_duplicate_decision(48, Some(48));

    let outcome = execute_hepta_kernel_telegram_model_turn_after_candidate(
        HeptaKernelTelegramModelExecutionInput {
            candidate: Some(candidate),
            duplicate_decision: Some(decision),
            model_turn_gate_env: "MODEL_GATE",
            model_turn_gate_enabled: false,
        },
        |_| panic!("model runner must not run while gated"),
    );

    assert_eq!(outcome.report.status, "gated");
    assert!(!outcome.report.runner_invocation_allowed);
    assert!(!outcome.report.session_runner_invoked);
    assert_eq!(outcome.model_output, None);
    assert!(outcome.report.error.unwrap().contains("MODEL_GATE"));
}

#[test]
fn kernel_model_execution_suppresses_duplicate_before_runner() {
    let candidate = HeptaKernelTelegramCandidateMaterial {
        update_id: Some(48),
        kind: "message:text".to_string(),
        prompt_text: Some("private duplicate prompt".to_string()),
        has_reply_target: true,
        reply_target: Some(HeptaKernelTelegramReplyTargetMaterial {
            chat_id: 6476198178,
            reply_to_message_id: Some(13),
            raw_identifiers_exposed: false,
        }),
        requires_model: true,
        raw_identifiers_exposed: false,
    };
    let decision = hepta_kernel_telegram_duplicate_decision(48, Some(49));

    let outcome = execute_hepta_kernel_telegram_model_turn_after_candidate(
        HeptaKernelTelegramModelExecutionInput {
            candidate: Some(candidate),
            duplicate_decision: Some(decision),
            model_turn_gate_env: "MODEL_GATE",
            model_turn_gate_enabled: true,
        },
        |_| panic!("duplicate candidate must not invoke model runner"),
    );

    assert_eq!(outcome.report.status, "duplicate_suppressed");
    assert!(!outcome.report.runner_invocation_allowed);
    assert!(!outcome.report.session_runner_invoked);
    assert_eq!(outcome.model_output, None);
    assert_eq!(outcome.candidate_next_update_offset, Some(49));
}
