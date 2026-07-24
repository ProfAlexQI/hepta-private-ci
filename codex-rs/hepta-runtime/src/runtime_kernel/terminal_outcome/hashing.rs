//! Length-framed hashes for terminal outcomes.

use hepta_contracts::AuthorizationRef;
use hepta_contracts::CandidateRef;
use hepta_contracts::CapabilityManifestRef;
use hepta_contracts::ContentHash;
use hepta_contracts::FrozenTurnContext;
use hepta_contracts::OutcomeStatus;
use hepta_contracts::PrincipalId;
use hepta_contracts::ReceiptId;
use hepta_contracts::RevisionStamp;
use sha2::Digest;
use sha2::Sha256;

use super::ToolExecutorBinding;

const SENSITIVE_DOMAIN: &str = "hepta.runtime.tool-outcome.sensitive.v1";
const EXECUTOR_BINDING_DOMAIN: &str = "hepta.runtime.tool-executor-binding.v1";
const RECEIPT_ID_DOMAIN: &str = "hepta.runtime.outcome-receipt-id.v1";
const RECEIPT_DOMAIN: &str = "hepta.runtime.outcome-receipt.v1";

pub(super) trait FrameSink {
    fn text(&mut self, name: &str, value: &str);
    fn number(&mut self, name: &str, value: u64);
}

pub(super) struct HashFrames(Sha256);

impl HashFrames {
    pub(super) fn new(domain: &str) -> Self {
        let mut value = Self(Sha256::new());
        value.bytes("domain", domain.as_bytes());
        value
    }

    fn bytes(&mut self, name: &str, value: &[u8]) {
        self.0.update((name.len() as u64).to_be_bytes());
        self.0.update(name.as_bytes());
        self.0.update((value.len() as u64).to_be_bytes());
        self.0.update(value);
    }

    pub(super) fn finish(self) -> ContentHash {
        ContentHash::new(format!("sha256:{:x}", self.0.finalize()))
    }
}

impl FrameSink for HashFrames {
    fn text(&mut self, name: &str, value: &str) {
        self.bytes(name, value.as_bytes());
    }

    fn number(&mut self, name: &str, value: u64) {
        self.bytes(name, &value.to_be_bytes());
    }
}

pub(super) fn receipt_id(
    attempt_id: &str,
    authorization: &AuthorizationRef,
    outcome_hash: &ContentHash,
) -> ReceiptId {
    let revision = authorization.revision().get().to_be_bytes();
    let hash = framed_hash(
        RECEIPT_ID_DOMAIN,
        &[
            ("attempt.id", attempt_id.as_bytes()),
            ("authorization.id", authorization.id().as_str().as_bytes()),
            ("authorization.revision", &revision),
            (
                "authorization.content_hash",
                authorization.content_hash().as_str().as_bytes(),
            ),
            ("outcome.hash", outcome_hash.as_str().as_bytes()),
        ],
    );
    ReceiptId::new(format!("receipt:{}", hash.as_str()))
}

#[allow(clippy::too_many_arguments)]
pub(super) fn receipt_hash(
    id: &ReceiptId,
    candidate: &CandidateRef,
    authorization: &AuthorizationRef,
    capability: &CapabilityManifestRef,
    payload_hash: &ContentHash,
    executor: &ToolExecutorBinding,
    status: &OutcomeStatus,
    outcome_hash: &ContentHash,
) -> ContentHash {
    let mut hash = HashFrames::new(RECEIPT_DOMAIN);
    hash.text("receipt.id", id.as_str());
    append_candidate(&mut hash, "candidate", candidate);
    hash.text("authorization.id", authorization.id().as_str());
    hash.number("authorization.revision", authorization.revision().get());
    hash.text(
        "authorization.content_hash",
        authorization.content_hash().as_str(),
    );
    append_capability(&mut hash, "capability", capability);
    hash.text("payload_set_hash", payload_hash.as_str());
    hash.text("executor", executor.executor.as_str());
    hash.text("executor.provider", &executor.provider);
    hash.text(
        "executor.manifest_hash",
        executor.executor_manifest_hash.as_str(),
    );
    hash.text("executor.binding_hash", executor.binding_hash.as_str());
    let (tag, code) = match status {
        OutcomeStatus::Succeeded => ("succeeded", ""),
        OutcomeStatus::Failed { error_code } => ("failed", error_code.as_str()),
        OutcomeStatus::Cancelled { reason_code } => ("cancelled", reason_code.as_str()),
        _ => ("unknown", "unknown"),
    };
    hash.text("status.tag", tag);
    hash.text("status.code", code);
    hash.text("outcome.hash", outcome_hash.as_str());
    hash.finish()
}

pub(super) fn executor_binding_hash(
    capability: &CapabilityManifestRef,
    executor: &PrincipalId,
    provider: &str,
    executor_manifest_hash: &ContentHash,
) -> ContentHash {
    let mut hash = HashFrames::new(EXECUTOR_BINDING_DOMAIN);
    append_capability(&mut hash, "capability", capability);
    hash.text("executor", executor.as_str());
    hash.text("executor.provider", provider);
    hash.text("executor.manifest_hash", executor_manifest_hash.as_str());
    hash.finish()
}

pub(super) fn append_candidate(out: &mut impl FrameSink, prefix: &str, candidate: &CandidateRef) {
    out.text(&format!("{prefix}.id"), candidate.id().as_str());
    out.number(&format!("{prefix}.revision"), candidate.revision().get());
    out.text(
        &format!("{prefix}.content_hash"),
        candidate.content_hash().as_str(),
    );
    append_context(out, &format!("{prefix}.context"), candidate.context());
    out.text(
        &format!("{prefix}.action_hash"),
        candidate.action_hash().as_str(),
    );
    out.text(
        &format!("{prefix}.metacontrol_hash"),
        candidate.metacontrol_hash().as_str(),
    );
    out.text(
        &format!("{prefix}.payload_set_hash"),
        candidate.payload_set_hash().as_str(),
    );
}

pub(super) fn append_capability(
    out: &mut impl FrameSink,
    prefix: &str,
    capability: &CapabilityManifestRef,
) {
    out.text(&format!("{prefix}.id"), capability.id().as_str());
    out.number(&format!("{prefix}.revision"), capability.revision().get());
    out.text(
        &format!("{prefix}.manifest_hash"),
        capability.manifest_hash().as_str(),
    );
    append_stamp(out, &format!("{prefix}.catalog"), capability.catalog());
}

pub(super) fn append_context(out: &mut impl FrameSink, prefix: &str, context: &FrozenTurnContext) {
    out.text(
        &format!("{prefix}.observation.id"),
        context.observation().id().as_str(),
    );
    out.number(
        &format!("{prefix}.observation.revision"),
        context.observation().revision().get(),
    );
    out.text(
        &format!("{prefix}.observation.content_hash"),
        context.observation().content_hash().as_str(),
    );
    append_stamp(out, &format!("{prefix}.state"), context.state());
    append_stamp(out, &format!("{prefix}.policy"), context.policy());
    append_stamp(
        out,
        &format!("{prefix}.capability_catalog"),
        context.capability_catalog(),
    );
    append_stamp(out, &format!("{prefix}.preference"), context.preference());
}

pub(super) fn append_stamp(out: &mut impl FrameSink, prefix: &str, stamp: &RevisionStamp) {
    out.number(&format!("{prefix}.revision"), stamp.revision().get());
    out.text(
        &format!("{prefix}.content_hash"),
        stamp.content_hash().as_str(),
    );
}

pub(super) fn sensitive_hash(label: &str, raw: &str) -> ContentHash {
    framed_hash(
        SENSITIVE_DOMAIN,
        &[("label", label.as_bytes()), ("content", raw.as_bytes())],
    )
}

fn framed_hash(domain: &str, fields: &[(&str, &[u8])]) -> ContentHash {
    let mut hash = HashFrames::new(domain);
    for (name, value) in fields {
        hash.bytes(name, value);
    }
    hash.finish()
}
