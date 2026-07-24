use hepta_contracts::CandidateRef;
use hepta_contracts::ContentHash;
use hepta_contracts::FrozenTurnContext;
use hepta_contracts::RevisionStamp;
use sha2::Digest;
use sha2::Sha256;

const CANDIDATE_REFERENCE_DOMAIN: &str = "hepta.memory.execution-intent.candidate-reference.v1";

/// Hashes every candidate field that survives in a terminal receipt.
///
/// This deliberately includes the declared candidate digest as one field
/// rather than trusting it as a digest of the remaining reference. The
/// pre-dispatch intent can therefore reject a terminal candidate whose
/// identity, revision, context, action, metacontrol, or payload was replaced
/// while its declared content hash was retained.
pub fn candidate_reference_hash(candidate: &CandidateRef) -> ContentHash {
    let mut hash = CandidateReferenceFrames::new();
    hash.text("candidate.id", candidate.id().as_str());
    hash.number("candidate.revision", candidate.revision().get());
    hash.text("candidate.content_hash", candidate.content_hash().as_str());
    append_context(&mut hash, "candidate.context", candidate.context());
    hash.text("candidate.action_hash", candidate.action_hash().as_str());
    hash.text(
        "candidate.metacontrol_hash",
        candidate.metacontrol_hash().as_str(),
    );
    hash.text(
        "candidate.payload_set_hash",
        candidate.payload_set_hash().as_str(),
    );
    hash.finish()
}

fn append_context(hash: &mut CandidateReferenceFrames, prefix: &str, context: &FrozenTurnContext) {
    hash.text(
        &format!("{prefix}.observation.id"),
        context.observation().id().as_str(),
    );
    hash.number(
        &format!("{prefix}.observation.revision"),
        context.observation().revision().get(),
    );
    hash.text(
        &format!("{prefix}.observation.content_hash"),
        context.observation().content_hash().as_str(),
    );
    append_stamp(hash, &format!("{prefix}.state"), context.state());
    append_stamp(hash, &format!("{prefix}.policy"), context.policy());
    append_stamp(
        hash,
        &format!("{prefix}.capability_catalog"),
        context.capability_catalog(),
    );
    append_stamp(hash, &format!("{prefix}.preference"), context.preference());
}

fn append_stamp(hash: &mut CandidateReferenceFrames, prefix: &str, stamp: &RevisionStamp) {
    hash.number(&format!("{prefix}.revision"), stamp.revision().get());
    hash.text(
        &format!("{prefix}.content_hash"),
        stamp.content_hash().as_str(),
    );
}

struct CandidateReferenceFrames(Sha256);

impl CandidateReferenceFrames {
    fn new() -> Self {
        let mut hash = Self(Sha256::new());
        hash.text("domain", CANDIDATE_REFERENCE_DOMAIN);
        hash
    }

    fn text(&mut self, name: &str, value: &str) {
        self.bytes(name, value.as_bytes());
    }

    fn number(&mut self, name: &str, value: u64) {
        self.bytes(name, &value.to_be_bytes());
    }

    fn bytes(&mut self, name: &str, value: &[u8]) {
        self.0.update((name.len() as u64).to_be_bytes());
        self.0.update(name.as_bytes());
        self.0.update((value.len() as u64).to_be_bytes());
        self.0.update(value);
    }

    fn finish(self) -> ContentHash {
        ContentHash::new(format!("sha256:{:x}", self.0.finalize()))
    }
}
