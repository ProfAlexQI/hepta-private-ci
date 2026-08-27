#!/usr/bin/env python3
"""Capture an exact, path-free toolchain receipt for a future Servo build.

The tool invokes only explicitly supplied absolute rustc, cargo, and linker
binaries with version-only arguments. It does not build or execute Servo.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import stat
import subprocess
import sys
from pathlib import Path
from typing import Any

SCHEMA = "hepta.browser.servo_toolchain_receipt.v1"
ALLOWED_TARGETS = {
    "x86_64-unknown-linux-gnu",
    "aarch64-apple-darwin",
    "x86_64-pc-windows-msvc",
}
ALLOWED_LINKER_KINDS = {"clang", "cc", "lld", "mold", "msvc-link", "zig"}
SHA1 = re.compile(r"^[0-9a-f]{40}$")
SAFE_TEXT = re.compile(r"^[^\x00-\x1f\x7f]{1,256}$")
MAX_BINARY_BYTES = 1024 * 1024 * 1024
MAX_OUTPUT_BYTES = 64 * 1024
DEFAULT_TIMEOUT_SECONDS = 10.0
AUTHORITY = {
    "runtime_authority": False,
    "effect_authority": False,
    "production_caller": False,
    "production_writer": False,
    "runtime_external_network": False,
    "raw_cookie_export": False,
    "credential_export": False,
    "operator_acceptance": False,
    "promotion": False,
    "release_qualified": False,
}


class ToolchainReceiptError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise ToolchainReceiptError(message)


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(
        value,
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def hash_binary(path: Path) -> tuple[str, int]:
    digest = hashlib.sha256()
    length = 0
    with path.open("rb") as handle:
        while True:
            block = handle.read(1024 * 1024)
            if not block:
                break
            length += len(block)
            if length > MAX_BINARY_BYTES:
                fail(f"toolchain binary exceeds the size bound: {path.name}")
            digest.update(block)
    if length == 0:
        fail(f"toolchain binary is empty: {path.name}")
    return digest.hexdigest(), length


def canonical_binary(value: str, label: str) -> Path:
    path = Path(value)
    if not path.is_absolute() or ".." in path.parts:
        fail(f"{label} must be a canonical absolute path")
    try:
        resolved = path.resolve(strict=True)
        metadata = path.lstat()
    except OSError as error:
        fail(f"{label} is unavailable: {error}")
    if resolved != path:
        fail(f"{label} must not contain a symlink component")
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(f"{label} must be a non-symlink regular file")
    if metadata.st_nlink != 1:
        fail(f"{label} must have exactly one hard link")
    if metadata.st_mode & 0o022:
        fail(f"{label} must not be group/world writable")
    if not os.access(path, os.X_OK):
        fail(f"{label} is not executable")
    hash_binary(path)
    return path


def minimal_environment() -> dict[str, str]:
    environment = {
        "LC_ALL": "C",
        "LANG": "C",
        "TZ": "UTC",
        "GIT_CONFIG_NOSYSTEM": "1",
        "GIT_TERMINAL_PROMPT": "0",
    }
    for name in ("SYSTEMROOT", "WINDIR"):
        value = os.environ.get(name)
        if value:
            environment[name] = value
    return environment


def run_bounded(
    binary: Path,
    arguments: list[str],
    *,
    timeout_seconds: float = DEFAULT_TIMEOUT_SECONDS,
) -> bytes:
    if timeout_seconds <= 0 or timeout_seconds > 60:
        fail("toolchain command timeout is outside the allowed range")
    try:
        result = subprocess.run(
            [os.fspath(binary), *arguments],
            check=False,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            env=minimal_environment(),
            timeout=timeout_seconds,
        )
    except subprocess.TimeoutExpired:
        fail(f"toolchain version command timed out: {binary.name}")
    except OSError as error:
        fail(f"cannot execute toolchain binary {binary.name}: {error}")
    output = result.stdout or b""
    if result.returncode != 0:
        detail = output[:4096].decode("utf-8", "replace").strip()
        fail(f"toolchain version command failed for {binary.name}: {detail}")
    if not output or len(output) > MAX_OUTPUT_BYTES:
        fail(f"toolchain version output is empty or too large: {binary.name}")
    try:
        output.decode("utf-8", "strict")
    except UnicodeError as error:
        fail(f"toolchain version output is not UTF-8: {error}")
    return output


def parse_key_value_output(output: bytes, label: str) -> tuple[str, dict[str, str]]:
    text = output.decode("utf-8", "strict")
    lines = [line.rstrip() for line in text.splitlines() if line.strip()]
    if not lines or not SAFE_TEXT.fullmatch(lines[0]):
        fail(f"{label} first version line is invalid")
    fields: dict[str, str] = {}
    for line in lines[1:]:
        key, separator, value = line.partition(":")
        if not separator:
            continue
        key = key.strip().lower().replace("-", "_").replace(" ", "_")
        value = value.strip()
        if not key or not value or not SAFE_TEXT.fullmatch(value):
            fail(f"{label} version field is invalid")
        if key in fields:
            fail(f"{label} version output contains duplicate field {key}")
        fields[key] = value
    return lines[0], fields


def parse_rustc(output: bytes) -> dict[str, str]:
    first_line, fields = parse_key_value_output(output, "rustc")
    commit_hash = fields.get("commit_hash")
    host = fields.get("host")
    release = fields.get("release")
    if not commit_hash or not SHA1.fullmatch(commit_hash):
        fail("rustc version output has no exact commit hash")
    if not host or not SAFE_TEXT.fullmatch(host):
        fail("rustc version output has no valid host")
    if not release or not SAFE_TEXT.fullmatch(release):
        fail("rustc version output has no valid release")
    return {
        "version": first_line,
        "release": release,
        "commit_hash": commit_hash,
        "host": host,
        "output_sha256": sha256_bytes(output),
    }


def parse_cargo(output: bytes) -> dict[str, str]:
    first_line, fields = parse_key_value_output(output, "cargo")
    host = fields.get("host")
    release = fields.get("release")
    commit_hash = fields.get("commit_hash")
    if not host or not SAFE_TEXT.fullmatch(host):
        fail("cargo version output has no valid host")
    if not release or not SAFE_TEXT.fullmatch(release):
        fail("cargo version output has no valid release")
    if commit_hash is not None and not SHA1.fullmatch(commit_hash):
        fail("cargo commit hash is invalid")
    result = {
        "version": first_line,
        "release": release,
        "host": host,
        "output_sha256": sha256_bytes(output),
    }
    if commit_hash is not None:
        result["commit_hash"] = commit_hash
    return result


def parse_linker(output: bytes, kind: str) -> dict[str, str]:
    text = output.decode("utf-8", "strict")
    first_line = next((line.strip() for line in text.splitlines() if line.strip()), "")
    if not first_line or not SAFE_TEXT.fullmatch(first_line):
        fail("linker version line is invalid")
    return {
        "kind": kind,
        "version": first_line,
        "output_sha256": sha256_bytes(output),
    }


def capture(
    *,
    target: str,
    linker_kind: str,
    rustc_path: Path,
    cargo_path: Path,
    linker_path: Path,
) -> dict[str, Any]:
    if target not in ALLOWED_TARGETS:
        fail("toolchain target is outside the initial platform allowlist")
    if linker_kind not in ALLOWED_LINKER_KINDS:
        fail("linker kind is outside the allowlist")
    rustc_sha256, rustc_bytes = hash_binary(rustc_path)
    cargo_sha256, cargo_bytes = hash_binary(cargo_path)
    linker_sha256, linker_bytes = hash_binary(linker_path)
    rustc = parse_rustc(run_bounded(rustc_path, ["-vV"]))
    cargo = parse_cargo(run_bounded(cargo_path, ["-Vv"]))
    linker = parse_linker(run_bounded(linker_path, ["--version"]), linker_kind)
    if cargo["host"] != rustc["host"]:
        fail("cargo and rustc hosts differ")
    rustc.update({"binary_sha256": rustc_sha256, "binary_bytes": rustc_bytes})
    cargo.update({"binary_sha256": cargo_sha256, "binary_bytes": cargo_bytes})
    linker.update({"binary_sha256": linker_sha256, "binary_bytes": linker_bytes})
    return {
        "schema": SCHEMA,
        "schema_version": 1,
        "target": target,
        "host": rustc["host"],
        "rustc": rustc,
        "cargo": cargo,
        "linker": linker,
        "capture": {
            "commands": ["rustc -vV", "cargo -Vv", f"{linker_kind} --version"],
            "minimal_environment": {
                "GIT_CONFIG_NOSYSTEM": "1",
                "GIT_TERMINAL_PROMPT": "0",
                "LANG": "C",
                "LC_ALL": "C",
                "TZ": "UTC",
            },
            "network_access_used": False,
            "build_run": False,
            "artifact_created": False,
        },
        "machine_local_paths_included": False,
        "authority": AUTHORITY,
    }


def write_atomic(path: Path, value: dict[str, Any]) -> None:
    if path.exists():
        fail("toolchain receipt output already exists")
    encoded = canonical_bytes(value)
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(descriptor, "wb") as handle:
        handle.write(encoded)
        handle.flush()
        os.fsync(handle.fileno())


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", required=True, choices=sorted(ALLOWED_TARGETS))
    parser.add_argument("--rustc", required=True)
    parser.add_argument("--cargo", required=True)
    parser.add_argument("--linker", required=True)
    parser.add_argument("--linker-kind", required=True, choices=sorted(ALLOWED_LINKER_KINDS))
    parser.add_argument("--output", required=True)
    return parser.parse_args()


def main() -> int:
    try:
        arguments = parse_arguments()
        rustc_path = canonical_binary(arguments.rustc, "--rustc")
        cargo_path = canonical_binary(arguments.cargo, "--cargo")
        linker_path = canonical_binary(arguments.linker, "--linker")
        output = Path(arguments.output)
        if not output.is_absolute() or output != output.parent.resolve(strict=True) / output.name:
            fail("--output must be a canonical absolute path")
        receipt = capture(
            target=arguments.target,
            linker_kind=arguments.linker_kind,
            rustc_path=rustc_path,
            cargo_path=cargo_path,
            linker_path=linker_path,
        )
        write_atomic(output, receipt)
    except (ToolchainReceiptError, OSError, UnicodeError) as error:
        print(f"HEPTA_SERVO_TOOLCHAIN_RECEIPT=FAIL: {error}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "schema": SCHEMA,
                "status": "PASS_TOOLCHAIN_FACTS_ONLY",
                "target": receipt["target"],
                "host": receipt["host"],
                "build_run": False,
                "artifact_created": False,
                "servo_runtime_qualified": False,
                "authority": "all_false",
            },
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
