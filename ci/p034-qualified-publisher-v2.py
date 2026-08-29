#!/usr/bin/env python3
"""Run P0.3.4 publication with monotonic predecessor-gate enforcement."""

from __future__ import annotations

import importlib.util
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
PUBLISHER = HERE / "p034-qualified-publisher.py"
FORWARD_COMPAT = HERE / "p034-forward-compat.py"
P02_RUST_FILES = (
    "hepta-memory/src/framing.rs",
    "hepta-memory/src/fact_grounding/durable.rs",
    "hepta-memory/src/fact_grounding/durable/schema.rs",
    "hepta-memory/src/fact_grounding/durable/grounding.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/prepare.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/ledger.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/ledger/insert.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/ledger/verify.rs",
    "hepta-memory/src/fact_grounding/durable/grounding/ledger/support.rs",
    "hepta-memory/src/fact_grounding/durable/tests.rs",
)


def main() -> int:
    spec = importlib.util.spec_from_file_location("p034_publisher", PUBLISHER)
    if spec is None or spec.loader is None:
        raise SystemExit("cannot load the governed P0.3.4 publisher")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    original_decode = module.decode_payload

    def decode_with_forward_compat(destination: Path) -> Path:
        original_apply = original_decode(destination)
        wrapper = destination / "p034_apply_with_forward_compat.py"
        predecessor_format = [
            "rustfmt",
            "--edition",
            "2024",
            "--config",
            "skip_children=true",
            "--check",
            *P02_RUST_FILES,
        ]
        wrapper.write_text(
            "from __future__ import annotations\n"
            "import subprocess\n"
            "import sys\n"
            f"subprocess.run([sys.executable, {str(original_apply)!r}], check=True)\n"
            f"subprocess.run([sys.executable, {str(FORWARD_COMPAT)!r}], check=True)\n"
            "subprocess.run([sys.executable, "
            "'scripts/verify-hepta-intelligence-grounding-ledger.py'], "
            "check=True)\n"
            f"subprocess.run({predecessor_format!r}, cwd='codex-rs', check=True)\n",
            encoding="utf-8",
        )
        return wrapper

    module.decode_payload = decode_with_forward_compat
    return int(module.main())


if __name__ == "__main__":
    raise SystemExit(main())
