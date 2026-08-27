#!/usr/bin/env python3
"""Create or verify an exact, negative-authority Servo worker build manifest."""
from __future__ import annotations

import argparse
import json
import pathlib
import sys

from hepta_servo_build_manifest_core import BuildManifestError, canonical, make_manifest, sha256, verify, write_new


def common(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--source-receipt", required=True)
    parser.add_argument("--patch-inventory", required=True)
    parser.add_argument("--license-packet", required=True)
    parser.add_argument("--sbom", required=True)
    parser.add_argument("--rustc-verbose", required=True)
    parser.add_argument("--build-command", required=True)
    parser.add_argument("--environment-allowlist", required=True)
    parser.add_argument("--target-triple", required=True)
    parser.add_argument("--build-profile", required=True)
    parser.add_argument("--cargo-version", required=True)
    parser.add_argument("--linker-id", required=True)
    parser.add_argument("--feature", action="append", default=[])


def arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    snapshot = commands.add_parser("snapshot"); common(snapshot); snapshot.add_argument("--output", required=True)
    check = commands.add_parser("verify"); common(check); check.add_argument("--manifest", required=True)
    return parser.parse_args()


def build(args: argparse.Namespace) -> dict[str, object]:
    return make_manifest(
        source_path=pathlib.Path(args.source_receipt), patch_path=pathlib.Path(args.patch_inventory),
        license_path=pathlib.Path(args.license_packet), sbom_path=pathlib.Path(args.sbom),
        rustc_path=pathlib.Path(args.rustc_verbose), command_path=pathlib.Path(args.build_command),
        environment_path=pathlib.Path(args.environment_allowlist), target=args.target_triple,
        profile=args.build_profile, cargo=args.cargo_version, linker=args.linker_id,
        features=args.feature,
    )


def main() -> int:
    try:
        args = arguments(); manifest = build(args)
        if args.command == "snapshot":
            write_new(pathlib.Path(args.output), manifest); status = "HEPTA_SERVO_BUILD_MANIFEST_CREATED"
        else:
            verify(pathlib.Path(args.manifest), manifest); status = "HEPTA_SERVO_BUILD_MANIFEST_VERIFIED"
        print(json.dumps({
            "claim": "BUILD_INPUTS_ONLY", "manifest_sha256": sha256(canonical(manifest)),
            "runtime_qualified": False, "status": status,
        }, sort_keys=True, separators=(",", ":")))
    except BuildManifestError as error:
        print(f"HEPTA_SERVO_BUILD_MANIFEST=FAIL: {error}", file=sys.stderr); return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
