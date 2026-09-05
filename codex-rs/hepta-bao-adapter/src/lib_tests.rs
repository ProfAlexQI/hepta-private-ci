use super::*;

fn id(value: &str) -> StableId {
    let Ok(value) = StableId::new(value) else {
        panic!("test identifier must be valid");
    };
    value
}

fn digest(value: &[u8]) -> Digest32 {
    Digest32::of_bytes(value)
}

fn fixture() -> (SecretRequest, SecretLease) {
    let reference = SecretReference {
        secret_id: id("secret:1"),
        version: 3,
        secret_digest: digest(b"secret-digest"),
    };
    let request = SecretRequest {
        request_id: id("request:1"),
        reference: reference.clone(),
        scope_digest: digest(b"scope"),
        deadline_ms: 2_000,
    };
    let lease = SecretLease {
        lease_id: id("lease:1"),
        secret_id: reference.secret_id,
        version: reference.version,
        secret_digest: reference.secret_digest,
        scope_digest: request.scope_digest,
        expires_at_ms: 1_500,
        revoked: false,
    };
    (request, lease)
}

#[test]
fn exact_reference_returns_only_opaque_digest() {
    let (request, lease) = fixture();
    let Ok(receipt) = resolve(1_000, request, lease) else {
        panic!("exact secret reference must resolve");
    };
    assert!(!receipt.contains_raw_secret);
    assert!(!receipt.authority.grants_any());
}

#[test]
fn revoked_lease_is_rejected() {
    let (request, mut lease) = fixture();
    lease.revoked = true;
    assert_eq!(resolve(1_000, request, lease), Err(Error::LeaseRevoked));
}

#[test]
fn scope_drift_is_rejected() {
    let (request, mut lease) = fixture();
    lease.scope_digest = digest(b"other");
    assert_eq!(resolve(1_000, request, lease), Err(Error::ScopeMismatch));
}

#[test]
fn version_drift_is_rejected() {
    let (request, mut lease) = fixture();
    lease.version = 4;
    assert_eq!(resolve(1_000, request, lease), Err(Error::VersionMismatch));
}
