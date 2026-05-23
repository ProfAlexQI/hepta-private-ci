# Hepta Upstream Codex Product Governance Translation

This packet translates the selected upstream `product-doc-release-governance`
bucket into Hepta-owned release governance. It is not an upstream documentation
copy and it does not promote upstream runtime behavior into the active Hepta
service.

Source range:

- Baseline: `108234b5ebe6941764a6b8edbb37b2aa04369f07`
- Target: `7d47056ea42636271ac020b86347fbbef49490aa`
- Selected bucket: `product-doc-release-governance`
- Selected changed paths: `22`

## Hepta Translation Rules

1. Package and install-context changes from `Cargo.toml`, `Cargo.lock`, and
   `install-context` are release-governance inputs only. They do not change the
   active `/Users/qianqi/.local/opt/hepta/bin/hepta` service binary unless the
   Hepta native packaging, release-hardening, watchdog, and soak gates pass.
2. README, protocol, and tool documentation changes are translated into Hepta
   route/gate language. They must not copy upstream public-release wording or
   imply a public release claim.
3. Plugin installation and marketplace changes, including
   `request_plugin_install` and `list_available_plugins_to_install`, remain
   operator-approved policy surfaces before any live plugin mutation.
4. Sandbox, exec, network, and app-server documentation stays behind the
   P0 security/runtime review buckets before it can affect active runtime
   policy.
5. Release-facing statements require clean preflight, active dependency
   isolation, watchdog, operator approval packet, browser smoke, and long soak
   evidence.

## Absorbed Hepta Actions

- Package policy: track upstream package/install deltas as inputs to Hepta
  native packaging governance, not as automatic active binary changes.
- Documentation policy: rewrite README/protocol/tool guidance in Hepta terms
  and anchor it to `/api/hepta-*` routes and `scripts/hepta-*` gates.
- Plugin policy: keep plugin request and marketplace behavior behind explicit
  operator approval and no-side-effect dry-run contracts.
- Runtime policy: do not use product documentation to bypass P0 sandbox,
  provider, credential, app-server, or session review.
- Release policy: require the GA readiness packet, watchdog, visual smoke, and
  long soak before any external release claim.

## Non-Goals

- No upstream fetch, merge, checkout, or auto-rebase.
- No raw upstream document or package-policy copy.
- No active runtime code wiring.
- No active Codex engine dependency.
- No credential read, provider invocation, channel delivery, gateway mutation,
  or public release publication.

The repeatable gate for this packet is:

```bash
scripts/hepta-upstream-codex-product-governance-translation.sh
```
