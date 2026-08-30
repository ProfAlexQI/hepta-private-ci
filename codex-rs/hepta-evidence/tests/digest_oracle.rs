use codex_hepta_contracts::Sha256Digest;
use sha2::Digest;
use sha2::Sha256;

#[test]
fn independent_sha2_oracle_matches_contract_digest() {
    const PAYLOAD: &[u8] = b"hepta-evidence-digest-oracle-v1";

    let contract_digest = Sha256Digest::for_bytes(PAYLOAD);
    let oracle_digest = Sha256::digest(PAYLOAD)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();

    assert_eq!(oracle_digest, contract_digest.as_str());
}
