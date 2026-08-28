# Hepta recovery order v1

Recovery is owner-ordered and fail closed. A later component must not infer authority from a stale
projection produced by an earlier generation.

1. **Supervisor registry and release identity**
   - Verify fleet root, release identity, registered Agent roots, and lifecycle generation.
   - Reject symlink/root drift and duplicate writer locks.
2. **Agentd authority and product graph**
   - Construct the closed-world `AuthorityGrant` for the exact Agent and spawn generation.
   - Construct and validate the real `ProductGraph` and single-writer map.
   - Reject any model/provider/effect/fleet/operator/promotion action in P0.
3. **Memory Runtime**
   - Open/migrate the Agent-private Cognitive SQLite store through the Memory facade.
   - Reopen and verify journals, leases, logical-turn and trajectory chains using existing store rules.
   - In the qualification cognitive-write profile, an unavailable store aborts startup.
   - In the normal read-only profile, unavailable/corrupt storage remains a typed degraded runtime.
4. **Read-only federation**
   - Refresh the Agent generation before discovery.
   - Discover owner stores read-only.
   - Refresh generation again before the federation set reaches App Server.
5. **Automation Runtime**
   - Open the Agent-private Automation SQLite store between generation checks.
   - Unavailable/corrupt automation disables only automation; it does not stop the App Server.
6. **Control and App Server**
   - Bind local control UDS.
   - Start App Server with required local thread store, exact home, and authority-derived feature states.
7. **Readiness and ingress**
   - Probe the exact App Server UDS and home identity.
   - Supervisor promotes the matching generation to Running.
   - Matrix/UI ingress may submit only after Running + App Server readiness.

Any generation change during steps 2–7 fences the process. Recovery never converts a queued,
accepted, source-present, or qualification-only record into effect success or production authority.
