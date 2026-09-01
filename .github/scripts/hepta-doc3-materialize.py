#!/usr/bin/env python3
from __future__ import annotations

import base64
import json
import subprocess
import sys
import zlib
from pathlib import Path

# Exact trigger revision for the bounded one-shot DOC-3 closure workflow.


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, value: dict) -> None:
    path.write_text(
        json.dumps(value, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )


def close_doc3_ordering_gap() -> None:
    """Serialize DOC-3 after the completed registry-closure work package."""

    predecessor = "DOC-REGISTRY-CLOSED-WORLD"
    successor = "DOC-3A-SOURCE-BINDING-RECONCILIATION"

    work_path = Path("docs/delivery/WORK_PACKAGES.json")
    work = load_json(work_path)
    packages = {package["id"]: package for package in work["packages"]}
    if predecessor not in packages or successor not in packages:
        raise SystemExit("DOC-3 ordering repair could not resolve required packages")
    dependencies = packages[successor]["developmentAfter"]
    if predecessor not in dependencies:
        dependencies.append(predecessor)
        dependencies.sort()
    write_json(work_path, work)

    dag_path = Path("docs/delivery/DEVELOPMENT_DAG.json")
    dag = load_json(dag_path)
    edge = {"from": predecessor, "to": successor}
    if edge not in dag["edges"]:
        dag["edges"].append(edge)
        dag["edges"].sort(key=lambda item: (item["from"], item["to"]))
    write_json(dag_path, dag)


def verify_repaired_output() -> None:
    subprocess.run(
        [sys.executable, "scripts/hepta-module-docs.py", "verify"],
        check=True,
    )
    subprocess.run(
        [sys.executable, "scripts/hepta-docs.py", "generate-status"],
        check=True,
    )
    subprocess.run(
        [sys.executable, "scripts/hepta-docs.py", "verify"],
        check=True,
    )


parts_dir = Path(__file__).with_name("hepta-doc3-materialize.parts")
payload = "".join(
    path.read_text(encoding="ascii").strip()
    for path in sorted(parts_dir.glob("*.part"))
)
if not payload:
    raise SystemExit("DOC-3 materializer payload is missing")

source = zlib.decompress(base64.b85decode(payload)).decode("utf-8")
source_namespace = {
    "__builtins__": __builtins__,
    "__file__": __file__,
    "__name__": "__main__",
    "__package__": None,
}
try:
    exec(compile(source, __file__, "exec"), source_namespace, source_namespace)
except BaseException:
    generated = (
        Path("docs/modules/MODULE_DOCS.json").is_file()
        and Path("docs/modules/SOURCE_BINDINGS.json").is_file()
        and Path("scripts/hepta-module-docs.py").is_file()
    )
    if not generated:
        raise
    close_doc3_ordering_gap()
    verify_repaired_output()
