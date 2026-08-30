#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import importlib.util
import json
import re
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
ALLOWED_MEDIA_TYPES = frozenset(
    {
        "application/json",
        "application/vnd.github+json",
    }
)


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


class RejectRedirectHandler(urllib.request.HTTPRedirectHandler):
    def redirect_request(
        self,
        request: urllib.request.Request,
        file_pointer: Any,
        code: int,
        message: str,
        headers: Any,
        new_url: str,
    ) -> urllib.request.Request | None:
        del request, file_pointer, message, headers, new_url
        raise V1.AdmissionError(
            f"GitHub Git commit API redirect rejected before follow: {code}"
        )


URL_OPENER = urllib.request.build_opener(
    urllib.request.ProxyHandler({}),
    RejectRedirectHandler(),
)


def reject_duplicate_json_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    value: dict[str, Any] = {}
    for key, item in pairs:
        require(key not in value, f"duplicate JSON key in GitHub response: {key}")
        value[key] = item
    return value


def response_header_values(response: Any, name: str) -> list[str]:
    headers = getattr(response, "headers", None)
    require(headers is not None, "GitHub Git commit API response headers are missing")
    if hasattr(headers, "get_all"):
        values = headers.get_all(name, [])
    else:
        value = headers.get(name)
        values = [] if value is None else [value]
    require(isinstance(values, list), f"GitHub response header set is invalid: {name}")
    result: list[str] = []
    for value in values:
        require(isinstance(value, str), f"GitHub response header is not text: {name}")
        require(
            "\r" not in value and "\n" not in value,
            f"GitHub response header folded: {name}",
        )
        result.append(value)
    return result


def one_response_header(
    response: Any,
    name: str,
    *,
    required: bool,
) -> str | None:
    values = response_header_values(response, name)
    if required:
        require(len(values) == 1, f"GitHub response header cardinality drifted: {name}")
    else:
        require(len(values) <= 1, f"GitHub response header duplicated: {name}")
    return values[0] if values else None


def validate_content_type(raw_content_type: str) -> None:
    parts = [part.strip() for part in raw_content_type.split(";")]
    media_type = parts[0].casefold()
    require(
        media_type in ALLOWED_MEDIA_TYPES,
        f"GitHub response media type is not JSON: {media_type!r}",
    )
    parameters: dict[str, str] = {}
    for parameter in parts[1:]:
        require(
            parameter and "=" in parameter,
            "GitHub Content-Type parameter is invalid",
        )
        name, value = parameter.split("=", 1)
        key = name.strip().casefold()
        normalized = value.strip().strip('"').casefold()
        require(
            key and key not in parameters,
            "GitHub Content-Type parameter duplicated",
        )
        require(
            key == "charset",
            f"GitHub Content-Type parameter is not allowed: {key!r}",
        )
        parameters[key] = normalized
    charset = parameters.get("charset")
    require(
        charset is None or charset == "utf-8",
        f"GitHub response charset is not UTF-8: {charset!r}",
    )


def validate_response_envelope(response: Any, expected_url: str) -> int | None:
    require(response.status == 200, f"GitHub Git commit API returned {response.status}")
    observed_url = response.geturl()
    require(
        isinstance(observed_url, str) and observed_url == expected_url,
        "GitHub Git commit API redirected or changed the response URL",
    )

    raw_content_type = one_response_header(
        response,
        "Content-Type",
        required=True,
    )
    assert raw_content_type is not None
    validate_content_type(raw_content_type)

    content_encoding = one_response_header(
        response,
        "Content-Encoding",
        required=False,
    )
    require(
        content_encoding is None
        or content_encoding.strip().casefold() in {"", "identity"},
        "GitHub response content encoding is not identity",
    )

    content_length = one_response_header(
        response,
        "Content-Length",
        required=False,
    )
    if content_length is None:
        return None
    require(
        re.fullmatch(r"0|[1-9][0-9]{0,9}", content_length) is not None,
        "GitHub response Content-Length is not canonical",
    )
    declared = int(content_length)
    require(
        declared <= MAX_GIT_COMMIT_RESPONSE_BYTES,
        "GitHub response Content-Length exceeds the bounded size",
    )
    return declared


def read_bounded(response: Any, limit: int) -> bytes:
    chunks: list[bytes] = []
    total = 0
    while True:
        remaining = limit + 1 - total
        require(remaining > 0, f"GitHub Git commit API payload exceeds {limit} bytes")
        chunk = response.read(min(64 * 1024, remaining))
        require(isinstance(chunk, bytes), "GitHub Git commit API payload is not bytes")
        if not chunk:
            break
        chunks.append(chunk)
        total += len(chunk)
        require(total <= limit, f"GitHub Git commit API payload exceeds {limit} bytes")
    return b"".join(chunks)


def decode_strict_json(encoded: bytes) -> dict[str, Any]:
    require(
        not encoded.startswith(b"\xef\xbb\xbf"),
        "GitHub Git commit API JSON must not contain a UTF-8 BOM",
    )
    try:
        text = encoded.decode("utf-8")
    except UnicodeDecodeError as error:
        raise V1.AdmissionError(
            f"GitHub Git commit API JSON is not strict UTF-8: {error}"
        ) from error
    try:
        value = json.loads(text, object_pairs_hook=reject_duplicate_json_keys)
    except json.JSONDecodeError as error:
        raise V1.AdmissionError(
            f"GitHub Git commit API returned invalid JSON: {error}"
        ) from error
    require(isinstance(value, dict), "GitHub Git commit API payload must be an object")
    return value


def fetch_commit_metadata(repository: str, head: str, token: str) -> dict[str, Any]:
    owner, separator, name = repository.partition("/")
    require(
        separator == "/" and owner and name and "/" not in name,
        "invalid repository",
    )
    url = (
        "https://api.github.com/repos/"
        f"{urllib.parse.quote(owner, safe='')}/"
        f"{urllib.parse.quote(name, safe='')}/git/commits/{head}"
    )
    request = urllib.request.Request(
        url,
        headers={
            "Accept": "application/vnd.github+json",
            "Accept-Encoding": "identity",
            "Authorization": f"Bearer {token}",
            "User-Agent": "hepta-intelligence-integration-admission-v2",
            "X-GitHub-Api-Version": "2022-11-28",
        },
        method="GET",
    )
    try:
        with URL_OPENER.open(request, timeout=30) as response:
            declared_length = validate_response_envelope(response, url)
            encoded = read_bounded(response, MAX_GIT_COMMIT_RESPONSE_BYTES)
    except V1.AdmissionError:
        raise
    except (
        urllib.error.HTTPError,
        urllib.error.URLError,
        TimeoutError,
        OSError,
    ) as error:
        raise V1.AdmissionError(
            f"GitHub Git commit API request failed: {error}"
        ) from error

    require(
        declared_length is None or declared_length == len(encoded),
        "GitHub response Content-Length differs from the received body",
    )
    value = decode_strict_json(encoded)
    require(value.get("sha") == head, "GitHub Git commit API head identity drifted")

    tree = value.get("tree")
    parents = value.get("parents")
    verification = value.get("verification")
    require(isinstance(tree, dict), "GitHub Git commit API tree is missing")
    require(isinstance(parents, list), "GitHub Git commit API parents are missing")
    require(isinstance(verification, dict), "GitHub signature verification is missing")
    require(verification.get("verified") is True, "GitHub signature is not verified")
    require(
        verification.get("reason") == "valid",
        "GitHub signature reason is not valid",
    )

    tree_sha = V1.require_sha(str(tree.get("sha", "")), "GitHub API tree")
    parent_shas: list[str] = []
    for index, parent in enumerate(parents):
        require(
            isinstance(parent, dict),
            f"GitHub API parent[{index}] is not an object",
        )
        parent_shas.append(
            V1.require_sha(str(parent.get("sha", "")), f"GitHub API parent[{index}]")
        )

    signature = verification.get("signature")
    signed_payload = verification.get("payload")
    require(
        isinstance(signature, str) and signature,
        "verified signature bytes are missing",
    )
    require(
        isinstance(signed_payload, str) and signed_payload,
        "verified payload is missing",
    )
    require("\x00" not in signature, "verified signature contains NUL")
    require("\x00" not in signed_payload, "verified payload contains NUL")
    require(
        len(signature.encode("utf-8")) <= MAX_SIGNATURE_BYTES,
        "verified signature exceeds the bounded size",
    )
    require(
        len(signed_payload.encode("utf-8")) <= MAX_SIGNED_PAYLOAD_BYTES,
        "verified payload exceeds the bounded size",
    )

    header_block, boundary, _message = signed_payload.partition("\n\n")
    require(boundary == "\n\n", "verified payload lacks the Git commit header boundary")
    header_lines = header_block.splitlines()
    signed_trees = [
        line.removeprefix("tree ") for line in header_lines if line.startswith("tree ")
    ]
    signed_parents = [
        line.removeprefix("parent ")
        for line in header_lines
        if line.startswith("parent ")
    ]
    require(
        signed_trees == [tree_sha],
        "verified payload tree differs from API metadata",
    )
    require(
        signed_parents == parent_shas,
        "verified payload parent order differs from API metadata",
    )

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
