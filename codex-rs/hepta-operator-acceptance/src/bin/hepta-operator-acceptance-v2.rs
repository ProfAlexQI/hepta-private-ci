use std::ffi::OsString;
use std::path::PathBuf;

use codex_hepta_operator_acceptance::v2::AssessRequest;
use codex_hepta_operator_acceptance::v2::PrepareRequestV2;
use codex_hepta_operator_acceptance::v2::ReadReceiptRequestV2;
use codex_hepta_operator_acceptance::v2::VerifyRequestV2;
use codex_hepta_operator_acceptance::v2::assess;
use codex_hepta_operator_acceptance::v2::prepare_v2;
use codex_hepta_operator_acceptance::v2::verify_and_seal_v2;
use codex_hepta_operator_acceptance::v2::verify_receipt_v2;

const USAGE: &str = "usage:\n  hepta-operator-acceptance-v2 assess         <aggregate-root> <externally-pinned-aggregate-manifest-sha256> <legacy-product-audit-root>\n  hepta-operator-acceptance-v2 prepare        <aggregate-root> <externally-pinned-aggregate-manifest-sha256> <legacy-product-audit-root> <sidecar-root> <allowed-signers> <trust-policy-v2> <externally-pinned-trust-policy-sha256>\n  hepta-operator-acceptance-v2 verify         <aggregate-root> <externally-pinned-aggregate-manifest-sha256> <legacy-product-audit-root> <sidecar-root> <allowed-signers> <trust-policy-v2> <externally-pinned-trust-policy-sha256> <detached-signature>\n  hepta-operator-acceptance-v2 verify-receipt <aggregate-root> <externally-pinned-aggregate-manifest-sha256> <legacy-product-audit-root> <sidecar-root> <allowed-signers> <trust-policy-v2> <externally-pinned-trust-policy-sha256>";

fn main() {
    if let Err(error) = run(std::env::args_os().collect()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(arguments: Vec<OsString>) -> Result<(), String> {
    let command = arguments
        .get(1)
        .and_then(|value| value.to_str())
        .ok_or_else(|| USAGE.to_string())?;
    match command {
        "assess" if arguments.len() == 5 => {
            let aggregate = PathBuf::from(&arguments[2]);
            let manifest_sha256 = utf8(&arguments[3], "aggregate manifest digest")?;
            let product_audit = PathBuf::from(&arguments[4]);
            print_json(
                &assess(AssessRequest {
                    aggregate_manifest_sha256: manifest_sha256,
                    aggregate_root: &aggregate,
                    legacy_product_audit_root: &product_audit,
                })
                .map_err(|error| error.to_string())?,
            )
        }
        "prepare" if arguments.len() == 9 => {
            let aggregate = PathBuf::from(&arguments[2]);
            let aggregate_sha256 = utf8(&arguments[3], "aggregate manifest digest")?;
            let product_audit = PathBuf::from(&arguments[4]);
            let sidecar = PathBuf::from(&arguments[5]);
            let allowed_signers = PathBuf::from(&arguments[6]);
            let trust_policy = PathBuf::from(&arguments[7]);
            let trust_sha256 = utf8(&arguments[8], "trust policy digest")?;
            print_json(
                &prepare_v2(PrepareRequestV2 {
                    aggregate_manifest_sha256: aggregate_sha256,
                    aggregate_root: &aggregate,
                    allowed_signers_path: &allowed_signers,
                    externally_pinned_trust_policy_sha256: trust_sha256,
                    legacy_product_audit_root: &product_audit,
                    sidecar_root: &sidecar,
                    trust_policy_path: &trust_policy,
                })
                .map_err(|error| error.to_string())?,
            )
        }
        "verify" if arguments.len() == 10 => {
            let aggregate = PathBuf::from(&arguments[2]);
            let aggregate_sha256 = utf8(&arguments[3], "aggregate manifest digest")?;
            let product_audit = PathBuf::from(&arguments[4]);
            let sidecar = PathBuf::from(&arguments[5]);
            let allowed_signers = PathBuf::from(&arguments[6]);
            let trust_policy = PathBuf::from(&arguments[7]);
            let trust_sha256 = utf8(&arguments[8], "trust policy digest")?;
            let signature = PathBuf::from(&arguments[9]);
            print_json(
                &verify_and_seal_v2(VerifyRequestV2 {
                    aggregate_manifest_sha256: aggregate_sha256,
                    aggregate_root: &aggregate,
                    allowed_signers_path: &allowed_signers,
                    externally_pinned_trust_policy_sha256: trust_sha256,
                    legacy_product_audit_root: &product_audit,
                    sidecar_root: &sidecar,
                    signature_path: &signature,
                    trust_policy_path: &trust_policy,
                })
                .map_err(|error| error.to_string())?,
            )
        }
        "verify-receipt" if arguments.len() == 9 => {
            let aggregate = PathBuf::from(&arguments[2]);
            let aggregate_sha256 = utf8(&arguments[3], "aggregate manifest digest")?;
            let product_audit = PathBuf::from(&arguments[4]);
            let sidecar = PathBuf::from(&arguments[5]);
            let allowed_signers = PathBuf::from(&arguments[6]);
            let trust_policy = PathBuf::from(&arguments[7]);
            let trust_sha256 = utf8(&arguments[8], "trust policy digest")?;
            print_json(
                &verify_receipt_v2(ReadReceiptRequestV2 {
                    aggregate_manifest_sha256: aggregate_sha256,
                    aggregate_root: &aggregate,
                    allowed_signers_path: &allowed_signers,
                    externally_pinned_trust_policy_sha256: trust_sha256,
                    legacy_product_audit_root: &product_audit,
                    sidecar_root: &sidecar,
                    trust_policy_path: &trust_policy,
                })
                .map_err(|error| error.to_string())?,
            )
        }
        _ => Err(USAGE.to_string()),
    }
}

fn utf8<'a>(value: &'a OsString, label: &str) -> Result<&'a str, String> {
    value
        .to_str()
        .ok_or_else(|| format!("{label} must be UTF-8"))
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), String> {
    println!(
        "{}",
        serde_json::to_string(value).map_err(|error| error.to_string())?
    );
    Ok(())
}
