#!/usr/bin/env python3
"""Command-line interface and create-only evidence writer."""

from __future__ import annotations

from .common import *  # noqa: F401,F403

from .classifier import *  # noqa: F401,F403
from .contracts import *  # noqa: F401,F403

def parse_jobs_arguments(values: list[str]) -> dict[int, dict[str, Any]]:
    jobs_by_run: dict[int, dict[str, Any]] = {}
    for value in values:
        run_id_text, separator, path_text = value.partition("=")
        if not separator or not run_id_text.isdigit() or not path_text:
            fail("--jobs-json entries must use RUN_ID=PATH")
        run_id = int(run_id_text)
        if run_id <= 0 or run_id in jobs_by_run:
            fail("--jobs-json run ids must be positive and unique")
        payload = load_json(pathlib.Path(path_text))
        jobs_by_run[run_id] = require_object(
            payload,
            f"jobs payload for run {run_id}",
        )
    return jobs_by_run


def write_output(path: pathlib.Path | None, value: dict[str, Any]) -> None:
    verify_evidence_digest(value)
    body = canonical(value) + b"\n"
    if path is None:
        sys.stdout.buffer.write(body)
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    flags = os.O_CREAT | os.O_EXCL | os.O_WRONLY
    if hasattr(os, "O_NOFOLLOW"):
        flags |= os.O_NOFOLLOW
    try:
        descriptor = os.open(path, flags, 0o600)
    except FileExistsError:
        fail(f"refusing to overwrite evidence output: {path}")
    try:
        with os.fdopen(descriptor, "wb") as output:
            output.write(body)
            output.flush()
            os.fsync(output.fileno())
    except BaseException:
        try:
            path.unlink()
        except OSError:
            pass
        raise
    if os.name == "posix":
        directory_descriptor = os.open(path.parent, os.O_RDONLY)
        try:
            os.fsync(directory_descriptor)
        finally:
            os.close(directory_descriptor)


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument("--head-sha")
    result.add_argument("--runs-json", type=pathlib.Path)
    result.add_argument(
        "--jobs-json",
        action="append",
        default=[],
        metavar="RUN_ID=PATH",
    )
    result.add_argument("--output", type=pathlib.Path)
    result.add_argument("--verify-evidence", type=pathlib.Path)
    return result


def main(argv: list[str] | None = None) -> int:
    args = parser().parse_args(argv)
    try:
        policy = verify_policy()
        verify_recovery_contract(policy=policy)
        verify_plan_bindings()
        classify_requested = any(
            (
                args.head_sha is not None,
                args.runs_json is not None,
                bool(args.jobs_json),
                args.output is not None,
            )
        )
        if args.verify_evidence is not None:
            if classify_requested:
                fail("--verify-evidence cannot be combined with classification")
            evidence = verify_evidence_digest(load_json(args.verify_evidence))
            print(
                json.dumps(
                    {
                        "disposition": evidence.get("disposition"),
                        "schema": "hepta.browser.runner_qualification_evidence.verification.v2",
                        "status": "PASS_BOUND_RUNNER_EVIDENCE_DIGEST",
                    },
                    sort_keys=True,
                    separators=(",", ":"),
                )
            )
            return 0
        if not classify_requested:
            print(
                json.dumps(
                    {
                        "authority": "all_false",
                        "plan_bindings": "verified",
                        "recovery_contract": "verified",
                        "required_workflows": [
                            item["workflow_name"]
                            for item in policy["required_workflows"]
                        ],
                        "schema": (
                            "hepta.browser.runner_qualification_policy."
                            "verification.v2"
                        ),
                        "status": "PASS_FAIL_CLOSED_RUNNER_EVIDENCE_CONTRACT_V2",
                    },
                    sort_keys=True,
                    separators=(",", ":"),
                )
            )
            return 0
        if args.head_sha is None or args.runs_json is None:
            fail("classification requires --head-sha and --runs-json")
        runs_payload = load_json(args.runs_json)
        jobs_by_run = parse_jobs_arguments(args.jobs_json)
        evidence = classify(
            runs_payload,
            jobs_by_run,
            args.head_sha,
            policy,
        )
        write_output(args.output, evidence)
        return int(evidence["exit_code"])
    except EvidenceError as error:
        print(f"HEPTA_RUNNER_EVIDENCE=FAIL: {error}", file=sys.stderr)
        return 2

