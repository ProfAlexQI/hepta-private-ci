use std::collections::BTreeMap;
use std::collections::HashSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::mpsc;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::TrySendError;
use std::thread;
use std::time::Instant;
use std::time::UNIX_EPOCH;

use anyhow::Context;
use anyhow::Result;
use hepta_core::MemoryQuery;
use hepta_core::MemoryRecord;
use hepta_core::MemoryScope;
use hepta_gateway::DEFAULT_NATIVE_POST_EXECUTION_STORE_DIR;
use hepta_gateway::DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS;
use hepta_gateway::DEFAULT_NATIVE_POST_STORE_MAX_BYTES;
use hepta_gateway::DEFAULT_NATIVE_POST_STORE_MAX_LINES;
use hepta_gateway::HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT;
use hepta_gateway::HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND;
use hepta_gateway::HEPTA_CORE_FUSION_READINESS_ENDPOINT;
#[cfg(test)]
use hepta_gateway::HEPTA_CORE_FUSION_READINESS_SOURCE_COMMAND;
use hepta_gateway::HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT;
use hepta_gateway::HEPTA_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND;
#[cfg(test)]
use hepta_gateway::HEPTA_ENGINE_DEPENDENCY_CLOSURE_ENDPOINT;
#[cfg(test)]
use hepta_gateway::HEPTA_ENGINE_DEPENDENCY_CLOSURE_SOURCE_COMMAND;
use hepta_gateway::HEPTA_NAME_REPOSITORY_CLOSURE_ENDPOINT;
#[cfg(test)]
use hepta_gateway::HEPTA_NAME_REPOSITORY_CLOSURE_SOURCE_COMMAND;
use hepta_gateway::NATIVE_POST_ACTIVATION_PLAN_ENDPOINT;
use hepta_gateway::NATIVE_POST_EXECUTION_READINESS_ENDPOINT;
use hepta_gateway::NATIVE_POST_EXECUTION_STORE_DIR_ENV;
use hepta_gateway::NATIVE_POST_EXECUTION_STORES_ENDPOINT;
use hepta_gateway::NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT;
use hepta_gateway::NATIVE_POST_RATE_LIMIT_WINDOW_MS_ENV;
use hepta_gateway::NATIVE_POST_REAL_HANDLER_APPROVAL_ENV;
use hepta_gateway::NATIVE_POST_REAL_HANDLER_SCOPE_ENV;
use hepta_gateway::NATIVE_POST_REAL_HANDLERS_ENV;
#[cfg(test)]
use hepta_gateway::NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT;
use hepta_gateway::NATIVE_POST_STORE_MAX_BYTES_ENV;
use hepta_gateway::NATIVE_POST_STORE_MAX_LINES_ENV;
use hepta_gateway::NativePostActivationPlanResponse;
#[cfg(test)]
use hepta_gateway::NativePostAuditEventContract;
#[cfg(test)]
use hepta_gateway::NativePostBodyAdmission;
#[cfg(test)]
use hepta_gateway::NativePostBodySchema;
#[cfg(test)]
use hepta_gateway::NativePostExecutionAdmission;
use hepta_gateway::NativePostExecutionReadinessResponse;
use hepta_gateway::NativePostExecutionStoreLimits;
#[cfg(test)]
use hepta_gateway::NativePostExecutionStoreRecord;
#[cfg(test)]
use hepta_gateway::NativePostExecutionStoreWriteReport;
use hepta_gateway::NativePostExecutionStoresResponse;
use hepta_gateway::NativePostGrayReleaseEvidenceResponse;
#[cfg(test)]
use hepta_gateway::NativePostIdempotencyEvidence;
#[cfg(test)]
use hepta_gateway::NativePostPlanResponse;
#[cfg(test)]
use hepta_gateway::NativePostPlanRouteSpec;
#[cfg(test)]
use hepta_gateway::NativePostRealHandlerHarness;
use hepta_gateway::NativePostRolloutEvidenceResponse;
use hepta_memory::InMemoryStore;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

#[cfg(test)]
use crate::gate_command::gate_command_json;
use crate::gate_spec::GateSpec as ControlUiRouteSpec;
#[cfg(test)]
use crate::gateway_options::DEFAULT_BIND_ADDR;
#[cfg(test)]
use crate::gateway_options::DEFAULT_TELEGRAM_POLL_MS;
use crate::gateway_options::NativeGatewayOptions;
#[cfg(test)]
use crate::gateway_options::parse_serve_ui_args;
use crate::http_transport::*;
use crate::native_telegram;
use crate::native_telegram::NativeTelegramPluginStatus;
use crate::operator_mutation::enabled as operator_mutation_enabled;
use crate::provider_domain::ProviderChannelDryRunPlanResponse;
use crate::provider_domain::ProviderReportContext;
use crate::route_manifest::RouteDispatchHandler;
use crate::route_manifest::route_manifest_entry;
use crate::route_registry::*;
use crate::runtime_composition::NativeGatewayRuntime;
use crate::runtime_composition::RuntimeRequestPreflightReceipt;
use crate::runtime_composition::runtime_kernel_canary_body_admitted;
use crate::runtime_ingress::operator_execution_response;
#[cfg(test)]
use crate::runtime_ingress::runtime_ingress_lifecycle;
use crate::runtime_ingress::runtime_preflight_matches;
use crate::runtime_ingress::telegram_receive_once_response;
use crate::runtime_mutation::RUNTIME_MUTATION_CANARY_ENV;
use crate::runtime_mutation::body_admitted as runtime_mutation_body_admitted;
use crate::runtime_mutation::enabled as runtime_mutation_enabled;
use crate::ui_domain::index_html;
use crate::ui_domain::route_native_gateway_binary_asset;

mod evidence_api;
mod evidence_dispatch;
mod http_rejections;
mod legacy_route_usage;
mod manifest_dispatch;
pub(crate) mod native_report_registry;
mod report_pagination;
mod report_response;
mod watchdog_state;

const RELEASE_BUILD_VERIFIED_ENV: &str = "HEPTA_CODEX_RELEASE_BUILD_VERIFIED";
const CONTROL_UI_PARITY_VERIFIED_ENV: &str = "HEPTA_CODEX_CONTROL_UI_PARITY_VERIFIED";
const HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV: &str =
    "HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED";
const CURRENT_HEPTA_CODEX_SCRIPT_TOTAL: usize = 21;
const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();
const NATIVE_GATEWAY_ROUTE_COUNT_CUTOVER_FLOOR: usize = 69;
const HEPTA_PROVIDER_CREDENTIALED_SMOKE_VERIFIED_ENV: &str =
    "HEPTA_PROVIDER_CREDENTIALED_SMOKE_VERIFIED";
const HEPTA_CHANNEL_LIVE_DELIVERY_VERIFIED_ENV: &str = "HEPTA_CHANNEL_LIVE_DELIVERY_VERIFIED";
const HEPTA_CHANNEL_LIVE_READ_VERIFIED_ENV: &str = "HEPTA_CHANNEL_LIVE_READ_VERIFIED";
const HEPTA_CHANNEL_LIVE_SEND_VERIFIED_ENV: &str = "HEPTA_CHANNEL_LIVE_SEND_VERIFIED";
const HEPTA_RELEASE_PROVENANCE_VERIFIED_ENV: &str = "HEPTA_RELEASE_PROVENANCE_VERIFIED";
const HEPTA_ACTIVE_BINARY_CONSISTENCY_VERIFIED_ENV: &str =
    "HEPTA_ACTIVE_BINARY_CONSISTENCY_VERIFIED";
const HEPTA_OLD_CLI_INVOCATION_COMPATIBILITY_VERIFIED_ENV: &str =
    "HEPTA_OLD_CLI_INVOCATION_COMPATIBILITY_VERIFIED";
const HEPTA_LAUNCHD_SERVICE_MUTATION_VERIFIED_ENV: &str = "HEPTA_LAUNCHD_SERVICE_MUTATION_VERIFIED";
const HEPTA_RECURRING_WATCHDOG_INSTALLED_ENV: &str = "HEPTA_RECURRING_WATCHDOG_INSTALLED";
const HEPTA_LOCAL_IMPORT_COMPATIBILITY_VERIFIED_ENV: &str =
    "HEPTA_LOCAL_IMPORT_COMPATIBILITY_VERIFIED";
const HEPTA_AUTONOMOUS_SUBAGENT_GATE_COMPATIBILITY_VERIFIED_ENV: &str =
    "HEPTA_AUTONOMOUS_SUBAGENT_GATE_COMPATIBILITY_VERIFIED";
const LEGACY_OPENCLAW_CONFIG_PATH_ENV: &str = "HEPTA_LEGACY_OPENCLAW_CONFIG_PATH";
const ACTIVE_GATEWAY_LABEL: &str = "ai.hepta.gateway";
const HEPTA_ACTIVE_RELEASE_BINARY: &str = "/Users/qianqi/.local/opt/hepta/bin/hepta";
const HEPTA_CODEX_TRANSITION_BINARY: &str = "/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex";
const MAX_NATIVE_SESSION_SUMMARIES: usize = 20;
const MAX_NATIVE_TRANSCRIPT_FILES: usize = 5;
const MAX_NATIVE_TRANSCRIPT_QUERY_FILES: usize = 20;
const MAX_NATIVE_TRANSCRIPT_LINES_PER_FILE: usize = 2_000;
const MAX_NATIVE_TRANSCRIPT_EVENT_PREVIEWS_PER_FILE: usize = 40;
const MAX_NATIVE_EVENT_FILES: usize = 20;
const MAX_NATIVE_EVENT_PREVIEWS: usize = 80;
const NATIVE_GATEWAY_WORKER_COUNT: usize = 8;
const NATIVE_GATEWAY_CONNECTION_QUEUE_CAPACITY: usize = 64;
const NATIVE_TASK_ARTIFACT_ROUTE_SPECS: &[NativeTaskArtifactRouteSpec] = &[
    NativeTaskArtifactRouteSpec {
        prefix: "/api/task/",
        source_command: "/task <task_id> --json",
        artifact_kind: "task_drilldown",
        compatibility_mode: "native_task_drilldown_redacted",
    },
    NativeTaskArtifactRouteSpec {
        prefix: "/api/task-patches/",
        source_command: "/task-patches <task_id> --json",
        artifact_kind: "task_patches",
        compatibility_mode: "native_task_patches_redacted",
    },
    NativeTaskArtifactRouteSpec {
        prefix: "/api/task-evidence/",
        source_command: "/task-evidence <task_id> --json",
        artifact_kind: "task_evidence",
        compatibility_mode: "native_task_evidence_redacted",
    },
    NativeTaskArtifactRouteSpec {
        prefix: "/api/task-replay/",
        source_command: "/task-replay <task_id> --json",
        artifact_kind: "task_replay",
        compatibility_mode: "native_task_replay_redacted",
    },
    NativeTaskArtifactRouteSpec {
        prefix: "/api/promotion-ledger/",
        source_command: "/promotion-ledger <task_id> --json",
        artifact_kind: "promotion_ledger",
        compatibility_mode: "native_promotion_ledger_redacted",
    },
    NativeTaskArtifactRouteSpec {
        prefix: "/api/handoff-bundle/",
        source_command: "/handoff-bundle <task_id> --json",
        artifact_kind: "handoff_bundle",
        compatibility_mode: "native_handoff_bundle_redacted",
    },
];

pub async fn run_native_gateway(
    options: NativeGatewayOptions,
    runtime: NativeGatewayRuntime,
) -> Result<()> {
    if !is_loopback_bind_addr(&options.bind_addr) && !allow_non_loopback_ui() {
        anyhow::bail!(
            "refusing to serve UI on non-loopback address {}; set HEPTA_ALLOW_NON_LOOPBACK_UI=1 only for an explicit local lab exposure",
            options.bind_addr
        );
    }
    runtime.validate_readiness()?;
    println!(
        "Hepta Architecture V2 runtime composition ready: durable_outcomes={} durable_preferences={} authenticated_preference_plan_and_commit=true secure_sensitive_ingress=true monotonic_anchor={} effect_reconciliation={} operator_mutation={} operator_telegram_pipeline={} legacy_telegram_live=false transport_confidentiality_claimed=local_host_only",
        runtime.outcome_mode(),
        runtime.preference_mode(),
        runtime.monotonic_anchor_enabled(),
        runtime.effect_reconciliation_enabled(),
        operator_mutation_enabled(),
        runtime.telegram_operator_pipeline_enabled(),
    );
    let runtime = Arc::new(runtime);

    if options.with_telegram_plugin {
        let telegram_plugin =
            native_telegram::telegram_plugin_status(true, options.telegram_plugin_poll_ms);
        eprintln!(
            "hepta-codex native gateway accepted --with-telegram-plugin; native Telegram supervisor status={} config_ready={} reply_loop_ready=false",
            telegram_plugin.status,
            telegram_plugin.config.config_ready()
        );
        if native_telegram::spawn_telegram_poll_loop_if_enabled(
            true,
            options.telegram_plugin_poll_ms,
            Arc::clone(&runtime),
        )
        .is_some()
        {
            eprintln!(
                "hepta-codex native Telegram poll loop armed at {} ms",
                options.telegram_plugin_poll_ms
            );
        }
    }

    let listener = TcpListener::bind(&options.bind_addr)
        .with_context(|| format!("failed to bind {}", options.bind_addr))?;
    println!(
        "Hepta native gateway listening on http://{}/",
        options.bind_addr
    );
    let connection_pool = NativeGatewayConnectionPool::new(
        options,
        runtime,
        NATIVE_GATEWAY_WORKER_COUNT,
        NATIVE_GATEWAY_CONNECTION_QUEUE_CAPACITY,
    )?;
    println!(
        "Native gateway HTTP admission: {} workers, {} queued connections, {} byte headers, {} byte bodies, {}s idle read timeout, {}s absolute request deadline, {}s idle write timeout, {}s absolute response deadline.",
        NATIVE_GATEWAY_WORKER_COUNT,
        NATIVE_GATEWAY_CONNECTION_QUEUE_CAPACITY,
        MAX_HTTP_HEADER_BYTES,
        MAX_HTTP_BODY_BYTES,
        HTTP_READ_TIMEOUT.as_secs(),
        HTTP_REQUEST_DEADLINE.as_secs(),
        HTTP_WRITE_TIMEOUT.as_secs(),
        HTTP_RESPONSE_DEADLINE.as_secs(),
    );
    println!("Press Ctrl-C to stop.");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => connection_pool.dispatch(stream)?,
            Err(error) => {
                eprintln!("native gateway connection accept failed: {error}");
            }
        }
    }

    Ok(())
}

struct NativeGatewayConnectionPool {
    sender: SyncSender<NativeGatewayConnection>,
}

struct NativeGatewayConnection {
    stream: TcpStream,
    deadline: Instant,
}

impl NativeGatewayConnectionPool {
    fn new(
        options: NativeGatewayOptions,
        runtime: Arc<NativeGatewayRuntime>,
        worker_count: usize,
        queue_capacity: usize,
    ) -> Result<Self> {
        if worker_count == 0 || queue_capacity == 0 {
            anyhow::bail!("native gateway worker and queue capacity must be positive");
        }
        let (sender, receiver) = mpsc::sync_channel::<NativeGatewayConnection>(queue_capacity);
        let receiver = Arc::new(Mutex::new(receiver));
        for index in 0..worker_count {
            let receiver = Arc::clone(&receiver);
            let runtime = Arc::clone(&runtime);
            let options = options.clone();
            thread::Builder::new()
                .name(format!("hepta-native-http-{index}"))
                .spawn(move || native_gateway_worker_loop(receiver, options, runtime))
                .with_context(|| format!("spawn native gateway HTTP worker {index}"))?;
        }
        Ok(Self { sender })
    }

    fn dispatch(&self, stream: TcpStream) -> Result<()> {
        let connection = NativeGatewayConnection {
            stream,
            deadline: Instant::now() + HTTP_REQUEST_DEADLINE,
        };
        match self.sender.try_send(connection) {
            Ok(()) => Ok(()),
            Err(TrySendError::Full(mut connection)) => {
                let rejection = configure_http_stream(&connection.stream).and_then(|()| {
                    write_http_response_with_timeout(
                        &mut connection.stream,
                        "503 Service Unavailable",
                        "application/json; charset=utf-8",
                        br#"{"error":"native gateway connection capacity exhausted"}"#,
                        HTTP_OVERLOAD_WRITE_TIMEOUT,
                    )
                    .context("write native gateway capacity response")
                });
                if let Err(error) = rejection {
                    eprintln!(
                        "native gateway overloaded connection rejection failed; closing connection: {error:#}"
                    );
                }
                Ok(())
            }
            Err(TrySendError::Disconnected(_)) => {
                anyhow::bail!("native gateway HTTP worker pool disconnected")
            }
        }
    }
}

fn native_gateway_worker_loop(
    receiver: Arc<Mutex<mpsc::Receiver<NativeGatewayConnection>>>,
    options: NativeGatewayOptions,
    runtime: Arc<NativeGatewayRuntime>,
) {
    loop {
        let connection = match receiver.lock() {
            Ok(receiver) => receiver.recv(),
            Err(_) => {
                eprintln!("native gateway HTTP worker queue lock was poisoned");
                return;
            }
        };
        let Ok(mut connection) = connection else {
            return;
        };
        if let Err(error) = handle_native_gateway_connection(
            &mut connection.stream,
            connection.deadline,
            &options,
            &runtime,
        ) {
            eprintln!("native gateway HTTP connection failed: {error:#}");
        }
    }
}

fn handle_native_gateway_connection(
    stream: &mut TcpStream,
    deadline: Instant,
    options: &NativeGatewayOptions,
    runtime: &NativeGatewayRuntime,
) -> Result<()> {
    configure_http_stream(stream)?;
    let request = match read_http_request_with_deadline(stream, deadline) {
        Ok(request) => request,
        Err(error) => return http_rejections::request_error(stream, error),
    };
    let Some((method, path)) = request_method_and_path(&request) else {
        return http_rejections::response(
            stream,
            "400 Bad Request",
            "application/json; charset=utf-8",
            br#"{"error":"bad request"}"#,
        );
    };
    if !matches!(method, "GET" | "POST") {
        return http_rejections::response(
            stream,
            "405 Method Not Allowed",
            "text/plain; charset=utf-8",
            b"method not allowed; supported POST endpoints are /api/actions/<action> and native POST route specs",
        );
    }
    if method == "POST"
        && crate::sensitive_http::requires_admission(path)
        && !crate::sensitive_http::admit(&request, &options.bind_addr)
    {
        return http_rejections::response(
            stream,
            "403 Forbidden",
            "application/json; charset=utf-8",
            br#"{"error":"secure_sensitive_ingress.denied","required_transport":"loopback+matching-host-origin+csrf+json"}"#,
        );
    }
    let request_body = request_body_text(&request);
    let request_query = request_query(&request);
    let preflight = match runtime.preflight_request(method, path, request_body) {
        Ok(preflight) => preflight,
        Err(error) => return http_rejections::runtime_ingress(stream, method, path, &error),
    };
    if !runtime_preflight_matches(method, path, &preflight)
        || preflight.mutation_authorized
        || preflight.durable_intent_recorded
        || preflight.provider_effect_ack_recorded
        || preflight.terminal_receipt_recorded
    {
        return http_rejections::response(
            stream,
            "503 Service Unavailable",
            "application/json; charset=utf-8",
            br#"{"error":"runtime request preflight invalid"}"#,
        );
    }
    manifest_dispatch::dispatch_manifest_route(
        stream,
        method,
        path,
        options,
        runtime,
        request_body,
        request_query,
        &preflight,
    )
}

#[cfg(test)]
fn route_native_gateway_request(
    method: &str,
    path: &str,
    options: &NativeGatewayOptions,
) -> (&'static str, &'static str, String) {
    route_native_gateway_request_with_body(method, path, options, None)
}

#[cfg(test)]
fn route_native_gateway_request_with_body(
    method: &str,
    path: &str,
    options: &NativeGatewayOptions,
    request_body: Option<&str>,
) -> (&'static str, &'static str, String) {
    if tests::is_quarantined_control_ui_route(method, path) {
        let preflight = tests::quarantined_preflight();
        return route_native_gateway_request_after_preflight(
            method,
            path,
            options,
            request_body,
            &preflight,
        );
    }
    let Some(lifecycle) = runtime_ingress_lifecycle(method, path) else {
        if crate::runtime_ingress::is_detached_control_ui_report_for_test(method, path) {
            let preflight = tests::quarantined_preflight();
            return route_native_gateway_request_after_preflight(
                method,
                path,
                options,
                request_body,
                &preflight,
            );
        }
        return http_rejections::runtime_ingress_tuple(method, path);
    };
    let preflight = RuntimeRequestPreflightReceipt {
        request_binding_hash: "unit-test-request-binding".into(),
        disposition: lifecycle.disposition(),
        ingress_kind: lifecycle.ingress_kind(),
        mutation_authorized: false,
        durable_intent_recorded: false,
        provider_effect_ack_recorded: false,
        terminal_receipt_recorded: false,
    };
    route_native_gateway_request_with_preflight(method, path, options, request_body, &preflight)
}

fn route_native_gateway_request_with_preflight(
    method: &str,
    path: &str,
    options: &NativeGatewayOptions,
    request_body: Option<&str>,
    preflight: &RuntimeRequestPreflightReceipt,
) -> (&'static str, &'static str, String) {
    if !runtime_preflight_matches(method, path, preflight)
        || preflight.mutation_authorized
        || preflight.durable_intent_recorded
        || preflight.provider_effect_ack_recorded
        || preflight.terminal_receipt_recorded
    {
        return (
            "503 Service Unavailable",
            "application/json; charset=utf-8",
            r#"{"error":"runtime request preflight invalid"}"#.to_string(),
        );
    }
    route_native_gateway_request_after_preflight(method, path, options, request_body, preflight)
}

include!("native_gateway/generated_dispatch.rs");

fn operator_snapshot_json(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    let gateway_replacement_readiness = gateway_replacement_readiness(options, telegram_plugin);
    let control_ui_route_parity = control_ui_route_parity_report();
    let telegram_poll_loop_status = native_telegram::telegram_poll_loop_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let telegram_live_soak_status = native_telegram::telegram_live_soak_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let telegram_cursor_status =
        native_telegram::telegram_cursor_status(options.with_telegram_plugin);

    let production_soak_ready = telegram_live_soak_status.production_readiness.ready
        && telegram_live_soak_status.health_ready
        && telegram_poll_loop_status.loop_invokes_drain_once
        && telegram_cursor_status.status == "ready";

    json_or_error(&NativeOperatorSnapshotResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if gateway_replacement_readiness.ready && production_soak_ready {
            "ready"
        } else {
            "attention"
        },
        source_command: "/operator-snapshot --json",
        native_route: true,
        compatibility_mode: "native_operator_snapshot",
        side_effect_free: true,
        health: HealthResponse {
            product: "Hepta",
            runtime: "hepta",
            status: "ready",
        },
        active_gateway_replacement_ready: gateway_replacement_readiness.ready,
        route_matrix_ready: control_ui_route_parity.ready,
        production_soak_ready,
        gateway_replacement_readiness,
        control_ui_route_parity,
        telegram_plugin,
        telegram_poll_loop_status,
        telegram_live_soak_status,
        telegram_cursor_status,
        telegram_read_performed: false,
        model_invoked: false,
        message_sent: false,
        cursor_written: false,
        raw_token_exposed: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        next_migration_slice: "promote the next high-value Control UI read-only route from parity shell to native status",
    })
}

fn native_sessions_json(source_command: &'static str, compatibility_mode: &'static str) -> String {
    json_or_error(&native_sessions_report(
        session_root_candidates(),
        source_command,
        compatibility_mode,
    ))
}

fn native_sessions_report(
    roots: Vec<NativeSessionRootCandidate>,
    source_command: &'static str,
    compatibility_mode: &'static str,
) -> NativeSessionsResponse {
    let mut root_reports = Vec::with_capacity(roots.len());
    let mut recent_sessions = Vec::new();
    let mut session_file_count = 0_u64;
    let mut total_bytes = 0_u64;
    let mut scan_error_count = 0_usize;

    for root in roots {
        let report = scan_session_root(&root);
        session_file_count = session_file_count.saturating_add(report.file_count);
        total_bytes = total_bytes.saturating_add(report.total_bytes);
        scan_error_count += usize::from(report.error.is_some());
        recent_sessions.extend(report.recent_sessions.clone());
        root_reports.push(report);
    }

    recent_sessions.sort_by(|left, right| {
        right
            .modified_unix_ms
            .cmp(&left.modified_unix_ms)
            .then_with(|| right.bytes.cmp(&left.bytes))
            .then_with(|| left.filename.cmp(&right.filename))
    });
    recent_sessions.truncate(MAX_NATIVE_SESSION_SUMMARIES);

    NativeSessionsResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if scan_error_count == 0 {
            "ready"
        } else {
            "attention"
        },
        source_command,
        native_route: true,
        compatibility_mode,
        side_effect_free: true,
        scanned_root_count: root_reports.len(),
        existing_root_count: root_reports.iter().filter(|root| root.exists).count(),
        scan_error_count,
        session_file_count,
        total_bytes,
        recent_session_count: recent_sessions.len(),
        roots: root_reports,
        recent_sessions,
        raw_transcript_exposed: false,
        transcript_text_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote task and transcript preview routes with the same metadata-only redaction boundary",
    }
}

fn session_root_candidates() -> Vec<NativeSessionRootCandidate> {
    let mut roots = Vec::new();
    let mut seen = HashSet::new();
    for home in session_home_candidates() {
        push_session_root_candidate(&mut roots, &mut seen, home.join("sessions"), "active");
        push_session_root_candidate(
            &mut roots,
            &mut seen,
            home.join("archived_sessions"),
            "archived",
        );
    }
    roots
}

fn session_home_candidates() -> Vec<PathBuf> {
    let mut homes = Vec::new();
    let mut seen = HashSet::new();

    for env_name in ["HEPTA_OPENAI_CODEX_HOME", "HEPTA_HOME", "CODEX_HOME"] {
        if let Ok(value) = env::var(env_name) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                push_unique_path(&mut homes, &mut seen, PathBuf::from(trimmed));
            }
        }
    }

    if let Ok(home) = env::var("HOME") {
        let home = PathBuf::from(home);
        push_unique_path(
            &mut homes,
            &mut seen,
            home.join(".openclaw/agents/main/agent/codex-home"),
        );
        push_unique_path(&mut homes, &mut seen, home.join(".codex"));
        push_unique_path(&mut homes, &mut seen, home.join(".hepta/workspace"));
    }

    homes
}

fn push_session_root_candidate(
    roots: &mut Vec<NativeSessionRootCandidate>,
    seen: &mut HashSet<String>,
    root: PathBuf,
    kind: &'static str,
) {
    let key = root.to_string_lossy().to_string();
    if seen.insert(key) {
        roots.push(NativeSessionRootCandidate { root, kind });
    }
}

fn push_unique_path(paths: &mut Vec<PathBuf>, seen: &mut HashSet<String>, path: PathBuf) {
    let key = path.to_string_lossy().to_string();
    if seen.insert(key) {
        paths.push(path);
    }
}

fn scan_session_root(candidate: &NativeSessionRootCandidate) -> NativeSessionRootReport {
    if !candidate.root.exists() {
        return NativeSessionRootReport {
            root: candidate.root.display().to_string(),
            kind: candidate.kind,
            exists: false,
            file_count: 0,
            total_bytes: 0,
            latest_modified_unix_ms: None,
            error: None,
            recent_sessions: Vec::new(),
        };
    }

    let mut report = NativeSessionRootReport {
        root: candidate.root.display().to_string(),
        kind: candidate.kind,
        exists: true,
        file_count: 0,
        total_bytes: 0,
        latest_modified_unix_ms: None,
        error: None,
        recent_sessions: Vec::new(),
    };
    scan_session_root_inner(&candidate.root, &candidate.root, &mut report);
    report.recent_sessions.sort_by(|left, right| {
        right
            .modified_unix_ms
            .cmp(&left.modified_unix_ms)
            .then_with(|| right.bytes.cmp(&left.bytes))
            .then_with(|| left.filename.cmp(&right.filename))
    });
    report
        .recent_sessions
        .truncate(MAX_NATIVE_SESSION_SUMMARIES);
    report
}

fn scan_session_root_inner(root: &Path, path: &Path, report: &mut NativeSessionRootReport) {
    if report.error.is_some() {
        return;
    }

    let entries = match std::fs::read_dir(path) {
        Ok(entries) => entries,
        Err(err) => {
            report.error = Some(err.to_string());
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                report.error = Some(err.to_string());
                return;
            }
        };
        let path = entry.path();
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                report.error = Some(err.to_string());
                return;
            }
        };
        if metadata.is_dir() {
            scan_session_root_inner(root, &path, report);
        } else if metadata.is_file() && is_rollout_file(&path) {
            let modified_unix_ms = metadata_modified_unix_ms(&metadata);
            report.file_count = report.file_count.saturating_add(1);
            report.total_bytes = report.total_bytes.saturating_add(metadata.len());
            report.latest_modified_unix_ms = report
                .latest_modified_unix_ms
                .max(modified_unix_ms)
                .or(modified_unix_ms);
            report.recent_sessions.push(session_summary_from_path(
                root,
                &path,
                metadata.len(),
                modified_unix_ms,
            ));
        }
    }
}

fn is_rollout_file(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("jsonl"))
        && path
            .file_name()
            .and_then(OsStr::to_str)
            .is_some_and(|name| name.starts_with("rollout-"))
}

fn session_summary_from_path(
    root: &Path,
    path: &Path,
    bytes: u64,
    modified_unix_ms: Option<u64>,
) -> NativeSessionSummary {
    let filename = path
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or_default()
        .to_string();
    let (started_at_filename, session_id) = rollout_filename_parts(&filename);
    NativeSessionSummary {
        session_id,
        started_at_filename,
        filename,
        relative_path: path
            .strip_prefix(root)
            .unwrap_or(path)
            .display()
            .to_string(),
        bytes,
        modified_unix_ms,
    }
}

fn rollout_filename_parts(filename: &str) -> (Option<String>, String) {
    let Some(rest) = filename
        .strip_prefix("rollout-")
        .and_then(|value| value.strip_suffix(".jsonl"))
    else {
        return (None, filename.to_string());
    };
    let started_at = rest.get(..19).map(str::to_string);
    let session_id = rest
        .get(20..)
        .filter(|value| !value.is_empty())
        .unwrap_or(rest)
        .to_string();
    (started_at, session_id)
}

fn metadata_modified_unix_ms(metadata: &std::fs::Metadata) -> Option<u64> {
    metadata
        .modified()
        .ok()
        .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
}

fn native_transcript_json(query: Option<&str>) -> String {
    json_or_error(&native_transcript_report(
        session_root_candidates(),
        query,
        if query.is_some() {
            MAX_NATIVE_TRANSCRIPT_QUERY_FILES
        } else {
            MAX_NATIVE_TRANSCRIPT_FILES
        },
    ))
}

fn native_transcript_report(
    roots: Vec<NativeSessionRootCandidate>,
    query: Option<&str>,
    max_files: usize,
) -> NativeTranscriptResponse {
    let mut candidates = collect_session_file_candidates(&roots);
    candidates.sort_by(|left, right| {
        right
            .summary
            .modified_unix_ms
            .cmp(&left.summary.modified_unix_ms)
            .then_with(|| right.summary.bytes.cmp(&left.summary.bytes))
            .then_with(|| left.summary.filename.cmp(&right.summary.filename))
    });
    let session_file_count = candidates.len();
    candidates.truncate(max_files);

    let query_lower = query.map(str::to_ascii_lowercase);
    let mut previews = Vec::new();
    let mut parse_error_count = 0_usize;
    let mut matched_session_count = 0_usize;
    let mut matched_line_count = 0_u64;

    for candidate in candidates {
        let preview = transcript_preview_from_file(&candidate, query_lower.as_deref());
        parse_error_count += preview.parse_error_count;
        if preview.query_match.matched_line_count > 0 {
            matched_session_count += 1;
            matched_line_count =
                matched_line_count.saturating_add(preview.query_match.matched_line_count);
        }
        previews.push(preview);
    }

    let scan_error_count = previews
        .iter()
        .filter(|preview| preview.read_error.is_some())
        .count();

    NativeTranscriptResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if scan_error_count == 0 {
            "ready"
        } else {
            "attention"
        },
        source_command: if query.is_some() {
            "/query-transcript <query> --json"
        } else {
            "/transcript --json"
        },
        native_route: true,
        compatibility_mode: if query.is_some() {
            "native_query_transcript_redacted"
        } else {
            "native_transcript_redacted_preview"
        },
        side_effect_free: true,
        query_present: query.is_some(),
        query_redacted: query.is_some(),
        query_length: query.map(str::len),
        scanned_session_file_count: previews.len(),
        available_session_file_count: session_file_count,
        max_files,
        max_lines_per_file: MAX_NATIVE_TRANSCRIPT_LINES_PER_FILE,
        matched_session_count,
        matched_line_count,
        parse_error_count,
        scan_error_count,
        sessions: previews,
        raw_transcript_exposed: false,
        transcript_text_exposed: false,
        query_text_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote task drilldown routes with artifact metadata and redacted evidence previews",
    }
}

fn native_task_artifact_json(spec: &NativeTaskArtifactRouteSpec, task_id: &str) -> String {
    json_or_error(&native_task_artifact_report(spec, task_id))
}

fn native_task_artifact_report(
    spec: &NativeTaskArtifactRouteSpec,
    task_id: &str,
) -> NativeTaskArtifactResponse {
    let evidence_search = native_transcript_report(
        session_root_candidates(),
        Some(task_id),
        MAX_NATIVE_TRANSCRIPT_QUERY_FILES,
    );
    let status = if evidence_search.scan_error_count == 0 {
        "ready"
    } else {
        "attention"
    };

    NativeTaskArtifactResponse {
        product: "Hepta",
        runtime: "hepta",
        status,
        source_command: spec.source_command,
        native_route: true,
        compatibility_mode: spec.compatibility_mode,
        side_effect_free: true,
        artifact_kind: spec.artifact_kind,
        task_id_redacted: true,
        task_id_length: task_id.len(),
        evidence_found: evidence_search.matched_line_count > 0,
        matched_session_count: evidence_search.matched_session_count,
        matched_line_count: evidence_search.matched_line_count,
        evidence_search,
        raw_task_id_exposed: false,
        raw_transcript_exposed: false,
        transcript_text_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "replace redacted task evidence search with structured task registry storage when available",
    }
}

#[cfg(test)]
fn native_post_plan_route_specs() -> &'static [NativePostPlanRouteSpec] {
    hepta_gateway::native_post_plan_route_specs()
}

#[cfg(test)]
fn native_post_plan_report(
    spec: &NativePostPlanRouteSpec,
    parameter: Option<&str>,
    request_body: Option<&str>,
) -> NativePostPlanResponse {
    hepta_gateway::native_post_plan_report(
        spec,
        parameter,
        request_body,
        env_truthy(NATIVE_POST_REAL_HANDLERS_ENV),
        env_truthy(NATIVE_POST_REAL_HANDLER_APPROVAL_ENV),
        native_post_real_handler_scope_from_env().as_deref(),
        &native_post_execution_store_root(),
        NativePostExecutionStoreLimits {
            max_store_bytes: native_post_store_max_bytes(),
            max_store_lines: native_post_store_max_lines(),
            rate_limit_window_ms: native_post_rate_limit_window_ms(),
        },
    )
}

#[cfg(test)]
fn native_post_body_schema(plan_kind: &str, body_read_during_plan: bool) -> NativePostBodySchema {
    hepta_gateway::native_post_body_schema(plan_kind, body_read_during_plan)
}

#[cfg(test)]
fn native_post_body_admission(
    spec: &NativePostPlanRouteSpec,
    schema: &NativePostBodySchema,
    request_body: Option<&str>,
) -> NativePostBodyAdmission {
    hepta_gateway::native_post_body_admission(spec, schema, request_body)
}

#[cfg(test)]
fn native_post_idempotency_evidence(
    spec: &NativePostPlanRouteSpec,
    body_admission: &NativePostBodyAdmission,
) -> NativePostIdempotencyEvidence {
    hepta_gateway::native_post_idempotency_evidence(spec, body_admission)
}

#[cfg(test)]
fn native_post_audit_event_contract(
    spec: &NativePostPlanRouteSpec,
    body_schema: &NativePostBodySchema,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
) -> NativePostAuditEventContract {
    hepta_gateway::native_post_audit_event_contract(
        spec,
        body_schema,
        body_admission,
        idempotency_evidence,
    )
}

#[cfg(test)]
fn native_post_execution_admission_with_gates(
    spec: &NativePostPlanRouteSpec,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    enablement_gate_enabled: bool,
    operator_approval_enabled: bool,
) -> NativePostExecutionAdmission {
    native_post_execution_admission_with_scope(
        spec,
        body_admission,
        idempotency_evidence,
        audit_event_contract,
        enablement_gate_enabled,
        operator_approval_enabled,
        Some(spec.plan_kind),
    )
}

#[cfg(test)]
fn native_post_execution_admission_with_scope(
    spec: &NativePostPlanRouteSpec,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    enablement_gate_enabled: bool,
    operator_approval_enabled: bool,
    handler_scope: Option<&str>,
) -> NativePostExecutionAdmission {
    hepta_gateway::native_post_execution_admission_with_scope(
        spec,
        body_admission,
        idempotency_evidence,
        audit_event_contract,
        enablement_gate_enabled,
        operator_approval_enabled,
        handler_scope,
    )
}

#[cfg(test)]
fn native_post_plan_kind_has_real_handler(plan_kind: &str) -> bool {
    hepta_gateway::native_post_plan_kind_has_real_handler(plan_kind)
}

fn native_post_real_handler_scope_from_env() -> Option<String> {
    env::var(NATIVE_POST_REAL_HANDLER_SCOPE_ENV)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

#[cfg(test)]
fn native_post_real_handler_harness(
    spec: &NativePostPlanRouteSpec,
    body_schema: &NativePostBodySchema,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    execution_admission: &NativePostExecutionAdmission,
    store_root: &Path,
) -> NativePostRealHandlerHarness {
    hepta_gateway::native_post_real_handler_harness(
        spec,
        body_schema,
        body_admission,
        idempotency_evidence,
        audit_event_contract,
        execution_admission,
        store_root,
        NativePostExecutionStoreLimits {
            max_store_bytes: native_post_store_max_bytes(),
            max_store_lines: native_post_store_max_lines(),
            rate_limit_window_ms: native_post_rate_limit_window_ms(),
        },
    )
}

fn native_post_execution_readiness_json() -> String {
    json_or_error(&native_post_execution_readiness_report())
}

fn native_post_activation_plan_json() -> String {
    json_or_error(&native_post_activation_plan_report())
}

fn native_post_rollout_evidence_json() -> String {
    json_or_error(&native_post_rollout_evidence_report())
}

fn native_post_gray_release_evidence_json() -> String {
    json_or_error(&native_post_gray_release_evidence_report())
}

fn native_post_gray_release_evidence_report() -> NativePostGrayReleaseEvidenceResponse {
    hepta_gateway::native_post_gray_release_evidence_report(
        &native_post_execution_store_root(),
        native_post_store_max_bytes(),
        native_post_store_max_lines(),
        native_post_real_handler_scope_from_env().as_deref(),
        env_truthy(NATIVE_POST_REAL_HANDLERS_ENV),
        env_truthy(NATIVE_POST_REAL_HANDLER_APPROVAL_ENV),
    )
}

fn native_post_rollout_evidence_report() -> NativePostRolloutEvidenceResponse {
    hepta_gateway::native_post_rollout_evidence_report(
        &native_post_execution_store_root(),
        native_post_store_max_bytes(),
        native_post_store_max_lines(),
        native_post_real_handler_scope_from_env().as_deref(),
    )
}

fn native_post_activation_plan_report() -> NativePostActivationPlanResponse {
    hepta_gateway::native_post_activation_plan_report(
        &native_post_execution_store_root(),
        native_post_store_max_bytes(),
        native_post_store_max_lines(),
        env_truthy(NATIVE_POST_REAL_HANDLERS_ENV),
        env_truthy(NATIVE_POST_REAL_HANDLER_APPROVAL_ENV),
        native_post_real_handler_scope_from_env().as_deref(),
    )
}

fn native_post_execution_readiness_report() -> NativePostExecutionReadinessResponse {
    hepta_gateway::native_post_execution_readiness_report(
        env_truthy(NATIVE_POST_REAL_HANDLERS_ENV),
        native_post_real_handler_scope_from_env().as_deref(),
    )
}

fn native_post_execution_stores_json() -> String {
    json_or_error(&native_post_execution_stores_report())
}

fn native_post_execution_stores_report() -> NativePostExecutionStoresResponse {
    hepta_gateway::native_post_execution_stores_report(
        &native_post_execution_store_root(),
        native_post_store_max_bytes(),
        native_post_store_max_lines(),
    )
}

fn native_post_execution_store_root() -> PathBuf {
    if let Ok(value) = env::var(NATIVE_POST_EXECUTION_STORE_DIR_ENV) {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return PathBuf::from(trimmed);
        }
    }
    env::var_os("HOME")
        .map(PathBuf::from)
        .filter(|home| home.is_absolute())
        .unwrap_or_else(|| PathBuf::from("/var/empty"))
        .join(".local/share/hepta")
        .join(
            Path::new(DEFAULT_NATIVE_POST_EXECUTION_STORE_DIR)
                .file_name()
                .unwrap_or_else(|| OsStr::new("native-post-execution")),
        )
}

fn native_post_store_max_bytes() -> u64 {
    env_u64(NATIVE_POST_STORE_MAX_BYTES_ENV)
        .map(|bytes| bytes.clamp(1_024, 1024 * 1024 * 1024))
        .unwrap_or(DEFAULT_NATIVE_POST_STORE_MAX_BYTES)
}

fn native_post_store_max_lines() -> u64 {
    env_u64(NATIVE_POST_STORE_MAX_LINES_ENV)
        .map(|lines| lines.clamp(1, 10_000_000))
        .unwrap_or(DEFAULT_NATIVE_POST_STORE_MAX_LINES)
}

#[cfg(test)]
fn native_post_execution_store_record(
    spec: &NativePostPlanRouteSpec,
    body_schema: &NativePostBodySchema,
    body_admission: &NativePostBodyAdmission,
    idempotency_evidence: &NativePostIdempotencyEvidence,
    audit_event_contract: &NativePostAuditEventContract,
    current_plan_executes_real_handler: bool,
) -> NativePostExecutionStoreRecord {
    hepta_gateway::native_post_execution_store_record(
        spec,
        body_schema,
        body_admission,
        idempotency_evidence,
        audit_event_contract,
        current_plan_executes_real_handler,
    )
}

#[cfg(test)]
fn native_post_execution_store_capacity_allows_append_with_limits(
    root: &Path,
    record: &NativePostExecutionStoreRecord,
    max_store_bytes: u64,
    max_store_lines: u64,
) -> Result<bool, String> {
    hepta_gateway::native_post_execution_store_capacity_allows_append_with_limits(
        root,
        record,
        max_store_bytes,
        max_store_lines,
    )
}

#[cfg(test)]
fn persist_native_post_execution_store_record(
    root: &Path,
    record: &NativePostExecutionStoreRecord,
) -> Result<NativePostExecutionStoreWriteReport, String> {
    hepta_gateway::persist_native_post_execution_store_record(root, record)
}

fn native_post_rate_limit_window_ms() -> u64 {
    env::var(NATIVE_POST_RATE_LIMIT_WINDOW_MS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS)
}

fn native_events_json(surface: NativeEventSurface, cursor: Option<&str>) -> String {
    json_or_error(&native_events_report(
        session_root_candidates(),
        surface,
        cursor,
    ))
}

fn native_events_report(
    roots: Vec<NativeSessionRootCandidate>,
    surface: NativeEventSurface,
    cursor: Option<&str>,
) -> NativeEventsResponse {
    let transcript = native_transcript_report(roots.clone(), None, MAX_NATIVE_EVENT_FILES);
    let activity_sessions = if surface.includes_activity_sessions() {
        Some(native_sessions_report(
            roots,
            "/activity sessions --json",
            "native_activity_session_inventory",
        ))
    } else {
        None
    };

    let mut event_type_counts = BTreeMap::<String, u64>::new();
    let mut recent_events = Vec::<NativeEventPreview>::new();
    let mut total_line_count = 0_u64;
    let mut parsed_json_line_count = 0_u64;
    let mut truncated_session_count = 0_usize;

    for session in &transcript.sessions {
        total_line_count = total_line_count.saturating_add(session.line_count);
        parsed_json_line_count =
            parsed_json_line_count.saturating_add(session.parsed_json_line_count);
        truncated_session_count += usize::from(session.truncated);
        for count in &session.event_type_counts {
            *event_type_counts
                .entry(count.event_type.clone())
                .or_default() += count.count;
        }
        for event in &session.redacted_events {
            if recent_events.len() >= MAX_NATIVE_EVENT_PREVIEWS {
                break;
            }
            recent_events.push(NativeEventPreview {
                root_kind: session.root_kind,
                session_id: session.session_id.clone(),
                started_at_filename: session.started_at_filename.clone(),
                relative_path: session.relative_path.clone(),
                line_number: event.line_number,
                event_type: event.event_type.clone(),
                role: event.role.clone(),
                has_text_fields: event.has_text_fields,
                redacted: true,
            });
        }
    }

    let activity_scan_errors = activity_sessions
        .as_ref()
        .map(|sessions| sessions.scan_error_count)
        .unwrap_or_default();
    let scan_error_count = transcript.scan_error_count + activity_scan_errors;
    let status = if scan_error_count == 0 {
        "ready"
    } else {
        "attention"
    };

    NativeEventsResponse {
        product: "Hepta",
        runtime: "hepta",
        status,
        source_command: surface.source_command(),
        native_route: true,
        compatibility_mode: surface.compatibility_mode(),
        side_effect_free: true,
        event_surface: surface.event_surface(),
        cursor_present: cursor.is_some(),
        cursor_redacted: cursor.is_some(),
        cursor_length: cursor.map(str::len),
        cursor_parseable_as_u64: cursor.and_then(|value| value.parse::<u64>().ok()).is_some(),
        scanned_session_file_count: transcript.scanned_session_file_count,
        available_session_file_count: transcript.available_session_file_count,
        max_files: transcript.max_files,
        max_lines_per_file: transcript.max_lines_per_file,
        total_line_count,
        parsed_json_line_count,
        parse_error_count: transcript.parse_error_count,
        scan_error_count,
        truncated_session_count,
        event_type_count: event_type_counts.len(),
        event_type_counts: event_type_counts
            .into_iter()
            .map(|(event_type, count)| NativeTranscriptEventCount { event_type, count })
            .collect(),
        recent_event_count: recent_events.len(),
        recent_events,
        activity_sessions,
        raw_transcript_exposed: false,
        transcript_text_exposed: false,
        raw_cursor_exposed: false,
        cursor_text_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote approvals, policy, and config surfaces with redacted local-only inventory",
    }
}

fn native_runtime_audit_json(surface: NativeRuntimeAuditSurface) -> String {
    json_or_error(&native_runtime_audit_report(
        session_root_candidates(),
        surface,
    ))
}

fn native_runtime_audit_report(
    roots: Vec<NativeSessionRootCandidate>,
    surface: NativeRuntimeAuditSurface,
) -> NativeRuntimeAuditResponse {
    let sessions = native_sessions_report(
        roots.clone(),
        surface.source_command(),
        "native_runtime_audit_session_inventory",
    );
    let events = native_events_report(roots, NativeEventSurface::EventsReport, None);
    let control_ui_route_parity = control_ui_route_parity_report();
    let approvals = native_approvals_report();
    let subagent_event_count =
        count_event_types_matching(&events.event_type_counts, runtime_event_type_is_subagent);
    let retry_or_error_event_count = count_event_types_matching(
        &events.event_type_counts,
        runtime_event_type_is_retry_or_error,
    );
    let multi_agent_event_count =
        count_event_types_matching(&events.event_type_counts, runtime_event_type_is_multi_agent);
    let ready = sessions.scan_error_count == 0
        && events.scan_error_count == 0
        && control_ui_route_parity.ready
        && approvals.status == "ready";

    NativeRuntimeAuditResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if ready { "ready" } else { "attention" },
        source_command: surface.source_command(),
        native_route: true,
        compatibility_mode: surface.compatibility_mode(),
        side_effect_free: true,
        audit_surface: surface.audit_surface(),
        event_focus: surface.event_focus(),
        agent_limit: surface.agent_limit(),
        message_limit: surface.message_limit(),
        route_matrix_ready: control_ui_route_parity.ready,
        route_count: control_ui_route_parity.route_count,
        missing_route_count: control_ui_route_parity.missing_route_count,
        approval_route_count: approvals.approval_route_count,
        guarded_approval_route_count: approvals.guarded_route_count,
        session_file_count: sessions.session_file_count,
        recent_session_count: sessions.recent_session_count,
        session_scan_error_count: sessions.scan_error_count,
        event_type_count: events.event_type_count,
        recent_event_count: events.recent_event_count,
        event_scan_error_count: events.scan_error_count,
        subagent_event_count,
        retry_or_error_event_count,
        multi_agent_event_count,
        sessions,
        events,
        redaction: NativeRuntimeAuditRedaction {
            raw_transcript_exposed: false,
            transcript_text_exposed: false,
            raw_agent_payload_exposed: false,
            raw_error_payload_exposed: false,
            raw_gateway_ledger_payload_exposed: false,
        },
        side_effects: NativeRuntimeAuditSideEffects {
            model_invoked: false,
            external_side_effects: false,
            gateway_mutation_performed: false,
            telegram_read_performed: false,
            message_sent: false,
            cursor_written: false,
        },
        next_migration_slice: "promote remaining runtime automation routes with the same local-only redacted audit boundary",
    }
}

fn count_event_types_matching(
    event_type_counts: &[NativeTranscriptEventCount],
    matches_event_type: fn(&str) -> bool,
) -> u64 {
    event_type_counts
        .iter()
        .filter(|count| matches_event_type(&count.event_type))
        .map(|count| count.count)
        .sum()
}

fn runtime_event_type_is_subagent(event_type: &str) -> bool {
    let event_type = event_type.to_ascii_lowercase();
    event_type.contains("subagent")
        || event_type.contains("sub_agent")
        || event_type.contains("agent_message")
}

fn runtime_event_type_is_retry_or_error(event_type: &str) -> bool {
    let event_type = event_type.to_ascii_lowercase();
    event_type.contains("retry")
        || event_type.contains("dead")
        || event_type.contains("error")
        || event_type.contains("failed")
        || event_type.contains("failure")
}

fn runtime_event_type_is_multi_agent(event_type: &str) -> bool {
    let event_type = event_type.to_ascii_lowercase();
    event_type.contains("multi_agent")
        || event_type.contains("multi-agent")
        || event_type.contains("subagent")
        || event_type.contains("agent_message")
}

fn native_control_ui_audit_json(
    surface: NativeControlUiAuditSurface,
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    json_or_error(&native_control_ui_audit_report(
        surface,
        options,
        telegram_plugin,
    ))
}

fn native_control_ui_audit_report(
    surface: NativeControlUiAuditSurface,
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> NativeControlUiAuditResponse {
    let route_matrix = control_ui_route_parity_report();
    let control_ui = hepta_core::control_ui_report();
    let approvals = native_approvals_report();
    let gateway_replacement = gateway_replacement_readiness(options, telegram_plugin);
    let get_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| !route.is_post())
        .count();
    let post_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.is_post())
        .count();
    let guarded_post_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.is_post())
        .filter(|route| route.is_guarded())
        .count();
    let dry_run_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.is_dry_run())
        .count();
    let read_only_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.is_read_only())
        .count();
    let ready = route_matrix.ready
        && approvals.status == "ready"
        && guarded_post_route_count == post_route_count;
    let control_ui_product_complete = control_ui.complete();

    NativeControlUiAuditResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if !ready {
            "attention"
        } else if surface.reports_control_ui_evidence() && !control_ui_product_complete {
            "static_contract_ready"
        } else {
            "ready"
        },
        source_command: surface.source_command(),
        native_route: true,
        compatibility_mode: surface.compatibility_mode(),
        side_effect_free: true,
        control_surface: surface.control_surface(),
        plan_target: surface.plan_target(),
        dry_run_only: surface.dry_run_only(),
        read_only: surface.read_only(),
        control_ui_product_status: control_ui.status,
        control_ui_product_complete,
        control_ui_live_operator_surface_percent: control_ui.live_operator_surface_percent,
        control_ui_evidence: control_ui.evidence_coverage,
        confirmation_required_for_real_mutation: false,
        route_matrix_ready: route_matrix.ready,
        route_count: route_matrix.route_count,
        implemented_route_count: route_matrix.implemented_route_count,
        missing_route_count: route_matrix.missing_route_count,
        get_route_count,
        post_route_count,
        dry_run_route_count,
        read_only_route_count,
        guarded_post_route_count,
        approval_route_count: approvals.approval_route_count,
        guarded_approval_route_count: approvals.guarded_route_count,
        gateway_replacement_ready: gateway_replacement.ready,
        gateway_replacement_blocker_count: gateway_replacement.blocker_count,
        external_agent_benchmark_executed: false,
        external_agent_spawned: false,
        action_dispatched: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        route_matrix,
        redaction: NativeControlUiAuditRedaction {
            raw_transcript_exposed: false,
            transcript_text_exposed: false,
            raw_token_exposed: false,
            raw_action_payload_exposed: false,
            raw_agent_payload_exposed: false,
        },
        side_effects: NativeControlUiAuditSideEffects {
            model_invoked: false,
            external_side_effects: false,
            gateway_mutation_performed: false,
            telegram_read_performed: false,
            message_sent: false,
            cursor_written: false,
        },
        next_migration_slice: "turn guarded POST action routes into explicit native dry-run planners before enabling any mutation",
    }
}

fn native_approvals_json() -> String {
    json_or_error(&native_approvals_report())
}

fn native_approvals_report() -> NativeApprovalsResponse {
    let approval_routes = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.is_post())
        .map(|route| NativeApprovalRoute {
            method: route.method,
            pattern: route.pattern,
            capability: route.capability,
            source_command: route.source_command,
            side_effect_boundary: route.side_effect_boundary,
            dry_run_only: route.is_dry_run(),
            guarded: route.is_guarded(),
            confirmation_required_for_real_mutation: route.requires_confirmation(),
        })
        .collect::<Vec<_>>();
    let guarded_route_count = approval_routes.iter().filter(|route| route.guarded).count();
    let pending_approval_count = 0usize;
    let ready = guarded_route_count == approval_routes.len();

    NativeApprovalsResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if ready { "ready" } else { "attention" },
        source_command: "/approvals --json",
        native_route: true,
        compatibility_mode: "native_approvals_redacted",
        side_effect_free: true,
        pending_approval_count,
        approval_route_count: approval_routes.len(),
        guarded_route_count,
        approval_routes,
        raw_command_payload_exposed: false,
        raw_approval_payload_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote approvals exec apply as an explicit dry-run plan before enabling mutation",
    }
}

fn native_policy_json(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    json_or_error(&native_policy_report(options, telegram_plugin))
}

fn native_policy_report(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> NativePolicyResponse {
    let gateway_replacement_readiness = gateway_replacement_readiness(options, telegram_plugin);
    let approvals = native_approvals_report();
    let loopback_bound = is_loopback_bind_addr(&options.bind_addr);
    let ready =
        loopback_bound && gateway_replacement_readiness.ready && approvals.status == "ready";

    NativePolicyResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if ready { "ready" } else { "attention" },
        source_command: "/policy --json",
        native_route: true,
        compatibility_mode: "native_policy_snapshot",
        side_effect_free: true,
        loopback_bind_required: true,
        loopback_bound,
        non_loopback_override_enabled: allow_non_loopback_ui(),
        bind_addr: options.bind_addr.clone(),
        telegram_gate_summary: native_telegram::telegram_gateway_gate_summary(),
        gateway_replacement_ready: gateway_replacement_readiness.ready,
        gateway_replacement_blocker_count: gateway_replacement_readiness.blocker_count,
        approval_route_count: approvals.approval_route_count,
        guarded_approval_route_count: approvals.guarded_route_count,
        raw_token_exposed: false,
        raw_transcript_exposed: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "thread policy snapshot into operator console once config and optional-configs are native",
    }
}

fn native_config_json(options: &NativeGatewayOptions) -> String {
    json_or_error(&native_config_report(options))
}

fn native_config_report(options: &NativeGatewayOptions) -> NativeConfigResponse {
    let config_roots = session_home_candidates()
        .into_iter()
        .map(|path| NativeConfigPathStatus {
            label: "session_home_candidate",
            path: path.display().to_string(),
            exists: path.exists(),
            is_dir: path.is_dir(),
            bytes: path
                .metadata()
                .ok()
                .filter(std::fs::Metadata::is_file)
                .map(|meta| meta.len()),
        })
        .collect::<Vec<_>>();

    NativeConfigResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        source_command: "/config-surface --json",
        native_route: true,
        compatibility_mode: "native_config_surface_redacted",
        side_effect_free: true,
        bind_addr: options.bind_addr.clone(),
        telegram_plugin_requested: options.with_telegram_plugin,
        telegram_plugin_poll_ms: options.telegram_plugin_poll_ms,
        default_model_present: env::var("HEPTA_DEFAULT_MODEL")
            .ok()
            .is_some_and(|value| !value.trim().is_empty()),
        telegram_model_present: env::var("HEPTA_TELEGRAM_MODEL")
            .ok()
            .is_some_and(|value| !value.trim().is_empty()),
        openai_codex_home_present: env::var("HEPTA_OPENAI_CODEX_HOME")
            .ok()
            .is_some_and(|value| !value.trim().is_empty()),
        gateway_token_file_present: env::var("HEPTA_GATEWAY_TOKEN_FILE")
            .ok()
            .is_some_and(|value| !value.trim().is_empty()),
        release_build_verified: env_truthy(RELEASE_BUILD_VERIFIED_ENV),
        control_ui_parity_verified: env_truthy(CONTROL_UI_PARITY_VERIFIED_ENV),
        telegram_gate_summary: native_telegram::telegram_gateway_gate_summary(),
        config_root_count: config_roots.len(),
        config_roots,
        raw_env_exposed: false,
        raw_token_exposed: false,
        raw_config_value_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote optional config catalog and config edit dry-run plans without exposing raw config values",
    }
}

fn native_optional_configs_json() -> String {
    json_or_error(&native_optional_configs_report())
}

fn native_optional_configs_report() -> NativeOptionalConfigsResponse {
    let workspace_root = env::var("HOME")
        .map(|home| PathBuf::from(home).join(".openclaw/workspace"))
        .unwrap_or_else(|_| PathBuf::from("."));
    let today = "2026-05-18";
    let yesterday = "2026-05-17";
    let candidates = [
        ("agents", workspace_root.join("AGENTS.md"), true),
        ("soul", workspace_root.join("SOUL.md"), true),
        ("user", workspace_root.join("USER.md"), true),
        ("tools", workspace_root.join("TOOLS.md"), false),
        ("heartbeat", workspace_root.join("HEARTBEAT.md"), false),
        ("long_term_memory", workspace_root.join("MEMORY.md"), true),
        (
            "today_memory",
            workspace_root.join(format!("memory/{today}.md")),
            true,
        ),
        (
            "yesterday_memory",
            workspace_root.join(format!("memory/{yesterday}.md")),
            false,
        ),
    ];
    let configs = candidates
        .into_iter()
        .map(|(label, path, expected)| {
            let metadata = path.metadata().ok();
            NativeOptionalConfigStatus {
                label,
                path: path.display().to_string(),
                expected,
                exists: metadata.is_some(),
                is_file: metadata.as_ref().is_some_and(std::fs::Metadata::is_file),
                bytes: metadata
                    .as_ref()
                    .filter(|meta| meta.is_file())
                    .map(std::fs::Metadata::len),
                content_exposed: false,
            }
        })
        .collect::<Vec<_>>();
    let missing_expected_count = configs
        .iter()
        .filter(|config| config.expected && !config.exists)
        .count();
    NativeOptionalConfigsResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if missing_expected_count == 0 {
            "ready"
        } else {
            "attention"
        },
        source_command: "/optional-configs --json",
        native_route: true,
        compatibility_mode: "native_optional_configs_redacted",
        side_effect_free: true,
        config_count: configs.len(),
        missing_expected_count,
        configs,
        raw_config_value_exposed: false,
        config_content_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote config edit and approval apply endpoints as explicit dry-run/confirm-required plans",
    }
}

fn collect_session_file_candidates(
    roots: &[NativeSessionRootCandidate],
) -> Vec<NativeSessionFileCandidate> {
    let mut files = Vec::new();
    for root in roots {
        if root.root.exists() {
            collect_session_file_candidates_inner(&root.root, &root.root, root.kind, &mut files);
        }
    }
    files
}

fn collect_session_file_candidates_inner(
    root: &Path,
    path: &Path,
    root_kind: &'static str,
    files: &mut Vec<NativeSessionFileCandidate>,
) {
    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        if metadata.is_dir() {
            collect_session_file_candidates_inner(root, &path, root_kind, files);
        } else if metadata.is_file() && is_rollout_file(&path) {
            files.push(NativeSessionFileCandidate {
                path: path.clone(),
                root_kind,
                summary: session_summary_from_path(
                    root,
                    &path,
                    metadata.len(),
                    metadata_modified_unix_ms(&metadata),
                ),
            });
        }
    }
}

fn transcript_preview_from_file(
    candidate: &NativeSessionFileCandidate,
    query_lower: Option<&str>,
) -> NativeTranscriptSessionPreview {
    let mut preview = NativeTranscriptSessionPreview {
        root_kind: candidate.root_kind,
        session_id: candidate.summary.session_id.clone(),
        started_at_filename: candidate.summary.started_at_filename.clone(),
        filename: candidate.summary.filename.clone(),
        relative_path: candidate.summary.relative_path.clone(),
        bytes: candidate.summary.bytes,
        modified_unix_ms: candidate.summary.modified_unix_ms,
        line_count: 0,
        parsed_json_line_count: 0,
        parse_error_count: 0,
        truncated: false,
        event_type_counts: Vec::new(),
        redacted_events: Vec::new(),
        query_match: NativeTranscriptQueryMatch {
            matched_line_count: 0,
            first_match_line: None,
            matched_event_type_counts: Vec::new(),
        },
        read_error: None,
    };

    let file = match std::fs::File::open(&candidate.path) {
        Ok(file) => file,
        Err(err) => {
            preview.read_error = Some(err.to_string());
            return preview;
        }
    };

    let mut event_type_counts = BTreeMap::<String, u64>::new();
    let mut matched_event_type_counts = BTreeMap::<String, u64>::new();

    for (index, line) in BufReader::new(file).lines().enumerate() {
        if index >= MAX_NATIVE_TRANSCRIPT_LINES_PER_FILE {
            preview.truncated = true;
            break;
        }

        let line_number = index + 1;
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                preview.read_error = Some(err.to_string());
                break;
            }
        };
        preview.line_count += 1;

        let value = match serde_json::from_str::<serde_json::Value>(&line) {
            Ok(value) => {
                preview.parsed_json_line_count += 1;
                value
            }
            Err(_) => {
                preview.parse_error_count += 1;
                continue;
            }
        };

        let event_type = transcript_event_type(&value);
        *event_type_counts.entry(event_type.clone()).or_default() += 1;

        let query_matched = query_lower
            .map(|query| line.to_ascii_lowercase().contains(query))
            .unwrap_or(false);
        if query_matched {
            preview.query_match.matched_line_count += 1;
            if preview.query_match.first_match_line.is_none() {
                preview.query_match.first_match_line = Some(line_number);
            }
            *matched_event_type_counts
                .entry(event_type.clone())
                .or_default() += 1;
        }

        if preview.redacted_events.len() < MAX_NATIVE_TRANSCRIPT_EVENT_PREVIEWS_PER_FILE {
            preview.redacted_events.push(redacted_transcript_event(
                line_number,
                &event_type,
                &value,
            ));
        }
    }

    preview.event_type_counts = event_type_counts
        .into_iter()
        .map(|(event_type, count)| NativeTranscriptEventCount { event_type, count })
        .collect();
    preview.query_match.matched_event_type_counts = matched_event_type_counts
        .into_iter()
        .map(|(event_type, count)| NativeTranscriptEventCount { event_type, count })
        .collect();
    preview
}

fn transcript_event_type(value: &serde_json::Value) -> String {
    let top_level = value
        .get("type")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    let payload_type = value
        .get("payload")
        .and_then(|payload| payload.get("type"))
        .and_then(serde_json::Value::as_str);
    match payload_type {
        Some(payload_type) => format!("{top_level}:{payload_type}"),
        None => top_level.to_string(),
    }
}

fn redacted_transcript_event(
    line_number: usize,
    event_type: &str,
    value: &serde_json::Value,
) -> NativeTranscriptEventPreview {
    let payload = value.get("payload");
    NativeTranscriptEventPreview {
        line_number,
        event_type: event_type.to_string(),
        role: payload
            .and_then(|payload| payload.get("role"))
            .and_then(serde_json::Value::as_str)
            .map(str::to_string),
        has_text_fields: json_contains_text_field(value),
        redacted: true,
    }
}

fn json_contains_text_field(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, value)| {
            key == "text" || key == "message" || key == "content" || json_contains_text_field(value)
        }),
        serde_json::Value::Array(values) => values.iter().any(json_contains_text_field),
        _ => false,
    }
}

fn operator_console_json(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    let gateway_replacement_readiness = gateway_replacement_readiness(options, telegram_plugin);
    let control_ui_route_parity = control_ui_route_parity_report();
    let sessions = native_sessions_report(
        session_root_candidates(),
        "/sessions --json",
        "native_sessions_inventory",
    );
    let telegram_live_soak_status = native_telegram::telegram_live_soak_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let telegram_poll_loop_status = native_telegram::telegram_poll_loop_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );

    let ready = gateway_replacement_readiness.ready
        && control_ui_route_parity.ready
        && sessions.scan_error_count == 0
        && telegram_live_soak_status.health_ready;

    json_or_error(&NativeOperatorConsoleResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if ready { "ready" } else { "attention" },
        source_command: "/operator-console --json",
        native_route: true,
        compatibility_mode: "native_operator_console",
        side_effect_free: true,
        health: HealthResponse {
            product: "Hepta",
            runtime: "hepta",
            status: "ready",
        },
        operator_snapshot_endpoint: "/api/operator-snapshot",
        operator_security_endpoint: "/api/operator-security",
        sessions_endpoint: "/api/sessions",
        session_activity_endpoint: "/api/session-activity",
        gateway_replacement_readiness,
        control_ui_route_parity,
        sessions,
        telegram_plugin,
        telegram_poll_loop_status,
        telegram_live_soak_status,
        raw_transcript_exposed: false,
        raw_token_exposed: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        model_invoked: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "promote transcript and task drilldown previews with explicit redaction contracts",
    })
}

fn operator_security_json(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    json_or_error(&operator_security_report(options, telegram_plugin))
}

fn operator_security_report(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> NativeOperatorSecurityResponse {
    let control_ui_route_parity = control_ui_route_parity_report();
    let gateway_replacement_readiness = gateway_replacement_readiness(options, telegram_plugin);
    let post_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.is_post())
        .count();
    let dry_run_post_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.is_post())
        .filter(|route| route.is_dry_run())
        .count();
    let guarded_post_route_count = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.is_post())
        .filter(|route| route.is_guarded())
        .count();
    let telegram_production_readiness_status =
        native_telegram::telegram_production_readiness_status(
            options.with_telegram_plugin,
            options.telegram_plugin_poll_ms,
        );
    let telegram_owner_handoff_status = telegram_owner_handoff_status(options);
    let post_execution_readiness = native_post_execution_readiness_report();
    let post_execution_stores = native_post_execution_stores_report();
    let post_activation_plan = native_post_activation_plan_report();
    let post_gray_release_evidence = native_post_gray_release_evidence_report();
    let post_execution_stores_ready =
        hepta_gateway::native_post_execution_store_contracts_ready(&post_execution_stores);
    let NativePostCompatibilityReadiness {
        activation_plan_ready: post_activation_plan_ready,
        gray_release_evidence_ready: post_gray_release_evidence_ready,
    } = native_post_compatibility_readiness(
        &post_execution_readiness,
        &post_activation_plan,
        &post_gray_release_evidence,
    );
    let production_soak_ready = telegram_production_readiness_status.ready;
    let loopback_bound = is_loopback_bind_addr(&options.bind_addr);
    let telegram_owner_or_parallel_ready = telegram_owner_handoff_status.hepta_takeover_ready
        || telegram_owner_handoff_status.hepta_parallel_bot_ready;
    let legacy_owner_coexistence_ready = control_ui_route_parity.ready
        && post_execution_readiness.all_evidence_contracts_ready
        && post_execution_stores_ready
        && post_activation_plan_ready
        && post_gray_release_evidence_ready
        && telegram_owner_handoff_status.conflict_free
        && telegram_owner_handoff_status.active_owner == "legacy_openclaw"
        && !telegram_owner_handoff_status.hepta_poll_loop_armed
        && loopback_bound
        && guarded_post_route_count == post_route_count;
    let ready = control_ui_route_parity.ready
        && gateway_replacement_readiness.ready
        && production_soak_ready
        && post_execution_readiness.all_evidence_contracts_ready
        && post_execution_stores_ready
        && post_activation_plan_ready
        && post_gray_release_evidence_ready
        && telegram_owner_handoff_status.conflict_free
        && telegram_owner_or_parallel_ready
        && loopback_bound
        && guarded_post_route_count == post_route_count;

    NativeOperatorSecurityResponse {
        product: "Hepta",
        runtime: "hepta",
        status: if ready { "ready" } else { "attention" },
        source_command: "/operator-security --json",
        native_route: true,
        compatibility_mode: "native_operator_security",
        side_effect_free: true,
        product_role: hepta_contracts::OPENCLAW_GOVERNED_BACKEND_ROLE,
        product_disposition: hepta_contracts::LEGACY_CONTROL_UI_MUTATION_DISPOSITION,
        security_mode: if ready {
            "active_replacement_ready"
        } else if legacy_owner_coexistence_ready {
            "legacy_owner_coexistence_ready"
        } else {
            "attention_required"
        },
        legacy_owner_coexistence_ready,
        attention_reason: if ready {
            "none"
        } else if legacy_owner_coexistence_ready {
            "telegram_replacement_not_requested"
        } else {
            "security_gate_not_ready"
        },
        loopback_bind_required: true,
        loopback_bound,
        non_loopback_override_enabled: allow_non_loopback_ui(),
        bind_addr: options.bind_addr.clone(),
        control_ui_route_parity,
        gateway_replacement_readiness,
        post_route_count,
        dry_run_post_route_count,
        guarded_post_route_count,
        post_execution_readiness_endpoint: NATIVE_POST_EXECUTION_READINESS_ENDPOINT,
        post_execution_stores_endpoint: NATIVE_POST_EXECUTION_STORES_ENDPOINT,
        post_activation_plan_endpoint: NATIVE_POST_ACTIVATION_PLAN_ENDPOINT,
        post_execution_readiness,
        post_execution_stores_ready,
        post_execution_stores,
        post_activation_plan_ready,
        post_activation_plan,
        post_gray_release_evidence_endpoint: NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT,
        post_gray_release_evidence_ready,
        post_gray_release_evidence,
        production_soak_ready,
        telegram_gate_summary: native_telegram::telegram_gateway_gate_summary(),
        telegram_production_readiness_status,
        telegram_owner_handoff_endpoint: TELEGRAM_OWNER_HANDOFF_ENDPOINT,
        telegram_owner_handoff_status,
        telegram_plugin_requested: options.with_telegram_plugin,
        telegram_plugin_status: telegram_plugin.status,
        redaction: NativeOperatorSecurityRedaction {
            raw_transcript_exposed: false,
            raw_token_exposed: false,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_response_text_exposed: false,
            raw_idempotency_key_exposed: false,
            raw_audit_payload_exposed: false,
        },
        side_effects: NativeOperatorSecuritySideEffects {
            external_side_effects: false,
            gateway_mutation_performed: false,
            telegram_read_performed: false,
            model_invoked: false,
            message_sent: false,
            cursor_written: false,
        },
        next_migration_slice: hepta_contracts::LEGACY_CONTROL_UI_MUTATION_NEXT_ACTION,
    }
}

fn telegram_owner_handoff_status(
    options: &NativeGatewayOptions,
) -> NativeTelegramOwnerHandoffStatus {
    let legacy_config_path = legacy_openclaw_config_path();
    let (
        legacy_config_found,
        legacy_config_parse_ok,
        legacy_telegram_enabled,
        legacy_token_fingerprint,
        error,
    ) = read_legacy_openclaw_telegram_state(legacy_config_path.as_deref());
    let poll_loop_status = native_telegram::telegram_poll_loop_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let gate_summary = native_telegram::telegram_gateway_gate_summary();
    let hepta_token_fingerprint = if options.with_telegram_plugin {
        native_telegram::effective_telegram_token_fingerprint()
    } else {
        None
    };

    telegram_owner_handoff_status_from_inputs(NativeTelegramOwnerHandoffInputs {
        legacy_config_path: legacy_config_path.map(|path| path.display().to_string()),
        legacy_config_found,
        legacy_config_parse_ok,
        legacy_telegram_enabled,
        legacy_token_fingerprint,
        legacy_config_error: error,
        hepta_token_fingerprint,
        hepta_telegram_requested: options.with_telegram_plugin,
        hepta_poll_loop_armed: poll_loop_status.status == "armed"
            && poll_loop_status.loop_invokes_drain_once,
        hepta_poll_loop_gate_enabled: poll_loop_status.poll_loop_gate_enabled,
        hepta_delivery_approval_gate_enabled: gate_summary.delivery_approval_gate_enabled,
    })
}

fn telegram_owner_handoff_status_from_inputs(
    inputs: NativeTelegramOwnerHandoffInputs,
) -> NativeTelegramOwnerHandoffStatus {
    let legacy_telegram_enabled = inputs.legacy_telegram_enabled;
    let legacy_telegram_enabled_explicit = legacy_telegram_enabled.is_some();
    let legacy_enabled = legacy_telegram_enabled == Some(true);
    let bot_identity_match = match (
        &inputs.legacy_token_fingerprint,
        &inputs.hepta_token_fingerprint,
    ) {
        (Some(legacy), Some(hepta)) => Some(legacy == hepta),
        _ => None,
    };
    let parallel_bot_mode =
        legacy_enabled && inputs.hepta_poll_loop_armed && bot_identity_match == Some(false);
    let double_poller_risk =
        legacy_enabled && inputs.hepta_poll_loop_armed && bot_identity_match != Some(false);
    let conflict_free = inputs.legacy_config_parse_ok && !double_poller_risk;
    let hepta_takeover_ready = conflict_free
        && inputs.hepta_telegram_requested
        && inputs.hepta_poll_loop_armed
        && !legacy_enabled;
    let hepta_parallel_bot_ready = conflict_free && parallel_bot_mode;

    let active_owner = if double_poller_risk {
        "conflict_risk"
    } else if parallel_bot_mode {
        "parallel_bots"
    } else if inputs.hepta_poll_loop_armed {
        "hepta"
    } else if legacy_enabled {
        "legacy_openclaw"
    } else {
        "none"
    };

    let status = if !inputs.legacy_config_parse_ok {
        "attention"
    } else if double_poller_risk {
        "conflict_risk"
    } else if hepta_parallel_bot_ready {
        "parallel_bot_ready"
    } else if hepta_takeover_ready {
        "hepta_takeover_ready"
    } else if legacy_enabled {
        "legacy_owner"
    } else {
        "handoff_pending"
    };

    let mut takeover_blockers = Vec::new();
    if !inputs.legacy_config_parse_ok {
        takeover_blockers.push("legacy_openclaw_config_unreadable");
    }
    if legacy_enabled && !parallel_bot_mode {
        takeover_blockers.push("legacy_openclaw_telegram_enabled");
    }
    if !inputs.hepta_telegram_requested {
        takeover_blockers.push("hepta_telegram_not_requested");
    }
    if !inputs.hepta_delivery_approval_gate_enabled {
        takeover_blockers.push("hepta_delivery_approval_gate_disabled");
    }
    if !inputs.hepta_poll_loop_gate_enabled {
        takeover_blockers.push("hepta_poll_loop_gate_disabled");
    }
    if !inputs.hepta_poll_loop_armed {
        takeover_blockers.push("hepta_poll_loop_not_armed");
    }

    NativeTelegramOwnerHandoffStatus {
        product: "Hepta",
        runtime: "hepta",
        status,
        endpoint: TELEGRAM_OWNER_HANDOFF_ENDPOINT,
        ready: conflict_free,
        conflict_free,
        hepta_takeover_ready,
        hepta_parallel_bot_ready,
        side_effect_free: true,
        active_owner,
        legacy_config_path: inputs.legacy_config_path,
        legacy_config_found: inputs.legacy_config_found,
        legacy_config_parse_ok: inputs.legacy_config_parse_ok,
        legacy_telegram_enabled,
        legacy_telegram_enabled_explicit,
        legacy_token_fingerprint: inputs.legacy_token_fingerprint,
        hepta_token_fingerprint: inputs.hepta_token_fingerprint,
        bot_identity_match,
        parallel_bot_mode,
        hepta_telegram_requested: inputs.hepta_telegram_requested,
        hepta_poll_loop_armed: inputs.hepta_poll_loop_armed,
        hepta_poll_loop_gate_enabled: inputs.hepta_poll_loop_gate_enabled,
        hepta_delivery_approval_gate_enabled: inputs.hepta_delivery_approval_gate_enabled,
        double_poller_risk,
        takeover_blockers,
        legacy_config_error: inputs.legacy_config_error,
        raw_token_exposed: false,
        raw_update_payload_exposed: false,
        raw_prompt_text_exposed: false,
        raw_response_text_exposed: false,
        next_migration_slice: if hepta_takeover_ready {
            "legacy Telegram polling is disabled and Hepta is the only armed poller; continue live soak and ledger freshness checks"
        } else if hepta_parallel_bot_ready {
            "legacy OpenClaw and Hepta use distinct Telegram bot identities; run bounded parallel-bot soak before any owner replacement"
        } else if double_poller_risk {
            "disable the legacy OpenClaw Telegram plugin or switch Hepta to a distinct bot identity before arming Hepta polling to avoid Bot API 409 conflicts"
        } else if legacy_enabled {
            "keep legacy OpenClaw as owner until Hepta is armed with a verified distinct bot identity or the controlled flip disables legacy Telegram"
        } else {
            "arm Hepta Telegram gates only after confirming legacy OpenClaw Telegram polling is disabled"
        },
    }
}

fn legacy_openclaw_config_path() -> Option<PathBuf> {
    if let Ok(path) = env::var(LEGACY_OPENCLAW_CONFIG_PATH_ENV) {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Some(path);
        }
    }

    env::var_os("HOME").map(|home| PathBuf::from(home).join(".openclaw/openclaw.json"))
}

fn read_legacy_openclaw_telegram_state(
    path: Option<&Path>,
) -> (bool, bool, Option<bool>, Option<String>, Option<String>) {
    let Some(path) = path else {
        return (false, true, None, None, None);
    };
    if !path.is_file() {
        return (false, true, None, None, None);
    }
    let raw = match fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(error) => {
            return (
                true,
                false,
                None,
                None,
                Some(format!("failed to read legacy OpenClaw config: {error}")),
            );
        }
    };
    let value = match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(value) => value,
        Err(error) => {
            return (
                true,
                false,
                None,
                None,
                Some(format!("failed to parse legacy OpenClaw config: {error}")),
            );
        }
    };

    (
        true,
        true,
        legacy_openclaw_telegram_enabled(&value),
        legacy_openclaw_telegram_token_fingerprint(path, &value),
        None,
    )
}

fn legacy_openclaw_telegram_enabled(value: &serde_json::Value) -> Option<bool> {
    value
        .pointer("/plugins/entries/telegram/enabled")
        .and_then(serde_json::Value::as_bool)
        .or_else(|| {
            value
                .pointer("/channels/telegram/enabled")
                .and_then(serde_json::Value::as_bool)
        })
}

fn legacy_openclaw_telegram_token_fingerprint(
    config_path: &Path,
    value: &serde_json::Value,
) -> Option<String> {
    let token_ref = value.pointer("/channels/telegram/botToken")?;
    let token = telegram_token_from_config_ref(config_path, value, token_ref)?;
    native_telegram::redacted_telegram_token_fingerprint(&token)
}

fn telegram_token_from_config_ref(
    config_path: &Path,
    config: &serde_json::Value,
    token_ref: &serde_json::Value,
) -> Option<String> {
    if let Some(inline) = token_ref.as_str() {
        return non_empty_trimmed(inline);
    }
    let source = token_ref
        .get("source")
        .and_then(serde_json::Value::as_str)?;
    if source != "file" {
        return None;
    }
    let provider = token_ref
        .get("provider")
        .and_then(serde_json::Value::as_str)?;
    let raw_path = config
        .get("secrets")?
        .get("providers")?
        .get(provider)?
        .get("path")?
        .as_str()?;
    let path = PathBuf::from(raw_path);
    let path = if path.is_absolute() {
        path
    } else if let Some(parent) = config_path.parent() {
        parent.join(path)
    } else {
        path
    };
    fs::read_to_string(path)
        .ok()
        .and_then(|token| non_empty_trimmed(&token))
}

fn non_empty_trimmed(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn gateway_replacement_readiness(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> NativeGatewayReplacementReadiness {
    let telegram_gate_summary = native_telegram::telegram_gateway_gate_summary();
    let telegram_poll_loop_status = native_telegram::telegram_poll_loop_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let telegram_owner_handoff_status = telegram_owner_handoff_status(options);
    let telegram_owner_or_parallel_ready = telegram_owner_handoff_status.hepta_takeover_ready
        || telegram_owner_handoff_status.hepta_parallel_bot_ready;
    let control_ui_route_parity = control_ui_route_parity_report();
    let release_build_verified = env_truthy(RELEASE_BUILD_VERIFIED_ENV);
    let control_ui_parity_verified =
        control_ui_route_parity.ready && env_truthy(CONTROL_UI_PARITY_VERIFIED_ENV);
    let model_runner_plan = native_telegram::telegram_model_runner_plan();
    let in_process_model_runner_ready =
        env_truthy(native_telegram::TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV);
    let hepta_kernel_model_runner_ready = native_telegram::telegram_hepta_kernel_runner_enabled();
    let telegram_model_runner_plan_ready = model_runner_plan.runner_plan_ready;
    let side_effect_free = true;

    let checks = vec![
        NativeGatewayReplacementCheck {
            name: "launchd_entrypoint_compatible",
            ready: true,
            detail: "serve-ui loopback entrypoint is available in hepta-codex",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_plugin_requested",
            ready: options.with_telegram_plugin,
            detail: "production replacement requires --with-telegram-plugin",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_config_binding_ready",
            ready: telegram_plugin.config.config_ready(),
            detail: "Telegram config, secure token source, and binding are redacted and resolvable",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_native_supervisor_ready",
            ready: telegram_plugin.in_process_supervisor_ready,
            detail: "native supervisor can load without OpenClaw gateway runtime dependency",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_delivery_approval_gate_enabled",
            ready: telegram_gate_summary.delivery_approval_gate_enabled,
            detail: "HEPTA_NATIVE_TELEGRAM_DELIVERY_APPROVED must be enabled before live poll loop drain side effects",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_live_read_gate_enabled",
            ready: telegram_gate_summary.live_read_gate_enabled,
            detail: "HEPTA_NATIVE_TELEGRAM_LIVE_READ must be enabled for active polling",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_model_turn_gate_enabled",
            ready: telegram_gate_summary.model_turn_gate_enabled,
            detail: "HEPTA_NATIVE_TELEGRAM_MODEL_TURN must be enabled for active replies",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_send_gate_enabled",
            ready: telegram_gate_summary.send_gate_enabled,
            detail: "HEPTA_NATIVE_TELEGRAM_SEND must be enabled for active delivery",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_poll_loop_gate_enabled",
            ready: telegram_poll_loop_status.poll_loop_gate_enabled,
            detail: "HEPTA_NATIVE_TELEGRAM_POLL_LOOP must be enabled for background draining",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_poll_loop_invokes_drain_once",
            ready: telegram_poll_loop_status.loop_invokes_drain_once,
            detail: "poll loop must route through the gated drain-once pipeline",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_owner_handoff_ready",
            ready: telegram_owner_or_parallel_ready,
            detail: "Hepta must either own Telegram after handoff or run as an approved distinct-token parallel bot",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_cursor_policy_ready",
            ready: telegram_plugin.cursor_plan.duplicate_suppression_ready
                && telegram_plugin.cursor_plan.commit_offset_after_delivery
                && !telegram_plugin.cursor_plan.raw_update_payload_persisted,
            detail: "cursor commits only after delivery or duplicate suppression without raw payload persistence",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_model_runner_plan_ready",
            ready: telegram_model_runner_plan_ready,
            detail: "Telegram replies must have a selected runner plan such as Hepta kernel session runner, local MLX, in-process Hepta exec, or gated child exec",
        },
        NativeGatewayReplacementCheck {
            name: "telegram_hepta_kernel_runner_context_ready",
            ready: !hepta_kernel_model_runner_ready
                || (model_runner_plan.codex_core_runner_enabled
                    && model_runner_plan.hepta_intelligence_context_injected
                    && model_runner_plan.plugin_capability_context_injected),
            detail: "when HEPTA_NATIVE_TELEGRAM_HEPTA_KERNEL_RUNNER is enabled, Telegram model turns must route through the Hepta kernel with Hepta intelligence and plugin/MCP capability context before using Codex as an internal engine; the old CODEX_CORE runner env is compatibility-only",
        },
        NativeGatewayReplacementCheck {
            name: "release_build_verified",
            ready: release_build_verified,
            detail: "release fat-LTO build must pass before replacing the active gateway",
        },
        NativeGatewayReplacementCheck {
            name: "control_ui_route_matrix_ready",
            ready: control_ui_route_parity.ready,
            detail: "hepta-codex must expose the old Hepta Control UI HTTP route matrix",
        },
        NativeGatewayReplacementCheck {
            name: "control_ui_route_parity_verified",
            ready: control_ui_parity_verified,
            detail: "HEPTA_CODEX_CONTROL_UI_PARITY_VERIFIED must be enabled after route parity smoke passes",
        },
        NativeGatewayReplacementCheck {
            name: "readiness_report_side_effect_free",
            ready: side_effect_free,
            detail: "readiness report does not read Telegram, invoke model, send message, or write cursor",
        },
    ];

    let blockers = checks
        .iter()
        .filter(|check| !check.ready)
        .map(|check| check.name)
        .collect::<Vec<_>>();
    let ready = blockers.is_empty();
    let status = if ready { "ready" } else { "blocked" };

    NativeGatewayReplacementReadiness {
        product: "Hepta",
        runtime: "hepta",
        status,
        ready,
        active_install_allowed: ready,
        side_effect_free,
        product_role: hepta_contracts::OPENCLAW_GOVERNED_BACKEND_ROLE,
        product_disposition: hepta_contracts::LEGACY_TELEGRAM_REPLACEMENT_DISPOSITION,
        compatibility_surface: true,
        blocker_count: blockers.len(),
        blockers,
        checks,
        required_env_gates: NativeGatewayReplacementEnvGates {
            delivery_approval: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_DELIVERY_APPROVED_ENV,
                enabled: telegram_gate_summary.delivery_approval_gate_enabled,
            },
            live_read: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_LIVE_READ_ENV,
                enabled: telegram_gate_summary.live_read_gate_enabled,
            },
            model_turn: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_MODEL_TURN_GATE_ENV,
                enabled: telegram_gate_summary.model_turn_gate_enabled,
            },
            send: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_SEND_GATE_ENV,
                enabled: telegram_gate_summary.send_gate_enabled,
            },
            poll_loop: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_POLL_LOOP_ENV,
                enabled: telegram_poll_loop_status.poll_loop_gate_enabled,
            },
            in_process_model_runner: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV,
                enabled: in_process_model_runner_ready,
            },
            hepta_kernel_model_runner: NativeGatewayReplacementGate {
                env: native_telegram::TELEGRAM_HEPTA_KERNEL_RUNNER_ENV,
                enabled: hepta_kernel_model_runner_ready,
            },
            release_build_verified: NativeGatewayReplacementGate {
                env: RELEASE_BUILD_VERIFIED_ENV,
                enabled: release_build_verified,
            },
            control_ui_parity_verified: NativeGatewayReplacementGate {
                env: CONTROL_UI_PARITY_VERIFIED_ENV,
                enabled: control_ui_parity_verified,
            },
        },
        telegram_owner_handoff_endpoint: TELEGRAM_OWNER_HANDOFF_ENDPOINT,
        telegram_owner_handoff_status,
        control_ui_route_parity,
        next_migration_slice: hepta_contracts::LEGACY_TELEGRAM_REPLACEMENT_NEXT_ACTION,
    }
}

fn gateway_live_activation_plan(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> NativeGatewayLiveActivationPlan {
    let readiness = gateway_replacement_readiness(options, telegram_plugin);
    let telegram_gate_summary = native_telegram::telegram_gateway_gate_summary();
    let poll_loop_status = native_telegram::telegram_poll_loop_status(
        options.with_telegram_plugin,
        options.telegram_plugin_poll_ms,
    );
    let status = if readiness.ready && poll_loop_status.loop_invokes_drain_once {
        "active_live_soak_ready"
    } else if readiness.ready {
        "ready_for_operator_live_smoke"
    } else if telegram_gate_summary.delivery_approval_gate_enabled {
        "blocked_after_operator_approval"
    } else {
        "operator_approval_required"
    };

    NativeGatewayLiveActivationPlan {
        product: "Hepta",
        runtime: "hepta",
        status,
        endpoint: GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT,
        operator_approval_required: !telegram_gate_summary.delivery_approval_gate_enabled,
        active_install_allowed: readiness.ready,
        readiness_blocker_count: readiness.blocker_count,
        readiness_blockers: readiness.blockers.clone(),
        active_gateway_label: ACTIVE_GATEWAY_LABEL,
        current_legacy_binary: HEPTA_CODEX_TRANSITION_BINARY,
        replacement_binary: HEPTA_ACTIVE_RELEASE_BINARY,
        bind_addr: options.bind_addr.clone(),
        launch_arguments: vec![
            HEPTA_ACTIVE_RELEASE_BINARY.to_string(),
            "--serve-ui".to_string(),
            options.bind_addr.clone(),
            "--with-telegram-plugin".to_string(),
            "--telegram-plugin-poll-ms".to_string(),
            options.telegram_plugin_poll_ms.to_string(),
        ],
        required_env_gates: vec![
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_DELIVERY_APPROVED_ENV,
                enabled: telegram_gate_summary.delivery_approval_gate_enabled,
                purpose: "explicit operator approval before any live Telegram drain side effects",
            },
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_LIVE_READ_ENV,
                enabled: telegram_gate_summary.live_read_gate_enabled,
                purpose: "allow one-shot getUpdates and poll-loop live reads",
            },
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_MODEL_TURN_GATE_ENV,
                enabled: telegram_gate_summary.model_turn_gate_enabled,
                purpose: "allow model execution for redacted Telegram candidates",
            },
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_SEND_GATE_ENV,
                enabled: telegram_gate_summary.send_gate_enabled,
                purpose: "allow sendMessage only after model success and reply-target validation",
            },
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_POLL_LOOP_ENV,
                enabled: poll_loop_status.poll_loop_gate_enabled,
                purpose: "allow supervised background drain loop from --serve-ui",
            },
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV,
                enabled: env_truthy(native_telegram::TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV),
                purpose: "use library-backed in-process runner instead of child exec fallback",
            },
            NativeGatewayLiveActivationEnv {
                env: native_telegram::TELEGRAM_HEPTA_KERNEL_RUNNER_ENV,
                enabled: native_telegram::telegram_hepta_kernel_runner_enabled(),
                purpose: "force Telegram model turns through Hepta kernel with Hepta intelligence and plugin/MCP capability context before using Codex as an internal engine",
            },
            NativeGatewayLiveActivationEnv {
                env: RELEASE_BUILD_VERIFIED_ENV,
                enabled: env_truthy(RELEASE_BUILD_VERIFIED_ENV),
                purpose: "operator asserts current release binary passed the fat-LTO build gate",
            },
            NativeGatewayLiveActivationEnv {
                env: CONTROL_UI_PARITY_VERIFIED_ENV,
                enabled: env_truthy(CONTROL_UI_PARITY_VERIFIED_ENV),
                purpose: "operator asserts Control UI route parity smoke passed",
            },
        ],
        live_smoke_sequence: &[
            "start isolated hepta-cli release binary on a non-production loopback port",
            "GET /api/gateway-replacement-readiness and require active_install_allowed=false until delivery approval is explicit",
            "GET /api/control-ui-route-parity and require missing_route_count=0",
            "GET /api/telegram-poll-loop and require no status-triggered external read/send",
            "with explicit approval gates only, call /api/telegram-drain-once once and inspect redacted status",
            "allow production replacement only if readiness has no blockers after the smoke",
        ],
        production_replacement_sequence: &[
            "keep the transition hepta-codex binary and launchd label as rollback anchors",
            "install the verified hepta-cli release binary under the first-class Hepta path",
            "switch the active launchd ProgramArguments to hepta --serve-ui loopback with Telegram plugin flags",
            "set only the audited HEPTA_NATIVE_* and HEPTA_CODEX_* gate env vars",
            "kickstart the gateway service and verify /health plus /api/native-gateway",
            "rollback by restoring the old ProgramArguments/binary and kickstarting ai.hepta.gateway",
        ],
        safety: NativeGatewayLiveActivationSafety {
            side_effect_free: true,
            status_probe_reads_telegram: false,
            status_probe_invokes_model: false,
            status_probe_sends_message: false,
            status_probe_writes_cursor: false,
            raw_token_exposed: false,
            raw_update_payload_exposed: false,
            raw_prompt_text_exposed: false,
            raw_response_text_exposed: false,
        },
        next_migration_slice: hepta_contracts::LEGACY_TELEGRAM_REPLACEMENT_NEXT_ACTION,
    }
}

include!("native_gateway/report_types.rs");
include!("native_gateway/inventory_reports.rs");
include!("native_gateway/memory_kg_reports.rs");
include!("native_gateway/activation_memory_reports.rs");
include!("native_gateway/model_invocation_reports.rs");
include!("native_gateway/release_ui_reports.rs");

#[cfg(test)]
mod tests {
    include!("native_gateway/tests.rs");
    include!("native_gateway/socket_tests.rs");
}
