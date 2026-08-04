use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const EVENT_SCHEMA: &str = "hepta_control_ui_legacy_http_event_v2";
pub const WINDOW_SCHEMA: &str = "hepta_control_ui_legacy_http_window_v2";
pub const MAX_FILE_BYTES: u64 = 8 * 1024 * 1024;
pub const MAX_EVENT_BYTES: usize = 16 * 1024;
const DAY_MS: u64 = 86_400_000;
const MIN_WINDOW_MS: u64 = 30 * DAY_MS;
const MIN_ACTIVE_DAYS: usize = 14;
const MIN_ZERO_USE_MS: u64 = 14 * DAY_MS;
const MAX_WINDOW_STALENESS_MS: u64 = 2 * DAY_MS;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Event {
    schema: String,
    event_type: String,
    process_run_identifier_sha256: String,
    sequence: u64,
    time_unix_ms: u64,
    process_class: String,
    run_class: String,
    head_sha: String,
    catalog_sha: String,
    source_binding_valid: bool,
    catalog_binding_valid: bool,
    route_key: Option<String>,
    route_state: Option<String>,
    consumer_class: Option<String>,
    preflight: Option<String>,
    http_status: Option<u16>,
    write_result: Option<String>,
    observation_complete: bool,
    dropped_event_count: u64,
    persist_error_count: u64,
    incomplete_observation_count: u64,
    capacity_reached: bool,
}

#[derive(Debug)]
struct Segment {
    next_sequence: u64,
    last_time_unix_ms: u64,
    run_class: String,
    stopped: bool,
}

#[derive(Debug, Default, Serialize)]
pub struct RouteWindowSummary {
    total_requests: u64,
    non_ci_requests: u64,
    status_counts: BTreeMap<u16, u64>,
}

#[derive(Debug, Serialize)]
pub struct LegacyRouteWindowSummary {
    pub schema: &'static str,
    pub status: &'static str,
    producer: &'static str,
    event_file_sha256: String,
    promotion_authoritative: bool,
    process_stop_marker_observed: bool,
    durable_process_stop_observed: bool,
    continuous_coverage_declared: bool,
    shutdown_flush_verified: bool,
    source_head_sha: String,
    route_catalog_sha256: String,
    event_count: usize,
    process_segment_count: usize,
    operator_active_day_count: usize,
    window_start_unix_ms: Option<u64>,
    window_end_unix_ms: Option<u64>,
    window_span_days: u64,
    trailing_zero_use_days: u64,
    total_legacy_requests: u64,
    non_ci_legacy_requests: u64,
    routes: BTreeMap<String, RouteWindowSummary>,
    pub decision: WindowDecision,
}

#[derive(Debug, Serialize)]
pub struct WindowDecision {
    pub eligible: bool,
    pub blockers: Vec<String>,
}

pub fn canonical_catalog_sha256() -> String {
    format!(
        "{:x}",
        Sha256::digest(include_bytes!(
            "../routes/control_ui_route_catalog_v1.jsonl"
        ))
    )
}

pub fn summarize_path(
    path: &Path,
    expected_head: &str,
    now_unix_ms: u64,
) -> Result<LegacyRouteWindowSummary> {
    let mut file = open_bounded_file(path)?;
    let metadata = file.metadata().context("stat legacy HTTP event file")?;
    anyhow::ensure!(
        metadata.len() <= MAX_FILE_BYTES,
        "event file exceeds bounded size"
    );
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    file.by_ref()
        .take(MAX_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .context("read bounded legacy HTTP event file")?;
    anyhow::ensure!(
        bytes.len() as u64 <= MAX_FILE_BYTES,
        "event file grew past bounded size"
    );
    summarize_bytes(&bytes, expected_head, now_unix_ms)
}

pub fn summarize_bytes(
    bytes: &[u8],
    expected_head: &str,
    now_unix_ms: u64,
) -> Result<LegacyRouteWindowSummary> {
    anyhow::ensure!(
        valid_hex(expected_head, 7, 64),
        "invalid expected source HEAD"
    );
    anyhow::ensure!(!bytes.is_empty(), "event file is empty");
    anyhow::ensure!(
        bytes.len() as u64 <= MAX_FILE_BYTES,
        "event file exceeds bounded size"
    );
    anyhow::ensure!(
        bytes.last() == Some(&b'\n'),
        "event file has an incomplete final record"
    );
    anyhow::ensure!(!bytes.contains(&0), "event file contains NUL bytes");
    let catalog_sha = canonical_catalog_sha256();
    let route_states = canonical_legacy_route_states()?;
    let mut segments = BTreeMap::<String, Segment>::new();
    let mut active_days = BTreeSet::new();
    let mut operator_event_times = Vec::new();
    let mut window_start = None::<u64>;
    let mut window_end = None::<u64>;
    let mut last_operator_request = None::<u64>;
    let mut routes = BTreeMap::<String, RouteWindowSummary>::new();
    let mut total_requests = 0_u64;
    let mut non_ci_requests = 0_u64;
    let mut event_count = 0_usize;

    for (index, line) in bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        anyhow::ensure!(
            line.len() <= MAX_EVENT_BYTES,
            "event {} exceeds bounded record size",
            index + 1
        );
        let event: Event =
            serde_json::from_slice(line).with_context(|| format!("parse event {}", index + 1))?;
        validate_binding(&event, expected_head, &catalog_sha, now_unix_ms)
            .with_context(|| format!("validate event {}", index + 1))?;
        let process_id = event.process_run_identifier_sha256.clone();
        if event.event_type == "process_start" {
            anyhow::ensure!(
                !segments.contains_key(&process_id),
                "duplicate process-start segment"
            );
            anyhow::ensure!(event.sequence == 1, "process-start sequence must be 1");
            validate_lifecycle(&event)?;
            segments.insert(
                process_id,
                Segment {
                    next_sequence: 2,
                    last_time_unix_ms: event.time_unix_ms,
                    run_class: event.run_class.clone(),
                    stopped: false,
                },
            );
        } else {
            let segment = segments
                .get_mut(&process_id)
                .context("event has no process-start segment")?;
            anyhow::ensure!(
                event.sequence == segment.next_sequence,
                "process sequence gap or reorder"
            );
            anyhow::ensure!(
                event.time_unix_ms >= segment.last_time_unix_ms,
                "process timestamp regressed"
            );
            anyhow::ensure!(
                event.run_class == segment.run_class,
                "run class changed within process"
            );
            anyhow::ensure!(!segment.stopped, "event followed process-stop marker");
            segment.next_sequence = segment.next_sequence.saturating_add(1);
            segment.last_time_unix_ms = event.time_unix_ms;
            match event.event_type.as_str() {
                "heartbeat" => validate_lifecycle(&event)?,
                "process_stop" => {
                    validate_lifecycle(&event)?;
                    segment.stopped = true;
                }
                "legacy_request" => {
                    validate_request(&event, &route_states)?;
                    total_requests = total_requests.saturating_add(1);
                    let route = routes
                        .entry(event.route_key.clone().expect("validated route"))
                        .or_default();
                    route.total_requests = route.total_requests.saturating_add(1);
                    *route
                        .status_counts
                        .entry(event.http_status.expect("validated status"))
                        .or_default() += 1;
                    if event.run_class == "operator" {
                        non_ci_requests = non_ci_requests.saturating_add(1);
                        route.non_ci_requests = route.non_ci_requests.saturating_add(1);
                        last_operator_request = Some(event.time_unix_ms);
                    }
                }
                _ => anyhow::bail!("unknown event type"),
            }
        }
        if event.run_class == "operator" {
            operator_event_times.push(event.time_unix_ms);
            active_days.insert(event.time_unix_ms / DAY_MS);
            window_start = Some(
                window_start.map_or(event.time_unix_ms, |value| value.min(event.time_unix_ms)),
            );
            window_end =
                Some(window_end.map_or(event.time_unix_ms, |value| value.max(event.time_unix_ms)));
        }
        event_count += 1;
    }
    let span_ms = window_end
        .zip(window_start)
        .map_or(0, |(end, start)| end.saturating_sub(start));
    let trailing_ms = window_end.zip(window_start).map_or(0, |(end, start)| {
        end.saturating_sub(last_operator_request.unwrap_or(start))
    });
    let mut blockers = Vec::new();
    if span_ms < MIN_WINDOW_MS {
        blockers.push("observation_window_lt_30_days".into());
    }
    if active_days.len() < MIN_ACTIVE_DAYS {
        blockers.push("operator_active_days_lt_14".into());
    }
    if trailing_ms < MIN_ZERO_USE_MS {
        blockers.push("trailing_zero_use_lt_14_days".into());
    }
    if window_end.is_none_or(|end| now_unix_ms.saturating_sub(end) > MAX_WINDOW_STALENESS_MS) {
        blockers.push("observation_window_stale".into());
    }
    if non_ci_requests != 0 {
        blockers.push("non_ci_legacy_usage_nonzero".into());
    }
    if segments.is_empty() {
        blockers.push("process_segments_missing".into());
    }
    let process_stop_marker_observed =
        !segments.is_empty() && segments.values().all(|segment| segment.stopped);
    if !process_stop_marker_observed {
        blockers.push("process_stop_marker_missing".into());
    }
    let durable_process_stop_observed = false;
    blockers.push("independent_shutdown_durability_receipt_missing".into());
    operator_event_times.sort_unstable();
    let continuous_coverage_declared = operator_event_times.len() >= 2
        && operator_event_times
            .windows(2)
            .all(|pair| pair[1].saturating_sub(pair[0]) <= MAX_WINDOW_STALENESS_MS);
    if !continuous_coverage_declared {
        blockers.push("continuous_operator_coverage_unproven".into());
    }
    // The exact-source writer now acknowledges each append and sync_data call,
    // but creating the telemetry directory or its first file is not yet bound
    // to a parent-directory fsync receipt. A terminal marker alone therefore
    // cannot promote this stream as crash-durable evidence.
    let shutdown_flush_verified = false;
    blockers.push("telemetry_parent_directory_fsync_unproven".into());
    Ok(LegacyRouteWindowSummary {
        schema: WINDOW_SCHEMA,
        status: if blockers.is_empty() {
            "ready"
        } else {
            "blocked"
        },
        producer: "hepta-native-gateway/hepta-legacy-route-window",
        event_file_sha256: format!("{:x}", Sha256::digest(bytes)),
        promotion_authoritative: false,
        process_stop_marker_observed,
        durable_process_stop_observed,
        continuous_coverage_declared,
        shutdown_flush_verified,
        source_head_sha: expected_head.to_string(),
        route_catalog_sha256: catalog_sha,
        event_count,
        process_segment_count: segments.len(),
        operator_active_day_count: active_days.len(),
        window_start_unix_ms: window_start,
        window_end_unix_ms: window_end,
        window_span_days: span_ms / DAY_MS,
        trailing_zero_use_days: trailing_ms / DAY_MS,
        total_legacy_requests: total_requests,
        non_ci_legacy_requests: non_ci_requests,
        routes,
        decision: WindowDecision {
            eligible: blockers.is_empty(),
            blockers,
        },
    })
}

fn validate_binding(event: &Event, head: &str, catalog: &str, now: u64) -> Result<()> {
    anyhow::ensure!(event.schema == EVENT_SCHEMA, "event schema mismatch");
    anyhow::ensure!(
        event.process_class == "hepta_native_gateway",
        "process class mismatch"
    );
    anyhow::ensure!(
        matches!(event.run_class.as_str(), "operator" | "ci_test"),
        "untrusted run class"
    );
    anyhow::ensure!(
        valid_hex(&event.process_run_identifier_sha256, 64, 64),
        "invalid process identifier"
    );
    anyhow::ensure!(
        event.head_sha == head && event.source_binding_valid,
        "source HEAD binding mismatch"
    );
    anyhow::ensure!(
        event.catalog_sha == catalog && event.catalog_binding_valid,
        "catalog binding mismatch"
    );
    anyhow::ensure!(
        event.time_unix_ms > 0 && event.time_unix_ms <= now.saturating_add(300_000),
        "invalid or future timestamp"
    );
    anyhow::ensure!(
        event.dropped_event_count == 0,
        "dropped event count is nonzero"
    );
    anyhow::ensure!(
        event.persist_error_count == 0,
        "persist error count is nonzero"
    );
    anyhow::ensure!(
        event.incomplete_observation_count == 0,
        "incomplete observation count is nonzero"
    );
    anyhow::ensure!(!event.capacity_reached, "writer capacity was reached");
    Ok(())
}

fn validate_lifecycle(event: &Event) -> Result<()> {
    anyhow::ensure!(
        event.route_key.is_none() && event.route_state.is_none() && event.consumer_class.is_none(),
        "lifecycle event contains route data"
    );
    anyhow::ensure!(
        event.preflight.is_none() && event.http_status.is_none() && event.write_result.is_none(),
        "lifecycle event contains outcome data"
    );
    anyhow::ensure!(event.observation_complete, "lifecycle event is incomplete");
    Ok(())
}

fn validate_request(event: &Event, states: &BTreeMap<String, String>) -> Result<()> {
    let route = event
        .route_key
        .as_deref()
        .context("request route missing")?;
    anyhow::ensure!(
        states.get(route).map(String::as_str) == event.route_state.as_deref(),
        "route state or catalog membership mismatch"
    );
    anyhow::ensure!(
        matches!(
            event.consumer_class.as_deref(),
            Some("browser" | "json_client" | "unclassified")
        ),
        "consumer class invalid"
    );
    anyhow::ensure!(
        matches!(
            event.preflight.as_deref(),
            Some("accepted" | "rejected" | "invalid")
        ),
        "preflight result invalid"
    );
    anyhow::ensure!(
        matches!(event.http_status, Some(100..=599)),
        "HTTP status invalid"
    );
    anyhow::ensure!(
        matches!(event.write_result.as_deref(), Some("ok" | "error")),
        "write result missing"
    );
    anyhow::ensure!(event.observation_complete, "request observation incomplete");
    Ok(())
}

fn canonical_legacy_route_states() -> Result<BTreeMap<String, String>> {
    let mut routes = BTreeMap::new();
    for line in include_str!("../routes/control_ui_route_catalog_v1.jsonl").lines() {
        let value: serde_json::Value =
            serde_json::from_str(line).context("parse embedded route catalog")?;
        if value["kind"] != "control_ui_route" || value["legacy_compatibility_route"] != true {
            continue;
        }
        let state = if value["permission_profile"] == "control_ui_quarantined_legacy_mutation" {
            "quarantine_preflight_410"
        } else if value["dispatch_handler"] == "RetiredCompatibility" {
            "canonical_only_gone_410"
        } else {
            "legacy_200"
        };
        routes.insert(
            value["path"]
                .as_str()
                .context("catalog route path missing")?
                .to_string(),
            state.to_string(),
        );
    }
    anyhow::ensure!(routes.len() == 206, "embedded legacy route count drifted");
    Ok(routes)
}

fn valid_hex(value: &str, min: usize, max: usize) -> bool {
    (min..=max).contains(&value.len()) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn open_bounded_file(path: &Path) -> Result<File> {
    #[cfg(unix)]
    {
        use std::os::fd::AsRawFd;
        use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
        let file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(path)
            .with_context(|| format!("open legacy HTTP event file {}", path.display()))?;
        let metadata = file.metadata().context("stat legacy HTTP event file")?;
        anyhow::ensure!(
            metadata.is_file() && metadata.nlink() == 1,
            "event input is not a single-link regular file"
        );
        anyhow::ensure!(
            metadata.uid() == unsafe { libc::geteuid() },
            "event input owner mismatch"
        );
        anyhow::ensure!(
            metadata.mode() & 0o777 == 0o600,
            "event input permissions must be 0600"
        );
        let locked = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) };
        anyhow::ensure!(locked == 0, "event input is busy or cannot be locked");
        Ok(file)
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        anyhow::bail!("legacy HTTP window producer requires Unix no-follow file semantics")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn event(kind: &str, sequence: u64, time: u64, run_class: &str) -> serde_json::Value {
        json!({
            "schema": EVENT_SCHEMA, "event_type": kind,
            "process_run_identifier_sha256": "1111111111111111111111111111111111111111111111111111111111111111",
            "sequence": sequence, "time_unix_ms": time, "process_class": "hepta_native_gateway",
            "run_class": run_class, "head_sha": "abcdef123456", "catalog_sha": canonical_catalog_sha256(),
            "source_binding_valid": true, "catalog_binding_valid": true,
            "route_key": null, "route_state": null, "consumer_class": null, "preflight": null,
            "http_status": null, "write_result": null, "observation_complete": true,
            "dropped_event_count": 0, "persist_error_count": 0,
            "incomplete_observation_count": 0, "capacity_reached": false
        })
    }

    fn encode(events: &[serde_json::Value]) -> Vec<u8> {
        events
            .iter()
            .flat_map(|event| {
                let mut line = serde_json::to_vec(event).expect("event JSON");
                line.push(b'\n');
                line
            })
            .collect()
    }

    #[test]
    fn complete_synthetic_window_without_process_stop_stays_blocked() {
        let now = 2_000_000_000_000_u64;
        let start = now - 31 * DAY_MS;
        let mut events = vec![event("process_start", 1, start, "operator")];
        for day in 1..=31 {
            events.push(event(
                "heartbeat",
                day + 1,
                start + day * DAY_MS,
                "operator",
            ));
        }
        let summary = summarize_bytes(&encode(&events), "abcdef123456", now).expect("summary");
        assert!(!summary.decision.eligible);
        assert_eq!(summary.status, "blocked");
        assert!(
            summary
                .decision
                .blockers
                .contains(&"process_stop_marker_missing".to_string())
        );
        assert_eq!(summary.operator_active_day_count, 32);
    }

    #[test]
    fn complete_zero_use_window_stays_blocked_without_parent_fsync_receipt() {
        let now = 2_000_000_000_000_u64;
        let start = now - 31 * DAY_MS;
        let mut events = vec![event("process_start", 1, start, "operator")];
        for day in 1..=30 {
            events.push(event(
                "heartbeat",
                day + 1,
                start + day * DAY_MS,
                "operator",
            ));
        }
        events.push(event("process_stop", 32, now, "operator"));

        let summary = summarize_bytes(&encode(&events), "abcdef123456", now)
            .expect("durable complete summary");

        assert!(!summary.decision.eligible);
        assert_eq!(summary.status, "blocked");
        assert!(summary.process_stop_marker_observed);
        assert!(!summary.durable_process_stop_observed);
        assert!(summary.continuous_coverage_declared);
        assert!(!summary.shutdown_flush_verified);
        assert_eq!(
            summary.decision.blockers,
            vec![
                "independent_shutdown_durability_receipt_missing",
                "telemetry_parent_directory_fsync_unproven",
            ]
        );
    }

    #[test]
    fn rejects_sequence_gap_missing_segment_and_binding_drift() {
        let now = 2_000_000_000_000_u64;
        let start = now - 31 * DAY_MS;
        let gap = vec![
            event("process_start", 1, start, "operator"),
            event("heartbeat", 3, now, "operator"),
        ];
        assert!(summarize_bytes(&encode(&gap), "abcdef123456", now).is_err());
        assert!(
            summarize_bytes(
                &encode(&[event("heartbeat", 1, start, "operator")]),
                "abcdef123456",
                now
            )
            .is_err()
        );
        let mut drift = event("process_start", 1, start, "operator");
        drift["catalog_sha"] = json!("2".repeat(64));
        assert!(summarize_bytes(&encode(&[drift]), "abcdef123456", now).is_err());
    }

    #[test]
    fn rejects_corruption_health_gaps_and_untrusted_run_class() {
        let now = 2_000_000_000_000_u64;
        assert!(summarize_bytes(b"{bad}\n", "abcdef123456", now).is_err());
        let mut unhealthy = event("process_start", 1, now - DAY_MS, "operator");
        unhealthy["persist_error_count"] = json!(1);
        assert!(summarize_bytes(&encode(&[unhealthy]), "abcdef123456", now).is_err());
        let forged = event("process_start", 1, now - DAY_MS, "operator_from_header");
        assert!(summarize_bytes(&encode(&[forged]), "abcdef123456", now).is_err());
    }

    #[test]
    fn blocks_short_inactive_or_used_window() {
        let now = 2_000_000_000_000_u64;
        let start = now - 31 * DAY_MS;
        let mut events = vec![event("process_start", 1, start, "operator")];
        for day in 1..=12 {
            events.push(event(
                "heartbeat",
                day + 1,
                start + day * DAY_MS,
                "operator",
            ));
        }
        let summary =
            summarize_bytes(&encode(&events), "abcdef123456", now).expect("blocked summary");
        assert!(!summary.decision.eligible);
        assert!(
            summary
                .decision
                .blockers
                .contains(&"operator_active_days_lt_14".to_string())
        );
        assert!(
            summary
                .decision
                .blockers
                .contains(&"observation_window_lt_30_days".to_string())
        );
    }

    #[test]
    fn blocks_non_ci_use_and_short_trailing_zero_use() {
        let now = 2_000_000_000_000_u64;
        let start = now - 31 * DAY_MS;
        let mut events = vec![event("process_start", 1, start, "operator")];
        for day in 1..=30 {
            events.push(event(
                "heartbeat",
                day + 1,
                start + day * DAY_MS,
                "operator",
            ));
        }
        let mut request = event("legacy_request", 32, now - DAY_MS / 2, "operator");
        let (route, state) = canonical_legacy_route_states()
            .unwrap()
            .into_iter()
            .next()
            .unwrap();
        request["route_key"] = json!(route);
        request["route_state"] = json!(state);
        request["consumer_class"] = json!("browser");
        request["preflight"] = json!("accepted");
        request["http_status"] = json!(410);
        request["write_result"] = json!("ok");
        events.push(request);
        events.push(event("heartbeat", 33, now, "operator"));
        let summary =
            summarize_bytes(&encode(&events), "abcdef123456", now).expect("blocked summary");
        assert_eq!(summary.non_ci_legacy_requests, 1);
        assert!(
            summary
                .decision
                .blockers
                .contains(&"non_ci_legacy_usage_nonzero".to_string())
        );
        assert!(
            summary
                .decision
                .blockers
                .contains(&"trailing_zero_use_lt_14_days".to_string())
        );
    }

    #[test]
    fn blocks_stale_complete_window() {
        let now = 2_000_000_000_000_u64;
        let start = now - 34 * DAY_MS;
        let mut events = vec![event("process_start", 1, start, "operator")];
        for day in 1..=31 {
            events.push(event(
                "heartbeat",
                day + 1,
                start + day * DAY_MS,
                "operator",
            ));
        }
        let summary =
            summarize_bytes(&encode(&events), "abcdef123456", now).expect("stale summary");
        assert!(
            summary
                .decision
                .blockers
                .contains(&"observation_window_stale".to_string())
        );
    }

    #[cfg(unix)]
    #[test]
    fn bounded_file_reader_rejects_wide_permissions() {
        use std::io::Write;
        use std::os::unix::fs::PermissionsExt;

        let now = 2_000_000_000_000_u64;
        let mut input = tempfile::NamedTempFile::new().expect("event file");
        input
            .write_all(&encode(&[event(
                "process_start",
                1,
                now - DAY_MS,
                "operator",
            )]))
            .expect("write event file");
        std::fs::set_permissions(input.path(), PermissionsExt::from_mode(0o644))
            .expect("widen event file");
        assert!(summarize_path(input.path(), "abcdef123456", now).is_err());
    }
}
