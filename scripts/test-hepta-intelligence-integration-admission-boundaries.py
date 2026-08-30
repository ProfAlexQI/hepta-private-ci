#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
from types import ModuleType, SimpleNamespace
import tempfile
import unittest
from unittest.mock import patch

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
    "hepta_intelligence_integration_admission_api_v2_boundary_test",
    "verify-hepta-intelligence-integration-admission-api-v2.py",
)
RUNNER = load_script(
    "hepta_intelligence_integration_admission_runner_boundary_test",
    "run-hepta-intelligence-integration-admission.py",
)


class FakeResponse:
    def __init__(
        self,
        payload: bytes,
        *,
        url: str = EXPECTED_URL,
        status: int = 200,
        content_type: str = "application/json; charset=utf-8",
        content_encoding: str | None = None,
        content_length: str | None = None,
    ) -> None:
        self.payload = payload
        self.url = url
        self.status = status
        self.headers: dict[str, str] = {"Content-Type": content_type}
        if content_encoding is not None:
            self.headers["Content-Encoding"] = content_encoding
        if content_length is None:
            content_length = str(len(payload))
        self.headers["Content-Length"] = content_length

    def __enter__(self) -> FakeResponse:
        return self

    def __exit__(self, *args: object) -> None:
        del args

    def geturl(self) -> str:
        return self.url

    def read(self, limit: int) -> bytes:
        return self.payload[:limit]


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
            "verified_at": "2026-08-30T16:52:35Z",
        },
    }


def encoded_metadata() -> bytes:
    return json.dumps(
        valid_metadata(),
        separators=(",", ":"),
        sort_keys=True,
    ).encode("utf-8")


class GitHubResponseBoundaryTest(unittest.TestCase):
    def fetch(self, response: FakeResponse) -> dict[str, object]:
        with patch.object(API.urllib.request, "urlopen", return_value=response):
            return API.fetch_commit_metadata(REPOSITORY, HEAD, "fixture-token")

    def test_valid_response_is_adapted(self) -> None:
        observed = self.fetch(FakeResponse(encoded_metadata()))
        self.assertEqual(observed["sha"], HEAD)
        self.assertEqual(observed["parents"], [{"sha": value} for value in PARENTS])
        commit = observed["commit"]
        self.assertIsInstance(commit, dict)
        assert isinstance(commit, dict)
        self.assertEqual(commit["tree"], {"sha": TREE})

    def test_duplicate_top_level_key_fails_closed(self) -> None:
        text = encoded_metadata().decode("utf-8")
        needle = f'"sha":"{HEAD}"'
        duplicate = text.replace(needle, f"{needle},{needle}", 1).encode("utf-8")
        with self.assertRaisesRegex(API.V1.AdmissionError, "duplicate JSON key"):
            self.fetch(FakeResponse(duplicate))

    def test_duplicate_nested_key_fails_closed(self) -> None:
        text = encoded_metadata().decode("utf-8")
        needle = f'"sha":"{TREE}"'
        duplicate = text.replace(needle, f"{needle},{needle}", 1).encode("utf-8")
        with self.assertRaisesRegex(API.V1.AdmissionError, "duplicate JSON key"):
            self.fetch(FakeResponse(duplicate))

    def test_redirect_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "redirected"):
            self.fetch(
                FakeResponse(
                    encoded_metadata(),
                    url="https://example.invalid/substitute",
                )
            )

    def test_non_json_media_type_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "media type"):
            self.fetch(
                FakeResponse(
                    encoded_metadata(),
                    content_type="text/html; charset=utf-8",
                )
            )

    def test_content_encoding_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "content encoding"):
            self.fetch(
                FakeResponse(
                    encoded_metadata(),
                    content_encoding="gzip",
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

    def test_declared_oversize_fails_closed(self) -> None:
        with self.assertRaisesRegex(API.V1.AdmissionError, "bounded size"):
            self.fetch(
                FakeResponse(
                    encoded_metadata(),
                    content_length=str(API.MAX_GIT_COMMIT_RESPONSE_BYTES + 1),
                )
            )

    def test_streamed_oversize_fails_closed(self) -> None:
        payload = b"x" * (API.MAX_GIT_COMMIT_RESPONSE_BYTES + 1)
        with self.assertRaisesRegex(API.V1.AdmissionError, "payload exceeds"):
            API.read_bounded(
                FakeResponse(payload),
                API.MAX_GIT_COMMIT_RESPONSE_BYTES,
            )

    def test_unverified_signature_fails_closed_in_bounded_adapter(self) -> None:
        value = valid_metadata()
        verification = value["verification"]
        assert isinstance(verification, dict)
        verification["verified"] = False
        payload = json.dumps(value, separators=(",", ":")).encode("utf-8")
        with self.assertRaisesRegex(API.V1.AdmissionError, "not verified"):
            self.fetch(FakeResponse(payload))

    def test_signed_parent_order_drift_fails_closed(self) -> None:
        value = valid_metadata()
        verification = value["verification"]
        assert isinstance(verification, dict)
        payload = str(verification["payload"])
        verification["payload"] = payload.replace(
            f"parent {PARENTS[0]}\nparent {PARENTS[1]}",
            f"parent {PARENTS[1]}\nparent {PARENTS[0]}",
        )
        encoded = json.dumps(value, separators=(",", ":")).encode("utf-8")
        with self.assertRaisesRegex(API.V1.AdmissionError, "parent order"):
            self.fetch(FakeResponse(encoded))


class RunnerEnvironmentBoundaryTest(unittest.TestCase):
    def parent_environment(
        self,
        home: Path,
        *,
        token_name: str = "GH_TOKEN",
    ) -> dict[str, str]:
        return {
            "PATH": "/usr/bin:/bin",
            "HOME": str(home),
            "LANG": "C",
            "LC_ALL": "C",
            "PYTHONHASHSEED": "0",
            "PYTHONNOUSERSITE": "1",
            "PYTHONPYCACHEPREFIX": str(home / "python-cache"),
            "PYTHONDONTWRITEBYTECODE": "1",
            "GITHUB_ACTIONS": "true",
            "RUNNER_OS": "Linux",
            token_name: "fixture-token",
        }

    def fake_context(self) -> SimpleNamespace:
        return SimpleNamespace(
            executable=Path("/usr/bin/git"),
            root=Path("/repo"),
            git_dir=Path("/repo/.git"),
            index=Path("/repo/.git/index"),
            env={
                "PATH": "/attacker:/usr/bin",
                "LD_PRELOAD": "/attacker/lib.so",
                "PYTHONPATH": "/attacker/python",
                "GIT_OBJECT_DIRECTORY": "/attacker/objects",
                "HTTPS_PROXY": "http://attacker.invalid",
            },
        )

    def test_exact_parent_environment_and_child_allowlist(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            source = self.parent_environment(Path(directory))
            RUNNER.validate_parent_environment(source, token_name="GH_TOKEN")
            child = RUNNER.child_environment(
                self.fake_context(),
                token_name="GH_TOKEN",
                source=source,
            )
        expected = {
            "PATH",
            "LANG",
            "LC_ALL",
            *RUNNER.CANONICAL_GIT_NAMES,
            *RUNNER.CHILD_PYTHON_ENVIRONMENT,
            "GH_TOKEN",
        }
        self.assertEqual(set(child), expected)
        self.assertEqual(child["PATH"], "/usr/bin")
        for forbidden in (
            "HOME",
            "GITHUB_ACTIONS",
            "RUNNER_OS",
            "LD_PRELOAD",
            "PYTHONPATH",
            "PYTHONHOME",
            "SSL_CERT_FILE",
            "SSL_CERT_DIR",
            "REQUESTS_CA_BUNDLE",
            "BASH_ENV",
            "ENV",
            "GIT_OBJECT_DIRECTORY",
            "HTTPS_PROXY",
        ):
            self.assertNotIn(forbidden, child)

    def test_full_worktree_status_includes_untracked_and_ignored(self) -> None:
        calls: list[tuple[str, ...]] = []

        def fake_run_bound_git(_context: object, *args: str) -> SimpleNamespace:
            calls.append(args)
            return SimpleNamespace(stdout="", stderr="", returncode=0)

        with patch.object(RUNNER, "run_bound_git", side_effect=fake_run_bound_git):
            self.assertEqual(RUNNER.worktree_status(self.fake_context()), "")
        self.assertEqual(
            calls,
            [
                (
                    "status",
                    "--porcelain=v1",
                    "--untracked-files=all",
                    "--ignored=matching",
                )
            ],
        )

    def test_parent_environment_pollution_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            source = self.parent_environment(Path(directory))
            source["LD_PRELOAD"] = "/attacker/lib.so"
            with self.assertRaisesRegex(
                RUNNER.AdmissionRunnerError,
                "parent environment is not exact",
            ):
                RUNNER.validate_parent_environment(source, token_name="GH_TOKEN")

    def test_case_colliding_token_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            source = self.parent_environment(Path(directory))
            source["gh_token"] = "other-token"
            with self.assertRaisesRegex(
                RUNNER.AdmissionRunnerError,
                "exactly once",
            ):
                RUNNER.exact_environment_value(source, "GH_TOKEN")

    def test_non_printable_token_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            source = self.parent_environment(Path(directory))
            source["GH_TOKEN"] = "fixture\ntoken"
            with self.assertRaisesRegex(
                RUNNER.AdmissionRunnerError,
                "printable ASCII",
            ):
                RUNNER.validate_parent_environment(source, token_name="GH_TOKEN")

    def test_token_name_is_closed_world(self) -> None:
        self.assertEqual(
            RUNNER.token_environment_name(["--token-env", "GH_TOKEN"]),
            "GH_TOKEN",
        )
        self.assertEqual(
            RUNNER.token_environment_name(["--token-env=GITHUB_TOKEN"]),
            "GITHUB_TOKEN",
        )
        for name in (
            "LD_PRELOAD",
            "PYTHONPATH",
            "PYTHONHOME",
            "SSL_CERT_FILE",
            "HTTP_PROXY",
            "PATH",
            "GIT_DIR",
            "AWS_SECRET_ACCESS_KEY",
        ):
            with self.assertRaisesRegex(
                RUNNER.AdmissionRunnerError,
                "fixed admission allowlist",
            ):
                RUNNER.token_environment_name(["--token-env", name])


if __name__ == "__main__":
    unittest.main(verbosity=2)
