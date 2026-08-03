use serde::{Deserialize, Serialize};
use url::{Host, Url};

use super::{adapter::BridgeCapabilities, contract::SessionId};

/// Canonical read-only endpoint required by the Native bridge contract.
///
/// None of the legacy Control UI report endpoints are substitutes for this
/// envelope. In particular, `/api/operator-snapshot`, `/api/session-activity`,
/// `/api/task/<id>`, `/api/approvals`, `/api/activity`, and
/// `/api/gateway-runtime` do not carry the complete bridge metadata contract.
pub const HEPTA_LIVE_BRIDGE_SNAPSHOT_PATH: &str = "/api/hepta-native-bridge/v1/snapshot";

/// A reason that a logged-in Native session must not construct a live bridge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveBridgeBlocker {
    MatrixSessionNotAuthenticated,
    ExplicitOptInMissing,
    EndpointInvalid,
    EndpointNotLoopback,
    EndpointPathMismatch,
    AuthenticatedSessionBindingMissing,
    RunIdentifierInvalid,
    InitialSequenceInvalid,
    AuthoritativeSnapshotContractMissing,
}

/// Inputs owned by the post-login UI flow and a future trusted host adapter.
///
/// These inputs are deliberately not read from environment variables here.
/// Product code must derive them from the in-process login state and from a
/// successfully authenticated bridge handshake. This prevents process
/// environment injection from turning a fixture or report endpoint into live
/// product truth.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveBridgeActivationContext<'a> {
    pub matrix_session_authenticated: bool,
    pub explicit_user_opt_in: bool,
    pub endpoint: &'a str,
    pub authenticated_session_id: SessionId,
    pub run_identifier_sha256: &'a str,
    pub initial_sequence: u64,
    pub authoritative_snapshot_contract: bool,
}

/// Side-effect-free result of evaluating whether a live adapter may even be
/// constructed. Passing this preflight does not itself create a transport or
/// perform a request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiveBridgePreflight {
    pub eligible_for_adapter_construction: bool,
    pub snapshot_enabled: bool,
    pub subscribe_enabled: bool,
    pub prepare_enabled: bool,
    pub confirm_enabled: bool,
    pub reject_enabled: bool,
    pub cancel_enabled: bool,
    pub blockers: Vec<LiveBridgeBlocker>,
}

impl LiveBridgePreflight {
    pub fn evaluate(context: &LiveBridgeActivationContext<'_>) -> Self {
        let mut blockers = Vec::new();

        if !context.matrix_session_authenticated {
            blockers.push(LiveBridgeBlocker::MatrixSessionNotAuthenticated);
        }
        if !context.explicit_user_opt_in {
            blockers.push(LiveBridgeBlocker::ExplicitOptInMissing);
        }

        match validate_loopback_snapshot_endpoint(context.endpoint) {
            Ok(()) => {}
            Err(blocker) => blockers.push(blocker),
        }

        if context.authenticated_session_id.is_blank() {
            blockers.push(LiveBridgeBlocker::AuthenticatedSessionBindingMissing);
        }
        if !is_sha256(context.run_identifier_sha256) {
            blockers.push(LiveBridgeBlocker::RunIdentifierInvalid);
        }
        if context.initial_sequence == 0 {
            blockers.push(LiveBridgeBlocker::InitialSequenceInvalid);
        }
        if !context.authoritative_snapshot_contract {
            blockers.push(LiveBridgeBlocker::AuthoritativeSnapshotContractMissing);
        }

        let eligible_for_adapter_construction = blockers.is_empty();
        Self {
            eligible_for_adapter_construction,
            snapshot_enabled: eligible_for_adapter_construction,
            subscribe_enabled: false,
            prepare_enabled: false,
            confirm_enabled: false,
            reject_enabled: false,
            cancel_enabled: false,
            blockers,
        }
    }

    /// Capabilities remain snapshot-only even after every preflight input is
    /// satisfied. Write capabilities are never inferred from login or opt-in.
    pub fn capabilities(&self) -> BridgeCapabilities {
        BridgeCapabilities {
            snapshot: self.snapshot_enabled,
            subscribe: false,
            prepare: false,
            confirm: false,
            reject: false,
            cancel: false,
        }
    }
}

pub(super) fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

fn validate_loopback_snapshot_endpoint(endpoint: &str) -> Result<(), LiveBridgeBlocker> {
    let parsed = Url::parse(endpoint).map_err(|_| LiveBridgeBlocker::EndpointInvalid)?;
    if !matches!(parsed.scheme(), "http" | "https")
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
    {
        return Err(LiveBridgeBlocker::EndpointInvalid);
    }

    let is_loopback = match parsed.host() {
        Some(Host::Ipv4(address)) => address.is_loopback(),
        Some(Host::Ipv6(address)) => address.is_loopback(),
        Some(Host::Domain(domain)) => domain.eq_ignore_ascii_case("localhost"),
        None => false,
    };
    if !is_loopback {
        return Err(LiveBridgeBlocker::EndpointNotLoopback);
    }
    if parsed.path() != HEPTA_LIVE_BRIDGE_SNAPSHOT_PATH {
        return Err(LiveBridgeBlocker::EndpointPathMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const RUN_IDENTIFIER_SHA256: &str =
        "7777777777777777777777777777777777777777777777777777777777777777";

    fn eligible_context(endpoint: &str) -> LiveBridgeActivationContext<'_> {
        LiveBridgeActivationContext {
            matrix_session_authenticated: true,
            explicit_user_opt_in: true,
            endpoint,
            authenticated_session_id: SessionId::new("session-7"),
            run_identifier_sha256: RUN_IDENTIFIER_SHA256,
            initial_sequence: 3,
            authoritative_snapshot_contract: true,
        }
    }

    const CANONICAL_ENDPOINT: &str = "http://127.0.0.1:47821/api/hepta-native-bridge/v1/snapshot";

    #[test]
    fn pre_login_flow_is_fail_closed() {
        let mut context = eligible_context(CANONICAL_ENDPOINT);
        context.matrix_session_authenticated = false;

        let preflight = LiveBridgePreflight::evaluate(&context);

        assert!(!preflight.eligible_for_adapter_construction);
        assert_eq!(
            preflight.blockers,
            vec![LiveBridgeBlocker::MatrixSessionNotAuthenticated]
        );
        assert_eq!(preflight.capabilities(), BridgeCapabilities::default());
    }

    #[test]
    fn post_login_still_requires_explicit_opt_in_and_authenticated_binding() {
        let mut context = eligible_context(CANONICAL_ENDPOINT);
        context.explicit_user_opt_in = false;
        context.authenticated_session_id = SessionId::new(" ");

        let preflight = LiveBridgePreflight::evaluate(&context);

        assert!(!preflight.eligible_for_adapter_construction);
        assert_eq!(
            preflight.blockers,
            vec![
                LiveBridgeBlocker::ExplicitOptInMissing,
                LiveBridgeBlocker::AuthenticatedSessionBindingMissing,
            ]
        );
    }

    #[test]
    fn run_and_sequence_bindings_are_concrete_and_fail_closed() {
        let mut context = eligible_context(CANONICAL_ENDPOINT);
        context.run_identifier_sha256 = "not-a-sha256";
        context.initial_sequence = 0;

        let preflight = LiveBridgePreflight::evaluate(&context);

        assert_eq!(
            preflight.blockers,
            vec![
                LiveBridgeBlocker::RunIdentifierInvalid,
                LiveBridgeBlocker::InitialSequenceInvalid,
            ]
        );
        assert_eq!(preflight.capabilities(), BridgeCapabilities::default());
    }

    #[test]
    fn legacy_report_endpoint_cannot_be_promoted_as_a_snapshot() {
        for path in [
            "/api/operator-snapshot",
            "/api/session-activity",
            "/api/task/example",
            "/api/approvals",
            "/api/activity",
            "/api/gateway-runtime",
        ] {
            let endpoint = format!("http://127.0.0.1:47821{path}");
            let context = eligible_context(&endpoint);
            let preflight = LiveBridgePreflight::evaluate(&context);

            assert_eq!(
                preflight.blockers,
                vec![LiveBridgeBlocker::EndpointPathMismatch],
                "legacy path unexpectedly passed: {path}"
            );
            assert!(!preflight.snapshot_enabled);
        }
    }

    #[test]
    fn non_loopback_and_credential_bearing_urls_are_rejected() {
        for (endpoint, expected) in [
            (
                "http://192.0.2.10:47821/api/hepta-native-bridge/v1/snapshot",
                LiveBridgeBlocker::EndpointNotLoopback,
            ),
            (
                "http://token@127.0.0.1:47821/api/hepta-native-bridge/v1/snapshot",
                LiveBridgeBlocker::EndpointInvalid,
            ),
            (
                "file:///api/hepta-native-bridge/v1/snapshot",
                LiveBridgeBlocker::EndpointInvalid,
            ),
        ] {
            let context = eligible_context(endpoint);
            let preflight = LiveBridgePreflight::evaluate(&context);
            assert_eq!(preflight.blockers, vec![expected]);
        }
    }

    #[test]
    fn eligible_policy_is_strictly_read_only() {
        let preflight = LiveBridgePreflight::evaluate(&eligible_context(CANONICAL_ENDPOINT));
        let capabilities = preflight.capabilities();

        assert!(preflight.eligible_for_adapter_construction);
        assert!(capabilities.snapshot);
        assert!(!capabilities.subscribe);
        assert!(!capabilities.prepare);
        assert!(!capabilities.confirm);
        assert!(!capabilities.reject);
        assert!(!capabilities.cancel);
    }
}
