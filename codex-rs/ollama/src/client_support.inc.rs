fn decode_pull_frame(frame: &[u8]) -> Result<Vec<PullEvent>, String> {
    if frame.len() > MAX_PULL_FRAME_BYTES {
        return Err(format!(
            "OLLAMA_PULL_FRAME_TOO_LARGE: maximum={MAX_PULL_FRAME_BYTES}"
        ));
    }
    let frame = frame.strip_suffix(b"\n").unwrap_or(frame);
    let frame = frame.strip_suffix(b"\r").unwrap_or(frame);
    if frame.is_empty() {
        // Blank keepalive frames are valid no-op frames; this is unrelated to
        // model-discovery failure handling, which remains fail-closed.
        return Ok(Vec::default());
    }

    let text = std::str::from_utf8(frame).map_err(|_| "OLLAMA_PULL_INVALID_UTF8".to_string())?;
    let value = serde_json::from_str::<JsonValue>(text)
        .map_err(|_| "OLLAMA_PULL_INVALID_JSON".to_string())?;
    if let Some(error) = value.get("error").and_then(JsonValue::as_str) {
        return Err(format!(
            "OLLAMA_PULL_SERVER_ERROR: {}",
            sanitize_remote_message(error)
        ));
    }

    let events = pull_events_from_value(&value);
    if events.is_empty() {
        return Err("OLLAMA_PULL_UNRECOGNIZED_EVENT".to_string());
    }
    Ok(events)
}

pub(crate) fn validate_model_identifier(model: &str) -> io::Result<()> {
    if model.is_empty()
        || model.len() > 512
        || model != model.trim()
        || model.chars().any(char::is_control)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "OLLAMA_INVALID_MODEL_IDENTIFIER",
        ));
    }
    Ok(())
}

async fn read_bounded_control_body(
    mut response: codex_http_client::HttpResponse,
    operation: &str,
) -> io::Result<Vec<u8>> {
    if response
        .content_length()
        .is_some_and(|length| length > MAX_CONTROL_RESPONSE_BYTES as u64)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(concat!(
                "OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={operation} ",
                "maximum={MAX_CONTROL_RESPONSE_BYTES}"
            )),
        ));
    }
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| request_error(operation, error))?
    {
        let next_len = body.len().checked_add(chunk.len()).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={operation}"),
            )
        })?;
        if next_len > MAX_CONTROL_RESPONSE_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(concat!(
                    "OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={operation} ",
                    "maximum={MAX_CONTROL_RESPONSE_BYTES}"
                )),
            ));
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

fn validate_loopback_http_base_url(base_url: &str) -> io::Result<()> {
    if base_url.is_empty()
        || base_url != base_url.trim()
        || base_url.chars().any(char::is_control)
        || base_url.contains('?')
        || base_url.contains('#')
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "OLLAMA_BASE_URL_INVALID",
        ));
    }
    let remainder = base_url.strip_prefix("http://").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::PermissionDenied,
            "OLLAMA_BASE_URL_NOT_LOOPBACK_HTTP",
        )
    })?;
    let authority = remainder.split('/').next().unwrap_or_default();
    if authority.is_empty() || authority.contains('@') {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "OLLAMA_BASE_URL_NOT_LOOPBACK_HTTP",
        ));
    }
    let path = &remainder[authority.len()..];
    if !matches!(path, "" | "/" | "/v1" | "/v1/") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "OLLAMA_BASE_URL_PATH_UNSUPPORTED",
        ));
    }

    let (host, port) = parse_authority(authority, "OLLAMA_BASE_URL_NOT_LOOPBACK_HTTP")?;
    if !matches!(host, "localhost" | "127.0.0.1" | "::1") {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "OLLAMA_BASE_URL_NOT_LOOPBACK_HTTP",
        ));
    }
    require_loopback_resolution(host, port, "OLLAMA_BASE_URL_NOT_LOOPBACK_HTTP")
}

fn parse_authority<'a>(authority: &'a str, code: &str) -> io::Result<(&'a str, u16)> {
    let (host, port) = if let Some(ipv6) = authority.strip_prefix('[') {
        let closing = ipv6
            .find(']')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, code.to_string()))?;
        let host = &ipv6[..closing];
        let suffix = &ipv6[closing + 1..];
        let port = suffix
            .strip_prefix(':')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, code.to_string()))?;
        (host, port)
    } else {
        let (host, port) = authority
            .rsplit_once(':')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, code.to_string()))?;
        if host.contains(':') {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                code.to_string(),
            ));
        }
        (host, port)
    };
    let port = port
        .parse::<u16>()
        .ok()
        .filter(|port| *port != 0)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, code.to_string()))?;
    Ok((host, port))
}

fn require_loopback_resolution(host: &str, port: u16, code: &str) -> io::Result<()> {
    let authority = if host.contains(':') {
        format!("[{host}]:{port}")
    } else {
        format!("{host}:{port}")
    };
    let addresses = authority
        .to_socket_addrs()
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, format!("{code}: {error}")))?;
    let mut resolved = false;
    for address in addresses {
        resolved = true;
        if !is_loopback_ip(address.ip()) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                code.to_string(),
            ));
        }
    }
    if !resolved {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            code.to_string(),
        ));
    }
    Ok(())
}

fn is_loopback_ip(address: IpAddr) -> bool {
    address.is_loopback()
}

fn request_error(operation: &str, error: impl std::fmt::Display) -> io::Error {
    io::Error::other(format!(
        "OLLAMA_REQUEST_ERROR operation={operation}: {}",
        sanitize_remote_message(&error.to_string())
    ))
}

fn status_error(operation: &str, status: u16) -> io::Error {
    io::Error::other(format!(
        "OLLAMA_HTTP_STATUS operation={operation} status={status}"
    ))
}

fn invalid_payload(operation: &str, error: impl std::fmt::Display) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "OLLAMA_INVALID_JSON operation={operation}: {}",
            sanitize_remote_message(&error.to_string())
        ),
    )
}

fn invalid_shape(operation: &str, reason: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("OLLAMA_INVALID_PAYLOAD operation={operation}: {reason}"),
    )
}

fn sanitize_remote_message(message: &str) -> String {
    let mut sanitized = String::new();
    let mut written = 0usize;
    for character in message.chars() {
        if written >= MAX_REMOTE_ERROR_CHARS {
            break;
        }
        if character.is_control() {
            if character.is_whitespace() {
                sanitized.push(' ');
                written += 1;
            }
        } else {
            sanitized.push(character);
            written += 1;
        }
    }
    sanitized
}
