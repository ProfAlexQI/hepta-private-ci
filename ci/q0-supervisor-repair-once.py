#!/usr/bin/env python3
"""Apply the exact bounded supervisor source repair for the Q0 ARM64 carrier."""

from __future__ import annotations

from pathlib import Path


def replace_exact(path: str, old: str, new: str, expected: int = 1) -> None:
    file = Path(path)
    text = file.read_text(encoding="utf-8")
    count = text.count(old)
    if count != expected:
        raise AssertionError(
            f"{path}: expected {expected} copies, found {count}: {old[:160]!r}"
        )
    file.write_text(text.replace(old, new, expected), encoding="utf-8")


def main() -> int:
    replace_exact(
        "codex-rs/hepta-supervisor/src/signed_authority.rs",
        "H7ArtifactVerifier::new(signer_id.clone(), signer_epoch, verifying_key.clone())",
        "H7ArtifactVerifier::new(signer_id.clone(), signer_epoch, verifying_key)",
    )
    replace_exact(
        "codex-rs/hepta-supervisor/src/signed_intent.rs",
        "impl SignedSupervisorIntent {\n    pub fn new(",
        "impl SignedSupervisorIntent {\n"
        "    #[expect(\n"
        "        clippy::too_many_arguments,\n"
        "        reason = \"the durable intent binds every signed release and fencing field explicitly\"\n"
        "    )]\n"
        "    pub fn new(",
    )
    replace_exact(
        "codex-rs/hepta-supervisor/src/signed_intent.rs",
        "    fn compute_digest(&self) -> Sha256Digest {",
        "    #[expect(\n"
        "        clippy::expect_used,\n"
        "        reason = \"the fixed tuple contains only infallibly serializable typed fields\"\n"
        "    )]\n"
        "    fn compute_digest(&self) -> Sha256Digest {",
    )
    replace_exact(
        "codex-rs/hepta-supervisor/src/signed_intent.rs",
        '''    if let Some(existing) = read_intent(run_root)? {
        if matches!(
            existing.status,
            SignedIntentStatus::Prepared
                | SignedIntentStatus::Queued
                | SignedIntentStatus::RecoveryRequired
        ) && existing.grant_sha256 != intent.grant_sha256
        {
            return Err(SignedIntentError::Invalid(
                "another signed supervisor intent is unresolved".to_string(),
            ));
        }
    }''',
        '''    if let Some(existing) = read_intent(run_root)?
        && matches!(
            existing.status,
            SignedIntentStatus::Prepared
                | SignedIntentStatus::Queued
                | SignedIntentStatus::RecoveryRequired
        )
        && existing.grant_sha256 != intent.grant_sha256
    {
        return Err(SignedIntentError::Invalid(
            "another signed supervisor intent is unresolved".to_string(),
        ));
    }''',
    )
    replace_exact(
        "codex-rs/hepta-supervisor/src/supervisor.rs",
        "    pub fn apply_production_grant(\n",
        "    #[expect(\n"
        "        clippy::too_many_arguments,\n"
        "        reason = \"the production grant boundary keeps every authority and time witness explicit\"\n"
        "    )]\n"
        "    pub fn apply_production_grant(\n",
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
