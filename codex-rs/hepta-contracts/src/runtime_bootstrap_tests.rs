use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use ed25519_dalek::Signer as _;
use ed25519_dalek::SigningKey;

use crate::AgentId;
use crate::Sha256Digest;

use super::RuntimeBootstrapDocument;
use super::RuntimeBootstrapEnvelope;
use super::RuntimeBootstrapEnvelopeFields;
use super::RuntimeBootstrapError;
use super::RuntimeBootstrapExpectation;
use super::RuntimeBootstrapReservation;
use super::RuntimeBootstrapSignature;
use super::RuntimeBootstrapTrustRoot;
use super::verify_runtime_bootstrap;

const AGENT_ID: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const SOURCE_COMMIT: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const SOURCE_TREE: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn signing_key() -> SigningKey {
    SigningKey::from_bytes(&[7_u8; 32])
}

fn envelope() -> RuntimeBootstrapEnvelope {
    RuntimeBootstrapEnvelope::new(RuntimeBootstrapEnvelopeFields {
        subject_agent_id: AgentId::parse(AGENT_ID).expect("agent id"),
        release_id: "release-v1".to_string(),
        source_commit: SOURCE_COMMIT.to_string(),
        source_tree: SOURCE_TREE.to_string(),
        binary_sha256: Sha256Digest::for_bytes(b"binary"),
        runtime_profile: "agent_local".to_string(),
        runtime_profile_sha256: Sha256Digest::for_bytes(b"profile"),
        authority_grant_sha256: Sha256Digest::for_bytes(b"grant"),
        product_graph_sha256: Sha256Digest::for_bytes(b"graph"),
        authority_epoch: 5,
        owner_epoch: 7,
        generation: 11,
        fencing_token_sha256: Sha256Digest::for_bytes(b"fence"),
        signer_key_id: "runtime-bootstrap-key".to_string(),
        signer_epoch: 3,
        issued_at_unix_seconds: 100,
        not_before_unix_seconds: 100,
        expires_at_unix_seconds: 200,
        nonce_sha256: Sha256Digest::for_bytes(b"nonce"),
    })
    .expect("envelope")
}

fn document() -> RuntimeBootstrapDocument {
    let envelope = envelope();
    let signature = signing_key().sign(&envelope.signing_bytes());
    RuntimeBootstrapDocument::new(
        envelope.clone(),
        RuntimeBootstrapSignature::new(
            envelope.signer_key_id(),
            envelope.signer_epoch(),
            envelope.digest(),
            STANDARD.encode(signature.to_bytes()),
        )
        .expect("signature"),
    )
    .expect("document")
}

fn expectation() -> RuntimeBootstrapExpectation {
    let envelope = envelope();
    RuntimeBootstrapExpectation {
        subject_agent_id: envelope.subject_agent_id().clone(),
        release_id: envelope.release_id().to_string(),
        source_commit: envelope.source_commit().to_string(),
        source_tree: envelope.source_tree().to_string(),
        binary_sha256: envelope.binary_sha256().clone(),
        runtime_profile: envelope.runtime_profile().to_string(),
        runtime_profile_sha256: envelope.runtime_profile_sha256().clone(),
        authority_grant_sha256: envelope.authority_grant_sha256().clone(),
        product_graph_sha256: envelope.product_graph_sha256().clone(),
        authority_epoch: envelope.authority_epoch(),
        owner_epoch: envelope.owner_epoch(),
        generation: envelope.generation(),
        fencing_token_sha256: envelope.fencing_token_sha256().clone(),
        signer_key_id: envelope.signer_key_id().to_string(),
        signer_epoch: envelope.signer_epoch(),
    }
}

#[test]
fn signed_document_round_trips_and_binds_every_local_fact() {
    let document = document();
    let bytes = document.encode().expect("encode");
    let decoded = RuntimeBootstrapDocument::decode(&bytes).expect("decode");
    assert_eq!(decoded, document);

    let trust = RuntimeBootstrapTrustRoot::new(
        "runtime-bootstrap-key",
        3,
        signing_key().verifying_key().to_bytes(),
    )
    .expect("trust root");
    let verified = verify_runtime_bootstrap(
        &decoded,
        &expectation(),
        150,
        &trust.verifier().expect("verifier"),
    )
    .expect("verified bootstrap");
    assert_eq!(verified.document_sha256(), &document.digest());
    assert_eq!(verified.nonce_sha256(), document.envelope.nonce_sha256());
    assert_eq!(verified.generation(), 11);

    let reservation = RuntimeBootstrapReservation::new(&document).expect("reservation");
    reservation.validate().expect("valid reservation");
    assert_eq!(reservation.envelope_sha256, document.digest());
}

#[test]
fn local_binding_drift_fails_before_signature_acceptance() {
    let document = document();
    let mut expected = expectation();
    expected.release_id = "other-release".to_string();
    let trust = RuntimeBootstrapTrustRoot::new(
        "runtime-bootstrap-key",
        3,
        signing_key().verifying_key().to_bytes(),
    )
    .expect("trust root");
    assert!(matches!(
        verify_runtime_bootstrap(
            &document,
            &expected,
            150,
            &trust.verifier().expect("verifier")
        ),
        Err(RuntimeBootstrapError::Binding("release_id"))
    ));
}

#[test]
fn invalid_signature_and_time_windows_fail_closed() {
    let document = document();
    let wrong_key = SigningKey::from_bytes(&[9_u8; 32]);
    let wrong_trust = RuntimeBootstrapTrustRoot::new(
        "runtime-bootstrap-key",
        3,
        wrong_key.verifying_key().to_bytes(),
    )
    .expect("wrong trust root");
    assert!(matches!(
        verify_runtime_bootstrap(
            &document,
            &expectation(),
            150,
            &wrong_trust.verifier().expect("verifier")
        ),
        Err(RuntimeBootstrapError::SignatureRejected(_))
    ));

    let trust = RuntimeBootstrapTrustRoot::new(
        "runtime-bootstrap-key",
        3,
        signing_key().verifying_key().to_bytes(),
    )
    .expect("trust root");
    let verifier = trust.verifier().expect("verifier");
    assert!(matches!(
        verify_runtime_bootstrap(&document, &expectation(), 99, &verifier),
        Err(RuntimeBootstrapError::NotYetValid { not_before: 100 })
    ));
    assert!(matches!(
        verify_runtime_bootstrap(&document, &expectation(), 200, &verifier),
        Err(RuntimeBootstrapError::Expired { expires_at: 200 })
    ));
}

#[test]
fn strict_decode_rejects_unknown_and_duplicate_fields() {
    let bytes = document().encode().expect("encode");
    let source = String::from_utf8(bytes).expect("utf8");
    let unknown = source.replacen(
        "\"envelope\":{",
        "\"envelope\":{\"unexpected\":true,",
        1,
    );
    assert!(RuntimeBootstrapDocument::decode(unknown.as_bytes()).is_err());

    let duplicate = source.replacen(
        "\"schema_version\":1",
        "\"schema_version\":1,\"schema_version\":1",
        1,
    );
    assert!(RuntimeBootstrapDocument::decode(duplicate.as_bytes()).is_err());
}

#[test]
fn noncanonical_signature_and_git_identity_are_rejected() {
    let envelope = envelope();
    assert!(RuntimeBootstrapSignature::new(
        envelope.signer_key_id(),
        envelope.signer_epoch(),
        envelope.digest(),
        "not-base64"
    )
    .is_err());

    let mut fields = RuntimeBootstrapEnvelopeFields {
        subject_agent_id: AgentId::parse(AGENT_ID).expect("agent id"),
        release_id: "release-v1".to_string(),
        source_commit: SOURCE_COMMIT.to_uppercase(),
        source_tree: SOURCE_TREE.to_string(),
        binary_sha256: Sha256Digest::for_bytes(b"binary"),
        runtime_profile: "agent_local".to_string(),
        runtime_profile_sha256: Sha256Digest::for_bytes(b"profile"),
        authority_grant_sha256: Sha256Digest::for_bytes(b"grant"),
        product_graph_sha256: Sha256Digest::for_bytes(b"graph"),
        authority_epoch: 5,
        owner_epoch: 7,
        generation: 11,
        fencing_token_sha256: Sha256Digest::for_bytes(b"fence"),
        signer_key_id: "runtime-bootstrap-key".to_string(),
        signer_epoch: 3,
        issued_at_unix_seconds: 100,
        not_before_unix_seconds: 100,
        expires_at_unix_seconds: 200,
        nonce_sha256: Sha256Digest::for_bytes(b"nonce"),
    };
    assert!(RuntimeBootstrapEnvelope::new(fields.clone()).is_err());
    fields.source_commit = SOURCE_COMMIT.to_string();
    fields.expires_at_unix_seconds = 401;
    assert!(RuntimeBootstrapEnvelope::new(fields).is_err());
}
