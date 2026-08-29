# Hepta physical capability boundary matrix V1

**Status:** normative P0.7b contract. All rows default to denied.

A typed grant or a start-time runtime bootstrap is insufficient by itself. The
component crossing a physical boundary must obtain a short-lived,
operation-bound verified-use token immediately before the crossing. The token
is non-serializable, carries the current authority context digest and cannot be
reused for another operation.

| Boundary | Typed capability | Immediate verifier facts | Unknown outcome | Local profiles |
|---|---|---|---|---|
| model request submission | `ModelInvocationCapability` | Agent, release, profile, model artifact, policy, epochs, generation, fence, expiry, revocation | no blind retry; query provider operation when available | denied |
| provider dispatch | `ProviderDispatchCapability` plus `ExternalEffectCapability` where effectful | request digest, provider namespace, operation key, epochs, generation, fence, expiry, revocation | lookup-only reconcile | denied |
| tool process spawn | tool-class capability plus sandbox policy | command/tool digest, cwd, environment policy, approval, epochs, generation, fence | process observation and explicit terminal receipt | denied unless locally approved non-external tool class |
| outbound network connect | network-destination capability | destination, protocol, DNS/IP binding, policy, epochs, generation, fence, expiry | connection observation; never infer remote effect | denied |
| filesystem mutation outside Agent root | external filesystem capability | canonical target, mount/device identity, no-follow policy, operation digest, fence | inspect target; do not replay blindly | denied |
| secret read/refresh/rotate | opaque SecretRef capability | provider/profile/token family, purpose, audience, expected revision, deadline, fence | status lookup by operation key | denied except process-bound approved adapter |
| Matrix send | provider/effect capability | room, event, payload digest, operation journal, current fence | Matrix event lookup/reconcile | denied |
| fleet lifecycle mutation | `FleetMutationCapability` | fleet record revision, release, owner epoch, generation, signer and expiry | registry CAS inspection | denied |
| operator acceptance | `OperatorAcceptanceCapability` | exact candidate, evidence manifest, reviewer identity and signature | remain unaccepted | denied |
| release promotion | `ReleasePromotionCapability` | exact accepted candidate, release manifest, SBOM, rollback evidence | remain unpromoted | denied |

## Cross-cutting invariants

1. Verification occurs after the final operation payload is known and before the
   irreversible boundary.
2. A verified-use token binds one operation ID and one payload digest.
3. Revocation, epoch advancement, generation drift, fence drift or expiry
   invalidates an unused token.
4. Queue acknowledgement is not terminal effect success.
5. `Indeterminate` remains open until a current-fence reconciler commits a
   terminal decision.
6. Qualification mocks cannot be linked into a production artifact.
7. No adapter may mint the capability it consumes.
