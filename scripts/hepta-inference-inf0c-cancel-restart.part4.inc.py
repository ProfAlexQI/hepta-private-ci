def execute_evidence(
    ollama_endpoint: Endpoint,
    ollama_model: str,
    lmstudio_endpoint: Endpoint,
    lmstudio_model: str,
    timeout: float,
    restart_timeout: float,
    cancel_read_bytes: int,
    run_controlled_restart: bool,
) -> dict[str, Any]:
    cancellation = {
        "ollama": cancellation_probe(
            "ollama", ollama_endpoint, ollama_model, timeout, cancel_read_bytes
        ),
        "lmstudio": cancellation_probe(
            "lmstudio", lmstudio_endpoint, lmstudio_model, timeout, cancel_read_bytes
        ),
    }
    controlled_restart: dict[str, Any] = {
        "executed": False,
        "ollama": None,
        "lmstudio": None,
    }
    if run_controlled_restart:
        helper = load_control_helper()
        controlled_restart = {
            "executed": True,
            "ollama": controlled_restart_probe(
                "ollama",
                ollama_endpoint,
                ollama_model,
                helper,
                timeout,
                restart_timeout,
            ),
            "lmstudio": controlled_restart_probe(
                "lmstudio",
                lmstudio_endpoint,
                lmstudio_model,
                helper,
                timeout,
                restart_timeout,
            ),
        }

    cancel_prompt = CANCEL_PROMPT.encode("utf-8")
    followup_prompt = FOLLOWUP_PROMPT.encode("utf-8")
    return {
        "schema": "hepta.inference.inf0c.cancel_restart_evidence.v1",
        "source": {
            "commit": git_value("rev-parse", "HEAD"),
            "tree": git_value("rev-parse", "HEAD^{tree}"),
        },
        "scope": "QUALIFICATION_ONLY_LOCAL_CANCELLATION_AND_CONTROLLED_RESTART",
        "prompts": {
            "cancellation": {
                "sha256": hashlib.sha256(cancel_prompt).hexdigest(),
                "byte_length": len(cancel_prompt),
                "raw_persisted": False,
            },
            "followup": {
                "sha256": hashlib.sha256(followup_prompt).hexdigest(),
                "byte_length": len(followup_prompt),
                "raw_persisted": False,
            },
        },
        "cancellation": {
            "executed": True,
            "evidence_level": "TRANSPORT_DISCONNECT_WITH_POST_HEALTH_V1",
            "backend_acknowledged": False,
            "providers": cancellation,
        },
        "controlled_restart": controlled_restart,
        "implicit_download": False,
        "raw_model_output_persisted": False,
        "authority": {
            "production": False,
            "effect": False,
            "memory_write": False,
            "kg_write": False,
            "route_write": False,
            "fleet_write": False,
            "model_npu": False,
            "remote_inference": False,
            "promotion": False,
        },
        "qualified": False,
    }


class FakeState:
    def __init__(self, provider: str, model: str, marker: pathlib.Path) -> None:
        self.provider = provider
        self.model = model
        self.marker = marker


class FakeServer(ThreadingHTTPServer):
    daemon_threads = True


def fake_handler(state: FakeState) -> type[BaseHTTPRequestHandler]:
    class Handler(BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"

        def log_message(self, format_string: str, *args: Any) -> None:
            del format_string, args

        def unavailable(self) -> bool:
            if not state.marker.exists():
                return False
            self.send_response(503)
            self.send_header("Content-Length", "0")
            self.end_headers()
            return True

        def send_json(self, value: dict[str, Any]) -> None:
            body = json.dumps(value, separators=(",", ":")).encode("utf-8")
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def do_GET(self) -> None:
            if self.unavailable():
                return
            if state.provider == "ollama" and self.path == "/api/version":
                self.send_json({"version": "0.14.1"})
            elif state.provider == "ollama" and self.path == "/api/tags":
                self.send_json({"models": [{"name": state.model}]})
            elif state.provider == "lmstudio" and self.path == "/v1/models":
                self.send_json({"data": [{"id": state.model}]})
            else:
                self.send_response(404)
                self.send_header("Content-Length", "0")
                self.end_headers()

        def do_POST(self) -> None:
            if self.unavailable():
                return
            length = int(self.headers.get("Content-Length", "0"))
            payload = json.loads(self.rfile.read(length) or b"{}")
            expected = "/v1/responses"
            if self.path != expected:
                self.send_response(404)
                self.send_header("Content-Length", "0")
                self.end_headers()
                return
            if payload.get("stream"):
                self.send_response(200)
                self.send_header("Content-Type", "text/event-stream")
                self.send_header("Connection", "close")
                self.end_headers()
                try:
                    for index in range(128):
                        self.wfile.write(f"data: {index:04d}-HEPTA_CANCEL_STREAM_TOKEN\n\n".encode())
                        self.wfile.flush()
                        time.sleep(0.01)
                except (BrokenPipeError, ConnectionResetError):
                    return
            else:
                self.send_json({"id": "fixture", "status": "completed"})

    return Handler


