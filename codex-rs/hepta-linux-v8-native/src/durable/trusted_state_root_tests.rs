use super::*;

#[test]
fn layout_and_profile_digests_are_deterministic_and_exact() {
    let layout = state_root_layout_manifest_sha256_v8();
    assert_eq!(layout.len(), 64);
    assert_eq!(layout, state_root_layout_manifest_sha256_v8());
    let profile =
        state_root_profile_sha256_v8("/var/lib/hepta-linux-v8", 0, 0, 0o700, &layout).unwrap();
    assert_eq!(profile.len(), 64);
    assert!(state_root_profile_sha256_v8("relative", 0, 0, 0o700, &layout).is_err());
    assert!(state_root_profile_sha256_v8("/var/lib/hepta-linux-v8", 0, 0, 0o755, &layout).is_err());
    assert_ne!(
        profile,
        state_root_profile_sha256_v8("/var/lib/hepta-linux-v8", 0, 0, 0o700, &"2".repeat(64),)
            .unwrap()
    );

    let machine = "1".repeat(64);
    let binding = trusted_state_root_binding_sha256_v8(&machine, &profile, &layout).unwrap();
    assert_eq!(binding.len(), 64);
    assert_ne!(
        binding,
        trusted_state_root_binding_sha256_v8(&"2".repeat(64), &profile, &layout).unwrap()
    );
}

#[test]
fn bootstrap_binary_cannot_self_publish_a_production_profile() {
    let error = open_production_trusted_state_root_v8()
        .expect_err("bootstrap must remain NO_AUTHORITY until external profile publication");
    assert!(
        error
            .to_string()
            .contains("production state-root profile is not independently published")
    );
}

#[cfg(target_os = "linux")]
mod linux {
    use std::ffi::CString;
    use std::fs;
    use std::os::unix::ffi::OsStrExt as _;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    use super::*;
    use crate::observe_machine_id_v8;

    struct TestRoot {
        path: PathBuf,
    }

    impl TestRoot {
        fn new(label: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "hepta-v8-trusted-root-{label}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            fs::create_dir(&path).unwrap();
            fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
            for (name, _) in REQUIRED_DIRECTORIES_V8 {
                let child = path.join(name);
                fs::create_dir(&child).unwrap();
                fs::set_permissions(child, fs::Permissions::from_mode(0o700)).unwrap();
            }
            let lock = path.join(STATE_ROOT_LOCK_LEAF_V8);
            fs::write(&lock, b"").unwrap();
            fs::set_permissions(lock, fs::Permissions::from_mode(0o600)).unwrap();
            Self { path }
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn exact_layout_opens_as_an_opaque_locked_root() {
        let root = TestRoot::new("exact");
        let observed = observe_machine_id_v8().unwrap();
        let trusted = open_test_trusted_state_root_v8(&root.path).unwrap();
        assert_eq!(trusted.identity().mode(), 0o700);
        assert_eq!(trusted.machine_id_sha256(), observed.machine_id_sha256());
        assert_eq!(
            trusted.layout_manifest_sha256(),
            state_root_layout_manifest_sha256_v8()
        );
        trusted.revalidate().unwrap();
        let anchor = DirectoryAnchorV8::open(&root.path).unwrap();
        assert!(anchor.trusted_node_metadata().unwrap().mount_id() > 0);
        assert!(root.path.join(STATE_ROOT_LOCK_LEAF_V8).exists());
    }

    fn set_user_xattr(path: &Path) {
        let path = CString::new(path.as_os_str().as_bytes()).unwrap();
        // SAFETY: both C strings and the one-byte value remain live and the
        // kernel retains no pointers after setxattr returns.
        let result = unsafe {
            libc::setxattr(
                path.as_ptr(),
                c"user.hepta-v8-unapproved".as_ptr(),
                b"x".as_ptr().cast(),
                1,
                0,
            )
        };
        assert_eq!(
            result,
            0,
            "set disposable xattr: {}",
            std::io::Error::last_os_error()
        );
    }

    #[test]
    fn root_directory_and_lock_xattrs_fail_closed() {
        let root_xattr = TestRoot::new("root-xattr");
        set_user_xattr(&root_xattr.path);
        assert!(open_test_trusted_state_root_v8(&root_xattr.path).is_err());

        let directory_xattr = TestRoot::new("directory-xattr");
        set_user_xattr(&directory_xattr.path.join(JOURNAL_DIRECTORY_V8));
        assert!(open_test_trusted_state_root_v8(&directory_xattr.path).is_err());

        let lock_xattr = TestRoot::new("lock-xattr");
        set_user_xattr(&lock_xattr.path.join(STATE_ROOT_LOCK_LEAF_V8));
        assert!(open_test_trusted_state_root_v8(&lock_xattr.path).is_err());
    }

    #[test]
    fn unknown_missing_incoming_and_wrong_mode_are_rejected() {
        let unknown = TestRoot::new("unknown");
        fs::write(unknown.path.join("unknown"), b"x").unwrap();
        assert!(open_test_trusted_state_root_v8(&unknown.path).is_err());

        let missing = TestRoot::new("missing");
        fs::remove_dir(missing.path.join(INSTALL_EPOCH_DIRECTORY_V8)).unwrap();
        assert!(open_test_trusted_state_root_v8(&missing.path).is_err());

        let incoming = TestRoot::new("incoming");
        fs::write(incoming.path.join(".record.123.incoming"), b"x").unwrap();
        assert!(open_test_trusted_state_root_v8(&incoming.path).is_err());

        let wrong_mode = TestRoot::new("wrong-mode");
        fs::set_permissions(
            wrong_mode.path.join(JOURNAL_DIRECTORY_V8),
            fs::Permissions::from_mode(0o755),
        )
        .unwrap();
        assert!(open_test_trusted_state_root_v8(&wrong_mode.path).is_err());
    }

    #[test]
    fn symlink_and_active_attempt_identity_substitution_are_rejected() {
        use std::os::unix::fs::symlink;

        let symlinked = TestRoot::new("symlink");
        fs::remove_dir(symlinked.path.join(QUARANTINE_DIRECTORY_V8)).unwrap();
        symlink(
            Path::new("journal"),
            symlinked.path.join(QUARANTINE_DIRECTORY_V8),
        )
        .unwrap();
        assert!(open_test_trusted_state_root_v8(&symlinked.path).is_err());

        let active = TestRoot::new("active-mode");
        fs::write(active.path.join(ACTIVE_ATTEMPT_LEAF_V8), b"not-authority").unwrap();
        fs::set_permissions(
            active.path.join(ACTIVE_ATTEMPT_LEAF_V8),
            fs::Permissions::from_mode(0o644),
        )
        .unwrap();
        assert!(open_test_trusted_state_root_v8(&active.path).is_err());
    }
}
