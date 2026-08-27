# Hepta tree hygiene snapshot hepta-tree-hygiene-20260827-r25

- Canonical candidate: `4d5b0b2d082ddbe0abb6d2fc880d9d9448434ab2` (tree `a3988c2d4624437080a2cdb65185ce74e7cee488`).
- This is an immutable pre-cleanup registry snapshot. Dirty WIP is recorded only; no dirty path was modified.
- Worktree records: 138 total / 137 non-bare; dirty 14; detached 2 (clean detached 2).
- `registry.json` contains every worktree status and branch/ref; `dirty-wip.json` is the explicit dirty index; `refs.txt` and `worktree-list.porcelain` are raw inputs.
- No branch refs are deleted and no merge is performed by this snapshot.
- Post-snapshot cleanup is recorded in `CLEANUP-DELTA.json`: six clean redundant directories were removed only after archive refs were created; the resulting registry is 132 total / 131 non-bare / 14 dirty / 1 detached. Dirty WIP and the fresh `e41-capture-4d5b0b2-20260827` capture remain.
