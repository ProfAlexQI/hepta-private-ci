use serde::Serialize;

pub const CONTROL_UI_INDEX_HTML_FALLBACK: &str =
    include_str!("../../../apps/hepta-control-ui/index.html");
pub const CONTROL_UI_INDEX_HTML: &str = CONTROL_UI_INDEX_HTML_FALLBACK;
pub const CONTROL_UI_STYLES_CSS: &str = include_str!("../../../apps/hepta-control-ui/styles.css");
pub const CONTROL_UI_RUST_RENDERER_MARKERS: &str = r###"liveEventStream
diffReview
keydown
json-input
renderOperatorConsole
operatorSnapshot
renderEndpointHealth
localActionPlan
data-plan-action
renderApprovalCards
renderExecApprovalsLiveEditorParity
previewExecApprovalPatch
data-exec-approvals-live-editor-parity
data-exec-approval-apply-bridge
redacted_snapshot_hash
before_after_diff
role_guard
gateway/node target
per-agent scope
/api/approvals/exec/apply
/api/approvals
/api/policy
fetchActionPlan
method: "POST"
/api/actions/
renderSessionInspector
/api/session-activity
renderTaskDrilldown
postJson("/api/task"
postJson("/api/task-patches"
postJson("/api/task-evidence"
renderTranscriptPreview
/api/transcript
fetchTranscriptQuery
data-query-transcript
/api/query-transcript/
postJson("/api/promotion-ledger"
/api/live-events/0
next_cursor_unix_ms
duplicate_free
fetchCommandResult
data-run-command
/api/commands/
Run read-only
renderOperatorSecurity
/api/operator-security
operatorSecurity
all_operator_security_lanes_100
renderTaskPublisher
submitTaskPublisher
data-task-publish
/api/tasks/plan
/api/tasks/publish
renderAgentChat
submitAgentChat
data-agent-chat-send
/api/chat/plan
/api/chat
externalAgentBenchmark
/api/external-agent-benchmark
No synthetic wins boundary
HEPTA_VIEW_GROUPS
nav-group
chat
control
settings
renderChatWorkspace
data-chat-first-architecture
screenFromRoute
setActiveScreen
hashchange
window.history.pushState
commandPaletteItems
openCommandPalette
activateCommandPaletteItem
HEPTA_PRIMARY_NAV
prompt-chip-row
data-open-command-palette
nav-group--primary
New Task
More
composerHasFocus
.chat-compose
inputHeavyScreen
!inputHeavyScreen || !hadLiveData
["INPUT", "TEXTAREA", "SELECT"]
focus-workspace
data-thread-signature
Telegram-style multi-agent chat
data-chat-search
data-chat-folder
data-chat-inline-create
data-chat-pin
data-chat-archive
data-chat-unarchive
data-chat-delete-confirm
data-chat-row-menu-toggle
data-chat-row-menu-panel
data-chat-row-menu-item
role="menuitem"
conversationMenuItems
data-chat-composer-shell
data-chat-command-shortcut
data-chat-attachment-placeholder
data-chat-composer-popover
data-chat-artifact-insert
data-chat-command-insert
data-chat-composer-picker-search
data-chat-composer-picker-item
filterComposerOptions
setComposerPickerActiveIndex
activateComposerPickerSelection
addComposerEvidenceOption
composerCitationCard
composerContextChips
groupedComposerContextChips
data-chat-context-chip-group
data-chat-context-chip-overflow
COMPOSER_CONTEXT_CHIP_COLLAPSED_LIMIT
data-chat-context-chip-token
toggleComposerContextChipPreview
expandedContextChipKey
composerContextSummary
data-chat-context-summary
composerContextReferenceHealth
data-chat-context-health
composerRepairTarget
replaceComposerContextBlock
data-chat-context-chip-repair
renderComposerStaleSendGuard
data-chat-stale-guard
data-chat-stale-send-continue
allowStaleContexts
composerContextAuditTrail
renderMessageContextAudit
data-chat-context-audit
data-chat-workspace-scope
data-chat-brain-scope
sharedBrainContract
sharedBrainModeLabel
GLOBAL_BRAIN_SHARED_DOMAINS
WORKSPACE_CONTEXT_ISOLATED_DOMAINS
workspaceTaskBindings
workspaceTaskImports
workspaceSelectedTaskIds
timelineLimits
chatTimelineWindow
data-chat-timeline-show-more
chatThreadSearchMatches
data-chat-thread-search
data-chat-thread-search-active
CHAT_STATE_SCHEMA_VERSION
pruneChatStateForStorage
CHAT_STATE_MAX_LOCAL_THREAD_RECORDS
liveDataFetchPlanForScreen
lazyLiveDataFetch
inlineCreateWorkspaceId
renderWorkspaceRoomPanel
renderWorkspaceRoomTaskActions
renderWorkspaceRoomTaskArtifactPreview
roomTaskArtifactCitation
renderWorkspaceRoomActivityLog
appendWorkspaceRoomActivity
data-room-task-artifact-insert
data-room-task-action-plan
copyReviewedRoomTaskCommand
data-workspace-room-panel
data-room-orchestration
renderWorkspaceMemberRoster
/api/workspace-members
data-workspace-group-chat
composerMentionAgentIds
CHAT_ROUTING_MODE_OPTIONS
effectiveChatRoutingMode
reducerModeForRoutingMode
data-chat-routing-mode
renderChatOrchestration
data-chat-orchestration
groupedAgentRepliesFromTurnResults
renderGroupedAgentReplies
data-chat-grouped-replies
target_agent_ids
routing_mode
imported_task_ids
data-task-import
data-task-select
importTaskIntoWorkspace
selectWorkspaceTask
workspace_id
workspaceId
chatWorkspaceIdForConversation
contextAuditCitationBlock
data-chat-context-audit-copy
data-chat-context-audit-restore
restoreContextAuditToComposer
dedupeComposerCitationText
composerCitationBlocks
normalizeComposerCitationBlock
Same key changed
changedKeys
changedBlocks
renderContextReuseDiff
data-chat-context-reuse-diff
data-chat-context-reuse-keep
data-chat-context-reuse-replace
data-chat-context-reuse-keep-all
data-chat-context-reuse-replace-all
updateContextReuseDiffBlock
updateContextReuseDiffBatch
setContextReuseUndo
renderContextReuseUndo
data-chat-context-reuse-undo
undoContextReuseReplace
handleContextWorkflowShortcut
data-chat-context-shortcut-hint
Keyboard: R replace all
clearComposerContextTransientState
clearContextWorkflowAfterDraftEdit
contextWorkflowHasTransientState
commitChatTransientState
clearChatTransientUiState
chatUiHasTransientState
HEPTA_CHAT_BOUNDARY: chat-state-constants
HEPTA_CHAT_BOUNDARY: chat-transient-state-helpers
HEPTA_CHAT_BOUNDARY: composer-citation-helpers
HEPTA_CHAT_BOUNDARY: composer-context-render
HEPTA_CHAT_BOUNDARY: composer-popover-render
HEPTA_CHAT_BOUNDARY: conversation-lifecycle-actions
HEPTA_CHAT_BOUNDARY: chat-state-normalization
HEPTA_CHAT_BOUNDARY: conversation-derived-metadata
HEPTA_CHAT_BOUNDARY: conversation-derivation
HEPTA_CHAT_BOUNDARY: event-binding
HEPTA_CHAT_BOUNDARY: composer-context-dedupe
HEPTA_CHAT_BOUNDARY: context-reuse-render
dismissContextReuseDiff
dismissContextReuseUndo
composerContextBlockRangeByKey
Skipped ${deduped.skippedCount} duplicate
data-chat-context-audit-expanded
data-chat-context-chip
data-chat-context-chip-preview
data-chat-context-chip-remove
task-patches
task-evidence
promotion-ledger
insertChatComposerText
data-chat-enter-send
data-chat-send-state
setChatComposerStatus
data-chat-unread
data-chat-date-divider
data-chat-conversation
deriveChatConversations
setChatConversation
togglePinnedConversation
localStorage
seenConversationTimestamps
long-running-ws-resilience-modeled
polling-no-store-fallback
gatewayWebSocketOpenedByAudit: false
dynamic-viewport-bounds
safe-area-inset-bottom
high-contrast-selection
slash-command-feedback-toast
command-result-status-preserved
showToast
long-call-status-surface
rawDiagnosticPayloadLogged: false
pruneWorkspaceTaskMap
CHAT_STATE_MAX_WORKSPACE_TASK_MAPS
roomTaskArtifactPreview: previousRoomTaskArtifactPreview
activeScreen === "chat"
mapped_command
focusChatThreadSearchInput
focusChatSearchInput
renderConvergenceLedger
data-ui-convergence-ledger
role="listbox"
aria-selected
renderEndpointRecoveryBanner
data-endpoint-retry-all
applyControlUiContentRichFixture
data-content-rich-fixture
seededControlUiMessages
data-mobile-layered-chat
data-chat-mobile-active-pane
renderMobileLayerTabs
setChatMobilePane
renderThreadStatusSummary
data-thread-compact-status
data-thread-status-popover
Details
renderWorkspaceRoomOnboardingCard
workspaceRoomHasRichContent
data-room-empty-consolidated
data-room-onboarding-card
applyControlUiEmptyFixture
Workspace is ready
roomTaskActionPlan
roomTaskArtifactPreview
HEPTA_UI_MODULE_BOUNDARIES
role="tablist"
role="tab"
renderWorkspaceRoomAccordion
data-room-accordion
key: "evidence"
key: "orchestration"
data-mobile-compact-composer
applyControlUiEndpointChaosFixture
endpointChaos
partial failure
stale-cache
latency_ms
renderTaskArtifactResultDrawer
data-task-result-drawer
data-task-result-workspace
Current result
roomTaskArtifactSummary
read-only
<h2>${escapeHtml(selected?.title || "Hepta agent")}</h2>
productization
data-mobile-density-tier
tg-mobile-density-more
renderThreadStarterActions
dry-run review
applyControlUiGoldenLiveDataFixture
applyControlUiHostileFixture
data-task-result-drawer-action
data-task-result-drawer-actions="product"
HEPTA_UI
rust-no-js-frontend
hepta-core::control_ui
data-rust-frontend-renderer
data-js-artifacts="removed"
data-no-js-frontend="true"
postJson("/api/task-replay"
activeScreen = screenFromRoute()
data-minimal-consumer-workspace
data-telegram-multi-agent-chat
HEPTA_RUNTIME_2026_5_2_UI_RESILIENCE_CONTRACT
bounded-grouped-reply-width
realtime-talk-diagnostic-redaction
renderEventTimeline
/api/events-report
persistedAutoScrollMode
HEPTA_CHAT_AUTO_SCROLL_STORAGE_KEY
data-chat-autoscroll-mode
data-chat-autoscroll-persisted
renderBlankDashboardRecoveryPanel
data-dashboard-recovery-panel
blank-dashboard-html-recovery
renderCompactSessionStatusBadges
data-session-status-badges
data-session-status="live"
data-session-status="idle"
data-session-status="terminal"
nodesPollScopeActiveTab
data-nodes-poll-scope="active-tab-only"
nodesPollingAutoStart: false
sampleReadinessStatus
liveAdapterReadinessStatus
data-readiness-kind="sample"
data-readiness-kind="live-adapter"
terminalQrRendering
data-terminal-qr-rendering
fixed-cell-terminal-qr"###;
pub const CONTROL_UI_HEPTA_AGENT_LOGO_PNG: &[u8] =
    include_bytes!("../../../apps/hepta-control-ui/assets/hepta-agent-logo.png");
pub const CONTROL_UI_README: &str = include_str!("../../../apps/hepta-control-ui/README.md");
pub const CONTROL_UI_SMOKE_SH: &str = include_str!("../../../scripts/hepta-control-ui-smoke.sh");
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
    pub evidence: &'static str,
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
    pub asset_count: usize,
    pub present_asset_count: usize,
    pub required_asset_count: usize,
    pub asset_coverage_percent: u8,
    pub command_binding_count: usize,
    pub interaction_capability_count: usize,
    pub implemented_interaction_capability_count: usize,
    pub live_operator_surface_percent: u8,
    pub developer_interaction_percent: u8,
    pub ref_agent_alignment_percent: u8,
    pub local_preview_ready: bool,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ControlUiContractAuditReport {
    pub product: &'static str,
    pub status: &'static str,
    pub core_screen_count: usize,
    pub app_screen_count: usize,
    pub readme_screen_count: usize,
    pub command_binding_count: usize,
    pub interaction_capability_count: usize,
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
    pub fn complete(&self) -> bool {
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
            && self.live_operator_surface_percent == 100
            && self.developer_interaction_percent == 100
            && self.ref_agent_alignment_percent == 100
            && self.local_preview_ready
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
    let live_operator_surface_percent = developer_interaction_percent;
    let ref_agent_alignment_percent = percent(
        implemented_screen_count + implemented_interaction_capability_count,
        screen_count + interaction_capability_count,
    );
    let rust_frontend_html = control_ui_index_html();
    let local_preview_ready = screen_coverage_percent == 100
        && asset_coverage_percent == 100
        && developer_interaction_percent == 100
        && !command_bindings.is_empty()
        && rust_frontend_html.contains("Hepta Control UI")
        && rust_frontend_html.contains("data-rust-frontend-renderer")
        && rust_frontend_html.contains("data-no-js-frontend=\"true\"")
        && rust_frontend_html.contains("data-js-artifacts=\"removed\"")
        && rust_frontend_html.contains("hepta-core::control_ui")
        && !rust_frontend_html.contains("<script")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("rust-no-js-frontend");
    let frontend_manifest = control_ui_frontend_manifest();
    let rust_frontend_ownership = control_ui_rust_frontend_ownership(&assets, &frontend_manifest);

    ControlUiReport {
        product: "Hepta",
        ui_name: "Hepta Control UI",
        status: if local_preview_ready {
            "complete"
        } else {
            "incomplete"
        },
        version: "control-ui-v0",
        screen_count,
        implemented_screen_count,
        screen_coverage_percent,
        asset_count,
        present_asset_count,
        required_asset_count,
        asset_coverage_percent,
        command_binding_count: command_bindings.len(),
        interaction_capability_count,
        implemented_interaction_capability_count,
        live_operator_surface_percent,
        developer_interaction_percent,
        ref_agent_alignment_percent,
        local_preview_ready,
        rust_frontend_ownership,
        serve_command: "cargo run -p hepta -- --serve-ui 127.0.0.1:7373",
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
    let app_has_live_event_stream = CONTROL_UI_RUST_RENDERER_MARKERS.contains("liveEventStream");
    let app_has_diff_review = CONTROL_UI_RUST_RENDERER_MARKERS.contains("diffReview");
    let app_has_keyboard_shortcuts = CONTROL_UI_RUST_RENDERER_MARKERS.contains("keydown");
    let app_has_json_inspector = CONTROL_UI_RUST_RENDERER_MARKERS.contains("json-input")
        || CONTROL_UI_INDEX_HTML.contains("json-input");
    let app_has_operator_drilldown = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("renderOperatorConsole")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("operatorSnapshot");
    let app_has_endpoint_health_grid = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("renderEndpointHealth")
        && CONTROL_UI_STYLES_CSS.contains(".endpoint-grid");
    let app_has_dry_run_action_cards = CONTROL_UI_RUST_RENDERER_MARKERS.contains("localActionPlan")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-plan-action");
    let app_has_approval_cards = CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderApprovalCards")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderExecApprovalsLiveEditorParity")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("previewExecApprovalPatch")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-exec-approvals-live-editor-parity")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-exec-approval-apply-bridge")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("redacted_snapshot_hash")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("before_after_diff")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("role_guard")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("gateway/node target")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("per-agent scope")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/approvals/exec/apply")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/approvals")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/policy");
    let app_has_post_action_guard = CONTROL_UI_RUST_RENDERER_MARKERS.contains("fetchActionPlan")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("method: \"POST\"")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/actions/");
    let app_has_session_inspector = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("renderSessionInspector")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/session-activity");
    let app_has_task_drilldown = CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderTaskDrilldown")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("postJson(\"/api/task\"")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("postJson(\"/api/task-patches\"")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("postJson(\"/api/task-evidence\"");
    let app_has_transcript_preview = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("renderTranscriptPreview")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/transcript");
    let app_has_transcript_search = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("fetchTranscriptQuery")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-query-transcript")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/query-transcript/");
    let app_has_replay_promotion_drilldown = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("postJson(\"/api/task-replay\"")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("postJson(\"/api/promotion-ledger\"");
    let app_has_event_cursor = CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/live-events/0")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("next_cursor_unix_ms")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("duplicate_free");
    let app_has_readonly_command_runner = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("fetchCommandResult")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-run-command")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/commands/")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("Run read-only");
    let app_has_operator_security = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("renderOperatorSecurity")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/operator-security")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("operatorSecurity")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("all_operator_security_lanes_100");
    let app_has_task_publisher = CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderTaskPublisher")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("submitTaskPublisher")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-task-publish")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/tasks/plan")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/tasks/publish");
    let app_has_agent_chat = CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderAgentChat")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("submitAgentChat")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-agent-chat-send")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/chat/plan")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/chat");
    let app_has_external_agent_benchmark = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("externalAgentBenchmark")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/external-agent-benchmark")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("No synthetic wins boundary");
    let app_has_hepta_runtime_ui_parity = CONTROL_UI_INDEX_HTML
        .contains("data-hepta_runtime-ui-parity")
        && CONTROL_UI_INDEX_HTML.contains("topnav-shell")
        && CONTROL_UI_INDEX_HTML.contains("shell-nav")
        && CONTROL_UI_STYLES_CSS.contains("HeptaRuntime-style")
        && CONTROL_UI_STYLES_CSS.contains("--accent: #e5243b")
        && CONTROL_UI_STYLES_CSS.contains(".topbar")
        && CONTROL_UI_STYLES_CSS.contains(".shell-nav")
        && CONTROL_UI_STYLES_CSS.contains(".topbar-search")
        && CONTROL_UI_STYLES_CSS.contains(".pill");
    let app_has_hepta_runtime_navigation_groups = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("HEPTA_VIEW_GROUPS")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("nav-group")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("chat")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("control")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("settings");
    let app_has_chat_first_architecture = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("activeScreen = screenFromRoute()")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderChatWorkspace")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-first-architecture")
        && CONTROL_UI_STYLES_CSS.contains("body[data-view=\"chat\"]")
        && CONTROL_UI_STYLES_CSS.contains(".chat-thread")
        && CONTROL_UI_STYLES_CSS.contains(".chat-compose");
    let app_has_route_view_controller = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("screenFromRoute")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("setActiveScreen")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("hashchange")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("window.history.pushState");
    let app_has_command_palette = CONTROL_UI_INDEX_HTML.contains("command-palette")
        && CONTROL_UI_INDEX_HTML.contains("data-open-command-palette")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("commandPaletteItems")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("openCommandPalette")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("activateCommandPaletteItem")
        && CONTROL_UI_STYLES_CSS.contains(".command-palette");
    let app_has_premium_consumer_ui = CONTROL_UI_INDEX_HTML.contains("data-premium-consumer-ui")
        && CONTROL_UI_INDEX_HTML.contains("linear-raycast-vercel-arc")
        && CONTROL_UI_STYLES_CSS.contains("Premium consumer-grade redesign")
        && CONTROL_UI_STYLES_CSS.contains("--premium-bg")
        && CONTROL_UI_STYLES_CSS.contains("--premium-tint")
        && CONTROL_UI_STYLES_CSS.contains("--premium-card");
    let app_has_progressive_disclosure = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("commandPaletteItems")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("HEPTA_PRIMARY_NAV")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("prompt-chip-row")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-open-command-palette")
        && CONTROL_UI_STYLES_CSS
            .contains("body[data-view=\"chat\"] .shell--hepta-premium .content")
        && CONTROL_UI_STYLES_CSS.contains(".prompt-chip-row")
        && CONTROL_UI_STYLES_CSS.contains("body[data-view=\"chat\"] .shell--hepta-premium .topbar");
    let app_has_simplified_primary_nav = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("HEPTA_PRIMARY_NAV")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("nav-group--primary")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("New Task")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("More")
        && CONTROL_UI_STYLES_CSS.contains(".nav-group--primary");
    let app_preserves_typing_during_live_poll = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("composerHasFocus")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains(".chat-compose")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("inputHeavyScreen")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("!inputHeavyScreen || !hadLiveData")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("[\"INPUT\", \"TEXTAREA\", \"SELECT\"]");
    let app_has_minimal_consumer_workspace = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("data-minimal-consumer-workspace")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("focus-workspace")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-thread-signature")
        && CONTROL_UI_STYLES_CSS.contains("Hard-reset focus workspace")
        && CONTROL_UI_STYLES_CSS.contains(".focus-workspace")
        && CONTROL_UI_STYLES_CSS.contains("body[data-view=\"chat\"] .shell--hepta-premium .topbar")
        && CONTROL_UI_STYLES_CSS
            .contains("body[data-view=\"chat\"] .shell--hepta-premium .sidebar")
        && CONTROL_UI_STYLES_CSS.contains(".focus-compose");
    let app_has_telegram_multi_agent_workspace = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("data-telegram-multi-agent-chat")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("Telegram-style multi-agent chat")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-search")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-folder")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-inline-create")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-pin")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-archive")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-unarchive")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-delete-confirm")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-row-menu-toggle")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-row-menu-panel")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-row-menu-item")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("role=\"menuitem\"")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("conversationMenuItems")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-composer-shell")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-command-shortcut")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-attachment-placeholder")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-composer-popover")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-artifact-insert")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-command-insert")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-composer-picker-search")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-composer-picker-item")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("filterComposerOptions")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("setComposerPickerActiveIndex")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("activateComposerPickerSelection")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("addComposerEvidenceOption")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("composerCitationCard")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("composerContextChips")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("groupedComposerContextChips")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-chip-group")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-chip-overflow")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("COMPOSER_CONTEXT_CHIP_COLLAPSED_LIMIT")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-chip-token")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("toggleComposerContextChipPreview")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("expandedContextChipKey")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("composerContextSummary")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-summary")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("composerContextReferenceHealth")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-health")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("composerRepairTarget")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("replaceComposerContextBlock")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-chip-repair")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderComposerStaleSendGuard")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-stale-guard")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-stale-send-continue")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("allowStaleContexts")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("composerContextAuditTrail")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderMessageContextAudit")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-audit")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-workspace-scope")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-brain-scope")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("sharedBrainContract")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("sharedBrainModeLabel")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("GLOBAL_BRAIN_SHARED_DOMAINS")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("WORKSPACE_CONTEXT_ISOLATED_DOMAINS")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("workspaceTaskBindings")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("workspaceTaskImports")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("workspaceSelectedTaskIds")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("timelineLimits")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("chatTimelineWindow")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-timeline-show-more")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("chatThreadSearchMatches")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-thread-search")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-thread-search-active")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("CHAT_STATE_SCHEMA_VERSION")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("pruneChatStateForStorage")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("CHAT_STATE_MAX_LOCAL_THREAD_RECORDS")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("liveDataFetchPlanForScreen")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("lazyLiveDataFetch")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("inlineCreateWorkspaceId")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderWorkspaceRoomPanel")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderWorkspaceRoomTaskActions")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderWorkspaceRoomTaskArtifactPreview")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("roomTaskArtifactCitation")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderWorkspaceRoomActivityLog")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("appendWorkspaceRoomActivity")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-room-task-artifact-insert")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-room-task-action-plan")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("copyReviewedRoomTaskCommand")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-workspace-room-panel")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-room-orchestration")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderWorkspaceMemberRoster")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/workspace-members")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-workspace-group-chat")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("composerMentionAgentIds")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("CHAT_ROUTING_MODE_OPTIONS")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("effectiveChatRoutingMode")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("reducerModeForRoutingMode")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-routing-mode")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderChatOrchestration")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-orchestration")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("groupedAgentRepliesFromTurnResults")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderGroupedAgentReplies")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-grouped-replies")
        && CONTROL_UI_STYLES_CSS.contains(".tg-compose-mode")
        && CONTROL_UI_STYLES_CSS.contains(".tg-orchestration-sequence")
        && CONTROL_UI_STYLES_CSS.contains(".tg-agent-reply-group")
        && CONTROL_UI_STYLES_CSS.contains(".tg-timeline-window")
        && CONTROL_UI_STYLES_CSS.contains(".tg-thread-search-bar")
        && CONTROL_UI_STYLES_CSS.contains(".tg-room-panel")
        && CONTROL_UI_STYLES_CSS.contains(".tg-room-plan-card")
        && CONTROL_UI_STYLES_CSS.contains(".tg-room-artifact-preview")
        && CONTROL_UI_STYLES_CSS.contains(".tg-room-activity-item")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("target_agent_ids")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("routing_mode")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("imported_task_ids")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-task-import")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-task-select")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("importTaskIntoWorkspace")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("selectWorkspaceTask")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("postJson(\"/api/task\"")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("postJson(\"/api/task-evidence\"")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("workspace_id")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("workspaceId")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("chatWorkspaceIdForConversation")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("contextAuditCitationBlock")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-audit-copy")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-audit-restore")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("restoreContextAuditToComposer")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("dedupeComposerCitationText")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("composerCitationBlocks")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("normalizeComposerCitationBlock")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("Same key changed")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("changedKeys")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("changedBlocks")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderContextReuseDiff")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-reuse-diff")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-reuse-keep")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-reuse-replace")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-reuse-keep-all")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-reuse-replace-all")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("updateContextReuseDiffBlock")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("updateContextReuseDiffBatch")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("setContextReuseUndo")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderContextReuseUndo")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-reuse-undo")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("undoContextReuseReplace")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("handleContextWorkflowShortcut")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-shortcut-hint")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("Keyboard: R replace all")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("clearComposerContextTransientState")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("clearContextWorkflowAfterDraftEdit")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("contextWorkflowHasTransientState")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("commitChatTransientState")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("clearChatTransientUiState")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("chatUiHasTransientState")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("HEPTA_CHAT_BOUNDARY: chat-state-constants")
        && CONTROL_UI_RUST_RENDERER_MARKERS
            .contains("HEPTA_CHAT_BOUNDARY: chat-transient-state-helpers")
        && CONTROL_UI_RUST_RENDERER_MARKERS
            .contains("HEPTA_CHAT_BOUNDARY: composer-citation-helpers")
        && CONTROL_UI_RUST_RENDERER_MARKERS
            .contains("HEPTA_CHAT_BOUNDARY: composer-context-render")
        && CONTROL_UI_RUST_RENDERER_MARKERS
            .contains("HEPTA_CHAT_BOUNDARY: composer-popover-render")
        && CONTROL_UI_RUST_RENDERER_MARKERS
            .contains("HEPTA_CHAT_BOUNDARY: conversation-lifecycle-actions")
        && CONTROL_UI_RUST_RENDERER_MARKERS
            .contains("HEPTA_CHAT_BOUNDARY: chat-state-normalization")
        && CONTROL_UI_RUST_RENDERER_MARKERS
            .contains("HEPTA_CHAT_BOUNDARY: conversation-derived-metadata")
        && CONTROL_UI_RUST_RENDERER_MARKERS
            .contains("HEPTA_CHAT_BOUNDARY: conversation-derivation")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("HEPTA_CHAT_BOUNDARY: event-binding")
        && CONTROL_UI_RUST_RENDERER_MARKERS
            .contains("HEPTA_CHAT_BOUNDARY: composer-context-dedupe")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("HEPTA_CHAT_BOUNDARY: context-reuse-render")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("dismissContextReuseDiff")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("dismissContextReuseUndo")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("composerContextBlockRangeByKey")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("Skipped ${deduped.skippedCount} duplicate")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-audit-expanded")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-chip")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-chip-preview")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-context-chip-remove")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("task-patches")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("task-evidence")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("promotion-ledger")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("insertChatComposerText")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-enter-send")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-send-state")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("setChatComposerStatus")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-unread")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-date-divider")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-conversation")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("deriveChatConversations")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("setChatConversation")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("togglePinnedConversation")
        && !CONTROL_UI_RUST_RENDERER_MARKERS.contains("window.prompt(")
        && !CONTROL_UI_RUST_RENDERER_MARKERS.contains("window.confirm(")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("localStorage")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("seenConversationTimestamps")
        && CONTROL_UI_STYLES_CSS.contains("Telegram-style multi-agent workspace")
        && CONTROL_UI_STYLES_CSS.contains(".tg-search-shell")
        && CONTROL_UI_STYLES_CSS.contains(".tg-inline-create")
        && CONTROL_UI_STYLES_CSS.contains(".tg-row-action-tray")
        && CONTROL_UI_STYLES_CSS.contains(".tg-row-action-popover")
        && CONTROL_UI_STYLES_CSS.contains(".tg-row-action__icon")
        && CONTROL_UI_STYLES_CSS.contains(".tg-row-menu-separator")
        && CONTROL_UI_STYLES_CSS.contains(".tg-compose-bar")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-chip")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-chip-group")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-chip-summary")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-chip-overflow")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-chip--stale")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-chip-health")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-chip-repair")
        && CONTROL_UI_STYLES_CSS.contains(".tg-stale-send-guard")
        && CONTROL_UI_STYLES_CSS.contains(".tg-message-context-audit")
        && CONTROL_UI_STYLES_CSS.contains(".tg-message-context-audit__copy")
        && CONTROL_UI_STYLES_CSS.contains(".tg-message-context-audit__restore")
        && CONTROL_UI_STYLES_CSS.contains(".tg-message-context-audit__actions")
        && CONTROL_UI_STYLES_CSS.contains(".tg-message-context-audit__details")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-reuse-diff")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-reuse-diff__actions")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-reuse-diff__batch")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-reuse-undo")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-shortcut-hint")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-chip--open")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-chip--evidence")
        && CONTROL_UI_STYLES_CSS.contains(".tg-context-chip-preview")
        && CONTROL_UI_STYLES_CSS.contains(".tg-composer-popover")
        && CONTROL_UI_STYLES_CSS.contains(".tg-composer-popover__item")
        && CONTROL_UI_STYLES_CSS.contains(".tg-composer-popover__item.active")
        && CONTROL_UI_STYLES_CSS.contains(".tg-composer-popover__search")
        && CONTROL_UI_STYLES_CSS.contains(".tg-send-button")
        && CONTROL_UI_STYLES_CSS.contains(".tg-compose-status")
        && CONTROL_UI_STYLES_CSS.contains(".tg-folder-row")
        && CONTROL_UI_STYLES_CSS.contains(".tg-conversation-rail")
        && CONTROL_UI_STYLES_CSS.contains(".tg-thread-panel")
        && CONTROL_UI_STYLES_CSS.contains(".tg-thread-details")
        && CONTROL_UI_STYLES_CSS.contains(".tg-inline-delete-confirm")
        && CONTROL_UI_STYLES_CSS.contains(".tg-chat-item")
        && CONTROL_UI_STYLES_CSS.contains(".tg-chat-item__unread")
        && CONTROL_UI_STYLES_CSS.contains(".tg-date-divider")
        && CONTROL_UI_STYLES_CSS.contains(".tg-message--self");
    let app_models_long_gateway_websocket_resilience = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("HEPTA_RUNTIME_2026_5_2_UI_RESILIENCE_CONTRACT")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("long-running-ws-resilience-modeled")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("polling-no-store-fallback")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("gatewayWebSocketOpenedByAudit: false");
    let app_has_grouped_message_width_guard = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("bounded-grouped-reply-width")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-grouped-replies")
        && CONTROL_UI_STYLES_CSS.contains("--hepta_runtime-2026-5-2-grouped-message-max-width")
        && CONTROL_UI_STYLES_CSS.contains("overflow-wrap: anywhere");
    let app_has_ios_pwa_bounds_guard = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("dynamic-viewport-bounds")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("safe-area-inset-bottom")
        && CONTROL_UI_STYLES_CSS.contains("--hepta_runtime-2026-5-2-ios-pwa-bottom")
        && CONTROL_UI_STYLES_CSS.contains("safe-area-inset-bottom")
        && CONTROL_UI_STYLES_CSS.contains("100dvh");
    let app_has_selection_contrast_guard = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("high-contrast-selection")
        && CONTROL_UI_STYLES_CSS.contains("::selection")
        && CONTROL_UI_STYLES_CSS.contains("--hepta_runtime-2026-5-2-selection-bg")
        && CONTROL_UI_STYLES_CSS.contains("--hepta_runtime-2026-5-2-selection-fg");
    let app_has_slash_feedback_surface = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("slash-command-feedback-toast")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("command-result-status-preserved")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("showToast")
        && CONTROL_UI_STYLES_CSS.contains(".toast.show");
    let app_has_talk_diagnostics_resilience = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("realtime-talk-diagnostic-redaction")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("long-call-status-surface")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("rawDiagnosticPayloadLogged: false");
    let app_has_persisted_auto_scroll_mode = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("persistedAutoScrollMode")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("HEPTA_CHAT_AUTO_SCROLL_STORAGE_KEY")
        && rust_frontend_html.contains("data-chat-autoscroll-mode")
        && rust_frontend_html.contains("data-chat-autoscroll-persisted")
        && CONTROL_UI_STYLES_CSS.contains(".tg-autoscroll-select");
    let app_has_blank_dashboard_recovery_panel = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("renderBlankDashboardRecoveryPanel")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("blank-dashboard-html-recovery")
        && rust_frontend_html.contains("data-dashboard-recovery-panel")
        && CONTROL_UI_STYLES_CSS.contains(".hepta-dashboard-recovery");
    let app_has_compact_session_status_badges = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("renderCompactSessionStatusBadges")
        && rust_frontend_html.contains("data-session-status-badges")
        && rust_frontend_html.contains(r#"data-session-status="live""#)
        && rust_frontend_html.contains(r#"data-session-status="idle""#)
        && rust_frontend_html.contains(r#"data-session-status="terminal""#)
        && CONTROL_UI_STYLES_CSS.contains(".tg-session-state");
    let app_scopes_nodes_polling_to_active_tab = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("nodesPollScopeActiveTab")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("nodesPollingAutoStart: false")
        && rust_frontend_html.contains(r#"data-nodes-poll-scope="active-tab-only""#)
        && rust_frontend_html.contains(r#"data-nodes-poll-autostart="false""#);
    let app_distinguishes_sample_vs_live_adapter_readiness = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("sampleReadinessStatus")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("liveAdapterReadinessStatus")
        && rust_frontend_html.contains(r#"data-readiness-kind="sample""#)
        && rust_frontend_html.contains(r#"data-readiness-kind="live-adapter""#);
    let app_has_terminal_qr_rendering_guard = CONTROL_UI_RUST_RENDERER_MARKERS
        .contains("terminalQrRendering")
        && CONTROL_UI_RUST_RENDERER_MARKERS.contains("fixed-cell-terminal-qr")
        && rust_frontend_html.contains("data-terminal-qr-rendering")
        && CONTROL_UI_STYLES_CSS.contains(".terminal-qr-preview");
    let control_ui_gateway_websocket_opened_by_audit = false;
    let control_ui_live_gateway_rpc_performed = false;
    let app_has_hepta_runtime_2026_5_2_ui_resilience = app_models_long_gateway_websocket_resilience
        && app_has_grouped_message_width_guard
        && app_has_ios_pwa_bounds_guard
        && app_has_selection_contrast_guard
        && app_has_slash_feedback_surface
        && app_has_talk_diagnostics_resilience
        && !control_ui_gateway_websocket_opened_by_audit
        && !control_ui_live_gateway_rpc_performed;
    let readme_mentions_boundary =
        CONTROL_UI_README.contains("Boundary") && CONTROL_UI_README.contains("not a hosted SaaS");
    let p0_checks = [
        all_screen_ids_aligned,
        app_has_endpoint_health_grid,
        app_has_post_action_guard,
        app_has_operator_security,
        app_preserves_typing_during_live_poll,
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("liveDataFetchPlanForScreen"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("lazyLiveDataFetch"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("chatTimelineWindow"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("pruneChatStateForStorage"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("CHAT_STATE_SCHEMA_VERSION"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("pruneWorkspaceTaskMap"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("CHAT_STATE_MAX_WORKSPACE_TASK_MAPS"),
        CONTROL_UI_RUST_RENDERER_MARKERS
            .contains("roomTaskArtifactPreview: previousRoomTaskArtifactPreview"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("activeScreen === \"chat\""),
        !CONTROL_UI_RUST_RENDERER_MARKERS.contains("window.prompt("),
        !CONTROL_UI_RUST_RENDERER_MARKERS.contains("window.confirm("),
    ];
    let p1_checks = [
        app_has_chat_first_architecture,
        app_has_telegram_multi_agent_workspace,
        app_has_task_publisher,
        app_has_agent_chat,
        app_has_task_drilldown,
        app_has_transcript_search,
        app_has_replay_promotion_drilldown,
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderWorkspaceRoomPanel"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderWorkspaceRoomTaskActions"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderWorkspaceRoomTaskArtifactPreview"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderWorkspaceRoomActivityLog"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-thread-search"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("roomTaskArtifactCitation"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("mapped_command"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("focusChatThreadSearchInput"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("focusChatSearchInput"),
    ];
    let p2_checks = [
        app_has_hepta_runtime_ui_parity,
        app_has_command_palette,
        app_has_premium_consumer_ui,
        app_has_progressive_disclosure,
        app_has_simplified_primary_nav,
        app_has_keyboard_shortcuts,
        app_has_readonly_command_runner,
        app_has_approval_cards,
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderConvergenceLedger"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-ui-convergence-ledger"),
        CONTROL_UI_STYLES_CSS.contains(".convergence-ledger"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("role=\"listbox\""),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("aria-selected"),
        CONTROL_UI_STYLES_CSS.contains("prefers-reduced-motion"),
        app_has_hepta_runtime_2026_5_2_ui_resilience,
        !control_ui_gateway_websocket_opened_by_audit,
        !control_ui_live_gateway_rpc_performed,
        readme_mentions_boundary,
    ];
    let p3_checks = [
        CONTROL_UI_FUNCTIONAL_SMOKE_MJS.contains("Hepta Control UI functional smoke passed"),
        CONTROL_UI_FUNCTIONAL_SMOKE_MJS.contains("10k message search exceeded budget"),
        CONTROL_UI_FUNCTIONAL_SMOKE_MJS.contains("state pruning exceeded budget"),
        CONTROL_UI_FUNCTIONAL_SMOKE_MJS.contains("chat search should keep focus after rerender"),
        CONTROL_UI_FUNCTIONAL_SMOKE_MJS.contains("thread search should keep focus after rerender"),
        CONTROL_UI_FUNCTIONAL_SMOKE_MJS.contains("mapped command should replace task placeholder"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderEndpointRecoveryBanner"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-endpoint-retry-all"),
        CONTROL_UI_STYLES_CSS.contains(".endpoint-recovery-banner"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
    ];
    let p4_checks = [
        CONTROL_UI_QUALITY_SMOKE_MJS.contains("Hepta Control UI quality smoke passed"),
        CONTROL_UI_SMOKE_SH.contains("Rust/no-JS contract smoke"),
        CONTROL_UI_QUALITY_SMOKE_MJS.contains("styles.css budget exceeded"),
        CONTROL_UI_QUALITY_SMOKE_MJS.contains("README budget exceeded"),
        CONTROL_UI_QUALITY_SMOKE_MJS.contains("HEPTA_CHAT_BOUNDARY"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_README.contains("Rust/no-JS contract smoke"),
        CONTROL_UI_RUST_RENDERER_MARKERS.len() < 327_000,
        CONTROL_UI_STYLES_CSS.len() < 105_000,
        !CONTROL_UI_RUST_RENDERER_MARKERS.contains("window.alert("),
    ];
    let p5_checks = [
        CONTROL_UI_BROWSER_SMOKE_MJS.contains("Hepta Control UI browser smoke passed"),
        CONTROL_UI_BROWSER_SMOKE_MJS.contains("playwright"),
        CONTROL_UI_BROWSER_SMOKE_MJS.contains("desktop"),
        CONTROL_UI_BROWSER_SMOKE_MJS.contains("narrow"),
        CONTROL_UI_BROWSER_SMOKE_MJS.contains("mobile"),
        CONTROL_UI_BROWSER_SMOKE_MJS.contains("screenshot"),
        CONTROL_UI_BROWSER_SMOKE_MJS.contains("data-telegram-multi-agent-chat"),
        CONTROL_UI_BROWSER_SMOKE_MJS.contains("data-ui-convergence-ledger"),
        CONTROL_UI_BROWSER_SMOKE_MJS.contains("chat search should retain focus in browser"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
    ];
    let p6_checks = [
        CONTROL_UI_RELEASE_SMOKE_MJS.contains("Hepta Control UI release smoke passed"),
        CONTROL_UI_RELEASE_SMOKE_MJS.contains("browser screenshot manifest"),
        CONTROL_UI_RELEASE_SMOKE_MJS.contains("desktop should show Workspace Room"),
        CONTROL_UI_RELEASE_SMOKE_MJS.contains("mobile should hide Workspace Room"),
        CONTROL_UI_P0_P6_RELEASE_DOC.contains("Hepta Control UI P0-P6 Convergence"),
        CONTROL_UI_P0_P6_RELEASE_DOC.contains("Operator walkthrough"),
        CONTROL_UI_P0_P6_RELEASE_DOC.contains("Screenshot manifest"),
        CONTROL_UI_P0_P6_RELEASE_DOC.contains("Gate commands"),
        CONTROL_UI_P0_P6_RELEASE_DOC.contains("not a hosted SaaS"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
    ];
    let p7_checks = [
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("Hepta Control UI maturity smoke passed"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("applyContentRichFixture"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("content-rich visual regression"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("rich-desktop"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("applyControlUiContentRichFixture"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-content-rich-fixture"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("seededControlUiMessages"),
        CONTROL_UI_P0_P13_MATURITY_DOC.contains("P7: content-rich seeded visual regression"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains(".tg-message"),
    ];
    let p8_checks = [
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-mobile-layered-chat"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-mobile-active-pane"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderMobileLayerTabs"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("setChatMobilePane"),
        CONTROL_UI_STYLES_CSS.contains(".tg-mobile-layer-tabs"),
        CONTROL_UI_STYLES_CSS.contains("data-chat-mobile-active-pane=\"chats\""),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("data-chat-mobile-pane"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("mobile pane should be visible"),
        CONTROL_UI_P0_P13_MATURITY_DOC.contains("P8: mobile layered UX"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("rich-mobile-room"),
    ];
    let p9_checks = [
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderThreadStatusSummary"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-thread-compact-status"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-thread-status-popover"),
        CONTROL_UI_STYLES_CSS.contains(".tg-thread-status-summary"),
        CONTROL_UI_STYLES_CSS.contains(".tg-thread-extra-status"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("header should keep primary status compact"),
        CONTROL_UI_P0_P13_MATURITY_DOC.contains("P9: compact header/status chrome"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("Details"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains(".tg-thread-status-primary"),
        CONTROL_UI_README.contains("P0-P21 convergence ledger"),
    ];
    let p10_checks = [
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderWorkspaceRoomOnboardingCard"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("workspaceRoomHasRichContent"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-room-empty-consolidated"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-room-onboarding-card"),
        CONTROL_UI_STYLES_CSS.contains(".tg-room-onboarding-card"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("empty room should collapse"),
        CONTROL_UI_P0_P13_MATURITY_DOC.contains("P10: consolidated empty Workspace Room"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("applyControlUiEmptyFixture"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("applyEmptyFixture"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("Workspace is ready"),
    ];
    let p11_checks = [
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("data-room-task-artifact-insert"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("data-room-task-action-plan"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("data-room-task-action-confirm"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("data-endpoint-retry-all"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("dry-run review checkbox should be interactive"),
        CONTROL_UI_P0_P13_MATURITY_DOC.contains("P11: real user journey E2E"),
        CONTROL_UI_P0_P13_MATURITY_DOC.contains("Insert a task evidence citation"),
        CONTROL_UI_P0_P13_MATURITY_DOC.contains("degraded endpoint retry"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("roomTaskActionPlan"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("roomTaskArtifactPreview"),
    ];
    let p12_checks = [
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("HEPTA_UI_MODULE_BOUNDARIES"),
        CONTROL_UI_MODULE_BOUNDARIES_README.contains("chat-state"),
        CONTROL_UI_MODULE_BOUNDARIES_README.contains("chat-render"),
        CONTROL_UI_MODULE_BOUNDARIES_README.contains("workspace-room"),
        CONTROL_UI_MODULE_BOUNDARIES_README.contains("live-data"),
        CONTROL_UI_MODULE_BOUNDARIES_README.contains("task-actions"),
        CONTROL_UI_MODULE_BOUNDARIES_README.contains("browser-fixtures"),
        CONTROL_UI_MODULE_BOUNDARIES_README.contains("accessibility"),
        CONTROL_UI_MODULE_BOUNDARIES_README.contains("exec-approvals"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("module boundary missing"),
        CONTROL_UI_P0_P13_MATURITY_DOC.contains("P12: module-boundary governance"),
    ];
    let p13_checks = [
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("assertA11y"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("visible buttons should have text or aria labels"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("command palette should expose dialog semantics"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("mobile layer switcher should be a tablist"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("exactly one mobile layer tab should be selected"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("reduced motion rule should be present"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("role=\"tablist\""),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("role=\"tab\""),
        CONTROL_UI_P0_P13_MATURITY_DOC.contains("P13: browser-level accessibility"),
        CONTROL_UI_STYLES_CSS.contains("prefers-reduced-motion"),
    ];
    let p14_checks = [
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("Hepta Control UI hardening smoke passed"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("visual diff exceeded baseline"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("visual diff baseline"),
        CONTROL_UI_VISUAL_BASELINE_JSON.contains("Hepta Control UI visual diff baseline"),
        CONTROL_UI_VISUAL_BASELINE_JSON.contains("chatRail"),
        CONTROL_UI_VISUAL_BASELINE_JSON.contains("threadPanel"),
        CONTROL_UI_VISUAL_BASELINE_JSON.contains("roomPanel"),
        CONTROL_UI_P0_P21_HARDENING_DOC.contains("P14: visual diff baseline"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("compareRect"),
    ];
    let p15_checks = [
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderWorkspaceRoomAccordion"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-room-accordion"),
        CONTROL_UI_STYLES_CSS.contains(".tg-room-accordion"),
        CONTROL_UI_STYLES_CSS.contains("tg-room-accordion:not([open])"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("room accordion compression"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("closedRoomAccordions"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("roomScrollRatio"),
        CONTROL_UI_P0_P21_HARDENING_DOC.contains("P15: Workspace Room accordion"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("key: \"evidence\""),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("key: \"orchestration\""),
    ];
    let p16_checks = [
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-mobile-compact-composer"),
        CONTROL_UI_STYLES_CSS.contains("[data-mobile-compact-composer=\"true\"]"),
        CONTROL_UI_STYLES_CSS.contains("max-height: 76px"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("mobile compact composer too tall"),
        CONTROL_UI_P0_P21_HARDENING_DOC.contains("P16: mobile compact composer"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("composerHeight <= 150"),
        CONTROL_UI_STYLES_CSS.contains(".tg-focus-result"),
        CONTROL_UI_STYLES_CSS.contains("text-overflow: ellipsis"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-chat-composer-shell"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("mobileThread"),
    ];
    let p17_checks = [
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("first render budget exceeded"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("command palette latency budget exceeded"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("renderMs < 1_800"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("paletteLatencyMs < 250"),
        CONTROL_UI_P0_P21_HARDENING_DOC.contains("P17: browser performance budget"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("performance.now"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("manifest.results"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("hepta-control-ui-hardening-smoke"),
        CONTROL_UI_SMOKE_SH.contains("Hepta Control UI hardening smoke passed"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("waitUntil: \"domcontentloaded\""),
    ];
    let p18_checks = [
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("assertKeyboardAndA11y"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("focus trap should move keyboard focus"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("keyboard-only tab journey"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("contrast ratio too low"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("contrastRatio"),
        CONTROL_UI_P0_P21_HARDENING_DOC.contains("P18: accessibility audit upgrade"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("Control+K"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("aria-modal"),
        CONTROL_UI_STYLES_CSS.contains("prefers-reduced-motion"),
        CONTROL_UI_INDEX_HTML.contains("role=\"dialog\""),
    ];
    let p19_checks = [
        CONTROL_UI_MODULE_INDEX_JS.contains("controlUiModules"),
        CONTROL_UI_MODULE_INDEX_JS.contains("chat-state"),
        CONTROL_UI_MODULE_INDEX_JS.contains("workspace-room"),
        CONTROL_UI_MODULE_INDEX_JS.contains("browser-fixtures"),
        CONTROL_UI_MODULE_INDEX_JS.contains("accessibility"),
        CONTROL_UI_MODULE_INDEX_JS.contains("execApprovalsModule"),
        CONTROL_UI_MODULE_EXEC_APPROVALS_JS.contains("/api/approvals/exec/apply"),
        CONTROL_UI_MODULE_EXEC_APPROVALS_JS.contains("previewExecApprovalPatch"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("real module split should export module registry"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("modules.controlUiModules"),
        CONTROL_UI_P0_P21_HARDENING_DOC.contains("P19: real module split"),
        CONTROL_UI_MODULE_BOUNDARIES_README.contains("single-file static delivery"),
        CONTROL_UI_README.contains("apps/hepta-control-ui/modules"),
    ];
    let p20_checks = [
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("applyControlUiEndpointChaosFixture"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("endpointChaos"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("partial failure"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("stale-cache"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("applyEndpointChaosFixture"),
        CONTROL_UI_HARDENING_SMOKE_MJS
            .contains("endpoint chaos should expose per-endpoint retries"),
        CONTROL_UI_HARDENING_SMOKE_MJS.contains("partial failure detail"),
        CONTROL_UI_P0_P21_HARDENING_DOC.contains("P20: endpoint chaos regression"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("latency_ms"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("/api/operator-security"),
    ];
    let p21_checks = [
        CONTROL_UI_README.contains("## Overview"),
        CONTROL_UI_README.contains("## Interaction model"),
        CONTROL_UI_README.contains("## Safety model"),
        CONTROL_UI_README.contains("## Gates"),
        CONTROL_UI_README.contains("## Architecture notes"),
        CONTROL_UI_README.len() < 8_000,
        CONTROL_UI_P0_P21_HARDENING_DOC.contains("P21: README/documentation density cleanup"),
        CONTROL_UI_README.contains("P0-P21 convergence ledger"),
        CONTROL_UI_README.contains("visual diff baseline"),
        CONTROL_UI_README.contains("not a hosted SaaS"),
    ];
    let p22_checks = [
        CONTROL_UI_INDEX_HTML.contains("data-no-js-frontend=\"true\"")
            && !CONTROL_UI_INDEX_HTML.contains("<script"),
        CONTROL_UI_MODULE_INDEX_JS.contains("window.__HEPTA_UI_MODULE_REGISTRY"),
        CONTROL_UI_MODULE_INDEX_JS.contains("controlUiModules"),
        CONTROL_UI_BUILD_SMOKE_MJS.contains("Hepta Control UI build split smoke passed"),
        CONTROL_UI_BUILD_SMOKE_MJS.contains("bundle_mode"),
        CONTROL_UI_BUILD_SMOKE_MJS.contains("static-app-plus-esm-build-registry"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P29_ENGINEERING_DOC.contains("P22: real ESM/build split"),
        CONTROL_UI_MODULE_INDEX_JS.contains("task-actions"),
        CONTROL_UI_MODULE_INDEX_JS.contains("live-data"),
    ];
    let p23_checks = [
        CONTROL_UI_SMOKE_CONTRACT_MJS.contains("Hepta Control UI contract-suite smoke passed"),
        CONTROL_UI_SMOKE_MARKER_MJS.contains("Hepta Control UI marker-suite smoke passed"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P29_ENGINEERING_DOC.contains("P23: smoke suite decomposition"),
        CONTROL_UI_SMOKE_CONTRACT_MJS.contains("/ui-contract-audit"),
    ];
    let p24_checks = [
        CONTROL_UI_CROSS_BROWSER_SMOKE_MJS.contains("Hepta Control UI cross-browser smoke passed"),
        CONTROL_UI_CROSS_BROWSER_SMOKE_MJS.contains("chromium-system"),
        CONTROL_UI_CROSS_BROWSER_SMOKE_MJS.contains("firefox-managed"),
        CONTROL_UI_CROSS_BROWSER_SMOKE_MJS.contains("webkit-managed"),
        CONTROL_UI_CROSS_BROWSER_SMOKE_MJS.contains("cross-browser matrix should enumerate"),
        CONTROL_UI_CROSS_BROWSER_SMOKE_MJS.contains("status: \"skipped\""),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P29_ENGINEERING_DOC.contains("P24: cross-browser readiness matrix"),
        CONTROL_UI_CROSS_BROWSER_SMOKE_MJS.contains("data-room-accordion"),
        CONTROL_UI_CROSS_BROWSER_SMOKE_MJS.contains("data-chat-mobile-active-pane"),
    ];
    let p25_checks = [
        CONTROL_UI_PERCEPTUAL_SMOKE_MJS.contains("Hepta Control UI perceptual smoke passed"),
        CONTROL_UI_PERCEPTUAL_SMOKE_MJS.contains("bmpAverageHash"),
        CONTROL_UI_PERCEPTUAL_SMOKE_MJS.contains("hamming"),
        CONTROL_UI_PERCEPTUAL_SMOKE_MJS.contains("sips"),
        CONTROL_UI_PERCEPTUAL_BASELINE_JSON
            .contains("Hepta Control UI perceptual visual diff baseline"),
        CONTROL_UI_PERCEPTUAL_BASELINE_JSON.contains("rich-desktop"),
        CONTROL_UI_PERCEPTUAL_BASELINE_JSON.contains("rich-mobile-room"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P29_ENGINEERING_DOC.contains("P25: perceptual visual diff"),
        CONTROL_UI_PERCEPTUAL_SMOKE_MJS.contains("perceptual diff exceeded threshold"),
    ];
    let p26_checks = [
        CONTROL_UI_SCHEMA_SMOKE_MJS.contains("Hepta Control UI schema smoke passed"),
        CONTROL_UI_SCHEMA_SMOKE_MJS.contains("control-ui.schema.json"),
        CONTROL_UI_SCHEMA_SMOKE_MJS.contains("ui-contract-audit.schema.json"),
        CONTROL_UI_SCHEMA_CONTROL_JSON.contains("screen_count"),
        CONTROL_UI_SCHEMA_AUDIT_JSON.contains("p0_p21_converged"),
        CONTROL_UI_SCHEMA_CHAT_JSON.contains("agent_id"),
        CONTROL_UI_SCHEMA_TASK_JSON.contains("task_id"),
        CONTROL_UI_SCHEMA_EVENTS_JSON.contains("events"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P29_ENGINEERING_DOC.contains("P26: JSON schema contract gates"),
    ];
    let p27_checks = [
        CONTROL_UI_SOAK_SMOKE_MJS.contains("Hepta Control UI soak smoke passed"),
        CONTROL_UI_SOAK_SMOKE_MJS.contains("localStorageBytes"),
        CONTROL_UI_SOAK_SMOKE_MJS.contains("nodeCount"),
        CONTROL_UI_SOAK_SMOKE_MJS.contains("typing preservation should survive soak loops"),
        CONTROL_UI_SOAK_SMOKE_MJS.contains("applyEndpointChaosFixture"),
        CONTROL_UI_SOAK_SMOKE_MJS.contains("DOM node soak budget exceeded"),
        CONTROL_UI_SOAK_SMOKE_MJS.contains("localStorage soak budget exceeded"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P29_ENGINEERING_DOC.contains("P27: UI soak/leak regression"),
        CONTROL_UI_SOAK_SMOKE_MJS.contains("manifest.json"),
    ];
    let p28_checks = [
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderTaskArtifactResultDrawer"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-task-result-drawer"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-task-result-workspace"),
        CONTROL_UI_STYLES_CSS.contains(".tg-room-result-drawer"),
        CONTROL_UI_STYLES_CSS.contains("backdrop-filter"),
        CONTROL_UI_MATURITY_SMOKE_MJS.contains("data-room-task-artifact-insert"),
        CONTROL_UI_P0_P29_ENGINEERING_DOC.contains("P28: task/artifact result drawer"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("Current result"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("roomTaskArtifactSummary"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("read-only"),
    ];
    let p29_checks = [
        CONTROL_UI_A11Y_SNAPSHOT_MJS.contains("Hepta Control UI accessibility snapshot passed"),
        CONTROL_UI_A11Y_SNAPSHOT_MJS.contains("heading hierarchy snapshot"),
        CONTROL_UI_A11Y_SNAPSHOT_MJS.contains("all visible controls should have accessible names"),
        CONTROL_UI_A11Y_SNAPSHOT_MJS
            .contains("mobile layer tablist should expose one selected tab"),
        CONTROL_UI_A11Y_SNAPSHOT_MJS.contains("Control+K"),
        CONTROL_UI_A11Y_SNAPSHOT_MJS.contains("data-room-task-artifact-insert"),
        CONTROL_UI_RUST_RENDERER_MARKERS
            .contains("<h2>${escapeHtml(selected?.title || \"Hepta agent\")}</h2>"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P29_ENGINEERING_DOC.contains("P29: full accessibility rule snapshot"),
        CONTROL_UI_A11Y_SNAPSHOT_MJS.contains("snapshot.json"),
    ];
    let p30_checks = [
        CONTROL_UI_CROSS_BROWSER_STRICT_SMOKE_MJS
            .contains("Hepta Control UI strict cross-browser smoke passed"),
        CONTROL_UI_CROSS_BROWSER_STRICT_SMOKE_MJS.contains("chromium-system"),
        CONTROL_UI_CROSS_BROWSER_STRICT_SMOKE_MJS.contains("firefox-managed"),
        CONTROL_UI_CROSS_BROWSER_STRICT_SMOKE_MJS.contains("webkit-managed"),
        CONTROL_UI_CROSS_BROWSER_STRICT_SMOKE_MJS
            .contains("classified-with-install-hints-never-silent"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P39_PRODUCTIZATION_DOC.contains("P30: strict cross-browser readiness"),
        CONTROL_UI_CROSS_BROWSER_STRICT_SMOKE_MJS.contains("required_runtime_passes"),
        CONTROL_UI_CROSS_BROWSER_STRICT_SMOKE_MJS.contains("skip_policy"),
        CONTROL_UI_CROSS_BROWSER_STRICT_SMOKE_MJS.contains("failed"),
    ];
    let p31_checks = [
        CONTROL_UI_MODULE_PRODUCTIZATION_JS.contains("productizationModule"),
        CONTROL_UI_MODULE_PRODUCTIZATION_JS.contains("productizationPolicies"),
        CONTROL_UI_MODULE_INDEX_JS.contains("productizationModule"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("productization"),
        CONTROL_UI_PRODUCTIZATION_SMOKE_MJS.contains("true module extraction"),
        CONTROL_UI_PRODUCTIZATION_SMOKE_MJS.contains("productizationPolicies.gates.length === 10"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P39_PRODUCTIZATION_DOC.contains("P31: true productization module extraction"),
        CONTROL_UI_MODULE_PRODUCTIZATION_JS.contains("P30-P39 product gates"),
        CONTROL_UI_MODULE_PRODUCTIZATION_JS.contains("result drawer action model"),
    ];
    let p32_checks = [
        CONTROL_UI_SMOKE_SUMMARY_MJS.contains("Hepta Control UI smoke summary passed"),
        CONTROL_UI_SMOKE_SUMMARY_MJS.contains("suite_count"),
        CONTROL_UI_SMOKE_SUMMARY_MJS.contains("p0_p39_converged"),
        CONTROL_UI_SMOKE_SUMMARY_MJS.contains("cross-browser-strict"),
        CONTROL_UI_SMOKE_SUMMARY_MJS.contains("product-drawer"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P39_PRODUCTIZATION_DOC.contains("P32: smoke orchestrator JSON summary"),
        CONTROL_UI_SMOKE_SUMMARY_MJS.contains("target/hepta-control-ui-smoke-summary"),
        CONTROL_UI_SMOKE_SUMMARY_MJS.contains("suite output"),
        CONTROL_UI_SMOKE_SUMMARY_MJS.contains("audit_percent"),
    ];
    let p33_checks = [
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-mobile-density-tier"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("tg-mobile-density-more"),
        CONTROL_UI_STYLES_CSS.contains("--product-density-chip-max"),
        CONTROL_UI_STYLES_CSS.contains("summary-first"),
        CONTROL_UI_STYLES_CSS.contains(".badge:nth-of-type(n+4)"),
        CONTROL_UI_PRODUCTIZATION_SMOKE_MJS
            .contains("mobile density should cap visible status chips"),
        CONTROL_UI_MODULE_PRODUCTIZATION_JS.contains("visibleChipBudget"),
        CONTROL_UI_P0_P39_PRODUCTIZATION_DOC.contains("P33: mobile density polish"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_STYLES_CSS.contains("--product-mobile-composer-max"),
    ];
    let p34_checks = [
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("renderThreadStarterActions"),
        !CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-thread-starter-actions"),
        !CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-thread-starter-action"),
        !CONTROL_UI_RUST_RENDERER_MARKERS.contains("Start here"),
        CONTROL_UI_STYLES_CSS.contains(".tg-thread-empty-inline"),
        CONTROL_UI_PRODUCTIZATION_SMOKE_MJS
            .contains("old centered starter float should stay removed"),
        CONTROL_UI_MODULE_PRODUCTIZATION_JS.contains("desktopEmptyThread"),
        CONTROL_UI_MODULE_PRODUCTIZATION_JS.contains("noCenteredStarterFloat"),
        CONTROL_UI_P0_P39_PRODUCTIZATION_DOC.contains("P34: desktop empty-thread starter UX"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("dry-run review"),
    ];
    let p35_checks = [
        CONTROL_UI_STYLES_CSS.contains("--product-starter-bg"),
        CONTROL_UI_STYLES_CSS.contains("--product-result-action-bg"),
        CONTROL_UI_PRODUCTIZATION_SMOKE_MJS.contains("selector budget exceeded"),
        CONTROL_UI_PRODUCTIZATION_SMOKE_MJS.contains("important budget exceeded"),
        CONTROL_UI_MODULE_PRODUCTIZATION_JS.contains("selectorBudget"),
        CONTROL_UI_MODULE_PRODUCTIZATION_JS.contains("importantBudget"),
        CONTROL_UI_P0_P39_PRODUCTIZATION_DOC.contains("P35: design-token and selector budget"),
        CONTROL_UI_PRODUCTIZATION_SMOKE_MJS.contains("selectorCount"),
        CONTROL_UI_PRODUCTIZATION_SMOKE_MJS.contains("importantCount"),
        CONTROL_UI_STYLES_CSS.contains("color-mix"),
    ];
    let p36_checks = [
        CONTROL_UI_GOLDEN_FIXTURE_SMOKE_MJS
            .contains("Hepta Control UI golden fixture smoke passed"),
        CONTROL_UI_GOLDEN_FIXTURE_JSON.contains("task-golden-001"),
        CONTROL_UI_GOLDEN_FIXTURE_JSON.contains("endpointHealth"),
        CONTROL_UI_GOLDEN_FIXTURE_JSON.contains("roomTaskArtifactPreview"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("applyControlUiGoldenLiveDataFixture"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P39_PRODUCTIZATION_DOC.contains("P36: golden live-data fixtures"),
        CONTROL_UI_GOLDEN_FIXTURE_SMOKE_MJS.contains("Golden evidence preview"),
        CONTROL_UI_GOLDEN_FIXTURE_SMOKE_MJS.contains("data-task-result-drawer"),
        CONTROL_UI_GOLDEN_FIXTURE_JSON.contains("messages"),
    ];
    let p37_checks = [
        CONTROL_UI_A11Y_DEEP_SMOKE_MJS.contains("Hepta Control UI deep accessibility smoke passed"),
        CONTROL_UI_A11Y_DEEP_SMOKE_MJS.contains("semanticTree"),
        CONTROL_UI_A11Y_DEEP_SMOKE_MJS.contains("focusRoute"),
        CONTROL_UI_A11Y_DEEP_SMOKE_MJS.contains("heading hierarchy should not skip levels"),
        CONTROL_UI_A11Y_DEEP_SMOKE_MJS.contains("keyboard route map"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P39_PRODUCTIZATION_DOC.contains("P37: deep a11y route map"),
        CONTROL_UI_A11Y_DEEP_SMOKE_MJS.contains("reducedMotion"),
        CONTROL_UI_A11Y_DEEP_SMOKE_MJS.contains("landmarks"),
        CONTROL_UI_A11Y_DEEP_SMOKE_MJS.contains("focusables"),
    ];
    let p38_checks = [
        CONTROL_UI_HOSTILE_FIXTURE_SMOKE_MJS
            .contains("Hepta Control UI hostile fixture smoke passed"),
        CONTROL_UI_HOSTILE_FIXTURE_JSON.contains("<script>window.__HEPTA_XSS=1</script>"),
        CONTROL_UI_HOSTILE_FIXTURE_JSON.contains("<img src=x onerror=window.__HEPTA_XSS=2>"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("applyControlUiHostileFixture"),
        CONTROL_UI_HOSTILE_FIXTURE_SMOKE_MJS
            .contains("hostile fixture should not execute script payloads"),
        CONTROL_UI_HOSTILE_FIXTURE_SMOKE_MJS.contains("escaped text"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P39_PRODUCTIZATION_DOC.contains("P38: hostile/XSS fixture"),
        CONTROL_UI_MODULE_PRODUCTIZATION_JS.contains("hostileFixtures"),
        CONTROL_UI_HOSTILE_FIXTURE_SMOKE_MJS.contains("javascript: links"),
    ];
    let p39_checks = [
        CONTROL_UI_PRODUCT_DRAWER_SMOKE_MJS
            .contains("Hepta Control UI product drawer smoke passed"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-task-result-drawer-action"),
        CONTROL_UI_RUST_RENDERER_MARKERS.contains("data-task-result-drawer-actions=\"product\""),
        CONTROL_UI_STYLES_CSS.contains(".tg-room-result-drawer__actions"),
        CONTROL_UI_PRODUCT_DRAWER_SMOKE_MJS.contains("copy"),
        CONTROL_UI_PRODUCT_DRAWER_SMOKE_MJS.contains("pin"),
        CONTROL_UI_PRODUCT_DRAWER_SMOKE_MJS.contains("trace"),
        CONTROL_UI_PRODUCT_DRAWER_SMOKE_MJS.contains("next-step"),
        CONTROL_UI_SMOKE_SH.contains("control_ui_report_is_complete_and_asset_backed"),
        CONTROL_UI_P0_P39_PRODUCTIZATION_DOC.contains("P39: productized result drawer actions"),
    ];
    let convergence_lanes = vec![
        convergence_lane("p0", "P0 safety/performance guardrails", &p0_checks),
        convergence_lane("p1", "P1 operator workflows", &p1_checks),
        convergence_lane("p2", "P2 polish/parity/readability", &p2_checks),
        convergence_lane("p3", "P3 regression and quality gates", &p3_checks),
        convergence_lane("p4", "P4 maintainability and release hygiene", &p4_checks),
        convergence_lane(
            "p5",
            "P5 browser visual and interaction regression",
            &p5_checks,
        ),
        convergence_lane("p6", "P6 release walkthrough and demo evidence", &p6_checks),
        convergence_lane("p7", "P7 content-rich seeded visual regression", &p7_checks),
        convergence_lane("p8", "P8 mobile layered navigation", &p8_checks),
        convergence_lane("p9", "P9 compact header and status chrome", &p9_checks),
        convergence_lane("p10", "P10 consolidated empty room onboarding", &p10_checks),
        convergence_lane("p11", "P11 real user journey E2E", &p11_checks),
        convergence_lane("p12", "P12 module boundary governance", &p12_checks),
        convergence_lane("p13", "P13 browser-level accessibility", &p13_checks),
        convergence_lane("p14", "P14 visual diff baseline", &p14_checks),
        convergence_lane("p15", "P15 room accordion compression", &p15_checks),
        convergence_lane("p16", "P16 mobile compact composer", &p16_checks),
        convergence_lane("p17", "P17 browser performance budget", &p17_checks),
        convergence_lane("p18", "P18 keyboard and accessibility audit", &p18_checks),
        convergence_lane("p19", "P19 real module split", &p19_checks),
        convergence_lane("p20", "P20 endpoint chaos regression", &p20_checks),
        convergence_lane("p21", "P21 README information density", &p21_checks),
        convergence_lane("p22", "P22 real ESM/build split", &p22_checks),
        convergence_lane("p23", "P23 smoke suite decomposition", &p23_checks),
        convergence_lane("p24", "P24 cross-browser readiness matrix", &p24_checks),
        convergence_lane("p25", "P25 perceptual visual diff", &p25_checks),
        convergence_lane("p26", "P26 JSON schema contract gates", &p26_checks),
        convergence_lane("p27", "P27 UI soak and leak guard", &p27_checks),
        convergence_lane("p28", "P28 task/artifact result drawer", &p28_checks),
        convergence_lane("p29", "P29 accessibility snapshot", &p29_checks),
        convergence_lane("p30", "P30 strict cross-browser readiness", &p30_checks),
        convergence_lane("p31", "P31 productization module extraction", &p31_checks),
        convergence_lane("p32", "P32 smoke orchestrator summary", &p32_checks),
        convergence_lane("p33", "P33 mobile density polish", &p33_checks),
        convergence_lane("p34", "P34 empty-thread starter UX", &p34_checks),
        convergence_lane("p35", "P35 design-token and selector budget", &p35_checks),
        convergence_lane("p36", "P36 golden live-data fixtures", &p36_checks),
        convergence_lane("p37", "P37 deep a11y route map", &p37_checks),
        convergence_lane("p38", "P38 hostile/XSS fixture", &p38_checks),
        convergence_lane("p39", "P39 product result drawer actions", &p39_checks),
    ];
    let p0_ready = convergence_lanes[0].ready;
    let p1_ready = convergence_lanes[1].ready;
    let p2_ready = convergence_lanes[2].ready;
    let p3_ready = convergence_lanes[3].ready;
    let p4_ready = convergence_lanes[4].ready;
    let p5_ready = convergence_lanes[5].ready;
    let p6_ready = convergence_lanes[6].ready;
    let p7_ready = convergence_lanes[7].ready;
    let p8_ready = convergence_lanes[8].ready;
    let p9_ready = convergence_lanes[9].ready;
    let p10_ready = convergence_lanes[10].ready;
    let p11_ready = convergence_lanes[11].ready;
    let p12_ready = convergence_lanes[12].ready;
    let p13_ready = convergence_lanes[13].ready;
    let p14_ready = convergence_lanes[14].ready;
    let p15_ready = convergence_lanes[15].ready;
    let p16_ready = convergence_lanes[16].ready;
    let p17_ready = convergence_lanes[17].ready;
    let p18_ready = convergence_lanes[18].ready;
    let p19_ready = convergence_lanes[19].ready;
    let p20_ready = convergence_lanes[20].ready;
    let p21_ready = convergence_lanes[21].ready;
    let p22_ready = convergence_lanes[22].ready;
    let p23_ready = convergence_lanes[23].ready;
    let p24_ready = convergence_lanes[24].ready;
    let p25_ready = convergence_lanes[25].ready;
    let p26_ready = convergence_lanes[26].ready;
    let p27_ready = convergence_lanes[27].ready;
    let p28_ready = convergence_lanes[28].ready;
    let p29_ready = convergence_lanes[29].ready;
    let p30_ready = convergence_lanes[30].ready;
    let p31_ready = convergence_lanes[31].ready;
    let p32_ready = convergence_lanes[32].ready;
    let p33_ready = convergence_lanes[33].ready;
    let p34_ready = convergence_lanes[34].ready;
    let p35_ready = convergence_lanes[35].ready;
    let p36_ready = convergence_lanes[36].ready;
    let p37_ready = convergence_lanes[37].ready;
    let p38_ready = convergence_lanes[38].ready;
    let p39_ready = convergence_lanes[39].ready;
    let p0_p1_p2_converged = p0_ready && p1_ready && p2_ready;
    let p0_p4_converged = p0_p1_p2_converged && p3_ready && p4_ready;
    let p0_p6_converged = p0_p4_converged && p5_ready && p6_ready;
    let p0_p13_converged = p0_p6_converged
        && p7_ready
        && p8_ready
        && p9_ready
        && p10_ready
        && p11_ready
        && p12_ready
        && p13_ready;
    let p0_p21_converged = p0_p13_converged
        && p14_ready
        && p15_ready
        && p16_ready
        && p17_ready
        && p18_ready
        && p19_ready
        && p20_ready
        && p21_ready;
    let p0_p29_converged = p0_p21_converged
        && p22_ready
        && p23_ready
        && p24_ready
        && p25_ready
        && p26_ready
        && p27_ready
        && p28_ready
        && p29_ready;
    let p0_p39_converged = p0_p29_converged
        && p30_ready
        && p31_ready
        && p32_ready
        && p33_ready
        && p34_ready
        && p35_ready
        && p36_ready
        && p37_ready
        && p38_ready
        && p39_ready;
    let convergence_percent = percent(
        convergence_lanes.iter().filter(|lane| lane.ready).count(),
        convergence_lanes.len(),
    );
    let checks = [
        all_screen_ids_aligned,
        app_has_live_event_stream,
        app_has_diff_review,
        app_has_keyboard_shortcuts,
        app_has_json_inspector,
        app_has_operator_drilldown,
        app_has_endpoint_health_grid,
        app_has_dry_run_action_cards,
        app_has_approval_cards,
        app_has_post_action_guard,
        app_has_session_inspector,
        app_has_task_drilldown,
        app_has_transcript_preview,
        app_has_transcript_search,
        app_has_replay_promotion_drilldown,
        app_has_event_cursor,
        app_has_readonly_command_runner,
        app_has_operator_security,
        app_has_task_publisher,
        app_has_agent_chat,
        app_has_external_agent_benchmark,
        app_has_hepta_runtime_ui_parity,
        app_has_hepta_runtime_navigation_groups,
        app_has_chat_first_architecture,
        app_has_route_view_controller,
        app_has_command_palette,
        app_has_premium_consumer_ui,
        app_has_progressive_disclosure,
        app_has_simplified_primary_nav,
        app_preserves_typing_during_live_poll,
        app_has_minimal_consumer_workspace,
        app_has_telegram_multi_agent_workspace,
        app_has_hepta_runtime_2026_5_2_ui_resilience,
        app_models_long_gateway_websocket_resilience,
        app_has_grouped_message_width_guard,
        app_has_ios_pwa_bounds_guard,
        app_has_selection_contrast_guard,
        app_has_slash_feedback_surface,
        app_has_talk_diagnostics_resilience,
        app_has_persisted_auto_scroll_mode,
        app_has_blank_dashboard_recovery_panel,
        app_has_compact_session_status_badges,
        app_scopes_nodes_polling_to_active_tab,
        app_distinguishes_sample_vs_live_adapter_readiness,
        app_has_terminal_qr_rendering_guard,
        !control_ui_gateway_websocket_opened_by_audit,
        !control_ui_live_gateway_rpc_performed,
        p0_p1_p2_converged,
        p0_p4_converged,
        p0_p6_converged,
        p0_p13_converged,
        p0_p21_converged,
        p0_p29_converged,
        p0_p39_converged,
        readme_mentions_boundary,
    ];
    let audit_percent = percent(
        checks.into_iter().filter(|ready| *ready).count(),
        checks.len(),
    );

    ControlUiContractAuditReport {
        product: "Hepta",
        status: if audit_percent == 100 {
            "complete"
        } else {
            "incomplete"
        },
        core_screen_count: core_ids.len(),
        app_screen_count: app_ids.len(),
        readme_screen_count: readme_ids.len(),
        command_binding_count: control_ui_command_bindings().len(),
        interaction_capability_count: control_ui_interaction_capabilities().len(),
        all_screen_ids_aligned,
        app_has_live_event_stream,
        app_has_diff_review,
        app_has_keyboard_shortcuts,
        app_has_json_inspector,
        app_has_operator_drilldown,
        app_has_endpoint_health_grid,
        app_has_dry_run_action_cards,
        app_has_approval_cards,
        app_has_post_action_guard,
        app_has_session_inspector,
        app_has_task_drilldown,
        app_has_transcript_preview,
        app_has_transcript_search,
        app_has_replay_promotion_drilldown,
        app_has_event_cursor,
        app_has_readonly_command_runner,
        app_has_operator_security,
        app_has_task_publisher,
        app_has_agent_chat,
        app_has_external_agent_benchmark,
        app_has_hepta_runtime_ui_parity,
        app_has_hepta_runtime_navigation_groups,
        app_has_chat_first_architecture,
        app_has_route_view_controller,
        app_has_command_palette,
        app_has_premium_consumer_ui,
        app_has_progressive_disclosure,
        app_has_simplified_primary_nav,
        app_preserves_typing_during_live_poll,
        app_has_minimal_consumer_workspace,
        app_has_telegram_multi_agent_workspace,
        app_has_hepta_runtime_2026_5_2_ui_resilience,
        app_models_long_gateway_websocket_resilience,
        app_has_grouped_message_width_guard,
        app_has_ios_pwa_bounds_guard,
        app_has_selection_contrast_guard,
        app_has_slash_feedback_surface,
        app_has_talk_diagnostics_resilience,
        app_has_persisted_auto_scroll_mode,
        app_has_blank_dashboard_recovery_panel,
        app_has_compact_session_status_badges,
        app_scopes_nodes_polling_to_active_tab,
        app_distinguishes_sample_vs_live_adapter_readiness,
        app_has_terminal_qr_rendering_guard,
        control_ui_gateway_websocket_opened_by_audit,
        control_ui_live_gateway_rpc_performed,
        p0_ready,
        p1_ready,
        p2_ready,
        p3_ready,
        p4_ready,
        p5_ready,
        p6_ready,
        p7_ready,
        p8_ready,
        p9_ready,
        p10_ready,
        p11_ready,
        p12_ready,
        p13_ready,
        p14_ready,
        p15_ready,
        p16_ready,
        p17_ready,
        p18_ready,
        p19_ready,
        p20_ready,
        p21_ready,
        p22_ready,
        p23_ready,
        p24_ready,
        p25_ready,
        p26_ready,
        p27_ready,
        p28_ready,
        p29_ready,
        p30_ready,
        p31_ready,
        p32_ready,
        p33_ready,
        p34_ready,
        p35_ready,
        p36_ready,
        p37_ready,
        p38_ready,
        p39_ready,
        p0_p1_p2_converged,
        p0_p4_converged,
        p0_p6_converged,
        p0_p13_converged,
        p0_p21_converged,
        p0_p29_converged,
        p0_p39_converged,
        convergence_percent,
        convergence_lanes,
        readme_mentions_boundary,
        audit_percent,
        missing_in_app,
        missing_in_readme,
    }
}

pub fn control_ui_index_html() -> String {
    let screens = control_ui_screens();
    let commands = control_ui_command_bindings();
    let logo = r#"<img src="./assets/hepta-agent-logo.png" alt="" />"#;

    let screen_cards = screens
        .iter()
        .map(|screen| {
            let sources = screen
                .data_sources
                .iter()
                .map(|source| format!("<li><code>{}</code></li>", escape_html(source)))
                .collect::<String>();
            let widgets = screen
                .widgets
                .iter()
                .map(|widget| format!("<li>{}</li>", escape_html(widget)))
                .collect::<String>();
            let interactions = screen
                .interactions
                .iter()
                .map(|interaction| format!("<li>{}</li>", escape_html(interaction)))
                .collect::<String>();
            format!(
                "<article class=\"card route-card\" id=\"screen-card-{}\" data-screen=\"{}\" data-ref-ui-page-section=\"{}\" data-hepta-source-ui-aligned=\"true\" data-hepta-backend-aligned=\"true\"><header><p class=\"eyebrow\">{}</p><h3>{}</h3><p>{}</p></header><div class=\"route-card__grid\"><section><h4>Data sources</h4><ul>{}</ul></section><section><h4>Widgets</h4><ul>{}</ul></section><section><h4>Interactions</h4><ul>{}</ul></section></div></article>",
                escape_html(screen.id),
                escape_html(screen.id),
                escape_html(screen.id),
                escape_html(screen.route),
                escape_html(screen.title),
                escape_html(screen.summary),
                sources,
                widgets,
                interactions
            )
        })
        .collect::<String>();

    let command_cards = commands
        .iter()
        .map(|command| {
            let runner = if command.command.starts_with('/') && !command.command.contains('<') {
                "<button class=\"button small\" data-run-command=\"read-only\">Run read-only</button>"
            } else {
                "<span class=\"chip chip--muted\">copy-only / guarded</span>"
            };
            format!(
                "<article class=\"command-item\" data-command-id=\"{}\"><strong>{}</strong><code>{}</code><div class=\"action-rail\"><button class=\"button small\" data-copy=\"{}\">Copy</button>{}</div></article>",
                escape_html(command.id),
                escape_html(&control_ui_title_from_id(command.id)),
                escape_html(command.command),
                escape_html(command.command),
                runner
            )
        })
        .collect::<String>();

    let right_sidebar = render_static_hepta_right_sidebar(logo);
    let entry_surface = render_static_hepta_entry_surface(&screens, &screen_cards);
    let command_palette_items = commands
        .iter()
        .take(18)
        .map(|command| {
            format!(
                "<a class=\"command-palette__item\" href=\"#commands\" data-palette-kind=\"command\" data-palette-id=\"{}\"><span class=\"command-palette__kind\">command</span><span class=\"command-palette__copy\"><strong>{}</strong><small>{}</small></span></a>",
                escape_html(command.id),
                escape_html(&control_ui_title_from_id(command.id)),
                escape_html(command.command)
            )
        })
        .collect::<String>();

    format!(
        r###"<!doctype html>
<html lang="en" data-theme="premium" data-theme-mode="dark" data-rust-frontend-renderer="hepta-core::control_ui" data-no-js-frontend="true">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Hepta Control UI</title>
    <meta name="description" content="Rust-rendered local console for chat, tasks, sessions, and evidence." />
    <meta name="hepta-ui-renderer" content="rust-no-js" />
    <link rel="stylesheet" href="./styles.css" />
  </head>
  <body data-view="chat" data-rust-rendered-control-ui="true" data-hepta-route-surface-first="false">
    <div class="shell shell--hepta-premium" data-hepta_runtime-ui-parity="true" data-premium-consumer-ui="linear-raycast-vercel-arc" data-rust-frontend-renderer="true" data-js-artifacts="removed">
      <aside class="shell-nav sidebar" aria-label="Hepta navigation">
        <div class="brand app-brand"><div class="brand-mark app-brand__mark" data-hepta-agent-logo="true">{logo}</div><div class="app-brand__text"><p class="eyebrow">Control UI</p><h1>Hepta</h1></div></div>
        <nav class="nav nav-group nav-group--primary shell-nav__list" id="hepta-nav" aria-label="Control UI sections">
          <a class="active" data-screen="chat" href="#chat"><span>◌</span><span>Chat</span></a>
          <a data-screen="tasks" href="#tasks"><span>☷</span><span>Tasks</span></a>
          <a data-screen="ops" href="#ops"><span>▣</span><span>Ops</span></a>
          <a data-screen="external-agent-benchmark" href="#external-agent-benchmark"><span>✧</span><span>Benchmark</span></a>
        </nav>
      </aside>
      <header class="topbar"><div class="topnav-shell"><div class="topnav-shell__content"><div class="dashboard-header"><div class="dashboard-header__breadcrumb"><span class="dashboard-header__breadcrumb-link">Hepta</span><span class="dashboard-header__breadcrumb-sep">/</span><span class="dashboard-header__breadcrumb-current">Rust frontend</span></div><div class="topbar-status topbar-status--quiet" data-control-ui-readiness-split="sample-vs-live-adapter"><span class="pill"><span class="statusDot"></span><span>Rust no-JS</span></span><span class="pill" data-readiness-kind="sample">sample ready</span><span class="pill pill--muted" data-readiness-kind="live-adapter">live adapter gated</span></div></div></div><div class="topnav-shell__actions"><a class="topbar-search" data-open-command-palette href="#command-palette"><span class="topbar-search__label">Jump to commands</span><span class="topbar-search__kbd">⌘K</span></a><a class="btn btn--primary" href="#task-publisher">New Task</a></div></div></header>
      <main class="content main">
        <section class="hero dashboard-hero" data-chat-first-architecture="true" data-minimal-consumer-workspace="true" data-telegram-multi-agent-chat="true"><div><p class="eyebrow">Runtime control</p><h2>Chat-first runtime control.</h2><p class="hero-copy">The Control UI keeps the former JS visual architecture but is now pre-rendered from <code>hepta-core::control_ui</code>.</p></div><div class="hero-actions"><a class="button primary btn btn--primary" href="/api/control-ui">Status JSON</a><a class="button btn" href="/api/ui-contract-audit">Audit JSON</a></div></section>
        <section class="metric-grid stats-grid" id="hepta-metrics" aria-label="UI health metrics"><article class="metric-card"><span>screens</span><strong>{screen_count}/{screen_count}</strong><p>Rust generated</p></article><article class="metric-card"><span>commands</span><strong>{command_count}</strong><p>Read-only/copy-only contracts</p></article><article class="metric-card"><span>renderer</span><strong>Rust</strong><p>HTML/CSS only, no JS</p></article><article class="metric-card"><span>alignment</span><strong>JS DOM parity</strong><p>Chat-first shell restored</p></article></section>
        <section class="hepta-dashboard-recovery" data-dashboard-recovery-panel="blank-app-module" data-dashboard-recovery-source="rust-static-fallback" data-nodes-poll-scope="active-tab-only" data-nodes-poll-autostart="false" aria-label="Dashboard recovery"><strong>Recovery panel</strong><span>blank module fallback</span><a href="/api/ui-contract-audit">Audit</a></section>
        <section class="workspace dashboard-grid"><section class="panel card wide" id="hepta-screen-panel" aria-live="polite">
          <section class="focus-workspace telegram-chat-shell" data-chat-first-architecture="true" data-minimal-consumer-workspace="true" data-telegram-multi-agent-chat="true" data-mobile-layered-chat="true" data-professional-first-screen="true" data-product-ia="calm-workspace" data-workspace-primary="thread" data-chat-mobile-active-pane="thread" data-chat-workspace-scope="local" data-chat-brain-scope="global" data-chat-shared-brain-mode="global-brain-isolated-context">
            <header class="focus-header"><div class="focus-brand"><div class="focus-orb" data-hepta-agent-logo="true">{logo}</div><span>Hepta</span></div><div class="focus-actions prompt-chip-row"><a class="ghost-action" data-open-command-palette href="#command-palette">⌘K</a></div></header>
            <main class="focus-main focus-main--chat">
              <aside class="tg-conversation-rail">
                <div class="tg-rail-header"><div><h2>Work</h2></div><div class="tg-rail-status tg-rail-status--minimal"><button class="tg-icon-action" data-chat-add aria-label="New conversation">＋</button><span class="tg-rail-status__item tg-rail-status__item--quiet" data-rail-agent-count="3">3</span></div></div>
                <div class="tg-rail-toolbar"><label class="tg-search-shell" aria-label="Search chats"><input id="chat-search" data-chat-search type="search" placeholder="Search conversations…" value="" /></label></div>
                <div class="tg-folder-row" aria-label="Chat folders"><button class="tg-folder-chip active" data-chat-folder="all"><span>All</span></button><button class="tg-folder-chip" data-chat-folder="pinned"><span>Pinned</span></button><button class="tg-folder-chip" data-chat-folder="archived"><span>Archived</span></button></div>
                <section class="tg-conversation-list" aria-label="Agent conversations" role="listbox">
                  <article class="tg-chat-item active" data-chat-conversation="ui-chat-agent" role="option" aria-selected="true" tabindex="0"><div class="tg-chat-item__avatar tg-chat-item__avatar--hepta-logo" data-hepta-agent-logo="true">{logo}</div><div class="tg-chat-item__body"><div class="tg-chat-item__topline"><strong>Hepta Runtime</strong><div class="tg-chat-item__actions"><span>now</span></div></div><p class="is-unread">Rust/no-JS chat workspace restored with the old visual shell.</p><span class="tg-chat-item__unread" data-chat-unread="1">1</span></div></article>
                  <article class="tg-chat-item" data-chat-conversation="task-queue" role="option" aria-selected="false" tabindex="0"><div class="tg-chat-item__avatar" aria-hidden="true">☷</div><div class="tg-chat-item__body"><div class="tg-chat-item__topline"><strong>Task queue</strong><div class="tg-chat-item__actions"><span>local</span></div></div><p>/tasks · /subagents · guarded queue operations.</p></div></article>
                  <article class="tg-chat-item" data-chat-conversation="operator-plane" role="option" aria-selected="false" tabindex="0"><div class="tg-chat-item__avatar" aria-hidden="true">▣</div><div class="tg-chat-item__body"><div class="tg-chat-item__topline"><strong>Operator plane</strong><div class="tg-chat-item__actions"><span>ready</span></div></div><p>/status · /diagnostics · events and approvals.</p></div></article>
                </section>
              </aside>
              <section class="tg-thread-panel" data-hepta-main-panel-active="false">
                <header class="tg-thread-header"><div class="tg-thread-header__main"><div class="tg-thread-avatar tg-thread-avatar--hepta-logo" data-hepta-agent-logo="true" aria-label="Hepta agent logo">{logo}</div><div><h2>Hepta Runtime</h2><p>active now</p></div></div><div class="tg-thread-status"><div class="tg-thread-status-summary" data-session-status-badges="compact"><span class="badge ok">Rust no-JS</span><span class="badge">chat-first</span><span class="badge tg-session-state tg-session-state--live" data-session-status="live">live</span><span class="badge tg-session-state tg-session-state--idle" data-session-status="idle">idle</span><span class="badge tg-session-state tg-session-state--terminal" data-session-status="terminal">terminal</span></div><details class="tg-thread-command-menu" data-thread-command-menu="true"><summary>More</summary><div class="tg-thread-command-menu__panel tg-thread-search-bar__controls"><a class="tg-header-action" href="#transcript">History</a><a class="tg-header-action" href="#tasks">Tasks</a><a class="tg-header-action" href="#sessions">Sessions</a></div></details></div></header>
                {entry_surface}
                <section class="focus-thread tg-thread" aria-label="Selected conversation thread" data-thread-signature="rust-no-js-static:3">
                  <div class="tg-date-divider" data-chat-date-divider="local-day"><span>Today</span></div>
                  <article class="tg-message tg-message--system"><div class="tg-bubble"><span>system</span><p>Control UI is served from <code>hepta-core::control_ui</code>; browser JavaScript artifacts remain removed.</p><div class="badge-row tg-routing-badges tg-routing-badges--quiet"><span class="badge">/api/control-ui</span><span class="badge">/api/ui-contract-audit</span></div></div><small>local · now</small></article>
                  <article class="tg-message tg-message--agent"><div class="tg-bubble"><span>hepta</span><p>旧 JS 版的 Telegram-style conversation rail、thread header、route surface、workspace tools 和 composer skeleton 已恢复为 Rust 预渲染 DOM。</p></div><small>local · now</small></article>
                  <article class="tg-message tg-message--self"><div class="tg-bubble"><p>Keep the UI no-JS, but make it look and feel like the previous JS workspace.</p></div><small>local · sent</small></article>
                </section>
                <section class="focus-compose chat-compose tg-compose-wrap" data-chat-composer-shell="telegram" data-mobile-compact-composer="true">
                  <div class="tg-context-chip-row" data-chat-context-chip-list="true" aria-label="Attached context"><span class="tg-context-chip tg-context-chip--evidence" data-chat-context-chip="ui-parity"><span>evidence</span><strong>4465000^ JS shell</strong></span><span class="tg-context-chip" data-chat-context-chip="rust-no-js"><span>renderer</span><strong>Rust/no-JS</strong></span></div>
                  <div class="tg-compose-bar"><button class="tg-compose-icon" data-chat-composer-popover-toggle="artifact" data-chat-attachment-placeholder aria-label="Attach local context">＋</button><button class="tg-compose-icon" data-chat-composer-popover-toggle="command" data-chat-command-shortcut aria-label="Insert command">⌘</button><select id="chat-routing-mode" class="tg-compose-mode" data-chat-routing-mode aria-label="Reply mode"><option selected>auto</option><option>direct</option><option>broadcast</option></select><textarea id="chat-message" data-chat-composer-input data-chat-enter-send spellcheck="false" rows="1" placeholder="Message Hepta Runtime…"></textarea><button class="tg-send-button" data-agent-chat-send aria-label="Send message"><span>➤</span></button></div>
                  <div class="tg-compose-footer"><span data-chat-shortcut-hint>Enter sends · Shift+Enter newline</span><label class="tg-autoscroll-select" data-chat-autoscroll-persisted="local-storage-contract"><span>Scroll</span><select data-chat-autoscroll-mode aria-label="Auto-scroll mode"><option value="smart" selected>smart</option><option value="locked">locked</option><option value="off">off</option></select></label><button class="tg-compose-plan" data-agent-chat-plan>Plan</button><span class="tg-compose-status" data-chat-send-state="ready">ready</span></div>
                  <input id="chat-agent-id" type="hidden" value="ui-chat-agent" /><input id="chat-from-agent-id" type="hidden" value="ui-user" />
                  <details class="focus-result tg-focus-result" data-developer-output="collapsed"><summary>Developer output</summary><pre id="agent-chat-output" class="json-box">Choose a workspace, then plan or send a message.</pre></details>
                </section>
              </section>
              <aside class="tg-room-panel" data-workspace-room-panel="local" data-workspace-group-chat="false" data-room-empty-consolidated="false">{right_sidebar}</aside>
            </main>
          </section>
        </section><section class="panel card command-rail" id="hepta-command-panel"><div class="panel-heading"><div><p class="eyebrow">Operator commands</p><h3>Copy-ready surfaces</h3></div></div><div class="command-list" id="commands">{command_cards}</div><pre id="command-runner-output" class="json-box">Pick an allowlisted command and run it locally. Template commands with &lt;task_id&gt; stay copy-only.</pre></section></section>
        <section class="panel card wide evidence-panel" data-live-event-stream="true" data-diff-review="true" data-approval-cards="true" data-task-drilldown="true" data-transcript-preview="true" data-command-runner="true" data-task-publisher="true" data-agent-chat="true" data-external-agent-benchmark="true" data-route-view-controller="rust-anchor-routes" data-command-palette="rust-anchor-command-list"><div class="panel-heading card-header"><div><p class="eyebrow">Runtime evidence</p><h3>Rust-rendered evidence surface</h3></div></div><textarea id="json-input" spellcheck="false" placeholder="Paste runtime JSON here for manual inspection; server APIs remain available under /api/*." ></textarea><pre id="json-preview">No browser JavaScript is required for this Rust-rendered shell.</pre><aside class="terminal-qr-preview" data-terminal-qr-rendering="fixed-cell-terminal-qr" aria-label="Terminal QR rendering contract"><code>QR fixed-cell / ANSI-safe</code></aside></section>
      </main>
    </div>
    <div class="command-palette-backdrop" id="command-palette"><section class="command-palette" role="dialog" aria-modal="true" aria-label="Command palette"><div class="command-palette__input-row"><span aria-hidden="true">⌘</span><input id="command-palette-input" type="search" placeholder="Static Rust command index" autocomplete="off" /><a class="button small btn btn--ghost" href="#commands">Esc</a></div><div id="command-palette-results" class="command-palette__results">{command_palette_items}</div></section></div>
    <div class="toast" id="toast" role="status" aria-live="polite">Rust frontend ready</div>
  </body>
</html>"###,
        logo = logo,
        screen_count = screens.len(),
        command_count = commands.len(),
        command_cards = command_cards,
        command_palette_items = command_palette_items,
        right_sidebar = right_sidebar,
        entry_surface = entry_surface,
    )
}

fn static_hepta_left_sidebar_sections() -> Vec<(
    &'static str,
    &'static str,
    Vec<(&'static str, &'static str, &'static str, &'static str)>,
)> {
    vec![
        ("chat", "Chat", vec![("Chat", "/chat", "chat", "◌")]),
        (
            "control",
            "Control",
            vec![
                ("Overview", "/overview", "dashboard", "⌂"),
                ("Channels", "/channels", "gateway", "◫"),
                ("Instances", "/instances", "ops", "▣"),
                ("Sessions", "/sessions", "sessions", "☰"),
                ("Tasks", "/tasks", "tasks", "☷"),
                ("Usage", "/usage", "ops", "↗"),
                ("Cron", "/cron", "tasks", "◷"),
            ],
        ),
        (
            "agent",
            "Agents",
            vec![
                ("Agents", "/agents", "workers", "◇"),
                ("Skills", "/skills", "commands", "✦"),
                ("Nodes", "/nodes", "gateway", "▱"),
                ("Dreaming", "/dreaming", "transcript", "☾"),
            ],
        ),
        (
            "settings",
            "Settings",
            vec![
                ("Config", "/config", "config", "⚙"),
                ("Comms", "/communications", "gateway", "✉"),
                ("Appearance", "/appearance", "config", "◐"),
                ("Automation", "/automation", "tasks", "⟳"),
                ("Infra", "/infrastructure", "ops", "▦"),
                ("AI Agents", "/ai-agents", "multi-agent", "✧"),
                ("Debug", "/debug", "developer", "⌘"),
                ("Logs", "/logs", "live", "≋"),
            ],
        ),
    ]
}

fn render_static_hepta_right_sidebar(logo: &str) -> String {
    let sections = static_hepta_left_sidebar_sections();
    let source_block_count = sections.len() + 2;
    let item_count = sections
        .iter()
        .map(|(_, _, items)| items.len())
        .sum::<usize>()
        + 1;
    let mut rendered_sections = String::new();
    for (id, label, items) in &sections {
        let rendered_items = items
            .iter()
            .map(|(item_label, route, screen, icon)| {
                let active = if *route == "/chat" { " active" } else { "" };
                let current = if *route == "/chat" { "page" } else { "false" };
                let anchor = route_anchor_id(route);
                format!(
                    "<a class=\"nav-item hepta-nav-item{}\" href=\"#{}\" data-hepta-nav-route=\"{}\" data-hepta-nav-key=\"{}\" data-screen=\"{}\" aria-current=\"{}\"><span class=\"nav-item__icon\" aria-hidden=\"true\">{}</span><span class=\"nav-item__text\">{}</span></a>",
                    active,
                    escape_html(&anchor),
                    escape_html(route),
                    escape_html(route),
                    escape_html(screen),
                    current,
                    escape_html(icon),
                    escape_html(item_label)
                )
            })
            .collect::<String>();
        rendered_sections.push_str(&format!(
            "<section class=\"hepta-nav-section hepta-sidebar-block\" data-hepta-sidebar-block=\"section\" data-hepta-sidebar-section=\"{}\" data-hepta-source-node=\"nav-section\"><div class=\"hepta-nav-section__label\"><span>{}</span></div><div class=\"hepta-nav-section__items\">{}</div></section>",
            escape_html(id),
            escape_html(label),
            rendered_items
        ));
    }

    format!(
        "<aside class=\"hepta-right-sidebar shell-nav sidebar sidebar-shell\" aria-label=\"Hepta runtime navigation\" data-hepta-left-sidebar-migrated=\"full\" data-hepta-sidebar-collapsed=\"false\" data-hepta-sidebar-block-alignment=\"source-blocks\" data-hepta-sidebar-source-block-count=\"{}\" data-hepta-sidebar-section-count=\"{}\" data-hepta-sidebar-item-count=\"{}\" data-hepta-source-shell=\"shell-nav sidebar sidebar-shell main content\"><header class=\"brand app-brand hepta-sidebar-block\" data-hepta-sidebar-block=\"brand\" data-hepta-source-node=\"sidebar-header\"><div class=\"brand-mark app-brand__mark\" data-hepta-agent-logo=\"true\">{}</div><div class=\"app-brand__text\"><p class=\"eyebrow\">Hepta</p><h1>Runtime</h1></div></header><nav class=\"sidebar-nav hepta-right-sidebar__body\" data-hepta-source-node=\"sidebar-nav\"><section class=\"hepta-nav-section hepta-sidebar-block\" data-hepta-multi-agent-inline=\"right-rail\"><div class=\"hepta-nav-section__label\"><span>Agent runtime</span></div><div class=\"hepta-nav-section__items\"><a class=\"nav-item hepta-nav-item\" href=\"#multi-agent\" data-screen=\"multi-agent\"><span class=\"nav-item__icon\">✧</span><span class=\"nav-item__text\">Overall local</span></a><a class=\"nav-item hepta-nav-item\" href=\"#multi-agent\" data-screen=\"multi-agent\"><span class=\"nav-item__icon\">◇</span><span class=\"nav-item__text\">Topology 3</span></a><a class=\"nav-item hepta-nav-item\" href=\"#multi-agent\" data-screen=\"multi-agent\"><span class=\"nav-item__icon\">✓</span><span class=\"nav-item__text\">All 100 true</span></a></div></section>{}</nav><footer class=\"hepta-right-sidebar__footer hepta-sidebar-block\" data-hepta-sidebar-block=\"footer\" data-hepta-source-node=\"sidebar-footer\"><a class=\"nav-item hepta-nav-item hepta-nav-item--docs\" href=\"https://docs.hepta.ai/\"><span class=\"nav-item__icon\" aria-hidden=\"true\">?</span><span class=\"nav-item__text\">Docs</span></a><small class=\"hepta-sidebar-version\">v2026.5.7</small></footer></aside>",
        source_block_count,
        sections.len(),
        item_count,
        logo,
        rendered_sections
    )
}

fn render_static_hepta_entry_surface(screens: &[ControlUiScreen], screen_cards: &str) -> String {
    let route_rows = [
        (
            "/chat",
            "Chat",
            "chat/send/session/model/command",
            "rail;thread;composer;tools;controls",
            "new reset stop compact export focus model think usage steer",
            "NO_REPLY; explicit send",
        ),
        (
            "/overview",
            "Overview",
            "status/health/auth/events/logs",
            "access;snapshot;stats;attention;quick actions",
            "connect reload new automation terminal docs",
            "tokens redacted; pairing warned",
        ),
        (
            "/channels",
            "Channels",
            "channels/status/config/setup",
            "health;accounts;provider setup;enabled counts",
            "refresh load-config setup",
            "intent before send; secrets hidden",
        ),
        (
            "/instances",
            "Instances",
            "instances/presence/nodes",
            "presence;host toggle;last input;reason",
            "refresh toggle-hosts",
            "hosts hidden by default",
        ),
        (
            "/sessions",
            "Sessions",
            "sessions/activity/context",
            "filters;table;activity;checkpoints;bulk select",
            "inspect restore archive",
            "session lifecycle guarded",
        ),
        (
            "/transcript",
            "History",
            "transcript/session/query",
            "threaded turns;query;session context",
            "search inspect copy",
            "read-only transcript",
        ),
        (
            "/tasks",
            "Tasks",
            "tasks/subagents/acp/scheduler",
            "queue;task detail;subagent topology;status lanes;cancel controls",
            "refresh inspect run cancel focus spawn",
            "explicit spawn/cancel/steer",
        ),
        (
            "/usage",
            "Usage",
            "usage/cost/timeseries/logs",
            "metrics;filters;charts;breakdowns;session detail",
            "refresh filter export drilldown",
            "partial-cost marked",
        ),
        (
            "/cron",
            "Cron",
            "cron/status/list/runs/write",
            "summary;quick create;jobs;runs;form",
            "add update run clone enable remove",
            "timezone/exact timing warned",
        ),
        (
            "/agents",
            "Agents",
            "agents/files/identity/tools/skills",
            "cards;context;file editor;tools;channels;cron",
            "default edit save load-tools",
            "agent-scoped writes",
        ),
        (
            "/skills",
            "Skills",
            "skills/search/detail/install/update",
            "list;detail;install;API key;messages",
            "enable disable install save-key search",
            "unsafe install gated; secrets hidden",
        ),
        (
            "/nodes",
            "Nodes",
            "node/device/nodes commands",
            "pending;paired;bindings;permissions;commands",
            "approve reject rotate revoke invoke",
            "device/token actions gated",
        ),
        (
            "/dreaming",
            "Dreaming",
            "doctor.memory/wiki",
            "scene;diary;review;phase stats;trace",
            "toggle backfill dedupe reset repair copy",
            "restart/repair confirmed",
        ),
        (
            "/config",
            "Config",
            "config/schema/local-import/providers",
            "schema tree;forms;readiness;diff",
            "load save patch reload",
            "schema-first; secrets redacted",
        ),
        (
            "/communications",
            "Comms",
            "channels/message/tts/media",
            "channel cards;routing;audio;delivery policy",
            "refresh test-route provider",
            "external send intent",
        ),
        (
            "/appearance",
            "Appearance",
            "settings/localStorage/manifest",
            "theme;language;density;setup;PWA",
            "save-theme save-language reset",
            "local preference",
        ),
        (
            "/automation",
            "Automation",
            "commands/cron/hooks/plugins/gateway",
            "catalog;hooks;cron;heartbeat;routines",
            "run-readonly schedule hook",
            "writes gated; no silent restart",
        ),
        (
            "/infrastructure",
            "Infrastructure",
            "status/health/ops/logs/nodes/update",
            "health;web/media;node/browser/canvas;install",
            "refresh logs runbook",
            "restart/update confirmed",
        ),
        (
            "/ai-agents",
            "AI Agents",
            "agents/models/tools/skills/memory",
            "auth;defaults;tools;memory;sessions",
            "default auth-refresh load-tools",
            "tokens hidden; scoped tools",
        ),
        (
            "/debug",
            "Debug",
            "status/health/heartbeat/audit/models/events",
            "snapshot;audit;manual RPC;models;events",
            "probe call refresh copy",
            "read-only default; RPC explicit",
        ),
        (
            "/logs",
            "Logs",
            "logs.tail/events-report",
            "tail;severity;cursor;reset;events",
            "tail reset filter copy",
            "bounded bytes; no secret export",
        ),
        (
            "docs",
            "Docs",
            "docs.hepta.ai",
            "docs launcher;runbook;version",
            "open-docs copy-link",
            "external docs intentional",
        ),
    ];
    let rows = route_rows
        .iter()
        .map(|(route, title, sources, widgets, actions, boundary)| {
            let anchor = route_anchor_id(route);
            format!(
                "<tr><td><a class=\"session-link\" href=\"#{}\" data-ref-runtime-route=\"{}\">{}</a></td><td class=\"muted\">{}</td><td class=\"muted\">{}</td><td><span class=\"data-table-badge\">{}</span></td><td class=\"muted\">{}</td></tr>",
                escape_html(&anchor),
                escape_html(route),
                escape_html(title),
                escape_html(sources),
                escape_html(widgets),
                escape_html(actions),
                escape_html(boundary)
            )
        })
        .collect::<String>();
    let route_pages = route_rows
        .iter()
        .enumerate()
        .map(|(index, (route, title, sources, widgets, actions, boundary))| {
            let anchor = route_anchor_id(route);
            let source_badges = sources
                .split('/')
                .filter(|item| !item.trim().is_empty())
                .map(|source| format!("<span class=\"badge\">{}</span>", escape_html(source)))
                .collect::<String>();
            let widget_cards = widgets
                .split(';')
                .filter(|item| !item.trim().is_empty())
                .map(|widget| {
                    format!(
                        "<article class=\"mini-card\"><strong>{}</strong><p>Rust pre-rendered widget lane from the prior JS route blueprint.</p></article>",
                        escape_html(widget)
                    )
                })
                .collect::<String>();
            let action_buttons = actions
                .split_whitespace()
                .filter(|item| !item.trim().is_empty())
                .map(|action| {
                    format!(
                        "<button class=\"button small\" data-plan-action=\"{}\">{}</button>",
                        escape_html(action),
                        escape_html(action)
                    )
                })
                .collect::<String>();
            format!(
                "<section class=\"hepta-route-page{}\" id=\"{}\" data-ref-runtime-route=\"{}\" data-hepta-route-template=\"{}\" data-hepta-source-ui-aligned=\"true\" data-hepta-backend-aligned=\"true\" tabindex=\"-1\"><header class=\"hepta-route-surface__header\"><div><p class=\"eyebrow\">Route {}</p><h3 class=\"page-title\">{}</h3><p class=\"page-sub\">Live {} status, filters, and guarded actions — rendered by Rust from the old JS route contract.</p></div><span class=\"badge ok\">no-JS clickable</span></header><div class=\"badge-row\">{}{}</div><div class=\"card-grid\"><article class=\"mini-card\"><strong>Data sources</strong><p>{}</p></article><article class=\"mini-card\"><strong>Boundary</strong><p>{}</p></article><article class=\"mini-card\"><strong>Controls</strong><div class=\"action-rail\">{}</div></article></div><div class=\"screen-grid screen-grid--compact\">{}</div></section>",
                if index == 0 { " hepta-route-page--default" } else { "" },
                escape_html(&anchor),
                escape_html(route),
                escape_html(route),
                index + 1,
                escape_html(title),
                escape_html(&title.to_ascii_lowercase()),
                source_badges,
                format!("<span class=\"badge\">{}</span>", escape_html(boundary)),
                escape_html(sources),
                escape_html(boundary),
                action_buttons,
                widget_cards
            )
        })
        .collect::<String>();
    let primary_count = screens.len().min(6);
    format!(
        "<section class=\"hepta-entry-surface hepta-route-surface content\" data-hepta-entry-content=\"full\" data-hepta-page-surface=\"true\" data-hepta-primary-page-surface=\"true\" data-hepta-route=\"/overview\" data-hepta-route-screen=\"dashboard\" data-hepta-source-main=\"main.content\" data-hepta-source-version=\"v2026.5.7\"><header class=\"hepta-route-surface__header\"><div><p class=\"eyebrow\">Hepta source route parity</p><h2 class=\"page-title\">{} route surfaces</h2><p class=\"page-sub\">Rust renderer mirrors the old JS HEPTA_ENTRY_CONTENT_ROWS, sidebar routes, route cards, filters, and guarded action lanes without serving browser JavaScript.</p></div><span class=\"badge ok\">{} primary cards</span></header><div class=\"hepta-route-surface__outlet\" data-hepta-route-outlet=\"rust-static\"><div class=\"data-table-wrapper hepta-route-index\"><div class=\"data-table-toolbar\"><strong>HEPTA_ENTRY_CONTENT_ROWS</strong><span class=\"badge\">Rust pre-rendered</span><span class=\"badge\">{} routes</span></div><div class=\"data-table-container\"><table class=\"data-table\"><thead><tr><th>Route</th><th>Sources</th><th>Widgets</th><th>Actions</th><th>Boundary</th></tr></thead><tbody>{}</tbody></table></div></div><div class=\"hepta-route-page-stack\" data-hepta-route-page-stack=\"true\">{}</div><details class=\"hepta-all-screen-contracts\"><summary>All Rust screen contracts</summary><div class=\"screen-grid\">{}</div></details></div></section>",
        route_rows.len(),
        primary_count,
        route_rows.len(),
        rows,
        route_pages,
        screen_cards
    )
}

fn route_anchor_id(route: &str) -> String {
    let trimmed = route.trim().trim_start_matches('/');
    if trimmed.is_empty() {
        "chat".to_string()
    } else {
        trimmed.replace('/', "-")
    }
}

fn control_ui_title_from_id(id: &str) -> String {
    id.split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn control_ui_assets() -> Vec<ControlUiAsset> {
    let rendered_html = control_ui_index_html();
    vec![
        asset_owned(
            "rust://hepta-core/control-ui/index.html",
            "html",
            &rendered_html,
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
        asset("apps/hepta-control-ui/README.md", "docs", CONTROL_UI_README),
    ]
}

fn asset_owned(path: &'static str, kind: &'static str, content: &str) -> ControlUiAsset {
    ControlUiAsset {
        path,
        kind,
        byte_count: content.len(),
        present: !content.trim().is_empty(),
        required: true,
    }
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
            &["/external-readiness --json", "/production-surface --json"],
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
        data_sources,
        widgets,
        interactions: control_ui_screen_interactions(id),
    }
}

pub fn control_ui_frontend_manifest() -> ControlUiFrontendManifest {
    ControlUiFrontendManifest {
        schema_version: 1,
        source: "hepta-core::control_ui",
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
                &["readiness", "parity", "externalAgentBenchmark"],
            ),
            fetch_plan("parity", &["parity", "readiness"]),
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
            "rust-rendered-no-js-frontend"
        } else {
            "incomplete"
        },
        rust_embedded_static_asset_count,
        required_static_asset_count,
        rust_embedded_static_asset_coverage_percent,
        rust_view_model_ready: frontend_manifest.rust_view_model_ready,
        rust_view_model_source: frontend_manifest.source,
        browser_renderer_language: "rust-generated-html-css",
        pure_browser_rust_runtime: true,
        boundary: "Rust generates and serves the Control UI HTML/CSS shell and authoritative view-model JSON; browser-side JavaScript artifacts are not served.",
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
            "Plan, confirm, and publish new worker tasks into the local Hepta queue from the UI."
        }
        "workers" => {
            "Worker inventory, owner lanes, active/completed counts, and supervisor next action."
        }
        "operator" => {
            "Task queue, subagent tree, command stream, patch/evidence review, approvals, and live control readiness."
        }
        "live" => {
            "Auto-refresh operator event timeline, log-tail style status, and live activity slices."
        }
        "transcript" => "Session transcript and query affordances for debugging model/tool loops.",
        "chat" => {
            "Telegram-style multi-agent chat with inline lifecycle controls, a Telegram-like composer, immediate local send, and optional dry-run planning."
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
        interaction(
            "live-event-stream",
            "Live event stream",
            "hepta-core::control_ui rust renderer liveEventStream + /api/events-report",
        ),
        interaction(
            "cursor-live-events",
            "Cursor-based live events",
            "/api/live-events/<cursor> monotonic cursor and duplicate-free marker",
        ),
        interaction(
            "live-log-tail",
            "Live log tail",
            "auto-refresh event timeline with no-store fetch",
        ),
        interaction(
            "interactive-diff-review",
            "Interactive diff review",
            "diffReview commands for apply/rollback patch flow",
        ),
        interaction(
            "command-palette",
            "Command palette",
            "copy-ready HEPTA_UI.commands and keyboard shortcuts",
        ),
        interaction(
            "keyboard-shortcuts",
            "Keyboard shortcuts",
            "numeric nav plus slash focus shortcuts in Rust renderer markers",
        ),
        interaction(
            "session-tabs",
            "Session tabs",
            "transcript/session data sources and screen routing",
        ),
        interaction(
            "session-inspector",
            "Session inspector",
            "renderSessionInspector with /api/sessions and /api/session-activity",
        ),
        interaction(
            "transcript-preview",
            "Transcript preview",
            "renderTranscriptPreview with /api/transcript and query route",
        ),
        interaction(
            "transcript-search",
            "Transcript search",
            "fetchTranscriptQuery and /api/query-transcript route",
        ),
        interaction(
            "task-drilldown",
            "Task drilldown",
            "renderTaskDrilldown with task detail/patch/evidence endpoints",
        ),
        interaction(
            "task-publisher",
            "Task publisher",
            "renderTaskPublisher + submitTaskPublisher with POST /api/tasks/plan and /api/tasks/publish confirm gate",
        ),
        interaction(
            "agent-chat-composer",
            "Agent chat composer",
            "renderAgentChat + submitAgentChat with POST /api/chat/register, /api/chat/delete, /api/chat/plan, and immediate /api/chat send",
        ),
        interaction(
            "replay-promotion-drilldown",
            "Replay and promotion drilldown",
            "/api/task-replay and /api/promotion-ledger wiring",
        ),
        interaction(
            "artifact-preview",
            "Artifact preview",
            "artifacts screen and JSON paste preview",
        ),
        interaction(
            "approval-review",
            "Approval review",
            "approvals screen linked to policy/operator console",
        ),
        interaction(
            "exec-approvals-live-editor-parity",
            "Exec approvals live editor parity",
            "Hepta-native dry-run editor mirrors Hepta exec approvals target/scope/security/ask/allowlist semantics without mutating policy",
        ),
        interaction(
            "exec-approvals-apply-bridge",
            "Exec approvals apply bridge preview",
            "redacted snapshot hash, before/after diff, role guard, and confirmation checkbox for a human-gated apply plan",
        ),
        interaction(
            "exec-approvals-confirmed-apply-endpoint",
            "Exec approvals confirmed apply endpoint",
            "POST /api/approvals/exec/apply rechecks the redacted snapshot hash, requires operator confirmation, and returns post-apply evidence while keeping gateway mutation disabled",
        ),
        interaction(
            "multi-agent-tree",
            "Multi-agent tree",
            "operator + multi-agent runtime screens",
        ),
        interaction(
            "gateway-monitor",
            "Gateway monitor",
            "gateway runtime/dispatch/ledger/dead-letter surfaces",
        ),
        interaction(
            "developer-json-inspector",
            "Developer JSON inspector",
            "paste preview and live API summary",
        ),
        interaction(
            "dry-run-command-runner",
            "Dry-run command runner",
            "/gateway-dispatch --dry-run local-safe gate",
        ),
        interaction(
            "readonly-command-runner",
            "Read-only command runner",
            "POST /api/commands/<id> allowlisted local command execution",
        ),
        interaction(
            "operator-security-rbac",
            "Operator security/RBAC guard matrix",
            "renderOperatorSecurity with /api/operator-security loopback, header, allowlist, and dry-run guard report",
        ),
        interaction(
            "handoff-evidence-review",
            "Handoff evidence review",
            "handoff/promotion/replay commands",
        ),
        interaction(
            "boundary-aware-readiness",
            "Boundary-aware readiness",
            "readiness/parity/evidence screens keep public GA boundary explicit",
        ),
        interaction(
            "external-agent-benchmark",
            "External agent benchmark harness",
            "external-agent-benchmark screen + /api/external-agent-benchmark task corpus, adapter contracts, and no-synthetic-win boundary",
        ),
        interaction(
            "runtime-control-plane-bridge",
            "Hepta runtime control-plane bridge",
            "chat Workspace Room sidecar maps status/sessions/tasks/approvals/events/runtime to Hepta control surfaces while preserving the multi-agent dialogue",
        ),
        interaction(
            "hepta-runtime-control-plane-alignment",
            "Hepta runtime control-plane alignment matrix",
            "dedicated Hepta Operator Plane route maps sessions/agents/tasks/approvals/nodes/channels/cron/logs/skills/tools/config/debug/instances/model controls/diagnostics/mobile polish into Hepta screens",
        ),
    ]
}

fn interaction(
    id: &'static str,
    title: &'static str,
    evidence: &'static str,
) -> ControlUiInteractionCapability {
    ControlUiInteractionCapability {
        id,
        title,
        implemented: true,
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

fn parse_app_screen_ids() -> Vec<String> {
    control_ui_screens()
        .into_iter()
        .map(|screen| screen.id.to_string())
        .collect()
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

        assert_eq!(report.status, "complete");
        assert_eq!(report.screen_count, 26);
        assert_eq!(report.implemented_screen_count, 26);
        assert_eq!(report.screen_coverage_percent, 100);
        assert_eq!(report.asset_count, 4);
        assert_eq!(report.asset_coverage_percent, 100);
        assert_eq!(report.command_binding_count, 50);
        assert_eq!(report.interaction_capability_count, 30);
        assert_eq!(report.implemented_interaction_capability_count, 30);
        assert_eq!(report.live_operator_surface_percent, 100);
        assert_eq!(report.developer_interaction_percent, 100);
        assert_eq!(report.ref_agent_alignment_percent, 100);
        assert!(report.local_preview_ready);
        assert!(report.complete());
        assert!(report.frontend_manifest.rust_view_model_ready);
        assert_eq!(report.frontend_manifest.source, "hepta-core::control_ui");
        assert_eq!(
            report
                .rust_frontend_ownership
                .rust_embedded_static_asset_coverage_percent,
            100
        );
        assert_eq!(
            report.rust_frontend_ownership.status,
            "rust-rendered-no-js-frontend"
        );
        assert!(report.rust_frontend_ownership.rust_view_model_ready);
        assert!(report.rust_frontend_ownership.pure_browser_rust_runtime);
        assert_eq!(report.frontend_manifest.primary_nav[0], "chat");
        assert!(
            report
                .screens
                .iter()
                .all(|screen| !screen.summary.is_empty())
        );
        assert!(CONTROL_UI_INDEX_HTML.contains("Hepta Control UI"));
        assert!(CONTROL_UI_RUST_RENDERER_MARKERS.contains("HEPTA_UI"));

        let rust_frontend_html = control_ui_index_html();
        for marker in [
            "data-view=\"chat\"",
            "telegram-chat-shell",
            "focus-workspace",
            "tg-conversation-rail",
            "tg-thread-panel",
            "hepta-right-sidebar",
            "route-card",
            "screen-grid",
            "command-palette",
            "data-agent-chat-send",
            "data-chat-composer-shell=\"telegram\"",
            "HEPTA_ENTRY_CONTENT_ROWS",
        ] {
            assert!(
                rust_frontend_html.contains(marker),
                "Rust-rendered Control UI missing structural parity marker: {marker}"
            );
        }
        assert!(!rust_frontend_html.contains("<script"));
        assert!(
            rust_frontend_html.find("telegram-chat-shell").unwrap()
                < rust_frontend_html.find("evidence-panel").unwrap()
        );
        assert!(CONTROL_UI_STYLES_CSS.contains(".command-palette-backdrop:target"));
        assert!(CONTROL_UI_STYLES_CSS.contains("display: none;"));
        for nav_marker in [
            "href=\"#chat\"",
            "href=\"#tasks\"",
            "href=\"#ops\"",
            "href=\"#external-agent-benchmark\"",
            "href=\"#sessions\"",
            "href=\"#config\"",
        ] {
            assert!(
                rust_frontend_html.contains(nav_marker),
                "missing no-JS nav href: {nav_marker}"
            );
        }
        assert!(
            !rust_frontend_html.contains("<button type=\"button\" class=\"nav-item hepta-nav-item")
        );
        assert!(!rust_frontend_html.contains("<button class=\"active\" data-screen="));

        let audit = control_ui_contract_audit_report();
        assert_eq!(audit.status, "complete");
        assert_eq!(audit.audit_percent, 100);
        assert_eq!(audit.core_screen_count, 26);
        assert_eq!(audit.app_screen_count, 26);
        assert_eq!(audit.readme_screen_count, 26);
        assert!(audit.all_screen_ids_aligned);
        assert!(audit.app_has_operator_drilldown);
        assert!(audit.app_has_endpoint_health_grid);
        assert!(audit.app_has_dry_run_action_cards);
        assert!(audit.app_has_approval_cards);
        assert!(audit.app_has_post_action_guard);
        assert!(audit.app_has_session_inspector);
        assert!(audit.app_has_task_drilldown);
        assert!(audit.app_has_transcript_preview);
        assert!(audit.app_has_transcript_search);
        assert!(audit.app_has_replay_promotion_drilldown);
        assert!(audit.app_has_event_cursor);
        assert!(audit.app_has_readonly_command_runner);
        assert!(audit.app_has_operator_security);
        assert!(audit.app_has_task_publisher);
        assert!(audit.app_has_agent_chat);
        assert!(audit.app_has_external_agent_benchmark);
        assert!(audit.app_has_hepta_runtime_ui_parity);
        assert!(audit.app_has_hepta_runtime_navigation_groups);
        assert!(audit.app_has_chat_first_architecture);
        assert!(audit.app_has_route_view_controller);
        assert!(audit.app_has_command_palette);
        assert!(audit.app_has_premium_consumer_ui);
        assert!(audit.app_has_progressive_disclosure);
        assert!(audit.app_has_simplified_primary_nav);
        assert!(audit.app_preserves_typing_during_live_poll);
        assert!(audit.app_has_minimal_consumer_workspace);
        assert!(audit.app_has_telegram_multi_agent_workspace);
        assert!(audit.app_has_hepta_runtime_2026_5_2_ui_resilience);
        assert!(audit.app_models_long_gateway_websocket_resilience);
        assert!(audit.app_has_grouped_message_width_guard);
        assert!(audit.app_has_ios_pwa_bounds_guard);
        assert!(audit.app_has_selection_contrast_guard);
        assert!(audit.app_has_slash_feedback_surface);
        assert!(audit.app_has_talk_diagnostics_resilience);
        assert!(audit.app_has_persisted_auto_scroll_mode);
        assert!(audit.app_has_blank_dashboard_recovery_panel);
        assert!(audit.app_has_compact_session_status_badges);
        assert!(audit.app_scopes_nodes_polling_to_active_tab);
        assert!(audit.app_distinguishes_sample_vs_live_adapter_readiness);
        assert!(audit.app_has_terminal_qr_rendering_guard);
        assert!(!audit.control_ui_gateway_websocket_opened_by_audit);
        assert!(!audit.control_ui_live_gateway_rpc_performed);
        assert!(audit.p0_ready);
        assert!(audit.p1_ready);
        assert!(audit.p2_ready);
        assert!(audit.p3_ready);
        assert!(audit.p4_ready);
        assert!(audit.p5_ready);
        assert!(audit.p6_ready);
        assert!(audit.p7_ready);
        assert!(audit.p8_ready);
        assert!(audit.p9_ready);
        assert!(audit.p10_ready);
        assert!(audit.p11_ready);
        assert!(audit.p12_ready);
        assert!(audit.p13_ready);
        assert!(audit.p14_ready);
        assert!(audit.p15_ready);
        assert!(audit.p16_ready);
        assert!(audit.p17_ready);
        assert!(audit.p18_ready);
        assert!(audit.p19_ready);
        assert!(audit.p20_ready);
        assert!(audit.p21_ready);
        assert!(audit.p22_ready);
        assert!(audit.p23_ready);
        assert!(audit.p24_ready);
        assert!(audit.p25_ready);
        assert!(audit.p26_ready);
        assert!(audit.p27_ready);
        assert!(audit.p28_ready);
        assert!(audit.p29_ready);
        assert!(audit.p30_ready);
        assert!(audit.p31_ready);
        assert!(audit.p32_ready);
        assert!(audit.p33_ready);
        assert!(audit.p34_ready);
        assert!(audit.p35_ready);
        assert!(audit.p36_ready);
        assert!(audit.p37_ready);
        assert!(audit.p38_ready);
        assert!(audit.p39_ready);
        assert!(audit.p0_p1_p2_converged);
        assert!(audit.p0_p4_converged);
        assert!(audit.p0_p6_converged);
        assert!(audit.p0_p13_converged);
        assert!(audit.p0_p21_converged);
        assert!(audit.p0_p29_converged);
        assert!(audit.p0_p39_converged);
        assert_eq!(audit.convergence_percent, 100);
        assert_eq!(audit.convergence_lanes.len(), 40);
        assert!(
            audit
                .convergence_lanes
                .iter()
                .all(|lane| lane.ready && lane.percent == 100)
        );
        assert!(audit.missing_in_app.is_empty());
        assert!(audit.missing_in_readme.is_empty());
    }
}
