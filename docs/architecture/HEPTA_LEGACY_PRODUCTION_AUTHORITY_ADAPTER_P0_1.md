# Legacy production authority adapter P0.1

This tranche provides a compatibility bridge from the existing production
lease/verifier evidence shape to the unified typed authority kernel.

It does **not** mint authority from a serialized lease, status boolean, source
receipt, or qualification result. Adoption requires all of the following:

1. the current `AuthorityGrant` subject matches the lease owner Agent;
2. the grant generation matches the lease generation;
3. authority epoch, owner epoch, and generation are non-zero;
4. the evidence is not expired at the observation time;
5. capability ID and verifier ID are bounded and non-NUL;
6. an external `LegacyProductionAuthorityVerifier` accepts the exact
   digest-bound evidence;
7. the current grant independently authorizes `CognitiveWriteCapability`.

The result is only:

```text
VerifiedProductionCognitiveWrite
  └─ Authorized<CognitiveWriteCapability>
```

It does not contain or imply:

```text
ModelInvocationCapability
ProviderDispatchCapability
ExternalEffectCapability
FleetMutationCapability
OperatorAcceptanceCapability
ReleasePromotionCapability
```

## Exact evidence binding

The adapter digest covers:

- capability ID;
- owner Agent ID;
- authority epoch;
- owner epoch;
- generation;
- expiry;
- durable lease-head SHA-256;
- independent verifier-receipt SHA-256.

Changed evidence, stale generation, expiry, an invalid verifier identity, or a
verifier rejection fails closed before a typed witness exists.

## Honest integration boundary

The adapter source and tests are part of the new physical
`codex-hepta-memory-runtime` boundary. The existing
`AgentdProductionWriterHost` and its concrete external verifier are not yet
migrated to call this adapter. That caller migration remains a separate product
change because it must preserve the current durable lease and writer-lock
semantics without opening model, provider, external-effect, operator, or
promotion authority.
