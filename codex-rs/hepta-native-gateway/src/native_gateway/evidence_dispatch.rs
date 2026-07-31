use super::*;

pub(super) fn dispatch_evidence_route(
    stream: &mut TcpStream,
    path: &str,
    options: &NativeGatewayOptions,
    request_query: Option<&str>,
    preflight: &RuntimeRequestPreflightReceipt,
    manifest_entry: crate::route_manifest::RouteDefinition,
) -> Result<()> {
    let body = match evidence_api::requested_evidence_selector(request_query) {
        Ok(None) => json_or_error(&evidence_api::evidence_index_report()),
        Ok(Some(selector)) => {
            let Some(definition) =
                evidence_api::evidence_definition(selector).filter(|definition| {
                    definition.legacy_compatibility_route
                        && definition.dispatch_handler == RouteDispatchHandler::NativeGateway
                })
            else {
                return write_http_response(
                    stream,
                    "404 Not Found",
                    "application/json; charset=utf-8",
                    br#"{"error":"evidence route not found"}"#,
                );
            };
            let (source_status, source_content_type, source_body) =
                route_native_gateway_request_after_preflight(
                    "GET",
                    definition.lifecycle.path_pattern,
                    options,
                    None,
                    preflight,
                );
            match evidence_api::evidence_document_report(
                definition,
                source_status,
                source_content_type,
                source_body,
            ) {
                Ok(document) => json_or_error(&document),
                Err(error) => {
                    return write_http_response(
                        stream,
                        "502 Bad Gateway",
                        "application/json; charset=utf-8",
                        json_or_error(&serde_json::json!({"error": error})).as_bytes(),
                    );
                }
            }
        }
        Err(error) => {
            return write_http_response(
                stream,
                "400 Bad Request",
                "application/json; charset=utf-8",
                json_or_error(&serde_json::json!({"error": error})).as_bytes(),
            );
        }
    };
    report_response::write_typed_report_response(
        stream,
        path,
        request_query,
        "200 OK",
        "application/json; charset=utf-8",
        body,
        manifest_entry,
    )
}
