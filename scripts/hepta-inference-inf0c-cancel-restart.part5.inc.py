def write_fake_helper(
    path: pathlib.Path,
    ollama_marker: pathlib.Path,
    lmstudio_marker: pathlib.Path,
) -> None:
    script = f"""#!/usr/bin/env python3
import pathlib
import sys
import time
markers = {{
    "ollama": pathlib.Path({str(ollama_marker)!r}),
    "lmstudio": pathlib.Path({str(lmstudio_marker)!r}),
}}
if len(sys.argv) != 3 or sys.argv[1] != "restart" or sys.argv[2] not in markers:
    raise SystemExit(64)
marker = markers[sys.argv[2]]
marker.touch()
time.sleep(0.5)
marker.unlink(missing_ok=True)
"""
    path.write_text(script, encoding="utf-8")
    path.chmod(0o700)


def run_self_test(receipt_path: pathlib.Path) -> int:
    with tempfile.TemporaryDirectory(prefix="hepta-inf0c-evidence-") as directory:
        root = pathlib.Path(directory)
        ollama_marker = root / "ollama.down"
        lmstudio_marker = root / "lmstudio.down"
        ollama_model = "fixture/ollama:1"
        lmstudio_model = "fixture/lmstudio:1"
        ollama_server = FakeServer(
            ("127.0.0.1", 0),
            fake_handler(FakeState("ollama", ollama_model, ollama_marker)),
        )
        lmstudio_server = FakeServer(
            ("127.0.0.1", 0),
            fake_handler(FakeState("lmstudio", lmstudio_model, lmstudio_marker)),
        )
        threads = [
            threading.Thread(target=ollama_server.serve_forever, daemon=True),
            threading.Thread(target=lmstudio_server.serve_forever, daemon=True),
        ]
        for thread in threads:
            thread.start()
        helper = root / "control-helper"
        write_fake_helper(helper, ollama_marker, lmstudio_marker)
        previous_path = os.environ.get(CONTROL_HELPER_ENV)
        previous_sha = os.environ.get(CONTROL_HELPER_SHA_ENV)
        os.environ[CONTROL_HELPER_ENV] = str(helper)
        os.environ[CONTROL_HELPER_SHA_ENV] = f"sha256:{hash_file(helper)}"
        try:
            receipt = execute_evidence(
                normalize_loopback_base(
                    f"http://127.0.0.1:{ollama_server.server_address[1]}"
                ),
                ollama_model,
                normalize_loopback_base(
                    f"http://127.0.0.1:{lmstudio_server.server_address[1]}/v1"
                ),
                lmstudio_model,
                timeout=5.0,
                restart_timeout=10.0,
                cancel_read_bytes=64,
                run_controlled_restart=True,
            )
            write_receipt(receipt_path, receipt)
            text = receipt_path.read_text(encoding="utf-8")
            require(CANCEL_PROMPT not in text, "raw cancellation prompt leaked")
            require(FOLLOWUP_PROMPT not in text, "raw follow-up prompt leaked")
            require(receipt["cancellation"]["executed"] is True, "cancellation not executed")
            require(
                receipt["cancellation"]["backend_acknowledged"] is False,
                "transport disconnect overclaimed backend acknowledgement",
            )
            require(receipt["controlled_restart"]["executed"] is True, "restart not executed")
            for provider in ("ollama", "lmstudio"):
                evidence = receipt["controlled_restart"][provider]
                require(evidence["service_unavailable_observed"] is True, "down not observed")
                require(evidence["service_recovered_observed"] is True, "up not observed")
        finally:
            if previous_path is None:
                os.environ.pop(CONTROL_HELPER_ENV, None)
            else:
                os.environ[CONTROL_HELPER_ENV] = previous_path
            if previous_sha is None:
                os.environ.pop(CONTROL_HELPER_SHA_ENV, None)
            else:
                os.environ[CONTROL_HELPER_SHA_ENV] = previous_sha
            ollama_server.shutdown()
            lmstudio_server.shutdown()
            ollama_server.server_close()
            lmstudio_server.server_close()
    print("PASS_HEPTA_INFERENCE_INF0C_CANCEL_RESTART_SELF_TEST")
    return 0


def main() -> int:
    args = parse_args()
    require(args.timeout_seconds > 0, "timeout must be positive")
    require(args.restart_timeout_seconds > 0, "restart timeout must be positive")
    require(args.cancel_read_bytes > 0, "cancel read size must be positive")
    require(args.self_test != args.execute, "select exactly one of --self-test or --execute")
    if args.self_test:
        return run_self_test(args.receipt)

    require(args.ollama_model is not None, "--ollama-model is required")
    require(args.lmstudio_model is not None, "--lmstudio-model is required")
    validate_model_id(args.ollama_model)
    validate_model_id(args.lmstudio_model)
    receipt = execute_evidence(
        normalize_loopback_base(args.ollama_base),
        args.ollama_model,
        normalize_loopback_base(args.lmstudio_base),
        args.lmstudio_model,
        args.timeout_seconds,
        args.restart_timeout_seconds,
        args.cancel_read_bytes,
        args.controlled_restart,
    )
    write_receipt(args.receipt, receipt)
    print("PASS_HEPTA_INFERENCE_INF0C_CANCEL_RESTART_EVIDENCE")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (QualificationError, OSError, subprocess.SubprocessError) as error:
        print(f"FAIL_HEPTA_INFERENCE_INF0C_CANCEL_RESTART: {error}", file=sys.stderr)
        raise SystemExit(1) from error
