from __future__ import annotations

import hashlib
import json
import os
import pathlib
import shutil
import subprocess
import sys
import tarfile
from typing import Iterable

REPOSITORY = "ProfHepta/hepta-private-ci"
TARGET_BRANCH = "codex/hepta-inference-gap-closure-20260829"
EXPECTED_TARGET = "211948a45874890d69cb60b1bdafd946da2fc77f"
PAYLOAD_SHA256 = "fbc9c7cf4931f6c2a53dc448ac3089344ae0ee2ac7e6ce226d4c4950a7c3a590"
CHUNK_SIZES = [16000, 8000, 8000, 8000, 5264]
PUBLISHER_ROOT = pathlib.Path("tools/hepta-inference-v2-publisher")
WORKSPACE = pathlib.Path("codex-rs")
NEW_CRATES = (
    "hepta-infer-backend-v1",
    "hepta-infer-input-lease",
    "hepta-infer-model-registry",
    "hepta-infer-product-bridge",
    "hepta-infer-router",
    "hepta-infer-scheduler",
    "hepta-infer-semantic",
    "hepta-infer-worker-host",
)
PACKAGE_NAMES = (
    "codex-hepta-infer-core",
    "codex-hepta-infer-client",
    "codex-hepta-inferd",
    *(f"codex-{name}" for name in NEW_CRATES),
)
SINGLE_FILES = {"codex-rs/hepta-infer-core/tests/terminal_receipt_query.rs"}


class PublishError(RuntimeError):
    pass


def run(
    *argv: str,
    cwd: pathlib.Path | None = None,
    capture: bool = False,
    env: dict[str, str] | None = None,
) -> subprocess.CompletedProcess[str]:
    print("+", " ".join(argv), flush=True)
    result = subprocess.run(
        argv,
        cwd=cwd,
        check=False,
        text=True,
        capture_output=capture,
        env=env,
    )
    if result.returncode != 0:
        if capture:
            print(result.stdout, file=sys.stderr)
            print(result.stderr, file=sys.stderr)
        raise PublishError(f"command failed ({result.returncode}): {' '.join(argv)}")
    return result


def git(*argv: str, capture: bool = True) -> str:
    result = run("git", *argv, capture=capture)
    return result.stdout.strip() if capture else ""


def sha256(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        while block := handle.read(1024 * 1024):
            digest.update(block)
    return digest.hexdigest()


def reconstruct_payload(output: pathlib.Path) -> None:
    import base64

    encoded = bytearray()
    for index, size in enumerate(CHUNK_SIZES):
        path = PUBLISHER_ROOT / f"payload.part{index:02d}"
        if not path.is_file() or path.stat().st_size != size:
            raise PublishError(
                f"payload chunk boundary mismatch: {path} size={path.stat().st_size if path.exists() else None} expected={size}"
            )
        encoded.extend(path.read_bytes())
    try:
        decoded = base64.b64decode(encoded, validate=True)
    except Exception as error:
        raise PublishError(f"payload base64 invalid: {error}") from error
    output.write_bytes(decoded)
    actual = sha256(output)
    if actual != PAYLOAD_SHA256:
        raise PublishError(f"payload digest mismatch: {actual} != {PAYLOAD_SHA256}")


def permitted_file(path: str) -> bool:
    if path in SINGLE_FILES:
        return True
    return any(path.startswith(f"codex-rs/{name}/") for name in NEW_CRATES)


def validate_archive(archive: pathlib.Path) -> list[tarfile.TarInfo]:
    with tarfile.open(archive, "r:gz") as handle:
        members = handle.getmembers()
    if not members:
        raise PublishError("empty source payload")
    permitted_directories = {
        ".",
        "codex-rs",
        "codex-rs/hepta-infer-core",
        "codex-rs/hepta-infer-core/tests",
    }
    for name in NEW_CRATES:
        root = f"codex-rs/{name}"
        permitted_directories.update({root, f"{root}/src", f"{root}/src/bin", f"{root}/tests"})
    files: set[str] = set()
    for member in members:
        raw = pathlib.PurePosixPath(member.name)
        if raw.is_absolute() or ".." in raw.parts:
            raise PublishError(f"unsafe archive path: {member.name}")
        rendered = raw.as_posix().removeprefix("./").rstrip("/") or "."
        if member.isdir():
            if rendered not in permitted_directories:
                raise PublishError(f"out-of-scope archive directory: {member.name}")
            continue
        if not member.isfile():
            raise PublishError(f"non-regular archive member: {member.name}")
        if not permitted_file(rendered):
            raise PublishError(f"out-of-scope archive file: {member.name}")
        if rendered in files:
            raise PublishError(f"duplicate archive file: {rendered}")
        files.add(rendered)
    for crate in NEW_CRATES:
        required = {f"codex-rs/{crate}/Cargo.toml", f"codex-rs/{crate}/src/lib.rs"}
        missing = required - files
        if missing:
            raise PublishError(f"missing required crate files for {crate}: {sorted(missing)}")
    if not SINGLE_FILES.issubset(files):
        raise PublishError(f"missing receipt query test: {sorted(SINGLE_FILES - files)}")
    return members


def extract_archive(archive: pathlib.Path, members: list[tarfile.TarInfo]) -> None:
    with tarfile.open(archive, "r:gz") as handle:
        for member in members:
            raw = pathlib.PurePosixPath(member.name)
            rendered = raw.as_posix().removeprefix("./").rstrip("/") or "."
            if member.isdir():
                if rendered != ".":
                    pathlib.Path(rendered).mkdir(parents=True, exist_ok=True)
                continue
            target = pathlib.Path(rendered)
            if target.exists():
                raise PublishError(f"payload refuses to overwrite existing file: {rendered}")
            source = handle.extractfile(member)
            if source is None:
                raise PublishError(f"archive member unreadable: {rendered}")
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(source.read())
            target.chmod(member.mode & 0o777)


def patch_workspace_manifest() -> None:
    path = WORKSPACE / "Cargo.toml"
    text = path.read_text(encoding="utf-8")
    old_members = '''    "hepta-infer-core",
    "hepta-infer-client",
    "hepta-inferd",'''
    new_members = '''    "hepta-infer-backend-v1",
    "hepta-infer-core",
    "hepta-infer-client",
    "hepta-infer-input-lease",
    "hepta-infer-model-registry",
    "hepta-infer-product-bridge",
    "hepta-infer-router",
    "hepta-infer-scheduler",
    "hepta-infer-semantic",
    "hepta-infer-worker-host",
    "hepta-inferd",'''
    old_dependencies = '''codex-hepta-infer-core = { path = "hepta-infer-core" }
codex-hepta-infer-client = { path = "hepta-infer-client" }
codex-hepta-inferd = { path = "hepta-inferd" }'''
    new_dependencies = '''codex-hepta-infer-backend-v1 = { path = "hepta-infer-backend-v1" }
codex-hepta-infer-core = { path = "hepta-infer-core" }
codex-hepta-infer-client = { path = "hepta-infer-client" }
codex-hepta-infer-input-lease = { path = "hepta-infer-input-lease" }
codex-hepta-infer-model-registry = { path = "hepta-infer-model-registry" }
codex-hepta-infer-product-bridge = { path = "hepta-infer-product-bridge" }
codex-hepta-infer-router = { path = "hepta-infer-router" }
codex-hepta-infer-scheduler = { path = "hepta-infer-scheduler" }
codex-hepta-infer-semantic = { path = "hepta-infer-semantic" }
codex-hepta-infer-worker-host = { path = "hepta-infer-worker-host" }
codex-hepta-inferd = { path = "hepta-inferd" }'''
    if text.count(old_members) != 1 or text.count(old_dependencies) != 1:
        raise PublishError("workspace insertion anchors drifted")
    path.write_text(
        text.replace(old_members, new_members, 1).replace(
            old_dependencies, new_dependencies, 1
        ),
        encoding="utf-8",
    )


def package_records(path: pathlib.Path) -> list[dict[str, object]]:
    import tomllib

    return tomllib.loads(path.read_text(encoding="utf-8")).get("package", [])


def validate_lock(before: pathlib.Path, after: pathlib.Path) -> None:
    before_records = package_records(before)
    after_records = package_records(after)
    before_external = {
        (item["name"], item["version"], item.get("source"), item.get("checksum"))
        for item in before_records
        if item.get("source")
    }
    after_external = {
        (item["name"], item["version"], item.get("source"), item.get("checksum"))
        for item in after_records
        if item.get("source")
    }
    if before_external != after_external:
        raise PublishError("external Cargo.lock graph drifted")
    before_names = {item["name"] for item in before_records}
    after_names = {item["name"] for item in after_records}
    expected_new = {f"codex-{name}" for name in NEW_CRATES}
    actual_new = after_names - before_names
    if actual_new != expected_new:
        raise PublishError(
            f"unexpected local Cargo.lock packages: {sorted(actual_new)} expected={sorted(expected_new)}"
        )


def package_args() -> list[str]:
    result: list[str] = []
    for package in PACKAGE_NAMES:
        result.extend(("-p", package))
    return result


def run_rust_qualification(temp: pathlib.Path) -> None:
    before_lock = temp / "Cargo.lock.before"
    shutil.copy2(WORKSPACE / "Cargo.lock", before_lock)
    metadata = run(
        "cargo",
        "metadata",
        "--no-deps",
        "--format-version",
        "1",
        cwd=WORKSPACE,
        capture=True,
    ).stdout
    (temp / "runtime-metadata.json").write_text(metadata, encoding="utf-8")
    validate_lock(before_lock, WORKSPACE / "Cargo.lock")
    args = package_args()
    run("cargo", "fmt", *args, cwd=WORKSPACE, capture=False)
    run("cargo", "fmt", *args, "--", "--check", cwd=WORKSPACE, capture=False)
    commands: tuple[tuple[str, ...], ...] = (
        ("cargo", "check", "--locked", "--all-targets", *args),
        ("cargo", "test", "--locked", "--all-targets", *args, "--", "--test-threads=1"),
        ("cargo", "clippy", "--locked", "--all-targets", "--no-deps", *args, "--", "-D", "warnings"),
    )
    names = ("runtime-cargo-check.log", "runtime-cargo-test.log", "runtime-cargo-clippy.log")
    for command, name in zip(commands, names, strict=True):
        print("+", " ".join(command), flush=True)
        with (temp / name).open("w", encoding="utf-8") as output:
            process = subprocess.Popen(
                command,
                cwd=WORKSPACE,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
            )
            assert process.stdout is not None
            for line in process.stdout:
                sys.stdout.write(line)
                output.write(line)
            result = process.wait()
        if result != 0:
            raise PublishError(f"Rust qualification failed ({result}): {' '.join(command)}")


def changed_paths() -> list[str]:
    output = git("diff", "--cached", "--name-only")
    return [line for line in output.splitlines() if line]


def validate_changed_paths(paths: Iterable[str]) -> None:
    values = list(paths)
    exact = {
        "codex-rs/Cargo.toml",
        "codex-rs/Cargo.lock",
        *SINGLE_FILES,
    }
    prefixes = tuple(f"codex-rs/{name}/" for name in NEW_CRATES)
    unexpected = [path for path in values if path not in exact and not path.startswith(prefixes)]
    if unexpected:
        raise PublishError(f"changed-path escape: {unexpected}")
    missing = [prefix for prefix in prefixes if not any(path.startswith(prefix) for path in values)]
    if missing:
        raise PublishError(f"missing crate payload: {missing}")
    for required in ("codex-rs/Cargo.toml", "codex-rs/Cargo.lock"):
        if required not in values:
            raise PublishError(f"required workspace path unchanged: {required}")


def main() -> int:
    if os.environ.get("GITHUB_REPOSITORY") != REPOSITORY:
        raise PublishError("repository identity mismatch")
    temp = pathlib.Path(os.environ["RUNNER_TEMP"])
    archive = temp / "runtime-source.tar.gz"
    reconstruct_payload(archive)
    members = validate_archive(archive)

    run("git", "fetch", "--no-tags", "origin", TARGET_BRANCH, capture=False)
    if git("rev-parse", "FETCH_HEAD") != EXPECTED_TARGET:
        raise PublishError("target branch head drifted")
    run("git", "checkout", "--detach", EXPECTED_TARGET, capture=False)
    if git("status", "--porcelain"):
        raise PublishError("target checkout is dirty")

    extract_archive(archive, members)
    patch_workspace_manifest()
    run_rust_qualification(temp)

    run("git", "add", "-A", capture=False)
    validate_changed_paths(changed_paths())
    run("git", "diff", "--cached", "--check", capture=False)
    run("git", "config", "user.name", "github-actions[bot]", capture=False)
    run(
        "git",
        "config",
        "user.email",
        "41898282+github-actions[bot]@users.noreply.github.com",
        capture=False,
    )
    run(
        "git",
        "commit",
        "-m",
        "feat(inference): add bounded runtime closure foundations INF-2B through INF-5",
        capture=False,
    )
    source_sha = git("rev-parse", "HEAD")
    source_tree = git("rev-parse", "HEAD^{tree}")
    run("git", "fetch", "--no-tags", "origin", TARGET_BRANCH, capture=False)
    if git("rev-parse", "FETCH_HEAD") != EXPECTED_TARGET:
        raise PublishError("target branch drifted before push")
    run("git", "push", "origin", f"HEAD:refs/heads/{TARGET_BRANCH}", capture=False)

    receipt = {
        "schema": "hepta.inference.v2.runtime_source_publisher_receipt.v3",
        "source_sha": source_sha,
        "source_tree": source_tree,
        "source_parent": EXPECTED_TARGET,
        "payload_sha256": "sha256:" + PAYLOAD_SHA256,
        "rust_1_95_fmt_check_test_clippy": "EXECUTED_PASSED_ON_LINUX_ARM64",
        "qualification_only": True,
        "native_llamacpp_inference": False,
        "hardware_performance": False,
        "product_default_route_changed": False,
        "operator_accepted": False,
        "promoted": False,
        "released": False,
    }
    (temp / "runtime-publisher-receipt.json").write_text(
        json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(json.dumps(receipt, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except PublishError as error:
        print(f"FAIL_HEPTA_INFERENCE_V2_RUNTIME_PUBLISHER: {error}", file=sys.stderr)
        raise SystemExit(1)
