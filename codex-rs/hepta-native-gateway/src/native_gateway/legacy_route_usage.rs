use std::cell::RefCell;
use std::collections::BTreeMap;
use std::env;
use std::ffi::CString;
#[cfg(test)]
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
#[cfg(not(test))]
use std::sync::mpsc::SyncSender;
#[cfg(not(test))]
use std::sync::mpsc::TrySendError;
#[cfg(not(test))]
use std::sync::mpsc::sync_channel;
#[cfg(not(test))]
use std::thread;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::Context;
use anyhow::Result;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;

use crate::route_definition::RouteDefinition;
use crate::route_definition::RouteDispatchHandler;

const TELEMETRY_RELATIVE_PATH: &str = "control-ui/legacy-route-usage-v1.jsonl";
const TELEMETRY_ENABLED_ENV: &str = "HEPTA_CONTROL_UI_LEGACY_ROUTE_TELEMETRY";
const TELEMETRY_SCHEMA: &str = "hepta_legacy_route_usage_event_v1";
const TELEMETRY_HEALTH_SCHEMA: &str = "hepta_legacy_route_telemetry_health_v1";
const TELEMETRY_MAX_BYTES: u64 = 8 * 1024 * 1024;
const TELEMETRY_MAX_EVENT_BYTES: usize = 16 * 1024;
const TELEMETRY_QUEUE_CAPACITY: usize = 256;

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
static ROUTE_CATALOG_SHA: OnceLock<String> = OnceLock::new();
static PROCESS_RUN_IDENTIFIER_SHA: OnceLock<String> = OnceLock::new();
#[cfg(not(test))]
static TELEMETRY_SENDER: OnceLock<std::result::Result<SyncSender<TelemetryWrite>, String>> =
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

#[derive(Debug)]
struct PendingObservation {
    route_key: &'static str,
    route_state: LegacyRouteState,
    consumer_class: AnonymousConsumerClass,
    preflight: Option<PreflightResult>,
    http_status: Option<u16>,
    write_result: Option<ResponseWriteResult>,
}

#[cfg(not(test))]
struct TelemetryWrite {
    path: PathBuf,
    encoded: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
struct LegacyRouteUsageEvent<'a> {
    schema: &'static str,
    process_run_identifier_sha256: &'a str,
    sequence: u64,
    time_unix_ms: u64,
    process_class: &'static str,
    run_class: &'static str,
    head_sha: &'a str,
    catalog_sha: &'a str,
    source_binding_valid: bool,
    catalog_binding_valid: bool,
    route_key: &'a str,
    route_state: LegacyRouteState,
    consumer_class: AnonymousConsumerClass,
    preflight: Option<PreflightResult>,
    http_status: Option<u16>,
    write_result: Option<ResponseWriteResult>,
    observation_complete: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct LegacyRouteTelemetryHealth {
    schema: &'static str,
    status: &'static str,
    enable_env: &'static str,
    enabled: bool,
    writer_healthy: bool,
    process_run_identifier_sha256: &'static str,
    source_head_sha: &'static str,
    source_binding_valid: bool,
    route_catalog_sha256: &'static str,
    catalog_binding_valid: bool,
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
    let sequence = EVENT_SEQUENCE
        .fetch_add(1, Ordering::Relaxed)
        .saturating_add(1);
    let head_sha = hepta_core::production_surface_report().source_git_head;
    let catalog_sha = route_catalog_sha();
    let source_binding_valid = valid_source_head(head_sha);
    let catalog_binding_valid = valid_sha256(catalog_sha);
    let observation_complete = observation.preflight.is_some()
        && observation.http_status.is_some()
        && observation.write_result.is_some();
    if !observation_complete {
        INCOMPLETE_OBSERVATION_COUNT.fetch_add(1, Ordering::Relaxed);
    }
    let event = LegacyRouteUsageEvent {
        schema: TELEMETRY_SCHEMA,
        process_run_identifier_sha256: process_run_identifier_sha(),
        sequence,
        time_unix_ms: unix_time_ms(),
        process_class: "hepta_native_gateway",
        run_class: if cfg!(test) { "test" } else { "serve_ui" },
        head_sha,
        catalog_sha,
        source_binding_valid,
        catalog_binding_valid,
        route_key: observation.route_key,
        route_state: observation.route_state,
        consumer_class: observation.consumer_class,
        preflight: observation.preflight,
        http_status: observation.http_status,
        write_result: observation.write_result,
        observation_complete,
    };
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
    if let Err(error) = record_event(path, &event) {
        record_persist_error(&error);
    }
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
    let writer_healthy = enabled
        && source_binding_valid
        && catalog_binding_valid
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
        enabled,
        writer_healthy,
        process_run_identifier_sha256: process_run_identifier_sha(),
        source_head_sha,
        source_binding_valid,
        route_catalog_sha256,
        catalog_binding_valid,
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
        summary_producer_available: false,
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
            let mut definitions = crate::route_definition::route_definition_registry();
            definitions.sort_by_key(|definition| {
                (
                    definition.lifecycle.method,
                    definition.lifecycle.path_pattern,
                )
            });
            serde_json::to_vec(&definitions)
                .map(|bytes| format!("{:x}", Sha256::digest(bytes)))
                .unwrap_or_else(|_| "unknown".to_string())
        })
        .as_str()
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

#[cfg(not(test))]
fn record_event(path: PathBuf, event: &LegacyRouteUsageEvent<'_>) -> Result<()> {
    let encoded = encode_event(event)?;
    let sender = TELEMETRY_SENDER
        .get_or_init(start_telemetry_writer)
        .as_ref()
        .map_err(|error| anyhow::anyhow!(error.clone()))?;
    PENDING_EVENT_COUNT.fetch_add(1, Ordering::Relaxed);
    match sender.try_send(TelemetryWrite { path, encoded }) {
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
fn start_telemetry_writer() -> std::result::Result<SyncSender<TelemetryWrite>, String> {
    let (sender, receiver) = sync_channel::<TelemetryWrite>(TELEMETRY_QUEUE_CAPACITY);
    thread::Builder::new()
        .name("hepta-legacy-route-telemetry".to_string())
        .spawn(move || {
            for write in receiver {
                let result =
                    persist_encoded_with_limit(&write.path, &write.encoded, TELEMETRY_MAX_BYTES);
                PENDING_EVENT_COUNT.fetch_sub(1, Ordering::Relaxed);
                match result {
                    Ok(()) => {
                        PERSISTED_EVENT_COUNT.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(error) => record_persist_error(&error),
                }
            }
        })
        .map_err(|error| format!("spawn legacy route telemetry writer: {error}"))?;
    Ok(sender)
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
    let mut file = open_secure_telemetry_file(path)?;
    lock_telemetry_file(&file)?;
    validate_telemetry_file(&file)?;
    let current_bytes = file
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
    file.write_all(encoded)
        .context("append bounded legacy route telemetry event")?;
    file.sync_data()
        .context("sync legacy route telemetry event")?;
    Ok(())
}

#[cfg(unix)]
fn open_secure_telemetry_file(path: &Path) -> Result<File> {
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
    Ok(unsafe { File::from_raw_fd(file_fd) })
}

#[cfg(not(unix))]
fn open_secure_telemetry_file(_path: &Path) -> Result<File> {
    anyhow::bail!("legacy route telemetry persistence requires secure Unix openat semantics")
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
        LegacyRouteUsageEvent {
            schema: TELEMETRY_SCHEMA,
            process_run_identifier_sha256: "1111111111111111111111111111111111111111111111111111111111111111",
            sequence: 7,
            time_unix_ms: 1_722_600_000_000,
            process_class: "hepta_native_gateway",
            run_class: "test",
            head_sha: "0e52c78003b6",
            catalog_sha: "2222222222222222222222222222222222222222222222222222222222222222",
            source_binding_valid: true,
            catalog_binding_valid: true,
            route_key: "/api/example/<anonymous>",
            route_state: LegacyRouteState::Legacy200,
            consumer_class: AnonymousConsumerClass::Browser,
            preflight: Some(PreflightResult::Accepted),
            http_status: Some(200),
            write_result: Some(ResponseWriteResult::Ok),
            observation_complete: true,
        }
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
        let mut definitions = crate::route_definition::route_definition_registry();
        definitions.sort_by_key(|definition| {
            (
                definition.lifecycle.method,
                definition.lifecycle.path_pattern,
            )
        });
        let expected = format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&definitions).expect("route definitions JSON"))
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
                "catalog_binding_valid",
                "catalog_sha",
                "consumer_class",
                "head_sha",
                "http_status",
                "observation_complete",
                "preflight",
                "process_class",
                "process_run_identifier_sha256",
                "route_key",
                "route_state",
                "run_class",
                "schema",
                "sequence",
                "source_binding_valid",
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
        symlink(&target, parent.join("legacy-route-usage-v1.jsonl")).expect("target symlink");
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
        let value: serde_json::Value = serde_json::from_str(
            fs::read_to_string(root.path().join(TELEMETRY_RELATIVE_PATH))
                .expect("gap telemetry")
                .trim(),
        )
        .expect("gap telemetry JSON");
        assert!(value["http_status"].is_null());
        assert!(value["write_result"].is_null());
        assert_eq!(value["observation_complete"], false);
        assert_eq!(telemetry_health().retirement_evidence_ready, false);
    }

    #[cfg(unix)]
    #[test]
    fn competing_writer_lock_drops_instead_of_interleaving() {
        let root = tempfile::tempdir().expect("telemetry root");
        let path = root.path().join(TELEMETRY_RELATIVE_PATH);
        let locked = open_secure_telemetry_file(&path).expect("secure telemetry file");
        lock_telemetry_file(&locked).expect("hold writer lock");
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
