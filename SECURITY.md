# Hepta security policy

## Hepta security boundary

Hepta treats repository governance, typed capability issuance, Agent identity,
lifecycle generation, owner epochs, fencing tokens, SQLite integrity, outbox
recovery and candidate-bound qualification evidence as security boundaries.

The default and qualification profiles do not authorize production effects,
model/provider dispatch, fleet mutation, operator acceptance, promotion or
release.

## Reporting

Report suspected vulnerabilities privately through the repository owner's
GitHub security channel. Do not include credentials, private keys, access
tokens, raw secrets, personal data or working exploit payloads in public pull
requests, commit messages, logs or Actions artifacts.

A useful report includes the affected commit, component, trust boundary,
reproduction conditions, expected fail-closed behavior and the minimum
redacted evidence needed to verify the issue.

## Sensitive changes

Changes to these paths require explicit owner review:

- `.github/workflows/`
- `.github/CODEOWNERS`
- `docs/architecture/`
- `docs/governance/`
- `codex-rs/hepta-contracts/`
- `codex-rs/hepta-agentd/`
- `codex-rs/hepta-memory-runtime/`
- release, signing, authority, provider-effect and promotion code

Qualification success is not security approval. Live repository rulesets,
independent review, operator acceptance and release signing remain separate.
