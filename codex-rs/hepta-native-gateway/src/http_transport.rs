use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

use anyhow::Context;
use anyhow::Result;

pub(crate) fn read_http_request(stream: &mut TcpStream) -> Result<String> {
    let mut buffer = [0_u8; 8192];
    let mut bytes = Vec::new();
    let first_read = stream.read(&mut buffer).context("read request")?;
    bytes.extend_from_slice(&buffer[..first_read]);
    let content_length = String::from_utf8_lossy(&bytes)
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            if name.eq_ignore_ascii_case("content-length") {
                value.trim().parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or(0);
    while request_body_bytes(&bytes).len() < content_length {
        let read = stream.read(&mut buffer).context("read request body")?;
        if read == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..read]);
    }
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

fn request_body_bytes(bytes: &[u8]) -> &[u8] {
    bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| &bytes[index + 4..])
        .unwrap_or(&[])
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
    let header = format!(
        "HTTP/1.1 {status}\r\ncontent-type: {content_type}\r\ncontent-length: {}\r\nconnection: close\r\nContent-Security-Policy: default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; base-uri 'none'; frame-ancestors 'none'\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nX-Frame-Options: DENY\r\n\r\n",
        body.len()
    );
    stream
        .write_all(header.as_bytes())
        .context("write header")?;
    stream.write_all(body).context("write body")?;
    stream.flush().context("flush response")?;
    Ok(())
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

    #[test]
    fn parses_request_target_and_body_without_query_leakage() {
        let request = "POST /api/test?secret=redacted HTTP/1.1\r\ncontent-length: 7\r\n\r\npayload";

        assert_eq!(
            request_method_and_path(request),
            Some(("POST", "/api/test"))
        );
        assert_eq!(request_body_text(request), Some("payload"));
    }

    #[test]
    fn escapes_html_control_characters() {
        assert_eq!(escape_html("<a&b>"), "&lt;a&amp;b&gt;");
    }
}
