# WEB-C1 exact Servo source acceptance policy

A workflow success is necessary but not sufficient to change the canonical source status.

## Required evidence set

One candidate Hepta commit/tree must produce:

1. `fetch-a.receipt.json` and `fetch-b.receipt.json` from separate Git roots and object stores;
2. a deterministic `servo-source-a.tar.gz` whose duplicate build from fetch B is byte-identical;
3. `license-packet.json` binding MPL-2.0 bytes and the canonical patch inventory;
4. `independent-source-bundle.receipt.json`;
5. `source-bundle.verification.json` from the separate offline verifier;
6. workflow logs and artifact metadata binding the exact Hepta commit/tree;
7. SHA256SUMS for all retained evidence.

The offline verification receipt must recompute the Git root tree from archive content and produce:

```text
recomputed_tree = b04d2f75b3217374d079d579c270177b57fa1389
```

## Review rules

Acceptance requires a separate reviewed pointer update. The pointer must bind the workflow run, Hepta commit/tree, all receipt digests and retained source archive digest. A workflow may not modify the pointer itself.

Review must confirm:

- both fetch receipts are canonical and use the pinned HTTPS origin;
- acquisition nonces differ;
- source projections match;
- tar/gzip digests match across both acquisitions;
- offline tree reconstruction matches the pin;
- no machine-local paths appear;
- MPL source distribution is retained;
- no build/runtime/release authority is enabled.

## Rejection conditions

The source candidate is rejected on any of:

- unknown or changed commit/tree/origin;
- shared Git alternate object store;
- dirty/untracked checkout;
- nondeterministic archive bytes;
- unsafe path, mode, link or gzip structure;
- license/patch binding drift;
- missing logs or artifact retention;
- workflow running from a different Hepta candidate;
- any attempt to describe source acceptance as a Servo build or runtime qualification.

## Result boundary

An accepted source pointer permits only C1-004B input preparation. It does not permit a production caller, external browser network, operator acceptance, promotion or release.
