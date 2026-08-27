# Hepta Browser active plan — repository-native WEB-E successor

**Plan ID:** `WEB-PLAN-2026-08-27E`  
**Status:** `DEVELOPMENT / IMPLEMENTATION_IN_REVIEW / QUALIFICATION_ONLY`  
**Active stages:** `WEB-C0` through `WEB-C7` only  
**Supersedes:** the active-input role of the Dropbox `WEB-D` snapshot while retaining it as
historical provenance  
**Does not authorize:** production caller, production writer, effect authority, external web
mutation, login, operator acceptance, G5, promotion, or release

## 1. Why this successor exists

The Dropbox snapshot preserved the latest `WEB-D` decision, but several detailed browser files
were cloud-only at capture time and the snapshot explicitly had no machine authority. This
successor makes the plan reproducible from Git, gives it a dependency-free verifier, binds every
active requirement to code and CI, and prevents historical A/B/C/D text from becoming an
accidental implementation input.

This successor does not reverse the D decision. It carries forward the following non-negotiable
constraints:

- Servo is the only live browser engine.
- One session owns one live page.
- Human UI and Agent API share that page.
- Obscura may contribute semantic vocabulary and test ideas but no second DOM/JS/layout/network
  runtime enters the process.
- Raw WebDriver, CDP, unrestricted JavaScript evaluation, cookies, passwords, tokens, and profile
  files are not model-facing.
- TaskFlow remains effect/retry/reconcile authority.
- agentd/fleet remains process generation and fencing authority.
- web content is untrusted observation, never authority.

## 2. Document precedence

Active implementation and CI read only these files:

1. `HEPTA_VNEXT_CURRENT.yaml`
2. this file
3. `HEPTA_BROWSER_IMPLEMENTATION_PLAN_2026-08-27.md`
4. `HEPTA_BROWSER_STAGE_MATRIX_v1_4.yaml`
5. `HEPTA_BROWSER_TRACEABILITY_v1.yaml`
6. `HEPTA_BROWSER_THREAT_MODEL.md`
7. `hepta.browser_receipt.v2.schema.json`
8. exact Git source and CI evidence

The following are provenance-only:

- `dropbox-current-2026-08-27/root/hepta-vnext-development-plan-final-2026-08-23.md`
- historical browser A/B/C/D appendices and archived attachments
- any receipt that is not bound to the exact current implementation commit and tree

## 3. Upstream Servo research pin

The C1 research candidate is the immutable Servo commit:

```text
repository: servo/servo
commit: 0a48e298482659817eb50097df23841f2b8e3044
tree: b04d2f75b3217374d079d579c270177b57fa1389
observed_at_utc: 2026-08-27T07:23:21Z
license: MPL-2.0
status: RESEARCH_PIN_NOT_IMPORTED
```

This is not yet a Hepta source import or accepted dependency. Before C1 can begin, a source
receipt must bind the full submodule/vendor strategy, Rust toolchain, native dependencies,
license/SBOM inventory, patch queue, and reproducible build commands.

The upstream WebDriver server at this pin binds `0.0.0.0` and starts an HTTP listener. Hepta MUST
NOT expose that entry point. C1 must either remove the HTTP listener and call the handler inside
the process, or replace it with an authenticated owner-only local transport. Loopback TCP without
peer authentication is not sufficient for the final design.

## 4. Current implementation status

| Stage | Status | Claim |
|---|---|---|
| `WEB-C0` | `IMPLEMENTED_PENDING_CI` | repository-native plan, threat model, schema, matrix, traceability |
| `WEB-C1` | `NOT_STARTED` | Servo is not linked or built by this change |
| `WEB-C2` | `IMPLEMENTED_PENDING_CI` | canonical semantic contract and deterministic fixture |
| `WEB-C3` | `FIXTURE_IMPLEMENTED_PENDING_CI` | actor/epoch/revision/human lease/idempotency/receipt fixture only |
| `WEB-C4` | `NOT_STARTED` | no public-site corpus |
| `WEB-C5` | `NOT_STARTED` | no real network sandbox/SSRF qualification |
| `WEB-C6` | `NOT_STARTED` | no native window/IME/clipboard platform qualification |
| `WEB-C7` | `BLOCKED_FINAL_RELEASE` | authentication and external effects remain disabled |

The integrated C3 gate remains blocked by C1. Passing fixture tests cannot be relabeled as a
headed Servo qualification.

## 5. First executable vertical slice

The first vertical slice is fixed:

```text
Agent navigate fixture
→ Agent observe and receive semantic refs
→ Human obtains a bounded control lease
→ Human types and clicks through the same BrowserActor
→ Agent observes the changed values from the same page owner
→ Agent extracts an allowlisted non-secret value
→ ActivityReceipt + WebEvidenceReceipt
```

Required negative cases:

```text
wrong session
stale generation
stale owner epoch
stale page revision
stale semantic ref
request-id payload conflict
human lease expiry
external URL in fixture profile
raw cookie extraction
oversized command/snapshot
cross-tenant marker
```

## 6. Promotion boundary

No stage can set a production authority flag merely because its tests pass. The minimum future
promotion chain is:

```text
exact source/tree
→ reproducible Servo build
→ one-WebView headed smoke
→ semantic/arbiter qualification
→ C5 security matrix
→ C6 platform matrix
→ C7 AuthBus/profile/effect gates
→ artifact digest and SBOM
→ independent operator acceptance
→ promotion receipt
```

Until that chain exists, every authority field remains closed.
