def parse_sha256_binding(binding: str) -> str:
    digest = binding.removeprefix("sha256:")
    require(binding.startswith("sha256:"), "control helper digest must use sha256:")
    require(
        len(digest) == 64 and all(character in "0123456789abcdef" for character in digest),
        "control helper digest must be 64 lowercase hex characters",
    )
    return digest


def hash_file(path: pathlib.Path) -> str:
    hasher = hashlib.sha256()
    with path.open("rb") as handle:
        while chunk := handle.read(1024 * 1024):
            hasher.update(chunk)
    return hasher.hexdigest()


def load_control_helper() -> ControlHelper:
    configured = os.environ.get(CONTROL_HELPER_ENV, "")
    expected = os.environ.get(CONTROL_HELPER_SHA_ENV, "")
    require(configured, f"{CONTROL_HELPER_ENV} is required for controlled restart")
    require(expected, f"{CONTROL_HELPER_SHA_ENV} is required for controlled restart")
    path = pathlib.Path(configured)
    require(path.is_absolute(), "control helper path must be absolute")
    metadata = path.lstat()
    require(not stat.S_ISLNK(metadata.st_mode), "control helper may not be a symlink")
    require(stat.S_ISREG(metadata.st_mode), "control helper must be a regular file")
    require(os.access(path, os.X_OK), "control helper is not executable")
    if os.name != "nt":
        require(metadata.st_mode & 0o022 == 0, "control helper is group/world writable")
    canonical = path.resolve(strict=True)
    digest = hash_file(canonical)
    require(digest == parse_sha256_binding(expected), "control helper digest mismatch")
    return ControlHelper(path=canonical, sha256=digest)


def sanitized_subprocess_environment() -> dict[str, str]:
    return {
        name: value
        for name in SUBPROCESS_ENV_ALLOWLIST
        if (value := os.environ.get(name)) is not None
    }


def service_available(provider: str, endpoint: Endpoint, timeout: float) -> bool:
    suffix = "/api/version" if provider == "ollama" else "/models"
    try:
        request_json("GET", endpoint, suffix, min(timeout, 1.0))
    except QualificationError:
        return False
    return True


def controlled_restart_probe(
    provider: str,
    endpoint: Endpoint,
    model: str,
    helper: ControlHelper,
    timeout: float,
    restart_timeout: float,
) -> dict[str, Any]:
    require(service_available(provider, endpoint, timeout), f"{provider} unavailable before restart")
    started = time.monotonic()
    process = subprocess.Popen(
        [str(helper.path), "restart", provider],
        stdin=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        env=sanitized_subprocess_environment(),
        shell=False,
    )
    down_at: float | None = None
    up_at: float | None = None
    deadline = started + restart_timeout
    try:
        while time.monotonic() < deadline:
            available = service_available(provider, endpoint, min(timeout, 1.0))
            now = time.monotonic()
            if not available and down_at is None:
                down_at = now
            if available and down_at is not None:
                up_at = now
                if process.poll() is not None:
                    break
            if process.poll() is not None and down_at is None:
                break
            time.sleep(0.1)
        if process.poll() is None:
            process.kill()
        exit_code = process.wait(timeout=5)
    except BaseException:
        if process.poll() is None:
            process.kill()
            process.wait(timeout=5)
        raise

    require(exit_code == 0, f"control helper failed for {provider}: exit={exit_code}")
    require(down_at is not None, f"{provider} restart never became unavailable")
    require(up_at is not None, f"{provider} restart did not recover")
    post_restart = service_probe(provider, endpoint, model, timeout)
    return {
        "helper": {
            "basename": helper.path.name,
            "sha256": helper.sha256,
            "shell": False,
            "environment_allowlist": list(SUBPROCESS_ENV_ALLOWLIST),
        },
        "service_unavailable_observed": True,
        "service_recovered_observed": True,
        "downtime_ms": max(0, int((up_at - down_at) * 1000)),
        "helper_elapsed_ms": max(0, int((time.monotonic() - started) * 1000)),
        "helper_exit_code": exit_code,
        "post_restart_health": post_restart,
    }


def git_value(*args: str) -> str:
    completed = subprocess.run(
        ["git", *args],
        check=False,
        capture_output=True,
        text=True,
    )
    return completed.stdout.strip() if completed.returncode == 0 else "UNAVAILABLE"


def write_receipt(path: pathlib.Path, receipt: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8") as handle:
            json.dump(receipt, handle, indent=2, sort_keys=True)
            handle.write("\n")
            handle.flush()
            os.fsync(handle.fileno())
    except BaseException:
        path.unlink(missing_ok=True)
        raise
    if os.name != "nt":
        require(stat.S_IMODE(path.stat().st_mode) == 0o600, "receipt is not owner-only")


