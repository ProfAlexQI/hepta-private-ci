#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
from types import ModuleType
from unittest.mock import patch
import unittest

SCRIPTS = Path(__file__).resolve().parent
HEAD = "a" * 40
TREE = "b" * 40
PARENTS = ["c" * 40, "d" * 40]
REPOSITORY = "ProfHepta/hepta-private-ci"
EXPECTED_URL = (
    "https://api.github.com/repos/ProfHepta/hepta-private-ci/git/commits/" + HEAD
)


def load_script(module_name: str, filename: str) -> ModuleType:
    path = SCRIPTS / filename
    spec = importlib.util.spec_from_file_location(module_name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


API = load_script(
    "hepta_intelligence_integration_admission_api_v2_http_test",
    "verify-hepta-intelligence-integration-admission-api-v2.py",
)


class FakeHeaders:
    def __init__(self, values: list[tuple[str, str]]) -> None:
        self.values = values

    def get_all(self, name: str, default: list[str] | None = None) -> list[str]:
        matches = [
            value
            for key, value in self.values
            if key.casefold() == name.casefold()
        ]
        return matches if matches else ([] if default is None else default)

    def get(self, name: str) -> str | None:
        values = self.get_all(name)
        return values[0] if values else None


class FakeResponse:
    def __init__(
        self,
        payload: bytes,
        *,
        url: str = EXPECTED_URL,
        status: int = 200,
        headers: list[tuple[str, str]] | None = None,
        include_content_length: bool = True,
        content_length: str | None = None,
        max_chunk: int | None = None,
    ) -> None:
        self.payload = payload
        self.url = url
        self.status = status
        self.offset = 0
        self.max_chunk = max_chunk
        values = list(headers or [("Content-Type", "application/json; charset=utf-8")])
        if include_content_length:
            declared = str(len(payload)) if content_length is None else content_length
            values.append(("Content-Length", declared))
        self.headers = FakeHeaders(values)

    def __enter__(self) -> FakeResponse:
        return self

    def __exit__(self, *args: object) -> None:
        del args

    def geturl(self) -> str:
        return self.url

    def read(self, limit: int) -> bytes:
        if self.offset >= len(self.payload):
            return b""
        size = limit
        if self.max_chunk is not None:
            size = min(size, self.max_chunk)
        chunk = self.payload[self.offset : self.offset + size]
        self.offset += len(chunk)
        return chunk


def valid_metadata() -> dict[str, object]:
    signed_payload = "\n".join(
        [
            f"tree {TREE}",
            *(f"parent {parent}" for parent in PARENTS),
            "author Hepta Fixture <fixture@example.invalid> 0 +0000",
            "committer GitHub <noreply@github.com> 0 +0000",
            "",
            "fixture",
        ]
    )
    return {
        "sha": HEAD,
        "tree": {"sha": TREE},
        "parents": [{"sha": parent} for parent in PARENTS],
        "verification": {
            "verified": True,
            "reason": "valid",
            "signature": "fixture-signature",
            "payload": signed_payload,
            "verified_at": "2026-08-30T17:34:17Z",
        },
    }


def encoded_metadata() -> bytes:
    return json.dumps(
        valid_metadata(),
        separators=(",", ":"),
        sort_keys=True,
    ).encode("utf-8")


class GitHubHttpBoundaryTest(unittest.TestCase):
    def fetch(self, response: FakeResponse) -> dict[str, object]:
        with patch.object(API.URL_OPENER, "open", return_value=response):
            return API.fetch_commit_metadata(REPOSITORY, HEAD, "fixture-token")

    def test_valid_short_reads_are_reassembled(self) -> None:
        observed = self.fetch(FakeResponse(encoded_metadata(), max_chunk=7))
        self.assertEqual(observed["sha"], HEAD)

    def test_redirect_handler_fails_before_follow(self) -> None:
        handler = API.RejectRedirectHandler()
        with self.assertRaisesRegex(API.V1.AdmissionError, "before follow"):
            handler.redirect_request(
                object(),
                None,
                302,
                "Found",
                {},
                "https://example.invalid/leak",
            )

    def test_final_url_drift_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "redirected"):
            self.fetch(
                FakeResponse(
                    encoded_metadata(),
                    url="https://example.invalid/substitute",
                )
            )

    def test_duplicate_content_type_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "cardinality"):
            self.fetch(
                FakeResponse(
                    encoded_metadata(),
                    headers=[
                        ("Content-Type", "application/json"),
                        ("Content-Type", "application/vnd.github+json"),
                    ],
                )
            )

    def test_non_json_media_type_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "media type"):
            self.fetch(
                FakeResponse(
                    encoded_metadata(),
                    headers=[("Content-Type", "text/html; charset=utf-8")],
                )
            )

    def test_non_utf8_charset_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "charset"):
            self.fetch(
                FakeResponse(
                    encoded_metadata(),
                    headers=[("Content-Type", "application/json; charset=utf-16")],
                )
            )

    def test_unknown_content_type_parameter_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "not allowed"):
            self.fetch(
                FakeResponse(
                    encoded_metadata(),
                    headers=[
                        (
                            "Content-Type",
                            "application/json; charset=utf-8; profile=fixture",
                        )
                    ],
                )
            )

    def test_content_encoding_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "content encoding"):
            self.fetch(
                FakeResponse(
                    encoded_metadata(),
                    headers=[
                        ("Content-Type", "application/json"),
                        ("Content-Encoding", "gzip"),
                    ],
                )
            )

    def test_noncanonical_content_length_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "Content-Length"):
            self.fetch(
                FakeResponse(
                    encoded_metadata(),
                    content_length="+123",
                )
            )

    def test_duplicate_content_length_fails_closed(self) -> None:
        payload = encoded_metadata()
        with self.assertRaisesRegex(API.V1.AdmissionError, "duplicated"):
            self.fetch(
                FakeResponse(
                    payload,
                    headers=[
                        ("Content-Type", "application/json"),
                        ("Content-Length", str(len(payload))),
                    ],
                    content_length=str(len(payload)),
                )
            )

    def test_declared_oversize_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "bounded size"):
            self.fetch(
                FakeResponse(
                    encoded_metadata(),
                    content_length=str(API.MAX_GIT_COMMIT_RESPONSE_BYTES + 1),
                )
            )

    def test_received_length_mismatch_fails_closed(self) -> None:
        payload = encoded_metadata()
        with self.assertRaisesRegex(API.V1.AdmissionError, "differs"):
            self.fetch(
                FakeResponse(
                    payload,
                    content_length=str(len(payload) + 1),
                )
            )

    def test_streamed_oversize_fails_closed(self) -> None:
        payload = b"x" * (API.MAX_GIT_COMMIT_RESPONSE_BYTES + 1)
        with self.assertRaisesRegex(API.V1.AdmissionError, "payload exceeds"):
            self.fetch(
                FakeResponse(
                    payload,
                    include_content_length=False,
                    max_chunk=4096,
                )
            )

    def test_utf8_bom_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "BOM"):
            self.fetch(FakeResponse(b"\xef\xbb\xbf" + encoded_metadata()))

    def test_utf16_json_fails_closed(self) -> None:
        payload = json.dumps(valid_metadata()).encode("utf-16")
        with self.assertRaisesRegex(API.V1.AdmissionError, "strict UTF-8"):
            self.fetch(
                FakeResponse(
                    payload,
                    headers=[("Content-Type", "application/json")],
                )
            )


if __name__ == "__main__":
    unittest.main(verbosity=2)
