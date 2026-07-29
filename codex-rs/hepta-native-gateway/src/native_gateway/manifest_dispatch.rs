use super::*;

pub(super) fn dispatch_manifest_route(
    stream: &mut TcpStream,
    method: &str,
    path: &str,
    options: &NativeGatewayOptions,
    runtime: &NativeGatewayRuntime,
    request_body: Option<&str>,
    request_query: Option<&str>,
    preflight: &RuntimeRequestPreflightReceipt,
) -> Result<()> {
    let Some(manifest_entry) = route_manifest_entry(method, path) else {
        return http_rejections::runtime_ingress(
            stream,
            method,
            path,
            &anyhow::anyhow!("route manifest entry missing"),
        );
    };
    match manifest_entry.dispatch_handler {
        RouteDispatchHandler::PreferenceIngress => {
            let Some(response) = runtime.route_preference_ingress(
                method,
                path,
                request_body,
                &preflight.request_binding_hash,
            ) else {
                return manifest_dispatch_mismatch(stream);
            };
            write_http_response(
                stream,
                response.status,
                "application/json; charset=utf-8",
                response.body.as_bytes(),
            )
        }
        RouteDispatchHandler::EffectReconciliation => {
            let Some(response) = runtime.route_effect_reconciliation(
                method,
                path,
                request_body,
                &preflight.request_binding_hash,
            ) else {
                return manifest_dispatch_mismatch(stream);
            };
            write_http_response(
                stream,
                response.status,
                "application/json; charset=utf-8",
                response.body.as_bytes(),
            )
        }
        RouteDispatchHandler::TelegramReconciliation => {
            let Some(response) = runtime.route_telegram_reconciliation(
                method,
                path,
                request_body,
                &preflight.request_binding_hash,
            ) else {
                return manifest_dispatch_mismatch(stream);
            };
            write_http_response(
                stream,
                response.status,
                "application/json; charset=utf-8",
                response.body.as_bytes(),
            )
        }
        RouteDispatchHandler::RuntimeKernelCanary => {
            if !runtime_kernel_canary_body_admitted(request_body) {
                return write_http_response(
                    stream,
                    "400 Bad Request",
                    "application/json; charset=utf-8",
                    br#"{"error":"runtime_kernel_canary_requires_exact_dry_run"}"#,
                );
            }
            match runtime.execute_runtime_kernel_canary(&preflight.request_binding_hash) {
                Ok(receipt) => write_http_response(
                    stream,
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&receipt).as_bytes(),
                ),
                Err(error) => {
                    eprintln!("RuntimeKernel canary failed: {error:#}");
                    write_http_response(
                        stream,
                        "503 Service Unavailable",
                        "application/json; charset=utf-8",
                        br#"{"error":"runtime_kernel_canary_failed"}"#,
                    )
                }
            }
        }
        RouteDispatchHandler::RuntimeMutationCanary => {
            if !runtime_mutation_enabled() {
                return write_http_response(
                    stream,
                    "403 Forbidden",
                    "application/json; charset=utf-8",
                    format!(
                        r#"{{"error":"runtime_mutation_canary.disabled","required_gate":"{RUNTIME_MUTATION_CANARY_ENV}","durable_intent_recorded":false,"provider_effect_ack_recorded":false,"terminal_receipt_recorded":false,"filesystem_mutated":false}}"#
                    )
                    .as_bytes(),
                );
            }
            let Some(idempotency_key) = runtime_mutation_body_admitted(request_body) else {
                return write_http_response(
                    stream,
                    "400 Bad Request",
                    "application/json; charset=utf-8",
                    br#"{"error":"runtime_mutation_canary.exact_confirmation_required"}"#,
                );
            };
            match runtime
                .execute_runtime_mutation_canary(&preflight.request_binding_hash, &idempotency_key)
            {
                Ok(receipt) => write_http_response(
                    stream,
                    "200 OK",
                    "application/json; charset=utf-8",
                    json_or_error(&receipt).as_bytes(),
                ),
                Err(error) => {
                    eprintln!("RuntimeKernel mutation canary failed: {error:#}");
                    write_http_response(
                        stream,
                        "503 Service Unavailable",
                        "application/json; charset=utf-8",
                        br#"{"error":"runtime_mutation_canary.failed"}"#,
                    )
                }
            }
        }
        RouteDispatchHandler::OperatorExecution => {
            let Some(response) = operator_execution_response(
                runtime,
                method,
                path,
                request_body,
                &preflight.request_binding_hash,
            ) else {
                return manifest_dispatch_mismatch(stream);
            };
            write_http_response(
                stream,
                response.status,
                "application/json; charset=utf-8",
                response.body.as_bytes(),
            )
        }
        RouteDispatchHandler::TelegramReceiveOnce => {
            let response =
                telegram_receive_once_response(Some(runtime), options.with_telegram_plugin, 20);
            write_http_response(
                stream,
                response.status,
                "application/json; charset=utf-8",
                response.body.as_bytes(),
            )
        }
        RouteDispatchHandler::NativeGateway => {
            if let Some((status, content_type, body)) =
                route_native_gateway_binary_asset(method, path)
            {
                return write_http_response(stream, status, content_type, body);
            }
            let (status, content_type, body) = route_native_gateway_request_with_preflight(
                method,
                path,
                options,
                request_body,
                preflight,
            );
            if manifest_entry
                .report_descriptor()
                .is_some_and(|descriptor| {
                    descriptor.renderer == crate::route_manifest::ReportRenderer::NativeGatewayJson
                        && descriptor.response_policy
                            == crate::route_manifest::RouteResponsePolicy::DigestBoundPagination
                })
                && content_type.starts_with("application/json")
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
                    report_pagination::ReportResponse::SnapshotConflict(error) => {
                        write_http_response(
                            stream,
                            "409 Conflict",
                            "application/json; charset=utf-8",
                            json_or_error(&serde_json::json!({"error": error})).as_bytes(),
                        )
                    }
                };
            }
            write_http_response(stream, status, content_type, body.as_bytes())
        }
        RouteDispatchHandler::RetiredCompatibility => http_rejections::runtime_ingress(
            stream,
            method,
            path,
            &anyhow::anyhow!("retired compatibility route"),
        ),
    }
}

fn manifest_dispatch_mismatch(stream: &mut TcpStream) -> Result<()> {
    http_rejections::response(
        stream,
        "503 Service Unavailable",
        "application/json; charset=utf-8",
        br#"{"error":"route_manifest.dispatch_mismatch"}"#,
    )
}
