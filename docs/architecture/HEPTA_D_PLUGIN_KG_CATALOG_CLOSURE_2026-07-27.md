# Hepta D Plugin, KG, and Catalog Closure

- MCP catalog publication rebuilds an unpublished manager, revalidates auth/config, and atomically publishes generation + 1 before retiring the prior manager (`9d726f145e`).
- Plugin share save, update-targets, checkout, and delete now bind the exact App Server request, workspace, target, payload, effect plan, provider ACK, and terminal receipt (`80926b91e5`).
- Plugin mutation state is persisted under a sibling lock with private atomic publication; exact successful replays return the original response, while planned/committing ambiguity fails closed.
- KG recall now crosses a named local read-only adapter boundary (`4569fdb930`).
- The KG adapter never reads credentials, performs network calls, writes external state, or enables live KG writes.
- D does not deploy, restart services, enable Telegram/model execution, or mutate the active release artifact.

Validation: plugin mutation journal 2/2, App Server plugin-share protocol 13/13, KG adapter 1/1, Intelligence recall 3/3, changed-package Rust 1.95 Clippy, formatting, Architecture V2, architecture budget verify/self-test, and active-service dependency isolation.

Package-wide Clippy outside the changed packages remains blocked only by pre-existing audited diagnostics in `hepta-core`; no D change adds a diagnostic there.
