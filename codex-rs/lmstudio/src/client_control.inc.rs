async fn read_bounded_control_body(
    mut response: codex_http_client::HttpResponse,
    operation: &str,
) -> io::Result<Vec<u8>> {
    if response
        .content_length()
        .is_some_and(|length| length > MAX_CONTROL_RESPONSE_BYTES as u64)
    {
        return Err(coded_error(
            io::ErrorKind::InvalidData,
            "LMSTUDIO_CONTROL_RESPONSE_TOO_LARGE",
            format!("operation={operation} maximum={MAX_CONTROL_RESPONSE_BYTES}"),
        ));
    }
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|error| {
        coded_error(
            io::ErrorKind::ConnectionAborted,
            "LMSTUDIO_CONTROL_RESPONSE_READ_FAILED",
            sanitize_diagnostic(&error.to_string()),
        )
    })? {
        let next_len = body.len().checked_add(chunk.len()).ok_or_else(|| {
            coded_error(
                io::ErrorKind::InvalidData,
                "LMSTUDIO_CONTROL_RESPONSE_TOO_LARGE",
                format!("operation={operation}"),
            )
        })?;
        if next_len > MAX_CONTROL_RESPONSE_BYTES {
            return Err(coded_error(
                io::ErrorKind::InvalidData,
                "LMSTUDIO_CONTROL_RESPONSE_TOO_LARGE",
                format!("operation={operation} maximum={MAX_CONTROL_RESPONSE_BYTES}"),
            ));
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}
