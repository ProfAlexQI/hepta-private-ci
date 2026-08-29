# Hepta qualification execution policy v1

This document records the delivery rule for the architecture-convergence branch.

1. The canonical qualification workflow is read-only. It may not commit, push, update refs, or widen runtime authority.
2. Source-head and merge-candidate evidence are distinct identities. Both must execute with non-empty job and step records before the pull request can be considered qualified.
3. A head produced with the repository `GITHUB_TOKEN` is materialized source only. It is not executable qualification evidence when GitHub records the follow-on pull-request workflows as `action_required`, queued without a runner, or empty.
4. After a temporary write lane materializes deterministic source and retires itself, an owner-authored commit must establish the final immutable qualification head. No source-mutating workflow may remain on that head.
5. Operator acceptance, promotion, release, production caller/writer authority, provider dispatch, model invocation, external effects, and fleet mutation remain independently issued gates and are never implied by CI success.

The required hosted context remains `Hepta architecture convergence required`.
