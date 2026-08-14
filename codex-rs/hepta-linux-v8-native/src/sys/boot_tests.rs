use super::parse_boot_id_v8;

#[test]
fn parses_only_canonical_non_nil_boot_uuid() {
    let id =
        parse_boot_id_v8(b"550e8400-e29b-41d4-a716-446655440000\n").expect("canonical boot id");
    assert_eq!(id.as_str(), "550e8400-e29b-41d4-a716-446655440000");
    assert!(parse_boot_id_v8(b"550E8400-e29b-41d4-a716-446655440000\n").is_err());
    assert!(parse_boot_id_v8(b"00000000-0000-0000-0000-000000000000\n").is_err());
    assert!(parse_boot_id_v8(b"550e8400e29b41d4a716446655440000\n").is_err());
}

#[cfg(not(target_os = "linux"))]
#[test]
fn boot_observation_fails_closed_off_linux() {
    assert!(matches!(
        super::observe_boot_id_v8(),
        Err(super::NativeSysErrorV8::UnsupportedPlatform(_))
    ));
}

#[cfg(target_os = "linux")]
#[test]
fn fixed_procfs_boot_observation_is_canonical_and_stable() {
    let boot = super::observe_boot_id_v8().expect("anchored boot id");
    assert_eq!(boot.as_str().len(), 36);
    assert_eq!(boot, super::observe_boot_id_v8().expect("stable boot id"));
}
