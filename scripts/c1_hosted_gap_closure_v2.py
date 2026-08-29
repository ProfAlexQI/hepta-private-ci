#!/usr/bin/env python3
"""Epoch-13 wrapper for the exact-head WEB-C1 hosted gap closure.

The v1 producer owns bounded Rust 1.95 formatting, Cargo.lock regeneration,
contract validation, CAS commit, and no-force push. This wrapper makes the
repair transformations idempotent after partial predecessor commits, binds the
repository migration across runner evidence contracts, stages a private
single-link copy of the qualification executable before the artifact-to-browser
handshake, and removes ephemeral Cargo target directories before staging. It
does not weaken the artifact gate and grants no source, build, runtime,
operator, promotion, or release authority.
"""

from __future__ import annotations

import importlib.util
import pathlib
import shutil
import sys
from types import ModuleType

ROOT = pathlib.Path(__file__).resolve().parents[1]
BASE_PATH = ROOT / "scripts/c1_hosted_gap_closure_v1.py"
COMMON_PATH = "scripts/hepta_browser_runner_evidence/common.py"
CONTRACTS_PATH = "scripts/hepta_browser_runner_evidence/contracts.py"
POLICY_PATH = "docs/hepta-vnext/browser/RUNNER_QUALIFICATION_POLICY_V2.json"
STARTUP_PATH = (
    "tools/hepta-browser-c1-startup-bridge/src/bin/"
    "hepta-browser-c1-startup-bridge-trial.rs"
)
EPHEMERAL_TARGETS = (
    "tools/hepta-browser-c1-protocol/target",
    "tools/hepta-browser-c1-artifact-gate/target",
    "tools/hepta-browser-c1-startup-bridge/target",
)


def load_base() -> ModuleType:
    specification = importlib.util.spec_from_file_location(
        "hepta_c1_hosted_gap_closure_v1_core",
        BASE_PATH,
    )
    if specification is None or specification.loader is None:
        raise RuntimeError("cannot load C1 hosted gap-closure v1 core")
    module = importlib.util.module_from_spec(specification)
    sys.modules[specification.name] = module
    specification.loader.exec_module(module)
    return module


BASE = load_base()
ORIGINAL_PATCH_CONTRACTS = BASE.patch_contracts
ORIGINAL_VALIDATE = BASE.validate
ORIGINAL_PATH_ALLOWED = BASE.path_allowed
ORIGINAL_COMMIT_AND_PUSH = BASE.commit_and_push


def replace_once_idempotent(path: str, old: str, new: str) -> None:
    target = ROOT / path
    text = target.read_text(encoding="utf-8")
    old_count = text.count(old)
    new_count = text.count(new)
    if old_count == 1 and new_count == 0:
        target.write_text(text.replace(old, new, 1), encoding="utf-8")
        return
    if old_count == 0 and new_count == 1:
        return
    BASE.fail(
        f"{path}: ambiguous idempotent replacement; "
        f"old_count={old_count}, new_count={new_count}"
    )


def replace_all_idempotent(path: str, old: str, new: str) -> None:
    target = ROOT / path
    text = target.read_text(encoding="utf-8")
    if old in text:
        updated = text.replace(old, new)
        if old in updated:
            BASE.fail(f"{path}: replacement incomplete for {old!r}")
        target.write_text(updated, encoding="utf-8")
        return
    if new in text:
        return
    BASE.fail(f"{path}: neither old nor accepted replacement is present")


def patch_runner_evidence_contracts() -> None:
    BASE.replace_once(
        COMMON_PATH,
        'EXPECTED_REPOSITORY = "ProfAlexQI/hepta-private-ci"\n',
        'EXPECTED_REPOSITORY = "ProfHepta/hepta-private-ci"\n',
    )
    BASE.replace_once(
        POLICY_PATH,
        '"expected_repository_full_name":"ProfAlexQI/hepta-private-ci"',
        '"expected_repository_full_name":"ProfHepta/hepta-private-ci"',
    )
    BASE.replace_once(
        CONTRACTS_PATH,
        '''        "reuse evidence from another head SHA",
        "dispatch exact-source qualification before exact-head required graphs are executable",
''',
        '''        "reuse evidence from another head SHA",
        "cancel a run created after the queue-hygiene observation started",
        "dispatch exact-source qualification before exact-head required graphs are executable",
''',
    )
    BASE.replace_once(
        CONTRACTS_PATH,
        '''            "verification": (
                "obsolete queued runs are cancelled without cancelling the current "
                "exact-head required runs"
            ),
''',
        '''            "verification": (
                "only obsolete queued runs observed before cleanup starts are "
                "cancelled; the cleanup excludes its exact head and every run created "
                "after its observation timestamp"
            ),
''',
    )


def patch_contracts() -> None:
    # The predecessor commit may already contain any subset of the v1 repairs.
    # Rebind its helpers before calling it so partial progress is a verified
    # no-op rather than a fatal missing-preimage error.
    BASE.replace_once = replace_once_idempotent
    BASE.replace_all = replace_all_idempotent
    ORIGINAL_PATCH_CONTRACTS()
    patch_runner_evidence_contracts()
    BASE.replace_once(
        STARTUP_PATH,
        '''    let executable = std::env::current_exe()?;
    let expected_artifact_binding =
        artifact::binding_for_current_executable(BUILD_MANIFEST, SOURCE_RECEIPT)?;
''',
        '''    let staged_executable = StagedExecutable::from_current_executable()?;
    let executable = staged_executable.path();
    let expected_artifact_binding = artifact_binding_for_path(executable)?;
''',
    )
    BASE.replace_once(
        STARTUP_PATH,
        '''#[cfg(unix)]
fn fill_private_random(output: &mut [u8]) -> Result<(), Box<dyn Error>> {
    let mut random = std::fs::File::open("/dev/urandom")?;
    random.read_exact(output)?;
    Ok(())
}

struct SplitIo<R, W> {
''',
        '''#[cfg(unix)]
fn fill_private_random(output: &mut [u8]) -> Result<(), Box<dyn Error>> {
    let mut random = std::fs::File::open("/dev/urandom")?;
    random.read_exact(output)?;
    Ok(())
}

#[cfg(unix)]
fn artifact_binding_for_path(
    path: &std::path::Path,
) -> Result<artifact::ArtifactBinding, Box<dyn Error>> {
    Ok(artifact::ArtifactBinding::new(
        artifact::hash_file(path)?,
        artifact::Digest32::new(artifact::sha256(BUILD_MANIFEST))?,
        artifact::Digest32::new(artifact::sha256(SOURCE_RECEIPT))?,
    ))
}

#[cfg(unix)]
struct StagedExecutable {
    directory: std::path::PathBuf,
    path: std::path::PathBuf,
}

#[cfg(unix)]
impl StagedExecutable {
    fn from_current_executable() -> Result<Self, Box<dyn Error>> {
        use std::os::unix::fs::PermissionsExt;

        let source = std::env::current_exe()?;
        let mut nonce = [0_u8; 16];
        fill_private_random(&mut nonce)?;
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut suffix = String::with_capacity(nonce.len() * 2);
        for byte in nonce {
            suffix.push(char::from(HEX[usize::from(byte >> 4)]));
            suffix.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        let directory = std::env::temp_dir().join(format!(
            "hepta-c1-startup-bridge-{}-{suffix}",
            std::process::id()
        ));
        std::fs::create_dir(&directory)?;
        std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o700))?;
        let path = directory.join("worker");
        let staged = Self { directory, path };
        let mut input = std::fs::File::open(source)?;
        let mut output = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&staged.path)?;
        std::io::copy(&mut input, &mut output)?;
        output.set_permissions(std::fs::Permissions::from_mode(0o700))?;
        output.sync_all()?;
        std::fs::File::open(&staged.directory)?.sync_all()?;
        artifact::hash_file(&staged.path)?;
        Ok(staged)
    }

    fn path(&self) -> &std::path::Path {
        &self.path
    }
}

#[cfg(unix)]
impl Drop for StagedExecutable {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
        let _ = std::fs::remove_dir(&self.directory);
    }
}

struct SplitIo<R, W> {
''',
    )


def validate() -> None:
    ORIGINAL_VALIDATE()
    BASE.run("python3", "scripts/verify-hepta-browser-runner-evidence.py")


def path_allowed(path: str) -> bool:
    return path in {COMMON_PATH, CONTRACTS_PATH, POLICY_PATH} or ORIGINAL_PATH_ALLOWED(path)


def commit_and_push() -> None:
    for relative in EPHEMERAL_TARGETS:
        shutil.rmtree(ROOT / relative, ignore_errors=True)
    BASE.run("git", "add", "--", COMMON_PATH, CONTRACTS_PATH, POLICY_PATH)
    ORIGINAL_COMMIT_AND_PUSH()


def main() -> int:
    BASE.patch_contracts = patch_contracts
    BASE.validate = validate
    BASE.path_allowed = path_allowed
    BASE.commit_and_push = commit_and_push
    return BASE.main()


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (
        BASE.ClosureError,
        OSError,
        UnicodeError,
        RuntimeError,
        BASE.subprocess.CalledProcessError,
    ) as error:
        print(f"HEPTA_C1_GAP_CLOSURE_V2=FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
