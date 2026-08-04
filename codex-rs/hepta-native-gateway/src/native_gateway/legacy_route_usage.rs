use std::cell::RefCell;
use std::collections::BTreeMap;
use std::env;
use std::ffi::CString;
#[cfg(test)]
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
#[cfg(not(test))]
use std::sync::mpsc::SyncSender;
#[cfg(not(test))]
use std::sync::mpsc::TrySendError;
use std::sync::mpsc::channel;
use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
#[cfg(not(test))]
use std::time::{Duration, Instant};

use anyhow::Context;
use anyhow::Result;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::route_definition::RouteDefinition;
use crate::route_definition::RouteDispatchHandler;

const TELEMETRY_RELATIVE_PATH: &str = "control-ui/legacy-http-events-v3.jsonl";
const TELEMETRY_ENABLED_ENV: &str = "HEPTA_CONTROL_UI_LEGACY_ROUTE_TELEMETRY";
const TELEMETRY_RUN_CLASS_ENV: &str = "HEPTA_CONTROL_UI_LEGACY_ROUTE_RUN_CLASS";
const TELEMETRY_SCHEMA: &str = "hepta_control_ui_legacy_http_event_v3";
const TELEMETRY_HEALTH_SCHEMA: &str = "hepta_legacy_route_telemetry_health_v3";
const TELEMETRY_MAX_BYTES: u64 = 8 * 1024 * 1024;
const TELEMETRY_MAX_EVENT_BYTES: usize = 16 * 1024;
const TELEMETRY_QUEUE_CAPACITY: usize = 256;
const EMPTY_EVENT_CHAIN_SHA256: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

static DIRECT_CALL_COUNTS: OnceLock<Mutex<BTreeMap<&'static str, u64>>> = OnceLock::new();
static TELEMETRY_WRITE_LOCK: Mutex<()> = Mutex::new(());
static EVENT_SEQUENCE: AtomicU64 = AtomicU64::new(0);
static PENDING_EVENT_COUNT: AtomicU64 = AtomicU64::new(0);
static PERSISTED_EVENT_COUNT: AtomicU64 = AtomicU64::new(0);
static DISABLED_EVENT_COUNT: AtomicU64 = AtomicU64::new(0);
static DROPPED_EVENT_COUNT: AtomicU64 = AtomicU64::new(0);
static PERSIST_ERROR_COUNT: AtomicU64 = AtomicU64::new(0);
static INCOMPLETE_OBSERVATION_COUNT: AtomicU64 = AtomicU64::new(0);
static CORRUPT_FILE_COUNT: AtomicU64 = AtomicU64::new(0);
static CAPACITY_REACHED: AtomicBool = AtomicBool::new(false);
#[cfg(not(test))]
static PROCESS_START_RECORDED: AtomicBool = AtomicBool::new(false);
#[cfg(not(test))]
static LAST_HEARTBEAT_DAY: AtomicU64 = AtomicU64::new(u64::MAX);
#[cfg(test)]
static TEST_LIFECYCLE_STATE: OnceLock<Mutex<BTreeMap<PathBuf, (bool, u64)>>> = OnceLock::new();
static ROUTE_CATALOG_SHA: OnceLock<String> = OnceLock::new();
static PROCESS_RUN_IDENTIFIER_SHA: OnceLock<String> = OnceLock::new();
static SOURCE_EPOCH_SHA: OnceLock<String> = OnceLock::new();
static TELEMETRY_EMIT_LOCK: Mutex<()> = Mutex::new(());
static EVENT_CHAIN_CONTEXTS: OnceLock<Mutex<BTreeMap<PathBuf, EventChainContext>>> =
    OnceLock::new();
#[cfg(not(test))]
static TELEMETRY_SENDER: OnceLock<std::result::Result<SyncSender<TelemetryCommand>, String>> =
    OnceLock::new();

thread_local! {
    static PENDING_OBSERVATION: RefCell<Option<PendingObservation>> = const { RefCell::new(None) };
    #[cfg(test)]
    static TEST_STATE_ROOT: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
enum LegacyRouteState {
    #[serde(rename = "legacy_200")]
    Legacy200,
    #[serde(rename = "canonical_only_gone_410")]
    CanonicalOnlyGone410,
    #[serde(rename = "quarantine_preflight_410")]
    QuarantinePreflight410,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum AnonymousConsumerClass {
    Browser,
    JsonClient,
    Unclassified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum PreflightResult {
    Accepted,
    Rejected,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ResponseWriteResult {
    Ok,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum TelemetryEventType {
    ProcessStart,
    ProcessStop,
    Heartbeat,
    LegacyRequest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum PredecessorDisposition {
    Fresh,
    ContinuedGracefulEpoch,
    QuarantinedCrashedSegment,
    RotatedSourceEpoch,
    QuarantinedInvalidSegment,
}

#[derive(Debug, Clone)]
struct EventChainContext {
    previous_event_sha256: String,
    predecessor_segment_sha256: Option<String>,
    predecessor_disposition: PredecessorDisposition,
    next_sequence: u64,
}

struct SecureTelemetryFile {
    file: File,
    parent_dir: File,
}

#[derive(Debug)]
struct PendingObservation {
    route_key: &'static str,
    route_state: LegacyRouteState,
    consumer_class: AnonymousConsumerClass,
    preflight: Option<PreflightResult>,
    http_status: Option<u16>,
    write_result: Option<ResponseWriteResult>,
}

struct TelemetryWrite {
    path: PathBuf,
    encoded: Vec<u8>,
}

enum TelemetryCommand {
    Append {
        write: TelemetryWrite,
        acknowledge: Option<Sender<std::result::Result<(), String>>>,
    },
    Flush(Sender<WriterBarrierReceipt>),
}

#[derive(Debug, Clone)]
struct WriterBarrierReceipt {
    append_error_count: u64,
    last_append_error: Option<String>,
}

#[derive(Debug, Clone, Copy)]
struct ShutdownHealthSnapshot {
    pending_event_count: u64,
    dropped_event_count: u64,
    persist_error_count: u64,
    incomplete_observation_count: u64,
    corrupt_file_count: u64,
    capacity_reached: bool,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct LegacyRouteUsageEvent<'a> {
    schema: &'static str,
    event_type: TelemetryEventType,
    process_run_identifier_sha256: &'a str,
    sequence: u64,
    time_unix_ms: u64,
    process_class: &'static str,
    run_class: &'static str,
    head_sha: &'a str,
    catalog_sha: &'a str,
    source_epoch_sha256: &'a str,
    source_binding_valid: bool,
    catalog_binding_valid: bool,
    previous_event_sha256: String,
    event_body_sha256: String,
    authentication_key_id_sha256: Option<String>,
    event_hmac_sha256: Option<String>,
    predecessor_segment_sha256: Option<String>,
    predecessor_disposition: Option<PredecessorDisposition>,
    route_key: Option<&'a str>,
    route_state: Option<LegacyRouteState>,
    consumer_class: Option<AnonymousConsumerClass>,
    preflight: Option<PreflightResult>,
    http_status: Option<u16>,
    write_result: Option<ResponseWriteResult>,
    observation_complete: bool,
    dropped_event_count: u64,
    persist_error_count: u64,
    incomplete_observation_count: u64,
    capacity_reached: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct LegacyRouteTelemetryHealth {
    schema: &'static str,
    status: &'static str,
    enable_env: &'static str,
    run_class_env: &'static str,
    run_class: &'static str,
    run_class_binding_valid: bool,
    heartbeat_trigger: &'static str,
    unobserved_days_count_as_active: bool,
    enabled: bool,
    writer_healthy: bool,
    process_run_identifier_sha256: &'static str,
    source_head_sha: &'static str,
    source_binding_valid: bool,
    route_catalog_sha256: &'static str,
    catalog_binding_valid: bool,
    source_epoch_sha256: &'static str,
    exact_source_epoch_rotation_ready: bool,
    crashed_segment_quarantine_ready: bool,
    telemetry_parent_directory_fsync_verified: bool,
    event_authentication_plumbing_ready: bool,
    event_signature_or_mac_configured: bool,
    max_file_bytes: u64,
    queue_capacity: usize,
    emitted_sequence_count: u64,
    pending_event_count: u64,
    persisted_event_count: u64,
    disabled_event_count: u64,
    dropped_event_count: u64,
    persist_error_count: u64,
    incomplete_observation_count: u64,
    corrupt_file_count: u64,
    capacity_reached: bool,
    file_contents_fully_validated: bool,
    summary_producer_available: bool,
    observation_window_complete: bool,
    zero_usage_claim_allowed: bool,
    retirement_evidence_ready: bool,
}

pub(super) fn begin_request(request: &str, method: &str, path: &str) {
    record_process_lifecycle();
    let Some(definition) = crate::route_definition::route_definition(method, path)
        .filter(|definition| definition.legacy_compatibility_route && method == "GET")
    else {
        PENDING_OBSERVATION.with(|pending| *pending.borrow_mut() = None);
        return;
    };
    increment_direct_call_count(definition.lifecycle.path_pattern);
    PENDING_OBSERVATION.with(|pending| {
        *pending.borrow_mut() = Some(PendingObservation {
            route_key: definition.lifecycle.path_pattern,
            route_state: route_state(definition),
            consumer_class: anonymous_consumer_class(request),
            preflight: None,
            http_status: None,
            write_result: None,
        });
    });
}

pub(super) fn record_preflight(result: PreflightResult) {
    PENDING_OBSERVATION.with(|pending| {
        if let Some(observation) = pending.borrow_mut().as_mut() {
            observation.preflight = Some(result);
        }
    });
}

pub(super) fn record_response_write(status: &str, result: &Result<()>) {
    PENDING_OBSERVATION.with(|pending| {
        if let Some(observation) = pending.borrow_mut().as_mut() {
            observation.http_status = http_status_code(status);
            observation.write_result = Some(write_result(result));
        }
    });
}

pub(super) fn finish_request(_result: &Result<()>) {
    let observation = PENDING_OBSERVATION.with(|pending| pending.borrow_mut().take());
    let Some(observation) = observation else {
        return;
    };
    let observation_complete = observation.preflight.is_some()
        && observation.http_status.is_some()
        && observation.write_result.is_some();
    if !observation_complete {
        INCOMPLETE_OBSERVATION_COUNT.fetch_add(1, Ordering::Relaxed);
    }
    let path = match telemetry_path() {
        Ok(Some(path)) => path,
        Ok(None) => {
            DISABLED_EVENT_COUNT.fetch_add(1, Ordering::Relaxed);
            return;
        }
        Err(error) => {
            record_persist_error(&error);
            return;
        }
    };
    if let Err(error) = emit_event(
        path,
        TelemetryEventType::LegacyRequest,
        Some(observation.route_key),
        Some(observation.route_state),
        Some(observation.consumer_class),
        observation.preflight,
        observation.http_status,
        observation.write_result,
        observation_complete,
    ) {
        record_persist_error(&error);
    }
}

fn record_process_lifecycle() {
    let path = match telemetry_path() {
        Ok(Some(path)) => path,
        Ok(None) => return,
        Err(error) => {
            record_persist_error(&error);
            return;
        }
    };
    let _guard = TELEMETRY_EMIT_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if !process_start_recorded(&path) {
        if let Err(error) = prepare_event_epoch(&path) {
            record_persist_error(&error);
            return;
        }
        match emit_event_locked(
            path.clone(),
            TelemetryEventType::ProcessStart,
            None,
            None,
            None,
            None,
            None,
            None,
            true,
        ) {
            Ok(()) => mark_process_start_recorded(&path),
            Err(error) => {
                record_persist_error(&error);
                return;
            }
        }
    }
    let day = unix_time_ms() / 86_400_000;
    if last_heartbeat_day(&path) == day {
        return;
    }
    match emit_event_locked(
        path.clone(),
        TelemetryEventType::Heartbeat,
        None,
        None,
        None,
        None,
        None,
        None,
        true,
    ) {
        Ok(()) => mark_heartbeat_day(&path, day),
        Err(error) => record_persist_error(&error),
    }
}

/// Establishes a durable barrier for all prior observations, refuses to emit a
/// terminal marker if any writer/observation health counter is nonzero, then
/// waits for the marker's own append and `sync_data` result before establishing
/// a second barrier. A clean gateway shutdown must call this after the HTTP
/// worker pool drains; abrupt or degraded termination intentionally leaves the
/// segment incomplete and therefore ineligible for route retirement.
pub(super) fn record_process_stop_and_flush() -> Result<()> {
    let Some(path) = telemetry_path()? else {
        return Ok(());
    };
    let _guard = TELEMETRY_EMIT_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if !process_start_recorded(&path) {
        return Ok(());
    }
    let prior = flush_telemetry_writer().context("drain prior telemetry before process stop")?;
    ensure_shutdown_health(prior, "before process stop")?;
    emit_event_locked_with_ack(
        path,
        TelemetryEventType::ProcessStop,
        None,
        None,
        None,
        None,
        None,
        None,
        true,
    )
    .context("durably append process stop")?;
    let terminal = flush_telemetry_writer().context("flush terminal process stop")?;
    ensure_shutdown_health(terminal, "after process stop")
}

fn ensure_shutdown_health(snapshot: ShutdownHealthSnapshot, phase: &str) -> Result<()> {
    anyhow::ensure!(
        snapshot.pending_event_count == 0
            && snapshot.dropped_event_count == 0
            && snapshot.persist_error_count == 0
            && snapshot.incomplete_observation_count == 0
            && snapshot.corrupt_file_count == 0
            && !snapshot.capacity_reached,
        "legacy route telemetry is not shutdown-durable {phase}: pending={} dropped={} persist_errors={} incomplete={} corrupt={} capacity_reached={}",
        snapshot.pending_event_count,
        snapshot.dropped_event_count,
        snapshot.persist_error_count,
        snapshot.incomplete_observation_count,
        snapshot.corrupt_file_count,
        snapshot.capacity_reached,
    );
    Ok(())
}

#[cfg(not(test))]
fn process_start_recorded(_path: &Path) -> bool {
    PROCESS_START_RECORDED.load(Ordering::Acquire)
}

#[cfg(test)]
fn process_start_recorded(path: &Path) -> bool {
    lifecycle_state()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(path)
        .is_some_and(|state| state.0)
}

#[cfg(not(test))]
fn mark_process_start_recorded(_path: &Path) {
    PROCESS_START_RECORDED.store(true, Ordering::Release);
}

#[cfg(test)]
fn mark_process_start_recorded(path: &Path) {
    lifecycle_state()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .entry(path.to_path_buf())
        .or_insert((false, u64::MAX))
        .0 = true;
}

#[cfg(not(test))]
fn last_heartbeat_day(_path: &Path) -> u64 {
    LAST_HEARTBEAT_DAY.load(Ordering::Acquire)
}

#[cfg(test)]
fn last_heartbeat_day(path: &Path) -> u64 {
    lifecycle_state()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(path)
        .map_or(u64::MAX, |state| state.1)
}

#[cfg(not(test))]
fn mark_heartbeat_day(_path: &Path, day: u64) {
    LAST_HEARTBEAT_DAY.store(day, Ordering::Release);
}

#[cfg(test)]
fn mark_heartbeat_day(path: &Path, day: u64) {
    lifecycle_state()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .entry(path.to_path_buf())
        .or_insert((false, u64::MAX))
        .1 = day;
}

#[cfg(test)]
fn lifecycle_state() -> &'static Mutex<BTreeMap<PathBuf, (bool, u64)>> {
    TEST_LIFECYCLE_STATE.get_or_init(|| Mutex::new(BTreeMap::new()))
}

#[allow(clippy::too_many_arguments)]
fn emit_event(
    path: PathBuf,
    event_type: TelemetryEventType,
    route_key: Option<&str>,
    route_state: Option<LegacyRouteState>,
    consumer_class: Option<AnonymousConsumerClass>,
    preflight: Option<PreflightResult>,
    http_status: Option<u16>,
    write_result: Option<ResponseWriteResult>,
    observation_complete: bool,
) -> Result<()> {
    let _guard = TELEMETRY_EMIT_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    emit_event_locked(
        path,
        event_type,
        route_key,
        route_state,
        consumer_class,
        preflight,
        http_status,
        write_result,
        observation_complete,
    )
}

#[allow(clippy::too_many_arguments)]
fn emit_event_locked(
    path: PathBuf,
    event_type: TelemetryEventType,
    route_key: Option<&str>,
    route_state: Option<LegacyRouteState>,
    consumer_class: Option<AnonymousConsumerClass>,
    preflight: Option<PreflightResult>,
    http_status: Option<u16>,
    write_result: Option<ResponseWriteResult>,
    observation_complete: bool,
) -> Result<()> {
    let event = build_event(
        &path,
        event_type,
        route_key,
        route_state,
        consumer_class,
        preflight,
        http_status,
        write_result,
        observation_complete,
    )?;
    // Advance the chain only after the bounded queue accepts this append. A
    // later writer failure deliberately poisons health counters, so a stream
    // with a post-accept persistence gap can never pass trusted replay.
    record_event(path.clone(), &event)?;
    commit_event_chain(&path, &event);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn emit_event_locked_with_ack(
    path: PathBuf,
    event_type: TelemetryEventType,
    route_key: Option<&str>,
    route_state: Option<LegacyRouteState>,
    consumer_class: Option<AnonymousConsumerClass>,
    preflight: Option<PreflightResult>,
    http_status: Option<u16>,
    write_result: Option<ResponseWriteResult>,
    observation_complete: bool,
) -> Result<()> {
    let event = build_event(
        &path,
        event_type,
        route_key,
        route_state,
        consumer_class,
        preflight,
        http_status,
        write_result,
        observation_complete,
    )?;
    record_event_with_ack(path.clone(), &event)?;
    commit_event_chain(&path, &event);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn build_event<'a>(
    path: &Path,
    event_type: TelemetryEventType,
    route_key: Option<&'a str>,
    route_state: Option<LegacyRouteState>,
    consumer_class: Option<AnonymousConsumerClass>,
    preflight: Option<PreflightResult>,
    http_status: Option<u16>,
    write_result: Option<ResponseWriteResult>,
    observation_complete: bool,
) -> Result<LegacyRouteUsageEvent<'a>> {
    let head_sha = hepta_core::production_surface_report().source_git_head;
    let catalog_sha = route_catalog_sha();
    let source_epoch_sha256 = source_epoch_sha();
    let (sequence, previous_event_sha256, predecessor_segment_sha256, predecessor_disposition) = {
        let mut contexts = event_chain_contexts()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let context = contexts
            .entry(path.to_path_buf())
            .or_insert_with(fresh_event_chain_context);
        (
            context.next_sequence,
            context.previous_event_sha256.clone(),
            (event_type == TelemetryEventType::ProcessStart)
                .then(|| context.predecessor_segment_sha256.clone())
                .flatten(),
            (event_type == TelemetryEventType::ProcessStart)
                .then_some(context.predecessor_disposition),
        )
    };
    let mut event = LegacyRouteUsageEvent {
        schema: TELEMETRY_SCHEMA,
        event_type,
        process_run_identifier_sha256: process_run_identifier_sha(),
        sequence,
        time_unix_ms: unix_time_ms(),
        process_class: "hepta_native_gateway",
        run_class: process_run_class()?,
        head_sha,
        catalog_sha,
        source_epoch_sha256,
        source_binding_valid: valid_source_head(head_sha),
        catalog_binding_valid: valid_sha256(catalog_sha),
        previous_event_sha256,
        event_body_sha256: String::new(),
        authentication_key_id_sha256: None,
        event_hmac_sha256: None,
        predecessor_segment_sha256,
        predecessor_disposition,
        route_key,
        route_state,
        consumer_class,
        preflight,
        http_status,
        write_result,
        observation_complete,
        dropped_event_count: DROPPED_EVENT_COUNT.load(Ordering::Relaxed),
        persist_error_count: PERSIST_ERROR_COUNT.load(Ordering::Relaxed),
        incomplete_observation_count: INCOMPLETE_OBSERVATION_COUNT.load(Ordering::Relaxed),
        capacity_reached: CAPACITY_REACHED.load(Ordering::Relaxed),
    };
    event.event_body_sha256 = event_body_sha256(&event)?;
    Ok(event)
}

fn commit_event_chain(path: &Path, event: &LegacyRouteUsageEvent<'_>) {
    let mut contexts = event_chain_contexts()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let context = contexts
        .entry(path.to_path_buf())
        .or_insert_with(fresh_event_chain_context);
    context.next_sequence = context.next_sequence.saturating_add(1);
    context
        .previous_event_sha256
        .clone_from(&event.event_body_sha256);
    EVENT_SEQUENCE.fetch_add(1, Ordering::Relaxed);
}

fn process_run_class_value() -> &'static str {
    // Ordinary process environment is not a trusted deployment identity. Until
    // a sealed launcher proof is independently configured and verified, every
    // producer run is non-promotable test/observer traffic regardless of env.
    "ci_test"
}

fn process_run_class() -> Result<&'static str> {
    let value = process_run_class_value();
    anyhow::ensure!(
        valid_run_class(value),
        "legacy route telemetry run class must be operator or ci_test"
    );
    Ok(value)
}

fn valid_run_class(value: &str) -> bool {
    matches!(value, "operator" | "ci_test")
}

pub(super) fn telemetry_health() -> LegacyRouteTelemetryHealth {
    let enabled = telemetry_enabled();
    let source_head_sha = hepta_core::production_surface_report().source_git_head;
    let source_binding_valid = valid_source_head(source_head_sha);
    let route_catalog_sha256 = route_catalog_sha();
    let catalog_binding_valid = valid_sha256(route_catalog_sha256);
    let disabled_event_count = DISABLED_EVENT_COUNT.load(Ordering::Relaxed);
    let dropped_event_count = DROPPED_EVENT_COUNT.load(Ordering::Relaxed);
    let persist_error_count = PERSIST_ERROR_COUNT.load(Ordering::Relaxed);
    let incomplete_observation_count = INCOMPLETE_OBSERVATION_COUNT.load(Ordering::Relaxed);
    let corrupt_file_count = CORRUPT_FILE_COUNT.load(Ordering::Relaxed);
    let capacity_reached = CAPACITY_REACHED.load(Ordering::Relaxed);
    let run_class = process_run_class_value();
    let run_class_binding_valid = valid_run_class(run_class);
    let writer_healthy = enabled
        && source_binding_valid
        && catalog_binding_valid
        && run_class_binding_valid
        && disabled_event_count == 0
        && dropped_event_count == 0
        && persist_error_count == 0
        && incomplete_observation_count == 0
        && corrupt_file_count == 0
        && !capacity_reached;
    LegacyRouteTelemetryHealth {
        schema: TELEMETRY_HEALTH_SCHEMA,
        status: if !enabled {
            "disabled"
        } else if writer_healthy {
            "writer_observing"
        } else {
            "writer_degraded"
        },
        enable_env: TELEMETRY_ENABLED_ENV,
        run_class_env: TELEMETRY_RUN_CLASS_ENV,
        run_class,
        run_class_binding_valid,
        heartbeat_trigger: "first_parsed_supported_http_request_each_utc_day",
        unobserved_days_count_as_active: false,
        enabled,
        writer_healthy,
        process_run_identifier_sha256: process_run_identifier_sha(),
        source_head_sha,
        source_binding_valid,
        route_catalog_sha256,
        catalog_binding_valid,
        source_epoch_sha256: source_epoch_sha(),
        exact_source_epoch_rotation_ready: true,
        crashed_segment_quarantine_ready: true,
        telemetry_parent_directory_fsync_verified: true,
        event_authentication_plumbing_ready: true,
        event_signature_or_mac_configured: false,
        max_file_bytes: TELEMETRY_MAX_BYTES,
        queue_capacity: TELEMETRY_QUEUE_CAPACITY,
        emitted_sequence_count: EVENT_SEQUENCE.load(Ordering::Relaxed),
        pending_event_count: PENDING_EVENT_COUNT.load(Ordering::Relaxed),
        persisted_event_count: PERSISTED_EVENT_COUNT.load(Ordering::Relaxed),
        disabled_event_count,
        dropped_event_count,
        persist_error_count,
        incomplete_observation_count,
        corrupt_file_count,
        capacity_reached,
        // The append writer only validates the existing tail. A separate bounded reader must
        // validate every record before this stream can support a retirement decision.
        file_contents_fully_validated: false,
        summary_producer_available: true,
        // A process-local append stream is not, by itself, a complete observation window.
        observation_window_complete: false,
        zero_usage_claim_allowed: false,
        retirement_evidence_ready: false,
    }
}

pub(super) fn record_direct_call(definition: RouteDefinition) {
    if !definition.legacy_compatibility_route {
        return;
    }
    let already_recorded = PENDING_OBSERVATION.with(|pending| {
        pending
            .borrow()
            .as_ref()
            .is_some_and(|observation| observation.route_key == definition.lifecycle.path_pattern)
    });
    if !already_recorded {
        increment_direct_call_count(definition.lifecycle.path_pattern);
    }
}

pub(super) fn direct_call_count(path: &str) -> u64 {
    direct_call_counts()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(path)
        .copied()
        .unwrap_or_default()
}

pub(super) fn total_direct_call_count() -> u64 {
    direct_call_counts()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .values()
        .copied()
        .fold(0_u64, u64::saturating_add)
}

fn route_state(definition: RouteDefinition) -> LegacyRouteState {
    if definition.lifecycle.source == "control_ui_transitive_effect_quarantine" {
        LegacyRouteState::QuarantinePreflight410
    } else if definition.dispatch_handler == RouteDispatchHandler::RetiredCompatibility {
        LegacyRouteState::CanonicalOnlyGone410
    } else {
        LegacyRouteState::Legacy200
    }
}

fn anonymous_consumer_class(request: &str) -> AnonymousConsumerClass {
    let mut json_client = false;
    for line in request.lines().skip(1).take_while(|line| !line.is_empty()) {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if name.eq_ignore_ascii_case("sec-fetch-mode") {
            return AnonymousConsumerClass::Browser;
        }
        if name.eq_ignore_ascii_case("accept")
            && value
                .split(',')
                .any(|value| value.trim().starts_with("application/json"))
        {
            json_client = true;
        }
    }
    if json_client {
        AnonymousConsumerClass::JsonClient
    } else {
        AnonymousConsumerClass::Unclassified
    }
}

fn http_status_code(status: &str) -> Option<u16> {
    status.split_whitespace().next()?.parse().ok()
}

fn write_result(result: &Result<()>) -> ResponseWriteResult {
    if result.is_ok() {
        ResponseWriteResult::Ok
    } else {
        ResponseWriteResult::Error
    }
}

fn route_catalog_sha() -> &'static str {
    ROUTE_CATALOG_SHA
        .get_or_init(|| {
            format!(
                "{:x}",
                Sha256::digest(include_bytes!(
                    "../../routes/control_ui_route_catalog_v1.jsonl"
                ))
            )
        })
        .as_str()
}

fn source_epoch_sha() -> &'static str {
    SOURCE_EPOCH_SHA
        .get_or_init(|| {
            crate::legacy_route_window::canonical_source_epoch_sha256(
                hepta_core::production_surface_report().source_git_head,
                route_catalog_sha(),
            )
        })
        .as_str()
}

fn event_body_sha256(event: &LegacyRouteUsageEvent<'_>) -> Result<String> {
    crate::legacy_route_window::canonical_event_body_sha256(event)
}

fn fresh_event_chain_context() -> EventChainContext {
    EventChainContext {
        previous_event_sha256: EMPTY_EVENT_CHAIN_SHA256.to_string(),
        predecessor_segment_sha256: None,
        predecessor_disposition: PredecessorDisposition::Fresh,
        next_sequence: 1,
    }
}

fn event_chain_contexts() -> &'static Mutex<BTreeMap<PathBuf, EventChainContext>> {
    EVENT_CHAIN_CONTEXTS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn prepare_event_epoch(path: &Path) -> Result<()> {
    let _write_guard = TELEMETRY_WRITE_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut secure = open_secure_telemetry_file(path)?;
    lock_telemetry_file(&secure.file)?;
    validate_telemetry_file(&secure.file)?;
    let length = secure
        .file
        .metadata()
        .context("stat telemetry event epoch")?
        .len();
    if length == 0 {
        secure
            .parent_dir
            .sync_all()
            .context("sync fresh telemetry parent directory")?;
        event_chain_contexts()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(path.to_path_buf(), fresh_event_chain_context());
        return Ok(());
    }

    let mut bytes = Vec::with_capacity(length.min(TELEMETRY_MAX_BYTES) as usize);
    secure
        .file
        .seek(SeekFrom::Start(0))
        .context("rewind telemetry event epoch")?;
    Read::by_ref(&mut secure.file)
        .take(TELEMETRY_MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .context("read telemetry event epoch")?;
    let file_sha256 = format!("{:x}", Sha256::digest(&bytes));
    let expected_epoch = source_epoch_sha();
    let first_epoch = first_event_source_epoch(&bytes);
    let inspection = crate::legacy_route_window::inspect_append_stream(
        &bytes,
        hepta_core::production_surface_report().source_git_head,
        unix_time_ms(),
    );
    let disposition = match &inspection {
        Ok(inspection) if inspection.process_stop_marker_observed => {
            event_chain_contexts()
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .insert(
                    path.to_path_buf(),
                    EventChainContext {
                        previous_event_sha256: inspection.last_event_body_sha256.clone(),
                        predecessor_segment_sha256: Some(file_sha256),
                        predecessor_disposition: PredecessorDisposition::ContinuedGracefulEpoch,
                        next_sequence: 1,
                    },
                );
            return Ok(());
        }
        Ok(_) => PredecessorDisposition::QuarantinedCrashedSegment,
        Err(_)
            if first_epoch
                .as_deref()
                .is_some_and(|epoch| epoch != expected_epoch) =>
        {
            PredecessorDisposition::RotatedSourceEpoch
        }
        Err(_) => PredecessorDisposition::QuarantinedInvalidSegment,
    };
    quarantine_locked_telemetry_file(path, &secure, disposition, &file_sha256)?;
    event_chain_contexts()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .insert(
            path.to_path_buf(),
            EventChainContext {
                previous_event_sha256: EMPTY_EVENT_CHAIN_SHA256.to_string(),
                predecessor_segment_sha256: Some(file_sha256),
                predecessor_disposition: disposition,
                next_sequence: 1,
            },
        );
    Ok(())
}

fn first_event_source_epoch(bytes: &[u8]) -> Option<String> {
    let first = bytes
        .split(|byte| *byte == b'\n')
        .find(|line| !line.is_empty())?;
    serde_json::from_slice::<serde_json::Value>(first)
        .ok()?
        .get("source_epoch_sha256")?
        .as_str()
        .map(str::to_owned)
}

fn process_run_identifier_sha() -> &'static str {
    PROCESS_RUN_IDENTIFIER_SHA
        .get_or_init(|| {
            let mut hasher = Sha256::new();
            hasher.update(b"hepta-native-gateway-legacy-route-run-v1\0");
            hasher.update(process::id().to_be_bytes());
            hasher.update(unix_time_ms().to_be_bytes());
            hasher.update(
                hepta_core::production_surface_report()
                    .source_git_head
                    .as_bytes(),
            );
            format!("{:x}", hasher.finalize())
        })
        .as_str()
}

fn valid_source_head(value: &str) -> bool {
    value != "unknown"
        && (7..=64).contains(&value.len())
        && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn telemetry_enabled() -> bool {
    #[cfg(test)]
    if TEST_STATE_ROOT.with(|root| root.borrow().is_some()) {
        return true;
    }
    env::var(TELEMETRY_ENABLED_ENV).as_deref() == Ok("1")
}

fn telemetry_path() -> Result<Option<PathBuf>> {
    #[cfg(test)]
    if let Some(root) = TEST_STATE_ROOT.with(|root| root.borrow().clone()) {
        return Ok(Some(
            hepta_paths::HeptaStateRoot::parse(root)
                .context("parse test typed state root")?
                .join(TELEMETRY_RELATIVE_PATH)
                .context("resolve legacy route telemetry path")?,
        ));
    } else {
        return Ok(None);
    }

    #[cfg(not(test))]
    {
        if !telemetry_enabled() {
            return Ok(None);
        }
        Ok(Some(
            hepta_paths::HeptaStateRoot::discover()
                .context("discover typed Hepta state root")?
                .join(TELEMETRY_RELATIVE_PATH)
                .context("resolve legacy route telemetry path")?,
        ))
    }
}

#[cfg(test)]
fn persist_event(path: &Path, event: &LegacyRouteUsageEvent<'_>) -> Result<()> {
    persist_event_with_limit(path, event, TELEMETRY_MAX_BYTES)
}

#[cfg(test)]
fn record_event(path: PathBuf, event: &LegacyRouteUsageEvent<'_>) -> Result<()> {
    persist_event(&path, event)?;
    PERSISTED_EVENT_COUNT.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

#[cfg(test)]
fn record_event_with_ack(path: PathBuf, event: &LegacyRouteUsageEvent<'_>) -> Result<()> {
    record_event(path, event)
}

#[cfg(not(test))]
fn record_event(path: PathBuf, event: &LegacyRouteUsageEvent<'_>) -> Result<()> {
    let encoded = encode_event(event)?;
    let sender = TELEMETRY_SENDER
        .get_or_init(start_telemetry_writer)
        .as_ref()
        .map_err(|error| anyhow::anyhow!(error.clone()))?;
    PENDING_EVENT_COUNT.fetch_add(1, Ordering::Relaxed);
    match sender.try_send(TelemetryCommand::Append {
        write: TelemetryWrite { path, encoded },
        acknowledge: None,
    }) {
        Ok(()) => Ok(()),
        Err(TrySendError::Full(_)) => {
            PENDING_EVENT_COUNT.fetch_sub(1, Ordering::Relaxed);
            anyhow::bail!("legacy route telemetry writer queue is full")
        }
        Err(TrySendError::Disconnected(_)) => {
            PENDING_EVENT_COUNT.fetch_sub(1, Ordering::Relaxed);
            anyhow::bail!("legacy route telemetry writer queue is disconnected")
        }
    }
}

#[cfg(not(test))]
fn record_event_with_ack(path: PathBuf, event: &LegacyRouteUsageEvent<'_>) -> Result<()> {
    const APPEND_TIMEOUT: Duration = Duration::from_secs(5);

    let encoded = encode_event(event)?;
    let sender = TELEMETRY_SENDER
        .get_or_init(start_telemetry_writer)
        .as_ref()
        .map_err(|error| anyhow::anyhow!(error.clone()))?;
    let (acknowledge, completed) = channel();
    PENDING_EVENT_COUNT.fetch_add(1, Ordering::Relaxed);
    let command = TelemetryCommand::Append {
        write: TelemetryWrite { path, encoded },
        acknowledge: Some(acknowledge),
    };
    if let Err(error) = enqueue_writer_command(sender, command, APPEND_TIMEOUT, "append") {
        PENDING_EVENT_COUNT.fetch_sub(1, Ordering::Relaxed);
        return Err(error);
    }
    completed
        .recv_timeout(APPEND_TIMEOUT)
        .context("legacy route telemetry append acknowledgement timed out")?
        .map_err(anyhow::Error::msg)
}

#[cfg(not(test))]
fn start_telemetry_writer() -> std::result::Result<SyncSender<TelemetryCommand>, String> {
    let (sender, receiver) = sync_channel::<TelemetryCommand>(TELEMETRY_QUEUE_CAPACITY);
    thread::Builder::new()
        .name("hepta-legacy-route-telemetry".to_string())
        .spawn(move || {
            run_telemetry_writer(
                receiver,
                |write| {
                    persist_encoded_with_limit(&write.path, &write.encoded, TELEMETRY_MAX_BYTES)
                },
                true,
            );
        })
        .map_err(|error| format!("spawn legacy route telemetry writer: {error}"))?;
    Ok(sender)
}

fn run_telemetry_writer(
    receiver: std::sync::mpsc::Receiver<TelemetryCommand>,
    mut persist: impl FnMut(&TelemetryWrite) -> Result<()>,
    update_global_counters: bool,
) {
    let mut barrier = WriterBarrierReceipt {
        append_error_count: 0,
        last_append_error: None,
    };
    for command in receiver {
        match command {
            TelemetryCommand::Append { write, acknowledge } => {
                let result = persist(&write).map_err(|error| format!("{error:#}"));
                if update_global_counters {
                    PENDING_EVENT_COUNT.fetch_sub(1, Ordering::Relaxed);
                    match &result {
                        Ok(()) => {
                            PERSISTED_EVENT_COUNT.fetch_add(1, Ordering::Relaxed);
                        }
                        Err(message) => record_persist_error(&anyhow::anyhow!(message.clone())),
                    }
                }
                if let Err(message) = &result {
                    barrier.append_error_count = barrier.append_error_count.saturating_add(1);
                    barrier.last_append_error = Some(message.clone());
                }
                if let Some(acknowledge) = acknowledge {
                    let _ = acknowledge.send(result);
                }
            }
            TelemetryCommand::Flush(acknowledge) => {
                let _ = acknowledge.send(barrier.clone());
            }
        }
    }
}

#[cfg(not(test))]
fn enqueue_writer_command(
    sender: &SyncSender<TelemetryCommand>,
    mut command: TelemetryCommand,
    timeout: Duration,
    label: &str,
) -> Result<()> {
    let deadline = Instant::now() + timeout;
    loop {
        match sender.try_send(command) {
            Ok(()) => return Ok(()),
            Err(TrySendError::Full(returned)) if Instant::now() < deadline => {
                command = returned;
                thread::sleep(Duration::from_millis(5));
            }
            Err(TrySendError::Full(_)) => {
                anyhow::bail!("legacy route telemetry {label} enqueue timed out")
            }
            Err(TrySendError::Disconnected(_)) => {
                anyhow::bail!("legacy route telemetry writer disconnected before {label}")
            }
        }
    }
}

#[cfg(not(test))]
fn flush_telemetry_writer() -> Result<ShutdownHealthSnapshot> {
    const FLUSH_TIMEOUT: Duration = Duration::from_secs(5);

    let sender = TELEMETRY_SENDER
        .get_or_init(start_telemetry_writer)
        .as_ref()
        .map_err(|error| anyhow::anyhow!(error.clone()))?;
    let (acknowledge, completed) = channel();
    enqueue_writer_command(
        sender,
        TelemetryCommand::Flush(acknowledge),
        FLUSH_TIMEOUT,
        "flush",
    )?;
    let barrier = completed
        .recv_timeout(FLUSH_TIMEOUT)
        .context("legacy route telemetry flush acknowledgement timed out")?;
    anyhow::ensure!(
        barrier.append_error_count == 0,
        "legacy route telemetry barrier observed {} append errors; last error: {}",
        barrier.append_error_count,
        barrier
            .last_append_error
            .as_deref()
            .unwrap_or("unavailable"),
    );
    anyhow::ensure!(
        PENDING_EVENT_COUNT.load(Ordering::Acquire) == 0,
        "legacy route telemetry flush left pending events"
    );
    Ok(shutdown_health_snapshot())
}

#[cfg(test)]
fn flush_telemetry_writer() -> Result<ShutdownHealthSnapshot> {
    Ok(shutdown_health_snapshot())
}

#[cfg(not(test))]
fn shutdown_health_snapshot() -> ShutdownHealthSnapshot {
    ShutdownHealthSnapshot {
        pending_event_count: PENDING_EVENT_COUNT.load(Ordering::Acquire),
        dropped_event_count: DROPPED_EVENT_COUNT.load(Ordering::Acquire),
        persist_error_count: PERSIST_ERROR_COUNT.load(Ordering::Acquire),
        incomplete_observation_count: INCOMPLETE_OBSERVATION_COUNT.load(Ordering::Acquire),
        corrupt_file_count: CORRUPT_FILE_COUNT.load(Ordering::Acquire),
        capacity_reached: CAPACITY_REACHED.load(Ordering::Acquire),
    }
}

#[cfg(test)]
fn shutdown_health_snapshot() -> ShutdownHealthSnapshot {
    ShutdownHealthSnapshot {
        pending_event_count: 0,
        dropped_event_count: 0,
        persist_error_count: 0,
        incomplete_observation_count: 0,
        corrupt_file_count: 0,
        capacity_reached: false,
    }
}

#[cfg(test)]
fn persist_event_with_limit(
    path: &Path,
    event: &LegacyRouteUsageEvent<'_>,
    max_bytes: u64,
) -> Result<()> {
    let encoded = encode_event(event)?;
    persist_encoded_with_limit(path, &encoded, max_bytes)
}

fn encode_event(event: &LegacyRouteUsageEvent<'_>) -> Result<Vec<u8>> {
    let mut encoded =
        serde_json::to_vec(event).context("serialize legacy route telemetry event")?;
    encoded.push(b'\n');
    anyhow::ensure!(
        encoded.len() <= TELEMETRY_MAX_EVENT_BYTES,
        "legacy route telemetry event exceeds bounded event size"
    );
    Ok(encoded)
}

fn persist_encoded_with_limit(path: &Path, encoded: &[u8], max_bytes: u64) -> Result<()> {
    let _write_guard = TELEMETRY_WRITE_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    anyhow::ensure!(
        encoded.len() <= TELEMETRY_MAX_EVENT_BYTES,
        "legacy route telemetry event exceeds bounded event size"
    );
    let mut secure = open_secure_telemetry_file(path)?;
    lock_telemetry_file(&secure.file)?;
    validate_telemetry_file(&secure.file)?;
    let current_bytes = secure
        .file
        .metadata()
        .context("read legacy route telemetry metadata")?
        .len();
    let next_bytes = current_bytes
        .checked_add(encoded.len() as u64)
        .context("legacy route telemetry byte count overflow")?;
    if next_bytes > max_bytes {
        CAPACITY_REACHED.store(true, Ordering::Relaxed);
        anyhow::bail!("legacy route telemetry capacity reached: {next_bytes} > {max_bytes} bytes");
    }
    secure
        .file
        .write_all(encoded)
        .context("append bounded legacy route telemetry event")?;
    secure
        .file
        .sync_data()
        .context("sync legacy route telemetry event")?;
    secure
        .parent_dir
        .sync_all()
        .context("sync legacy route telemetry parent directory")?;
    Ok(())
}

#[cfg(unix)]
fn open_secure_telemetry_file(path: &Path) -> Result<SecureTelemetryFile> {
    use std::os::fd::AsRawFd;
    use std::os::fd::FromRawFd;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::OpenOptionsExt;

    let parent = path
        .parent()
        .context("legacy route telemetry has no parent")?;
    let root = parent
        .parent()
        .context("legacy route telemetry has no state root")?;
    let parent_name = CString::new(
        parent
            .file_name()
            .context("legacy route telemetry parent has no name")?
            .as_bytes(),
    )
    .context("legacy route telemetry parent contains NUL")?;
    let file_name = CString::new(
        path.file_name()
            .context("legacy route telemetry file has no name")?
            .as_bytes(),
    )
    .context("legacy route telemetry file contains NUL")?;

    let mut root_options = OpenOptions::new();
    root_options
        .read(true)
        .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC);
    let root_dir = root_options
        .open(root)
        .with_context(|| format!("open trusted telemetry state root {}", root.display()))?;
    let mkdir_result = unsafe { libc::mkdirat(root_dir.as_raw_fd(), parent_name.as_ptr(), 0o700) };
    if mkdir_result != 0 {
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() != Some(libc::EEXIST) {
            return Err(error).with_context(|| {
                format!(
                    "create legacy route telemetry directory {}",
                    parent.display()
                )
            });
        }
    } else {
        root_dir
            .sync_all()
            .context("sync telemetry state root after directory creation")?;
    }
    let parent_fd = unsafe {
        libc::openat(
            root_dir.as_raw_fd(),
            parent_name.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if parent_fd < 0 {
        return Err(std::io::Error::last_os_error()).with_context(|| {
            format!(
                "open non-symlink legacy route telemetry directory {}",
                parent.display()
            )
        });
    }
    let parent_dir = unsafe { File::from_raw_fd(parent_fd) };
    validate_private_directory(&parent_dir, parent)?;

    let file_fd = unsafe {
        libc::openat(
            parent_dir.as_raw_fd(),
            file_name.as_ptr(),
            libc::O_RDWR | libc::O_APPEND | libc::O_CREAT | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            0o600,
        )
    };
    if file_fd < 0 {
        return Err(std::io::Error::last_os_error())
            .with_context(|| format!("open non-symlink telemetry file {}", path.display()));
    }
    parent_dir
        .sync_all()
        .context("sync telemetry parent after file open/create")?;
    Ok(SecureTelemetryFile {
        file: unsafe { File::from_raw_fd(file_fd) },
        parent_dir,
    })
}

#[cfg(not(unix))]
fn open_secure_telemetry_file(_path: &Path) -> Result<SecureTelemetryFile> {
    anyhow::bail!("legacy route telemetry persistence requires secure Unix openat semantics")
}

#[cfg(unix)]
fn quarantine_locked_telemetry_file(
    path: &Path,
    secure: &SecureTelemetryFile,
    disposition: PredecessorDisposition,
    file_sha256: &str,
) -> Result<()> {
    use std::os::fd::AsRawFd;
    use std::os::unix::ffi::OsStrExt;

    let file_name = path
        .file_name()
        .context("telemetry quarantine source has no file name")?;
    let source_name =
        CString::new(file_name.as_bytes()).context("telemetry quarantine source contains NUL")?;
    let reason = match disposition {
        PredecessorDisposition::QuarantinedCrashedSegment => "crashed",
        PredecessorDisposition::RotatedSourceEpoch => "source-epoch",
        PredecessorDisposition::QuarantinedInvalidSegment => "invalid",
        PredecessorDisposition::Fresh | PredecessorDisposition::ContinuedGracefulEpoch => {
            anyhow::bail!("invalid telemetry quarantine disposition")
        }
    };
    let quarantine_name = format!(
        "{}.quarantine-{reason}-{}-{}-{}.jsonl",
        file_name.to_string_lossy(),
        &file_sha256[..16],
        process::id(),
        unix_time_ms(),
    );
    let quarantine_name =
        CString::new(quarantine_name).context("telemetry quarantine name contains NUL")?;
    // Move the single-link source in one namespace operation. A link+unlink
    // sequence has a crash window where both names are durable; the next
    // startup then rejects the source forever because its link count is two.
    // renameat leaves either the source name or the quarantine name after a
    // crash, never a deliberately-created hard-link alias.
    let rename_result = unsafe {
        libc::renameat(
            secure.parent_dir.as_raw_fd(),
            source_name.as_ptr(),
            secure.parent_dir.as_raw_fd(),
            quarantine_name.as_ptr(),
        )
    };
    if rename_result != 0 {
        return Err(std::io::Error::last_os_error())
            .context("atomically rename telemetry segment into quarantine");
    }
    secure
        .parent_dir
        .sync_all()
        .context("sync atomic telemetry quarantine rename")?;
    Ok(())
}

#[cfg(not(unix))]
fn quarantine_locked_telemetry_file(
    _path: &Path,
    _secure: &SecureTelemetryFile,
    _disposition: PredecessorDisposition,
    _file_sha256: &str,
) -> Result<()> {
    anyhow::bail!("telemetry segment quarantine requires Unix renameat semantics")
}

#[cfg(unix)]
fn validate_private_directory(directory: &File, path: &Path) -> Result<()> {
    use std::os::unix::fs::MetadataExt;

    let metadata = directory
        .metadata()
        .with_context(|| format!("stat telemetry directory {}", path.display()))?;
    anyhow::ensure!(metadata.is_dir(), "telemetry parent is not a directory");
    anyhow::ensure!(
        metadata.uid() == unsafe { libc::geteuid() },
        "telemetry parent is not owned by the current user"
    );
    anyhow::ensure!(
        metadata.mode() & 0o777 == 0o700,
        "telemetry parent permissions must be 0700"
    );
    Ok(())
}

#[cfg(unix)]
fn lock_telemetry_file(file: &File) -> Result<()> {
    use std::os::fd::AsRawFd;

    let result = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
            .context("acquire nonblocking cross-process telemetry writer lock")
    }
}

#[cfg(not(unix))]
fn lock_telemetry_file(_file: &File) -> Result<()> {
    anyhow::bail!("legacy route telemetry locking requires Unix flock semantics")
}

#[cfg(unix)]
fn validate_telemetry_file(file: &File) -> Result<()> {
    use std::os::unix::fs::FileExt;
    use std::os::unix::fs::MetadataExt;

    let metadata = file
        .metadata()
        .context("stat legacy route telemetry file")?;
    anyhow::ensure!(metadata.is_file(), "telemetry target is not a regular file");
    anyhow::ensure!(
        metadata.uid() == unsafe { libc::geteuid() },
        "telemetry file is not owned by the current user"
    );
    anyhow::ensure!(
        metadata.mode() & 0o777 == 0o600,
        "telemetry file permissions must be 0600"
    );
    anyhow::ensure!(
        metadata.nlink() == 1,
        "telemetry file must not have hard-link aliases"
    );
    if metadata.len() > 0 {
        let mut tail = [0_u8; 1];
        let read = file
            .read_at(&mut tail, metadata.len() - 1)
            .context("read legacy route telemetry tail")?;
        if read != 1 || tail[0] != b'\n' {
            CORRUPT_FILE_COUNT.fetch_add(1, Ordering::Relaxed);
            anyhow::bail!("legacy route telemetry ends with an incomplete JSONL record");
        }
    }
    Ok(())
}

#[cfg(not(unix))]
fn validate_telemetry_file(_file: &File) -> Result<()> {
    anyhow::bail!("legacy route telemetry validation requires Unix file metadata")
}

fn record_persist_error(error: &anyhow::Error) {
    let persist_error_count = PERSIST_ERROR_COUNT
        .fetch_add(1, Ordering::Relaxed)
        .saturating_add(1);
    let dropped_event_count = DROPPED_EVENT_COUNT
        .fetch_add(1, Ordering::Relaxed)
        .saturating_add(1);
    // A saturated telemetry queue or a persistently unwritable state root must not turn an
    // otherwise bounded observer into an unbounded stderr/disk amplifier. Keep the first error
    // and exponentially spaced updates while the counters retain the complete failure total.
    if persist_error_count.is_power_of_two() {
        eprintln!(
            "legacy route telemetry persist failed; response unchanged; persist_error_count={persist_error_count} dropped_event_count={dropped_event_count}: {error:#}"
        );
    }
}

fn unix_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
        .unwrap_or_default()
}

fn increment_direct_call_count(path: &'static str) {
    let mut counts = direct_call_counts()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let count = counts.entry(path).or_default();
    *count = count.saturating_add(1);
}

fn direct_call_counts() -> &'static Mutex<BTreeMap<&'static str, u64>> {
    DIRECT_CALL_COUNTS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

#[cfg(test)]
pub(super) fn with_test_state_root<T>(root: &Path, callback: impl FnOnce() -> T) -> T {
    TEST_STATE_ROOT.with(|state_root| {
        let previous = state_root.replace(Some(root.to_path_buf()));
        let result = callback();
        state_root.replace(previous);
        result
    })
}

#[cfg(test)]
pub(super) fn telemetry_relative_path() -> &'static str {
    TELEMETRY_RELATIVE_PATH
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_event() -> LegacyRouteUsageEvent<'static> {
        let mut event = LegacyRouteUsageEvent {
            schema: TELEMETRY_SCHEMA,
            event_type: TelemetryEventType::LegacyRequest,
            process_run_identifier_sha256: "1111111111111111111111111111111111111111111111111111111111111111",
            sequence: 7,
            time_unix_ms: 1_722_600_000_000,
            process_class: "hepta_native_gateway",
            run_class: "ci_test",
            head_sha: "0e52c78003b6",
            catalog_sha: "2222222222222222222222222222222222222222222222222222222222222222",
            source_epoch_sha256: "3333333333333333333333333333333333333333333333333333333333333333",
            source_binding_valid: true,
            catalog_binding_valid: true,
            previous_event_sha256: EMPTY_EVENT_CHAIN_SHA256.to_string(),
            event_body_sha256: String::new(),
            authentication_key_id_sha256: None,
            event_hmac_sha256: None,
            predecessor_segment_sha256: None,
            predecessor_disposition: None,
            route_key: Some("/api/example/<anonymous>"),
            route_state: Some(LegacyRouteState::Legacy200),
            consumer_class: Some(AnonymousConsumerClass::Browser),
            preflight: Some(PreflightResult::Accepted),
            http_status: Some(200),
            write_result: Some(ResponseWriteResult::Ok),
            observation_complete: true,
            dropped_event_count: 0,
            persist_error_count: 0,
            incomplete_observation_count: 0,
            capacity_reached: false,
        };
        event.event_body_sha256 = event_body_sha256(&event).expect("event body digest");
        event
    }

    fn test_lifecycle_event(path: &Path) -> LegacyRouteUsageEvent<'static> {
        build_event(
            path,
            TelemetryEventType::Heartbeat,
            None,
            None,
            None,
            None,
            None,
            None,
            true,
        )
        .expect("test lifecycle event")
    }

    #[test]
    fn event_chain_advances_only_after_enqueue_or_persist_acceptance() {
        let root = tempfile::tempdir().expect("telemetry root");
        let path = root.path().join(TELEMETRY_RELATIVE_PATH);
        event_chain_contexts()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(path.clone(), fresh_event_chain_context());

        let rejected = test_lifecycle_event(&path);
        let retry = test_lifecycle_event(&path);
        assert_eq!(rejected.sequence, 1);
        assert_eq!(retry.sequence, 1);
        assert_eq!(rejected.previous_event_sha256, EMPTY_EVENT_CHAIN_SHA256);
        assert_eq!(retry.previous_event_sha256, EMPTY_EVENT_CHAIN_SHA256);

        commit_event_chain(&path, &retry);
        let accepted_successor = test_lifecycle_event(&path);
        assert_eq!(accepted_successor.sequence, 2);
        assert_eq!(
            accepted_successor.previous_event_sha256,
            retry.event_body_sha256
        );
    }

    #[cfg(unix)]
    #[test]
    fn crashed_segment_is_atomically_quarantined_before_a_fresh_epoch() {
        use std::os::unix::fs::MetadataExt;

        let root = tempfile::tempdir().expect("telemetry root");
        let path = root.path().join(TELEMETRY_RELATIVE_PATH);
        // Construct an otherwise healthy, incomplete segment directly. Other
        // tests intentionally poison process-global health counters, so using
        // the production lifecycle helper here would make the quarantine
        // classification depend on test scheduling.
        let mut start = build_event(
            &path,
            TelemetryEventType::ProcessStart,
            None,
            None,
            None,
            None,
            None,
            None,
            true,
        )
        .expect("crashed process-start event");
        start.dropped_event_count = 0;
        start.persist_error_count = 0;
        start.incomplete_observation_count = 0;
        start.capacity_reached = false;
        start.event_body_sha256 = event_body_sha256(&start).expect("healthy event digest");
        persist_event(&path, &start).expect("persist crashed process-start segment");
        assert!(path.is_file());

        prepare_event_epoch(&path).expect("quarantine crashed segment");

        assert!(!path.exists());
        let parent = path.parent().expect("telemetry parent");
        let quarantine = fs::read_dir(parent)
            .expect("read telemetry parent")
            .map(|entry| entry.expect("telemetry entry").file_name())
            .find(|name| name.to_string_lossy().contains(".quarantine-crashed-"))
            .expect("crashed segment quarantine");
        let quarantine_path = parent.join(quarantine);
        assert!(quarantine_path.is_file());
        assert_eq!(
            fs::metadata(quarantine_path)
                .expect("quarantine metadata")
                .nlink(),
            1
        );
        let contexts = event_chain_contexts()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let context = contexts.get(&path).expect("fresh epoch context");
        assert_eq!(
            context.predecessor_disposition,
            PredecessorDisposition::QuarantinedCrashedSegment
        );
        assert_eq!(context.previous_event_sha256, EMPTY_EVENT_CHAIN_SHA256);
    }

    #[cfg(unix)]
    #[test]
    fn mismatched_source_epoch_is_rotated_without_becoming_current_evidence() {
        use std::os::unix::fs::PermissionsExt;

        let root = tempfile::tempdir().expect("telemetry root");
        let path = root.path().join(TELEMETRY_RELATIVE_PATH);
        let parent = path.parent().expect("telemetry parent");
        fs::create_dir(parent).expect("create telemetry parent");
        fs::set_permissions(parent, PermissionsExt::from_mode(0o700))
            .expect("private telemetry parent");
        fs::write(
            &path,
            format!("{{\"source_epoch_sha256\":\"{}\"}}\n", "f".repeat(64)),
        )
        .expect("write stale source epoch");
        fs::set_permissions(&path, PermissionsExt::from_mode(0o600))
            .expect("private telemetry file");

        prepare_event_epoch(&path).expect("rotate stale source epoch");

        assert!(!path.exists());
        assert!(
            fs::read_dir(parent)
                .expect("read telemetry parent")
                .any(|entry| entry
                    .expect("telemetry entry")
                    .file_name()
                    .to_string_lossy()
                    .contains(".quarantine-source-epoch-"))
        );
    }

    #[test]
    fn classifies_all_legacy_routes_before_preflight() {
        let states = crate::route_definition::route_definition_registry()
            .into_iter()
            .filter(|definition| definition.legacy_compatibility_route)
            .fold(BTreeMap::new(), |mut states, definition| {
                *states.entry(route_state(definition)).or_insert(0_usize) += 1;
                states
            });

        assert_eq!(
            states,
            BTreeMap::from([
                (LegacyRouteState::Legacy200, 40),
                (LegacyRouteState::CanonicalOnlyGone410, 138),
                (LegacyRouteState::QuarantinePreflight410, 28),
            ])
        );
    }

    #[test]
    fn catalog_digest_binds_complete_route_definitions() {
        let expected = format!(
            "{:x}",
            Sha256::digest(include_bytes!(
                "../../routes/control_ui_route_catalog_v1.jsonl"
            ))
        );
        assert_eq!(route_catalog_sha(), expected);
        assert!(valid_sha256(route_catalog_sha()));
        assert!(!valid_sha256("unknown"));
        assert!(!valid_source_head("unknown"));
    }

    #[test]
    fn persists_allowlisted_jsonl_without_request_identifiers() {
        let root = tempfile::tempdir().expect("telemetry root");
        let path = root.path().join(TELEMETRY_RELATIVE_PATH);
        let event = test_event();

        persist_event(&path, &event).expect("persist telemetry event");

        let line = fs::read_to_string(path).expect("read telemetry JSONL");
        let value: serde_json::Value = serde_json::from_str(line.trim()).expect("telemetry JSON");
        assert_eq!(
            value
                .as_object()
                .expect("telemetry object")
                .keys()
                .collect::<Vec<_>>(),
            vec![
                "authentication_key_id_sha256",
                "capacity_reached",
                "catalog_binding_valid",
                "catalog_sha",
                "consumer_class",
                "dropped_event_count",
                "event_body_sha256",
                "event_hmac_sha256",
                "event_type",
                "head_sha",
                "http_status",
                "incomplete_observation_count",
                "observation_complete",
                "persist_error_count",
                "predecessor_disposition",
                "predecessor_segment_sha256",
                "preflight",
                "previous_event_sha256",
                "process_class",
                "process_run_identifier_sha256",
                "route_key",
                "route_state",
                "run_class",
                "schema",
                "sequence",
                "source_binding_valid",
                "source_epoch_sha256",
                "time_unix_ms",
                "write_result",
            ]
        );
        assert!(!line.contains("secret-query"));
        assert!(!line.contains("secret-body"));
        assert!(!line.contains("127.0.0.1"));
        assert!(!line.contains("user-agent"));
    }

    #[test]
    fn graceful_shutdown_persists_process_stop_after_lifecycle_events() {
        let root = tempfile::tempdir().expect("telemetry root");
        let path = root.path().join(TELEMETRY_RELATIVE_PATH);

        with_test_state_root(root.path(), || {
            record_process_lifecycle();
            record_process_stop_and_flush().expect("flush graceful process stop");
        });

        let events = fs::read_to_string(path)
            .expect("read lifecycle events")
            .lines()
            .map(|line| serde_json::from_str::<serde_json::Value>(line).expect("event JSON"))
            .collect::<Vec<_>>();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0]["event_type"], "process_start");
        assert_eq!(events[1]["event_type"], "heartbeat");
        assert_eq!(events[2]["event_type"], "process_stop");
        assert!(
            events
                .iter()
                .all(|event| event["observation_complete"] == true)
        );
    }

    #[test]
    fn production_writer_reports_post_write_sync_failure_without_durability_claim() {
        let root = tempfile::tempdir().expect("telemetry root");
        let path = root.path().join(TELEMETRY_RELATIVE_PATH);
        let (sender, receiver) = sync_channel::<TelemetryCommand>(4);
        let writer = thread::spawn(move || {
            run_telemetry_writer(
                receiver,
                |write| {
                    if write
                        .encoded
                        .windows(b"\"event_type\":\"process_stop\"".len())
                        .any(|window| window == b"\"event_type\":\"process_stop\"")
                    {
                        let _write_guard = TELEMETRY_WRITE_LOCK
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner);
                        let mut secure = open_secure_telemetry_file(&write.path)?;
                        lock_telemetry_file(&secure.file)?;
                        validate_telemetry_file(&secure.file)?;
                        secure
                            .file
                            .write_all(&write.encoded)
                            .context("inject complete stop write before sync failure")?;
                        anyhow::bail!("injected sync_data failure after complete terminal write")
                    }
                    persist_encoded_with_limit(&write.path, &write.encoded, TELEMETRY_MAX_BYTES)
                },
                false,
            );
        });

        let mut start = test_event();
        start.event_type = TelemetryEventType::ProcessStart;
        start.sequence = 1;
        let (start_ack, start_done) = channel();
        sender
            .send(TelemetryCommand::Append {
                write: TelemetryWrite {
                    path: path.clone(),
                    encoded: encode_event(&start).expect("start event"),
                },
                acknowledge: Some(start_ack),
            })
            .expect("queue process start");
        assert_eq!(start_done.recv().expect("start ack"), Ok(()));

        let (prior_ack, prior_done) = channel();
        sender
            .send(TelemetryCommand::Flush(prior_ack))
            .expect("queue prior barrier");
        assert_eq!(
            prior_done.recv().expect("prior barrier").append_error_count,
            0
        );

        let mut stop = test_event();
        stop.event_type = TelemetryEventType::ProcessStop;
        stop.sequence = 2;
        let (stop_ack, stop_done) = channel();
        sender
            .send(TelemetryCommand::Append {
                write: TelemetryWrite {
                    path: path.clone(),
                    encoded: encode_event(&stop).expect("stop event"),
                },
                acknowledge: Some(stop_ack),
            })
            .expect("queue process stop");
        assert!(stop_done.recv().expect("stop ack").is_err());

        let (terminal_ack, terminal_done) = channel();
        sender
            .send(TelemetryCommand::Flush(terminal_ack))
            .expect("queue terminal barrier");
        let terminal = terminal_done.recv().expect("terminal barrier");
        assert_eq!(terminal.append_error_count, 1);
        assert!(terminal.last_append_error.is_some());
        drop(sender);
        writer.join().expect("join telemetry writer");

        let contents = fs::read_to_string(path).expect("read telemetry stream");
        assert!(contents.contains("\"event_type\":\"process_start\""));
        assert!(contents.contains("\"event_type\":\"process_stop\""));
        // The raw marker can remain after a failed sync. The window producer
        // therefore keeps durable_process_stop_observed and
        // shutdown_flush_verified false until an independent receipt exists.
    }

    #[test]
    fn shutdown_health_rejects_every_degraded_counter() {
        let healthy = ShutdownHealthSnapshot {
            pending_event_count: 0,
            dropped_event_count: 0,
            persist_error_count: 0,
            incomplete_observation_count: 0,
            corrupt_file_count: 0,
            capacity_reached: false,
        };
        ensure_shutdown_health(healthy, "test").expect("healthy snapshot");
        for degraded in [
            ShutdownHealthSnapshot {
                pending_event_count: 1,
                ..healthy
            },
            ShutdownHealthSnapshot {
                dropped_event_count: 1,
                ..healthy
            },
            ShutdownHealthSnapshot {
                persist_error_count: 1,
                ..healthy
            },
            ShutdownHealthSnapshot {
                incomplete_observation_count: 1,
                ..healthy
            },
            ShutdownHealthSnapshot {
                corrupt_file_count: 1,
                ..healthy
            },
            ShutdownHealthSnapshot {
                capacity_reached: true,
                ..healthy
            },
        ] {
            assert!(ensure_shutdown_health(degraded, "test").is_err());
        }
    }

    #[test]
    fn persistence_failure_increments_drop_and_error_counters() {
        let root = tempfile::tempdir().expect("telemetry root");
        let blocked_root = root.path().join("not-a-directory");
        fs::write(&blocked_root, b"file").expect("blocking file");
        let before_dropped = DROPPED_EVENT_COUNT.load(Ordering::Relaxed);
        let before_errors = PERSIST_ERROR_COUNT.load(Ordering::Relaxed);
        let event = test_event();
        let error = persist_event(&blocked_root.join(TELEMETRY_RELATIVE_PATH), &event)
            .expect_err("blocked telemetry path");

        record_persist_error(&error);

        assert!(DROPPED_EVENT_COUNT.load(Ordering::Relaxed) > before_dropped);
        assert!(PERSIST_ERROR_COUNT.load(Ordering::Relaxed) > before_errors);
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlink_parent_and_symlink_target() {
        use std::os::unix::fs::symlink;

        let root = tempfile::tempdir().expect("telemetry root");
        let redirected = tempfile::tempdir().expect("redirected root");
        let parent = root.path().join("control-ui");
        symlink(redirected.path(), &parent).expect("parent symlink");
        let parent_error = persist_event(&root.path().join(TELEMETRY_RELATIVE_PATH), &test_event())
            .expect_err("symlink parent must fail closed");
        assert!(
            format!("{parent_error:#}").contains("non-symlink legacy route telemetry directory")
        );

        fs::remove_file(&parent).expect("remove parent symlink");
        fs::create_dir(&parent).expect("private parent");
        fs::set_permissions(&parent, std::os::unix::fs::PermissionsExt::from_mode(0o700))
            .expect("private parent permissions");
        let target = redirected.path().join("redirected.jsonl");
        fs::write(&target, b"").expect("redirected file");
        symlink(
            &target,
            parent.join(
                Path::new(TELEMETRY_RELATIVE_PATH)
                    .file_name()
                    .expect("telemetry file name"),
            ),
        )
        .expect("target symlink");
        let target_error = persist_event(&root.path().join(TELEMETRY_RELATIVE_PATH), &test_event())
            .expect_err("symlink target must fail closed");
        assert!(format!("{target_error:#}").contains("non-symlink telemetry file"));
    }

    #[cfg(unix)]
    #[test]
    fn rejects_wide_permissions_partial_tail_and_capacity_overflow() {
        use std::os::unix::fs::PermissionsExt;

        let root = tempfile::tempdir().expect("telemetry root");
        let path = root.path().join(TELEMETRY_RELATIVE_PATH);
        persist_event(&path, &test_event()).expect("initial event");

        fs::set_permissions(&path, PermissionsExt::from_mode(0o644))
            .expect("widen telemetry permissions");
        assert!(
            format!(
                "{:#}",
                persist_event(&path, &test_event()).expect_err("wide file must fail closed")
            )
            .contains("permissions must be 0600")
        );

        fs::set_permissions(&path, PermissionsExt::from_mode(0o600))
            .expect("restore telemetry permissions");
        fs::write(&path, b"{\"partial\":true}").expect("partial JSONL record");
        let before_corrupt = CORRUPT_FILE_COUNT.load(Ordering::Relaxed);
        assert!(
            format!(
                "{:#}",
                persist_event(&path, &test_event()).expect_err("partial tail must fail closed")
            )
            .contains("incomplete JSONL record")
        );
        assert!(CORRUPT_FILE_COUNT.load(Ordering::Relaxed) > before_corrupt);

        fs::write(&path, b"").expect("clear partial record");
        CAPACITY_REACHED.store(false, Ordering::Relaxed);
        let encoded_len = serde_json::to_vec(&test_event()).expect("event JSON").len() as u64 + 1;
        assert!(persist_event_with_limit(&path, &test_event(), encoded_len - 1).is_err());
        assert!(CAPACITY_REACHED.load(Ordering::Relaxed));
        assert_eq!(fs::metadata(path).expect("telemetry metadata").len(), 0);
    }

    #[test]
    fn missing_observation_fields_are_serialized_as_gaps_not_guessed_statuses() {
        let root = tempfile::tempdir().expect("telemetry root");
        let path = root.path().join(TELEMETRY_RELATIVE_PATH);
        let mut event = test_event();
        event.preflight = None;
        event.http_status = None;
        event.write_result = None;
        event.observation_complete = false;

        persist_event(&path, &event).expect("persist incomplete observation");

        let value: serde_json::Value =
            serde_json::from_str(fs::read_to_string(path).expect("telemetry JSONL").trim())
                .expect("telemetry JSON");
        assert!(value["preflight"].is_null());
        assert!(value["http_status"].is_null());
        assert!(value["write_result"].is_null());
        assert_eq!(value["observation_complete"], false);
    }

    #[test]
    fn finish_request_marks_missing_response_observation_as_health_gap() {
        let root = tempfile::tempdir().expect("telemetry root");
        let definition = crate::route_definition::route_definition_registry()
            .into_iter()
            .rev()
            .find(|definition| {
                definition.legacy_compatibility_route
                    && definition.lifecycle.method == "GET"
                    && !definition.lifecycle.path_pattern.contains('<')
            })
            .expect("legacy route");
        let request = format!(
            "GET {} HTTP/1.1\r\nHost: localhost\r\n\r\n",
            definition.lifecycle.path_pattern
        );
        let before = INCOMPLETE_OBSERVATION_COUNT.load(Ordering::Relaxed);
        with_test_state_root(root.path(), || {
            begin_request(
                &request,
                definition.lifecycle.method,
                definition.lifecycle.path_pattern,
            );
            record_preflight(PreflightResult::Accepted);
            finish_request(&Ok(()));
        });
        assert!(INCOMPLETE_OBSERVATION_COUNT.load(Ordering::Relaxed) > before);
        let contents =
            fs::read_to_string(root.path().join(TELEMETRY_RELATIVE_PATH)).expect("gap telemetry");
        let value: serde_json::Value =
            serde_json::from_str(contents.lines().last().expect("request telemetry event"))
                .expect("gap telemetry JSON");
        assert!(value["http_status"].is_null());
        assert!(value["write_result"].is_null());
        assert_eq!(value["observation_complete"], false);
        assert_eq!(telemetry_health().retirement_evidence_ready, false);
    }

    #[test]
    fn run_class_is_process_fixed_and_not_request_header_controlled() {
        let forged = "GET / HTTP/1.1\r\nX-Hepta-Run-Class: operator\r\n\r\n";
        assert!(valid_run_class("ci_test"));
        assert!(valid_run_class("operator"));
        assert!(!valid_run_class("operator_from_header"));
        assert_eq!(process_run_class_value(), "ci_test");
        assert_eq!(
            anonymous_consumer_class(forged),
            AnonymousConsumerClass::Unclassified
        );
    }

    #[cfg(unix)]
    #[test]
    fn competing_writer_lock_drops_instead_of_interleaving() {
        let root = tempfile::tempdir().expect("telemetry root");
        let path = root.path().join(TELEMETRY_RELATIVE_PATH);
        let locked = open_secure_telemetry_file(&path).expect("secure telemetry file");
        lock_telemetry_file(&locked.file).expect("hold writer lock");
        let error = persist_event(&path, &test_event()).expect_err("contended writer must fail");
        assert!(
            format!("{error:#}").contains("cross-process telemetry writer lock"),
            "{error:#}"
        );
        assert_eq!(fs::metadata(path).expect("telemetry metadata").len(), 0);
    }

    #[test]
    fn records_only_legacy_direct_calls() {
        let mut legacy = crate::route_definition::route_definition_registry()
            .into_iter()
            .find(|definition| definition.legacy_compatibility_route)
            .expect("legacy route");
        legacy.lifecycle.path_pattern = "/__test__/legacy-route-usage/direct-call";
        let canonical = crate::route_definition::route_definition(
            "GET",
            crate::route_registry::EVIDENCE_INDEX_ENDPOINT,
        )
        .expect("canonical evidence route");
        let before = direct_call_count(legacy.lifecycle.path_pattern);

        record_direct_call(legacy);
        record_direct_call(canonical);

        assert_eq!(
            direct_call_count(legacy.lifecycle.path_pattern),
            before.saturating_add(1)
        );
        assert_eq!(direct_call_count(canonical.lifecycle.path_pattern), 0);
    }
}
