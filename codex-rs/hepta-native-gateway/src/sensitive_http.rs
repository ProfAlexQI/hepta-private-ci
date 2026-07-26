//! Fail-closed transport admission for credentialed or mutating HTTP ingress.

use std::net::IpAddr;
use std::net::SocketAddr;

use crate::effect_reconciliation::EFFECT_RECONCILIATION_INSPECT_ENDPOINT;
use crate::effect_reconciliation::EFFECT_RECONCILIATION_RESOLVE_ENDPOINT;
use crate::operator_mutation::OPERATOR_MUTATION_COMMIT_ENDPOINT;
use crate::operator_mutation::OPERATOR_MUTATION_PLAN_ENDPOINT;
use crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT;
use crate::operator_mutation_reconciliation::OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT;
use crate::preference_ingress::PREFERENCE_CHALLENGE_ENDPOINT;
use crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT;
use crate::runtime_ingress::OPERATOR_AUTHORITY_CHALLENGE_ENDPOINT;
use crate::runtime_ingress::TELEGRAM_RECEIVE_ONCE_ENDPOINT;
use crate::runtime_mutation::RUNTIME_MUTATION_CANARY_ENDPOINT;
use crate::telegram_authority::TELEGRAM_AUTHORITY_COMMIT_ENDPOINT;
use crate::telegram_authority::TELEGRAM_AUTHORITY_PLAN_ENDPOINT;
use crate::telegram_authority::TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT;
use crate::telegram_authority::TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT;

const CSRF_HEADER: &str = "x-hepta-csrf";

pub(crate) fn requires_admission(path: &str) -> bool {
    matches!(
        path,
        OPERATOR_AUTHORITY_CHALLENGE_ENDPOINT
            | TELEGRAM_RECEIVE_ONCE_ENDPOINT
            | RUNTIME_MUTATION_CANARY_ENDPOINT
            | OPERATOR_MUTATION_PLAN_ENDPOINT
            | OPERATOR_MUTATION_COMMIT_ENDPOINT
            | OPERATOR_MUTATION_RECONCILIATION_INSPECT_ENDPOINT
            | OPERATOR_MUTATION_RECONCILIATION_RESOLVE_ENDPOINT
            | EFFECT_RECONCILIATION_INSPECT_ENDPOINT
            | EFFECT_RECONCILIATION_RESOLVE_ENDPOINT
            | PREFERENCE_CHALLENGE_ENDPOINT
            | PREFERENCE_COMMIT_ENDPOINT
            | TELEGRAM_AUTHORITY_PLAN_ENDPOINT
            | TELEGRAM_AUTHORITY_COMMIT_ENDPOINT
            | TELEGRAM_RECONCILIATION_INSPECT_ENDPOINT
            | TELEGRAM_RECONCILIATION_RESOLVE_ENDPOINT
    )
}

pub(crate) fn admit(request: &str, bind_addr: &str) -> bool {
    let Some(bind_port) = loopback_port(bind_addr) else {
        return false;
    };
    let Ok(Some(host)) = unique_header(request, "host") else {
        return false;
    };
    if !loopback_host_matches_port(host, bind_port) {
        return false;
    }
    let Ok(Some(origin)) = unique_header(request, "origin") else {
        return false;
    };
    if origin != format!("http://{host}") {
        return false;
    }
    let Ok(Some(csrf)) = unique_header(request, CSRF_HEADER) else {
        return false;
    };
    if csrf != "1" {
        return false;
    }
    let Ok(Some(content_type)) = unique_header(request, "content-type") else {
        return false;
    };
    content_type
        .split(';')
        .next()
        .is_some_and(|value| value.trim().eq_ignore_ascii_case("application/json"))
}

fn loopback_port(bind_addr: &str) -> Option<u16> {
    if let Ok(address) = bind_addr.parse::<SocketAddr>() {
        return address.ip().is_loopback().then_some(address.port());
    }
    let (host, port) = bind_addr.rsplit_once(':')?;
    (host.trim().eq_ignore_ascii_case("localhost"))
        .then(|| port.parse::<u16>().ok())
        .flatten()
}

fn loopback_host_matches_port(host: &str, expected_port: u16) -> bool {
    if let Ok(address) = host.parse::<SocketAddr>() {
        return address.ip().is_loopback() && address.port() == expected_port;
    }
    let Some((name, port)) = host.rsplit_once(':') else {
        return false;
    };
    let Ok(port) = port.parse::<u16>() else {
        return false;
    };
    if port != expected_port {
        return false;
    }
    if name.eq_ignore_ascii_case("localhost") {
        return true;
    }
    name.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback())
}

fn unique_header<'a>(request: &'a str, expected_name: &str) -> Result<Option<&'a str>, ()> {
    let mut result = None;
    for line in request
        .split_once("\r\n\r\n")
        .map_or(request, |(head, _)| head)
        .lines()
        .skip(1)
    {
        let Some((name, value)) = line.split_once(':') else {
            return Err(());
        };
        if name.trim().eq_ignore_ascii_case(expected_name) {
            if result.is_some() {
                return Err(());
            }
            let value = value.trim();
            if value.is_empty() || value.chars().any(|character| character.is_control()) {
                return Err(());
            }
            result = Some(value);
        }
    }
    Ok(result)
}

#[cfg(test)]
#[path = "../tests/unit/sensitive_http.rs"]
mod tests;
