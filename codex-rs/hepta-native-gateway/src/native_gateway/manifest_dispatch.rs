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
    legacy_route_usage::record_direct_call(manifest_entry);
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
            write_json_response(stream, response.status, &response.body)
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
            write_json_response(stream, response.status, &response.body)
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
            write_json_response(stream, response.status, &response.body)
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
            write_json_response(stream, response.status, &response.body)
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
        RouteDispatchHandler::NduH1Status => match runtime.ndu_h1_status() {
            Ok(status) => write_http_response(
                stream,
                "200 OK",
                "application/json; charset=utf-8",
                json_or_error(&status).as_bytes(),
            ),
            Err(error) => {
                eprintln!("NDU H1 status failed: {error:#}");
                write_http_response(
                    stream,
                    "503 Service Unavailable",
                    "application/json; charset=utf-8",
                    br#"{"error":"ndu_h1_shadow.status_unavailable"}"#,
                )
            }
        },
        RouteDispatchHandler::EvidenceIndex => evidence_dispatch::dispatch_evidence_route(
            stream,
            path,
            options,
            request_query,
            preflight,
            manifest_entry,
        ),
        RouteDispatchHandler::NativeGateway => {
            if let Some(asset) = route_native_gateway_binary_asset(method, path) {
                return write_http_asset_response(
                    stream,
                    "200 OK",
                    asset.content_type,
                    asset.cache_control,
                    asset.etag,
                    asset.body,
                );
            }
            let (status, content_type, body) = route_native_gateway_request_with_preflight(
                method,
                path,
                options,
                request_body,
                preflight,
            );
            report_response::write_typed_report_response(
                stream,
                path,
                request_query,
                status,
                content_type,
                body,
                manifest_entry,
            )
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

fn write_json_response(stream: &mut TcpStream, status: &str, body: &str) -> Result<()> {
    write_http_response(
        stream,
        status,
        "application/json; charset=utf-8",
        body.as_bytes(),
    )
}
