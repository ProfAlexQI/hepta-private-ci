#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::ffi::OsStr;
use std::future::Future;
use std::io;
use std::time::Duration;

use anyhow::Result;
use anyhow::anyhow;
use rmcp_2026::model::PaginatedRequestParams;
use rmcp_2026::model::ProtocolVersion;
use rmcp_2026::service::ClientLifecycleMode;
use serde_json::Value;

pub const MCP_2026_PROTOCOL_VERSION: &str = "2026-07-28";
pub const MAX_DISCOVERY_MESSAGE_BYTES: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum McpProtocolMode {
    #[default]
    Legacy,
    V20260728,
}

impl McpProtocolMode {
    pub fn preferred_protocol_version(self) -> ProtocolVersion {
        match self {
            Self::Legacy => ProtocolVersion::V_2025_06_18,
            Self::V20260728 => ProtocolVersion::V_2026_07_28,
        }
    }

    pub fn client_lifecycle(self) -> ClientLifecycleMode {
        match self {
            Self::Legacy => ClientLifecycleMode::Initialize,
            Self::V20260728 => ClientLifecycleMode::Auto {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
                legacy_version: Some(ProtocolVersion::V_2025_06_18),
            },
        }
    }

    pub fn stdio_mode(self, requested_version: Option<&OsStr>) -> io::Result<Self> {
        match (self, requested_version) {
            (Self::Legacy, _) | (_, None) => Ok(Self::Legacy),
            (Self::V20260728, Some(version))
                if version == OsStr::new(MCP_2026_PROTOCOL_VERSION) =>
            {
                Ok(Self::V20260728)
            }
            (_, Some(version)) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "unsupported MCP protocol version `{}`; expected `{MCP_2026_PROTOCOL_VERSION}`",
                    version.to_string_lossy()
                ),
            )),
        }
    }
}

pub async fn collect_paginated<T, F, Fut>(
    method: &str,
    overall_timeout: Option<Duration>,
    mut fetch: F,
) -> Result<Vec<T>>
where
    F: FnMut(Option<PaginatedRequestParams>) -> Fut,
    Fut: Future<Output = Result<(Vec<T>, Option<String>)>>,
{
    let collect = async {
        let mut collected = Vec::new();
        let mut cursor = None;
        let mut seen_cursors = HashSet::new();
        loop {
            let params = cursor.as_ref().map(|next: &String| {
                PaginatedRequestParams::default().with_cursor(Some(next.clone()))
            });
            let (items, next_cursor) = fetch(params).await?;
            collected.extend(items);
            let Some(next_cursor) = next_cursor else {
                return Ok(collected);
            };
            if !seen_cursors.insert(next_cursor.clone()) {
                return Err(anyhow!("{method} returned a repeated pagination cursor"));
            }
            cursor = Some(next_cursor);
        }
    };
    match overall_timeout {
        Some(timeout) => tokio::time::timeout(timeout, collect)
            .await
            .map_err(|_| anyhow!("{method} pagination timed out after {timeout:?}"))?,
        None => collect.await,
    }
}

pub fn normalize_discovery_response(bytes: &[u8]) -> Result<Value> {
    if bytes.len() > MAX_DISCOVERY_MESSAGE_BYTES {
        return Err(anyhow!(
            "MCP discovery response exceeds {MAX_DISCOVERY_MESSAGE_BYTES} bytes"
        ));
    }
    let mut message: Value = serde_json::from_slice(bytes)?;
    let result = message
        .get_mut("result")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| anyhow!("MCP discovery response is missing an object result"))?;
    let versions = result
        .get("supportedVersions")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("MCP discovery response is missing supportedVersions"))?;
    if !versions
        .iter()
        .any(|version| version.as_str() == Some(MCP_2026_PROTOCOL_VERSION))
    {
        return Err(anyhow!(
            "MCP discovery response does not advertise {MCP_2026_PROTOCOL_VERSION}"
        ));
    }
    if !result.get("capabilities").is_some_and(Value::is_object) {
        return Err(anyhow!("MCP discovery response is missing capabilities"));
    }
    if !result.contains_key("serverInfo") {
        let server_info = result
            .get("_meta")
            .and_then(Value::as_object)
            .and_then(|meta| meta.get("io.modelcontextprotocol/serverInfo"))
            .cloned()
            .ok_or_else(|| anyhow!("MCP discovery response is missing serverInfo"))?;
        result.insert("serverInfo".to_string(), server_info);
    }
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_mode_uses_rmcp_3_mcp_2026_lifecycle() {
        assert_eq!(
            McpProtocolMode::V20260728.preferred_protocol_version(),
            ProtocolVersion::V_2026_07_28
        );
        assert_eq!(
            McpProtocolMode::V20260728.client_lifecycle(),
            ClientLifecycleMode::Auto {
                preferred_versions: vec![ProtocolVersion::V_2026_07_28],
                legacy_version: Some(ProtocolVersion::V_2025_06_18),
            }
        );
        assert_eq!(
            McpProtocolMode::V20260728
                .stdio_mode(Some(OsStr::new(MCP_2026_PROTOCOL_VERSION)))
                .expect("modern stdio opt-in"),
            McpProtocolMode::V20260728
        );
        assert_eq!(
            McpProtocolMode::V20260728
                .stdio_mode(None)
                .expect("missing marker remains legacy"),
            McpProtocolMode::Legacy
        );
    }

    #[tokio::test]
    async fn pagination_is_bounded_by_cursor_identity_and_timeout() {
        let mut page = 0;
        let items = collect_paginated("tools/list", Some(Duration::from_secs(1)), |_| {
            page += 1;
            async move {
                Ok(match page {
                    1 => (vec![1, 2], Some("next".to_string())),
                    _ => (vec![3], None),
                })
            }
        })
        .await
        .expect("collect pages");
        assert_eq!(items, vec![1, 2, 3]);

        let error = collect_paginated("tools/list", None, |_| async {
            Ok((Vec::<u8>::new(), Some("same".to_string())))
        })
        .await
        .expect_err("repeated cursor must fail");
        assert!(error.to_string().contains("repeated pagination cursor"));
    }

    #[test]
    fn discovery_normalizes_namespaced_server_info_and_rejects_oversize() {
        let response = serde_json::json!({
            "result": {
                "supportedVersions": [MCP_2026_PROTOCOL_VERSION],
                "capabilities": {},
                "_meta": {
                    "io.modelcontextprotocol/serverInfo": {
                        "name": "hepta-test",
                        "version": "1"
                    }
                }
            }
        });
        let normalized =
            normalize_discovery_response(&serde_json::to_vec(&response).expect("encode response"))
                .expect("normalize response");
        assert_eq!(
            normalized.pointer("/result/serverInfo/name"),
            Some(&Value::String("hepta-test".to_string()))
        );
        assert!(
            normalize_discovery_response(&vec![b'x'; MAX_DISCOVERY_MESSAGE_BYTES + 1]).is_err()
        );
    }
}
