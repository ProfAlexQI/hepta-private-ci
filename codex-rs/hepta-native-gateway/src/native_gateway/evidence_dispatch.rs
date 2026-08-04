use super::*;

pub(super) fn dispatch_evidence_route(
    stream: &mut TcpStream,
    path: &str,
    options: &NativeGatewayOptions,
    request_query: Option<&str>,
    manifest_entry: crate::route_manifest::RouteDefinition,
) -> Result<()> {
    let body = match evidence_api::requested_evidence_selector(request_query) {
        Ok(None) => json_or_error(&evidence_api::evidence_index_report()),
        Ok(Some(selector)) => {
            let Some(definition) =
                evidence_api::evidence_definition(selector).filter(|definition| {
                    definition.legacy_compatibility_route
                        && definition.method == "GET"
                        && !definition.route_selector.contains('<')
                        && definition.renderer_key.is_some()
                })
            else {
                return write_http_response(
                    stream,
                    "404 Not Found",
                    "application/json; charset=utf-8",
                    br#"{"error":"evidence route not found"}"#,
                );
            };
            let telegram_plugin = native_telegram::telegram_plugin_status(
                options.with_telegram_plugin,
                options.telegram_plugin_poll_ms,
            );
            let Some((source_status, source_content_type, source_body)) =
                native_report_registry::render_registered_evidence_report(
                    definition
                        .renderer_key
                        .expect("checked evidence renderer key"),
                    options,
                    telegram_plugin,
                )
            else {
                return write_http_response(
                    stream,
                    "502 Bad Gateway",
                    "application/json; charset=utf-8",
                    br#"{"error":"evidence renderer not found"}"#,
                );
            };
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
