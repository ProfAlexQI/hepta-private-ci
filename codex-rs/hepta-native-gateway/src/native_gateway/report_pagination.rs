use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;

pub(super) const MAX_DEFAULT_REPORT_BYTES: usize = 128 * 1024;
const MAX_REPORT_PAGE_SOURCE_BYTES: usize = 72 * 1024;
const MAX_SUMMARY_SCALARS: usize = 32;

#[derive(Debug, Serialize)]
struct ReportSummary<'a> {
    schema: &'static str,
    status: String,
    route: &'a str,
    content_sha256: String,
    full_size_bytes: usize,
    top_level_kind: &'static str,
    top_level_entries: usize,
    scalar_summary: Map<String, Value>,
    full_detail: ReportDetailLink,
}

#[derive(Debug, Serialize)]
struct ReportDetailLink {
    detail: &'static str,
    cursor: usize,
    snapshot: String,
    encoding: &'static str,
}

#[derive(Debug, Serialize)]
struct ReportPage<'a> {
    schema: &'static str,
    status: String,
    route: &'a str,
    content_sha256: String,
    full_size_bytes: usize,
    encoding: &'static str,
    cursor: usize,
    page_size_bytes: usize,
    page_data: String,
    next_cursor: Option<usize>,
    complete: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ReportResponse {
    Body(String),
    BadRequest(&'static str),
    InternalError(&'static str),
    SnapshotConflict(&'static str),
}

pub(super) fn bounded_report_response(
    route: &str,
    query: Option<&str>,
    body: String,
) -> ReportResponse {
    let detail_request = match detail_request(query) {
        Ok(request) => request,
        Err(error) => return ReportResponse::BadRequest(error),
    };
    if body.len() <= MAX_DEFAULT_REPORT_BYTES && detail_request.is_none() {
        return ReportResponse::Body(body);
    }
    let content_sha256 = format!("{:x}", Sha256::digest(body.as_bytes()));
    if let Some(request) = detail_request {
        if request.snapshot != content_sha256 {
            return ReportResponse::SnapshotConflict(
                "report snapshot changed; fetch a fresh summary and restart pagination",
            );
        }
        return report_page(route, body, content_sha256, request.cursor);
    }
    report_summary(route, &body, content_sha256)
}

#[derive(Debug, PartialEq, Eq)]
struct DetailRequest {
    cursor: usize,
    snapshot: String,
}

fn detail_request(query: Option<&str>) -> Result<Option<DetailRequest>, &'static str> {
    let Some(query) = query.filter(|query| !query.is_empty()) else {
        return Ok(None);
    };
    let mut detail_full = false;
    let mut cursor = None;
    let mut snapshot = None;
    for pair in query.split('&') {
        let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
        match name {
            "detail" if value == "full" && !detail_full => detail_full = true,
            "detail" => return Err("report detail must be exactly full"),
            "cursor" if cursor.is_none() => {
                cursor = Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| "report cursor must be an unsigned integer")?,
                );
            }
            "cursor" => return Err("report cursor may only be provided once"),
            "snapshot" if snapshot.is_none() => {
                if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                    return Err("report snapshot must be a SHA-256 digest");
                }
                snapshot = Some(value.to_ascii_lowercase());
            }
            "snapshot" => return Err("report snapshot may only be provided once"),
            _ => {}
        }
    }
    if !detail_full && (cursor.is_some() || snapshot.is_some()) {
        return Err("report cursor and snapshot require detail=full");
    }
    if !detail_full {
        return Ok(None);
    }
    let snapshot = snapshot.ok_or("detail=full requires a report snapshot digest")?;
    Ok(Some(DetailRequest {
        cursor: cursor.unwrap_or_default(),
        snapshot,
    }))
}

fn report_summary(route: &str, body: &str, content_sha256: String) -> ReportResponse {
    let parsed = serde_json::from_str::<Value>(body).ok();
    let (top_level_kind, top_level_entries, scalar_summary) = parsed
        .as_ref()
        .map(summary_fields)
        .unwrap_or(("invalid_json", 0, Map::new()));
    match serde_json::to_string(&ReportSummary {
        schema: "hepta_report_summary_v2",
        status: source_status(parsed.as_ref()),
        route,
        content_sha256: content_sha256.clone(),
        full_size_bytes: body.len(),
        top_level_kind,
        top_level_entries,
        scalar_summary,
        full_detail: ReportDetailLink {
            detail: "full",
            cursor: 0,
            snapshot: content_sha256,
            encoding: "base64_json_bytes",
        },
    }) {
        Ok(body) => ReportResponse::Body(body),
        Err(_) => ReportResponse::InternalError("report summary serialization failed"),
    }
}

fn source_status(value: Option<&Value>) -> String {
    value
        .and_then(|value| value.get("status"))
        .and_then(Value::as_str)
        .filter(|status| !status.is_empty() && status.len() <= 128)
        .unwrap_or("unreported")
        .to_string()
}

fn summary_fields(value: &Value) -> (&'static str, usize, Map<String, Value>) {
    match value {
        Value::Object(object) => {
            let mut scalars = Map::new();
            for (key, value) in object {
                if scalars.len() >= MAX_SUMMARY_SCALARS {
                    break;
                }
                if key.len() > 128 {
                    continue;
                }
                match value {
                    Value::Null | Value::Bool(_) | Value::Number(_) => {
                        scalars.insert(key.clone(), value.clone());
                    }
                    Value::String(text) if text.len() <= 256 => {
                        scalars.insert(key.clone(), value.clone());
                    }
                    Value::Array(values) => {
                        scalars.insert(format!("{key}_count"), Value::from(values.len()));
                    }
                    Value::Object(values) => {
                        scalars.insert(format!("{key}_field_count"), Value::from(values.len()));
                    }
                    Value::String(_) => {}
                }
            }
            ("object", object.len(), scalars)
        }
        Value::Array(values) => ("array", values.len(), Map::new()),
        Value::Null => ("null", 0, Map::new()),
        Value::Bool(_) => ("boolean", 1, Map::new()),
        Value::Number(_) => ("number", 1, Map::new()),
        Value::String(_) => ("string", 1, Map::new()),
    }
}

fn report_page(route: &str, body: String, content_sha256: String, cursor: usize) -> ReportResponse {
    if cursor > body.len() {
        return ReportResponse::BadRequest("report cursor exceeds full response size");
    }
    let end = cursor
        .saturating_add(MAX_REPORT_PAGE_SOURCE_BYTES)
        .min(body.len());
    let page = &body.as_bytes()[cursor..end];
    let next_cursor = (end < body.len()).then_some(end);
    let status = serde_json::from_str::<Value>(&body)
        .ok()
        .as_ref()
        .map_or_else(
            || "unreported".to_string(),
            |value| source_status(Some(value)),
        );
    match serde_json::to_string(&ReportPage {
        schema: "hepta_report_page_v2",
        status,
        route,
        content_sha256,
        full_size_bytes: body.len(),
        encoding: "base64_json_bytes",
        cursor,
        page_size_bytes: page.len(),
        page_data: STANDARD.encode(page),
        next_cursor,
        complete: next_cursor.is_none(),
    }) {
        Ok(body) => ReportResponse::Body(body),
        Err(_) => ReportResponse::InternalError("report page serialization failed"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_report_is_unchanged() {
        let body = r#"{"status":"ready"}"#.to_string();
        assert_eq!(
            bounded_report_response("/api/test", None, body.clone()),
            ReportResponse::Body(body)
        );
    }

    #[test]
    fn large_report_defaults_to_bounded_digest_summary() {
        let body = serde_json::json!({
            "status": "ready",
            "entries": vec!["x".repeat(1024); 256],
        })
        .to_string();
        let ReportResponse::Body(summary) =
            bounded_report_response("/api/test", None, body.clone())
        else {
            panic!("expected report summary");
        };
        let summary: Value = serde_json::from_str(&summary).expect("summary JSON");
        assert_eq!(summary["schema"], "hepta_report_summary_v2");
        assert_eq!(summary["status"], "ready");
        assert_eq!(summary["full_size_bytes"], body.len());
        assert_eq!(
            summary["content_sha256"],
            format!("{:x}", Sha256::digest(body.as_bytes()))
        );
        assert!(
            serde_json::to_vec(&summary).expect("summary bytes").len() <= MAX_DEFAULT_REPORT_BYTES
        );
    }

    #[test]
    fn full_pages_reassemble_exact_report_bytes() {
        let body = serde_json::json!({
            "status": "ready",
            "entries": vec!["x".repeat(1024); 256],
        })
        .to_string();
        let mut cursor = 0;
        let mut reassembled = Vec::new();
        let snapshot = format!("{:x}", Sha256::digest(body.as_bytes()));
        loop {
            let query = format!("detail=full&cursor={cursor}&snapshot={snapshot}");
            let ReportResponse::Body(page) =
                bounded_report_response("/api/test", Some(&query), body.clone())
            else {
                panic!("expected report page");
            };
            assert!(page.len() <= MAX_DEFAULT_REPORT_BYTES);
            let page: Value = serde_json::from_str(&page).expect("page JSON");
            reassembled.extend(
                STANDARD
                    .decode(page["page_data"].as_str().expect("page data"))
                    .expect("base64 page"),
            );
            let Some(next_cursor) = page["next_cursor"].as_u64() else {
                break;
            };
            cursor = usize::try_from(next_cursor).expect("cursor");
        }
        assert_eq!(reassembled, body.as_bytes());
    }

    #[test]
    fn cursor_requires_full_detail_and_stays_bounded() {
        assert_eq!(
            bounded_report_response(
                "/api/test",
                Some("cursor=1"),
                r#"{"status":"ready"}"#.to_string(),
            ),
            ReportResponse::BadRequest("report cursor and snapshot require detail=full")
        );
        assert_eq!(
            bounded_report_response(
                "/api/test",
                Some(&format!(
                    "detail=full&cursor=99&snapshot={:x}",
                    Sha256::digest(br#"{"status":"ready"}"#)
                )),
                r#"{"status":"ready"}"#.to_string(),
            ),
            ReportResponse::BadRequest("report cursor exceeds full response size")
        );
    }

    #[test]
    fn pagination_is_digest_bound_and_preserves_source_status() {
        let body = serde_json::json!({
            "status": "attention",
            "entries": vec!["x".repeat(1024); 256],
        })
        .to_string();
        let ReportResponse::Body(summary) =
            bounded_report_response("/api/test", None, body.clone())
        else {
            panic!("expected summary");
        };
        let summary: Value = serde_json::from_str(&summary).expect("summary JSON");
        assert_eq!(summary["schema"], "hepta_report_summary_v2");
        assert_eq!(summary["status"], "attention");
        let snapshot = summary["content_sha256"].as_str().expect("snapshot");
        assert_eq!(summary["full_detail"]["snapshot"], snapshot);

        let changed = format!("{body} ");
        let response = bounded_report_response(
            "/api/test",
            Some(&format!("detail=full&cursor=0&snapshot={snapshot}")),
            changed,
        );
        assert_eq!(
            response,
            ReportResponse::SnapshotConflict(
                "report snapshot changed; fetch a fresh summary and restart pagination"
            )
        );
    }
}
