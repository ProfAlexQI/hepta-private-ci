use std::{
    fmt::{self, Write as _},
    io::{Read, Write as _},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream},
    time::{Duration, Instant},
};

use hmac::{Hmac, Mac};
use sha2::Sha256;
use url::Url;
use zeroize::Zeroizing;

use super::{
    AuthenticatedLiveBridgeBinding, BridgeAdapterError, LiveSnapshotGet, LiveSnapshotHttpExecutor,
    LiveSnapshotHttpResponse, MAX_BRIDGE_CORRELATION_ID_BYTES, MAX_BRIDGE_SESSION_ID_BYTES,
    MAX_LIVE_SNAPSHOT_RESPONSE_BYTES,
};

const MAX_HTTP_RESPONSE_HEADER_BYTES: usize = 64 * 1024;
const MAX_HTTP_REQUEST_HEADER_BYTES: usize = 16 * 1024;
const MAX_AUTHORIZATION_BYTES: usize = 8 * 1024;
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
const DEFAULT_IO_TIMEOUT: Duration = Duration::from_secs(5);
const DEFAULT_ABSOLUTE_TIMEOUT: Duration = Duration::from_secs(10);

const SESSION_HEADER: &str = "x-hepta-bridge-session-id";
const RUN_HEADER: &str = "x-hepta-bridge-run-identifier-sha256";
const CORRELATION_HEADER: &str = "x-hepta-bridge-correlation-id";
const SEQUENCE_HEADER: &str = "x-hepta-bridge-sequence";
const RESPONSE_INTEGRITY_HEADER: &str = "x-hepta-bridge-response-hmac-sha256";
const RESPONSE_INTEGRITY_DOMAIN: &[u8] = b"hepta-native-live-bridge-response-v1\0";

type ResponseHmac = Hmac<Sha256>;

/// Concrete HTTP/1.1 executor for an already-authenticated loopback bridge.
///
/// Construction requires opaque authorization material issued by the trusted
/// backend. This type never obtains a Matrix access token, reads process
/// environment, follows redirects, resolves a non-loopback address, or sends
/// a request body. The secret is zeroized on drop and omitted from `Debug`.
/// A separate backend proof issuer is still required before the App can build
/// a production activation around this executor.
pub struct AuthenticatedLoopbackHttpExecutor {
    binding: AuthenticatedLiveBridgeBinding,
    authorization: Zeroizing<String>,
    response_integrity_key: Zeroizing<[u8; 32]>,
    connect_timeout: Duration,
    io_timeout: Duration,
    absolute_timeout: Duration,
}

impl fmt::Debug for AuthenticatedLoopbackHttpExecutor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthenticatedLoopbackHttpExecutor")
            .field("binding", &self.binding)
            .field("authorization", &"[REDACTED]")
            .field("response_integrity_key", &"[REDACTED]")
            .field("connect_timeout", &self.connect_timeout)
            .field("io_timeout", &self.io_timeout)
            .field("absolute_timeout", &self.absolute_timeout)
            .finish()
    }
}

impl AuthenticatedLoopbackHttpExecutor {
    pub fn try_new(
        binding: AuthenticatedLiveBridgeBinding,
        authorization: impl Into<String>,
        response_integrity_key: Zeroizing<[u8; 32]>,
    ) -> Result<Self, BridgeAdapterError> {
        // Wrap caller-provided authorization before inspecting it so every
        // rejected path still zeroizes the owned copy.
        let authorization = Zeroizing::new(authorization.into());
        if !valid_header_value(&authorization) || authorization.len() > MAX_AUTHORIZATION_BYTES {
            return Err(BridgeAdapterError::InvalidRequest(
                "authenticated bridge authorization material is invalid",
            ));
        }
        Ok(Self {
            binding,
            authorization,
            response_integrity_key,
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
            io_timeout: DEFAULT_IO_TIMEOUT,
            absolute_timeout: DEFAULT_ABSOLUTE_TIMEOUT,
        })
    }

    #[cfg(test)]
    fn with_timeouts(
        mut self,
        connect_timeout: Duration,
        io_timeout: Duration,
        absolute_timeout: Duration,
    ) -> Self {
        self.connect_timeout = connect_timeout;
        self.io_timeout = io_timeout;
        self.absolute_timeout = absolute_timeout;
        self
    }

    fn connect(&self, endpoint: &Url, deadline: Instant) -> Result<TcpStream, BridgeAdapterError> {
        let addresses = loopback_addresses(endpoint)?;
        for address in addresses {
            loop {
                let timeout = remaining_timeout(deadline, self.connect_timeout)?;
                match TcpStream::connect_timeout(&address, timeout) {
                    Ok(stream) => return Ok(stream),
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(_) => break,
                }
            }
        }
        Err(BridgeAdapterError::TransportUnavailable)
    }

    fn execute(&self, request: &LiveSnapshotGet) -> Result<Vec<u8>, BridgeAdapterError> {
        let deadline = Instant::now()
            .checked_add(self.absolute_timeout)
            .ok_or(BridgeAdapterError::TransportUnavailable)?;
        let endpoint = request.endpoint();
        let host = endpoint
            .host_str()
            .ok_or(BridgeAdapterError::InvalidRequest(
                "authenticated bridge endpoint host is missing",
            ))?;
        let port = endpoint
            .port_or_known_default()
            .ok_or(BridgeAdapterError::InvalidRequest(
                "authenticated bridge endpoint port is missing",
            ))?;

        for value in [
            request.session_id().as_str(),
            request.correlation_id().as_str(),
            request.run_identifier_sha256(),
        ] {
            if !valid_header_value(value) {
                return Err(BridgeAdapterError::InvalidRequest(
                    "authenticated bridge request binding is not HTTP-header safe",
                ));
            }
        }
        if request.session_id().as_str().len() > MAX_BRIDGE_SESSION_ID_BYTES
            || request.correlation_id().as_str().len() > MAX_BRIDGE_CORRELATION_ID_BYTES
        {
            return Err(BridgeAdapterError::InvalidRequest(
                "authenticated bridge request binding exceeds the HTTP-header limit",
            ));
        }

        let host_header = if host.parse::<IpAddr>().is_ok_and(|ip| ip.is_ipv6()) {
            format!("[{host}]:{port}")
        } else {
            format!("{host}:{port}")
        };
        let mut wire = Zeroizing::new(String::new());
        write!(
            wire,
            "GET {} HTTP/1.1\r\nHost: {}\r\nAccept: {}\r\nCache-Control: {}\r\nAuthorization: Hepta-Bridge {}\r\n{}: {}\r\n{}: {}\r\n{}: {}\r\n{}: {}\r\nConnection: close\r\n\r\n",
            endpoint.path(),
            host_header,
            request.accept(),
            request.cache_control(),
            self.authorization.as_str(),
            SESSION_HEADER,
            request.session_id(),
            RUN_HEADER,
            request.run_identifier_sha256(),
            CORRELATION_HEADER,
            request.correlation_id(),
            SEQUENCE_HEADER,
            request.expected_sequence(),
        )
        .map_err(|_| BridgeAdapterError::TransportUnavailable)?;
        if wire.len() > MAX_HTTP_REQUEST_HEADER_BYTES {
            return Err(BridgeAdapterError::InvalidRequest(
                "authenticated bridge request headers exceed the accepted limit",
            ));
        }

        let mut stream = self.connect(endpoint, deadline)?;
        write_all_before_deadline(&mut stream, wire.as_bytes(), deadline, self.io_timeout)?;
        flush_before_deadline(&mut stream, deadline, self.io_timeout)?;

        let capacity = MAX_HTTP_RESPONSE_HEADER_BYTES
            .saturating_add(4)
            .saturating_add(MAX_LIVE_SNAPSHOT_RESPONSE_BYTES);
        let mut response = Vec::with_capacity(16 * 1024);
        let mut chunk = [0u8; 16 * 1024];
        let mut framed_len: Option<usize> = None;
        loop {
            if framed_len.is_some_and(|length| response.len() == length) {
                break;
            }
            let accepted_remaining = framed_len
                .map(|length| length.saturating_sub(response.len()))
                .unwrap_or_else(|| capacity.saturating_sub(response.len()));
            if accepted_remaining == 0 {
                return Err(BridgeAdapterError::InvalidSnapshotResponse(
                    "HTTP response exceeds the accepted header and body limit",
                ));
            }
            stream
                .set_read_timeout(Some(remaining_timeout(deadline, self.io_timeout)?))
                .map_err(|_| BridgeAdapterError::TransportUnavailable)?;
            let read_limit = chunk.len().min(accepted_remaining);
            let count = match stream.read(&mut chunk[..read_limit]) {
                Ok(count) => count,
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => return Err(BridgeAdapterError::TransportUnavailable),
            };
            if count == 0 {
                break;
            }
            if response.len().saturating_add(count) > capacity {
                return Err(BridgeAdapterError::InvalidSnapshotResponse(
                    "HTTP response exceeds the accepted header and body limit",
                ));
            }
            response.extend_from_slice(&chunk[..count]);
            if framed_len.is_none() {
                framed_len = framed_response_len(&response)?;
            }
        }
        Ok(response)
    }
}

impl LiveSnapshotHttpExecutor for AuthenticatedLoopbackHttpExecutor {
    fn authenticated_binding(&self) -> &AuthenticatedLiveBridgeBinding {
        &self.binding
    }

    fn execute_get(
        &mut self,
        request: &LiveSnapshotGet,
    ) -> Result<LiveSnapshotHttpResponse, BridgeAdapterError> {
        if request.session_id() != self.binding.session_id()
            || request.run_identifier_sha256() != self.binding.run_identifier_sha256()
            || request.expected_sequence() < self.binding.initial_sequence()
        {
            return Err(BridgeAdapterError::InvalidRequest(
                "HTTP executor request does not match its authenticated binding",
            ));
        }
        let bytes = self.execute(request)?;
        parse_response(request.endpoint(), &bytes, &self.response_integrity_key)
    }
}

fn remaining_timeout(
    deadline: Instant,
    operation_cap: Duration,
) -> Result<Duration, BridgeAdapterError> {
    deadline
        .checked_duration_since(Instant::now())
        .filter(|remaining| !remaining.is_zero())
        .map(|remaining| remaining.min(operation_cap))
        .ok_or(BridgeAdapterError::TransportUnavailable)
}

fn write_all_before_deadline(
    stream: &mut TcpStream,
    mut bytes: &[u8],
    deadline: Instant,
    io_timeout: Duration,
) -> Result<(), BridgeAdapterError> {
    while !bytes.is_empty() {
        stream
            .set_write_timeout(Some(remaining_timeout(deadline, io_timeout)?))
            .map_err(|_| BridgeAdapterError::TransportUnavailable)?;
        let written = match stream.write(bytes) {
            Ok(0) => return Err(BridgeAdapterError::TransportUnavailable),
            Ok(written) => written,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => return Err(BridgeAdapterError::TransportUnavailable),
        };
        bytes = &bytes[written..];
    }
    Ok(())
}

fn flush_before_deadline(
    stream: &mut TcpStream,
    deadline: Instant,
    io_timeout: Duration,
) -> Result<(), BridgeAdapterError> {
    loop {
        stream
            .set_write_timeout(Some(remaining_timeout(deadline, io_timeout)?))
            .map_err(|_| BridgeAdapterError::TransportUnavailable)?;
        match stream.flush() {
            Ok(()) => return Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(_) => return Err(BridgeAdapterError::TransportUnavailable),
        }
    }
}

fn loopback_addresses(endpoint: &Url) -> Result<Vec<SocketAddr>, BridgeAdapterError> {
    if endpoint.scheme() != "http"
        || !endpoint.username().is_empty()
        || endpoint.password().is_some()
        || endpoint.query().is_some()
        || endpoint.fragment().is_some()
    {
        return Err(BridgeAdapterError::InvalidRequest(
            "authenticated bridge executor accepts only plain loopback HTTP without URL credentials, query, or fragment",
        ));
    }
    let host = endpoint
        .host_str()
        .ok_or(BridgeAdapterError::InvalidRequest(
            "authenticated bridge endpoint host is missing",
        ))?;
    let port = endpoint
        .port_or_known_default()
        .ok_or(BridgeAdapterError::InvalidRequest(
            "authenticated bridge endpoint port is missing",
        ))?;
    // Never invoke the system resolver on an activation-controlled host. The
    // sole accepted domain spelling is mapped locally and deterministically.
    let addresses = match host.parse::<IpAddr>() {
        Ok(ip) if ip.is_loopback() => vec![SocketAddr::new(ip, port)],
        Ok(_) => Vec::new(),
        Err(_) if host.eq_ignore_ascii_case("localhost") => vec![
            SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), port),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port),
        ],
        Err(_) => Vec::new(),
    };
    if addresses.is_empty() {
        return Err(BridgeAdapterError::InvalidRequest(
            "authenticated bridge executor resolved a non-loopback endpoint",
        ));
    }
    Ok(addresses)
}

fn valid_header_value(value: &str) -> bool {
    !value.trim().is_empty()
        && value == value.trim()
        && value.bytes().all(|byte| matches!(byte, 0x20..=0x7e))
}

fn parse_response(
    endpoint: &Url,
    bytes: &[u8],
    response_integrity_key: &[u8; 32],
) -> Result<LiveSnapshotHttpResponse, BridgeAdapterError> {
    let (header_end, status, headers) = parse_response_head(bytes)?.ok_or(
        BridgeAdapterError::InvalidSnapshotResponse("HTTP response headers are incomplete"),
    )?;
    if unique_header(&headers, "transfer-encoding")?.is_some() {
        return Err(BridgeAdapterError::InvalidSnapshotResponse(
            "chunked or encoded HTTP responses are not accepted",
        ));
    }
    let content_length = required_header(&headers, "content-length")?
        .parse::<usize>()
        .ok()
        .filter(|length| *length > 0 && *length <= MAX_LIVE_SNAPSHOT_RESPONSE_BYTES)
        .ok_or(BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP Content-Length is invalid",
        ))?;
    let body = &bytes[header_end + 4..];
    if body.len() != content_length {
        return Err(BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP response body length does not match Content-Length",
        ));
    }
    let sequence = required_header(&headers, SEQUENCE_HEADER)?
        .parse::<u64>()
        .map_err(|_| {
            BridgeAdapterError::InvalidSnapshotResponse("HTTP response sequence header is invalid")
        })?;
    let session = required_header(&headers, SESSION_HEADER)?;
    let correlation = required_header(&headers, CORRELATION_HEADER)?;
    let run = required_header(&headers, RUN_HEADER)?;
    let content_type = required_header(&headers, "content-type")?;
    let cache_control = required_header(&headers, "cache-control")?;
    if session.len() > MAX_BRIDGE_SESSION_ID_BYTES
        || correlation.len() > MAX_BRIDGE_CORRELATION_ID_BYTES
        || run.len() != 64
    {
        return Err(BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP response binding header exceeds the accepted limit",
        ));
    }
    verify_response_integrity(
        response_integrity_key,
        status,
        session,
        run,
        correlation,
        sequence,
        content_type,
        cache_control,
        body,
        required_header(&headers, RESPONSE_INTEGRITY_HEADER)?,
    )?;
    Ok(LiveSnapshotHttpResponse {
        final_endpoint: endpoint.clone(),
        status,
        content_type: content_type.to_string(),
        cache_control: cache_control.to_string(),
        authenticated_session_id: session.to_string().into(),
        authenticated_correlation_id: correlation.to_string().into(),
        run_identifier_sha256: run.to_string(),
        sequence,
        body: body.to_vec(),
    })
}

fn framed_response_len(bytes: &[u8]) -> Result<Option<usize>, BridgeAdapterError> {
    let Some((header_end, _, headers)) = parse_response_head(bytes)? else {
        return Ok(None);
    };
    if unique_header(&headers, "transfer-encoding")?.is_some() {
        return Err(BridgeAdapterError::InvalidSnapshotResponse(
            "chunked or encoded HTTP responses are not accepted",
        ));
    }
    let content_length = required_header(&headers, "content-length")?
        .parse::<usize>()
        .ok()
        .filter(|length| *length > 0 && *length <= MAX_LIVE_SNAPSHOT_RESPONSE_BYTES)
        .ok_or(BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP Content-Length is invalid",
        ))?;
    let framed_len = header_end
        .checked_add(4)
        .and_then(|length| length.checked_add(content_length))
        .ok_or(BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP response framing length overflowed",
        ))?;
    if bytes.len() > framed_len {
        return Err(BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP response contains bytes beyond Content-Length",
        ));
    }
    Ok(Some(framed_len))
}

type ParsedResponseHead = (usize, u16, Vec<(String, String)>);

fn parse_response_head(bytes: &[u8]) -> Result<Option<ParsedResponseHead>, BridgeAdapterError> {
    let Some(header_end) = bytes.windows(4).position(|window| window == b"\r\n\r\n") else {
        if bytes.len() > MAX_HTTP_RESPONSE_HEADER_BYTES.saturating_add(3) {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "HTTP response headers exceed the accepted limit",
            ));
        }
        return Ok(None);
    };
    if header_end > MAX_HTTP_RESPONSE_HEADER_BYTES {
        return Err(BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP response headers exceed the accepted limit",
        ));
    }
    let header = std::str::from_utf8(&bytes[..header_end]).map_err(|_| {
        BridgeAdapterError::InvalidSnapshotResponse("HTTP response headers are not UTF-8")
    })?;
    let mut lines = header.split("\r\n");
    let status_line = lines
        .next()
        .ok_or(BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP response status line is missing",
        ))?;
    let mut status_parts = status_line.split_whitespace();
    if !matches!(status_parts.next(), Some("HTTP/1.1" | "HTTP/1.0")) {
        return Err(BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP response version is unsupported",
        ));
    }
    let status = status_parts
        .next()
        .and_then(|value| value.parse::<u16>().ok())
        .filter(|value| (100..=599).contains(value))
        .ok_or(BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP response status is invalid",
        ))?;
    let headers = lines
        .map(|line| {
            let (name, value) =
                line.split_once(':')
                    .ok_or(BridgeAdapterError::InvalidSnapshotResponse(
                        "HTTP response header is malformed",
                    ))?;
            let name = name.trim().to_ascii_lowercase();
            let value = value.trim().to_string();
            if name.is_empty() || !valid_header_value(&value) {
                return Err(BridgeAdapterError::InvalidSnapshotResponse(
                    "HTTP response header is invalid",
                ));
            }
            Ok((name, value))
        })
        .collect::<Result<Vec<_>, BridgeAdapterError>>()?;
    Ok(Some((header_end, status, headers)))
}

#[allow(clippy::too_many_arguments)]
fn verify_response_integrity(
    key: &[u8; 32],
    status: u16,
    session: &str,
    run: &str,
    correlation: &str,
    sequence: u64,
    content_type: &str,
    cache_control: &str,
    body: &[u8],
    supplied_tag_hex: &str,
) -> Result<(), BridgeAdapterError> {
    let supplied_tag =
        decode_sha256_hex(supplied_tag_hex).ok_or(BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP response integrity HMAC header is invalid",
        ))?;
    let mac = response_integrity_mac(
        key,
        status,
        session,
        run,
        correlation,
        sequence,
        content_type,
        cache_control,
        body,
    );
    mac.verify_slice(&supplied_tag).map_err(|_| {
        BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP response integrity HMAC verification failed",
        )
    })
}

#[allow(clippy::too_many_arguments)]
fn response_integrity_mac(
    key: &[u8; 32],
    status: u16,
    session: &str,
    run: &str,
    correlation: &str,
    sequence: u64,
    content_type: &str,
    cache_control: &str,
    body: &[u8],
) -> ResponseHmac {
    let mut mac = ResponseHmac::new_from_slice(key)
        .expect("HMAC-SHA256 accepts the fixed 32-byte response integrity key");
    mac.update(RESPONSE_INTEGRITY_DOMAIN);
    update_mac_field(&mut mac, b"status", status.to_string().as_bytes());
    update_mac_field(&mut mac, b"session", session.as_bytes());
    update_mac_field(&mut mac, b"run", run.as_bytes());
    update_mac_field(&mut mac, b"correlation", correlation.as_bytes());
    update_mac_field(&mut mac, b"sequence", sequence.to_string().as_bytes());
    update_mac_field(&mut mac, b"content-type", content_type.as_bytes());
    update_mac_field(&mut mac, b"cache-control", cache_control.as_bytes());
    update_mac_field(&mut mac, b"body", body);
    mac
}

fn update_mac_field(mac: &mut ResponseHmac, label: &[u8], value: &[u8]) {
    mac.update(&(label.len() as u16).to_be_bytes());
    mac.update(label);
    mac.update(&(value.len() as u64).to_be_bytes());
    mac.update(value);
}

fn decode_sha256_hex(value: &str) -> Option<[u8; 32]> {
    if value.len() != 64 {
        return None;
    }
    let mut decoded = [0u8; 32];
    for (index, output) in decoded.iter_mut().enumerate() {
        let offset = index * 2;
        let high = decode_hex_nibble(value.as_bytes()[offset])?;
        let low = decode_hex_nibble(value.as_bytes()[offset + 1])?;
        *output = (high << 4) | low;
    }
    Some(decoded)
}

fn decode_hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

fn unique_header<'a>(
    headers: &'a [(String, String)],
    name: &str,
) -> Result<Option<&'a str>, BridgeAdapterError> {
    let mut values = headers
        .iter()
        .filter(|(candidate, _)| candidate == name)
        .map(|(_, value)| value.as_str());
    let value = values.next();
    if values.next().is_some() {
        return Err(BridgeAdapterError::InvalidSnapshotResponse(
            "HTTP response contains a duplicate security-relevant header",
        ));
    }
    Ok(value)
}

fn required_header<'a>(
    headers: &'a [(String, String)],
    name: &str,
) -> Result<&'a str, BridgeAdapterError> {
    unique_header(headers, name)?.ok_or(BridgeAdapterError::InvalidSnapshotResponse(
        "HTTP response is missing a required header",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_bridge::{CorrelationId, SessionId};
    use std::{net::TcpListener, sync::mpsc, thread, time::Instant};

    const RUN: &str = "7777777777777777777777777777777777777777777777777777777777777777";
    const RESPONSE_KEY: [u8; 32] = [0x42; 32];

    fn response_key() -> Zeroizing<[u8; 32]> {
        Zeroizing::new(RESPONSE_KEY)
    }

    fn binding() -> AuthenticatedLiveBridgeBinding {
        AuthenticatedLiveBridgeBinding::try_new(SessionId::from("session-7"), RUN, 3).unwrap()
    }

    fn request(endpoint: Url) -> LiveSnapshotGet {
        LiveSnapshotGet::try_new(
            endpoint,
            SessionId::from("session-7"),
            CorrelationId::from("correlation-9"),
            RUN,
            3,
        )
        .unwrap()
    }

    fn signed_response(body: &[u8], key: &[u8; 32]) -> Vec<u8> {
        let tag = response_integrity_mac(
            key,
            200,
            "session-7",
            RUN,
            "correlation-9",
            3,
            "application/json",
            "no-store",
            body,
        )
        .finalize()
        .into_bytes();
        let mut tag_hex = String::with_capacity(64);
        for byte in tag {
            write!(&mut tag_hex, "{byte:02x}").unwrap();
        }
        let head = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nCache-Control: no-store\r\nContent-Length: {}\r\n{}: session-7\r\n{}: correlation-9\r\n{}: {}\r\n{}: 3\r\n{}: {}\r\nConnection: close\r\n\r\n",
            body.len(),
            SESSION_HEADER,
            CORRELATION_HEADER,
            RUN_HEADER,
            RUN,
            SEQUENCE_HEADER,
            RESPONSE_INTEGRITY_HEADER,
            tag_hex,
        );
        [head.as_bytes(), body].concat()
    }

    fn read_request(stream: &mut TcpStream) -> Vec<u8> {
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut request = Vec::new();
        let mut byte = [0u8; 1];
        while !request.ends_with(b"\r\n\r\n") {
            stream.read_exact(&mut byte).unwrap();
            request.push(byte[0]);
        }
        request
    }

    fn spawn_server(response: Vec<u8>) -> (Url, thread::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = Url::parse(&format!(
            "http://{}/api/hepta-native-bridge/v1/snapshot",
            listener.local_addr().unwrap()
        ))
        .unwrap();
        let handle = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let request = read_request(&mut stream);
            stream.write_all(&response).unwrap();
            String::from_utf8(request).unwrap()
        });
        (endpoint, handle)
    }

    fn executor(key: [u8; 32]) -> AuthenticatedLoopbackHttpExecutor {
        AuthenticatedLoopbackHttpExecutor::try_new(binding(), "opaque-proof", Zeroizing::new(key))
            .unwrap()
    }

    #[test]
    fn real_loopback_get_is_bound_and_authorized_without_a_body() {
        let response = signed_response(b"{}", &RESPONSE_KEY);
        let (endpoint, server) = spawn_server(response);
        let mut executor = executor(RESPONSE_KEY).with_timeouts(
            Duration::from_secs(2),
            Duration::from_secs(2),
            Duration::from_secs(4),
        );
        let response = executor.execute_get(&request(endpoint)).unwrap();
        let observed = server.join().unwrap();

        assert_eq!(response.status, 200);
        assert_eq!(response.body, b"{}");
        assert_eq!(
            response.authenticated_correlation_id.as_str(),
            "correlation-9"
        );
        assert!(observed.starts_with("GET /api/hepta-native-bridge/v1/snapshot HTTP/1.1\r\n"));
        assert!(observed.contains("Authorization: Hepta-Bridge opaque-proof\r\n"));
        assert!(observed.contains("x-hepta-bridge-session-id: session-7\r\n"));
        assert!(observed.contains("x-hepta-bridge-correlation-id: correlation-9\r\n"));
        assert!(observed.ends_with("Connection: close\r\n\r\n"));
    }

    #[test]
    fn authorization_is_redacted_and_header_injection_is_rejected() {
        let executor =
            AuthenticatedLoopbackHttpExecutor::try_new(binding(), "opaque-proof", response_key())
                .unwrap();
        let debug = format!("{executor:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains("opaque-proof"));
        assert!(!debug.contains("42424242"));
        assert!(
            AuthenticatedLoopbackHttpExecutor::try_new(binding(), "bad\r\nheader", response_key(),)
                .is_err()
        );
    }

    #[test]
    fn non_loopback_and_duplicate_security_headers_fail_closed() {
        let endpoint =
            Url::parse("http://192.0.2.10:47821/api/hepta-native-bridge/v1/snapshot").unwrap();
        assert_eq!(
            loopback_addresses(&endpoint),
            Err(BridgeAdapterError::InvalidRequest(
                "authenticated bridge executor resolved a non-loopback endpoint"
            ))
        );

        let mut response = signed_response(b"{}", &RESPONSE_KEY);
        let insertion = response
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .unwrap();
        response.splice(
            insertion..insertion,
            format!("\r\n{}: other", SESSION_HEADER).bytes(),
        );
        let (endpoint, server) = spawn_server(response);
        let mut executor = executor(RESPONSE_KEY);
        assert!(matches!(
            executor.execute_get(&request(endpoint)),
            Err(BridgeAdapterError::InvalidSnapshotResponse(
                "HTTP response contains a duplicate security-relevant header"
            ))
        ));
        server.join().unwrap();
    }

    #[test]
    fn missing_wrong_key_and_tampered_body_fail_closed() {
        let mut missing = signed_response(b"{}", &RESPONSE_KEY);
        let marker = format!("{}: ", RESPONSE_INTEGRITY_HEADER);
        let line_start = missing
            .windows(marker.len())
            .position(|window| window == marker.as_bytes())
            .unwrap();
        let line_end = missing[line_start..]
            .windows(2)
            .position(|window| window == b"\r\n")
            .unwrap()
            + line_start
            + 2;
        missing.drain(line_start..line_end);

        let mut tampered = signed_response(b"{}", &RESPONSE_KEY);
        *tampered.last_mut().unwrap() = b']';

        for (response, key, expected) in [
            (
                missing,
                RESPONSE_KEY,
                "HTTP response is missing a required header",
            ),
            (
                signed_response(b"{}", &RESPONSE_KEY),
                [0x24; 32],
                "HTTP response integrity HMAC verification failed",
            ),
            (
                tampered,
                RESPONSE_KEY,
                "HTTP response integrity HMAC verification failed",
            ),
        ] {
            let (endpoint, server) = spawn_server(response);
            let mut executor = executor(key);
            assert_eq!(
                executor.execute_get(&request(endpoint)),
                Err(BridgeAdapterError::InvalidSnapshotResponse(expected))
            );
            server.join().unwrap();
        }
    }

    #[test]
    fn content_length_completion_does_not_wait_for_eof() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = Url::parse(&format!(
            "http://{}/api/hepta-native-bridge/v1/snapshot",
            listener.local_addr().unwrap()
        ))
        .unwrap();
        let (release_tx, release_rx) = mpsc::channel();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            read_request(&mut stream);
            stream
                .write_all(&signed_response(b"{}", &RESPONSE_KEY))
                .unwrap();
            release_rx.recv_timeout(Duration::from_secs(2)).unwrap();
        });
        let mut executor = executor(RESPONSE_KEY).with_timeouts(
            Duration::from_secs(1),
            Duration::from_secs(1),
            Duration::from_secs(1),
        );

        assert!(executor.execute_get(&request(endpoint)).is_ok());
        release_tx.send(()).unwrap();
        server.join().unwrap();
    }

    #[test]
    fn absolute_deadline_caps_a_stalled_response() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = Url::parse(&format!(
            "http://{}/api/hepta-native-bridge/v1/snapshot",
            listener.local_addr().unwrap()
        ))
        .unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            read_request(&mut stream);
            thread::sleep(Duration::from_millis(250));
        });
        let mut executor = executor(RESPONSE_KEY).with_timeouts(
            Duration::from_secs(1),
            Duration::from_secs(1),
            Duration::from_millis(50),
        );
        let started = Instant::now();

        assert_eq!(
            executor.execute_get(&request(endpoint)),
            Err(BridgeAdapterError::TransportUnavailable)
        );
        assert!(started.elapsed() < Duration::from_millis(200));
        server.join().unwrap();
    }

    #[test]
    fn oversized_session_correlation_and_request_headers_are_rejected() {
        assert!(
            AuthenticatedLiveBridgeBinding::try_new(
                SessionId::from("s".repeat(MAX_BRIDGE_SESSION_ID_BYTES + 1)),
                RUN,
                3,
            )
            .is_err()
        );

        let endpoint =
            Url::parse("http://127.0.0.1:9/api/hepta-native-bridge/v1/snapshot").unwrap();
        let oversized_request = LiveSnapshotGet::try_new(
            endpoint,
            SessionId::from("session-7"),
            CorrelationId::from("c".repeat(MAX_BRIDGE_CORRELATION_ID_BYTES + 1)),
            RUN,
            3,
        );
        assert_eq!(
            oversized_request,
            Err(BridgeAdapterError::InvalidRequest(
                "live snapshot request binding is not bounded and HTTP-header safe"
            ))
        );
    }
}
