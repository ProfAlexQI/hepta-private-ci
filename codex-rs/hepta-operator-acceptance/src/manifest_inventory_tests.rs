use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use pretty_assertions::assert_eq;
use tempfile::TempDir;

use super::VerifiedManifest;
use super::parse_manifest;
use crate::durable::sha256;
use crate::test_support::private_tempdir;

#[test]
fn manifest_parser_requires_sorted_safe_unique_paths() {
    let first = "0".repeat(64);
    let second = "1".repeat(64);
    let valid = format!("{first}  a/file\n{second}  b\n");
    let parsed = parse_manifest(valid.as_bytes()).expect("valid manifest");
    assert_eq!(parsed.keys().cloned().collect::<Vec<_>>(), ["a/file", "b"]);

    for invalid in [
        format!("{first}  ../escape\n"),
        format!("{first}  /absolute\n"),
        format!("{first}  field\tseparator\n"),
        format!("{first}  delete\u{7f}control\n"),
        format!("{first}  b\n{second}  a\n"),
        format!("{first}  duplicate\n{second}  duplicate\n"),
        format!("{first}  no-final-newline"),
    ] {
        assert!(parse_manifest(invalid.as_bytes()).is_err(), "{invalid:?}");
    }
}

#[test]
fn manifest_parser_rejects_malformed_unicode_without_panicking() {
    let mut crossing_boundary = vec![b'0'; 63];
    crossing_boundary.extend_from_slice("é".as_bytes());
    crossing_boundary.extend_from_slice(b"  artifact\n");
    let result = std::panic::catch_unwind(|| parse_manifest(&crossing_boundary));
    assert!(result.is_ok(), "malformed Unicode must not panic");
    assert!(result.expect("no panic").is_err());

    let mut invalid_path = vec![b'0'; 64];
    invalid_path.extend_from_slice(b"  artifact-");
    invalid_path.push(0xff);
    invalid_path.push(b'\n');
    let result = std::panic::catch_unwind(|| parse_manifest(&invalid_path));
    assert!(result.is_ok(), "invalid UTF-8 must not panic");
    assert!(result.expect("no panic").is_err());
}

#[test]
fn verified_manifest_rejects_extra_files_and_hardlinks() {
    let fixture = Fixture::new();
    fixture.write("artifact", b"sealed");
    let sums = fixture.seal(&[("artifact", b"sealed")]);
    VerifiedManifest::load(&fixture.root, &sha256(&sums), 1).expect("sealed fixture");

    fixture.write("extra", b"unsealed");
    assert!(VerifiedManifest::load(&fixture.root, &sha256(&sums), 1).is_err());
    std::fs::remove_file(fixture.root.join("extra")).expect("remove test extra");

    std::fs::hard_link(fixture.root.join("artifact"), fixture.root.join("alias"))
        .expect("create test hardlink");
    let linked_sums = fixture.seal(&[("alias", b"sealed"), ("artifact", b"sealed")]);
    assert!(VerifiedManifest::load(&fixture.root, &sha256(&linked_sums), 2).is_err());
}

#[test]
fn explicitly_bounded_manifest_read_accepts_just_over_one_mib_but_not_over_two() {
    const ONE_MIB: usize = 1024 * 1024;
    const TWO_MIB: usize = 2 * ONE_MIB;

    let fixture = Fixture::new();
    let large = vec![b'x'; ONE_MIB + 32_792];
    fixture.write("expected-git-blobs.tsv", &large);
    let sums = fixture.seal(&[("expected-git-blobs.tsv", &large)]);
    let manifest =
        VerifiedManifest::load(&fixture.root, &sha256(&sums), 1).expect("sealed >1 MiB fixture");
    assert!(manifest.bytes("expected-git-blobs.tsv").is_err());
    assert_eq!(
        manifest
            .bytes_bounded("expected-git-blobs.tsv", TWO_MIB)
            .expect("compiled 2 MiB path bound"),
        large
    );

    let over = Fixture::new();
    let too_large = vec![b'y'; TWO_MIB + 1];
    over.write("expected-git-blobs.tsv", &too_large);
    let sums = over.seal(&[("expected-git-blobs.tsv", &too_large)]);
    let manifest =
        VerifiedManifest::load(&over.root, &sha256(&sums), 1).expect("sealed >2 MiB fixture");
    assert!(
        manifest
            .bytes_bounded("expected-git-blobs.tsv", TWO_MIB)
            .is_err()
    );
}

#[cfg(unix)]
#[test]
fn terminal_reverification_rejects_content_and_metadata_drift() {
    use std::os::unix::fs::PermissionsExt;

    let content = Fixture::new();
    content.write("artifact", b"sealed");
    let sums = content.seal(&[("artifact", b"sealed")]);
    let verified =
        VerifiedManifest::load(&content.root, &sha256(&sums), 1).expect("sealed fixture");
    std::fs::set_permissions(
        content.root.join("artifact"),
        std::fs::Permissions::from_mode(0o600),
    )
    .expect("make artifact writable for mutation");
    std::fs::write(content.root.join("artifact"), b"mutated").expect("mutate artifact");
    assert!(verified.reverify().is_err());

    let metadata = Fixture::new();
    metadata.write("artifact", b"sealed");
    let sums = metadata.seal(&[("artifact", b"sealed")]);
    let verified =
        VerifiedManifest::load(&metadata.root, &sha256(&sums), 1).expect("sealed fixture");
    std::fs::set_permissions(
        metadata.root.join("artifact"),
        std::fs::Permissions::from_mode(0o400),
    )
    .expect("change sealed artifact mode");
    assert!(verified.reverify().is_err());
}

#[cfg(unix)]
#[test]
fn verified_manifest_rejects_symlinks() {
    use std::os::unix::fs::symlink;

    let fixture = Fixture::new();
    fixture.write("target", b"sealed");
    symlink(fixture.root.join("target"), fixture.root.join("alias")).expect("create test symlink");
    let sums = fixture.seal(&[("alias", b"sealed"), ("target", b"sealed")]);
    assert!(VerifiedManifest::load(&fixture.root, &sha256(&sums), 2).is_err());
}

#[cfg(target_os = "macos")]
#[test]
fn verified_manifest_rejects_extended_attributes_and_acls() {
    use std::process::Command;

    let xattr = Fixture::new();
    xattr.write("artifact", b"sealed");
    let sums = xattr.seal(&[("artifact", b"sealed")]);
    let xattr_status = Command::new("/usr/bin/xattr")
        .args(["-w", "com.hepta.v3-test", "present"])
        .arg(xattr.root.join("artifact"))
        .status()
        .expect("invoke xattr");
    assert!(xattr_status.success());
    assert!(VerifiedManifest::load(&xattr.root, &sha256(&sums), 1).is_err());

    let acl = Fixture::new();
    acl.write("artifact", b"sealed");
    let sums = acl.seal(&[("artifact", b"sealed")]);
    let acl_status = Command::new("/bin/chmod")
        .args(["+a", "everyone deny delete"])
        .arg(acl.root.join("artifact"))
        .status()
        .expect("invoke chmod +a");
    assert!(acl_status.success());
    assert!(VerifiedManifest::load(&acl.root, &sha256(&sums), 1).is_err());
}

struct Fixture {
    _temporary: TempDir,
    root: std::path::PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let temporary = private_tempdir("temporary manifest fixture directory");
        let root = temporary
            .path()
            .canonicalize()
            .expect("canonical fixture root");
        Self {
            _temporary: temporary,
            root,
        }
    }

    fn write(&self, relative: &str, bytes: &[u8]) {
        write_private(&self.root.join(relative), bytes);
    }

    fn seal(&self, entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut sorted = entries.to_vec();
        sorted.sort_by_key(|(relative, _)| *relative);
        let mut sums = Vec::new();
        for (relative, bytes) in sorted {
            writeln!(sums, "{}  {relative}", sha256(bytes)).expect("write sums line");
        }
        let sums_path = self.root.join("SHA256SUMS");
        if sums_path.exists() {
            std::fs::remove_file(&sums_path).expect("replace test sums");
        }
        write_private(&sums_path, &sums);
        sums
    }
}

fn write_private(path: &Path, bytes: &[u8]) {
    let mut options = OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path).expect("create private test file");
    file.write_all(bytes).expect("write private test file");
}
