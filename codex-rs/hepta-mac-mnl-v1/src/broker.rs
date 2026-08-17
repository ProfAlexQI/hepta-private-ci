//! Typed, non-executing broker protocol model.

use serde::Deserialize;
use serde::Serialize;

use crate::MnlError;
use crate::blocked;
use crate::canonical_json;
use crate::install::BROKER_SOCKET_PATH;
use crate::install::CLIENT_BINARY_PATH;
use crate::install::CandidateIdentityV1;
use crate::install::ClosedAuthorityV1;
use crate::install::InstallPlanV1;
use crate::install::OPERATOR_GID;
use crate::install::OPERATOR_GROUP;
use crate::install::OPERATOR_NAME;
use crate::install::OPERATOR_UID;
use crate::install::OwnerV1;
use crate::install::UnattestedCorrelationV1;
use crate::install::operator_staff;
use crate::install::require_activation_inputs;
use crate::install::validate_plan_shape;
use crate::invalid;
use crate::parse_canonical;
use crate::sha256;

pub const BROKER_PLAN_SCHEMA: &str = "hepta_mac_mnl_broker_plan_v1";
pub const BROKER_REQUEST_SCHEMA: &str = "hepta_mac_mnl_broker_request_v1";
pub const BROKER_RESPONSE_SCHEMA: &str = "hepta_mac_mnl_broker_closed_response_v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BrokerOperationV1 {
    VerifyExactInstalledLayoutReadOnly,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SocketModeV1 {
    #[serde(rename = "0600")]
    OwnerOnly0600,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PeerContractV1 {
    pub allow_caller_identity_override: bool,
    pub client_binary_path: String,
    pub client_binary_sha256: Option<String>,
    pub kernel_audit_token_required: bool,
    pub kernel_peer_pid_required: bool,
    pub operator_gid: u32,
    pub operator_group: String,
    pub operator_name: String,
    pub operator_uid: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SocketContractV1 {
    pub mode: SocketModeV1,
    pub owner: OwnerV1,
    pub path: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BrokerDispositionV1 {
    BlockedSuccessorFinalToolingUnfrozen,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BrokerPlanV1 {
    pub authority: ClosedAuthorityV1,
    pub candidate: CandidateIdentityV1,
    pub correlation: UnattestedCorrelationV1,
    pub disposition: BrokerDispositionV1,
    pub install_plan_sha256: String,
    pub operation: BrokerOperationV1,
    pub peer: PeerContractV1,
    pub request_schema: String,
    pub response_schema: String,
    pub schema: String,
    pub schema_version: u32,
    pub socket: SocketContractV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BrokerRequestV1 {
    pub client_binary_path: String,
    pub client_binary_sha256: Option<String>,
    pub correlation_nonce: String,
    pub install_plan_sha256: String,
    pub operation: BrokerOperationV1,
    pub schema: String,
    pub schema_version: u32,
}

// Raw response bytes are deliberately private. External callers can neither
// construct nor deserialize a response with authority bits of their choice.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct BrokerResponseWireV1 {
    authority: ClosedAuthorityV1,
    correlation_nonce: String,
    disposition: BrokerDispositionV1,
    schema: String,
    schema_version: u32,
}

/// Opaque, exact, closed response. It exposes bytes and read-only facts only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedClosedBrokerResponseV1 {
    wire: BrokerResponseWireV1,
}

impl VerifiedClosedBrokerResponseV1 {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, MnlError> {
        canonical_json(&self.wire)
    }

    pub fn correlation_nonce(&self) -> &str {
        &self.wire.correlation_nonce
    }

    pub fn authority_is_fully_closed(&self) -> bool {
        self.wire.authority.is_fully_closed()
    }
}

pub fn plan_read_only(install: &InstallPlanV1) -> Result<BrokerPlanV1, MnlError> {
    validate_plan_shape(install)?;
    exact_broker_plan(install)
}

pub fn request_read_only(
    broker: &BrokerPlanV1,
    install: &InstallPlanV1,
) -> Result<BrokerRequestV1, MnlError> {
    validate_broker_plan_shape(broker, install)?;
    Ok(exact_request(broker))
}

/// The only response constructor always fixes every authority bit to false.
pub fn closed_response_read_only(
    broker: &BrokerPlanV1,
    request: &BrokerRequestV1,
    install: &InstallPlanV1,
) -> Result<VerifiedClosedBrokerResponseV1, MnlError> {
    validate_request_shape(request, broker, install)?;
    Ok(VerifiedClosedBrokerResponseV1 {
        wire: exact_response(broker),
    })
}

pub fn verify_canonical_plan(
    bytes: &[u8],
    install: &InstallPlanV1,
) -> Result<BrokerPlanV1, MnlError> {
    let broker: BrokerPlanV1 = parse_canonical(bytes, "Mac MNL broker plan")?;
    validate_broker_plan_shape(&broker, install)?;
    require_activation_inputs(install)?;
    Ok(broker)
}

pub fn verify_canonical_request(
    bytes: &[u8],
    broker: &BrokerPlanV1,
    install: &InstallPlanV1,
) -> Result<BrokerRequestV1, MnlError> {
    let request: BrokerRequestV1 = parse_canonical(bytes, "Mac MNL broker request")?;
    validate_request_shape(&request, broker, install)?;
    require_activation_inputs(install)?;
    Ok(request)
}

pub fn verify_canonical_response(
    bytes: &[u8],
    broker: &BrokerPlanV1,
    request: &BrokerRequestV1,
    install: &InstallPlanV1,
) -> Result<VerifiedClosedBrokerResponseV1, MnlError> {
    let response = validate_response_shape(bytes, broker, request, install)?;
    require_activation_inputs(install)?;
    Ok(response)
}

/// There is no socket open, accept loop, or legacy bridge behind this boundary.
pub fn execute_live_broker_v1() -> Result<(), MnlError> {
    Err(blocked(
        "Mac MNL live broker has no implementation, socket access, or authority",
    ))
}

pub(crate) fn validate_broker_plan_shape(
    broker: &BrokerPlanV1,
    install: &InstallPlanV1,
) -> Result<(), MnlError> {
    validate_plan_shape(install)?;
    if broker != &exact_broker_plan(install)? {
        return Err(invalid(
            "broker plan differs from fixed physical socket, permitted peer, products, baseline, absent successor, operation, correlation, or closed authority",
        ));
    }
    Ok(())
}

pub(crate) fn validate_request_shape(
    request: &BrokerRequestV1,
    broker: &BrokerPlanV1,
    install: &InstallPlanV1,
) -> Result<(), MnlError> {
    validate_broker_plan_shape(broker, install)?;
    if request != &exact_request(broker) {
        return Err(invalid(
            "broker request differs from fixed read-only operation, client identity, correlation, or install-plan pin",
        ));
    }
    Ok(())
}

pub(crate) fn validate_response_shape(
    bytes: &[u8],
    broker: &BrokerPlanV1,
    request: &BrokerRequestV1,
    install: &InstallPlanV1,
) -> Result<VerifiedClosedBrokerResponseV1, MnlError> {
    let wire: BrokerResponseWireV1 = parse_canonical(bytes, "Mac MNL broker response")?;
    validate_request_shape(request, broker, install)?;
    if wire != exact_response(broker) {
        return Err(invalid(
            "broker response differs from exact closed authority, correlation, schema, or blocked disposition",
        ));
    }
    Ok(VerifiedClosedBrokerResponseV1 { wire })
}

fn exact_broker_plan(install: &InstallPlanV1) -> Result<BrokerPlanV1, MnlError> {
    Ok(BrokerPlanV1 {
        authority: ClosedAuthorityV1::exact(),
        candidate: install.candidate.clone(),
        correlation: install.correlation.clone(),
        disposition: BrokerDispositionV1::BlockedSuccessorFinalToolingUnfrozen,
        install_plan_sha256: sha256(&canonical_json(install)?),
        operation: BrokerOperationV1::VerifyExactInstalledLayoutReadOnly,
        peer: PeerContractV1 {
            allow_caller_identity_override: false,
            client_binary_path: CLIENT_BINARY_PATH.to_string(),
            client_binary_sha256: install.frozen_artifact_pins.client_binary_sha256.clone(),
            kernel_audit_token_required: true,
            kernel_peer_pid_required: true,
            operator_gid: OPERATOR_GID,
            operator_group: OPERATOR_GROUP.to_string(),
            operator_name: OPERATOR_NAME.to_string(),
            operator_uid: OPERATOR_UID,
        },
        request_schema: BROKER_REQUEST_SCHEMA.to_string(),
        response_schema: BROKER_RESPONSE_SCHEMA.to_string(),
        schema: BROKER_PLAN_SCHEMA.to_string(),
        schema_version: 1,
        socket: SocketContractV1 {
            mode: SocketModeV1::OwnerOnly0600,
            owner: operator_staff(),
            path: BROKER_SOCKET_PATH.to_string(),
        },
    })
}

fn exact_request(broker: &BrokerPlanV1) -> BrokerRequestV1 {
    BrokerRequestV1 {
        client_binary_path: broker.peer.client_binary_path.clone(),
        client_binary_sha256: broker.peer.client_binary_sha256.clone(),
        correlation_nonce: broker.correlation.nonce.clone(),
        install_plan_sha256: broker.install_plan_sha256.clone(),
        operation: BrokerOperationV1::VerifyExactInstalledLayoutReadOnly,
        schema: BROKER_REQUEST_SCHEMA.to_string(),
        schema_version: 1,
    }
}

fn exact_response(broker: &BrokerPlanV1) -> BrokerResponseWireV1 {
    BrokerResponseWireV1 {
        authority: ClosedAuthorityV1::exact(),
        correlation_nonce: broker.correlation.nonce.clone(),
        disposition: BrokerDispositionV1::BlockedSuccessorFinalToolingUnfrozen,
        schema: BROKER_RESPONSE_SCHEMA.to_string(),
        schema_version: 1,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;
    use crate::install::exact_plan_for_correlation;

    const NONCE: &str = "123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0";

    fn install() -> InstallPlanV1 {
        exact_plan_for_correlation(NONCE).expect("install")
    }

    fn broker(install: &InstallPlanV1) -> BrokerPlanV1 {
        plan_read_only(install).expect("broker")
    }

    fn request(broker: &BrokerPlanV1, install: &InstallPlanV1) -> BrokerRequestV1 {
        request_read_only(broker, install).expect("request")
    }

    #[test]
    fn socket_is_physical_and_connectable_by_exact_permitted_peer() {
        let install = install();
        let broker = broker(&install);
        assert_eq!(broker.socket.path, BROKER_SOCKET_PATH);
        assert_eq!(broker.socket.mode, SocketModeV1::OwnerOnly0600);
        assert_eq!(broker.socket.owner, operator_staff());
        assert_eq!(broker.peer.operator_uid, broker.socket.owner.uid);
        assert_eq!(broker.peer.operator_gid, broker.socket.owner.gid);
    }

    #[test]
    fn plan_and_request_verifiers_are_blocked_by_absent_successor() {
        let install = install();
        let broker = broker(&install);
        let request = request(&broker, &install);
        assert!(matches!(
            verify_canonical_plan(&canonical_json(&broker).expect("bytes"), &install),
            Err(MnlError::Blocked(_))
        ));
        assert!(matches!(
            verify_canonical_request(&canonical_json(&request).expect("bytes"), &broker, &install),
            Err(MnlError::Blocked(_))
        ));
    }

    #[test]
    fn response_can_only_be_constructed_fully_closed() {
        let install = install();
        let broker = broker(&install);
        let request = request(&broker, &install);
        let response = closed_response_read_only(&broker, &request, &install).expect("closed");
        assert!(response.authority_is_fully_closed());
        assert_eq!(response.correlation_nonce(), NONCE);
    }

    #[test]
    fn exact_response_verification_still_blocks_without_successor() {
        let install = install();
        let broker = broker(&install);
        let request = request(&broker, &install);
        let response = closed_response_read_only(&broker, &request, &install).expect("closed");
        assert!(matches!(
            verify_canonical_response(
                &response.canonical_bytes().expect("bytes"),
                &broker,
                &request,
                &install
            ),
            Err(MnlError::Blocked(_))
        ));
    }

    #[test]
    fn response_with_any_authority_is_invalid_before_blocker() {
        let install = install();
        let broker = broker(&install);
        let request = request(&broker, &install);
        let mut wire = exact_response(&broker);
        wire.authority.operator_acceptance = true;
        let error = verify_canonical_response(
            &canonical_json(&wire).expect("bytes"),
            &broker,
            &request,
            &install,
        )
        .expect_err("authority must reject");
        assert!(matches!(error, MnlError::Invalid(_)));
    }

    #[test]
    fn wrong_socket_peer_request_and_authority_are_rejected() {
        let install = install();
        let exact = broker(&install);
        let mut socket = exact.clone();
        socket.socket.path = "/var/run/caller.sock".to_string();
        assert!(validate_broker_plan_shape(&socket, &install).is_err());
        let mut owner = exact.clone();
        owner.socket.owner.uid = 0;
        assert!(validate_broker_plan_shape(&owner, &install).is_err());
        let mut peer = exact.clone();
        peer.peer.operator_uid = 502;
        assert!(validate_broker_plan_shape(&peer, &install).is_err());
        let mut authority = exact.clone();
        authority.authority.broker_live = true;
        assert!(validate_broker_plan_shape(&authority, &install).is_err());
        let mut caller = request(&exact, &install);
        caller.client_binary_path = "/tmp/client".to_string();
        assert!(validate_request_shape(&caller, &exact, &install).is_err());
    }

    #[test]
    fn request_unknown_argv_path_success_and_effect_fields_are_rejected() {
        let install = install();
        let broker = broker(&install);
        let request = request(&broker, &install);
        for field in ["argv", "path", "success", "effect"] {
            let mut value = serde_json::to_value(&request).expect("value");
            value
                .as_object_mut()
                .expect("object")
                .insert(field.to_string(), Value::Bool(true));
            assert!(
                verify_canonical_request(
                    &serde_json::to_vec(&value).expect("bytes"),
                    &broker,
                    &install,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn live_broker_boundary_is_blocked_without_socket_or_bridge() {
        assert!(matches!(
            execute_live_broker_v1(),
            Err(MnlError::Blocked(_))
        ));
    }
}
