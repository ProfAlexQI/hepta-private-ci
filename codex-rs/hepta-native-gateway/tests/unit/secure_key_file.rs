use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::symlink;

use tempfile::tempdir;

use super::*;

#[test]
fn private_key_reader_denies_symlinks_and_hardlinks() {
    let root = tempdir().expect("tempdir");
    let key = root.path().join("key");
    fs::write(
        &key,
        b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
    )
    .expect("key");
    fs::set_permissions(&key, fs::Permissions::from_mode(PRIVATE_FILE_MODE)).expect("permissions");
    let symlink_path = root.path().join("symlink");
    symlink(&key, &symlink_path).expect("symlink");
    assert!(read_private_key(&symlink_path, "KEY", "test").is_err());
    let hardlink_path = root.path().join("hardlink");
    fs::hard_link(&key, &hardlink_path).expect("hardlink");
    assert!(
        read_private_key(&key, "KEY", "test")
            .expect_err("multiple links must fail")
            .to_string()
            .contains("exactly one hard link")
    );
}

#[test]
fn private_key_reader_requires_absolute_private_regular_file() {
    let root = tempdir().expect("tempdir");
    let key = root.path().join("key");
    fs::write(
        &key,
        b"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
    )
    .expect("key");
    fs::set_permissions(&key, fs::Permissions::from_mode(0o640)).expect("permissions");
    assert!(
        read_private_key(&key, "KEY", "test")
            .expect_err("public mode must fail")
            .to_string()
            .contains("mode 0o600")
    );
    assert!(read_private_key(Path::new("relative"), "KEY", "test").is_err());
    let directory = root.path().join("directory");
    fs::create_dir(&directory).expect("directory");
    fs::set_permissions(&directory, fs::Permissions::from_mode(PRIVATE_FILE_MODE))
        .expect("directory permissions");
    assert!(read_private_key(&directory, "KEY", "test").is_err());
}
