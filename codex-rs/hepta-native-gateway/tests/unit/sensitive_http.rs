use super::*;

fn request(extra_headers: &str) -> String {
    format!(
        "POST /api/v2/preferences/commit HTTP/1.1\r\nHost: 127.0.0.1:7373\r\nOrigin: http://127.0.0.1:7373\r\nX-Hepta-CSRF: 1\r\nContent-Type: application/json; charset=utf-8\r\n{extra_headers}Content-Length: 2\r\n\r\n{{}}"
    )
}

#[test]
fn sensitive_transport_requires_exact_loopback_origin_host_csrf_and_json() {
    assert!(requires_admission(OPERATOR_AUTHORITY_CHALLENGE_ENDPOINT));
    assert!(requires_admission(
        OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT
    ));
    assert!(admit(&request(""), "127.0.0.1:7373"));
    assert!(!admit(&request(""), "0.0.0.0:7373"));
    assert!(!admit(
        &request("").replace("Origin: http://127.0.0.1:7373", "Origin: https://evil.test"),
        "127.0.0.1:7373"
    ));
    assert!(!admit(
        &request("").replace("X-Hepta-CSRF: 1\r\n", ""),
        "127.0.0.1:7373"
    ));
    assert!(!admit(
        &request("Host: 127.0.0.1:7373\r\n"),
        "127.0.0.1:7373"
    ));
    assert!(!admit(
        &request("").replace("application/json; charset=utf-8", "text/plain"),
        "127.0.0.1:7373"
    ));
}

#[test]
fn sensitive_transport_supports_canonical_ipv6_loopback() {
    let request = request("").replace("127.0.0.1:7373", "[::1]:7373");
    assert!(admit(&request, "[::1]:7373"));
}
