use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub use hepta_runtime::{
    DEFAULT_NATIVE_POST_EXECUTION_STORE_DIR, DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS,
    DEFAULT_NATIVE_POST_STORE_MAX_BYTES, DEFAULT_NATIVE_POST_STORE_MAX_LINES,
    NATIVE_POST_ACTIVATION_PLAN_ENDPOINT, NATIVE_POST_EXECUTION_READINESS_ENDPOINT,
    NATIVE_POST_EXECUTION_STORE_DIR_ENV, NATIVE_POST_EXECUTION_STORES_ENDPOINT,
    NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT, NATIVE_POST_MAX_BODY_BYTES,
    NATIVE_POST_RATE_LIMIT_WINDOW_MS_ENV, NATIVE_POST_REAL_HANDLER_APPROVAL_ENV,
    NATIVE_POST_REAL_HANDLER_PLAN_KINDS, NATIVE_POST_REAL_HANDLER_SCOPE_ENV,
    NATIVE_POST_REAL_HANDLERS_ENV, NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT,
    NATIVE_POST_STORE_MAX_BYTES_ENV, NATIVE_POST_STORE_MAX_LINES_ENV, NativePostActivationGate,
    NativePostActivationPlanResponse, NativePostAuditEventContract, NativePostBodyAdmission,
    NativePostBodySchema, NativePostConfirmationContract, NativePostExecutionAdmission,
    NativePostExecutionReadinessResponse, NativePostExecutionReadinessRoute,
    NativePostExecutionStoreFileSpec, NativePostExecutionStoreFileStatus,
    NativePostExecutionStoreLimits, NativePostExecutionStoreRecord,
    NativePostExecutionStoreWriteReport, NativePostExecutionStoresResponse,
    NativePostGrayReleaseEvidenceResponse, NativePostIdempotencyEvidence, NativePostPlanResponse,
    NativePostPlanRouteSpec, NativePostRealHandlerHarness, NativePostRollbackContract,
    NativePostRolloutEvidencePlanKindCount, NativePostRolloutEvidenceRecordSummary,
    NativePostRolloutEvidenceResponse, NativePostRolloutEvidenceScan,
    NativePostSelectedHandlerRolloutEvidence, native_post_audit_event_contract,
    native_post_body_admission, native_post_body_schema, native_post_confirmation_contract,
    native_post_execution_admission_with_scope, native_post_execution_readiness_report,
    native_post_execution_store_specs, native_post_idempotency_evidence,
    native_post_plan_kind_has_real_handler, native_post_plan_parameter,
    native_post_plan_route_specs, native_post_real_handler_scope_matches,
    native_post_real_handler_scope_selected_kinds, native_post_redacted_fingerprint,
    native_post_rollback_contract,
};

pub fn native_post_plan_report(
    spec: &NativePostPlanRouteSpec,
    parameter: Option<&str>,
    request_body: Option<&str>,
    real_handler_gate_enabled: bool,
    operator_approval_enabled: bool,
    handler_scope: Option<&str>,
    store_root: &Path,
    store_limits: NativePostExecutionStoreLimits,
) -> NativePostPlanResponse {
    let body_schema = native_post_body_schema(spec.plan_kind, request_body.is_some());
    let body_admission = native_post_body_admission(spec, &body_schema, request_body);
    let confirmation_contract = native_post_confirmation_contract(spec);
    let rollback_contract = native_post_rollback_contract();
    let mut idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
    let mut audit_event_contract = native_post_audit_event_contract(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
    );
    let execution_admission = native_post_execution_admission_with_scope(
        spec,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        real_handler_gate_enabled,
        operator_approval_enabled,
        handler_scope,
    );
    let real_handler_harness = native_post_real_handler_harness(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        &execution_admission,
        store_root,
        store_limits,
    );
    if real_handler_harness.duplicate_check_performed {
        idempotency_evidence.current_plan_lookup_performed = true;
    }
    if real_handler_harness.store_write_succeeded {
        idempotency_evidence.current_plan_store_written = true;
        audit_event_contract.current_plan_emits_audit_event = true;
        audit_event_contract.current_plan_persists_audit_event = true;
    }
    hepta_runtime::native_post_plan_response(
        spec,
        parameter.is_some(),
        parameter.map(str::len),
        body_schema,
        body_admission,
        confirmation_contract,
        rollback_contract,
        idempotency_evidence,
        audit_event_contract,
        execution_admission,
        real_handler_harness,
    )
}

pub fn native_post_dispatch_plan_report(
    method: &str,
    path: &str,
    request_body: Option<&str>,
    real_handler_gate_enabled: bool,
    operator_approval_enabled: bool,
    handler_scope: Option<&str>,
    store_root: &Path,
    store_limits: NativePostExecutionStoreLimits,
) -> Option<NativePostPlanResponse> {
    if method != "POST" {
        return None;
    }

    native_post_plan_route_specs().iter().find_map(|spec| {
        native_post_plan_parameter(spec, path).map(|parameter| {
            native_post_plan_report(
                spec,
                parameter,
                request_body,
                real_handler_gate_enabled,
                operator_approval_enabled,
                handler_scope,
                store_root,
                store_limits,
            )
        })
    })
}

pub fn native_post_execution_stores_report(
    root: &Path,
    max_store_bytes: u64,
    max_store_lines: u64,
) -> NativePostExecutionStoresResponse {
    let store_files =
        native_post_execution_store_file_statuses(root, max_store_bytes, max_store_lines);
    let root_exists = root.exists();
    let root_is_dir = root.is_dir();
    hepta_runtime::native_post_execution_stores_report(
        root.display().to_string(),
        root_exists,
        root_is_dir,
        max_store_bytes,
        max_store_lines,
        store_files,
    )
}

pub fn native_post_activation_plan_report(
    root: &Path,
    max_store_bytes: u64,
    max_store_lines: u64,
    real_handler_gate_enabled: bool,
    operator_approval_enabled: bool,
    handler_scope: Option<&str>,
) -> NativePostActivationPlanResponse {
    let stores = native_post_execution_stores_report(root, max_store_bytes, max_store_lines);
    let store_contracts_ready = stores.persistence_implementation_ready
        && stores.idempotency_store_ready
        && stores.audit_store_ready
        && stores.rollback_store_ready
        && stores.rate_limit_store_ready
        && stores.store_jsonl_valid
        && stores.store_capacity_ok;
    hepta_runtime::native_post_activation_plan_report(
        real_handler_gate_enabled,
        operator_approval_enabled,
        handler_scope,
        store_contracts_ready,
        stores.store_jsonl_valid,
        stores.store_capacity_ok,
        stores.rollback_store_ready,
    )
}

pub fn native_post_rollout_evidence_report(
    root: &Path,
    max_store_bytes: u64,
    max_store_lines: u64,
    handler_scope: Option<&str>,
) -> NativePostRolloutEvidenceResponse {
    let store_files =
        native_post_execution_store_file_statuses(root, max_store_bytes, max_store_lines);
    let store_jsonl_valid = store_files
        .iter()
        .all(|file| file.jsonl_readable && file.invalid_json_line_count == 0);
    let store_capacity_ok = store_files
        .iter()
        .all(|file| file.bytes_within_limit && file.line_count_within_limit);
    let rollback_path = root.join("rollback.jsonl");
    let scan = native_post_rollout_evidence_scan(&rollback_path);
    hepta_runtime::native_post_rollout_evidence_report(
        root.display().to_string(),
        store_jsonl_valid,
        store_capacity_ok,
        handler_scope,
        scan,
    )
}

pub fn native_post_gray_release_evidence_report(
    root: &Path,
    max_store_bytes: u64,
    max_store_lines: u64,
    handler_scope: Option<&str>,
    real_handler_gate_enabled: bool,
    operator_approval_enabled: bool,
) -> NativePostGrayReleaseEvidenceResponse {
    let store_files =
        native_post_execution_store_file_statuses(root, max_store_bytes, max_store_lines);
    let store_jsonl_valid = store_files
        .iter()
        .all(|file| file.jsonl_readable && file.invalid_json_line_count == 0);
    let store_capacity_ok = store_files
        .iter()
        .all(|file| file.bytes_within_limit && file.line_count_within_limit);
    let selected_handler_kinds = native_post_real_handler_scope_selected_kinds(handler_scope);
    let selected_handler_kind =
        (selected_handler_kinds.len() == 1).then(|| selected_handler_kinds[0]);
    let rollout_evidence =
        native_post_rollout_evidence_report(root, max_store_bytes, max_store_lines, handler_scope);
    let selected_handler_evidence = native_post_selected_handler_rollout_evidence(
        &root.join("rollback.jsonl"),
        selected_handler_kind,
    );
    hepta_runtime::native_post_gray_release_evidence_report(
        root.display().to_string(),
        handler_scope,
        real_handler_gate_enabled,
        operator_approval_enabled,
        store_jsonl_valid,
        store_capacity_ok,
        rollout_evidence.rollout_evidence_ready,
        rollout_evidence.raw_request_body_exposed,
        rollout_evidence.raw_field_values_exposed,
        rollout_evidence.raw_idempotency_key_exposed,
        rollout_evidence.raw_audit_payload_exposed,
        selected_handler_evidence,
    )
}

pub fn native_post_real_handler_harness(
    spec: &NativePostPlanRouteSpec,
    body_schema: &NativePostBodySchema,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    execution_admission: &NativePostExecutionAdmission,
    store_root: &Path,
    store_limits: NativePostExecutionStoreLimits,
) -> NativePostRealHandlerHarness {
    let duplicate_check_performed = execution_admission.current_plan_executes_real_handler
        && idempotency_evidence.key_fingerprint.is_some();
    let (duplicate_found, duplicate_check_error) = if duplicate_check_performed {
        match native_post_idempotency_duplicate_present(
            store_root,
            idempotency_evidence.key_fingerprint.as_deref(),
        ) {
            Ok(found) => (found, None),
            Err(_error) => (false, Some("native_post_idempotency_check_failed")),
        }
    } else {
        (false, None)
    };
    let duplicate_suppressed = duplicate_check_performed && duplicate_found;
    let rate_limit_check_performed = execution_admission.current_plan_executes_real_handler
        && !duplicate_suppressed
        && duplicate_check_error.is_none();
    let (rate_limited, rate_limit_check_error) = if rate_limit_check_performed {
        match native_post_rate_limit_recent_present(
            store_root,
            spec.plan_kind,
            store_limits.rate_limit_window_ms,
        ) {
            Ok(limited) => (limited, None),
            Err(_error) => (false, Some("native_post_rate_limit_check_failed")),
        }
    } else {
        (false, None)
    };
    let capacity_check_performed = execution_admission.current_plan_executes_real_handler
        && !duplicate_suppressed
        && duplicate_check_error.is_none()
        && !rate_limited
        && rate_limit_check_error.is_none();
    let pending_record = if capacity_check_performed {
        Some(native_post_execution_store_record(
            spec,
            body_schema,
            body_admission,
            idempotency_evidence,
            audit_event_contract,
            true,
        ))
    } else {
        None
    };
    let (store_capacity_ok, store_capacity_check_error) = if let Some(record) = &pending_record {
        match native_post_execution_store_capacity_allows_append_with_limits(
            store_root,
            record,
            store_limits.max_store_bytes,
            store_limits.max_store_lines,
        ) {
            Ok(ok) => (ok, None),
            Err(_error) => (false, Some("native_post_store_capacity_check_failed")),
        }
    } else {
        (true, None)
    };
    let store_write_attempted =
        capacity_check_performed && store_capacity_ok && store_capacity_check_error.is_none();
    let (store_write_succeeded, store_write_report, store_write_error) = if store_write_attempted {
        match persist_native_post_execution_store_record(
            store_root,
            pending_record
                .as_ref()
                .expect("pending record exists before store write"),
        ) {
            Ok(report) => (true, Some(report), None),
            Err(_error) => (
                false,
                None,
                Some("native_post_execution_store_write_failed"),
            ),
        }
    } else {
        (false, None, None)
    };
    hepta_runtime::native_post_real_handler_harness(
        spec,
        execution_admission,
        duplicate_check_performed,
        duplicate_found,
        duplicate_check_error,
        rate_limit_check_performed,
        rate_limited,
        store_limits.rate_limit_window_ms,
        rate_limit_check_error,
        capacity_check_performed,
        store_capacity_ok,
        store_capacity_check_error,
        store_write_attempted,
        store_write_succeeded,
        store_write_report,
        store_write_error,
    )
}

pub fn native_post_execution_store_record(
    spec: &NativePostPlanRouteSpec,
    body_schema: &NativePostBodySchema,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    current_plan_executes_real_handler: bool,
) -> NativePostExecutionStoreRecord {
    hepta_runtime::native_post_execution_store_record(
        spec,
        body_schema,
        body_admission,
        idempotency_evidence,
        audit_event_contract,
        current_plan_executes_real_handler,
        native_post_now_unix_ms(),
    )
}

pub fn native_post_execution_store_capacity_allows_append_with_limits(
    root: &Path,
    record: &NativePostExecutionStoreRecord,
    max_store_bytes: u64,
    max_store_lines: u64,
) -> Result<bool, String> {
    let line = serde_json::to_string(record)
        .map_err(|error| format!("failed to serialize native POST execution record: {error}"))?;
    let projected_line_bytes = line.len() as u64 + 1;
    for spec in native_post_execution_store_specs() {
        let status =
            native_post_execution_store_file_status(root, spec, max_store_bytes, max_store_lines);
        if !status.jsonl_readable || !status.jsonl_valid {
            return Ok(false);
        }
        if status.bytes.saturating_add(projected_line_bytes) > max_store_bytes {
            return Ok(false);
        }
        if status.line_count.saturating_add(1) > max_store_lines {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn persist_native_post_execution_store_record(
    root: &Path,
    record: &NativePostExecutionStoreRecord,
) -> Result<NativePostExecutionStoreWriteReport, String> {
    fs::create_dir_all(root).map_err(|error| {
        format!(
            "failed to create native POST execution store root {}: {error}",
            root.display()
        )
    })?;
    let line = serde_json::to_string(record)
        .map_err(|error| format!("failed to serialize native POST execution record: {error}"))?;
    let mut written_files = Vec::new();
    for spec in native_post_execution_store_specs() {
        let path = root.join(spec.filename);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|error| {
                format!(
                    "failed to open native POST execution store {}: {error}",
                    path.display()
                )
            })?;
        writeln!(file, "{line}").map_err(|error| {
            format!(
                "failed to append native POST execution store {}: {error}",
                path.display()
            )
        })?;
        written_files.push(path.display().to_string());
    }
    Ok(NativePostExecutionStoreWriteReport {
        status: "written",
        root: root.display().to_string(),
        written_file_count: written_files.len(),
        written_files,
        raw_request_body_exposed: false,
        raw_field_values_exposed: false,
        raw_idempotency_key_exposed: false,
        raw_audit_payload_exposed: false,
    })
}

fn native_post_idempotency_duplicate_present(
    root: &Path,
    key_fingerprint: Option<&str>,
) -> Result<bool, String> {
    let Some(key_fingerprint) = key_fingerprint else {
        return Ok(false);
    };
    let path = root.join("idempotency.jsonl");
    match fs::read_to_string(&path) {
        Ok(content) => Ok(content.lines().any(|line| line.contains(key_fingerprint))),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(format!(
            "failed to read native POST idempotency store {}: {error}",
            path.display()
        )),
    }
}

fn native_post_rate_limit_recent_present(
    root: &Path,
    bucket: &str,
    window_ms: u64,
) -> Result<bool, String> {
    let path = root.join("rate-limit.jsonl");
    let content = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => {
            return Err(format!(
                "failed to read native POST rate-limit store {}: {error}",
                path.display()
            ));
        }
    };
    let now_ms = native_post_now_unix_ms();
    for line in content.lines() {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        let Some(record_bucket) = value
            .get("rate_limit_bucket")
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        if record_bucket != bucket {
            continue;
        }
        let Some(recorded_at_ms) = value
            .get("recorded_at_unix_ms")
            .and_then(serde_json::Value::as_u64)
        else {
            continue;
        };
        if now_ms.saturating_sub(recorded_at_ms) <= window_ms {
            return Ok(true);
        }
    }
    Ok(false)
}

fn native_post_execution_store_file_statuses(
    root: &Path,
    max_store_bytes: u64,
    max_store_lines: u64,
) -> Vec<NativePostExecutionStoreFileStatus> {
    native_post_execution_store_specs()
        .iter()
        .map(|spec| {
            native_post_execution_store_file_status(root, spec, max_store_bytes, max_store_lines)
        })
        .collect()
}

fn native_post_execution_store_file_status(
    root: &Path,
    spec: &NativePostExecutionStoreFileSpec,
    max_store_bytes: u64,
    max_store_lines: u64,
) -> NativePostExecutionStoreFileStatus {
    let path = root.join(spec.filename);
    let metadata = path.metadata().ok();
    let exists = metadata.as_ref().is_some_and(std::fs::Metadata::is_file);
    let (jsonl_readable, line_count, valid_json_line_count, invalid_json_line_count) =
        native_post_execution_store_jsonl_health(&path, exists);
    let bytes = metadata.as_ref().map(std::fs::Metadata::len).unwrap_or(0);
    NativePostExecutionStoreFileStatus {
        store_kind: spec.store_kind,
        schema_id: spec.schema_id,
        filename: spec.filename,
        path: path.display().to_string(),
        exists,
        bytes,
        max_bytes: max_store_bytes,
        bytes_within_limit: bytes <= max_store_bytes,
        append_only: true,
        jsonl: true,
        jsonl_readable,
        jsonl_valid: jsonl_readable && invalid_json_line_count == 0,
        line_count,
        max_lines: max_store_lines,
        line_count_within_limit: line_count <= max_store_lines,
        valid_json_line_count,
        invalid_json_line_count,
        raw_body_exposed: false,
        raw_field_values_exposed: false,
        raw_idempotency_key_exposed: false,
    }
}

fn native_post_execution_store_jsonl_health(path: &Path, exists: bool) -> (bool, u64, u64, u64) {
    if !exists {
        return (true, 0, 0, 0);
    }
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return (false, 0, 0, 0),
    };
    let mut line_count = 0_u64;
    let mut valid_json_line_count = 0_u64;
    let mut invalid_json_line_count = 0_u64;
    for line in content.lines() {
        line_count = line_count.saturating_add(1);
        if serde_json::from_str::<serde_json::Value>(line).is_ok() {
            valid_json_line_count = valid_json_line_count.saturating_add(1);
        } else {
            invalid_json_line_count = invalid_json_line_count.saturating_add(1);
        }
    }
    (
        true,
        line_count,
        valid_json_line_count,
        invalid_json_line_count,
    )
}

fn native_post_rollout_evidence_scan(path: &Path) -> NativePostRolloutEvidenceScan {
    match fs::read_to_string(path) {
        Ok(content) => hepta_runtime::native_post_rollout_evidence_scan_from_content(&content),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            hepta_runtime::native_post_rollout_evidence_scan_missing()
        }
        Err(_) => hepta_runtime::native_post_rollout_evidence_scan_read_failed(),
    }
}

fn native_post_selected_handler_rollout_evidence(
    path: &Path,
    selected_handler_kind: Option<&str>,
) -> NativePostSelectedHandlerRolloutEvidence {
    match fs::read_to_string(path) {
        Ok(content) => hepta_runtime::native_post_selected_handler_rollout_evidence_from_content(
            selected_handler_kind,
            &content,
        ),
        Err(_) => hepta_runtime::native_post_selected_handler_rollout_evidence_missing(
            selected_handler_kind,
        ),
    }
}

fn native_post_now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::{
        native_post_plan_kind_has_real_handler, native_post_plan_parameter,
        native_post_plan_route_specs,
    };

    #[test]
    fn native_post_route_contracts_cover_real_handler_candidates() {
        let specs = native_post_plan_route_specs();

        for plan_kind in ["approval_apply", "task_publish", "chat_send"] {
            assert!(specs.iter().any(|spec| spec.plan_kind == plan_kind));
            assert!(native_post_plan_kind_has_real_handler(plan_kind));
        }
    }

    #[test]
    fn native_post_route_parameter_matches_exact_and_prefix_routes() {
        let specs = native_post_plan_route_specs();
        let action = specs
            .iter()
            .find(|spec| spec.pattern == "/api/actions/<action>")
            .expect("action spec should exist");
        assert_eq!(
            native_post_plan_parameter(action, "/api/actions/reload"),
            Some(Some("reload"))
        );

        let task_publish = specs
            .iter()
            .find(|spec| spec.pattern == "/api/tasks/publish")
            .expect("task publish spec should exist");
        assert_eq!(
            native_post_plan_parameter(task_publish, "/api/tasks/publish"),
            Some(None)
        );
    }

    #[test]
    fn native_post_body_admission_validates_redacted_real_handler_input() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let schema = super::native_post_body_schema(spec.plan_kind, true);
        let admission = super::native_post_body_admission(
            spec,
            &schema,
            Some(
                r#"{"task":"secret task","confirm":true,"dry_run":true,"idempotency_key":"secret-key"}"#,
            ),
        );

        assert_eq!(schema.schema_id, "hepta.post.task_publish.v1");
        assert_eq!(admission.admission_status, "ready_for_real_handler");
        assert_eq!(admission.ready_for_real_handler_input, true);
        assert_eq!(admission.idempotency_key_present, true);
        assert_eq!(admission.raw_body_exposed, false);
        assert_eq!(admission.raw_field_values_exposed, false);
        let fingerprint = admission
            .idempotency_key_fingerprint
            .as_deref()
            .expect("fingerprint");
        assert!(fingerprint.starts_with("sha256:"));
        assert!(!fingerprint.contains("secret-key"));
    }

    #[test]
    fn native_post_audit_contract_waits_for_valid_real_handler_input() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "chat_send")
            .expect("chat send spec");
        let schema = super::native_post_body_schema(spec.plan_kind, true);
        let admission =
            super::native_post_body_admission(spec, &schema, Some(r#"{"chat_id":"c1"}"#));
        let idempotency = super::native_post_idempotency_evidence(spec, &admission);
        let audit =
            super::native_post_audit_event_contract(spec, &schema, &admission, &idempotency);

        assert_eq!(admission.admission_status, "missing_required_fields");
        assert_eq!(idempotency.key_shape_valid, false);
        assert_eq!(audit.ready_for_real_handler, false);
        assert_eq!(audit.raw_body_exposed, false);
        assert_eq!(audit.raw_idempotency_key_exposed, false);
    }

    #[test]
    fn native_post_execution_admission_requires_matching_scope() {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "chat_send")
            .expect("chat send spec");
        let schema = super::native_post_body_schema(spec.plan_kind, true);
        let admission = super::native_post_body_admission(
            spec,
            &schema,
            Some(
                r#"{"chat_id":"c1","message":"secret","confirm":true,"dry_run":true,"idempotency_key":"key"}"#,
            ),
        );
        let idempotency = super::native_post_idempotency_evidence(spec, &admission);
        let audit =
            super::native_post_audit_event_contract(spec, &schema, &admission, &idempotency);

        let mismatched = super::native_post_execution_admission_with_scope(
            spec,
            &admission,
            &idempotency,
            &audit,
            true,
            true,
            Some("task_publish"),
        );
        assert_eq!(mismatched.admission_status, "blocked");
        assert_eq!(mismatched.blocked_reason, "handler_scope_not_selected");
        assert_eq!(mismatched.current_plan_executes_real_handler, false);

        let matched = super::native_post_execution_admission_with_scope(
            spec,
            &admission,
            &idempotency,
            &audit,
            true,
            true,
            Some("task_publish, chat_send"),
        );
        assert_eq!(matched.admission_status, "harness_ready");
        assert_eq!(matched.current_plan_executes_real_handler, true);
        assert_eq!(matched.blocked_reason, "real_handler_harness_dry_run_only");
    }

    #[test]
    fn native_post_real_handler_scope_selection_uses_gateway_registry() {
        let selected =
            super::native_post_real_handler_scope_selected_kinds(Some("approval_apply chat_send"));

        assert_eq!(selected, vec!["approval_apply", "chat_send"]);
        assert!(super::native_post_real_handler_scope_matches(
            "chat_send",
            Some("task_publish,chat_send")
        ));
        assert!(!super::native_post_real_handler_scope_matches(
            "approval_apply",
            Some("task_publish,chat_send")
        ));
    }

    #[test]
    fn native_post_execution_readiness_report_uses_kernel_contract() {
        let report =
            super::native_post_execution_readiness_report(false, Some("task_publish chat_send"));

        assert_eq!(report.status, "ready");
        assert_eq!(report.post_route_count, 12);
        assert_eq!(report.real_handler_candidate_count, 3);
        assert_eq!(report.real_handler_implemented_count, 3);
        assert_eq!(report.selected_handler_count, 2);
        assert_eq!(report.all_real_handlers_blocked, true);
        assert_eq!(report.real_handler_gate_enabled, false);
        assert!(report.routes.iter().any(|route| {
            route.plan_kind == "task_publish"
                && route.ready_for_real_handler_wiring
                && route.blocked_reason == "real_handler_gate_disabled"
        }));
    }

    #[test]
    fn native_post_real_handler_harness_persists_redacted_evidence_in_gateway() {
        let temp = tempfile::tempdir().expect("tempdir");
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let schema = super::native_post_body_schema(spec.plan_kind, true);
        let admission = super::native_post_body_admission(
            spec,
            &schema,
            Some(
                r#"{"task":"secret gateway task","confirm":true,"dry_run":true,"idempotency_key":"secret-gateway-idem"}"#,
            ),
        );
        let idempotency = super::native_post_idempotency_evidence(spec, &admission);
        let audit =
            super::native_post_audit_event_contract(spec, &schema, &admission, &idempotency);
        let execution = super::native_post_execution_admission_with_scope(
            spec,
            &admission,
            &idempotency,
            &audit,
            true,
            true,
            Some("task_publish"),
        );

        let harness = super::native_post_real_handler_harness(
            spec,
            &schema,
            &admission,
            &idempotency,
            &audit,
            &execution,
            temp.path(),
            super::NativePostExecutionStoreLimits {
                max_store_bytes: super::DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
                max_store_lines: super::DEFAULT_NATIVE_POST_STORE_MAX_LINES,
                rate_limit_window_ms: super::DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS,
            },
        );

        assert_eq!(harness.status, "dry_run_recorded");
        assert_eq!(harness.store_write_attempted, true);
        assert_eq!(harness.store_write_succeeded, true);
        assert_eq!(harness.task_published, false);
        assert_eq!(harness.external_side_effects, false);
        for filename in [
            "idempotency.jsonl",
            "audit.jsonl",
            "rollback.jsonl",
            "rate-limit.jsonl",
        ] {
            let content =
                std::fs::read_to_string(temp.path().join(filename)).expect("store content");
            assert!(content.contains("hepta.post.execution_store_record.v1"));
            assert!(content.contains("\"current_plan_executes_real_handler\":true"));
            assert!(!content.contains("secret gateway task"));
            assert!(!content.contains("secret-gateway-idem"));
        }
    }

    #[test]
    fn native_post_real_handler_harness_suppresses_duplicate_in_gateway() {
        let temp = tempfile::tempdir().expect("tempdir");
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == "task_publish")
            .expect("task publish spec");
        let schema = super::native_post_body_schema(spec.plan_kind, true);
        let admission = super::native_post_body_admission(
            spec,
            &schema,
            Some(
                r#"{"task":"secret duplicate gateway task","confirm":true,"dry_run":true,"idempotency_key":"secret-gateway-duplicate"}"#,
            ),
        );
        let idempotency = super::native_post_idempotency_evidence(spec, &admission);
        let audit =
            super::native_post_audit_event_contract(spec, &schema, &admission, &idempotency);
        let execution = super::native_post_execution_admission_with_scope(
            spec,
            &admission,
            &idempotency,
            &audit,
            true,
            true,
            Some("task_publish"),
        );
        let limits = super::NativePostExecutionStoreLimits {
            max_store_bytes: super::DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
            max_store_lines: super::DEFAULT_NATIVE_POST_STORE_MAX_LINES,
            rate_limit_window_ms: super::DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS,
        };

        let first = super::native_post_real_handler_harness(
            spec,
            &schema,
            &admission,
            &idempotency,
            &audit,
            &execution,
            temp.path(),
            limits,
        );
        let second = super::native_post_real_handler_harness(
            spec,
            &schema,
            &admission,
            &idempotency,
            &audit,
            &execution,
            temp.path(),
            limits,
        );

        assert_eq!(first.status, "dry_run_recorded");
        assert_eq!(second.status, "duplicate_suppressed");
        assert_eq!(second.store_write_attempted, false);
        let idempotency_content =
            std::fs::read_to_string(temp.path().join("idempotency.jsonl")).expect("store");
        assert_eq!(idempotency_content.lines().count(), 1);
        assert!(!idempotency_content.contains("secret duplicate gateway task"));
        assert!(!idempotency_content.contains("secret-gateway-duplicate"));
    }
}
