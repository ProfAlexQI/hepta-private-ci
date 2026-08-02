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
    assert_eq!(request_query(&request), Some("secret=redacted"));
    assert_eq!(request_body_text(&request), Some("payload"));
    assert!(!request.contains("ignored"));
}

#[test]
fn request_query_distinguishes_absent_and_empty_queries() {
    assert_eq!(request_query("GET /api/test HTTP/1.1\r\n\r\n"), None);
    assert_eq!(request_query("GET /api/test? HTTP/1.1\r\n\r\n"), Some(""));
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

    let error =
        read_http_request_with_deadline(&mut server, Instant::now()).expect_err("expired deadline");
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
    let error = read_http_request_with_deadline(&mut server, started + Duration::from_millis(100))
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

#[test]
fn response_csp_allows_only_same_origin_external_scripts() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let mut client = TcpStream::connect(listener.local_addr().expect("address")).expect("client");
    let (mut server, _) = listener.accept().expect("server");
    let writer = thread::spawn(move || {
        write_http_response(&mut server, "200 OK", "text/plain", b"ready").expect("write response");
    });

    let mut response = String::new();
    client
        .read_to_string(&mut response)
        .expect("read response headers");
    writer.join().expect("response writer");

    assert!(response.contains("script-src 'self';"));
    assert!(response.contains("connect-src 'self';"));
    assert!(!response.contains("script-src 'self' 'unsafe-inline'"));
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
    let mut client = TcpStream::connect(listener.local_addr().expect("address")).expect("client");
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
