# Hepta Browser Threat Model V1

Status: **normative for development and qualification; not production authority**.

## 1. Assets

The browser subsystem may handle authenticated web sessions, page content, user-entered text, downloads, uploads, local files, browser profile state, evidence receipts, semantic observations and automation commands. Raw credentials, cookies, authorization headers, private keys and complete browser profiles are high-sensitivity assets and MUST NOT cross the browser boundary as ordinary model or Agent data.

## 2. Trust boundaries

1. **Hepta caller → Browser Actor**: all requests are untrusted until schema, size, session, generation, owner epoch, page revision, request ID and canonical payload digest are validated.
2. **Browser Actor → Servo runtime**: Servo is a computation and rendering substrate, not an authority source. It cannot grant egress, credential, file, effect or promotion authority.
3. **Servo runtime → remote site**: page content, scripts, frames, service workers and downloads are hostile by default.
4. **Human ↔ shared page**: human takeover changes the mutation owner. Agent authority is fenced immediately and cannot resume from an old epoch.
5. **Browser → evidence/logging**: only bounded, redacted, digest-bound facts may leave the runtime. Raw secrets and unrestricted DOM dumps are forbidden.
6. **Build → runtime artifact**: source commit alone is insufficient. Target, toolchain, features, Servo source, patches, SBOM and artifact digests form the executable identity.

## 3. Adversaries

- A malicious or prompt-injected web page.
- A compromised Agent, tool, plugin or MCP server.
- Another process under the same local user account.
- A stale Agent process from an earlier generation.
- A malicious remote site, redirect chain, download or upload endpoint.
- A compromised native browser worker or dependency.
- A malicious relay/proxy/DNS response when external networking is later enabled.
- A local attacker able to replace sockets, profile files, receipts or binaries but not the independently pinned trust root.
- An operator error that reuses stale evidence or confuses qualification with promotion.

Root/administrator compromise is not fully preventable by this subsystem; it must be detected through artifact attestation, private file ownership, independent trust roots and post-event evidence.

## 4. Required controls

### 4.1 Session and concurrency

- Exactly one durable actor owns mutations for a session.
- Every request binds `session_id`, `generation`, `owner_epoch`, `page_revision`, `request_id` and canonical payload digest.
- Stale or unknown values fail closed; no best-effort rebinding.
- Request IDs are idempotent only when the payload digest is identical.
- Mutation queues, observations, nodes, text, evidence and cached replies are bounded.

### 4.2 Human takeover

- Human takeover increments the owner epoch and records a bounded lease.
- Agent mutation is denied for the entire active lease.
- Human lease expiry does not validate old Agent requests; callers must obtain the current epoch and revision.
- Teardown, crash or supervisor fencing invalidates both human and Agent mutation authority.

### 4.3 Semantic references

- A `SemanticRef` identifies a node only within one page revision.
- Navigation, DOM replacement, frame switch or equivalent semantic invalidation increments the revision.
- Stale refs are never silently re-resolved by role/name heuristics.
- Cross-origin frames remain separate authority and evidence domains.

### 4.4 Secrets and privacy

- Raw cookies, local/session storage, authorization headers, password values, private keys and profile databases are not exportable.
- Secret use must be process-bound and capability-scoped; future credential injection accepts opaque `SecretRef`, never raw bytes from an Agent.
- Screenshots, DOM fragments and accessibility text require bounded redaction policy and origin classification before evidence export.
- Logs contain IDs, digests, sizes, timings, denial classes and policy references—not prompts, credentials or unrestricted page content.

### 4.5 Network and files

- Development fixture runtime denies all external HTTP(S) navigation.
- Real Servo C1 starts with deny-by-default egress. Allowed destinations must bind origin, resolved address policy, redirect policy, method, upload/download limits and expiry.
- Loopback, link-local, private, metadata-service and Unix-socket targets require explicit policy; DNS rebinding and redirect changes are re-evaluated.
- Downloads land in an isolated, size-bounded staging directory and never execute automatically.
- Uploads require an explicit file capability; arbitrary path selection is forbidden.

### 4.6 Servo control surface

- Hepta must not expose Servo's wildcard WebDriver bind. C1 uses an authenticated private local transport or a loopback-only ephemeral listener protected by a bearer/capability boundary and OS ownership checks.
- Only an allowlisted typed subset is exposed; raw WebDriver passthrough and arbitrary JavaScript execution are not default capabilities.
- Servo preferences and extension routes are not caller-controlled unless separately contracted and receipt-bound.
- Servo crash, hang or protocol violation terminates the session and produces an indeterminate/failed-closed outcome; it does not trigger blind replay.

### 4.7 Supply chain

- Servo and all native dependencies are pinned by exact commit/artifact digest.
- MPL-2.0 source obligations, patch inventory, SBOM and notices are retained.
- A source update runs compatibility, security, malicious-site and platform qualification before replacing the prior pin.
- Production-capable build features are visible in a signed build manifest.

## 5. Abuse cases that must be tested

1. Page asks the Agent to reveal cookies, tokens, system prompts or local files.
2. Redirect moves an allowed origin to loopback, private address or metadata service.
3. DOM changes between observation and click.
4. Human takes control while an Agent mutation is queued.
5. Old process replays a valid-looking request after generation change.
6. Same request ID is reused with a different action.
7. Servo worker crashes after a command may have crossed the boundary.
8. Oversized page, recursive frames, event flood or endless navigation attempts exhaust resources.
9. Malicious download attempts execution, path traversal, symlink or hardlink escape.
10. Evidence/log output contains password fields, authorization headers or raw cookies.

## 6. Residual risks and release blockers

Current C0/C2/C3 implementation uses a deterministic fixture engine. It does not prove Servo behavior, network isolation, browser sandboxing, authenticated control transport, durable recovery, product integration or multi-platform performance. Those gaps remain release blockers until C1 and C4–C7 produce receipts bound to one exact source and artifact set.
