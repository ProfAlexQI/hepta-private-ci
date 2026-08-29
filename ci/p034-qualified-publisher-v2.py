#!/usr/bin/env python3
"""Run the P0.3.4 publisher with the forward-compatibility repair hook."""

from __future__ import annotations

import importlib.util
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
PUBLISHER = HERE / "p034-qualified-publisher.py"
FORWARD_COMPAT = HERE / "p034-forward-compat.py"


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
        wrapper.write_text(
            "from __future__ import annotations\n"
            "import subprocess\n"
            "import sys\n"
            f"subprocess.run([sys.executable, {str(original_apply)!r}], check=True)\n"
            f"subprocess.run([sys.executable, {str(FORWARD_COMPAT)!r}], check=True)\n",
            encoding="utf-8",
        )
        return wrapper

    module.decode_payload = decode_with_forward_compat
    return int(module.main())


if __name__ == "__main__":
    raise SystemExit(main())
