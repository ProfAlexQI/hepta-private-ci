#[cfg(test)]
use std::collections::VecDeque;

use thiserror::Error;

use super::contract::{
    BridgeRequest, BridgeRequestKind, BridgeUpdate, HEPTA_BRIDGE_SCHEMA_VERSION,
};

/// Capabilities are denied unless a trusted adapter opts into them explicitly.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BridgeCapabilities {
    pub snapshot: bool,
    pub subscribe: bool,
    pub prepare: bool,
    pub confirm: bool,
    pub reject: bool,
    pub cancel: bool,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BridgeAdapterError {
    #[error("the Hepta bridge is disabled")]
    Disabled,
    #[error("unsupported Hepta bridge schema version {received}; expected {supported}")]
    UnsupportedSchema { received: u16, supported: u16 },
    #[error("invalid bridge request: {0}")]
    InvalidRequest(&'static str),
    #[error("the requested Hepta bridge capability is disabled")]
    CapabilityDisabled,
    #[error("the trusted adapter returned an invalid or unsafe update")]
    UnsafeUpdate,
    #[error("bridge transport is unavailable")]
    TransportUnavailable,
    #[error("bridge adapter rejected the request: {0}")]
    Rejected(String),
}

/// Narrow host boundary. Implementations must perform policy and authorization checks.
///
/// The product UI treats all confirmation bindings as opaque. Implementations must not
/// infer runtime authority from Matrix events or equate Matrix delivery with a Hepta receipt.
pub(crate) trait BridgeTransport: Send {
    fn capabilities(&self) -> BridgeCapabilities { BridgeCapabilities::default() }

    fn handle(&mut self, request: BridgeRequest) -> Result<Vec<BridgeUpdate>, BridgeAdapterError>;
}

/// The production default: no subscriptions, network access, or mutations.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct DisabledBridgeAdapter;

impl BridgeTransport for DisabledBridgeAdapter {
    fn handle(&mut self, _request: BridgeRequest) -> Result<Vec<BridgeUpdate>, BridgeAdapterError> {
        Err(BridgeAdapterError::Disabled)
    }
}

/// An update that crossed a trusted adapter and passed schema, session,
/// origin, redaction, and provenance checks. Its inner value is intentionally
/// inaccessible outside the bridge implementation.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ValidatedBridgeUpdate(BridgeUpdate);

impl ValidatedBridgeUpdate {
    pub(super) fn as_update(&self) -> &BridgeUpdate { &self.0 }

    pub(super) fn into_update(self) -> BridgeUpdate { self.0 }

    #[cfg(test)]
    pub(super) fn for_test(update: BridgeUpdate) -> Self { Self(update) }

    #[cfg(test)]
    pub(super) fn as_update_mut(&mut self) -> &mut BridgeUpdate { &mut self.0 }
}

/// Mandatory validation wrapper around every bridge transport.
///
/// Product code never calls a transport directly. Capabilities are checked
/// before dispatch, and untrusted or cross-session results never reach the UI.
pub(crate) struct GuardedBridgeAdapter<T: BridgeTransport> {
    transport: T,
}

impl GuardedBridgeAdapter<DisabledBridgeAdapter> {
    pub fn disabled() -> Self {
        Self {
            transport: DisabledBridgeAdapter,
        }
    }
}

impl<T: BridgeTransport> GuardedBridgeAdapter<T> {
    #[cfg(test)]
    fn new(transport: T) -> Self { Self { transport } }

    pub fn capabilities(&self) -> BridgeCapabilities { self.transport.capabilities() }

    pub fn handle(
        &mut self,
        request: BridgeRequest,
    ) -> Result<Vec<ValidatedBridgeUpdate>, BridgeAdapterError> {
        if request.metadata.schema_version != HEPTA_BRIDGE_SCHEMA_VERSION {
            return Err(BridgeAdapterError::UnsupportedSchema {
                received: request.metadata.schema_version,
                supported: HEPTA_BRIDGE_SCHEMA_VERSION,
            });
        }
        if !request.is_contract_valid() {
            return Err(BridgeAdapterError::InvalidRequest(
                "metadata, origin, provenance, fields, and conversation binding must be internally consistent",
            ));
        }
        if !self.capabilities().allows(&request.request) {
            return Err(BridgeAdapterError::CapabilityDisabled);
        }

        let expected_session = request.metadata.session_id.clone();
        let expected_correlation = request.metadata.correlation_id.clone();
        let updates = self.transport.handle(request)?;

        updates
            .into_iter()
            .map(|update| {
                if !update.is_contract_valid()
                    || !update.is_presenter_safe()
                    || update.metadata.session_id != expected_session
                    || update.metadata.correlation_id != expected_correlation
                {
                    return Err(BridgeAdapterError::UnsafeUpdate);
                }
                Ok(ValidatedBridgeUpdate(update))
            })
            .collect()
    }
}

impl BridgeCapabilities {
    fn allows(self, request: &BridgeRequestKind) -> bool {
        match request {
            BridgeRequestKind::Snapshot => self.snapshot,
            BridgeRequestKind::Subscribe { .. } => self.subscribe,
            BridgeRequestKind::Prepare { .. } => self.prepare,
            BridgeRequestKind::Confirm { .. } => self.confirm,
            BridgeRequestKind::Reject { .. } => self.reject,
            BridgeRequestKind::Cancel { .. } => self.cancel,
        }
    }
}

/// Deterministic in-memory adapter for contract and UI tests only.
///
/// It never connects to Matrix or a Hepta runtime. Its only mutation is recording
/// requests in this local instance and consuming explicitly queued responses.
#[derive(Debug, Default)]
#[cfg(test)]
pub struct FakeBridgeAdapter {
    capabilities: BridgeCapabilities,
    queued: VecDeque<Result<Vec<BridgeUpdate>, BridgeAdapterError>>,
    observed: Vec<BridgeRequest>,
}

#[cfg(test)]
impl FakeBridgeAdapter {
    pub fn with_capabilities(capabilities: BridgeCapabilities) -> Self {
        Self {
            capabilities,
            ..Self::default()
        }
    }

    pub fn push_updates(&mut self, updates: Vec<BridgeUpdate>) {
        self.queued.push_back(Ok(updates));
    }

}

#[cfg(test)]
impl BridgeTransport for FakeBridgeAdapter {
    fn capabilities(&self) -> BridgeCapabilities { self.capabilities }

    fn handle(&mut self, request: BridgeRequest) -> Result<Vec<BridgeUpdate>, BridgeAdapterError> {
        if request.metadata.schema_version != HEPTA_BRIDGE_SCHEMA_VERSION {
            return Err(BridgeAdapterError::UnsupportedSchema {
                received: request.metadata.schema_version,
                supported: HEPTA_BRIDGE_SCHEMA_VERSION,
            });
        }
        self.observed.push(request);
        self.queued.pop_front().unwrap_or_else(|| Ok(Vec::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_bridge::contract::{
        BridgeRequestKind, BridgeUpdateKind, Redaction,
        tests_support::{binding, request_metadata, update},
    };

    fn snapshot_request() -> BridgeRequest {
        BridgeRequest {
            metadata: request_metadata("request-1"),
            binding: binding(),
            request: BridgeRequestKind::Snapshot,
        }
    }

    #[test]
    fn disabled_is_the_side_effect_free_default() {
        let mut transport = DisabledBridgeAdapter;

        assert_eq!(
            transport.handle(snapshot_request()),
            Err(BridgeAdapterError::Disabled)
        );
        assert_eq!(transport.capabilities(), BridgeCapabilities::default());

        let mut adapter = GuardedBridgeAdapter::disabled();
        assert_eq!(
            adapter.handle(snapshot_request()),
            Err(BridgeAdapterError::CapabilityDisabled)
        );
    }

    #[test]
    fn fake_adapter_only_returns_explicitly_queued_results() {
        let transport = FakeBridgeAdapter::with_capabilities(BridgeCapabilities {
            snapshot: true,
            ..BridgeCapabilities::default()
        });
        let mut adapter = GuardedBridgeAdapter::new(transport);

        assert_eq!(adapter.handle(snapshot_request()).unwrap(), Vec::new());
        assert!(adapter.capabilities().snapshot);
    }

    #[test]
    fn guard_rejects_cross_session_adapter_updates() {
        let mut unsafe_update = update(Redaction::redacted("trusted-test"));
        let BridgeUpdateKind::TaskUpsert { task } = &mut unsafe_update.update else {
            unreachable!();
        };
        task.metadata.session_id = "different-session".into();

        let mut transport = FakeBridgeAdapter::with_capabilities(BridgeCapabilities {
            snapshot: true,
            ..BridgeCapabilities::default()
        });
        transport.push_updates(vec![unsafe_update]);
        let mut adapter = GuardedBridgeAdapter::new(transport);

        assert_eq!(
            adapter.handle(snapshot_request()),
            Err(BridgeAdapterError::UnsafeUpdate)
        );
    }
}
