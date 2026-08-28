from __future__ import annotations

from pathlib import Path


def replace_exact(path: str, old: str, new: str, expected: int = 1) -> None:
    target = Path(path)
    text = target.read_text(encoding="utf-8")
    count = text.count(old)
    if count != expected:
        raise SystemExit(f"{path}: expected {expected} replacement(s), found {count}")
    target.write_text(text.replace(old, new), encoding="utf-8")


def main() -> None:
    replace_exact(
        "codex-rs/ollama/src/client_support.inc.rs",
        '''format!(concat!(
                "OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={operation} ",
                "maximum={MAX_CONTROL_RESPONSE_BYTES}"
            ))''',
        '''format!(
                "OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={} maximum={}",
                operation, MAX_CONTROL_RESPONSE_BYTES
            )''',
    )
    replace_exact(
        "codex-rs/ollama/src/client_support.inc.rs",
        '''format!(concat!(
                    "OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={operation} ",
                    "maximum={MAX_CONTROL_RESPONSE_BYTES}"
                ))''',
        '''format!(
                    "OLLAMA_CONTROL_RESPONSE_TOO_LARGE operation={} maximum={}",
                    operation, MAX_CONTROL_RESPONSE_BYTES
                )''',
    )
    replace_exact(
        "codex-rs/ollama/src/lib.rs",
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
    )
    replace_exact(
        "codex-rs/lmstudio/src/lib.rs",
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
    )


if __name__ == "__main__":
    main()
