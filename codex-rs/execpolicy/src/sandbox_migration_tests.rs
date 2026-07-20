use super::*;
use pretty_assertions::assert_eq;
use tempfile::tempdir;

const BANNED_PREFIXES: &[&[&str]] = &[&["cmd.exe", "/k"], &["git"], &["pwsh", "-ec"], &["rm"]];

fn policy_path(codex_home: &Path) -> PathBuf {
    codex_home.join("rules/default.rules")
}

#[test]
fn removes_only_exact_legacy_allow_rules() {
    let codex_home = tempdir().expect("create codex home");
    let policy_path = policy_path(codex_home.path());
    fs::create_dir_all(policy_path.parent().expect("rules directory"))
        .expect("create rules directory");
    fs::write(
        &policy_path,
        r#"# keep comments and every supported non-legacy rule
prefix_rule(pattern=["git"], decision="allow")
prefix_rule(pattern=["git"], decision="prompt")
prefix_rule(pattern=["git"], decision="deny")
prefix_rule(pattern=["git", "status"], decision="allow")
prefix_rule(pattern=["CMD.EXE", "/K"], decision="allow")
prefix_rule(pattern=["PWSH", "-EC"], decision="allow")
prefix_rule(pattern=[["git", "hub"]], decision="allow")
prefix_rule(pattern=["rm"], decision="allow", justification="operator-owned")
network_rule(host="api.github.com", protocol="https", decision="allow")
"#,
    )
    .expect("write legacy policy");

    prefix_rule_migration(codex_home.path(), &policy_path, BANNED_PREFIXES).expect("run migration");

    assert_eq!(
        fs::read_to_string(&policy_path).expect("read migrated policy"),
        r#"# keep comments and every supported non-legacy rule
prefix_rule(pattern=["git"], decision="prompt")
prefix_rule(pattern=["git"], decision="deny")
prefix_rule(pattern=["git", "status"], decision="allow")
prefix_rule(pattern=[["git", "hub"]], decision="allow")
prefix_rule(pattern=["rm"], decision="allow", justification="operator-owned")
network_rule(host="api.github.com", protocol="https", decision="allow")
"#
    );
    assert_eq!(
        fs::read(codex_home.path().join(MIGRATION_MARKER_FILENAME)).expect("read migration marker"),
        MIGRATION_MARKER_CONTENTS
    );
}

#[test]
fn migration_is_one_shot_and_idempotent() {
    let codex_home = tempdir().expect("create codex home");
    let policy_path = policy_path(codex_home.path());
    fs::create_dir_all(policy_path.parent().expect("rules directory"))
        .expect("create rules directory");
    fs::write(
        &policy_path,
        "prefix_rule(pattern=[\"git\"], decision=\"allow\")\n",
    )
    .expect("write legacy policy");

    prefix_rule_migration(codex_home.path(), &policy_path, BANNED_PREFIXES)
        .expect("first migration");
    assert_eq!(fs::read_to_string(&policy_path).expect("read policy"), "");

    let post_migration_policy = "prefix_rule(pattern=[\"git\"], decision=\"allow\")\n";
    fs::write(&policy_path, post_migration_policy).expect("write new policy rule");
    prefix_rule_migration(codex_home.path(), &policy_path, BANNED_PREFIXES)
        .expect("repeat migration");
    assert_eq!(
        fs::read_to_string(&policy_path).expect("read post-migration policy"),
        post_migration_policy
    );
}

#[test]
fn invalid_marker_fails_closed_without_touching_policy() {
    let codex_home = tempdir().expect("create codex home");
    let policy_path = policy_path(codex_home.path());
    fs::create_dir_all(policy_path.parent().expect("rules directory"))
        .expect("create rules directory");
    let legacy_policy = "prefix_rule(pattern=[\"git\"], decision=\"allow\")\n";
    fs::write(&policy_path, legacy_policy).expect("write legacy policy");
    fs::write(
        codex_home.path().join(MIGRATION_MARKER_FILENAME),
        b"partial",
    )
    .expect("write invalid marker");

    let error = prefix_rule_migration(codex_home.path(), &policy_path, BANNED_PREFIXES)
        .expect_err("invalid marker must fail closed");
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    assert_eq!(
        fs::read_to_string(&policy_path).expect("read untouched policy"),
        legacy_policy
    );
}

#[test]
fn missing_policy_records_completed_migration() {
    let codex_home = tempdir().expect("create codex home");
    let policy_path = policy_path(codex_home.path());

    prefix_rule_migration(codex_home.path(), &policy_path, BANNED_PREFIXES)
        .expect("migrate missing policy");

    assert_eq!(
        fs::read(codex_home.path().join(MIGRATION_MARKER_FILENAME)).expect("read migration marker"),
        MIGRATION_MARKER_CONTENTS
    );
    assert!(!policy_path.exists());
}

#[cfg(unix)]
#[test]
fn migration_preserves_policy_symlink() {
    use std::os::unix::fs::symlink;

    let codex_home = tempdir().expect("create codex home");
    let external = tempdir().expect("create external policy directory");
    let target = external.path().join("policy.rules");
    fs::write(
        &target,
        "prefix_rule(pattern=[\"git\"], decision=\"allow\")\n",
    )
    .expect("write policy target");
    let policy_path = policy_path(codex_home.path());
    fs::create_dir_all(policy_path.parent().expect("rules directory"))
        .expect("create rules directory");
    symlink(&target, &policy_path).expect("create policy symlink");

    prefix_rule_migration(codex_home.path(), &policy_path, BANNED_PREFIXES)
        .expect("migrate symlinked policy");

    assert!(
        fs::symlink_metadata(&policy_path)
            .expect("read symlink metadata")
            .file_type()
            .is_symlink()
    );
    assert_eq!(fs::read_to_string(&target).expect("read target"), "");
}
