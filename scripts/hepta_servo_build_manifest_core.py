"""Build-command, environment, and manifest assembly for Hepta Servo C1."""
from __future__ import annotations

import os
import pathlib
from typing import Any

from hepta_servo_build_manifest_io import (
    ALLOWED_ENV, BUILD_KEYS, BuildManifestError, FEATURE, SCHEMA, SBOM_FORMAT, SECRET_MARKERS,
    SOURCE_ID_DOMAIN, TARGETS, canonical, fail, license_packet, load,
    patch_inventory, require_safe, require_sha, rustc_record, sha256,
    source_receipt, spdx_sbom,
)

def build_command(path: pathlib.Path) -> bytes:
    value, raw = load(path, "build command")
    argv = value.get("argv")
    if set(value) != {"argv"} or not isinstance(argv, list) or not 2 <= len(argv) <= 256:
        fail("build command must contain only a bounded argv")
    for item in argv:
        if not isinstance(item, str) or not item or len(item.encode()) > 4096 or any(char in item for char in "\0\n\r"):
            fail("build command contains an empty, oversized, or ambiguous argv item")
    if pathlib.PurePath(argv[0]).name not in {"cargo", "cargo.exe"} or argv[1] not in {"build", "rustc"}:
        fail("build command must invoke cargo build or cargo rustc directly")
    if any(item in {"install", "publish", "login", "search", "owner", "yank"} for item in argv[1:]):
        fail("build command contains a registry-mutating Cargo operation")
    return raw


def environment(path: pathlib.Path) -> bytes:
    value, raw = load(path, "build environment allowlist")
    env = value.get("environment")
    if set(value) != {"environment"} or not isinstance(env, dict) or len(env) > 128:
        fail("build environment allowlist must be one bounded object")
    for key, item in env.items():
        if not isinstance(key, str) or key not in ALLOWED_ENV:
            fail(f"build environment key {key!r} is outside the fixed allowlist")
        if any(marker in key for marker in SECRET_MARKERS):
            fail(f"build environment key {key!r} looks secret- or identity-bearing")
        if not isinstance(item, str) or len(item.encode()) > 4096 or any(char in item for char in "\0\n\r"):
            fail(f"build environment value for {key!r} is oversized or ambiguous")
        if key == "PATH_DIGEST_SHA256":
            require_sha(item, key)
    return raw


def normalize_features(features: list[str]) -> list[str]:
    if len(features) > 128:
        fail("build features exceed the 128-item bound")
    if any(not FEATURE.fullmatch(feature) for feature in features):
        fail("build feature is noncanonical")
    if len(features) != len(set(features)):
        fail("build features contain duplicates")
    return sorted(features)


def make_manifest(*, source_path: pathlib.Path, patch_path: pathlib.Path, license_path: pathlib.Path,
                  sbom_path: pathlib.Path, rustc_path: pathlib.Path, command_path: pathlib.Path,
                  environment_path: pathlib.Path, target: str, profile: str, cargo: str,
                  linker: str, features: list[str]) -> dict[str, Any]:
    source, source_raw = source_receipt(source_path)
    patch_raw = patch_inventory(patch_path)
    license_raw = license_packet(license_path)
    sbom_raw = spdx_sbom(sbom_path)
    rustc_raw = rustc_record(rustc_path)
    command_raw = build_command(command_path)
    environment_raw = environment(environment_path)
    if target not in TARGETS:
        fail("target triple is outside the C1 allowlist")
    return {
        "schema": SCHEMA, "source_receipt_id": source["receipt_id"],
        "source_receipt_sha256": sha256(source_raw), "target_triple": target,
        "build_profile": require_safe(profile, "build profile"),
        "rustc_verbose_sha256": sha256(rustc_raw),
        "cargo_version": require_safe(cargo, "cargo version"),
        "linker_id": require_safe(linker, "linker ID"),
        "features": normalize_features(features),
        "build_command_sha256": sha256(command_raw),
        "environment_allowlist_sha256": sha256(environment_raw),
        "patch_inventory_sha256": sha256(patch_raw),
        "license_packet_sha256": sha256(license_raw), "sbom_sha256": sha256(sbom_raw),
        "sbom_format": SBOM_FORMAT, "network_access_during_build": False,
        "worker_tcp_listener": False, "worker_http_surface": False,
        "worker_external_network": False, "worker_credential_export": False,
        "worker_production_authority": False, "worker_effect_authority": False,
    }


def write_new(path: pathlib.Path, value: dict[str, Any]) -> None:
    destination = path.absolute()
    try:
        destination.parent.resolve(strict=True)
    except OSError as error:
        fail(f"output parent is unavailable: {error}")
    if destination.exists() or destination.is_symlink():
        fail("output is create-only and already exists")
    temporary = destination.parent / f".{destination.name}.tmp-{os.getpid()}"
    try:
        descriptor = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(canonical(value)); stream.flush(); os.fsync(stream.fileno())
        os.replace(temporary, destination)
    except OSError as error:
        temporary.unlink(missing_ok=True)
        fail(f"cannot write build manifest: {error}")


def verify(path: pathlib.Path, expected: dict[str, Any]) -> dict[str, Any]:
    actual, raw = load(path, "Servo worker build manifest")
    if set(actual) != BUILD_KEYS or actual != expected or raw != canonical(expected):
        fail("build manifest does not match its exact canonical inputs")
    return actual
