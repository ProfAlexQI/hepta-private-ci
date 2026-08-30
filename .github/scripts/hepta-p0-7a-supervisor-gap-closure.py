#!/usr/bin/env python3
"""Apply the fail-closed P0.7a Supervisor/Fleet closure candidate.

The script is intentionally exact-anchor based. It aborts instead of guessing
when the source generation drifts.
"""
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one anchor, found {count}")
    return text.replace(old, new, 1)


def patch_registry() -> None:
    path = ROOT / "codex-rs/hepta-fleet/src/registry.rs"
    text = path.read_text()
    text = replace_once(
        text,
        '    /// One-time, fail-closed upgrade for Agent roots created before the Matrix\n'
        '    /// companion geometry existed. New private directories are staged,\n',
        '    /// One-time, fail-closed upgrade for Agent roots created before private\n'
        '    /// runtime and Matrix companion geometry existed. New private directories are staged,\n',
        "registry migration contract",
    )
    text = replace_once(
        text,
        '            validate_physical_directory(layout.agent_root())?;\n'
        '            migrate_private_directory(layout.agent_root(), "matrix")?;\n',
        '            validate_physical_directory(layout.agent_root())?;\n'
        '            migrate_private_directory(layout.agent_root(), "run")?;\n'
        '            migrate_private_directory(layout.agent_root(), "matrix")?;\n',
        "registry run-root migration",
    )
    text = replace_once(
        text,
        '            migrate_private_directory(layout.matrix_root(), "secrets")?;\n'
        '            validate_private_directory(layout.matrix_root())?;\n',
        '            migrate_private_directory(layout.matrix_root(), "secrets")?;\n'
        '            validate_private_directory(layout.run_root())?;\n'
        '            validate_private_directory(layout.matrix_root())?;\n',
        "registry migrated run-root validation",
    )
    text = replace_once(
        text,
        '        std::fs::create_dir(&matrix_secrets_root)?;\n'
        '        set_private_directory_permissions(&staging_root.join("matrix"))?;\n',
        '        std::fs::create_dir(&matrix_secrets_root)?;\n'
        '        set_private_directory_permissions(&staging_root.join("run"))?;\n'
        '        set_private_directory_permissions(&staging_root.join("matrix"))?;\n',
        "registry new run-root permissions",
    )
    text = replace_once(
        text,
        '            layout.automation_root(),\n'
        '        ] {\n'
        '            validate_physical_directory(directory)?;\n'
        '        }\n'
        '        validate_private_directory(layout.matrix_root())?;\n',
        '            layout.automation_root(),\n'
        '        ] {\n'
        '            validate_physical_directory(directory)?;\n'
        '        }\n'
        '        validate_private_directory(layout.run_root())?;\n'
        '        validate_private_directory(layout.matrix_root())?;\n',
        "registry loaded run-root validation",
    )
    text = replace_once(
        text,
        '                    "private Matrix path is not a physical directory: {}",\n',
        '                    "private control path is not a physical directory: {}",\n',
        "generic private path diagnostic",
    )
    text = replace_once(
        text,
        '            "private Matrix directory has unsafe permissions: {}",\n',
        '            "private control directory has unsafe permissions: {}",\n',
        "generic private permission diagnostic",
    )
    path.write_text(text)


def patch_registry_tests() -> None:
    path = ROOT / "codex-rs/hepta-fleet/src/registry_tests.rs"
    text = path.read_text()
    text = replace_once(
        text,
        'fn concurrent_open_atomically_migrates_legacy_matrix_private_roots()\n',
        'fn concurrent_open_atomically_migrates_legacy_private_roots()\n',
        "private-root migration test name",
    )
    text = replace_once(
        text,
        '    let record = fleet.registry.register(manifest)?;\n'
        '    fs::remove_dir(record.layout.matrix_secrets_root())?;\n'
        '    fs::remove_dir(record.layout.matrix_root())?;\n',
        '    let record = fleet.registry.register(manifest)?;\n'
        '    fs::set_permissions(\n'
        '        record.layout.run_root(),\n'
        '        fs::Permissions::from_mode(0o755),\n'
        '    )?;\n'
        '    fs::remove_dir(record.layout.matrix_secrets_root())?;\n'
        '    fs::remove_dir(record.layout.matrix_root())?;\n',
        "legacy run-root migration fixture",
    )
    text = replace_once(
        text,
        '    for private_root in [layout.matrix_root(), layout.matrix_secrets_root()] {\n',
        '    for private_root in [\n'
        '        layout.run_root(),\n'
        '        layout.matrix_root(),\n'
        '        layout.matrix_secrets_root(),\n'
        '    ] {\n',
        "private root mode assertions",
    )
    test = r'''
#[cfg(unix)]
#[test]
fn symlinked_run_root_is_rejected_without_mutating_target_permissions()
-> Result<(), FleetRegistryError> {
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::fs::symlink;

    let fleet = TestFleet::new()?;
    let manifest = fleet.manifest(FIRST_AGENT_ID, &fleet.first_workspace)?;
    let record = fleet.registry.register(manifest)?;
    let target = fleet._temp.path().join("external-run-target");
    fs::create_dir(&target)?;
    fs::set_permissions(&target, fs::Permissions::from_mode(0o755))?;
    fs::remove_dir_all(record.layout.run_root())?;
    symlink(&target, record.layout.run_root())?;

    let error = FleetRegistry::open_existing(fleet.root.clone())
        .err()
        .expect("symlinked run root must fail closed");
    assert!(matches!(error, FleetRegistryError::Corrupt(_)));
    assert_eq!(
        fs::symlink_metadata(&target)?.permissions().mode() & 0o777,
        0o755
    );
    Ok(())
}

'''
    text = replace_once(
        text,
        'fn create_workspace(parent: &Path, name: &str) -> Result<std::path::PathBuf, FleetRegistryError> {\n',
        test
        + 'fn create_workspace(parent: &Path, name: &str) -> Result<std::path::PathBuf, FleetRegistryError> {\n',
        "run-root symlink regression insertion",
    )
    path.write_text(text)


def patch_supervisor_tests() -> None:
    path = ROOT / "codex-rs/hepta-supervisor/src/supervisor_tests.rs"
    text = path.read_text()
    old_impl = (
        '        Ok(Self {\n'
        '            _temp: temp,\n'
        '            registry,\n'
        '            first,\n'
        '            second,\n'
        '        })\n'
        '    }\n'
        '}\n\n'
        'fn register_agent(\n'
    )
    new_impl = (
        '        Ok(Self {\n'
        '            _temp: temp,\n'
        '            registry,\n'
        '            first,\n'
        '            second,\n'
        '        })\n'
        '    }\n\n'
        '    fn test_program(&self, label: &str) -> Result<PathBuf, SupervisorError> {\n'
        '        let root = self._temp.path().join("programs");\n'
        '        std::fs::create_dir_all(&root)?;\n'
        '        let file_name: String = label\n'
        '            .as_bytes()\n'
        '            .iter()\n'
        '            .map(|byte| format!("{byte:02x}"))\n'
        '            .collect();\n'
        '        let path = root.join(file_name);\n'
        '        if !path.exists() {\n'
        '            std::fs::write(&path, b"#!/bin/sh\\nexit 0\\n")?;\n'
        '            #[cfg(unix)]\n'
        '            {\n'
        '                use std::os::unix::fs::PermissionsExt;\n'
        '                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700))?;\n'
        '            }\n'
        '        }\n'
        '        Ok(path.canonicalize()?)\n'
        '    }\n'
        '}\n\n'
        'fn register_agent(\n'
    )
    text = replace_once(text, old_impl, new_impl, "supervisor executable fixture")
    text = replace_once(
        text,
        'fn command() -> Result<AgentCommand, SupervisorError> {\n'
        '    AgentCommand::new("/fake/hepta-agentd", Vec::new())\n'
        '}\n\n'
        'fn release(identity: &str, program: &str) -> Result<AgentRelease, SupervisorError> {\n'
        '    AgentRelease::new(identity, AgentCommand::new(program, Vec::new())?)\n'
        '}\n',
        'fn command(fleet: &TestFleet) -> Result<AgentCommand, SupervisorError> {\n'
        '    AgentCommand::new(fleet.test_program("unversioned-hepta-agentd")?, Vec::new())\n'
        '}\n\n'
        'fn release(\n'
        '    fleet: &TestFleet,\n'
        '    identity: &str,\n'
        '    program: &str,\n'
        ') -> Result<AgentRelease, SupervisorError> {\n'
        '    AgentRelease::new(identity, AgentCommand::new(fleet.test_program(program)?, Vec::new())?)\n'
        '}\n',
        "supervisor command and release fixtures",
    )
    command_count = text.count("command()?")
    if command_count == 0:
        raise SystemExit("supervisor command fixtures: no call sites found")
    text = text.replace("command()?", "command(&fleet)?")
    release_count = text.count('release("')
    if release_count == 0:
        raise SystemExit("supervisor release fixtures: no call sites found")
    text = text.replace('release("', 'release(&fleet, "')
    text = replace_once(
        text,
        '    control.reject_spawn_program("/fake/release-spawn-fails/hepta-agentd");\n',
        '    control.reject_spawn_program(\n'
        '        fleet.test_program("/fake/release-spawn-fails/hepta-agentd")?,\n'
        '    );\n',
        "supervisor rejected spawn fixture",
    )
    shell_count = text.count('Path::new("/bin/sh")')
    if shell_count != 2:
        raise SystemExit(f"paired release fixture: expected two /bin/sh anchors, found {shell_count}")
    text = text.replace(
        'Path::new("/bin/sh")',
        'fleet.test_program("portable-shell")?.as_path()',
    )
    path.write_text(text)


def patch_nextest() -> None:
    path = ROOT / "codex-rs/.config/nextest.toml"
    text = path.read_text()
    text = replace_once(
        text,
        '[test-groups.windows_process_heavy]\nmax-threads = 2\n',
        '[test-groups.windows_process_heavy]\nmax-threads = 2\n\n'
        '[test-groups.hepta_supervisor_paired_process]\nmax-threads = 1\n',
        "nextest paired-process group",
    )
    text = replace_once(
        text,
        '[[profile.default.overrides]]\n# Do not add new tests here\n',
        '[[profile.default.overrides]]\n'
        '# These cases spawn real agentd+matrixd fleets. nextest launches each\n'
        '# test in a separate process, so an in-process mutex cannot serialize them.\n'
        "filter = 'package(codex-hepta-supervisor) & (test(two_real_pairs_restart_one_without_peer_pid_churn) | test(five_real_pairs_adopt_all_ten_children_and_isolate_one_matrix_crash))'\n"
        "test-group = 'hepta_supervisor_paired_process'\n\n"
        '[[profile.default.overrides]]\n'
        '# Do not add new tests here\n',
        "nextest paired-process override",
    )
    path.write_text(text)


def main() -> None:
    patch_registry()
    patch_registry_tests()
    patch_supervisor_tests()
    patch_nextest()


if __name__ == "__main__":
    main()
