use super::machine_id::machine_id_mode_is_frozen_read_only_v8;
use super::machine_id::parse_machine_id_bytes_v8;
use super::observe_machine_id_v8;

const CANONICAL_MACHINE_ID: &[u8; 32] = b"0123456789abcdef0123456789abcdef";

#[test]
fn parser_accepts_exact_canonical_bytes_and_one_optional_lf() {
    assert_eq!(
        parse_machine_id_bytes_v8(CANONICAL_MACHINE_ID).expect("parse canonical machine-id"),
        *CANONICAL_MACHINE_ID
    );

    let mut with_lf = CANONICAL_MACHINE_ID.to_vec();
    with_lf.push(b'\n');
    assert_eq!(
        parse_machine_id_bytes_v8(&with_lf).expect("parse machine-id with LF"),
        *CANONICAL_MACHINE_ID
    );
}

#[test]
fn parser_rejects_noncanonical_or_zero_sources() {
    let mut uppercase = CANONICAL_MACHINE_ID.to_vec();
    uppercase[10] = b'A';
    let mut embedded_lf = CANONICAL_MACHINE_ID.to_vec();
    embedded_lf[10] = b'\n';
    let mut trailing_space = CANONICAL_MACHINE_ID.to_vec();
    trailing_space.push(b' ');
    let mut crlf = CANONICAL_MACHINE_ID.to_vec();
    crlf.extend_from_slice(b"\r\n");
    let mut double_lf = CANONICAL_MACHINE_ID.to_vec();
    double_lf.extend_from_slice(b"\n\n");
    let mut nul = CANONICAL_MACHINE_ID.to_vec();
    nul[10] = 0;

    for invalid in [
        Vec::new(),
        CANONICAL_MACHINE_ID[..31].to_vec(),
        [CANONICAL_MACHINE_ID.as_slice(), b"0"].concat(),
        b"00000000000000000000000000000000".to_vec(),
        b"00000000000000000000000000000000\n".to_vec(),
        uppercase,
        embedded_lf,
        trailing_space,
        crlf,
        double_lf,
        nul,
        b"gggggggggggggggggggggggggggggggg".to_vec(),
    ] {
        assert!(
            parse_machine_id_bytes_v8(&invalid).is_err(),
            "invalid source unexpectedly parsed: {invalid:?}"
        );
    }
}

#[test]
fn frozen_source_modes_are_read_only_and_exact() {
    assert!(machine_id_mode_is_frozen_read_only_v8(0o444));
    for rejected in [
        0, 0o004, 0o040, 0o400, 0o404, 0o440, 0o441, 0o500, 0o600, 0o640, 0o644, 0o2444, 0o4444,
    ] {
        assert!(!machine_id_mode_is_frozen_read_only_v8(rejected));
    }
}

#[cfg(target_os = "linux")]
#[test]
fn observes_the_real_fixed_linux_machine_id() {
    let observed = observe_machine_id_v8().expect("observe fixed /etc/machine-id");
    let digest = observed.machine_id_sha256();
    let identity = observed.source_identity();

    assert_eq!(digest.len(), 64);
    assert!(!digest.bytes().all(|byte| byte == b'0'));
    assert!(
        digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    );
    assert_eq!(identity.owner_uid(), 0);
    assert_eq!(identity.owner_gid(), 0);
    assert_eq!(identity.link_count(), 1);
    assert!(machine_id_mode_is_frozen_read_only_v8(identity.mode()));
    assert!(matches!(identity.size(), 32 | 33));
    observed
        .revalidate_descriptor_bound_v8()
        .expect("retained machine-id descriptor and fixed pathname replay exactly");
}

#[cfg(not(target_os = "linux"))]
#[test]
fn observer_fails_closed_off_linux() {
    let error = observe_machine_id_v8().expect_err("non-Linux observation must fail closed");
    assert!(matches!(
        error,
        super::NativeSysErrorV8::UnsupportedPlatform("observe fixed /etc/machine-id")
    ));
}
