# Hepta vNext main cutover

This tree is the only destination for new Hepta development. The legacy Hepta
source, common Git directory, linked worktrees, installed release, and live
state are frozen migration and rollback inputs. They must not receive product
features.

## Canonical integration base

- Upstream commit: `41ece455b7fa7166f4fc38522952afdaa2604e18`
- Qualification reference: `3110c5aba5daa0af1498b3eec85272011589ce8e`
- Product reference: `7f72ddd92a6ab5a411aac0634940998361e6cbb6`
- Frozen-product oracle: `2f704dc7c1172cefca908852456beccf4d02a5d1`
- Acceptance-tool reference: `00b2eac833a56afbbe9dc7219a23fb2083ac9281`

The integration history is built from the upstream commit. It does not rename
or promote any of the reference heads.

## Capability boundary

The unified tree includes:

- the qualified authorized-read and private qualification infrastructure from
  `3110c5...`;
- the real-caller governance, evidence, provider lifecycle, product-home, and
  `hepta` CLI stack from `7f72ddd...`;
- the required fail-closed and platform ratchets from that product line;
- file-level ports of the `2f704...` same-thread, digest-only Memory read path
  and governance/provider evidence read endpoints;
- the minimal legacy live shell needed to open schema-v5 state, run an
  isolated canary, and perform an atomic release cutover with rollback.

The unified tree deliberately excludes the `2f704...` Memory mutation surface,
channel facade, proof surface, and promotion framework because they have no
complete product caller. An unavailable writer or channel is never represented
as a successful zero-event result.

Production callers remain in Shadow mode. Enforce, promotion, outbound,
retirement, and automatic transition remain disabled.

## Evidence and backups

The independent recovery set is stored outside T5 at:

`/Users/qianqi/.openclaw/backups/hepta-vnext-logical-cutover-20260811T075708Z`

It contains 2,103 manifest entries. The SHA-256 of its `SHA256SUMS` file is:

`0200944345af18e958ed7d4c94c985a80cab327da6af49dde39dd2c2c14502db`

Exact source and migration heads are also retained under the private GitHub
`archive/` namespace. The signed acceptance assets for `3110c5...` are retained
in the private prerelease tagged
`hepta-vnext-acceptance-3110c5aba5-20260811`.

That acceptance applies only to the old frozen qualification receipt. It does
not qualify this integrated head. The final unified SHA must complete a new
Mac/Linux/Nix/GitHub/Windows qualification and receive its own operator
acceptance before any default-branch or live-runtime cutover.

## Retirement boundary

The legacy checkout and installed release remain available until all of these
conditions are true:

1. the unified head passes the full platform matrix;
2. the private bare repository, remote default branch, agent paths, and SSD
   worktrees point to the unified head;
3. Hepta UI is independently recoverable and no longer linked to the old
   common Git directory;
4. a full-root v2 snapshot covers every legacy top-level entry and preserves
   portable metadata, SQLite sidecars, keys, archive, and release-run payloads;
5. the candidate opens that receipt-bound copied state generation in an
   isolated loopback canary without writing production state;
6. cutover, rollback, then a fresh-evidence epoch-v2 rebase and recutover
   succeed without accepting a stale or forked chain; and
7. one complete development cycle finishes on the new tree.

Only then may the old linked worktrees, common Git directory, release, or state
be physically retired.
