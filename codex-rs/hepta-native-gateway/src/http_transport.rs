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
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::net::TcpListener;
    use std::thread;

    #[test]
    fn parses_request_target_and_bounded_body_without_query_leakage() {
        let mut request = Cursor::new(
            b"POST /api/test?secret=redacted HTTP/1.1\r\ncontent-length: 7\r\n\r\npayloadignored"
                .to_vec(),
        );
        let request = read_http_request_from_reader(&mut request).expect("request");

        assert_eq!(
            request_method_and_path(&request),
            Some(("POST", "/api/test"))
        );
        assert_eq!(request_body_text(&request), Some("payload"));
        assert!(!request.contains("ignored"));
    }

    #[test]
    fn rejects_oversized_headers_with_431() {
        let request = format!(
            "GET / HTTP/1.1\r\nx-padding: {}\r\n\r\n",
            "a".repeat(MAX_HTTP_HEADER_BYTES)
        );
        let error = read_http_request_from_reader(&mut Cursor::new(request.into_bytes()))
            .expect_err("oversized header");

        assert_eq!(error, HttpRequestError::HeaderTooLarge);
        assert_eq!(error.status(), "431 Request Header Fields Too Large");
    }

    #[test]
    fn rejects_oversized_declared_body_with_413_before_reading_body() {
        let request = format!(
            "POST /api/test HTTP/1.1\r\ncontent-length: {}\r\n\r\n",
            MAX_HTTP_BODY_BYTES + 1
        );
        let error = read_http_request_from_reader(&mut Cursor::new(request.into_bytes()))
            .expect_err("oversized body");

        assert_eq!(error, HttpRequestError::PayloadTooLarge);
        assert_eq!(error.status(), "413 Payload Too Large");
    }

    #[test]
    fn rejects_ambiguous_body_framing() {
        for request in [
            "POST / HTTP/1.1\r\ncontent-length: 1\r\ncontent-length: 1\r\n\r\na",
            "POST / HTTP/1.1\r\ntransfer-encoding: chunked\r\n\r\n0\r\n\r\n",
        ] {
            let error = read_http_request_from_reader(&mut Cursor::new(request.as_bytes()))
                .expect_err("ambiguous framing");
            assert!(matches!(error, HttpRequestError::BadRequest(_)));
            assert_eq!(error.status(), "400 Bad Request");
        }
    }

    #[test]
    fn content_length_accepts_only_ascii_digits_with_http_ows() {
        let mut accepted =
            Cursor::new(b"POST / HTTP/1.1\r\ncontent-length: \t1 \t\r\n\r\na".as_slice());
        assert!(read_http_request_from_reader(&mut accepted).is_ok());

        for value in ["+1", "-1", "1 1", "\u{00a0}1", "1\u{000b}", ""] {
            let request = format!("POST / HTTP/1.1\r\ncontent-length: {value}\r\n\r\na");
            let error = read_http_request_from_reader(&mut Cursor::new(request.into_bytes()))
                .expect_err("non-canonical content-length");
            assert!(
                matches!(error, HttpRequestError::BadRequest(_)),
                "{value:?}"
            );
        }
    }

    #[test]
    fn maps_socket_timeout_to_408() {
        struct TimeoutReader;
        impl Read for TimeoutReader {
            fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::TimedOut, "test timeout"))
            }
        }

        let error = read_http_request_from_reader(&mut TimeoutReader).expect_err("timeout");
        assert_eq!(error, HttpRequestError::RequestTimeout);
        assert_eq!(error.status(), "408 Request Timeout");
    }

    #[test]
    fn rejects_request_when_absolute_deadline_is_already_expired() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
        let client = TcpStream::connect(listener.local_addr().expect("address")).expect("client");
        let (mut server, _) = listener.accept().expect("server");

        let error = read_http_request_with_deadline(&mut server, Instant::now())
            .expect_err("expired deadline");
        assert_eq!(error, HttpRequestError::RequestTimeout);
        drop(client);
    }

    #[test]
    fn absolute_deadline_stops_a_client_that_drips_before_each_idle_timeout() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
        let address = listener.local_addr().expect("address");
        let writer = thread::spawn(move || {
            let mut client = TcpStream::connect(address).expect("client");
            for _ in 0..50 {
                if client.write_all(b"G").is_err() {
                    break;
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
        let (mut server, _) = listener.accept().expect("server");
        let started = Instant::now();
        let error =
            read_http_request_with_deadline(&mut server, started + Duration::from_millis(100))
                .expect_err("absolute deadline");

        assert_eq!(error, HttpRequestError::RequestTimeout);
        assert!(started.elapsed() < Duration::from_secs(1));
        drop(server);
        writer.join().expect("writer");
    }

    #[test]
    fn rejects_response_when_absolute_deadline_is_already_expired() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
        let client = TcpStream::connect(listener.local_addr().expect("address")).expect("client");
        let (mut server, _) = listener.accept().expect("server");

        let error = write_http_response_before_deadline(
            &mut server,
            "200 OK",
            "text/plain",
            b"payload",
            Instant::now(),
        )
        .expect_err("expired response deadline");

        assert!(format!("{error:#}").contains("absolute deadline exceeded"));
        drop(client);
    }

    #[cfg(unix)]
    #[test]
    fn absolute_response_deadline_stops_a_slow_reader_that_keeps_making_progress() {
        use std::os::fd::AsRawFd;
        use std::sync::Arc;
        use std::sync::atomic::AtomicBool;
        use std::sync::atomic::Ordering;

        fn set_socket_buffer(stream: &TcpStream, option: libc::c_int) {
            let bytes: libc::c_int = 4 * 1024;
            // SAFETY: the socket descriptor and pointer/length pair are valid for
            // the duration of this setsockopt call.
            let result = unsafe {
                libc::setsockopt(
                    stream.as_raw_fd(),
                    libc::SOL_SOCKET,
                    option,
                    std::ptr::from_ref(&bytes).cast(),
                    std::mem::size_of_val(&bytes) as libc::socklen_t,
                )
            };
            assert_eq!(result, 0, "set bounded test socket buffer");
        }

        let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
        let mut client =
            TcpStream::connect(listener.local_addr().expect("address")).expect("client");
        let (mut server, _) = listener.accept().expect("server");
        set_socket_buffer(&client, libc::SO_RCVBUF);
        set_socket_buffer(&server, libc::SO_SNDBUF);
        client
            .set_read_timeout(Some(Duration::from_secs(1)))
            .expect("client read timeout");

        let stop = Arc::new(AtomicBool::new(false));
        let reader_stop = Arc::clone(&stop);
        let reader = thread::spawn(move || {
            let mut byte = [0_u8; 1];
            while !reader_stop.load(Ordering::Relaxed) {
                match client.read(&mut byte) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => thread::sleep(Duration::from_millis(10)),
                }
            }
        });

        let body = vec![b'x'; 4 * 1024 * 1024];
        let started = Instant::now();
        let error = write_http_response_before_deadline(
            &mut server,
            "200 OK",
            "application/octet-stream",
            &body,
            started + Duration::from_millis(150),
        )
        .expect_err("slow reader must hit the absolute response deadline");
        stop.store(true, Ordering::Relaxed);
        drop(server);
        reader.join().expect("slow reader");

        assert!(format!("{error:#}").contains("response"));
        assert!(started.elapsed() < Duration::from_secs(2));
    }

    #[test]
    fn escapes_html_control_characters() {
        assert_eq!(escape_html("<a&b>"), "&lt;a&amp;b&gt;");
    }
}
