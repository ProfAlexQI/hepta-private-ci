# Hepta Local Inference Threat Model V1

## Scope

This threat model covers the V4 owner-local inference daemon, public client API, private
worker and operator channels, external local-provider adapters, native model worker,
receipt store, scheduler/cache, and product shadow bridge.

## Assets

- raw prompts and generated text;
- request, session, worker, and operator capabilities;
- exact model/runtime/tokenizer/template identity;
- process and device resources;
- terminal receipts and audit provenance;
- Hepta Memory, KG, route, fleet, production, promotion, and release authority.

## Trust boundaries

1. product process to public daemon socket;
2. daemon to private worker process;
3. daemon/operator client to operator socket;
4. provider worker to loopback Ollama/LM Studio service;
5. native worker to llama.cpp runtime and GGUF files;
6. daemon to receipt journal/index;
7. product shadow bridge to authoritative product path;
8. CI source to assigned runner, model/device prerequisite, artifacts, and operator.

Same UID does not collapse these boundaries.

## Adversaries

- malicious or compromised same-UID local process;
- compromised plugin, MCP server, tool, or provider service;
- malformed or hostile model/runtime artifact;
- stale or restarted worker;
- resource-exhaustion client;
- CI job with missing runner or empty steps;
- author attempting to self-accept or overstate evidence;
- accidental document/workflow drift.

## Required controls

| Threat | Required control | Evidence |
|---|---|---|
| Role confusion | typed protocols, separate sockets, role capability | E0/E1 |
| Request hijack | daemon-minted unguessable capability bound to owner session | E0/E1 |
| Stale worker injection | worker-session nonce + backend/request generations + sequence | E0/E1 |
| Token/result forgery | rolling digest, count/byte/limit reconciliation | E0/E1/E3 |
| Connection/queue DoS | semaphores, I/O deadlines, distinct budgets | E0/E1 |
| Hung cancellation | ACK deadline, process-group kill, observed death | E1/E2/E3 |
| Receipt loss/replay | atomic durability, recovery, unique key, replay rejection | E0/E1 |
| Disk exhaustion | hard budget, TTL, compaction | E0/E1 |
| Raw data leakage | digest-only receipts/logs; bounded discarded child output | E0/E1 |
| Provider impersonation | loopback pinning, helper digest, service/model inventory | E2 |
| Mutable model substitution | exact tuple and artifact digests | E3 |
| Runtime compromise | isolated worker, minimal environment/authority | E1/E3 |
| Cache cross-contamination | exact digest key, tenant/policy fence, lease revocation | E0/E1 |
| Silent remote fallback | no remote endpoint and explicit fail-closed route | E0/E1/E2/E3 |
| Product influence during shadow | one-way non-authoritative bridge and kill switch | E5 |
| Evidence laundering | exact-head, non-empty assigned-runner receipts, no substitution | E0-E7 |
| Self-promotion | independent operator and separate activation authority | E6/E7 |

## Security invariants

- Public clients never send worker events or operator commands.
- Capability secrets and nonces never enter logs or durable receipts.
- Every active request and event has a complete generation/session/capability fence.
- Running cancellation completes only after ACK or observed process death.
- Worker restart invalidates all old sessions and rolls backend generation.
- Provider adapters never follow redirects, use system proxies, or pull models.
- Native model registration never trusts a mutable tag.
- No inference worker holds Memory/KG/production/promotion/release credentials.
- Required evidence jobs cannot be skipped and still produce a passing aggregate.

## Residual risks before module closed candidate

- unreviewed model behavior and prompt-level safety;
- kernel, driver, GPU, and native runtime vulnerabilities;
- same-account filesystem access outside the daemon's protected directories;
- performance variation across unsupported devices;
- provider cancellation semantics that may remain unsupported;
- supply-chain risk not covered by the bound SBOM/license/runtime provenance.

These risks must be explicitly accepted or mitigated in E6. They are not erased by source
qualification.
