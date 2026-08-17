//! Read-only client bundle and the only two CLI parsers in this crate.

use std::ffi::OsString;

use serde::Deserialize;
use serde::Serialize;

use crate::MnlError;
use crate::blocked;
use crate::broker::BrokerPlanV1;
use crate::broker::BrokerRequestV1;
use crate::broker::closed_response_read_only;
use crate::broker::plan_read_only as plan_broker_read_only;
use crate::broker::request_read_only;
use crate::broker::validate_broker_plan_shape;
use crate::broker::validate_request_shape;
use crate::broker::validate_response_shape;
use crate::canonical_json;
use crate::install::BROKER_SOCKET_PATH;
use crate::install::CLIENT_BINARY_PATH;
use crate::install::CandidateIdentityV1;
use crate::install::ClosedAuthorityV1;
use crate::install::InstallPlanV1;
use crate::install::UnattestedCorrelationV1;
use crate::install::plan_read_only as plan_install_read_only;
use crate::install::require_activation_inputs;
use crate::install::validate_plan_shape;
use crate::invalid;
use crate::parse_canonical;
use crate::sha256;

pub const CLIENT_PLAN_SCHEMA: &str = "hepta_mac_mnl_client_plan_v1";
pub const BUNDLE_SCHEMA: &str = "hepta_mac_mnl_read_only_plan_bundle_v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientOperationV1 {
    VerifyExactInstalledLayoutReadOnly,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientDispositionV1 {
    BlockedSuccessorFinalToolingUnfrozen,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClientPlanV1 {
    pub authority: ClosedAuthorityV1,
    pub broker_closed_response_sha256: String,
    pub broker_plan_sha256: String,
    pub broker_request_sha256: String,
    pub broker_socket_path: String,
    pub candidate: CandidateIdentityV1,
    pub client_binary_path: String,
    pub client_binary_sha256: Option<String>,
    pub correlation: UnattestedCorrelationV1,
    pub disposition: ClientDispositionV1,
    pub operation: ClientOperationV1,
    pub schema: String,
    pub schema_version: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReadOnlyPlanBundleV1 {
    pub authority: ClosedAuthorityV1,
    pub broker: BrokerPlanV1,
    pub broker_closed_response_canonical_json: String,
    pub broker_request: BrokerRequestV1,
    pub client: ClientPlanV1,
    pub install: InstallPlanV1,
    pub schema: String,
    pub schema_version: u32,
}

pub fn plan_read_only_bundle() -> Result<ReadOnlyPlanBundleV1, MnlError> {
    exact_bundle(plan_install_read_only()?)
}

pub fn verify_canonical_bundle(bytes: &[u8]) -> Result<ReadOnlyPlanBundleV1, MnlError> {
    let bundle: ReadOnlyPlanBundleV1 = parse_canonical(bytes, "Mac MNL read-only plan bundle")?;
    validate_bundle_shape(&bundle)?;
    require_activation_inputs(&bundle.install)?;
    Ok(bundle)
}

pub fn run_plan_cli(arguments: Vec<OsString>) -> Result<String, String> {
    const USAGE: &str = "usage: hepta-mac-mnl-plan-v1 plan";
    if arguments.len() == 2
        && matches!(
            arguments.get(1).and_then(|value| value.to_str()),
            Some("--help" | "-h" | "help")
        )
    {
        return Ok(USAGE.to_string());
    }
    if arguments.len() != 2 || arguments.get(1).and_then(|value| value.to_str()) != Some("plan") {
        return Err(USAGE.to_string());
    }
    let bundle = plan_read_only_bundle().map_err(|error| error.to_string())?;
    String::from_utf8(canonical_json(&bundle).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())
}

pub fn run_verify_cli(arguments: Vec<OsString>, stdin: &[u8]) -> Result<String, String> {
    const USAGE: &str = "usage: hepta-mac-mnl-verify-v1 verify < canonical-plan-bundle.json";
    if arguments.len() == 2
        && matches!(
            arguments.get(1).and_then(|value| value.to_str()),
            Some("--help" | "-h" | "help")
        )
    {
        return Ok(USAGE.to_string());
    }
    if arguments.len() != 2 || arguments.get(1).and_then(|value| value.to_str()) != Some("verify") {
        return Err(USAGE.to_string());
    }
    verify_canonical_bundle(stdin).map_err(|error| error.to_string())?;
    Err("BLOCKED: Mac MNL v1 cannot grant live authority".to_string())
}

/// There is no socket open or dispatch behind this boundary.
pub fn execute_live_client_v1() -> Result<(), MnlError> {
    Err(blocked(
        "Mac MNL live client has no implementation, socket access, or authority",
    ))
}

pub(crate) fn validate_bundle_shape(bundle: &ReadOnlyPlanBundleV1) -> Result<(), MnlError> {
    validate_plan_shape(&bundle.install)?;
    validate_broker_plan_shape(&bundle.broker, &bundle.install)?;
    validate_request_shape(&bundle.broker_request, &bundle.broker, &bundle.install)?;
    let response_bytes = bundle.broker_closed_response_canonical_json.as_bytes();
    validate_response_shape(
        response_bytes,
        &bundle.broker,
        &bundle.broker_request,
        &bundle.install,
    )?;
    let expected = exact_bundle(bundle.install.clone())?;
    if bundle != &expected {
        return Err(invalid(
            "bundle differs from compiled install, broker, opaque closed response, client, products, baseline, absent successor, correlation, or closed authority",
        ));
    }
    Ok(())
}

fn exact_bundle(install: InstallPlanV1) -> Result<ReadOnlyPlanBundleV1, MnlError> {
    validate_plan_shape(&install)?;
    let broker = plan_broker_read_only(&install)?;
    let broker_request = request_read_only(&broker, &install)?;
    let response = closed_response_read_only(&broker, &broker_request, &install)?;
    let response_bytes = response.canonical_bytes()?;
    let response_json = String::from_utf8(response_bytes.clone())
        .map_err(|error| invalid(format!("closed response is not UTF-8: {error}")))?;
    let client = ClientPlanV1 {
        authority: ClosedAuthorityV1::exact(),
        broker_closed_response_sha256: sha256(&response_bytes),
        broker_plan_sha256: sha256(&canonical_json(&broker)?),
        broker_request_sha256: sha256(&canonical_json(&broker_request)?),
        broker_socket_path: BROKER_SOCKET_PATH.to_string(),
        candidate: install.candidate.clone(),
        client_binary_path: CLIENT_BINARY_PATH.to_string(),
        client_binary_sha256: install.frozen_artifact_pins.client_binary_sha256.clone(),
        correlation: install.correlation.clone(),
        disposition: ClientDispositionV1::BlockedSuccessorFinalToolingUnfrozen,
        operation: ClientOperationV1::VerifyExactInstalledLayoutReadOnly,
        schema: CLIENT_PLAN_SCHEMA.to_string(),
        schema_version: 1,
    };
    Ok(ReadOnlyPlanBundleV1 {
        authority: ClosedAuthorityV1::exact(),
        broker,
        broker_closed_response_canonical_json: response_json,
        broker_request,
        client,
        install,
        schema: BUNDLE_SCHEMA.to_string(),
        schema_version: 1,
    })
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use serde_json::Value;

    use super::*;

    #[test]
    fn bundle_is_digest_chained_to_opaque_closed_response() {
        let bundle = plan_read_only_bundle().expect("bundle");
        assert_eq!(
            bundle.client.broker_plan_sha256,
            sha256(&canonical_json(&bundle.broker).expect("broker bytes"))
        );
        assert_eq!(
            bundle.client.broker_request_sha256,
            sha256(&canonical_json(&bundle.broker_request).expect("request bytes"))
        );
        assert_eq!(
            bundle.client.broker_closed_response_sha256,
            sha256(bundle.broker_closed_response_canonical_json.as_bytes())
        );
        assert!(bundle.authority.is_fully_closed());
        assert!(bundle.client.authority.is_fully_closed());
    }

    #[test]
    fn bundle_verifier_is_blocked_by_absent_successor() {
        let bundle = plan_read_only_bundle().expect("bundle");
        let error = verify_canonical_bundle(&canonical_json(&bundle).expect("bytes"))
            .expect_err("successor absent");
        assert!(matches!(error, MnlError::Blocked(_)));
        assert!(error.to_string().contains("successor final tooling"));
    }

    #[test]
    fn tampered_response_digest_body_and_authority_are_rejected() {
        let exact = plan_read_only_bundle().expect("bundle");
        let mut digest = exact.clone();
        digest.client.broker_closed_response_sha256 = "0".repeat(64);
        assert!(validate_bundle_shape(&digest).is_err());

        let mut body = exact.clone();
        body.broker_closed_response_canonical_json.push('\n');
        assert!(validate_bundle_shape(&body).is_err());

        let mut authority = exact;
        authority.client.authority.operator_acceptance = true;
        assert!(validate_bundle_shape(&authority).is_err());
    }

    #[test]
    fn wrong_identity_path_correlation_and_pin_are_rejected() {
        let exact = plan_read_only_bundle().expect("bundle");
        let mut identity = exact.clone();
        identity.client.candidate.tooling_baseline.head = "0".repeat(40);
        assert!(validate_bundle_shape(&identity).is_err());
        let mut path = exact.clone();
        path.client.broker_socket_path = "/var/run/caller.sock".to_string();
        assert!(validate_bundle_shape(&path).is_err());
        let mut correlation = exact.clone();
        correlation.client.correlation.nonce = "0".repeat(64);
        assert!(validate_bundle_shape(&correlation).is_err());
        let mut pin = exact;
        pin.client.client_binary_sha256 = Some("a".repeat(64));
        assert!(validate_bundle_shape(&pin).is_err());
    }

    #[test]
    fn unknown_argv_path_success_effect_and_execute_fields_are_rejected() {
        let bundle = plan_read_only_bundle().expect("bundle");
        for field in ["argv", "path", "success", "effect", "execute"] {
            let mut value = serde_json::to_value(&bundle).expect("value");
            value
                .as_object_mut()
                .expect("object")
                .insert(field.to_string(), Value::Bool(true));
            assert!(verify_canonical_bundle(&serde_json::to_vec(&value).expect("bytes")).is_err());
        }
    }

    #[test]
    fn cli_accepts_only_plan_or_stdin_verify() {
        let plan_args = vec![
            OsString::from("hepta-mac-mnl-plan-v1"),
            OsString::from("plan"),
        ];
        let output = run_plan_cli(plan_args).expect("plan output");
        let parsed: ReadOnlyPlanBundleV1 =
            serde_json::from_slice(output.as_bytes()).expect("bundle");
        for forbidden in ["--path", "--argv", "--success", "--execute"] {
            assert!(
                run_plan_cli(vec![
                    OsString::from("hepta-mac-mnl-plan-v1"),
                    OsString::from("plan"),
                    OsString::from(forbidden),
                ])
                .is_err()
            );
            assert!(
                run_verify_cli(
                    vec![
                        OsString::from("hepta-mac-mnl-verify-v1"),
                        OsString::from("verify"),
                        OsString::from(forbidden),
                    ],
                    &canonical_json(&parsed).expect("bytes"),
                )
                .is_err()
            );
        }
    }

    #[test]
    fn live_client_boundary_is_blocked_without_socket_access() {
        assert!(matches!(
            execute_live_client_v1(),
            Err(MnlError::Blocked(_))
        ));
    }
}
