#![forbid(unsafe_code)]

use std::borrow::Borrow;
use std::fmt;
use std::ops::Deref;

use serde::Deserialize;
use serde::Serialize;

pub const EXEC_SIGNAL_METHOD: &str = "process/signal";
pub const EXEC_TERMINATE_METHOD: &str = "process/terminate";
pub const HTTP_REQUEST_METHOD: &str = "http/request";
pub const NETWORK_POLICY_REQUEST_METHOD: &str = "network/policyRequest";
pub const MAX_NETWORK_POLICY_HOST_BYTES: usize = 253;
pub const MAX_NETWORK_POLICY_PROCESS_ID_BYTES: usize = 256;
pub const MAX_NETWORK_POLICY_REASON_BYTES: usize = 1024;
pub const MAX_HTTP_TIMEOUT_MS: u64 = 3_600_000;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProcessId(String);

impl ProcessId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn validate(&self) -> Result<(), ProtocolValidationError> {
        validate_nonempty_bounded(
            "process id",
            self.as_str(),
            MAX_NETWORK_POLICY_PROCESS_ID_BYTES,
        )
    }
}

impl Deref for ProcessId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for ProcessId {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ProcessId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl From<&str> for ProcessId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyRequestParams {
    pub process_id: ProcessId,
    pub request: ExecServerNetworkPolicyRequest,
}

impl NetworkPolicyRequestParams {
    pub fn validate(&self) -> Result<(), ProtocolValidationError> {
        self.process_id.validate()?;
        validate_nonempty_bounded(
            "network host",
            &self.request.host,
            MAX_NETWORK_POLICY_HOST_BYTES,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecServerNetworkPolicyRequest {
    pub protocol: ExecServerNetworkProtocol,
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecServerNetworkProtocol {
    Http,
    HttpsConnect,
    Socks5Tcp,
    Socks5Udp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicyRequestResponse {
    pub decision: ExecServerNetworkPolicyDecision,
}

impl NetworkPolicyRequestResponse {
    pub fn validate(&self) -> Result<(), ProtocolValidationError> {
        match &self.decision {
            ExecServerNetworkPolicyDecision::Allow => Ok(()),
            ExecServerNetworkPolicyDecision::Deny { reason }
            | ExecServerNetworkPolicyDecision::Ask { reason } => validate_nonempty_bounded(
                "network policy reason",
                reason,
                MAX_NETWORK_POLICY_REASON_BYTES,
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ExecServerNetworkPolicyDecision {
    Allow,
    Deny { reason: String },
    Ask { reason: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProcessSignal {
    Interrupt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignalParams {
    pub process_id: ProcessId,
    pub signal: ProcessSignal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateParams {
    pub process_id: ProcessId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRequestParams {
    pub request_id: String,
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

impl HttpRequestParams {
    pub fn validate(&self) -> Result<(), ProtocolValidationError> {
        validate_nonempty_bounded("request id", &self.request_id, 256)?;
        if self
            .timeout_ms
            .is_some_and(|timeout| timeout > MAX_HTTP_TIMEOUT_MS)
        {
            return Err(ProtocolValidationError(
                "HTTP request timeout exceeds protocol maximum".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionTerminalStatus {
    Exited,
    Cancelled,
    TimedOut,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionTerminalReceipt {
    pub process_id: ProcessId,
    pub status: ExecutionTerminalStatus,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolValidationError(String);

impl fmt::Display for ProtocolValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for ProtocolValidationError {}

fn validate_nonempty_bounded(
    label: &str,
    value: &str,
    max_bytes: usize,
) -> Result<(), ProtocolValidationError> {
    if value.is_empty() || value.len() > max_bytes {
        return Err(ProtocolValidationError(format!(
            "{label} must contain 1..={max_bytes} bytes"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_policy_shapes_are_stable_and_bounded() {
        let params = NetworkPolicyRequestParams {
            process_id: ProcessId::from("process-1"),
            request: ExecServerNetworkPolicyRequest {
                protocol: ExecServerNetworkProtocol::HttpsConnect,
                host: "example.com".to_string(),
                port: 443,
            },
        };
        params.validate().expect("valid request");
        assert_eq!(
            serde_json::to_value(params).expect("serialize request"),
            serde_json::json!({
                "processId": "process-1",
                "request": {
                    "protocol": "https_connect",
                    "host": "example.com",
                    "port": 443
                }
            })
        );
        let oversized = NetworkPolicyRequestParams {
            process_id: ProcessId::from("process-1"),
            request: ExecServerNetworkPolicyRequest {
                protocol: ExecServerNetworkProtocol::Http,
                host: "x".repeat(MAX_NETWORK_POLICY_HOST_BYTES + 1),
                port: 80,
            },
        };
        assert!(oversized.validate().is_err());
    }

    #[test]
    fn cancellation_timeout_and_terminal_receipts_remain_protocol_only() {
        let signal = SignalParams {
            process_id: ProcessId::from("process-1"),
            signal: ProcessSignal::Interrupt,
        };
        assert_eq!(
            serde_json::to_value(signal).expect("serialize signal")["signal"],
            "interrupt"
        );

        let timeout: HttpRequestParams = serde_json::from_value(serde_json::json!({
            "requestId": "request-1",
            "method": "GET",
            "url": "https://example.com",
            "timeoutMs": 1234
        }))
        .expect("deserialize timeout");
        timeout.validate().expect("bounded timeout");

        let receipt = ExecutionTerminalReceipt {
            process_id: ProcessId::from("process-1"),
            status: ExecutionTerminalStatus::Cancelled,
            exit_code: None,
        };
        assert_eq!(
            serde_json::to_value(receipt).expect("serialize receipt")["status"],
            "cancelled"
        );
    }
}
