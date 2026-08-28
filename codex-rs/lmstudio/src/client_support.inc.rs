fn validate_loopback_http_base_url(base_url: &str) -> io::Result<()> {
    if base_url.is_empty()
        || base_url != base_url.trim()
        || base_url.chars().any(char::is_control)
        || base_url.contains('?')
        || base_url.contains('#')
    {
        return Err(coded_error(
            io::ErrorKind::InvalidInput,
            "LMSTUDIO_BASE_URL_INVALID",
            "base URL is empty, padded, contains control characters, query, or fragment",
        ));
    }
    let remainder = base_url.strip_prefix("http://").ok_or_else(|| {
        coded_error(
            io::ErrorKind::PermissionDenied,
            "LMSTUDIO_BASE_URL_NOT_LOOPBACK_HTTP",
            "only loopback HTTP endpoints are allowed",
        )
    })?;
    let authority = remainder.split('/').next().unwrap_or_default();
    if authority.is_empty() || authority.contains('@') {
        return Err(coded_error(
            io::ErrorKind::PermissionDenied,
            "LMSTUDIO_BASE_URL_NOT_LOOPBACK_HTTP",
            "userinfo and empty authorities are forbidden",
        ));
    }
    let path = &remainder[authority.len()..];
    if !matches!(path, "" | "/" | "/v1" | "/v1/") {
        return Err(coded_error(
            io::ErrorKind::InvalidInput,
            "LMSTUDIO_BASE_URL_PATH_UNSUPPORTED",
            "only the host root or /v1 is supported",
        ));
    }
    let (host, port) = parse_authority(authority, "LMSTUDIO_BASE_URL_NOT_LOOPBACK_HTTP")?;
    if !matches!(host, "localhost" | "127.0.0.1" | "::1") {
        return Err(coded_error(
            io::ErrorKind::PermissionDenied,
            "LMSTUDIO_BASE_URL_NOT_LOOPBACK_HTTP",
            "only localhost, 127.0.0.1, or ::1 is allowed",
        ));
    }
    require_loopback_resolution(host, port, "LMSTUDIO_BASE_URL_NOT_LOOPBACK_HTTP")
}

fn parse_authority<'a>(authority: &'a str, code: &str) -> io::Result<(&'a str, u16)> {
    let (host, port) = if let Some(ipv6) = authority.strip_prefix('[') {
        let closing = ipv6.find(']').ok_or_else(|| {
            coded_error(io::ErrorKind::InvalidInput, code, "invalid IPv6 authority")
        })?;
        let host = &ipv6[..closing];
        let suffix = &ipv6[closing + 1..];
        let port = suffix.strip_prefix(':').ok_or_else(|| {
            coded_error(io::ErrorKind::InvalidInput, code, "missing port")
        })?;
        (host, port)
    } else {
        let (host, port) = authority.rsplit_once(':').ok_or_else(|| {
            coded_error(io::ErrorKind::InvalidInput, code, "missing port")
        })?;
        if host.contains(':') {
            return Err(coded_error(
                io::ErrorKind::InvalidInput,
                code,
                "IPv6 authorities must use brackets",
            ));
        }
        (host, port)
    };
    let port = port
        .parse::<u16>()
        .ok()
        .filter(|port| *port != 0)
        .ok_or_else(|| coded_error(io::ErrorKind::InvalidInput, code, "invalid port"))?;
    Ok((host, port))
}

fn require_loopback_resolution(host: &str, port: u16, code: &str) -> io::Result<()> {
    let authority = if host.contains(':') {
        format!("[{host}]:{port}")
    } else {
        format!("{host}:{port}")
    };
    let addresses = authority.to_socket_addrs().map_err(|error| {
        coded_error(
            io::ErrorKind::InvalidInput,
            code,
            sanitize_diagnostic(&error.to_string()),
        )
    })?;
    let mut resolved = false;
    for address in addresses {
        resolved = true;
        if !is_loopback_ip(address.ip()) {
            return Err(coded_error(
                io::ErrorKind::PermissionDenied,
                code,
                "endpoint resolved outside loopback",
            ));
        }
    }
    if !resolved {
        return Err(coded_error(
            io::ErrorKind::InvalidInput,
            code,
            "endpoint resolved to no addresses",
        ));
    }
    Ok(())
}

fn is_loopback_ip(address: IpAddr) -> bool {
    address.is_loopback()
}

fn canonical_executable(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let canonical = std::fs::canonicalize(path.as_ref()).map_err(|error| {
        coded_error(
            io::ErrorKind::NotFound,
            "LMSTUDIO_CLI_CANONICALIZE_FAILED",
            sanitize_diagnostic(&error.to_string()),
        )
    })?;
    let metadata = std::fs::metadata(&canonical).map_err(|error| {
        coded_error(
            io::ErrorKind::NotFound,
            "LMSTUDIO_CLI_METADATA_FAILED",
            sanitize_diagnostic(&error.to_string()),
        )
    })?;
    if !metadata.is_file() {
        return Err(coded_error(
            io::ErrorKind::InvalidData,
            "LMSTUDIO_CLI_NOT_REGULAR_FILE",
            canonical.display().to_string(),
        ));
    }
    Ok(canonical)
}

fn parse_sha256_binding(binding: &str) -> io::Result<String> {
    let digest = binding.strip_prefix("sha256:").ok_or_else(|| {
        coded_error(
            io::ErrorKind::InvalidInput,
            "LMSTUDIO_CLI_DIGEST_INVALID",
            "expected sha256:<64 lowercase hex>",
        )
    })?;
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(coded_error(
            io::ErrorKind::InvalidInput,
            "LMSTUDIO_CLI_DIGEST_INVALID",
            "expected sha256:<64 lowercase hex>",
        ));
    }
    Ok(digest.to_string())
}

pub(crate) fn validate_model_identifier(model: &str) -> io::Result<()> {
    if model.is_empty()
        || model.len() > 512
        || model != model.trim()
        || model.chars().any(char::is_control)
    {
        return Err(coded_error(
            io::ErrorKind::InvalidInput,
            "LMSTUDIO_INVALID_MODEL_IDENTIFIER",
            "model identifier is empty, oversized, padded, or contains control characters",
        ));
    }
    Ok(())
}

fn coded_error(kind: io::ErrorKind, code: &str, detail: impl AsRef<str>) -> io::Error {
    io::Error::new(kind, format!("{code}: {}", detail.as_ref()))
}

fn sanitize_diagnostic(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars().take(MAX_STDERR_BYTES) {
        if character.is_control() {
            if character.is_whitespace() {
                output.push(' ');
            }
        } else {
            output.push(character);
        }
    }
    output
}
