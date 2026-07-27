use std::net::TcpStream;

use anyhow::Context;
use anyhow::Result;

use crate::http_transport::HttpRequestError;
use crate::http_transport::write_http_response;
use crate::runtime_ingress::runtime_ingress_rejection_response;

pub(super) fn request_error(stream: &mut TcpStream, error: HttpRequestError) -> Result<()> {
    write_http_response(
        stream,
        error.status(),
        "application/json; charset=utf-8",
        error.response_body(),
    )
    .with_context(|| format!("write bounded HTTP rejection for {error}"))
}

pub(super) fn response(
    stream: &mut TcpStream,
    status: &'static str,
    content_type: &'static str,
    body: &[u8],
) -> Result<()> {
    write_http_response(stream, status, content_type, body)
}

pub(super) fn runtime_ingress(
    stream: &mut TcpStream,
    method: &str,
    path: &str,
    error: &anyhow::Error,
) -> Result<()> {
    eprintln!("RuntimeKernel request preflight rejected {method} {path}: {error:#}");
    let rejection = runtime_ingress_rejection_response(method, path);
    write_http_response(
        stream,
        rejection.status,
        "application/json; charset=utf-8",
        rejection.body.as_bytes(),
    )
}

#[cfg(test)]
pub(super) fn runtime_ingress_tuple(
    method: &str,
    path: &str,
) -> (&'static str, &'static str, String) {
    let rejection = runtime_ingress_rejection_response(method, path);
    (
        rejection.status,
        "application/json; charset=utf-8",
        rejection.body,
    )
}
