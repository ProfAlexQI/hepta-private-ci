def endpoint_path(endpoint: Endpoint, suffix: str) -> str:
    require(suffix.startswith("/"), "endpoint suffix must be absolute")
    return f"{endpoint.path}{suffix}" or "/"


def request_json(
    method: str,
    endpoint: Endpoint,
    suffix: str,
    timeout: float,
    payload: dict[str, Any] | None = None,
) -> HttpResult:
    headers = {"Accept": "application/json"}
    data = None
    if payload is not None:
        data = json.dumps(payload, separators=(",", ":")).encode("utf-8")
        headers["Content-Type"] = "application/json"
    request = urllib.request.Request(
        f"{endpoint.base}{suffix}",
        method=method,
        headers=headers,
        data=data,
    )
    started = time.monotonic_ns()
    try:
        with LOOPBACK_OPENER.open(request, timeout=timeout) as response:
            body = response.read(MAX_HTTP_BODY + 1)
            status_code = response.status
    except urllib.error.HTTPError as error:
        raise QualificationError(f"HTTP status {error.code} from {suffix}") from error
    except (urllib.error.URLError, TimeoutError, OSError) as error:
        raise QualificationError(f"request failed for {suffix}: {error}") from error
    elapsed_ms = max(0, (time.monotonic_ns() - started) // 1_000_000)
    require(len(body) <= MAX_HTTP_BODY, "HTTP response exceeded bounded body limit")
    require(200 <= status_code < 300, f"unexpected HTTP status {status_code}")
    return HttpResult(status=status_code, body=body, elapsed_ms=elapsed_ms)


def parse_object(result: HttpResult, label: str) -> dict[str, Any]:
    try:
        value = json.loads(result.body)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise QualificationError(f"{label} returned invalid JSON") from error
    require(isinstance(value, dict), f"{label} must return a JSON object")
    return value


def body_receipt(result: HttpResult) -> dict[str, Any]:
    return {
        "status": result.status,
        "byte_length": len(result.body),
        "sha256": hashlib.sha256(result.body).hexdigest(),
        "elapsed_ms": result.elapsed_ms,
    }


def service_probe(
    provider: str,
    endpoint: Endpoint,
    model: str,
    timeout: float,
) -> dict[str, Any]:
    if provider == "ollama":
        identity_result = request_json("GET", endpoint, "/api/version", timeout)
        identity = parse_object(identity_result, "Ollama version")
        version = identity.get("version")
        require(isinstance(version, str) and version.strip(), "Ollama version is missing")
        models_result = request_json("GET", endpoint, "/api/tags", timeout)
        models = parse_object(models_result, "Ollama models").get("models")
        require(isinstance(models, list), "Ollama models array is missing")
        names = [entry.get("name") for entry in models if isinstance(entry, dict)]
        response_suffix = "/v1/responses"
        identity_receipt: dict[str, Any] = {
            "version": version,
            "version_response": body_receipt(identity_result),
        }
    elif provider == "lmstudio":
        models_result = request_json("GET", endpoint, "/models", timeout)
        models = parse_object(models_result, "LM Studio models").get("data")
        require(isinstance(models, list), "LM Studio data array is missing")
        names = [entry.get("id") for entry in models if isinstance(entry, dict)]
        response_suffix = "/responses"
        identity_receipt = {}
    else:
        raise QualificationError(f"unsupported provider: {provider}")

    require(model in names, f"requested {provider} model is not pre-installed")
    response_result = request_json(
        "POST",
        endpoint,
        response_suffix,
        timeout,
        {
            "model": model,
            "input": FOLLOWUP_PROMPT,
            "max_output_tokens": 16,
            "stream": False,
        },
    )
    parse_object(response_result, f"{provider} follow-up response")
    return {
        **identity_receipt,
        "model_present": True,
        "models_response": body_receipt(models_result),
        "inference_response": body_receipt(response_result),
    }


def cancellation_probe(
    provider: str,
    endpoint: Endpoint,
    model: str,
    timeout: float,
    cancel_read_bytes: int,
) -> dict[str, Any]:
    require(cancel_read_bytes > 0, "cancel read size must be positive")
    suffix = "/v1/responses" if provider == "ollama" else "/responses"
    payload = json.dumps(
        {
            "model": model,
            "input": CANCEL_PROMPT,
            "max_output_tokens": 1024,
            "stream": True,
        },
        separators=(",", ":"),
    ).encode("utf-8")
    connection = http.client.HTTPConnection(endpoint.host, endpoint.port, timeout=timeout)
    started = time.monotonic_ns()
    prefix = b""
    status_code = 0
    try:
        connection.request(
            "POST",
            endpoint_path(endpoint, suffix),
            body=payload,
            headers={
                "Accept": "text/event-stream",
                "Content-Type": "application/json",
            },
        )
        response = connection.getresponse()
        status_code = response.status
        require(200 <= status_code < 300, f"{provider} cancellation stream status={status_code}")
        prefix = response.read(cancel_read_bytes)
        require(bool(prefix), f"{provider} cancellation stream produced no bytes")
        response.close()
    except (OSError, TimeoutError, http.client.HTTPException) as error:
        raise QualificationError(f"{provider} cancellation probe failed: {error}") from error
    finally:
        connection.close()
    disconnect_ms = max(0, (time.monotonic_ns() - started) // 1_000_000)
    time.sleep(0.2)
    post_health = service_probe(provider, endpoint, model, timeout)
    return {
        "kind": "client_transport_disconnect_v1",
        "stream_status": status_code,
        "prefix_byte_length": len(prefix),
        "prefix_sha256": hashlib.sha256(prefix).hexdigest(),
        "disconnect_elapsed_ms": disconnect_ms,
        "connection_closed": True,
        "backend_acknowledged": False,
        "post_disconnect_health_passed": True,
        "post_disconnect_health": post_health,
    }


