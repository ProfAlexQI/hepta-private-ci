use serde::Serialize;

/// Canonical Control UI document. Rust embeds and serves this exact snapshot;
/// there is no second renderer-owned HTML body. A bounded same-origin script
/// progressively enhances read-only inspection while anchors remain usable
/// without JavaScript.
pub const CONTROL_UI_INDEX_HTML: &str = include_str!("../../../apps/hepta-control-ui/index.html");
pub const CONTROL_UI_STYLES_CSS: &str = concat!(
    include_str!("../../../apps/hepta-control-ui/light-glass-tokens.generated.css"),
    "\n",
    include_str!("../../../apps/hepta-control-ui/styles.legacy.css"),
    "\n",
    include_str!("../../../apps/hepta-control-ui/styles.foundation.css"),
    "\n",
    include_str!("../../../apps/hepta-control-ui/styles.components.css"),
    "\n",
    include_str!("../../../apps/hepta-control-ui/styles.responsive.css"),
    "\n",
    include_str!("../../../apps/hepta-control-ui/styles.accessibility.css"),
);
const CONTROL_UI_UNIFIED_LANE_STYLES_CSS_BUDGET_BYTES: usize = 300_000;
/// Compatibility-only serialized security contract. The typed
/// `control_ui_interaction_capabilities` manifest is authoritative for visible
/// browser behavior; callers must not infer UI capability from substring checks.
pub const CONTROL_UI_RUST_RENDERER_MARKERS: &str = r###"{
  "schema_version": 2,
  "kind": "control-ui-local-security-contract",
  "live_adapter_bound": false,
  "contracts": [
    {
      "id": "policy-approval-readonly-inventory",
      "availability": "same-origin-read-only",
      "evidence": ["renderApprovalCards", "/api/approvals", "/api/policy"]
    },
    {
      "id": "runtime-operator-plan-catalog",
      "availability": "catalog-only",
      "enabled": false,
      "evidence": [
        "POST /api/runtime/operator",
        "Confirm-gated runtime kill/steer dry-run evidence"
      ]
    },
    {
      "id": "event-visibility-readonly-inventory",
      "availability": "same-origin-read-only",
      "evidence": [
        "renderEventTimeline",
        "/api/events-report",
        "/api/live-events/0",
        "duplicate_free"
      ]
    }
  ]
}"###;
include!("control_ui_static_assets.rs");
pub const CONTROL_UI_README: &str = include_str!("../../../apps/hepta-control-ui/README.md");
pub const CONTROL_UI_SMOKE_SH: &str = include_str!("../../../scripts/hepta-control-ui-smoke.sh");
pub const CONTROL_UI_BROWSER_SMOKE_SH: &str =
    include_str!("../../../scripts/hepta-browser-visual-smoke.sh");
pub const CONTROL_UI_BROWSER_RUNNER_SMOKE_SH: &str =
    include_str!("../../../scripts/hepta-control-ui-browser-smoke.sh");
pub const CONTROL_UI_FUNCTIONAL_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: functional-smoke
Hepta Control UI functional smoke passed
10k message search exceeded budget
state pruning exceeded budget
chat search should keep focus after rerender
thread search should keep focus after rerender
mapped command should replace task placeholder"###;
pub const CONTROL_UI_QUALITY_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: quality-smoke
Hepta Control UI quality smoke passed
styles.css budget exceeded
README budget exceeded
HEPTA_CHAT_BOUNDARY"###;
pub const CONTROL_UI_BROWSER_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: browser-smoke
Hepta Control UI browser smoke passed
playwright
desktop
narrow
mobile
screenshot
data-telegram-multi-agent-chat
data-ui-convergence-ledger
chat search should retain focus in browser"###;
pub const CONTROL_UI_RELEASE_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: release-smoke
Hepta Control UI release smoke passed
browser screenshot manifest
desktop should show Workspace Room
mobile should hide Workspace Room"###;
pub const CONTROL_UI_MATURITY_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: maturity-smoke
Hepta Control UI maturity smoke passed
applyContentRichFixture
content-rich visual regression
rich-desktop
.tg-message
data-chat-mobile-pane
mobile pane should be visible
rich-mobile-room
header should keep primary status compact
.tg-thread-status-primary
empty room should collapse
applyEmptyFixture
data-room-task-artifact-insert
data-room-task-action-plan
data-room-task-action-confirm
data-endpoint-retry-all
dry-run review checkbox should be interactive
module boundary missing
assertA11y
visible buttons should have text or aria labels
command palette should expose dialog semantics
mobile layer switcher should be a tablist
exactly one mobile layer tab should be selected
reduced motion rule should be present"###;
pub const CONTROL_UI_HARDENING_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: hardening-smoke
Hepta Control UI hardening smoke passed
visual diff exceeded baseline
visual diff baseline
compareRect
room accordion compression
closedRoomAccordions
roomScrollRatio
mobile compact composer too tall
composerHeight <= 150
mobileThread
first render budget exceeded
command palette latency budget exceeded
renderMs < 1_800
paletteLatencyMs < 250
performance.now
manifest.results
hepta-control-ui-hardening-smoke
waitUntil: "domcontentloaded"
assertKeyboardAndA11y
focus trap should move keyboard focus
keyboard-only tab journey
contrast ratio too low
contrastRatio
Control+K
aria-modal
real module split should export module registry
modules.controlUiModules
applyEndpointChaosFixture
endpoint chaos should expose per-endpoint retries
partial failure detail"###;
pub const CONTROL_UI_BUILD_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: build-smoke
Hepta Control UI build split smoke passed
bundle_mode
static-app-plus-esm-build-registry"###;
pub const CONTROL_UI_SMOKE_CONTRACT_MJS: &str = r###"Rust-native retired Node smoke marker: smoke-contract
Hepta Control UI contract-suite smoke passed
/ui-contract-audit"###;
pub const CONTROL_UI_SMOKE_MARKER_MJS: &str = r###"Rust-native retired Node smoke marker: smoke-marker
Hepta Control UI marker-suite smoke passed"###;
pub const CONTROL_UI_CROSS_BROWSER_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: cross-browser-smoke
Hepta Control UI cross-browser smoke passed
chromium-system
firefox-managed
webkit-managed
cross-browser matrix should enumerate
status: "skipped"
data-room-accordion
data-chat-mobile-active-pane"###;
pub const CONTROL_UI_CROSS_BROWSER_STRICT_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: cross-browser-strict-smoke
Hepta Control UI strict cross-browser smoke passed
chromium-system
firefox-managed
webkit-managed
classified-with-install-hints-never-silent
required_runtime_passes
skip_policy
failed"###;
pub const CONTROL_UI_PERCEPTUAL_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: perceptual-smoke
Hepta Control UI perceptual smoke passed
bmpAverageHash
hamming
sips
perceptual diff exceeded threshold"###;
pub const CONTROL_UI_SCHEMA_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: schema-smoke
Hepta Control UI schema smoke passed
control-ui.schema.json
ui-contract-audit.schema.json"###;
pub const CONTROL_UI_SOAK_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: soak-smoke
Hepta Control UI soak smoke passed
localStorageBytes
nodeCount
typing preservation should survive soak loops
applyEndpointChaosFixture
DOM node soak budget exceeded
localStorage soak budget exceeded
manifest.json"###;
pub const CONTROL_UI_A11Y_SNAPSHOT_MJS: &str = r###"Rust-native retired Node smoke marker: a11y-snapshot
Hepta Control UI accessibility snapshot passed
heading hierarchy snapshot
all visible controls should have accessible names
mobile layer tablist should expose one selected tab
Control+K
data-room-task-artifact-insert
snapshot.json"###;
pub const CONTROL_UI_PRODUCTIZATION_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: productization-smoke
true module extraction
productizationPolicies.gates.length === 10
mobile density should cap visible status chips
old centered starter float should stay removed
selector budget exceeded
important budget exceeded
selectorCount
importantCount"###;
pub const CONTROL_UI_GOLDEN_FIXTURE_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: golden-fixture-smoke
Hepta Control UI golden fixture smoke passed
Golden evidence preview
data-task-result-drawer"###;
pub const CONTROL_UI_A11Y_DEEP_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: a11y-deep-smoke
Hepta Control UI deep accessibility smoke passed
semanticTree
focusRoute
heading hierarchy should not skip levels
keyboard route map
reducedMotion
landmarks
focusables"###;
pub const CONTROL_UI_HOSTILE_FIXTURE_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: hostile-fixture-smoke
Hepta Control UI hostile fixture smoke passed
hostile fixture should not execute script payloads
escaped text
javascript: links"###;
pub const CONTROL_UI_PRODUCT_DRAWER_SMOKE_MJS: &str = r###"Rust-native retired Node smoke marker: product-drawer-smoke
Hepta Control UI product drawer smoke passed
copy
pin
trace
next-step"###;
pub const CONTROL_UI_SMOKE_SUMMARY_MJS: &str = r###"Rust-native retired Node smoke marker: smoke-summary
Hepta Control UI smoke summary passed
suite_count
p0_p39_converged
cross-browser-strict
product-drawer
target/hepta-control-ui-smoke-summary
suite output
audit_percent"###;
pub const CONTROL_UI_P0_P6_RELEASE_DOC: &str =
    include_str!("../../../docs/release/HEPTA_CONTROL_UI_P0_P6_CONVERGENCE_2026-04-27.md");
pub const CONTROL_UI_P0_P13_MATURITY_DOC: &str =
    include_str!("../../../docs/release/HEPTA_CONTROL_UI_P0_P13_MATURITY_2026-04-27.md");
pub const CONTROL_UI_P0_P21_HARDENING_DOC: &str =
    include_str!("../../../docs/release/HEPTA_CONTROL_UI_P0_P21_HARDENING_2026-04-27.md");
pub const CONTROL_UI_P0_P29_ENGINEERING_DOC: &str =
    include_str!("../../../docs/release/HEPTA_CONTROL_UI_P0_P29_ENGINEERING_2026-04-27.md");
pub const CONTROL_UI_P0_P39_PRODUCTIZATION_DOC: &str =
    include_str!("../../../docs/release/HEPTA_CONTROL_UI_P0_P39_PRODUCTIZATION_2026-04-28.md");
pub const CONTROL_UI_MODULE_BOUNDARIES_README: &str =
    include_str!("../../../apps/hepta-control-ui/modules/README.md");
pub const CONTROL_UI_VISUAL_BASELINE_JSON: &str =
    include_str!("../../../apps/hepta-control-ui/baselines/visual-layout-baseline.json");
pub const CONTROL_UI_PERCEPTUAL_BASELINE_JSON: &str =
    include_str!("../../../apps/hepta-control-ui/baselines/perceptual-baseline.json");
pub const CONTROL_UI_MODULE_INDEX_JS: &str = "rust renderer module registry: controlUiModules chat-state workspace-room browser-fixtures accessibility execApprovalsModule task-actions live-data productizationModule window.__HEPTA_UI_MODULE_REGISTRY";
pub const CONTROL_UI_MODULE_EXEC_APPROVALS_JS: &str =
    "rust renderer exec approvals bridge: /api/approvals/exec/apply previewExecApprovalPatch";
pub const CONTROL_UI_MODULE_PRODUCTIZATION_JS: &str = "rust renderer productizationModule productizationPolicies P30-P39 product gates result drawer action model visibleChipBudget desktopEmptyThread noCenteredStarterFloat selectorBudget importantBudget hostileFixtures";
pub const CONTROL_UI_GOLDEN_FIXTURE_JSON: &str =
    include_str!("../../../apps/hepta-control-ui/fixtures/golden-live-data.json");
pub const CONTROL_UI_HOSTILE_FIXTURE_JSON: &str =
    include_str!("../../../apps/hepta-control-ui/fixtures/hostile-xss.json");
pub const CONTROL_UI_SCHEMA_CONTROL_JSON: &str =
    include_str!("../../../apps/hepta-control-ui/schemas/control-ui.schema.json");
pub const CONTROL_UI_SCHEMA_AUDIT_JSON: &str =
    include_str!("../../../apps/hepta-control-ui/schemas/ui-contract-audit.schema.json");
pub const CONTROL_UI_SCHEMA_CHAT_JSON: &str =
    include_str!("../../../apps/hepta-control-ui/schemas/chat-api.schema.json");
pub const CONTROL_UI_SCHEMA_TASK_JSON: &str =
    include_str!("../../../apps/hepta-control-ui/schemas/task-api.schema.json");
pub const CONTROL_UI_SCHEMA_EVENTS_JSON: &str =
    include_str!("../../../apps/hepta-control-ui/schemas/events-report.schema.json");

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlUiScreen {
    pub id: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub route: &'static str,
    pub implemented: bool,
    pub implementation_scope: &'static str,
    pub live_adapter_ready: bool,
    pub data_sources: &'static [&'static str],
    pub widgets: &'static [&'static str],
    pub interactions: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlUiFetchPlan {
    pub screen_id: &'static str,
    pub endpoint_keys: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlUiFrontendManifest {
    pub schema_version: u8,
    pub source: &'static str,
    pub rust_view_model_ready: bool,
    pub primary_nav: &'static [&'static str],
    pub live_data_endpoint_keys: &'static [&'static str],
    pub screen_fetch_plans: Vec<ControlUiFetchPlan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlUiAsset {
    pub path: &'static str,
    pub kind: &'static str,
    pub byte_count: usize,
    pub present: bool,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlUiCommandBinding {
    pub id: &'static str,
    pub command: &'static str,
    pub used_by: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlUiInteractionCapability {
    pub id: &'static str,
    pub title: &'static str,
    pub implemented: bool,
    pub availability: &'static str,
    pub requires_live_adapter: bool,
    pub evidence_kind: &'static str,
    pub evidence: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlUiEvidenceLayer {
    pub status: &'static str,
    pub coverage_percent: u8,
    pub verified: bool,
    pub evidence_ref: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlUiEvidenceCoverage {
    pub schema_version: u8,
    pub static_contract: ControlUiEvidenceLayer,
    pub unit_state: ControlUiEvidenceLayer,
    pub browser_behavior: ControlUiEvidenceLayer,
    pub backend_mutation_readback: ControlUiEvidenceLayer,
    pub live_adapter: ControlUiEvidenceLayer,
    pub overall_evidence_percent: u8,
    pub all_required_layers_verified: bool,
    pub boundary: &'static str,
}

impl ControlUiEvidenceLayer {
    pub fn complete(&self) -> bool {
        self.status == "verified"
            && self.coverage_percent == 100
            && self.verified
            && self.evidence_ref.is_some()
    }
}

impl ControlUiEvidenceCoverage {
    pub fn complete(&self) -> bool {
        self.all_required_layers_verified
            && self.overall_evidence_percent == 100
            && self.static_contract.complete()
            && self.unit_state.complete()
            && self.browser_behavior.complete()
            && self.backend_mutation_readback.complete()
            && self.live_adapter.complete()
    }

    pub fn live_operator_surface_percent(&self) -> u8 {
        [
            self.browser_behavior.coverage_percent,
            self.backend_mutation_readback.coverage_percent,
            self.live_adapter.coverage_percent,
        ]
        .into_iter()
        .min()
        .unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlUiConvergenceLane {
    pub id: &'static str,
    pub title: &'static str,
    pub ready: bool,
    pub passed_count: usize,
    pub check_count: usize,
    pub percent: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlUiRustFrontendOwnership {
    pub status: &'static str,
    pub rust_embedded_static_asset_count: usize,
    pub required_static_asset_count: usize,
    pub rust_embedded_static_asset_coverage_percent: u8,
    pub rust_view_model_ready: bool,
    pub rust_view_model_source: &'static str,
    pub browser_renderer_language: &'static str,
    pub pure_browser_rust_runtime: bool,
    pub boundary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlUiReport {
    pub product: &'static str,
    pub ui_name: &'static str,
    pub status: &'static str,
    pub version: &'static str,
    pub screen_count: usize,
    pub implemented_screen_count: usize,
    pub screen_coverage_percent: u8,
    pub screen_coverage_percent_basis: &'static str,
    pub live_implemented_screen_count: usize,
    pub asset_count: usize,
    pub present_asset_count: usize,
    pub required_asset_count: usize,
    pub asset_coverage_percent: u8,
    pub command_binding_count: usize,
    pub interaction_capability_count: usize,
    pub implemented_interaction_capability_count: usize,
    pub capability_manifest_schema_version: u8,
    pub capability_mode: &'static str,
    pub live_adapter_bound: bool,
    pub static_interaction_contract_percent: u8,
    pub live_operator_surface_percent: u8,
    pub developer_interaction_percent: u8,
    pub developer_interaction_percent_basis: &'static str,
    pub ref_agent_alignment_percent: u8,
    pub local_preview_ready: bool,
    pub evidence_coverage: ControlUiEvidenceCoverage,
    pub rust_frontend_ownership: ControlUiRustFrontendOwnership,
    pub serve_command: &'static str,
    pub smoke_gate: &'static str,
    pub package_path: &'static str,
    pub screens: Vec<ControlUiScreen>,
    pub frontend_manifest: ControlUiFrontendManifest,
    pub assets: Vec<ControlUiAsset>,
    pub command_bindings: Vec<ControlUiCommandBinding>,
    pub interaction_capabilities: Vec<ControlUiInteractionCapability>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct ControlUiContractAuditReport {
    pub product: &'static str,
    pub status: &'static str,
    pub evidence_scope: &'static str,
    pub live_product_complete: bool,
    pub core_screen_count: usize,
    pub app_screen_count: usize,
    pub readme_screen_count: usize,
    pub command_binding_count: usize,
    pub interaction_capability_count: usize,
    pub capability_manifest_schema_version: u8,
    pub capability_mode: &'static str,
    pub live_adapter_bound: bool,
    pub all_screen_ids_aligned: bool,
    pub app_has_live_event_stream: bool,
    pub app_has_diff_review: bool,
    pub app_has_keyboard_shortcuts: bool,
    pub app_has_json_inspector: bool,
    pub app_has_operator_drilldown: bool,
    pub app_has_endpoint_health_grid: bool,
    pub app_has_dry_run_action_cards: bool,
    pub app_has_approval_cards: bool,
    pub app_has_post_action_guard: bool,
    pub app_has_session_inspector: bool,
    pub app_has_task_drilldown: bool,
    pub app_has_transcript_preview: bool,
    pub app_has_transcript_search: bool,
    pub app_has_replay_promotion_drilldown: bool,
    pub app_has_event_cursor: bool,
    pub app_has_readonly_command_runner: bool,
    pub app_has_operator_security: bool,
    pub app_has_task_publisher: bool,
    pub app_has_agent_chat: bool,
    pub app_has_external_agent_benchmark: bool,
    pub app_has_hepta_runtime_ui_parity: bool,
    pub app_has_hepta_runtime_navigation_groups: bool,
    pub app_has_chat_first_architecture: bool,
    pub app_has_route_view_controller: bool,
    pub app_has_command_palette: bool,
    pub app_has_premium_consumer_ui: bool,
    pub app_has_progressive_disclosure: bool,
    pub app_has_simplified_primary_nav: bool,
    pub app_preserves_typing_during_live_poll: bool,
    pub app_has_minimal_consumer_workspace: bool,
    pub app_has_telegram_multi_agent_workspace: bool,
    pub app_has_hepta_runtime_2026_5_2_ui_resilience: bool,
    pub app_models_long_gateway_websocket_resilience: bool,
    pub app_has_grouped_message_width_guard: bool,
    pub app_has_ios_pwa_bounds_guard: bool,
    pub app_has_selection_contrast_guard: bool,
    pub app_has_slash_feedback_surface: bool,
    pub app_has_talk_diagnostics_resilience: bool,
    pub app_has_persisted_auto_scroll_mode: bool,
    pub app_has_blank_dashboard_recovery_panel: bool,
    pub app_has_compact_session_status_badges: bool,
    pub app_scopes_nodes_polling_to_active_tab: bool,
    pub app_distinguishes_sample_vs_live_adapter_readiness: bool,
    pub app_has_terminal_qr_rendering_guard: bool,
    pub control_ui_gateway_websocket_opened_by_audit: bool,
    pub control_ui_live_gateway_rpc_performed: bool,
    pub p0_ready: bool,
    pub p1_ready: bool,
    pub p2_ready: bool,
    pub p3_ready: bool,
    pub p4_ready: bool,
    pub p5_ready: bool,
    pub p6_ready: bool,
    pub p7_ready: bool,
    pub p8_ready: bool,
    pub p9_ready: bool,
    pub p10_ready: bool,
    pub p11_ready: bool,
    pub p12_ready: bool,
    pub p13_ready: bool,
    pub p14_ready: bool,
    pub p15_ready: bool,
    pub p16_ready: bool,
    pub p17_ready: bool,
    pub p18_ready: bool,
    pub p19_ready: bool,
    pub p20_ready: bool,
    pub p21_ready: bool,
    pub p22_ready: bool,
    pub p23_ready: bool,
    pub p24_ready: bool,
    pub p25_ready: bool,
    pub p26_ready: bool,
    pub p27_ready: bool,
    pub p28_ready: bool,
    pub p29_ready: bool,
    pub p30_ready: bool,
    pub p31_ready: bool,
    pub p32_ready: bool,
    pub p33_ready: bool,
    pub p34_ready: bool,
    pub p35_ready: bool,
    pub p36_ready: bool,
    pub p37_ready: bool,
    pub p38_ready: bool,
    pub p39_ready: bool,
    pub p0_p1_p2_converged: bool,
    pub p0_p4_converged: bool,
    pub p0_p6_converged: bool,
    pub p0_p13_converged: bool,
    pub p0_p21_converged: bool,
    pub p0_p29_converged: bool,
    pub p0_p39_converged: bool,
    pub convergence_percent: u8,
    pub convergence_lanes: Vec<ControlUiConvergenceLane>,
    pub readme_mentions_boundary: bool,
    pub audit_percent: u8,
    pub missing_in_app: Vec<String>,
    pub missing_in_readme: Vec<String>,
}

impl ControlUiReport {
    pub fn static_contract_complete(&self) -> bool {
        self.screen_count > 0
            && self.screen_count == self.implemented_screen_count
            && self.required_asset_count == self.present_asset_count
            && self.screen_coverage_percent == 100
            && self.asset_coverage_percent == 100
            && self
                .rust_frontend_ownership
                .rust_embedded_static_asset_coverage_percent
                == 100
            && self.rust_frontend_ownership.rust_view_model_ready
            && self.developer_interaction_percent == 100
            && self.ref_agent_alignment_percent == 100
            && self.local_preview_ready
    }

    pub fn complete(&self) -> bool {
        self.status == "complete"
            && self.static_contract_complete()
            && self.evidence_coverage.complete()
            && self.live_operator_surface_percent == 100
    }
}

pub fn control_ui_report() -> ControlUiReport {
    let screens = control_ui_screens();
    let assets = control_ui_assets();
    let command_bindings = control_ui_command_bindings();
    let interaction_capabilities = control_ui_interaction_capabilities();
    let screen_count = screens.len();
    let implemented_screen_count = screens.iter().filter(|screen| screen.implemented).count();
    let asset_count = assets.len();
    let present_asset_count = assets.iter().filter(|asset| asset.present).count();
    let required_asset_count = assets.iter().filter(|asset| asset.required).count();
    let present_required_asset_count = assets
        .iter()
        .filter(|asset| asset.required && asset.present)
        .count();
    let interaction_capability_count = interaction_capabilities.len();
    let implemented_interaction_capability_count = interaction_capabilities
        .iter()
        .filter(|capability| capability.implemented)
        .count();

    let screen_coverage_percent = percent(implemented_screen_count, screen_count);
    let asset_coverage_percent = percent(present_required_asset_count, required_asset_count);
    let developer_interaction_percent = percent(
        implemented_interaction_capability_count,
        interaction_capability_count,
    );
    let static_interaction_contract_percent = developer_interaction_percent;
    let ref_agent_alignment_percent = percent(
        implemented_screen_count + implemented_interaction_capability_count,
        screen_count + interaction_capability_count,
    );
    let declared_screen_ids = screens
        .iter()
        .map(|screen| screen.id.to_string())
        .collect::<Vec<_>>();
    let declared_command_ids = command_bindings
        .iter()
        .map(|binding| binding.id.to_string())
        .collect::<Vec<_>>();
    let app_screen_ids = parse_app_screen_ids();
    let app_command_ids = parse_app_command_ids();
    let renderer_contract_aligned = same_unique_ids(&declared_screen_ids, &app_screen_ids)
        && same_unique_ids(&declared_command_ids, &app_command_ids);
    let rust_frontend_html = control_ui_index_html();
    let progressive_javascript = std::str::from_utf8(CONTROL_UI_JS).unwrap_or_default();
    let local_preview_ready = screen_coverage_percent == 100
        && asset_coverage_percent == 100
        && developer_interaction_percent == 100
        && !command_bindings.is_empty()
        && renderer_contract_aligned
        && rust_frontend_html.contains("Hepta Control UI")
        && rust_frontend_html.contains("data-rust-frontend-renderer")
        && rust_frontend_html.contains("data-no-js-fallback=\"navigation\"")
        && rust_frontend_html.contains("data-progressive-enhancement=\"same-origin-read-only\"")
        && rust_frontend_html.contains("data-js-artifacts=\"external-read-only\"")
        && rust_frontend_html.contains("hepta-core::control_ui")
        && rust_frontend_html.contains("<script defer src=\"./control-ui.js\"></script>")
        && !rust_frontend_html.contains("<script>")
        && progressive_javascript.contains("/api/operator-snapshot")
        && progressive_javascript.contains("READ_ONLY_ROUTES")
        && progressive_javascript.contains("UNAVAILABLE_PREVIEW_CONTROLS")
        && progressive_javascript.contains("configureLocalJsonPreview")
        && progressive_javascript.contains("insertLocalDraftText")
        && !progressive_javascript.contains("innerHTML")
        && rust_frontend_html.contains("data-control-ui-capability-mode=\"local-read-only\"")
        && rust_frontend_html.contains("data-control-ui-live-adapter-bound=\"false\"")
        && interaction_capabilities
            .iter()
            .all(|capability| capability.implemented && !capability.requires_live_adapter);
    let frontend_manifest = control_ui_frontend_manifest();
    let rust_frontend_ownership = control_ui_rust_frontend_ownership(&assets, &frontend_manifest);
    let static_contract_percent = [
        screen_coverage_percent,
        asset_coverage_percent,
        static_interaction_contract_percent,
        ref_agent_alignment_percent,
        rust_frontend_ownership.rust_embedded_static_asset_coverage_percent,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let static_contract_verified = local_preview_ready && static_contract_percent == 100;
    let evidence_coverage =
        control_ui_evidence_coverage(static_contract_percent, static_contract_verified);
    let live_operator_surface_percent = evidence_coverage.live_operator_surface_percent();

    ControlUiReport {
        product: "Hepta",
        ui_name: "Hepta Control UI",
        status: if evidence_coverage.complete() {
            "complete"
        } else if static_contract_verified {
            "static_contract_complete"
        } else {
            "incomplete"
        },
        version: "control-ui-v0",
        screen_count,
        implemented_screen_count,
        screen_coverage_percent,
        screen_coverage_percent_basis: "static local read-only screen shells; not live adapter or mutation coverage",
        live_implemented_screen_count: 0,
        asset_count,
        present_asset_count,
        required_asset_count,
        asset_coverage_percent,
        command_binding_count: command_bindings.len(),
        interaction_capability_count,
        implemented_interaction_capability_count,
        capability_manifest_schema_version: 2,
        capability_mode: "local-read-only",
        live_adapter_bound: false,
        static_interaction_contract_percent,
        live_operator_surface_percent,
        developer_interaction_percent,
        developer_interaction_percent_basis: "compatibility alias for static_interaction_contract_percent; not browser or live execution evidence",
        ref_agent_alignment_percent,
        local_preview_ready,
        evidence_coverage,
        rust_frontend_ownership,
        serve_command: "cargo run --manifest-path codex-rs/Cargo.toml -p hepta-cli --bin hepta -- --serve-ui 127.0.0.1:7373",
        smoke_gate: "./scripts/hepta-control-ui-smoke.sh",
        package_path: "apps/hepta-control-ui",
        screens,
        frontend_manifest,
        assets,
        command_bindings,
        interaction_capabilities,
    }
}

pub fn control_ui_contract_audit_report() -> ControlUiContractAuditReport {
    let rust_frontend_html = control_ui_index_html();
    let progressive_javascript = std::str::from_utf8(CONTROL_UI_JS).unwrap_or_default();
    let core_ids = control_ui_screens()
        .into_iter()
        .map(|screen| screen.id.to_string())
        .collect::<Vec<_>>();
    let app_ids = parse_app_screen_ids();
    let readme_ids = parse_readme_screen_ids();
    let missing_in_app = core_ids
        .iter()
        .filter(|id| !app_ids.iter().any(|candidate| candidate == *id))
        .cloned()
        .collect::<Vec<_>>();
    let missing_in_readme = core_ids
        .iter()
        .filter(|id| !readme_ids.iter().any(|candidate| candidate == *id))
        .cloned()
        .collect::<Vec<_>>();
    let all_screen_ids_aligned = missing_in_app.is_empty()
        && missing_in_readme.is_empty()
        && core_ids.len() == app_ids.len()
        && core_ids.len() == readme_ids.len();

    let capabilities = control_ui_interaction_capabilities();
    let typed_capability_manifest_ready = !capabilities.is_empty()
        && capabilities.iter().all(|capability| {
            capability.implemented
                && !capability.requires_live_adapter
                && matches!(
                    capability.availability,
                    "browser-native" | "local-only" | "same-origin-read-only"
                )
        });
    let capability_ids = capabilities
        .iter()
        .map(|capability| capability.id.to_string())
        .collect::<Vec<_>>();
    let mut unique_capability_ids = capability_ids.clone();
    unique_capability_ids.sort();
    unique_capability_ids.dedup();
    let unique_capability_ids = unique_capability_ids.len() == capability_ids.len();
    let serialized_security_contract_ready =
        serde_json::from_str::<serde_json::Value>(CONTROL_UI_RUST_RENDERER_MARKERS).is_ok();

    let app_has_json_inspector = rust_frontend_html.contains("id=\"json-input\"")
        && rust_frontend_html.contains("id=\"json-preview\"")
        && progressive_javascript.contains("configureLocalJsonPreview")
        && progressive_javascript.contains("JSON.parse(source)");
    let app_has_readonly_command_runner = progressive_javascript.contains("READ_ONLY_ROUTES")
        && progressive_javascript.contains("getSameOriginJson")
        && progressive_javascript.contains("method: \"GET\"")
        && !progressive_javascript.contains("method: \"POST\"");
    let app_has_operator_security = progressive_javascript.contains("\"operator-security\"")
        && progressive_javascript.contains("/api/operator-security");
    let app_has_hepta_runtime_navigation_groups = rust_frontend_html.contains("nav-group--primary")
        && rust_frontend_html.contains("href=\"#chat\"")
        && rust_frontend_html.contains("href=\"#screen-card-tasks\"")
        && rust_frontend_html.contains("href=\"#screen-card-ops\"");
    let app_has_chat_first_architecture = rust_frontend_html
        .contains("data-chat-first-architecture=\"true\"")
        && rust_frontend_html.contains("data-control-ui-primary-path=\"telegram-chat-shell\"");
    let app_has_route_view_controller =
        rust_frontend_html.contains("data-route-view-controller=\"native-anchor-routes\"");
    let app_has_command_palette = rust_frontend_html.contains("id=\"command-palette\"")
        && rust_frontend_html.contains("popover=\"auto\"")
        && progressive_javascript.contains("command-palette-input");
    let app_has_premium_consumer_ui = rust_frontend_html
        .contains("data-control-ui-top-design-referee")
        && rust_frontend_html.contains("data-control-ui-telegram-shell=\"true\"");
    let app_has_progressive_disclosure = rust_frontend_html.contains("popover=\"auto\"")
        && rust_frontend_html.contains("popovertarget=");
    let app_has_simplified_primary_nav = rust_frontend_html.contains("nav-group--primary")
        && rust_frontend_html.contains("data-control-ui-secondary-nav=\"collapsed\"");
    let app_has_minimal_consumer_workspace =
        rust_frontend_html.contains("data-minimal-consumer-workspace=\"true\"");
    let app_has_grouped_message_width_guard =
        CONTROL_UI_STYLES_CSS.contains("--hepta_runtime-2026-5-2-grouped-message-max-width");
    let app_has_ios_pwa_bounds_guard = CONTROL_UI_STYLES_CSS.contains("safe-area-inset-bottom")
        && CONTROL_UI_STYLES_CSS.contains("100dvh");
    let app_has_selection_contrast_guard = CONTROL_UI_STYLES_CSS.contains("::selection")
        && CONTROL_UI_STYLES_CSS.contains("--hepta_runtime-2026-5-2-selection-bg")
        && CONTROL_UI_STYLES_CSS.contains("--hepta_runtime-2026-5-2-selection-fg");
    let app_distinguishes_sample_vs_live_adapter_readiness = rust_frontend_html
        .contains("data-readiness-kind=\"sample\"")
        && rust_frontend_html.contains("data-readiness-kind=\"live-adapter\"")
        && rust_frontend_html.contains("data-control-ui-live-adapter-bound=\"false\"");

    let truth_checks = [
        all_screen_ids_aligned,
        typed_capability_manifest_ready,
        unique_capability_ids,
        serialized_security_contract_ready,
        rust_frontend_html.contains("data-control-ui-capability-mode=\"local-read-only\""),
        rust_frontend_html.contains("<span>live adapter</span><strong>0</strong>"),
        rust_frontend_html.contains("documented, not live workflows"),
        rust_frontend_html.contains("data-live-event-stream=\"false\""),
        rust_frontend_html.contains("data-task-publisher=\"false\""),
        rust_frontend_html.contains("data-agent-chat=\"false\""),
        progressive_javascript.contains("UNAVAILABLE_PREVIEW_CONTROLS"),
        progressive_javascript.contains("insertLocalDraftText"),
        progressive_javascript.contains("configureComposerPickerSearch"),
        app_has_json_inspector,
        app_has_readonly_command_runner,
        CONTROL_UI_STYLES_CSS.len() <= CONTROL_UI_UNIFIED_LANE_STYLES_CSS_BUDGET_BYTES,
        !progressive_javascript.contains("fetchActionPlan"),
        !progressive_javascript.contains("submitAgentChat"),
    ];
    let audit_percent = percent(
        truth_checks.into_iter().filter(|ready| *ready).count(),
        truth_checks.len(),
    );
    let convergence_lanes = capabilities
        .iter()
        .map(|capability| {
            convergence_lane(
                capability.id,
                capability.title,
                &[capability.implemented && !capability.requires_live_adapter],
            )
        })
        .collect::<Vec<_>>();
    let convergence_percent = percent(
        convergence_lanes.iter().filter(|lane| lane.ready).count(),
        convergence_lanes.len(),
    );
    let readme_mentions_boundary =
        CONTROL_UI_README.contains("Boundary") && CONTROL_UI_README.contains("not a hosted SaaS");

    ControlUiContractAuditReport {
        product: "Hepta",
        status: if audit_percent == 100 {
            "static_contract_complete"
        } else {
            "static_contract_incomplete"
        },
        evidence_scope: "typed local read-only capability manifest plus DOM and bounded JavaScript evidence; no live workflow claim",
        live_product_complete: false,
        core_screen_count: core_ids.len(),
        app_screen_count: app_ids.len(),
        readme_screen_count: readme_ids.len(),
        command_binding_count: control_ui_command_bindings().len(),
        interaction_capability_count: capabilities.len(),
        capability_manifest_schema_version: 2,
        capability_mode: "local-read-only",
        live_adapter_bound: false,
        all_screen_ids_aligned,
        app_has_json_inspector,
        app_has_readonly_command_runner,
        app_has_operator_security,
        app_has_hepta_runtime_navigation_groups,
        app_has_chat_first_architecture,
        app_has_route_view_controller,
        app_has_command_palette,
        app_has_premium_consumer_ui,
        app_has_progressive_disclosure,
        app_has_simplified_primary_nav,
        app_has_minimal_consumer_workspace,
        app_has_grouped_message_width_guard,
        app_has_ios_pwa_bounds_guard,
        app_has_selection_contrast_guard,
        app_distinguishes_sample_vs_live_adapter_readiness,
        control_ui_gateway_websocket_opened_by_audit: false,
        control_ui_live_gateway_rpc_performed: false,
        convergence_percent,
        convergence_lanes,
        readme_mentions_boundary,
        audit_percent,
        missing_in_app,
        missing_in_readme,
        ..ControlUiContractAuditReport::default()
    }
}

/// Return the canonical static Control UI snapshot embedded from
/// `apps/hepta-control-ui/index.html`.
///
/// The gateway and direct-file preview intentionally consume the same bytes;
/// view-model reports remain separate read-only evidence and do not synthesize
/// a second HTML implementation.
pub fn control_ui_index_html() -> String {
    CONTROL_UI_INDEX_HTML.to_owned()
}

pub fn control_ui_assets() -> Vec<ControlUiAsset> {
    vec![
        asset(
            "apps/hepta-control-ui/index.html",
            "html",
            CONTROL_UI_INDEX_HTML,
        ),
        asset(
            "apps/hepta-control-ui/styles.css",
            "css",
            CONTROL_UI_STYLES_CSS,
        ),
        asset_bytes(
            "apps/hepta-control-ui/assets/hepta-agent-logo.png",
            "image/png",
            CONTROL_UI_HEPTA_AGENT_LOGO_PNG,
        ),
        asset_bytes(
            "apps/hepta-control-ui/control-ui.js",
            "text/javascript",
            CONTROL_UI_JS,
        ),
        asset("apps/hepta-control-ui/README.md", "docs", CONTROL_UI_README),
    ]
}

fn asset(path: &'static str, kind: &'static str, content: &'static str) -> ControlUiAsset {
    ControlUiAsset {
        path,
        kind,
        byte_count: content.len(),
        present: !content.trim().is_empty(),
        required: true,
    }
}

fn asset_bytes(path: &'static str, kind: &'static str, content: &'static [u8]) -> ControlUiAsset {
    ControlUiAsset {
        path,
        kind,
        byte_count: content.len(),
        present: !content.is_empty(),
        required: true,
    }
}

pub fn control_ui_screens() -> Vec<ControlUiScreen> {
    vec![
        screen(
            "dashboard",
            "Dashboard",
            "#dashboard",
            &[
                "/doctor --json",
                "/native-capabilities --json",
                "/external-readiness --json",
            ],
            &[
                "Health strip",
                "Capability coverage",
                "Readiness summary",
                "Artifact status",
            ],
        ),
        screen(
            "config",
            "Config Surface",
            "#config",
            &[
                "/config-surface --json",
                "/local-import --json",
                "/providers --json",
                "/image-models --json",
                "/optional-configs --json",
            ],
            &[
                "Redacted option map",
                "Provider/search/image/channel readiness",
                "Top-level key alignment",
            ],
        ),
        screen(
            "sessions",
            "Sessions",
            "#sessions",
            &[
                "/sessions --json",
                "/activity --json",
                "/session-activity --json",
            ],
            &["Session table", "Topic state", "Activity counters"],
        ),
        screen(
            "tasks",
            "Tasks",
            "#tasks",
            &[
                "/tasks --json",
                "/run-ready-tasks --json",
                "/join-tasks --json",
            ],
            &["Queue lanes", "Dependency gates", "Ready/due counters"],
        ),
        screen(
            "task-publisher",
            "Task Publisher",
            "#task-publisher",
            &[
                "POST /api/tasks/plan",
                "POST /api/tasks/publish",
                "/spawn-task <worker_id> <prompt> --json",
            ],
            &[
                "Worker id input",
                "Task prompt composer",
                "Confirm publish gate",
                "Queue result preview",
            ],
        ),
        screen(
            "workers",
            "Workers",
            "#workers",
            &["/workers --json", "/task-supervisor --json"],
            &["Worker cards", "Supervisor next action", "Join safety"],
        ),
        screen(
            "operator",
            "Operator Console",
            "#operator",
            &[
                "/operator-console --json",
                "/runtime/operator --json",
                "POST /api/runtime/operator",
                "/subagent-observatory --json",
                "/task-supervisor --json",
                "/events --json",
            ],
            &[
                "Live subagent tree",
                "Task queue controls",
                "Command stream",
                "Patch/evidence review",
                "Pause/resume/interrupt controls",
                "Confirm-gated runtime kill/steer dry-run evidence",
            ],
        ),
        screen(
            "live",
            "Live Event Stream",
            "#live",
            &[
                "/events-report --json",
                "/api/events-report",
                "/api/activity",
            ],
            &[
                "Auto-refresh event timeline",
                "Live log tail contract",
                "No-cache API polling",
            ],
        ),
        screen(
            "transcript",
            "Conversation Transcript",
            "#transcript",
            &["/transcript --json", "/query-transcript <query> --json"],
            &[
                "Threaded turn preview",
                "Search/query transcript affordance",
                "Session-scoped context panel",
            ],
        ),
        screen(
            "chat",
            "Agent Chat",
            "#chat",
            &[
                "POST /api/chat/register",
                "POST /api/chat/delete",
                "POST /api/chat/plan",
                "POST /api/chat",
                "/agent-send <agent_id> --from <from_agent_id> <message> --json",
            ],
            &[
                "Conversation rail",
                "Add/remove conversation controls",
                "Per-agent thread",
                "Message composer",
                "Immediate local send",
                "Optional dry-run plan",
            ],
        ),
        screen(
            "diff",
            "Diff Review",
            "#diff",
            &[
                "/task-diff <task_id> --json",
                "/apply-task-patches <task_id> --json",
                "/rollback-task-patches <task_id> --json",
            ],
            &[
                "Patch hunk review",
                "Apply/rollback command rail",
                "Evidence-linked risk notes",
            ],
        ),
        screen(
            "approvals",
            "Approvals",
            "#approvals",
            &[
                "/approvals --json",
                "/policy --json",
                "/operator-console --json",
                "/approve",
                "/allowlist",
            ],
            &[
                "Approval card state",
                "Policy/risk tier snapshot",
                "Exec approvals live editor parity",
                "gateway/node target selector",
                "per-agent scope diff preview",
                "Human gate reminder",
            ],
        ),
        screen(
            "security",
            "Operator Security",
            "#security",
            &[
                "/operator-security --json",
                "/api/operator-security",
                "/api/policy",
            ],
            &[
                "Loopback bind guard",
                "Security headers",
                "Read-only command allowlist",
                "Dry-run action planner",
                "Role/endpoint guard matrix",
            ],
        ),
        screen(
            "gateway",
            "Gateway Monitor",
            "#gateway",
            &[
                "/gateway-runtime --json",
                "/gateway-dispatch --dry-run --json",
                "/gateway-ledger --json",
                "/gateway-retry-dead-letter --json",
            ],
            &[
                "Adapter/queue status",
                "Delivery ledger replay",
                "Retry/dead-letter diagnostics",
            ],
        ),
        screen(
            "runtime-control-plane",
            "Hepta Operator Plane",
            "#runtime-control-plane",
            &[
                "upstream runtime control modules",
                "/status",
                "/sessions",
                "/tasks",
                "/approve",
                "/tools",
                "/models",
                "/plugins",
                "/nodes",
                "/channels",
                "/cron",
                "/logs",
            ],
            &[
                "Full alignment matrix",
                "Hepta route mapping",
                "Boundary badges",
                "Next packet priorities",
            ],
        ),
        screen(
            "multi-agent",
            "Multi-Agent Runtime",
            "#multi-agent",
            &["/multi-agent-runtime --agents 4 --messages 8 --json"],
            &[
                "Agent pool topology",
                "Reducer/consensus status",
                "Failure recovery evidence",
            ],
        ),
        screen(
            "developer",
            "Developer Console",
            "#developer",
            &[
                "/help",
                "/doctor --json",
                "/gateway-dispatch --dry-run --json",
            ],
            &[
                "Command palette",
                "JSON inspector",
                "Dry-run command runner contract",
            ],
        ),
        screen(
            "artifacts",
            "Artifacts",
            "#artifacts",
            &["docs/release", "dist", "/production-parity --json"],
            &[
                "Release docs index",
                "Preflight package refs",
                "Boundary-aware artifact preview",
            ],
        ),
        screen(
            "handoff",
            "Worker Handoff",
            "#handoff",
            &[
                "/handoff-bundle <task_id> --json",
                "/promotion-ledger <task_id> --json",
                "/task-replay <task_id> --json",
                "/api/handoff-bundle/<task_id>",
            ],
            &[
                "Signed handoff bundle",
                "Promotion approval trail",
                "Replay/risk evidence",
            ],
        ),
        screen(
            "ops",
            "Ops Status",
            "#ops",
            &[
                "/ops-status --json",
                "/api/ops-status",
                "/api/hepta-merge-completion",
                "scripts/hepta-installed-live-watchdog-recurring.sh",
                "scripts/hepta-gateway-service.sh",
            ],
            &[
                "Installed binary status",
                "Live UI/API status",
                "Watchdog latest state",
                "Service plist state",
            ],
        ),
        screen(
            "readiness",
            "Readiness",
            "#readiness",
            &[
                "/external-readiness --json",
                "/production-surface --json",
                "/api/hepta-merge-completion",
            ],
            &["Gate matrix", "Blocker list", "Manifest health"],
        ),
        screen(
            "parity",
            "Production Parity",
            "#parity",
            &[
                "/production-parity --json",
                "/native-capabilities --json",
                "/external-readiness --json",
                "/control-ui --json",
                "/api/hepta-merge-completion",
            ],
            &[
                "Completion dimensions",
                "Production baseline comparison",
                "Remaining gap ledger",
            ],
        ),
        screen(
            "external-agent-benchmark",
            "External Agent Benchmark",
            "#external-agent-benchmark",
            &[
                "/external-agent-benchmark --json",
                "/api/external-agent-benchmark",
                "/agent-advantage --json",
            ],
            &[
                "Benchmark task corpus",
                "Adapter contracts",
                "Scoring rubric",
                "No synthetic wins boundary",
            ],
        ),
        screen(
            "evidence",
            "Evidence",
            "#evidence",
            &[
                ".hepta/external-production/manifest.json",
                "docs/release/HEPTA_EXTERNAL_PRODUCTION_EXECUTION_EVIDENCE_2026-04-25.md",
            ],
            &["Redacted refs", "Boundary notes", "Secret-safety reminders"],
        ),
        screen(
            "commands",
            "Commands",
            "#commands",
            &["/help", "/control-ui --json"],
            &["Command palette", "Copy buttons", "Smoke checklist"],
        ),
        screen(
            "runbook",
            "Runbook",
            "#runbook",
            &["docs/release", "scripts/hepta-v0.1-preflight.sh"],
            &["Release docs", "Preflight artifact", "Rollback boundary"],
        ),
    ]
}

fn screen(
    id: &'static str,
    title: &'static str,
    route: &'static str,
    data_sources: &'static [&'static str],
    widgets: &'static [&'static str],
) -> ControlUiScreen {
    ControlUiScreen {
        id,
        title,
        summary: control_ui_screen_summary(id),
        route,
        implemented: true,
        implementation_scope: "static-local-read-only-screen-shell",
        live_adapter_ready: false,
        data_sources,
        widgets,
        interactions: control_ui_screen_interactions(id),
    }
}

pub fn control_ui_frontend_manifest() -> ControlUiFrontendManifest {
    ControlUiFrontendManifest {
        schema_version: 1,
        source: "apps/hepta-control-ui/index.html",
        rust_view_model_ready: true,
        primary_nav: &["chat", "tasks", "ops", "external-agent-benchmark"],
        live_data_endpoint_keys: &[
            "ui",
            "opsStatus",
            "readiness",
            "parity",
            "sessions",
            "sessionActivity",
            "transcript",
            "tasks",
            "workspaceMembers",
            "workers",
            "supervisor",
            "configSurface",
            "localImport",
            "providers",
            "imageModels",
            "optionalConfigs",
            "operatorConsole",
            "approvals",
            "policy",
            "eventsReport",
            "liveEvents",
            "activity",
            "gatewayRuntime",
            "gatewayDispatch",
            "multiAgentRuntime",
            "uiContractAudit",
            "operatorSnapshot",
            "operatorSecurity",
            "mergeCompletion",
            "externalAgentBenchmark",
            "uiActionPlan",
            "cron",
        ],
        screen_fetch_plans: vec![
            fetch_plan(
                "chat",
                &[
                    "sessions",
                    "sessionActivity",
                    "transcript",
                    "tasks",
                    "workspaceMembers",
                    "activity",
                    "multiAgentRuntime",
                    "uiContractAudit",
                ],
            ),
            fetch_plan("task-publisher", &["tasks", "workers", "supervisor"]),
            fetch_plan(
                "tasks",
                &[
                    "tasks",
                    "workers",
                    "supervisor",
                    "taskDetails",
                    "gatewayDispatch",
                ],
            ),
            fetch_plan(
                "workers",
                &[
                    "workers",
                    "supervisor",
                    "tasks",
                    "providers",
                    "configSurface",
                    "localImport",
                    "optionalConfigs",
                ],
            ),
            fetch_plan(
                "ops",
                &[
                    "readiness",
                    "parity",
                    "operatorConsole",
                    "eventsReport",
                    "liveEvents",
                    "gatewayRuntime",
                    "gatewayDispatch",
                    "operatorSnapshot",
                    "operatorSecurity",
                    "mergeCompletion",
                    "uiActionPlan",
                    "configSurface",
                ],
            ),
            fetch_plan(
                "operator",
                &[
                    "operatorConsole",
                    "operatorSnapshot",
                    "eventsReport",
                    "liveEvents",
                ],
            ),
            fetch_plan(
                "live",
                &["eventsReport", "liveEvents", "activity", "operatorConsole"],
            ),
            fetch_plan("security", &["operatorSecurity", "policy", "approvals"]),
            fetch_plan(
                "gateway",
                &[
                    "gatewayRuntime",
                    "gatewayDispatch",
                    "uiActionPlan",
                    "configSurface",
                ],
            ),
            fetch_plan(
                "multi-agent",
                &[
                    "multiAgentRuntime",
                    "sessions",
                    "activity",
                    "providers",
                    "configSurface",
                    "localImport",
                    "optionalConfigs",
                ],
            ),
            fetch_plan("approvals", &["approvals", "policy"]),
            fetch_plan(
                "readiness",
                &[
                    "readiness",
                    "parity",
                    "externalAgentBenchmark",
                    "mergeCompletion",
                ],
            ),
            fetch_plan("parity", &["parity", "readiness", "mergeCompletion"]),
            fetch_plan(
                "external-agent-benchmark",
                &["externalAgentBenchmark", "readiness"],
            ),
            fetch_plan(
                "config",
                &[
                    "configSurface",
                    "localImport",
                    "providers",
                    "imageModels",
                    "optionalConfigs",
                ],
            ),
            fetch_plan(
                "developer",
                &[
                    "configSurface",
                    "localImport",
                    "providers",
                    "commands",
                    "eventsReport",
                    "optionalConfigs",
                ],
            ),
            fetch_plan(
                "transcript",
                &[
                    "transcript",
                    "sessionActivity",
                    "sessions",
                    "eventsReport",
                    "configSurface",
                ],
            ),
            fetch_plan(
                "commands",
                &[
                    "ui",
                    "opsStatus",
                    "providers",
                    "configSurface",
                    "localImport",
                    "optionalConfigs",
                ],
            ),
            fetch_plan("runbook", &["ui", "opsStatus"]),
            fetch_plan("evidence", &["tasks", "taskDetails"]),
            fetch_plan("diff", &["tasks", "taskDetails"]),
            fetch_plan("artifacts", &["tasks", "taskDetails"]),
            fetch_plan("handoff", &["tasks", "taskDetails"]),
        ],
    }
}

pub fn control_ui_rust_frontend_ownership(
    assets: &[ControlUiAsset],
    frontend_manifest: &ControlUiFrontendManifest,
) -> ControlUiRustFrontendOwnership {
    let required_static_asset_count = assets.iter().filter(|asset| asset.required).count();
    let rust_embedded_static_asset_count = assets
        .iter()
        .filter(|asset| asset.required && asset.present)
        .count();
    let rust_embedded_static_asset_coverage_percent = percent(
        rust_embedded_static_asset_count,
        required_static_asset_count,
    );

    ControlUiRustFrontendOwnership {
        status: if rust_embedded_static_asset_coverage_percent == 100
            && frontend_manifest.rust_view_model_ready
        {
            "rust-embedded-progressive-frontend"
        } else {
            "incomplete"
        },
        rust_embedded_static_asset_count,
        required_static_asset_count,
        rust_embedded_static_asset_coverage_percent,
        rust_view_model_ready: frontend_manifest.rust_view_model_ready,
        rust_view_model_source: frontend_manifest.source,
        browser_renderer_language: "html-css-javascript-rust-embedded",
        pure_browser_rust_runtime: false,
        boundary: "apps/hepta-control-ui/index.html is the single authoritative HTML snapshot; Rust embeds it and the digest-bound /control-ui.js asset, which only performs allowlisted same-origin GET inspection and local copy/search enhancement. Navigation remains usable without JavaScript.",
    }
}

fn fetch_plan(
    screen_id: &'static str,
    endpoint_keys: &'static [&'static str],
) -> ControlUiFetchPlan {
    ControlUiFetchPlan {
        screen_id,
        endpoint_keys,
    }
}

fn control_ui_screen_summary(id: &str) -> &'static str {
    match id {
        "dashboard" => {
            "One-glance runtime health, local RC readiness, and production evidence status."
        }
        "config" => "Redacted startup config option map aligned to local external interfaces.",
        "sessions" => "Active, archived, forked, merged, and exported session continuity overview.",
        "tasks" => {
            "Worker task queue, dependency gates, scheduled runs, and safe cancellation state."
        }
        "task-publisher" => {
            "Read-only task-publisher preview; publish remains unavailable until a verified live adapter and mutation receipt exist."
        }
        "workers" => {
            "Worker inventory, owner lanes, active/completed counts, and supervisor next action."
        }
        "operator" => {
            "Read-only task, agent, command, patch, evidence, and approval inspection surface; no live control mutation."
        }
        "live" => {
            "Documented event-timeline and activity preview backed by local read-only reports, not a live event stream."
        }
        "transcript" => "Session transcript and query affordances for debugging model/tool loops.",
        "chat" => {
            "Telegram-style read-only multi-agent transcript with local draft composition; send and planning actions remain unavailable without a verified live adapter."
        }
        "diff" => {
            "Patch hunk review, risk trail, and apply/rollback command rail for coding workflows."
        }
        "approvals" => {
            "Approval cards, policy state, compatibility-style exec approval editor parity, and human-gated action review for risky operations."
        }
        "security" => {
            "Local-only operator security contract: loopback bind guard, security headers, read-only command allowlist, dry-run action planning, and role/endpoint audit matrix."
        }
        "gateway" => {
            "Gateway adapter inventory, dispatch dry-run, persistent ledger replay, and retry/dead-letter diagnostics."
        }
        "runtime-control-plane" => {
            "Full upstream runtime control UI absorption matrix, translated into Hepta's chat-first operator sidecar instead of replacing the multi-agent dialogue."
        }
        "multi-agent" => {
            "Concurrent agent topology, reducer consensus, and failure-recovery evidence."
        }
        "developer" => {
            "Command palette, JSON inspector, keyboard shortcuts, and local-safe dry-run command runner."
        }
        "artifacts" => {
            "Release docs, preflight package refs, and boundary-aware artifact previews."
        }
        "handoff" => {
            "Signed promotion handoff bundles combining evidence, replay, merge risk, gate, and approval trail."
        }
        "ops" => {
            "Installed binary, live UI/API, watchdog latest status, and Hepta service plist state."
        }
        "readiness" => {
            "External-production gate ledger with blockers, pending evidence, and verified refs."
        }
        "parity" => {
            "Hepta Production Parity v1 completion against mature reference-agent baselines."
        }
        "external-agent-benchmark" => {
            "Task corpus, reference-agent adapters, rubric, and evidence ledger boundary for real external comparisons."
        }
        "evidence" => {
            "Redacted evidence refs for provider, channel, node, worker, scheduler, soak, and recovery drills."
        }
        "commands" => {
            "Copy-ready operator commands for status, exports, smoke, soak, preflight, and packaging."
        }
        "runbook" => "Release handoff, install smoke, rollback boundary, and public-GA caveats.",
        _ => "Hepta control UI screen.",
    }
}

fn control_ui_screen_interactions(id: &str) -> &'static [&'static str] {
    match id {
        "task-publisher" => &["taskPublisher", "humanConfirmation", "queueRefresh"],
        "operator" => &["multiAgentTree", "approvalReview", "handoffEvidenceReview"],
        "live" => &["liveEventStream", "liveLogTail"],
        "transcript" => &["sessionTabs", "developerJsonInspector"],
        "chat" => &[
            "agentChatComposer",
            "humanConfirmation",
            "multiAgentRuntime",
            "heptaRuntimeControlPlaneBridge",
        ],
        "diff" => &["diffReview", "handoffEvidenceReview"],
        "approvals" => &["approvalReview", "execApprovalsLiveEditorParity"],
        "security" => &[
            "operatorSecurityRbac",
            "approvalReview",
            "boundaryAwareReadiness",
        ],
        "gateway" => &["gatewayMonitor", "dryRunCommandRunner"],
        "runtime-control-plane" => &[
            "heptaRuntimeControlPlaneBridge",
            "heptaRuntimeControlPlaneAlignment",
            "boundaryAwareReadiness",
        ],
        "multi-agent" => &["multiAgentTree"],
        "developer" => &[
            "commandPalette",
            "keyboardShortcuts",
            "developerJsonInspector",
        ],
        "artifacts" => &["artifactPreview", "boundaryAwareReadiness"],
        "external-agent-benchmark" => &["externalAgentBenchmark", "boundaryAwareReadiness"],
        _ => &[],
    }
}

pub fn control_ui_command_bindings() -> Vec<ControlUiCommandBinding> {
    vec![
        binding(
            "control-ui",
            "/control-ui --json",
            &["dashboard", "commands"],
        ),
        binding(
            "config-surface",
            "/config-surface --json",
            &["dashboard", "config"],
        ),
        binding("local-import", "/local-import --json", &["config"]),
        binding("providers", "/providers --json", &["config"]),
        binding("image-models", "/image-models --json", &["config"]),
        binding(
            "optional-configs",
            "/optional-configs --json",
            &["config", "commands"],
        ),
        binding("doctor", "/doctor --json", &["dashboard"]),
        binding(
            "native-capabilities",
            "/native-capabilities --json",
            &["dashboard"],
        ),
        binding(
            "external-readiness",
            "/external-readiness --json",
            &["dashboard", "readiness", "evidence"],
        ),
        binding(
            "production-surface",
            "/production-surface --json",
            &["readiness", "runbook"],
        ),
        binding(
            "production-parity",
            "/production-parity --json",
            &["dashboard", "parity", "runbook"],
        ),
        binding(
            "hepta-merge-completion",
            "/hepta-merge-completion --json",
            &["ops", "readiness", "parity", "commands"],
        ),
        binding(
            "external-agent-benchmark",
            "/external-agent-benchmark --json",
            &["external-agent-benchmark", "parity", "evidence"],
        ),
        binding("sessions", "/sessions --json", &["sessions"]),
        binding(
            "session-activity",
            "/session-activity --json",
            &["sessions", "transcript"],
        ),
        binding("tasks", "/tasks --json", &["tasks", "task-publisher"]),
        binding("task", "/task <task_id> --json", &["tasks"]),
        binding(
            "spawn-task",
            "/spawn-task <worker_id> <prompt> --json",
            &["tasks", "task-publisher"],
        ),
        binding(
            "ui-task-publisher-plan",
            "POST /api/tasks/plan",
            &["tasks", "task-publisher"],
        ),
        binding(
            "ui-task-publisher-publish",
            "POST /api/tasks/publish",
            &["tasks", "task-publisher"],
        ),
        binding("workers", "/workers --json", &["workers"]),
        binding(
            "operator-console",
            "/operator-console --json",
            &["operator", "workers", "tasks"],
        ),
        binding(
            "subagent-observatory",
            "/subagent-observatory --json",
            &["operator", "workers"],
        ),
        binding(
            "task-supervisor",
            "/task-supervisor --json",
            &["tasks", "workers", "operator"],
        ),
        binding(
            "handoff-bundle",
            "/handoff-bundle <task_id> --json",
            &["tasks", "workers", "handoff"],
        ),
        binding(
            "task-patches",
            "/task-patches <task_id> --json",
            &["tasks", "diff", "handoff"],
        ),
        binding(
            "task-loop",
            "/task-loop <task_id> --json",
            &["tasks", "operator"],
        ),
        binding(
            "task-evidence",
            "/task-evidence <task_id> --json",
            &["tasks", "evidence"],
        ),
        binding(
            "task-replay",
            "/task-replay <task_id> --json",
            &["tasks", "diff", "evidence"],
        ),
        binding(
            "promotion-ledger",
            "/promotion-ledger <task_id> --json",
            &["tasks", "handoff", "evidence"],
        ),
        binding(
            "ops-status",
            "/ops-status --json",
            &["dashboard", "ops", "runbook"],
        ),
        binding(
            "events",
            "/events --json",
            &["dashboard", "operator", "live", "runbook"],
        ),
        binding(
            "events-report",
            "/events-report --json",
            &["live", "operator"],
        ),
        binding("activity", "/activity --json", &["live", "sessions"]),
        binding("transcript", "/transcript --json", &["transcript"]),
        binding(
            "agent-send",
            "/agent-send <agent_id> --from <from_agent_id> <message> --json",
            &["chat", "multi-agent"],
        ),
        binding(
            "ui-agent-chat-plan",
            "POST /api/chat/plan",
            &["chat", "multi-agent"],
        ),
        binding(
            "ui-agent-chat-send",
            "POST /api/chat",
            &["chat", "multi-agent"],
        ),
        binding(
            "query-transcript",
            "/query-transcript <query> --json",
            &["transcript", "developer"],
        ),
        binding("approvals", "/approvals --json", &["approvals", "operator"]),
        binding("policy", "/policy --json", &["approvals", "developer"]),
        binding(
            "exec-approvals-apply",
            "POST /api/approvals/exec/apply",
            &["approvals", "security", "operator"],
        ),
        binding(
            "operator-security",
            "/operator-security --json",
            &["security", "operator", "developer"],
        ),
        binding(
            "gateway-runtime",
            "/gateway-runtime --json",
            &["gateway", "ops"],
        ),
        binding(
            "gateway-dispatch",
            "/gateway-dispatch --dry-run --json",
            &["gateway", "developer"],
        ),
        binding("gateway-ledger", "/gateway-ledger --json", &["gateway"]),
        binding(
            "gateway-retry-dead-letter",
            "/gateway-retry-dead-letter --json",
            &["gateway"],
        ),
        binding(
            "multi-agent-runtime",
            "/multi-agent-runtime --agents 4 --messages 8 --json",
            &["multi-agent", "operator"],
        ),
        binding(
            "apply-task-patches",
            "/apply-task-patches <task_id> --json",
            &["diff", "handoff"],
        ),
        binding(
            "rollback-task-patches",
            "/rollback-task-patches <task_id> --json",
            &["diff", "handoff"],
        ),
        binding(
            "ui-readonly-command-runner",
            "POST /api/commands/<id>",
            &["developer", "commands", "operator"],
        ),
    ]
}

pub fn control_ui_interaction_capabilities() -> Vec<ControlUiInteractionCapability> {
    vec![
        local_interaction(
            "native-anchor-navigation",
            "Native anchor navigation",
            "browser-native",
            "hash anchors keep the 26-screen route catalog reachable without JavaScript",
        ),
        local_interaction(
            "native-popovers",
            "Native popovers",
            "browser-native",
            "eight popover=auto surfaces with popovertarget controls",
        ),
        local_interaction(
            "chat-list-search",
            "Seeded chat-list search",
            "local-only",
            "control-ui.js filters seeded .tg-chat-item nodes without network access",
        ),
        local_interaction(
            "command-palette-search",
            "Command palette search",
            "local-only",
            "control-ui.js filters native-popover command links without network access",
        ),
        local_interaction(
            "composer-picker-search",
            "Composer picker search",
            "local-only",
            "control-ui.js filters local context and command draft options",
        ),
        local_interaction(
            "local-draft-insertion",
            "Local draft insertion",
            "local-only",
            "artifact and command picker items update only the local textarea; send and plan stay disabled",
        ),
        local_interaction(
            "local-json-inspector",
            "Local JSON inspector",
            "local-only",
            "JSON.parse plus textContent formatting; pasted data is not uploaded",
        ),
        local_interaction(
            "clipboard-copy",
            "Clipboard copy",
            "local-only",
            "bounded data-copy handler with clipboard API and local textarea fallback",
        ),
        local_interaction(
            "same-origin-readonly-command-runner",
            "Same-origin read-only command runner",
            "same-origin-read-only",
            "fixed 21-route GET allowlist, canonical path validation, bounded JSON responses, and no POST",
        ),
        local_interaction(
            "operator-snapshot-read",
            "Operator snapshot read",
            "same-origin-read-only",
            "GET /api/operator-snapshot hydrates a status-only local summary",
        ),
    ]
}

fn local_interaction(
    id: &'static str,
    title: &'static str,
    availability: &'static str,
    evidence: &'static str,
) -> ControlUiInteractionCapability {
    let evidence_kind = match availability {
        "browser-native" => "browser_native_dom",
        "same-origin-read-only" => "bounded_same_origin_get",
        _ => "local_dom_handler",
    };
    ControlUiInteractionCapability {
        id,
        title,
        implemented: true,
        availability,
        requires_live_adapter: false,
        evidence_kind,
        evidence,
    }
}

fn binding(
    id: &'static str,
    command: &'static str,
    used_by: &'static [&'static str],
) -> ControlUiCommandBinding {
    ControlUiCommandBinding {
        id,
        command,
        used_by,
    }
}

fn percent(numerator: usize, denominator: usize) -> u8 {
    if denominator == 0 {
        return 0;
    }
    ((numerator * 100) / denominator) as u8
}

fn control_ui_evidence_coverage(
    static_contract_percent: u8,
    static_contract_verified: bool,
) -> ControlUiEvidenceCoverage {
    let static_contract = ControlUiEvidenceLayer {
        status: if static_contract_verified {
            "verified"
        } else {
            "incomplete"
        },
        coverage_percent: static_contract_percent,
        verified: static_contract_verified,
        evidence_ref: Some(
            "hepta-core::control_ui typed local-read-only capability manifest, asset/schema checks, no-JavaScript anchor navigation, local DOM handlers, and bounded same-origin GET contract",
        ),
    };
    let unit_state = unevidenced_control_ui_layer();
    let browser_behavior = unevidenced_control_ui_layer();
    let backend_mutation_readback = unevidenced_control_ui_layer();
    let live_adapter = unevidenced_control_ui_layer();
    let layers = [
        &static_contract,
        &unit_state,
        &browser_behavior,
        &backend_mutation_readback,
        &live_adapter,
    ];
    let all_required_layers_verified = layers.iter().all(|layer| layer.complete());
    let overall_evidence_percent = percent(
        layers
            .iter()
            .map(|layer| layer.coverage_percent as usize)
            .sum(),
        layers.len() * 100,
    );

    ControlUiEvidenceCoverage {
        schema_version: 1,
        static_contract,
        unit_state,
        browser_behavior,
        backend_mutation_readback,
        live_adapter,
        overall_evidence_percent,
        all_required_layers_verified,
        boundary: "The typed source manifest proves only the local read-only contract. Unit/state runs and real browser behavior must be bound explicitly; backend mutation/readback and live-adapter coverage remain zero until those capabilities exist and are verified.",
    }
}

fn unevidenced_control_ui_layer() -> ControlUiEvidenceLayer {
    ControlUiEvidenceLayer {
        status: "not_bound_to_report",
        coverage_percent: 0,
        verified: false,
        evidence_ref: None,
    }
}

fn convergence_lane(
    id: &'static str,
    title: &'static str,
    checks: &[bool],
) -> ControlUiConvergenceLane {
    let passed_count = checks.iter().filter(|ready| **ready).count();
    let check_count = checks.len();
    let percent = percent(passed_count, check_count);
    ControlUiConvergenceLane {
        id,
        title,
        ready: check_count > 0 && passed_count == check_count,
        passed_count,
        check_count,
        percent,
    }
}

fn parse_app_attribute_values(attribute: &str) -> Vec<String> {
    let marker = format!("{attribute}=\"");
    let mut remainder = CONTROL_UI_INDEX_HTML;
    let mut values = Vec::new();
    while let Some(marker_offset) = remainder.find(&marker) {
        let value_start = marker_offset + marker.len();
        let after_marker = &remainder[value_start..];
        let Some(value_end) = after_marker.find('"') else {
            break;
        };
        let value = &after_marker[..value_end];
        if !value.is_empty() {
            values.push(value.to_string());
        }
        remainder = &after_marker[value_end + 1..];
    }
    values.sort();
    values.dedup();
    values
}

fn parse_app_screen_ids() -> Vec<String> {
    parse_app_attribute_values("data-screen")
}

fn parse_app_command_ids() -> Vec<String> {
    let Some(source) = std::str::from_utf8(CONTROL_UI_JS).ok() else {
        return Vec::new();
    };
    let Some((_, after_start)) = source.split_once("const COMMAND_CATALOG = Object.freeze([")
    else {
        return Vec::new();
    };
    let Some((catalog, _)) = after_start.split_once("].map(") else {
        return Vec::new();
    };
    let mut ids = catalog
        .lines()
        .filter_map(|line| {
            let entry = line.trim().strip_prefix("[\"")?;
            let end = entry.find('"')?;
            let id = &entry[..end];
            (!id.is_empty()
                && id
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'))
            .then(|| id.to_string())
        })
        .collect::<Vec<_>>();
    ids.sort();
    ids.dedup();
    ids
}

fn same_unique_ids(left: &[String], right: &[String]) -> bool {
    let mut left = left.to_vec();
    left.sort();
    left.dedup();
    let mut right = right.to_vec();
    right.sort();
    right.dedup();
    left == right
}

fn parse_readme_screen_ids() -> Vec<String> {
    CONTROL_UI_README
        .lines()
        .skip_while(|line| line.trim() != "## Screens")
        .skip(1)
        .take_while(|line| !line.starts_with("## Run locally"))
        .filter_map(|line| line.trim().strip_prefix("- "))
        .map(screen_title_to_id)
        .collect()
}

fn screen_title_to_id(title: &str) -> String {
    match title.trim() {
        "Config Surface" => "config".into(),
        "Task Publisher" => "task-publisher".into(),
        "Operator Console" => "operator".into(),
        "Live Event Stream" => "live".into(),
        "Conversation Transcript" => "transcript".into(),
        "Agent Chat" => "chat".into(),
        "Diff Review" => "diff".into(),
        "Operator Security" => "security".into(),
        "Gateway Monitor" => "gateway".into(),
        "Hepta Operator Plane" => "runtime-control-plane".into(),
        "Multi-Agent Runtime" => "multi-agent".into(),
        "Developer Console" => "developer".into(),
        "Worker Handoff" => "handoff".into(),
        "Ops Status" => "ops".into(),
        "Production Parity" => "parity".into(),
        other => other.trim().to_ascii_lowercase().replace(' ', "-"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_ui_report_is_complete_and_asset_backed() {
        let report = control_ui_report();
        let rust_frontend_html = control_ui_index_html();

        assert_eq!(report.status, "static_contract_complete");
        assert_eq!(report.screen_count, 26);
        assert_eq!(report.implemented_screen_count, 26);
        assert_eq!(report.screen_coverage_percent, 100);
        assert_eq!(
            report.screen_coverage_percent_basis,
            "static local read-only screen shells; not live adapter or mutation coverage"
        );
        assert_eq!(report.live_implemented_screen_count, 0);
        assert!(report.screens.iter().all(|screen| {
            screen.implemented
                && screen.implementation_scope == "static-local-read-only-screen-shell"
                && !screen.live_adapter_ready
        }));
        assert_eq!(report.asset_count, 5);
        assert_eq!(report.asset_coverage_percent, 100);
        assert_eq!(report.command_binding_count, 51);
        assert!(same_unique_ids(
            &control_ui_screens()
                .iter()
                .map(|screen| screen.id.to_string())
                .collect::<Vec<_>>(),
            &parse_app_screen_ids(),
        ));
        assert!(same_unique_ids(
            &control_ui_command_bindings()
                .iter()
                .map(|binding| binding.id.to_string())
                .collect::<Vec<_>>(),
            &parse_app_command_ids(),
        ));

        assert_eq!(report.interaction_capability_count, 10);
        assert_eq!(report.implemented_interaction_capability_count, 10);
        assert_eq!(report.capability_manifest_schema_version, 2);
        assert_eq!(report.capability_mode, "local-read-only");
        assert!(!report.live_adapter_bound);
        assert_eq!(report.static_interaction_contract_percent, 100);
        assert_eq!(report.live_operator_surface_percent, 0);
        assert_eq!(report.developer_interaction_percent, 100);
        assert_eq!(
            report.developer_interaction_percent_basis,
            "compatibility alias for static_interaction_contract_percent; not browser or live execution evidence"
        );
        assert_eq!(report.ref_agent_alignment_percent, 100);
        assert!(report.local_preview_ready);
        assert!(report.static_contract_complete());
        assert!(!report.complete());
        assert!(report.interaction_capabilities.iter().all(|capability| {
            capability.implemented
                && !capability.requires_live_adapter
                && matches!(
                    capability.availability,
                    "browser-native" | "local-only" | "same-origin-read-only"
                )
                && capability.evidence_kind != "static_contract_marker"
        }));

        assert_eq!(report.evidence_coverage.schema_version, 1);
        assert!(report.evidence_coverage.static_contract.verified);
        assert_eq!(
            report.evidence_coverage.static_contract.coverage_percent,
            100
        );
        assert_eq!(report.evidence_coverage.overall_evidence_percent, 20);
        assert!(!report.evidence_coverage.all_required_layers_verified);
        for layer in [
            &report.evidence_coverage.unit_state,
            &report.evidence_coverage.browser_behavior,
            &report.evidence_coverage.backend_mutation_readback,
            &report.evidence_coverage.live_adapter,
        ] {
            assert_eq!(layer.status, "not_bound_to_report");
            assert_eq!(layer.coverage_percent, 0);
            assert!(!layer.verified);
            assert_eq!(layer.evidence_ref, None);
        }

        for (schema_json, route, source_command, compatibility_mode, control_surface) in [
            (
                CONTROL_UI_SCHEMA_CONTROL_JSON,
                "/api/control-ui",
                "/control-ui --json",
                "native_control_ui_shell_snapshot",
                "control_ui",
            ),
            (
                CONTROL_UI_SCHEMA_AUDIT_JSON,
                "/api/ui-contract-audit",
                "/ui-contract-audit --json",
                "native_ui_contract_audit",
                "ui_contract_audit",
            ),
        ] {
            let schema: serde_json::Value =
                serde_json::from_str(schema_json).expect("parse Control UI HTTP report schema");
            assert_eq!(
                schema
                    .pointer("/oneOf/0/$ref")
                    .and_then(serde_json::Value::as_str),
                Some("#/$defs/summaryEnvelope")
            );
            assert_eq!(
                schema
                    .pointer("/oneOf/1/$ref")
                    .and_then(serde_json::Value::as_str),
                Some("#/$defs/pageEnvelope")
            );
            assert_eq!(
                schema
                    .pointer("/$defs/summaryEnvelope/properties/route/const")
                    .and_then(serde_json::Value::as_str),
                Some(route)
            );
            assert_eq!(
                schema
                    .pointer("/$defs/pageEnvelope/properties/route/const")
                    .and_then(serde_json::Value::as_str),
                Some(route)
            );
            assert_eq!(
                schema
                    .pointer("/$defs/sourcePayload/properties/source_command/const")
                    .and_then(serde_json::Value::as_str),
                Some(source_command)
            );
            assert_eq!(
                schema
                    .pointer("/$defs/sourcePayload/properties/compatibility_mode/const")
                    .and_then(serde_json::Value::as_str),
                Some(compatibility_mode)
            );
            assert_eq!(
                schema
                    .pointer("/$defs/sourcePayload/properties/control_surface/const")
                    .and_then(serde_json::Value::as_str),
                Some(control_surface)
            );
            assert_eq!(
                schema
                    .pointer("/$defs/sourcePayload/properties/control_ui_product_complete/const")
                    .and_then(serde_json::Value::as_bool),
                Some(false)
            );
            assert_eq!(
                schema
                    .pointer("/$defs/sourcePayload/properties/control_ui_live_operator_surface_percent/const")
                    .and_then(serde_json::Value::as_u64),
                Some(0)
            );
            assert_eq!(
                schema
                    .pointer("/$defs/evidenceCoverage/properties/overall_evidence_percent/const")
                    .and_then(serde_json::Value::as_u64),
                Some(20)
            );
            assert_eq!(
                schema
                    .pointer(
                        "/$defs/evidenceCoverage/properties/all_required_layers_verified/const",
                    )
                    .and_then(serde_json::Value::as_bool),
                Some(false)
            );
        }
        assert!(report.frontend_manifest.rust_view_model_ready);
        assert_eq!(
            report.frontend_manifest.source,
            "apps/hepta-control-ui/index.html"
        );
        assert_eq!(
            report
                .rust_frontend_ownership
                .rust_embedded_static_asset_coverage_percent,
            100
        );
        assert_eq!(
            report.rust_frontend_ownership.status,
            "rust-embedded-progressive-frontend"
        );
        assert!(!report.rust_frontend_ownership.pure_browser_rust_runtime);
        assert_eq!(rust_frontend_html, CONTROL_UI_INDEX_HTML);

        for marker in [
            "data-view=\"chat\"",
            "telegram-chat-shell",
            "data-control-ui-capability-mode=\"local-read-only\"",
            "data-control-ui-live-adapter-bound=\"false\"",
            "<span>route catalog</span><strong>26/26</strong>",
            "<span>live adapter</span><strong>0</strong>",
            "documented, not live workflows",
            "data-live-event-stream=\"false\"",
            "data-task-publisher=\"false\"",
            "data-agent-chat=\"false\"",
            "data-command-runner=\"same-origin-read-only\"",
            "data-route-view-controller=\"native-anchor-routes\"",
            "data-command-palette=\"native-popover-filter\"",
            "<script defer src=\"./control-ui.js\"></script>",
        ] {
            assert!(
                rust_frontend_html.contains(marker),
                "Control UI missing local read-only truth marker: {marker}"
            );
        }
        assert!(!rust_frontend_html.contains("<script>"));
        assert!(!rust_frontend_html.contains("<span>workflows</span>"));
        assert!(!rust_frontend_html.contains("<p>ready locally</p>"));
        assert!(CONTROL_UI_STYLES_CSS.contains("#command-palette:popover-open"));
        assert!(CONTROL_UI_STYLES_CSS.contains("#command-palette:not(:popover-open)"));

        let capability_contract: serde_json::Value =
            serde_json::from_str(CONTROL_UI_RUST_RENDERER_MARKERS)
                .expect("parse serialized Control UI security compatibility contract");
        assert_eq!(
            capability_contract
                .pointer("/live_adapter_bound")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
        assert!(!CONTROL_UI_RUST_RENDERER_MARKERS.contains("submitAgentChat"));
        assert!(!CONTROL_UI_RUST_RENDERER_MARKERS.contains("togglePinnedConversation"));

        let audit = control_ui_contract_audit_report();
        assert_eq!(
            audit.status,
            "static_contract_complete",
            "{}",
            serde_json::to_string_pretty(&audit).expect("serialize control UI audit")
        );
        assert_eq!(
            audit.evidence_scope,
            "typed local read-only capability manifest plus DOM and bounded JavaScript evidence; no live workflow claim"
        );
        assert!(!audit.live_product_complete);
        assert_eq!(audit.audit_percent, 100);
        assert_eq!(audit.core_screen_count, 26);
        assert_eq!(audit.app_screen_count, 26);
        assert_eq!(audit.readme_screen_count, 26);
        assert_eq!(audit.interaction_capability_count, 10);
        assert_eq!(audit.capability_manifest_schema_version, 2);
        assert_eq!(audit.capability_mode, "local-read-only");
        assert!(!audit.live_adapter_bound);
        assert!(audit.all_screen_ids_aligned);
        assert!(audit.app_has_json_inspector);
        assert!(audit.app_has_readonly_command_runner);
        assert!(audit.app_has_operator_security);
        assert!(audit.app_has_hepta_runtime_navigation_groups);
        assert!(audit.app_has_chat_first_architecture);
        assert!(audit.app_has_route_view_controller);
        assert!(audit.app_has_command_palette);
        assert!(audit.app_has_premium_consumer_ui);
        assert!(audit.app_has_progressive_disclosure);
        assert!(audit.app_has_simplified_primary_nav);
        assert!(audit.app_has_minimal_consumer_workspace);
        assert!(audit.app_has_grouped_message_width_guard);
        assert!(audit.app_has_ios_pwa_bounds_guard);
        assert!(audit.app_has_selection_contrast_guard);
        assert!(audit.app_distinguishes_sample_vs_live_adapter_readiness);
        assert!(!audit.app_has_live_event_stream);
        assert!(!audit.app_has_task_publisher);
        assert!(!audit.app_has_agent_chat);
        assert!(!audit.control_ui_gateway_websocket_opened_by_audit);
        assert!(!audit.control_ui_live_gateway_rpc_performed);
        assert!(!audit.p0_p39_converged);
        assert_eq!(audit.convergence_percent, 100);
        assert_eq!(audit.convergence_lanes.len(), 10);
        assert!(
            audit
                .convergence_lanes
                .iter()
                .all(|lane| lane.ready && lane.percent == 100)
        );
        assert!(audit.missing_in_app.is_empty());
        assert!(audit.missing_in_readme.is_empty());
    }

    #[test]
    fn control_ui_native_popover_and_accessibility_contract_is_current() {
        let html = control_ui_index_html();

        assert_eq!(html.matches("popover=\"auto\"").count(), 8);
        assert!(html.matches("popovertarget=\"").count() >= 9);
        assert!(!html.contains("href=\"#command-palette\""));
        assert!(!html.contains("role=\"menu\""));
        assert!(!html.contains("role=\"menuitem\""));
        assert!(html.contains("role=\"group\""));
        assert!(html.contains("dir=\"auto\""));
        assert!(html.contains("data-theme-mode=\"light\""));
        assert!(html.contains("data-chat-mobile-pane-tabs=\"native-anchor\""));
        assert!(html.contains("data-chat-mobile-pane=\"chats\""));
        assert!(html.contains("data-chat-mobile-pane=\"thread\""));
        assert!(html.contains("data-chat-mobile-pane=\"room\""));
        assert_eq!(
            html.matches("tabindex=\"-1\" data-chat-mobile-pane=")
                .count(),
            3
        );
        assert!(html.contains("<link rel=\"icon\" href=\"data:,\" />"));
        assert!(CONTROL_UI_STYLES_CSS.contains("prefers-contrast:more"));
        assert!(CONTROL_UI_STYLES_CSS.contains("forced-colors:active"));
        assert!(CONTROL_UI_STYLES_CSS.contains("prefers-reduced-transparency:reduce"));
        assert!(CONTROL_UI_STYLES_CSS.contains("prefers-reduced-motion:reduce"));
    }
}
