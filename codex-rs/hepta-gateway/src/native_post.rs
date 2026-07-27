use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub use hepta_runtime::DEFAULT_NATIVE_POST_EXECUTION_STORE_DIR;
pub use hepta_runtime::DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS;
pub use hepta_runtime::DEFAULT_NATIVE_POST_STORE_MAX_BYTES;
pub use hepta_runtime::DEFAULT_NATIVE_POST_STORE_MAX_LINES;
pub use hepta_runtime::NATIVE_POST_ACTIVATION_PLAN_ENDPOINT;
pub use hepta_runtime::NATIVE_POST_COMPATIBILITY_HARNESS_PLAN_KINDS;
pub use hepta_runtime::NATIVE_POST_EXECUTION_READINESS_ENDPOINT;
pub use hepta_runtime::NATIVE_POST_EXECUTION_STORE_DIR_ENV;
pub use hepta_runtime::NATIVE_POST_EXECUTION_STORES_ENDPOINT;
pub use hepta_runtime::NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT;
pub use hepta_runtime::NATIVE_POST_MAX_BODY_BYTES;
pub use hepta_runtime::NATIVE_POST_RATE_LIMIT_WINDOW_MS_ENV;
pub use hepta_runtime::NATIVE_POST_REAL_HANDLER_APPROVAL_ENV;
pub use hepta_runtime::NATIVE_POST_REAL_HANDLER_PLAN_KINDS;
pub use hepta_runtime::NATIVE_POST_REAL_HANDLER_SCOPE_ENV;
pub use hepta_runtime::NATIVE_POST_REAL_HANDLERS_ENV;
pub use hepta_runtime::NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT;
pub use hepta_runtime::NATIVE_POST_STORE_MAX_BYTES_ENV;
pub use hepta_runtime::NATIVE_POST_STORE_MAX_LINES_ENV;
pub use hepta_runtime::NativePostActivationGate;
pub use hepta_runtime::NativePostActivationPlanResponse;
pub use hepta_runtime::NativePostAuditEventContract;
pub use hepta_runtime::NativePostBodyAdmission;
pub use hepta_runtime::NativePostBodySchema;
pub use hepta_runtime::NativePostConfirmationContract;
pub use hepta_runtime::NativePostExecutionAdmission;
pub use hepta_runtime::NativePostExecutionReadinessResponse;
pub use hepta_runtime::NativePostExecutionReadinessRoute;
pub use hepta_runtime::NativePostExecutionStoreFileObservation;
pub use hepta_runtime::NativePostExecutionStoreFileSpec;
pub use hepta_runtime::NativePostExecutionStoreFileStatus;
pub use hepta_runtime::NativePostExecutionStoreLimits;
pub use hepta_runtime::NativePostExecutionStoreRecord;
pub use hepta_runtime::NativePostExecutionStoreWriteReport;
pub use hepta_runtime::NativePostExecutionStoresResponse;
pub use hepta_runtime::NativePostGrayReleaseEvidenceResponse;
pub use hepta_runtime::NativePostIdempotencyEvidence;
pub use hepta_runtime::NativePostPlanResponse;
pub use hepta_runtime::NativePostPlanRouteSpec;
pub use hepta_runtime::NativePostRealHandlerHarness;
pub use hepta_runtime::NativePostRealHandlerObservation;
pub use hepta_runtime::NativePostRollbackContract;
pub use hepta_runtime::NativePostRolloutEvidenceFileObservation;
pub use hepta_runtime::NativePostRolloutEvidencePlanKindCount;
pub use hepta_runtime::NativePostRolloutEvidenceRecordSummary;
pub use hepta_runtime::NativePostRolloutEvidenceResponse;
pub use hepta_runtime::NativePostRolloutEvidenceScan;
pub use hepta_runtime::NativePostSelectedHandlerRolloutEvidence;
pub use hepta_runtime::NativePostStoreEffectProjection;
pub use hepta_runtime::NativePostStoreReadObservation;
pub use hepta_runtime::native_post_audit_event_contract;
pub use hepta_runtime::native_post_body_admission;
pub use hepta_runtime::native_post_body_schema;
pub use hepta_runtime::native_post_confirmation_contract;
pub use hepta_runtime::native_post_duplicate_check_required;
pub use hepta_runtime::native_post_execution_admission_with_scope;
pub use hepta_runtime::native_post_execution_readiness_report;
pub use hepta_runtime::native_post_execution_store_capacity_allows_append;
pub use hepta_runtime::native_post_execution_store_capacity_ok;
pub use hepta_runtime::native_post_execution_store_contracts_ready;
pub use hepta_runtime::native_post_execution_store_file_status_from_observation;
pub use hepta_runtime::native_post_execution_store_jsonl_valid;
pub use hepta_runtime::native_post_execution_store_record_json_line;
pub use hepta_runtime::native_post_execution_store_record_projected_append_bytes;
pub use hepta_runtime::native_post_execution_store_specs;
pub use hepta_runtime::native_post_execution_store_write_report;
pub use hepta_runtime::native_post_idempotency_duplicate_present_from_observation;
pub use hepta_runtime::native_post_idempotency_evidence;
pub use hepta_runtime::native_post_plan_kind_has_real_handler;
pub use hepta_runtime::native_post_plan_parameter;
pub use hepta_runtime::native_post_plan_route_specs;
pub use hepta_runtime::native_post_rate_limit_check_required;
pub use hepta_runtime::native_post_rate_limit_recent_present_from_observation;
pub use hepta_runtime::native_post_real_handler_harness_from_observation;
pub use hepta_runtime::native_post_real_handler_scope_matches;
pub use hepta_runtime::native_post_real_handler_scope_selected_kinds;
pub use hepta_runtime::native_post_real_handler_scope_single_selected_kind;
pub use hepta_runtime::native_post_redacted_fingerprint;
pub use hepta_runtime::native_post_rollback_contract;
pub use hepta_runtime::native_post_rollout_evidence_scan_from_observation;
pub use hepta_runtime::native_post_selected_handler_rollout_evidence_from_observation;
pub use hepta_runtime::native_post_store_capacity_check_required;
pub use hepta_runtime::native_post_store_effect_projection;
pub use hepta_runtime::native_post_store_write_attempt_required;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativePostPersistenceInvariantError {
    MissingPendingRecord,
}

impl NativePostPersistenceInvariantError {
    fn code(self) -> &'static str {
        match self {
            Self::MissingPendingRecord => "native_post_execution_store_pending_record_missing",
        }
    }
}

fn native_post_record_for_store_write(
    store_write_attempted: bool,
    pending_record: Option<&NativePostExecutionStoreRecord>,
) -> Result<Option<&NativePostExecutionStoreRecord>, NativePostPersistenceInvariantError> {
    match (store_write_attempted, pending_record) {
        (true, None) => Err(NativePostPersistenceInvariantError::MissingPendingRecord),
        (true, Some(record)) => Ok(Some(record)),
        (false, _) => Ok(None),
    }
}

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
    let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
    let audit_event_contract = native_post_audit_event_contract(
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
    let store_effect_projection = native_post_store_effect_projection(
        idempotency_evidence,
        audit_event_contract,
        &real_handler_harness,
    );
    hepta_runtime::native_post_plan_response(
        spec,
        parameter.is_some(),
        parameter.map(str::len),
        body_schema,
        body_admission,
        confirmation_contract,
        rollback_contract,
        store_effect_projection.idempotency_evidence,
        store_effect_projection.audit_event_contract,
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
    let store_contracts_ready = native_post_execution_store_contracts_ready(&stores);
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
    let store_jsonl_valid = native_post_execution_store_jsonl_valid(&store_files);
    let store_capacity_ok = native_post_execution_store_capacity_ok(&store_files);
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
    let store_jsonl_valid = native_post_execution_store_jsonl_valid(&store_files);
    let store_capacity_ok = native_post_execution_store_capacity_ok(&store_files);
    let selected_handler_kind = native_post_real_handler_scope_single_selected_kind(handler_scope);
    let rollback_path = root.join("rollback.jsonl");
    let rollout_observation = native_post_rollout_evidence_file_observation(&rollback_path);
    let rollout_evidence = hepta_runtime::native_post_rollout_evidence_report(
        root.display().to_string(),
        store_jsonl_valid,
        store_capacity_ok,
        handler_scope,
        native_post_rollout_evidence_scan_from_observation(rollout_observation.clone()),
    );
    let selected_handler_evidence = native_post_selected_handler_rollout_evidence_from_observation(
        selected_handler_kind,
        rollout_observation,
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
    let duplicate_check_performed =
        native_post_duplicate_check_required(execution_admission, idempotency_evidence);
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
    let rate_limit_check_performed = native_post_rate_limit_check_required(
        execution_admission,
        duplicate_check_performed,
        duplicate_found,
        duplicate_check_error,
    );
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
    let capacity_check_performed = native_post_store_capacity_check_required(
        execution_admission,
        duplicate_check_performed,
        duplicate_found,
        duplicate_check_error,
        rate_limited,
        rate_limit_check_error,
    );
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
    let store_write_attempted = native_post_store_write_attempt_required(
        capacity_check_performed,
        store_capacity_ok,
        store_capacity_check_error,
    );
    let (store_write_succeeded, store_write_report, store_write_error) =
        match native_post_record_for_store_write(store_write_attempted, pending_record.as_ref()) {
            Ok(Some(record)) => {
                match persist_native_post_execution_store_record(store_root, record) {
                    Ok(report) => (true, Some(report), None),
                    Err(_error) => (
                        false,
                        None,
                        Some("native_post_execution_store_write_failed"),
                    ),
                }
            }
            Ok(None) => (false, None, None),
            Err(error) => (false, None, Some(error.code())),
        };
    let observation = NativePostRealHandlerObservation {
        duplicate_check_performed,
        duplicate_found,
        duplicate_check_error,
        rate_limit_check_performed,
        rate_limited,
        rate_limit_window_ms: store_limits.rate_limit_window_ms,
        rate_limit_check_error,
        capacity_check_performed,
        store_capacity_ok,
        store_capacity_check_error,
        store_write_attempted,
        store_write_succeeded,
        store_write_report,
        store_write_error,
    };
    native_post_real_handler_harness_from_observation(spec, execution_admission, observation)
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
    let projected_line_bytes = native_post_execution_store_record_projected_append_bytes(record)?;
    let stores = native_post_execution_store_file_statuses(root, max_store_bytes, max_store_lines);
    Ok(native_post_execution_store_capacity_allows_append(
        &stores,
        projected_line_bytes,
        max_store_bytes,
        max_store_lines,
    ))
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
    let line = native_post_execution_store_record_json_line(record)?;
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
    Ok(native_post_execution_store_write_report(
        root.display().to_string(),
        written_files,
    ))
}

fn native_post_idempotency_duplicate_present(
    root: &Path,
    key_fingerprint: Option<&str>,
) -> Result<bool, &'static str> {
    let path = root.join("idempotency.jsonl");
    native_post_idempotency_duplicate_present_from_observation(
        native_post_store_read_observation(&path),
        key_fingerprint,
    )
}

fn native_post_rate_limit_recent_present(
    root: &Path,
    bucket: &str,
    window_ms: u64,
) -> Result<bool, &'static str> {
    let path = root.join("rate-limit.jsonl");
    let now_ms = native_post_now_unix_ms();
    native_post_rate_limit_recent_present_from_observation(
        native_post_store_read_observation(&path),
        bucket,
        window_ms,
        now_ms,
    )
}

fn native_post_store_read_observation(path: &Path) -> NativePostStoreReadObservation {
    match fs::read_to_string(path) {
        Ok(content) => NativePostStoreReadObservation {
            content: Some(content),
            missing: false,
            read_failed: false,
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            NativePostStoreReadObservation {
                content: None,
                missing: true,
                read_failed: false,
            }
        }
        Err(_) => NativePostStoreReadObservation {
            content: None,
            missing: false,
            read_failed: true,
        },
    }
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
    let bytes = metadata.as_ref().map(std::fs::Metadata::len).unwrap_or(0);
    let jsonl_observation = if exists {
        native_post_store_read_observation(&path)
    } else {
        NativePostStoreReadObservation {
            content: None,
            missing: true,
            read_failed: false,
        }
    };
    native_post_execution_store_file_status_from_observation(
        spec,
        NativePostExecutionStoreFileObservation {
            path: path.display().to_string(),
            exists,
            bytes,
            max_bytes: max_store_bytes,
            max_lines: max_store_lines,
            jsonl_observation,
        },
    )
}

fn native_post_rollout_evidence_scan(path: &Path) -> NativePostRolloutEvidenceScan {
    native_post_rollout_evidence_scan_from_observation(
        native_post_rollout_evidence_file_observation(path),
    )
}

fn native_post_rollout_evidence_file_observation(
    path: &Path,
) -> NativePostRolloutEvidenceFileObservation {
    match fs::read_to_string(path) {
        Ok(content) => NativePostRolloutEvidenceFileObservation {
            content: Some(content),
            missing: false,
            read_failed: false,
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            NativePostRolloutEvidenceFileObservation {
                content: None,
                missing: true,
                read_failed: false,
            }
        }
        Err(_) => NativePostRolloutEvidenceFileObservation {
            content: None,
            missing: false,
            read_failed: true,
        },
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
    use super::NativePostPersistenceInvariantError;
    use super::native_post_plan_kind_has_real_handler;
    use super::native_post_plan_parameter;
    use super::native_post_plan_route_specs;
    use super::native_post_record_for_store_write;

    #[test]
    fn native_post_route_contracts_cover_real_handler_candidates() {
        let specs = native_post_plan_route_specs();

        for plan_kind in ["approval_apply", "task_publish", "chat_send"] {
            assert!(specs.iter().any(|spec| spec.plan_kind == plan_kind));
            assert!(!native_post_plan_kind_has_real_handler(plan_kind));
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
        assert!(admission.ready_for_real_handler_input);
        assert!(admission.idempotency_key_present);
        assert!(!admission.raw_body_exposed);
        assert!(!admission.raw_field_values_exposed);
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
        assert!(!idempotency.key_shape_valid);
        assert!(!audit.ready_for_real_handler);
        assert!(!audit.raw_body_exposed);
        assert!(!audit.raw_idempotency_key_exposed);
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
        assert_eq!(mismatched.blocked_reason, "real_handler_not_wired");
        assert!(!mismatched.current_plan_executes_real_handler);
        assert!(!super::native_post_duplicate_check_required(
            &mismatched,
            &idempotency
        ));

        let matched = super::native_post_execution_admission_with_scope(
            spec,
            &admission,
            &idempotency,
            &audit,
            true,
            true,
            Some("task_publish, chat_send"),
        );
        assert_eq!(matched.admission_status, "blocked");
        assert!(!matched.current_plan_executes_real_handler);
        assert_eq!(matched.blocked_reason, "real_handler_not_wired");
        assert!(!super::native_post_duplicate_check_required(
            &matched,
            &idempotency
        ));
        assert!(!super::native_post_rate_limit_check_required(
            &matched, true, false, None
        ));
        assert!(!super::native_post_rate_limit_check_required(
            &matched, true, true, None
        ));
        assert!(!super::native_post_store_capacity_check_required(
            &matched, true, false, None, false, None
        ));
        assert!(!super::native_post_store_capacity_check_required(
            &matched, true, false, None, true, None
        ));
        assert!(super::native_post_store_write_attempt_required(
            true, true, None
        ));
        assert!(!super::native_post_store_write_attempt_required(
            true,
            true,
            Some("native_post_store_capacity_check_failed")
        ));
    }

    #[test]
    fn native_post_store_write_fails_closed_without_pending_record() {
        assert!(matches!(
            native_post_record_for_store_write(true, None),
            Err(NativePostPersistenceInvariantError::MissingPendingRecord)
        ));
        assert!(matches!(
            native_post_record_for_store_write(false, None),
            Ok(None)
        ));
    }

    #[test]
    fn native_post_real_handler_scope_selection_uses_gateway_registry() {
        let selected =
            super::native_post_real_handler_scope_selected_kinds(Some("approval_apply chat_send"));

        assert!(selected.is_empty());
        assert_eq!(
            super::native_post_real_handler_scope_single_selected_kind(Some("task_publish")),
            None
        );
        assert_eq!(
            super::native_post_real_handler_scope_single_selected_kind(Some(
                "approval_apply chat_send"
            )),
            None
        );
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
        assert_eq!(report.real_handler_implemented_count, 0);
        assert_eq!(report.real_handler_ready_count, 0);
        assert_eq!(report.selected_handler_count, 0);
        assert!(report.all_real_handlers_blocked);
        assert!(!report.real_handler_gate_enabled);
        assert!(report.routes.iter().any(|route| {
            route.plan_kind == "task_publish"
                && !route.ready_for_real_handler_wiring
                && route.blocked_reason == "real_handler_not_wired"
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

        assert_eq!(harness.status, "not_implemented");
        assert!(!harness.store_write_attempted);
        assert!(!harness.store_write_succeeded);
        assert!(!harness.task_published);
        assert!(!harness.external_side_effects);
        for filename in [
            "idempotency.jsonl",
            "audit.jsonl",
            "rollback.jsonl",
            "rate-limit.jsonl",
        ] {
            assert!(!temp.path().join(filename).exists());
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

        assert_eq!(first.status, "not_implemented");
        assert_eq!(second.status, "not_implemented");
        assert!(!second.duplicate_suppressed);
        assert!(!second.store_write_attempted);
        assert!(!temp.path().join("idempotency.jsonl").exists());
    }
}
