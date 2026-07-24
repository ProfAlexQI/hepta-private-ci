//! Canonical hashing for authenticated preference authority evidence.

#[path = "preference_authority/canonical.rs"]
mod framing;

use hepta_contracts::ContentHash;
use hepta_contracts::PreferenceEvidenceSignal;

use self::framing::FramedHash;
use super::types::PreferenceFeedbackRequest;
use super::types::PreferenceFeedbackSourceRef;
use super::types::PreferenceReducerRef;

const AUTHORITY_EVIDENCE_HASH_DOMAIN: &str = "hepta.memory.preference-authority.evidence.v1";

pub(super) fn authority_evidence_hash(
    request: &PreferenceFeedbackRequest,
    source: &PreferenceFeedbackSourceRef,
    reducer: &PreferenceReducerRef,
) -> ContentHash {
    let mut hash = FramedHash::new(AUTHORITY_EVIDENCE_HASH_DOMAIN);
    hash.text("source.identity", source.identity().as_str());
    hash.number("source.revision", source.revision().get());
    hash.text("source.content_hash", source.content_hash().as_str());
    hash.text("reducer.identity", reducer.identity());
    hash.text("reducer.version", reducer.version());
    hash.text("transition.id", request.transition_id().as_str());
    hash.text("evidence.id", request.evidence_id().as_str());
    hash.text(
        "evidence.signal",
        match request.signal() {
            PreferenceEvidenceSignal::Accepted => "accepted",
            PreferenceEvidenceSignal::Rejected => "rejected",
        },
    );
    hash.text("receipt.id", request.receipt().id().as_str());
    hash.text(
        "receipt.content_hash",
        request.receipt().receipt_hash().as_str(),
    );
    hash.text(
        "session_binding_hash",
        request.session_binding_hash().as_str(),
    );
    hash.text("subject", request.subject().as_str());
    hash.text("preference", request.preference().as_str());
    hash.text(
        "target_binding_hash",
        request.target_binding_hash().as_str(),
    );
    hash.number(
        "expected_previous.revision",
        request.expected_previous().revision().get(),
    );
    hash.text(
        "expected_previous.content_hash",
        request.expected_previous().content_hash().as_str(),
    );
    hash.finish()
}
