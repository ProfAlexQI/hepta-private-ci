use std::fmt;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::str;
use std::time::Duration;
use std::time::Instant;

use anyhow::Context;
use anyhow::Result;
use hepta_gateway::NATIVE_POST_MAX_BODY_BYTES;

pub(crate) const MAX_HTTP_HEADER_BYTES: usize = 32 * 1024;
pub(crate) const MAX_HTTP_BODY_BYTES: usize = NATIVE_POST_MAX_BODY_BYTES;
pub(crate) const HTTP_READ_TIMEOUT: Duration = Duration::from_secs(5);
pub(crate) const HTTP_WRITE_TIMEOUT: Duration = Duration::from_secs(5);
pub(crate) const HTTP_REQUEST_DEADLINE: Duration = Duration::from_secs(10);
pub(crate) const HTTP_RESPONSE_DEADLINE: Duration = Duration::from_secs(10);
pub(crate) const HTTP_OVERLOAD_WRITE_TIMEOUT: Duration = Duration::from_millis(250);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HttpRequestError {
    RequestTimeout,
    HeaderTooLarge,
    PayloadTooLarge,
    BadRequest(&'static str),
}

impl HttpRequestError {
    pub(crate) fn status(self) -> &'static str {
        match self {
            Self::RequestTimeout => "408 Request Timeout",
            Self::HeaderTooLarge => "431 Request Header Fields Too Large",
            Self::PayloadTooLarge => "413 Payload Too Large",
            Self::BadRequest(_) => "400 Bad Request",
        }
    }

    pub(crate) fn response_body(self) -> &'static [u8] {
        match self {
            Self::RequestTimeout => br#"{"error":"request timeout"}"#,
            Self::HeaderTooLarge => br#"{"error":"request headers too large"}"#,
            Self::PayloadTooLarge => br#"{"error":"request body too large"}"#,
            Self::BadRequest(_) => br#"{"error":"bad request"}"#,
        }
    }
}

impl fmt::Display for HttpRequestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestTimeout => formatter.write_str("request timed out"),
            Self::HeaderTooLarge => formatter.write_str("request headers exceeded limit"),
            Self::PayloadTooLarge => formatter.write_str("request body exceeded limit"),
            Self::BadRequest(reason) => write!(formatter, "bad request: {reason}"),
        }
    }
}

pub(crate) fn configure_http_stream(stream: &TcpStream) -> Result<()> {
    stream
        .set_read_timeout(Some(HTTP_READ_TIMEOUT))
        .context("set native gateway HTTP read timeout")?;
    stream
        .set_write_timeout(Some(HTTP_WRITE_TIMEOUT))
        .context("set native gateway HTTP write timeout")?;
    Ok(())
}

pub(crate) fn read_http_request_with_deadline(
    stream: &mut TcpStream,
    deadline: Instant,
) -> Result<String, HttpRequestError> {
    read_http_request_with(|buffer| read_stream_bytes_before_deadline(stream, deadline, buffer))
}

#[cfg(test)]
fn read_http_request_from_reader(reader: &mut impl Read) -> Result<String, HttpRequestError> {
    read_http_request_with(|buffer| read_request_bytes(reader, buffer))
}

fn read_http_request_with(
    mut read_bytes: impl FnMut(&mut [u8]) -> Result<usize, HttpRequestError>,
) -> Result<String, HttpRequestError> {
    let mut buffer = [0_u8; 8192];
    let mut bytes = Vec::with_capacity(8192);
    let header_end = loop {
        if let Some(index) = find_header_end(&bytes) {
            break index;
        }
        if bytes.len() >= MAX_HTTP_HEADER_BYTES {
            return Err(HttpRequestError::HeaderTooLarge);
        }

        let remaining = MAX_HTTP_HEADER_BYTES - bytes.len();
        let read_capacity = remaining.min(buffer.len());
        let read = read_bytes(&mut buffer[..read_capacity])?;
        if read == 0 {
            return Err(HttpRequestError::BadRequest("incomplete request headers"));
        }
        bytes.extend_from_slice(&buffer[..read]);
    };

    let header_bytes = &bytes[..header_end];
    let header = str::from_utf8(header_bytes)
        .map_err(|_| HttpRequestError::BadRequest("headers are not valid UTF-8"))?;
    let content_length = parse_content_length(header)?;
    if content_length > MAX_HTTP_BODY_BYTES {
        return Err(HttpRequestError::PayloadTooLarge);
    }

    let body_start = header_end
        .checked_add(4)
        .ok_or(HttpRequestError::HeaderTooLarge)?;
    let request_end = body_start
        .checked_add(content_length)
        .ok_or(HttpRequestError::PayloadTooLarge)?;
    while bytes.len() < request_end {
        let remaining = request_end - bytes.len();
        let read_capacity = remaining.min(buffer.len());
        let read = read_bytes(&mut buffer[..read_capacity])?;
        if read == 0 {
            return Err(HttpRequestError::BadRequest("incomplete request body"));
        }
        bytes.extend_from_slice(&buffer[..read]);
    }

    // The gateway closes every response, so never admit pipelined bytes as part of
    // the first request body.
    bytes.truncate(request_end);
    String::from_utf8(bytes)
        .map_err(|_| HttpRequestError::BadRequest("request body is not valid UTF-8"))
}

fn read_stream_bytes_before_deadline(
    stream: &mut TcpStream,
    deadline: Instant,
    buffer: &mut [u8],
) -> Result<usize, HttpRequestError> {
    let remaining = deadline
        .checked_duration_since(Instant::now())
        .filter(|remaining| !remaining.is_zero())
        .ok_or(HttpRequestError::RequestTimeout)?;
    stream
        .set_read_timeout(Some(remaining.min(HTTP_READ_TIMEOUT)))
        .map_err(|_| HttpRequestError::BadRequest("failed to apply request deadline"))?;
    read_request_bytes(stream, buffer)
}

fn read_request_bytes(
    reader: &mut impl Read,
    buffer: &mut [u8],
) -> Result<usize, HttpRequestError> {
    loop {
        match reader.read(buffer) {
            Ok(read) => return Ok(read),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock
                ) =>
            {
                return Err(HttpRequestError::RequestTimeout);
            }
            Err(_) => return Err(HttpRequestError::BadRequest("request read failed")),
        }
    }
}

fn find_header_end(bytes: &[u8]) -> Option<usize> {
    bytes.windows(4).position(|window| window == b"\r\n\r\n")
}

fn parse_content_length(header: &str) -> Result<usize, HttpRequestError> {
    let mut lines = header.split("\r\n");
    let request_line = lines
        .next()
        .ok_or(HttpRequestError::BadRequest("missing request line"))?;
    validate_request_line(request_line)?;

    let mut content_length = None;
    for line in lines {
        if line.starts_with(' ') || line.starts_with('\t') {
            return Err(HttpRequestError::BadRequest(
                "obsolete folded headers are not supported",
            ));
        }
        let (name, value) = line
            .split_once(':')
            .ok_or(HttpRequestError::BadRequest("malformed header"))?;
        if name.is_empty() || name.bytes().any(|byte| !is_http_token_byte(byte)) {
            return Err(HttpRequestError::BadRequest("invalid header name"));
        }
        if name.eq_ignore_ascii_case("transfer-encoding") {
            return Err(HttpRequestError::BadRequest(
                "transfer-encoding is not supported",
            ));
        }
        if name.eq_ignore_ascii_case("content-length") {
            if content_length.is_some() {
                return Err(HttpRequestError::BadRequest(
                    "duplicate content-length header",
                ));
            }
            let value = value.trim_matches([' ', '\t']);
            if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(HttpRequestError::BadRequest("invalid content-length"));
            }
            content_length = Some(
                value
                    .parse::<usize>()
                    .map_err(|_| HttpRequestError::BadRequest("invalid content-length"))?,
            );
        }
    }
    Ok(content_length.unwrap_or(0))
}

fn validate_request_line(line: &str) -> Result<(), HttpRequestError> {
    let mut parts = line.split_whitespace();
    let method = parts.next();
    let target = parts.next();
    let version = parts.next();
    if method.is_none() || target.is_none() || parts.next().is_some() {
        return Err(HttpRequestError::BadRequest("malformed request line"));
    }
    if !matches!(version, Some("HTTP/1.1" | "HTTP/1.0")) {
        return Err(HttpRequestError::BadRequest("unsupported HTTP version"));
    }
    if !target.is_some_and(|target| target.starts_with('/')) {
        return Err(HttpRequestError::BadRequest("invalid request target"));
    }
    Ok(())
}

fn is_http_token_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        )
}

pub(crate) fn request_body_text(request: &str) -> Option<&str> {
    request
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .filter(|body| !body.is_empty())
}

pub(crate) fn request_method_and_path(request: &str) -> Option<(&str, &str)> {
    let first_line = request.lines().next()?;
    let mut parts = first_line.split_whitespace();
    let method = parts.next()?;
    let raw_path = parts.next()?;
    Some((method, raw_path.split('?').next().unwrap_or(raw_path)))
}

pub(crate) fn request_query(request: &str) -> Option<&str> {
    let first_line = request.lines().next()?;
    let mut parts = first_line.split_whitespace();
    parts.next()?;
    parts.next()?.split_once('?').map(|(_, query)| query)
}

pub(crate) fn write_http_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> Result<()> {
    write_http_response_with_timeout(stream, status, content_type, body, HTTP_RESPONSE_DEADLINE)
}

pub(crate) fn write_http_response_with_timeout(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
    timeout: Duration,
) -> Result<()> {
    let deadline = Instant::now()
        .checked_add(timeout)
        .context("native gateway HTTP response deadline overflow")?;
    write_http_response_before_deadline(stream, status, content_type, body, deadline)
}

fn write_http_response_before_deadline(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
    deadline: Instant,
) -> Result<()> {
    let header = format!(
        "HTTP/1.1 {status}\r\ncontent-type: {content_type}\r\ncontent-length: {}\r\nconnection: close\r\nContent-Security-Policy: default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; base-uri 'none'; frame-ancestors 'none'\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nX-Frame-Options: DENY\r\n\r\n",
        body.len()
    );
    write_http_bytes_before_deadline(stream, header.as_bytes(), deadline)
        .context("write header")?;
    write_http_bytes_before_deadline(stream, body, deadline).context("write body")?;
    flush_http_stream_before_deadline(stream, deadline).context("flush response")?;
    Ok(())
}

fn write_http_bytes_before_deadline(
    stream: &mut TcpStream,
    mut bytes: &[u8],
    deadline: Instant,
) -> Result<()> {
    while !bytes.is_empty() {
        apply_http_write_deadline(stream, deadline)?;
        match stream.write(bytes) {
            Ok(0) => return Err(io::Error::from(io::ErrorKind::WriteZero).into()),
            Ok(written) => bytes = &bytes[written..],
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock
                ) =>
            {
                anyhow::bail!("native gateway HTTP response timed out")
            }
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

fn flush_http_stream_before_deadline(stream: &mut TcpStream, deadline: Instant) -> Result<()> {
    loop {
        apply_http_write_deadline(stream, deadline)?;
        match stream.flush() {
            Ok(()) => return Ok(()),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock
                ) =>
            {
                anyhow::bail!("native gateway HTTP response timed out")
            }
            Err(error) => return Err(error.into()),
        }
    }
}

fn apply_http_write_deadline(stream: &TcpStream, deadline: Instant) -> Result<()> {
    let remaining = deadline
        .checked_duration_since(Instant::now())
        .filter(|remaining| !remaining.is_zero())
        .context("native gateway HTTP response absolute deadline exceeded")?;
    stream
        .set_write_timeout(Some(remaining.min(HTTP_WRITE_TIMEOUT)))
        .context("apply native gateway HTTP response deadline")
}

pub(crate) fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
#[path = "../tests/unit/http_transport.rs"]
mod tests;
