# WEB-C1 offline Servo source-bundle verification

Status: **implemented tooling; canonical exact-source evidence still pending**  
Stage: `WEB-C1 / C1-004A-2..4`  
Authority: `source-verification-only`

The source generator is not trusted as its own verifier. A separate standard-library tool, `scripts/hepta-servo-source-bundle-verify.py`, accepts only the path-free bundle directory and the frozen Servo pin.

It verifies all canonical JSON bytes, the two acquisition receipts, distinct acquisition nonces, pinned HTTPS acquisition, independent object-store claims, the license packet, patch-inventory binding, compressed archive hash, tar hash and negative authority posture.

The verifier then decompresses exactly one gzip member. Optional gzip header fields, nonzero mtime, concatenated members, trailing data and truncation fail closed.

Every tar entry is inspected. Absolute paths, `..`, duplicate paths, hard links, device nodes, FIFOs, backslashes, unsafe file modes and symlink escape are rejected. Regular file and symlink content is hashed with Git's object framing. Directory trees are reconstructed recursively using Git tree ordering and modes. The recomputed root tree must be exactly:

```text
b04d2f75b3217374d079d579c270177b57fa1389
```

This makes archive verification independent of the acquisition checkout and of the generator's reported tree digest.

A successful verification receipt can prove only:

- two canonical source acquisition receipts were bound;
- one deterministic compressed source archive was verified;
- archive bytes reconstruct the pinned Git tree;
- MPL-2.0 LICENSE bytes and source-distribution obligation were bound;
- all browser/runtime/release authority remains false.

It cannot prove a Servo build, worker artifact, WebView, sandbox, listener posture, external-egress denial, product caller, operator acceptance or promotion.
