# Hepta Browser threat model

**Status:** normative for `WEB-C0` through `WEB-C7`  
**Phase:** development/internal qualification  
**Default decision:** fail closed

## 1. Protected assets

The browser subsystem must protect:

- session and generation identity;
- human/Agent control ownership;
- live DOM, JavaScript heap, history, storage, cookies, credentials, downloads, and profile;
- semantic snapshots and references;
- TaskFlow effect and reconciliation authority;
- Agent/workspace/tenant isolation;
- receipts, evidence digests, policy digests, and audit order;
- host filesystem, loopback services, cloud metadata endpoints, and private network targets;
- operator-controlled login and challenge state.

## 2. Trust boundaries

### 2.1 Untrusted

- all web content, scripts, frames, workers, service workers, redirects, downloads, and WebSockets;
- model-generated browser commands;
- semantic text extracted from a page;
- remote DNS and server responses;
- public relay, proxy, and certificate inputs not pinned by policy;
- stale semantic refs, cached requests, and replayed receipts.

### 2.2 Conditionally trusted

- the Hepta planner only after policy and command validation;
- a local human only for the bounded session/lease actually granted;
- a Servo adapter only for facts it returns through the bounded contract;
- the fixture engine only as deterministic qualification code, never as production evidence;
- the local OS account only to the extent stated by the deployment profile.

### 2.3 Trusted but fallible

- BrowserActor state machine;
- agentd/fleet generation and process fence;
- TaskFlow effect/reconcile records;
- receipt canonicalization and hashing;
- build and qualification infrastructure.

Compromise or malfunction of a trusted component must still be detectable through exact-source,
artifact, feature-profile, and receipt bindings.

## 3. Attacker classes

| Attacker | Capabilities | Required defense |
|---|---|---|
| Malicious model output | arbitrary semantic commands and text | allowlist, bounds, authority separation, no raw eval |
| Malicious page | arbitrary JS/DOM/events/redirects | sandbox, navigation policy, semantic projection, challenge outcomes |
| Stale Agent process | old generation/session/epoch | generation, session, owner-epoch and revision fences |
| Same-UID malicious process | local socket and file attempts | owner-only transport plus peer/process identity; no unauthenticated TCP |
| Malicious remote site | SSRF, rebinding, exfiltration | C5 network broker, IP-class policy, redirect revalidation, DNS pinning |
| Compromised native worker/engine | malformed snapshots/results | structural bounds, privacy markers, process isolation, crash containment |
| Malicious human input | unexpected mutation during Agent work | bounded takeover lease, ordered actor, ref invalidation, receipts |
| Supply-chain attacker | modified Servo/native dependency | immutable pin, SBOM, patch inventory, signed build provenance |
| Evidence attacker | replay/fork/modify receipt | canonical bytes, domain-separated digests, exact commit/tree binding |

## 4. Core invariants

### TM-01 One live owner

One session has one BrowserActor and one live engine page owner. A second CDP/WebDriver page,
headless mirror, synthetic DOM, or migration target cannot be used as evidence for the session.

### TM-02 Authority separation

Browser results are observations. They cannot mint capability, approval, operator acceptance,
promotion, payment, secret, or external-effect authority.

### TM-03 Ordered mutations

Agent and human mutations enter the same ordered actor. Human takeover advances owner epoch.
Every DOM-affecting mutation advances page revision and invalidates published refs.

### TM-04 Idempotency

A request ID is bound to canonical request bytes. Exact replay returns the sealed prior response.
The same ID with different bytes is denied as a conflict and performs no mutation.

### TM-05 Sensitive-data suppression

Cookies, passwords, tokens, authorization headers, raw profile data, and secret-bearing form values
must not be returned through semantic extract. Engine adapters must mark sensitive or cross-tenant
content before projection; the actor denies it and emits no state evidence receipt.

### TM-06 External effects disabled by default

The development fixture accepts only `fixture://` URLs. Real HTTP(S) navigation belongs to C4/C5
read-only qualification. Login, upload, download, form submission with external effect, purchase,
message send, and account mutation remain C7-only.

### TM-07 Bounded resources

Requests, semantic nodes, snapshot fields, extracted output, human leases, response cache, network
activity, download size, process memory, CPU, GPU, and wall-clock duration need explicit limits.
Exceeding a limit is a typed denial, never implicit truncation of an authority-relevant field.

### TM-08 No public control listener

The current upstream Servo WebDriver implementation opens `0.0.0.0`. Hepta must not use this
entry point. The production design requires in-process handler calls or an authenticated local
transport tied to the session and process generation.

## 5. Required C5 network defenses

C5 cannot pass until all request-producing paths enforce:

- scheme allowlist;
- hostname normalization and IDNA policy;
- DNS answer pinning for the request lifetime;
- private, loopback, link-local, multicast, metadata, and configured sensitive CIDR denial;
- redirect target revalidation on every hop;
- proxy and environment-variable isolation;
- iframe, worker, service-worker, WebSocket, WebTransport, prefetch, beacon, and speculative-load
  policy;
- bounded request/response bytes and duration;
- download quarantine and content digest;
- certificate and HSTS behavior recording;
- no automatic fallback to another browser or remote fetch backend.

## 6. Required native UI defenses

C6 must prove:

- human input reaches the same WebView as Agent semantic operations;
- focus, IME, clipboard, drag/drop, file picker, permission prompt, and window visibility are
  governed by policy;
- clipboard and file picker are disabled by default;
- show/hide cannot clone or replace the page;
- takeover and release are visible in receipts;
- stale events from a prior generation are ignored.

## 7. Incident and recovery behavior

On engine crash, renderer failure, unknown mutation outcome, generation fence, storage corruption,
or policy mismatch:

1. stop accepting mutations;
2. return `Challenge` or `Indeterminate`, not success;
3. preserve the last sealed receipts;
4. do not blindly retry a possibly applied external mutation;
5. terminate or quarantine the process;
6. require a fresh session/generation and policy binding;
7. reconcile through TaskFlow when external effect may have crossed the boundary.

## 8. Residual risk during the current tranche

The current code is a deterministic qualification fixture. It does not yet defend a real network,
real Servo process, native window, real cookie store, downloads, cross-origin frames, browser
extensions, or native libraries. Those risks remain open and block C1/C4/C5/C6/C7.
