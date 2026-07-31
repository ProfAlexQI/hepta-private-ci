use crate::gateway_options::NativeGatewayOptions;
use crate::http_transport::escape_html;
use crate::native_gateway::native_gateway_json;
use crate::native_telegram::NativeTelegramPluginStatus;

pub(crate) const NATIVE_GATEWAY_BINARY_ASSET_PATHS: &[&str] = &["/assets/hepta-agent-logo.png"];

pub(crate) fn route_native_gateway_binary_asset(
    method: &str,
    path: &str,
) -> Option<(&'static str, &'static str, &'static [u8])> {
    if method == "GET" && NATIVE_GATEWAY_BINARY_ASSET_PATHS.contains(&path) {
        Some((
            "200 OK",
            "image/png",
            hepta_core::control_ui::CONTROL_UI_HEPTA_AGENT_LOGO_PNG,
        ))
    } else {
        None
    }
}

pub(crate) fn index_html(
    options: &NativeGatewayOptions,
    telegram_plugin: &NativeTelegramPluginStatus,
) -> String {
    let readiness = native_gateway_json(options, telegram_plugin);
    let control_ui = hepta_core::control_ui_report();
    format!(
        r#"<!doctype html>
<html lang="en" data-runtime="hepta-native-gateway">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Hepta Control UI</title>
    <style>
      :root {{ color-scheme: dark; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; }}
      body {{ margin: 0; background: #101214; color: #f4f1ec; }}
      main {{ max-width: 880px; margin: 0 auto; padding: 32px 20px; }}
      h1 {{ font-size: 28px; margin: 0 0 10px; font-weight: 680; }}
      p {{ color: #c7c0b8; line-height: 1.55; }}
      .panel {{ border: 1px solid #34302b; border-radius: 8px; padding: 16px; background: #17191b; }}
      .grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 12px; margin: 18px 0; }}
      .metric {{ border: 1px solid #2d3135; border-radius: 8px; padding: 12px; background: #121517; }}
      .label {{ color: #9ca3aa; font-size: 12px; text-transform: uppercase; letter-spacing: 0; }}
      .value {{ margin-top: 6px; font-size: 16px; font-weight: 650; }}
      pre {{ overflow: auto; white-space: pre-wrap; border-radius: 8px; padding: 14px; background: #0b0d0f; border: 1px solid #2a2f34; }}
      code {{ font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size: 12px; }}
    </style>
  </head>
  <body>
    <main>
      <h1>Hepta Control UI</h1>
      <p>Native gateway entrypoint running from the Hepta release package. Codex remains contained as an internal engine-adapter compatibility layer.</p>
      <section class="grid" aria-label="gateway status">
        <div class="metric"><div class="label">Runtime</div><div class="value">hepta</div></div>
        <div class="metric"><div class="label">Gateway</div><div class="value">ready</div></div>
        <div class="metric"><div class="label">Telegram</div><div class="value">{telegram_status}</div></div>
        <div class="metric"><div class="label">Control UI evidence</div><div class="value">{control_ui_status} · static {control_ui_static_percent}% · live {control_ui_live_percent}%</div></div>
      </section>
      <section class="panel">
        <p><code>/api/hepta-merge-completion</code> exposes the current merge/function completion audit, route parity, and production-replacement closure without reading Telegram, sending messages, or performing native POST real mutations.</p>
        <p><code>/api/hepta-cli-command-inventory</code> exposes the old standalone Hepta CLI breadth inventory as a read-only migration map.</p>
        <p><code>/api/hepta-provider-metadata-inventory</code> narrows the provider/search bridge slice to metadata-only status without reading credentials or invoking providers.</p>
        <p><code>/api/hepta-runtime-session-dry-run-inventory</code> covers runtime-event, task, session, gateway, diagnostics, and admin ops as local dry-run migration plans without mutating registries or enqueuing gateway events.</p>
        <p><code>/api/hepta-channel-adapter-status-inventory</code> keeps Discord, Feishu, iMessage, Telegram, voice, webhook, and file-transfer adapters visible only as disabled/live-gated status entries.</p>
        <p><code>/api/hepta-local-tooling-content-inventory</code> maps canvas, diffs, filesystem, process, local content, search, readability, wiki, and tool invocation surfaces as local plans only.</p>
        <p><code>/api/hepta-systems-tool-registry-inventory</code> exposes built-in, MCP, dynamic, plugin, and connector tool registry metadata as a read-only report without invoking tools.</p>
        <p><code>/api/hepta-memory-capability-absorption-inventory</code> exposes memory, capability, plugin, coding-agent, search-provider, and skill-workshop gaps as read-only absorption status.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-runtime-readiness</code> binds the memory, Intelligence, and KG activation-readiness chain to a route-count-aware runtime surface without enabling live mutation or prompt/context execution.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-readiness</code> reports the operator-approved shadow context activation execution source surface without invoking providers, reading credentials, writing KG, or mutating live Memory.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled</code> exposes the controlled shadow execution gate/readback contract while keeping live route execution, provider/model calls, credential reads, KG writes, and Memory writes disabled.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence</code> reports that controlled readback receipts remain non-persistent and cannot become approval evidence, activation authority, public claims, or live writes.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-authority-denial</code> reports that controlled readback receipts cannot become trusted operator acceptance records, activation authority, activation commands, or public claims.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-separation</code> reports that controlled readback receipts cannot substitute, bind, refresh, replay, or materialize a trusted operator packet.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-materialization-lane</code> reports that explicit KG prompt payload shape materialization is now lane-authorized while report routes still cannot materialize payloads, read credentials, invoke KG adapters/providers/models, write KG, deliver channels, or claim public release.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-acceptance-receipt-lane</code> reports that explicit redacted KG prompt payload acceptance receipt shape is now lane-authorized while report routes still cannot record, persist, accept, materialize, expose raw payloads, write KG, invoke providers/models, deliver channels, or claim public release.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-readback-audit-receipt-lane</code> reports that explicit redacted KG prompt payload readback audit receipt shape is now lane-authorized while report routes still cannot render, record, persist, accept, materialize, expose raw payloads, write KG, invoke providers/models, deliver channels, or claim public release.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-acceptance-lane</code> reports that explicit redacted context handoff acceptance shape is now lane-authorized while report routes still cannot attach or inject context, record, persist, accept handoffs, write KG, invoke providers/models, deliver channels, or claim public release.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-receipt-audit-lane</code> reports that explicit redacted context handoff receipt audit shape is now lane-authorized while report routes still cannot attach or inject context, render, record, persist, accept audit receipts, write KG, invoke providers/models, deliver channels, or claim public release.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-precondition-lane</code> reports that bounded provider-router injection preconditions are now lane-authorized while report routes still cannot inject context, mutate provider prompts, invoke providers/models, write KG, deliver channels, or claim public release.</p>
        <p><code>/api/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-lane</code> reports that bounded provider-router injection dry-run envelope shape is now lane-authorized while report routes still cannot construct, render, record, persist, accept, or execute envelopes, inject context, mutate provider prompts, invoke providers/models, write KG, deliver channels, or claim public release.</p>
        <p><code>/api/hepta-release-hardening-status-gate</code> keeps remaining release, external-production, launchd, ops, and hardening script families visible as local-only status gates.</p>
        <p><code>/api/hepta-provider-channel-dry-run-plan</code> promotes provider, search, channel, and runtime/session gaps into deterministic dry-run plan contracts without credentials, external calls, delivery, or store mutation.</p>
        <p><code>/api/hepta-native-packaging-gate</code> tracks Hepta Native manifest, package metadata, app resources, and local smoke readiness before signing or public distribution.</p>
        <p><code>/api/hepta-legacy-compatibility-closure</code> closes the old Hepta CLI/script family gap as native read-only route/script coverage, without reenabling live execution.</p>
        <p><code>/api/hepta-public-ga-readiness</code> aggregates the public GA readiness blockers and explicit operator approvals without publishing, reading credentials, or invoking live channels.</p>
        <p><code>/api/hepta-core-fusion-readiness</code> reports Hepta as root runtime owner and Codex as an internal engine adapter while keeping remaining direct Codex base dependencies explicit.</p>
        <p><code>/api/hepta-name-repository-closure</code> inventories the remaining Phase 4 transition names that still block full fusion while keeping the active Hepta binary path intact.</p>
        <p><code>/api/hepta-engine-dependency-closure</code> inventories retained direct Codex engine dependencies that now define the Phase 5 full-fusion closure path.</p>
        <p><code>/api/hepta-engine-adapter-boundary</code> enumerates model, session, tool, sandbox, MCP, app-server, and legacy TUI/CLI adapter contracts before any Codex base dependency is removed; <code>/api/hepta-codex-engine-adapter-boundary</code> remains a compatibility alias.</p>
      </section>
      <section class="panel">
        <p>Readiness payload:</p>
        <pre><code>{readiness}</code></pre>
      </section>
    </main>
  </body>
</html>
"#,
        telegram_status = telegram_plugin.status.replace('_', " "),
        control_ui_status = control_ui.status,
        control_ui_static_percent = control_ui
            .evidence_coverage
            .static_contract
            .coverage_percent,
        control_ui_live_percent = control_ui.live_operator_surface_percent,
        readiness = escape_html(&readiness),
    )
}
