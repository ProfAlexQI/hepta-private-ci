use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use sha2::Digest as _;

use crate::FrozenSshsigTrustPolicyV8;
use crate::QualificationError;
use crate::SshsigTrustPurposeV8;

const PRINCIPAL: &str = "linux-v8-operator@example";
const NAMESPACE: &str = "hepta-linux-v8-execution";
const ARMOR_BEGIN: &[u8] = b"-----BEGIN SSH SIGNATURE-----\n";
const ARMOR_END: &[u8] = b"-----END SSH SIGNATURE-----\n";
static NEXT_TEMP_DIRECTORY: AtomicU64 = AtomicU64::new(0);

#[test]
fn verifies_real_openssh_sha256_sshsig_and_binds_observation() {
    let fixture = SshsigFixture::new();
    let statement = b"canonical Linux v8 one-shot authority";
    let signature = fixture.sign(statement, NAMESPACE, true, "accepted");
    let observation = verify_sshsig_ed25519_v8(
        statement,
        &signature,
        &fixture.allowed_signers,
        PRINCIPAL,
        NAMESPACE,
    )
    .expect("verify exact in-process SSHSIG");
    let policy = fixture.policy(NAMESPACE);
    assert!(
        observation.exactly_matches(&sha256(&signature), &sha256(statement), policy.binding(),)
    );
}

#[test]
fn rejects_wrong_namespace_and_statement() {
    let fixture = SshsigFixture::new();
    let statement = b"canonical statement";
    let signature = fixture.sign(statement, NAMESPACE, true, "namespace");
    assert!(
        verify_sshsig_ed25519_v8(
            statement,
            &signature,
            &fixture.allowed_signers,
            PRINCIPAL,
            "hepta-linux-v8-break-glass",
        )
        .is_err()
    );
    assert!(
        verify_sshsig_ed25519_v8(
            b"different statement",
            &signature,
            &fixture.allowed_signers,
            PRINCIPAL,
            NAMESPACE,
        )
        .is_err()
    );
}

#[test]
fn rejects_corrupted_signature_and_wrong_key() {
    let fixture = SshsigFixture::new();
    let other = SshsigFixture::new();
    let statement = b"canonical statement";
    let signature = fixture.sign(statement, NAMESPACE, true, "signature");
    let corrupted = corrupt_final_signature_byte(&signature);
    assert!(
        verify_sshsig_ed25519_v8(
            statement,
            &corrupted,
            &fixture.allowed_signers,
            PRINCIPAL,
            NAMESPACE,
        )
        .is_err()
    );
    assert!(
        verify_sshsig_ed25519_v8(
            statement,
            &signature,
            &other.allowed_signers,
            PRINCIPAL,
            NAMESPACE,
        )
        .is_err()
    );
}

#[test]
fn rejects_wrong_principal_and_noncanonical_allowed_signers() {
    let fixture = SshsigFixture::new();
    let statement = b"canonical statement";
    let signature = fixture.sign(statement, NAMESPACE, true, "principal");
    assert!(
        verify_sshsig_ed25519_v8(
            statement,
            &signature,
            &fixture.allowed_signers,
            "different@example",
            NAMESPACE,
        )
        .is_err()
    );

    let crlf = with_crlf(&fixture.allowed_signers);
    assert!(verify_sshsig_ed25519_v8(statement, &signature, &crlf, PRINCIPAL, NAMESPACE,).is_err());
    let mut extra = fixture.allowed_signers.clone();
    extra.extend_from_slice(&fixture.allowed_signers);
    assert!(
        verify_sshsig_ed25519_v8(statement, &signature, &extra, PRINCIPAL, NAMESPACE,).is_err()
    );
    let mut trailing = fixture.allowed_signers.clone();
    trailing.extend_from_slice(b" ");
    assert!(
        verify_sshsig_ed25519_v8(statement, &signature, &trailing, PRINCIPAL, NAMESPACE,).is_err()
    );
}

#[test]
fn rejects_noncanonical_armor_and_trailing_bytes() {
    let fixture = SshsigFixture::new();
    let statement = b"canonical statement";
    let signature = fixture.sign(statement, NAMESPACE, true, "armor");

    let crlf = with_crlf(&signature);
    assert!(
        verify_sshsig_ed25519_v8(
            statement,
            &crlf,
            &fixture.allowed_signers,
            PRINCIPAL,
            NAMESPACE,
        )
        .is_err()
    );
    let mut trailing = signature.clone();
    trailing.extend_from_slice(b"\n");
    assert!(
        verify_sshsig_ed25519_v8(
            statement,
            &trailing,
            &fixture.allowed_signers,
            PRINCIPAL,
            NAMESPACE,
        )
        .is_err()
    );

    let mut irregular = signature;
    let body_start = ARMOR_BEGIN.len();
    let first_newline = irregular[body_start..]
        .iter()
        .position(|byte| *byte == b'\n')
        .expect("base64 line")
        + body_start;
    irregular.insert(first_newline - 1, b'\n');
    assert!(
        verify_sshsig_ed25519_v8(
            statement,
            &irregular,
            &fixture.allowed_signers,
            PRINCIPAL,
            NAMESPACE,
        )
        .is_err()
    );
}

#[test]
fn rejects_sha512_envelope_and_weak_anchor() {
    let fixture = SshsigFixture::new();
    let statement = b"canonical statement";
    let sha512_signature = fixture.sign(statement, NAMESPACE, false, "sha512");
    assert!(
        verify_sshsig_ed25519_v8(
            statement,
            &sha512_signature,
            &fixture.allowed_signers,
            PRINCIPAL,
            NAMESPACE,
        )
        .is_err()
    );

    let mut weak_blob = ssh_string(b"ssh-ed25519");
    weak_blob.extend(ssh_string(&[0_u8; 32]));
    let weak_allowed = format!("{PRINCIPAL} ssh-ed25519 {}\n", STANDARD.encode(weak_blob));
    assert!(
        verify_sshsig_ed25519_v8(
            statement,
            &fixture.sign(statement, NAMESPACE, true, "weak"),
            weak_allowed.as_bytes(),
            PRINCIPAL,
            NAMESPACE,
        )
        .is_err()
    );
}

struct SshsigFixture {
    _temporary: TemporaryDirectory,
    allowed_signers: Vec<u8>,
    key: PathBuf,
}

impl SshsigFixture {
    fn new() -> Self {
        let temporary = TemporaryDirectory::new();
        let key = temporary.path().join("operator-key");
        let generated = Command::new("/usr/bin/ssh-keygen")
            .args(["-q", "-t", "ed25519", "-N", "", "-f"])
            .arg(&key)
            .output()
            .expect("start ssh-keygen fixture generation");
        assert!(
            generated.status.success(),
            "generate Ed25519 fixture: {}",
            String::from_utf8_lossy(&generated.stderr)
        );

        let public =
            std::fs::read_to_string(key.with_extension("pub")).expect("read fixture public key");
        let fields = public.split_ascii_whitespace().collect::<Vec<_>>();
        assert_eq!(fields.first().copied(), Some("ssh-ed25519"));
        let encoded_key = fields.get(1).expect("public key base64");
        let public_key_blob = STANDARD
            .decode(encoded_key)
            .expect("decode public key blob");
        let allowed_signers = format!("{PRINCIPAL} ssh-ed25519 {encoded_key}\n").into_bytes();
        let fingerprint = format!(
            "SHA256:{}",
            STANDARD_NO_PAD.encode(sha2::Sha256::digest(&public_key_blob))
        );
        let fingerprint_output = Command::new("/usr/bin/ssh-keygen")
            .args(["-E", "sha256", "-lf"])
            .arg(key.with_extension("pub"))
            .output()
            .expect("calculate OpenSSH fixture fingerprint");
        assert!(fingerprint_output.status.success());
        let openssh_fingerprint = std::str::from_utf8(&fingerprint_output.stdout)
            .expect("OpenSSH fingerprint output is UTF-8")
            .split_ascii_whitespace()
            .nth(1)
            .expect("OpenSSH fingerprint field");
        assert_eq!(fingerprint, openssh_fingerprint);
        Self {
            _temporary: temporary,
            allowed_signers,
            key,
        }
    }

    fn policy(&self, namespace: &str) -> FrozenSshsigTrustPolicyV8 {
        FrozenSshsigTrustPolicyV8::for_test_only(
            self.allowed_signers.clone(),
            PRINCIPAL.to_string(),
            namespace.to_string(),
            SshsigTrustPurposeV8::OneShotRunAuthority,
        )
        .expect("valid test-only frozen policy")
    }

    fn sign(&self, statement: &[u8], namespace: &str, sha256: bool, name: &str) -> Vec<u8> {
        let statement_path = self._temporary.path().join(name);
        std::fs::write(&statement_path, statement).expect("write fixture statement");
        let mut command = Command::new("/usr/bin/ssh-keygen");
        command.args(["-Y", "sign", "-f"]).arg(&self.key);
        command.args(["-n", namespace]);
        if sha256 {
            command.args(["-O", "hashalg=sha256"]);
        }
        let signed = command
            .arg(&statement_path)
            .output()
            .expect("start ssh-keygen SSHSIG fixture signer");
        assert!(
            signed.status.success(),
            "sign SSHSIG fixture: {}",
            String::from_utf8_lossy(&signed.stderr)
        );
        std::fs::read(format!("{}.sig", statement_path.display())).expect("read SSHSIG fixture")
    }
}

fn verify_sshsig_ed25519_v8(
    statement: &[u8],
    signature_bytes: &[u8],
    allowed_signers_bytes: &[u8],
    expected_principal: &str,
    expected_namespace: &str,
) -> Result<crate::CryptographicSignatureObservation, QualificationError> {
    let policy = FrozenSshsigTrustPolicyV8::for_test_only(
        allowed_signers_bytes.to_vec(),
        expected_principal.to_string(),
        expected_namespace.to_string(),
        SshsigTrustPurposeV8::OneShotRunAuthority,
    )?;
    super::verify_sshsig_ed25519_v8(statement, signature_bytes, &policy)
}

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new() -> Self {
        let sequence = NEXT_TEMP_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "hepta-linux-v8-sshsig-{}-{sequence}",
            std::process::id()
        ));
        std::fs::create_dir(&path).expect("create SSHSIG fixture directory");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn corrupt_final_signature_byte(armored: &[u8]) -> Vec<u8> {
    let body = &armored[ARMOR_BEGIN.len()..armored.len() - ARMOR_END.len()];
    let encoded = body
        .iter()
        .copied()
        .filter(|byte| *byte != b'\n')
        .collect::<Vec<_>>();
    let mut packet = STANDARD.decode(encoded).expect("decode SSHSIG packet");
    *packet.last_mut().expect("signature packet byte") ^= 1;
    canonical_armor(&packet)
}

fn canonical_armor(packet: &[u8]) -> Vec<u8> {
    let encoded = STANDARD.encode(packet);
    let mut armored = ARMOR_BEGIN.to_vec();
    for line in encoded.as_bytes().chunks(70) {
        armored.extend_from_slice(line);
        armored.push(b'\n');
    }
    armored.extend_from_slice(ARMOR_END);
    armored
}

fn ssh_string(bytes: &[u8]) -> Vec<u8> {
    let mut encoded = u32::try_from(bytes.len())
        .expect("test SSH string length")
        .to_be_bytes()
        .to_vec();
    encoded.extend_from_slice(bytes);
    encoded
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

fn with_crlf(bytes: &[u8]) -> Vec<u8> {
    let mut converted =
        Vec::with_capacity(bytes.len() + bytes.iter().filter(|b| **b == b'\n').count());
    for byte in bytes {
        if *byte == b'\n' {
            converted.push(b'\r');
        }
        converted.push(*byte);
    }
    converted
}
