from __future__ import annotations

from pathlib import Path


OLLAMA_SUPPORT = Path("codex-rs/ollama/src/client_support.inc.rs")
OLLAMA_LIB = Path("codex-rs/ollama/src/lib.rs")
LMSTUDIO_LIB = Path("codex-rs/lmstudio/src/lib.rs")
CANONICAL_WORKFLOW = Path(".github/workflows/hepta-inference-inf0.yml")


def replace_optional(path: Path, old: str, new: str, label: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count > 1:
        raise SystemExit(f"{label}: expected at most one old form, found {count}")
    if count == 1:
        path.write_text(text.replace(old, new), encoding="utf-8")


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def converge_format_macros() -> None:
    replace_optional(
        OLLAMA_SUPPORT,
        '''format!(concat!(
                "OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={operation} ",
                "maximum={MAX_CONTROL_RESPONSE_BYTES}"
            ))''',
        '''format!(
                "OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={} maximum={}",
                operation, MAX_CONTROL_RESPONSE_BYTES
            )''',
        "Ollama bounded control response preflight",
    )
    replace_optional(
        OLLAMA_SUPPORT,
        '''format!(concat!(
                    "OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={operation} ",
                    "maximum={MAX_CONTROL_RESPONSE_BYTES}"
                ))''',
        '''format!(
                    "OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={} maximum={}",
                    operation, MAX_CONTROL_RESPONSE_BYTES
                )''',
        "Ollama bounded control response stream",
    )
    replace_optional(
        OLLAMA_LIB,
        '''format!(
            concat!(
                "OLLAMA_MODEL_NOT_INSTALLED model={model}; ",
                "automatic model installation is disabled. ",
                "Run `ollama pull {model}` explicitly and retry."
            )
        )''',
        '''format!(
            "OLLAMA_MODEL_NOT_INSTALLED model={}; automatic model installation is disabled. Run `ollama pull {}` explicitly and retry.",
            model, model
        )''',
        "Ollama missing model diagnostic",
    )
    replace_optional(
        LMSTUDIO_LIB,
        '''format!(
                concat!(
                    "LMSTUDIO_MODEL_NOT_INSTALLED model={model}; ",
                    "automatic model installation is disabled. ",
                    "Install the model explicitly in LM Studio and retry."
                )
            )''',
        '''format!(
                "LMSTUDIO_MODEL_NOT_INSTALLED model={}; automatic model installation is disabled. Install the model explicitly in LM Studio and retry.",
                model
            )''',
        "LM Studio missing model diagnostic",
    )

    support = OLLAMA_SUPPORT.read_text(encoding="utf-8")
    ollama = OLLAMA_LIB.read_text(encoding="utf-8")
    lmstudio = LMSTUDIO_LIB.read_text(encoding="utf-8")
    require(
        support.count("OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={} maximum={}") == 2,
        "Ollama bounded control diagnostics are not explicitly bound twice",
    )
    require(
        "OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={operation}" not in support,
        "Ollama implicit format capture remains",
    )
    require(
        "OLLAMA_MODEL_NOT_INSTALLED model={model}" not in ollama
        and "OLLAMA_MODEL_NOT_INSTALLED model={};" in ollama,
        "Ollama missing-model format binding is not converged",
    )
    require(
        "LMSTUDIO_MODEL_NOT_INSTALLED model={model}" not in lmstudio
        and "LMSTUDIO_MODEL_NOT_INSTALLED model={};" in lmstudio,
        "LM Studio missing-model format binding is not converged",
    )


def converge_canonical_workflow() -> None:
    old_format = '''      - name: Workspace formatting
        shell: bash
        run: cargo fmt --all -- --check
'''
    new_format = '''      - name: Check owned inference package formatting
        shell: bash
        run: |
          cargo fmt \\
            -p codex-lmstudio \\
            -p codex-ollama \\
            -p codex-utils-oss \\
            -p codex-responses-api-proxy \\
            -- --check
'''
    old_clippy = '''          cargo clippy --locked --all-targets \\
            -p codex-lmstudio \\
            -p codex-ollama \\
            -p codex-utils-oss \\
            -p codex-responses-api-proxy \\
            -- -D warnings
'''
    new_clippy = '''          cargo clippy --locked --all-targets --no-deps \\
            -p codex-lmstudio \\
            -p codex-ollama \\
            -p codex-utils-oss \\
            -p codex-responses-api-proxy \\
            -- -D warnings
'''

    replace_optional(CANONICAL_WORKFLOW, old_format, new_format, "canonical formatting scope")
    replace_optional(CANONICAL_WORKFLOW, old_clippy, new_clippy, "canonical Clippy scope")

    workflow = CANONICAL_WORKFLOW.read_text(encoding="utf-8")
    require("cargo fmt --all -- --check" not in workflow, "whole-workspace formatting remains")
    require(
        workflow.count("Check owned inference package formatting") == 1,
        "owned package formatting gate missing or duplicated",
    )
    require(
        workflow.count("cargo clippy --locked --all-targets --no-deps") == 1,
        "owned package Clippy --no-deps gate missing or duplicated",
    )


def main() -> None:
    converge_format_macros()
    converge_canonical_workflow()


if __name__ == "__main__":
    main()
