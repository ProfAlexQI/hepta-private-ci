#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import importlib.util
import json
import sys
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from types import ModuleType
from typing import Any

V1_FILENAME = "verify-hepta-intelligence-integration-admission.py"
V1_SHA256 = "86ad5a7cf238711f7c944be85bfc1369a8a6dcb7527f325b61ec91bafe6be413"
MAX_GIT_COMMIT_RESPONSE_BYTES = 512 * 1024
MAX_SIGNATURE_BYTES = 128 * 1024
MAX_SIGNED_PAYLOAD_BYTES = 256 * 1024


def load_v1_verifier() -> ModuleType:
    path = Path(__file__).with_name(V1_FILENAME)
    if not path.is_file():
        raise SystemExit(f"missing delegated admission verifier: {path}")
    actual_sha256 = hashlib.sha256(path.read_bytes()).hexdigest()
    if actual_sha256 != V1_SHA256:
        raise SystemExit(
            "delegated admission verifier digest drifted: "
            f"expected={V1_SHA256} actual={actual_sha256}"
        )
    spec = importlib.util.spec_from_file_location(
        "hepta_intelligence_integration_admission_v1",
        path,
    )
    if spec is None or spec.loader is None:
        raise SystemExit("unable to load delegated admission verifier")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


V1 = load_v1_verifier()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise V1.AdmissionError(message)


def read_bounded(response: Any, limit: int) -> bytes:
    payload = response.read(limit + 1)
    require(len(payload) <= limit, f"GitHub Git commit API payload exceeds {limit} bytes")
    return payload


def fetch_commit_metadata(repository: str, head: str, token: str) -> dict[str, Any]:
    owner, separator, name = repository.partition("/")
    require(separator == "/" and owner and name and "/" not in name, "invalid repository")
    url = (
        "https://api.github.com/repos/"
        f"{urllib.parse.quote(owner, safe='')}/"
        f"{urllib.parse.quote(name, safe='')}/git/commits/{head}"
    )
    request = urllib.request.Request(
        url,
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token}",
            "User-Agent": "hepta-intelligence-integration-admission-v2",
            "X-GitHub-Api-Version": "2022-11-28",
        },
        method="GET",
    )
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            require(response.status == 200, f"GitHub Git commit API returned {response.status}")
            encoded = read_bounded(response, MAX_GIT_COMMIT_RESPONSE_BYTES)
    except (urllib.error.HTTPError, urllib.error.URLError, TimeoutError) as error:
        raise V1.AdmissionError(f"GitHub Git commit API request failed: {error}") from error

    try:
        value = json.loads(encoded)
    except json.JSONDecodeError as error:
        raise V1.AdmissionError(
            f"GitHub Git commit API returned invalid JSON: {error}"
        ) from error
    require(isinstance(value, dict), "GitHub Git commit API payload must be an object")
    require(value.get("sha") == head, "GitHub Git commit API head identity drifted")

    tree = value.get("tree")
    parents = value.get("parents")
    verification = value.get("verification")
    require(isinstance(tree, dict), "GitHub Git commit API tree is missing")
    require(isinstance(parents, list), "GitHub Git commit API parents are missing")
    require(isinstance(verification, dict), "GitHub signature verification is missing")

    tree_sha = V1.require_sha(str(tree.get("sha", "")), "GitHub API tree")
    parent_shas: list[str] = []
    for index, parent in enumerate(parents):
        require(isinstance(parent, dict), f"GitHub API parent[{index}] is not an object")
        parent_shas.append(
            V1.require_sha(str(parent.get("sha", "")), f"GitHub API parent[{index}]")
        )

    signature = verification.get("signature")
    signed_payload = verification.get("payload")
    require(isinstance(signature, str) and signature, "verified signature bytes are missing")
    require(isinstance(signed_payload, str) and signed_payload, "verified payload is missing")
    require(
        len(signature.encode("utf-8")) <= MAX_SIGNATURE_BYTES,
        "verified signature exceeds the bounded size",
    )
    require(
        len(signed_payload.encode("utf-8")) <= MAX_SIGNED_PAYLOAD_BYTES,
        "verified payload exceeds the bounded size",
    )

    header_block, separator, _message = signed_payload.partition("\n\n")
    require(separator == "\n\n", "verified payload lacks the Git commit header boundary")
    header_lines = header_block.splitlines()
    signed_trees = [line.removeprefix("tree ") for line in header_lines if line.startswith("tree ")]
    signed_parents = [
        line.removeprefix("parent ")
        for line in header_lines
        if line.startswith("parent ")
    ]
    require(signed_trees == [tree_sha], "verified payload tree differs from API metadata")
    require(signed_parents == parent_shas, "verified payload parent order differs from API metadata")

    # Adapt the bounded Git-database response to the v1 verifier's internal
    # shape. The v1 verifier continues to own all local Git, A0 overlay,
    # authority-negative and deterministic-receipt checks.
    return {
        "sha": value["sha"],
        "commit": {
            "tree": tree,
            "verification": verification,
        },
        "parents": parents,
    }


V1.fetch_commit_metadata = fetch_commit_metadata


if __name__ == "__main__":
    raise SystemExit(V1.main())
