use super::*;

pub(super) fn write_typed_report_response(
    stream: &mut TcpStream,
    path: &str,
    request_query: Option<&str>,
    status: &'static str,
    content_type: &'static str,
    body: String,
    definition: crate::route_manifest::RouteDefinition,
) -> Result<()> {
    if definition.report_descriptor().is_some_and(|descriptor| {
        descriptor.response_policy
            == crate::route_manifest::RouteResponsePolicy::DigestBoundPagination
    }) && content_type.starts_with("application/json")
    {
        return match report_pagination::bounded_report_response(path, request_query, body) {
            report_pagination::ReportResponse::Body(body) => {
                write_http_response(stream, status, content_type, body.as_bytes())
            }
            report_pagination::ReportResponse::BadRequest(error) => write_http_response(
                stream,
                "400 Bad Request",
                "application/json; charset=utf-8",
                json_or_error(&serde_json::json!({"error": error})).as_bytes(),
            ),
            report_pagination::ReportResponse::InternalError(error) => write_http_response(
                stream,
                "500 Internal Server Error",
                "application/json; charset=utf-8",
                json_or_error(&serde_json::json!({"error": error})).as_bytes(),
            ),
            report_pagination::ReportResponse::SnapshotConflict(error) => write_http_response(
                stream,
                "409 Conflict",
                "application/json; charset=utf-8",
                json_or_error(&serde_json::json!({"error": error})).as_bytes(),
            ),
        };
    }
    write_http_response(stream, status, content_type, body.as_bytes())
}
