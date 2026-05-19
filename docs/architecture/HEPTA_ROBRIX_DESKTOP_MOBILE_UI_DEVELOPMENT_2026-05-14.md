# Hepta Robrix/Matrix-heart Desktop-Mobile UI Development Plan

Date: 2026-05-14
Status: Draft v2 / Matrix-heart fast path
Owner intent: build Hepta desktop + mobile UI by directly transplanting Robrix's Matrix chat heart, then aggressively modifying it into a Hepta-native multi-agent collaboration cockpit.

## 1. Executive decision

Hepta should take the **fast path**:

> Directly carry Robrix's Matrix heart first, then replace semantics from the outside inward.

The previous conservative plan said: use Robrix mostly as architecture reference and avoid importing Matrix assumptions. That is safer, but slower. The revised decision is:

- Start from Robrix as a working Rust chat client.
- Keep the Matrix SDK / Ruma / matrix-sdk-ui / sliding sync / timeline / room list / composer machinery initially.
- Treat Matrix room/event/timeline semantics as the first working UI data substrate.
- Layer Hepta semantics on top through custom Matrix-style events and a Hepta bridge.
- Only de-Matrixify later if the dependency surface becomes a blocker.

This is the fastest route because Robrix has already solved the hard UI problems:

- desktop/mobile split
- native Rust UI shell
- conversation list
- message timeline
- input composer
- reactions/replies/edits
- avatars/media/profile caches
- pagination and timeline stability
- modal/popup/action loop
- mobile safe-area and keyboard lessons

The development goal is not “a Matrix client with Hepta branding.” The goal is:

> use Matrix as the initial event/timeline engine for Hepta's agent runtime UI.

## 2. Current inputs

### 2.1 Hepta baseline

Verified before this document revision:

- Repository: `/Users/qianqi/.openclaw/workspace/Hepta`
- Branch: `main`
- Head at inspection: `b6d7543 fix: expand rust control ui route parity`
- Current UI path: Rust-rendered Control UI
- Key files:
  - `apps/hepta-control-ui/index.html`
  - `apps/hepta-control-ui/styles.css`
  - `crates/hepta-core/src/control_ui.rs`
  - `apps/hepta/src/main.rs`
  - `scripts/hepta-control-ui-smoke.sh`

Current Hepta UI constraints:

1. Do not silently revert to the old JS bundle path.
2. Preserve the existing Rust/no-JS Control UI as a compatibility surface while native UI work proceeds.
3. Keep existing control UI gates green during the Robrix/Matrix-heart spike.
4. Do not add unsafe mutations merely because the UI can render them.

### 2.2 Robrix reference

- Upstream: `https://github.com/project-robius/robrix`
- Local clone: `/Users/qianqi/.openclaw/workspace/research/rust-chat-ui/robrix`
- Checked commit: `b2bb6cf`
- License file: `LICENSE-MIT`
- Stack: Rust + Makepad UI + Robius + Matrix SDK
- Supported targets per README:
  - macOS
  - Linux
  - Windows
  - Android
  - iOS/iPadOS
  - OpenHarmony build path, runtime WIP

Important source files:

```text
src/app.rs
src/sliding_sync.rs
src/home/main_desktop_ui.rs
src/home/main_mobile_ui.rs
src/home/room_screen.rs
src/home/rooms_list.rs
src/home/rooms_sidebar.rs
src/room/room_input_bar.rs
src/shared/mentionable_text_input.rs
src/shared/command_text_input.rs
src/shared/popup_list.rs
src/shared/styles.rs
src/media_cache.rs
src/avatar_cache.rs
src/profile/*
```

## 3. Revised strategy: carry the Matrix heart

### 3.1 What “Matrix heart” means

For this plan, “Matrix heart” means these Robrix/Matrix layers should be preserved initially:

- `matrix-sdk`
- `matrix-sdk-ui`
- `matrix-sdk-base`
- `ruma`
- Robrix `sliding_sync.rs`
- Robrix timeline handling in `room_screen.rs`
- Robrix room list and room selection model
- Robrix room input/composer model
- Matrix IDs and event IDs as internal identifiers
- Matrix media/avatar/profile cache paths
- Matrix login/session persistence at first, if needed for bootstrapping

This is intentionally pragmatic. The first milestone should be a working Hepta-flavored Robrix fork, not a clean-room architecture diagram.

### 3.2 What changes immediately

These are modified early:

- Branding: Robrix → Hepta
- App identity and package names
- visual theme and assets
- top-level navigation labels
- room terminology exposed to user:
  - rooms → workspaces/conversations
  - users → humans/agents/providers
  - events → messages/runtime events
- custom event rendering for Hepta objects
- local safety boundaries around mutation controls

### 3.3 What changes later

These stay Matrix-shaped until they block us:

- room IDs
- event IDs
- timeline store
- sync model
- media cache
- profile cache
- login/session persistence
- sliding sync-like update flow

Later, if necessary, they can be replaced by a Hepta-native event store. But that should not block the first working desktop/mobile native UI.

## 4. Target architecture

### 4.1 Fast-path architecture

```text
┌────────────────────────────────────────────────────────────┐
│            Hepta Desktop / Mobile Native Apps              │
│  Robrix-derived Makepad UI shells                           │
│  - desktop: dock/multi-pane                                 │
│  - mobile: stack/single-room focus                          │
└───────────────────────────┬────────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────────┐
│                  Hepta Robrix UI Layer                      │
│  Room list / timeline / composer / modals / popups          │
│  mostly carried from Robrix, branded and extended           │
└───────────────────────────┬────────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────────┐
│                  Matrix Heart Layer                         │
│  matrix-sdk / matrix-sdk-ui / ruma / sliding sync           │
│  used as initial event, room, sync, and timeline engine      │
└───────────────────────────┬────────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────────┐
│                 Hepta Matrix Bridge                         │
│  Maps Hepta sessions/tasks/tools/runtime events into         │
│  Matrix-shaped rooms, state events, and timeline items       │
└───────────────────────────┬────────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────────┐
│               Existing Hepta Runtime                        │
│  hepta-core / hepta-runtime / hepta-cli / hepta-gateway      │
│  hepta-memory / hepta-intelligence / hepta-plugins           │
└────────────────────────────────────────────────────────────┘
```

### 4.2 Workspace shape

Recommended implementation layout:

```text
crates/
  hepta-matrix-heart/
    # copied/adapted Robrix Matrix and sync state
    src/lib.rs
    src/sliding_sync.rs
    src/event_mapping.rs
    src/hepta_event.rs
    src/session_store.rs

  hepta-robrix-ui/
    # Makepad UI fork from Robrix widgets
    src/lib.rs
    src/app.rs
    src/desktop.rs
    src/mobile.rs
    src/widgets/
      conversation_list.rs
      timeline.rs
      message_bubble.rs
      composer.rs
      inspector.rs
      task_card.rs
      tool_call_card.rs
      approval_card.rs
      runtime_event_card.rs
      popup_list.rs
      styles.rs

  hepta-matrix-bridge/
    # maps Hepta runtime snapshots/actions to Matrix-shaped events
    src/lib.rs
    src/runtime_adapter.rs
    src/session_adapter.rs
    src/task_adapter.rs
    src/tool_adapter.rs
    src/memory_adapter.rs

apps/
  hepta-native/
    src/main.rs
```

This can be simplified during the first spike. The important boundary is:

- UI fork may remain Robrix-shaped.
- Matrix heart may remain Matrix-shaped.
- Hepta bridge owns semantic conversion.

## 5. Why this is faster

The clean `hepta-ui-kernel` route would require us to rebuild several hard things:

- timeline item model
- virtualized scrolling
- message rendering variants
- composer behavior
- mobile keyboard/safe area behavior
- reply/edit/action affordances
- event update stability
- cache integration
- desktop/mobile shell behavior

Robrix already has most of this working. Carrying Matrix heart lets us focus on:

1. Booting the native app.
2. Rebranding and route shaping.
3. Mapping Hepta objects into events.
4. Rendering Hepta-specific event cards.
5. Enabling safe interactions.

The risk is coupling. The payoff is speed.

## 6. Hepta over Matrix: semantic mapping

### 6.1 Core mapping

| Hepta concept | Matrix-shaped representation | Notes |
| --- | --- | --- |
| Workspace | Matrix space or top-level room grouping | Use rooms first; spaces later if useful |
| Conversation/session | Matrix room | One Hepta session can map to one room |
| User message | `m.room.message` | Normal text/markdown body |
| Assistant reply | `m.room.message` with Hepta sender metadata | Sender may be agent identity |
| Agent | Matrix user/profile-like identity | Local virtual users are acceptable |
| Tool call | custom event `m.hepta.tool_call` | Also render fallback text body |
| Tool result | custom event `m.hepta.tool_result` | Include redacted evidence |
| Runtime event | custom event `m.hepta.runtime_event` | Started/completed/failed states |
| Task | custom state/timeline event `m.hepta.task` | Can be state event + timeline update |
| Approval request | custom event `m.hepta.approval_request` | Must require explicit confirmation |
| Memory citation | custom message part / relation | Render as citation card |
| Subagent run | custom event `m.hepta.agent_run` | Thread/group replies later |
| Channel/provider/node status | room state event or inspector data | Do not overfill timeline |

### 6.2 Custom event namespace

Use a Hepta event namespace from the beginning:

```text
m.hepta.runtime_event
m.hepta.tool_call
m.hepta.tool_result
m.hepta.approval_request
m.hepta.approval_result
m.hepta.task
m.hepta.agent_run
m.hepta.memory_citation
m.hepta.context_snapshot
m.hepta.policy_notice
m.hepta.channel_status
m.hepta.node_status
```

Every custom event should include:

```json
{
  "hepta_schema": "hepta.event.v1",
  "event_kind": "tool_call",
  "id": "...",
  "conversation_id": "...",
  "created_at_ms": 0,
  "status": "started|completed|failed|blocked",
  "redaction": {
    "secrets_redacted": true,
    "raw_secret_fields": []
  },
  "fallback_body": "Human-readable fallback text"
}
```

### 6.3 Local virtual identities

Hepta can model agents as local Matrix-style senders:

```text
@hepta:local
@agent-main:local
@agent-researcher:local
@tool-exec:local
@system-runtime:local
@user-local:local
```

These identities do not need to mean real external Matrix accounts in the first local mode. They are UI identities over the event substrate.

## 7. Desktop shell

Robrix source to carry:

- `src/home/main_desktop_ui.rs`
- `src/home/rooms_sidebar.rs`
- `src/home/rooms_list.rs`
- `src/home/room_screen.rs`
- dock/splitter layout pattern

Hepta desktop target:

```text
┌───────────────┬──────────────────────────────┬────────────────────┐
│ Workspaces    │ Conversation / Timeline      │ Inspector / Control │
│ Agents        │ Messages + runtime cards     │ Tasks               │
│ Tasks         │ Composer                     │ Approvals           │
│ Search        │                              │ Context / Evidence  │
└───────────────┴──────────────────────────────┴────────────────────┘
```

Desktop-specific modifications:

1. Keep Robrix dock/multi-pane behavior.
2. Rename Rooms sidebar to Workspaces/Conversations.
3. Add right inspector pane:
   - task details
   - runtime status
   - approval cards
   - context citations
   - evidence/source view
4. Add command palette entries:
   - status
   - tasks
   - sessions
   - approvals
   - tools
   - nodes/channels/providers
5. Keep existing Robrix room open/focus semantics initially.

## 8. Mobile shell

Robrix source to carry:

- `src/home/main_mobile_ui.rs`
- selected-room state model
- room screen reuse
- mobile safe-area handling
- keyboard shift/composer handling

Hepta mobile target:

```text
Stack 1: Workspace/conversation list
Stack 2: Active conversation timeline
Stack 3: Inspector/task/approval detail
Modal: command palette / composer picker / context picker
```

Mobile-specific modifications:

1. Keep single selected-conversation screen model.
2. Convert right inspector into drill-down screens.
3. Keep composer visible and keyboard-safe.
4. Put destructive actions behind large explicit confirmations.
5. Start mobile in read-only + draft mode until mutation policy is proven.

## 9. Component transplant map

| Robrix source | Carry directly? | Hepta modification |
| --- | --- | --- |
| `src/app.rs` | yes, fork | rename app, initialize Hepta bridge, preserve modal overlay root |
| `src/sliding_sync.rs` | yes, initially | wrap with HeptaMatrixBridge; later local sync source |
| `src/home/main_desktop_ui.rs` | yes | add inspector/control pane and Hepta labels |
| `src/home/main_mobile_ui.rs` | yes | map selected room to selected conversation/task stack |
| `src/home/rooms_list.rs` | yes | room list becomes conversation/workspace list |
| `src/home/rooms_sidebar.rs` | yes | workspace rail, agent/task filters |
| `src/home/room_screen.rs` | yes | add Hepta event renderers |
| `src/room/room_input_bar.rs` | yes | add slash commands, agent mentions, context chips |
| `src/shared/mentionable_text_input.rs` | yes | `@agent`, `#task`, `/command`, `memory:` pickers |
| `src/shared/command_text_input.rs` | yes | Hepta command palette and slash command bridge |
| `src/shared/popup_list.rs` | yes | runtime/task/approval toasts |
| `src/media_cache.rs` | yes | artifact/media cache adaptation |
| `src/avatar_cache.rs` | yes | agent/channel/provider avatars |
| `src/profile/*` | yes, partly | agent/human/provider profile cards |
| login/logout modules | yes for spike | local auth/runtime identity later |
| Matrix E2EE/device verification | defer | keep disabled/hidden unless needed |

## 10. Development phases

### Phase 0 — Working fork import

Goal: create a Hepta-controlled Robrix fork/spike that still builds as Robrix-style app.

Tasks:

1. Create a working copy separate from the reference clone:
   - recommended: `/Users/qianqi/.openclaw/workspace/Hepta/vendor/robrix-hepta-spike` for initial vendor audit, or
   - `apps/hepta-native` + copied Robrix source for direct workspace integration.
2. Preserve upstream commit metadata:
   - `project-robius/robrix @ b2bb6cf`
3. Add Robrix MIT attribution.
4. Build original Robrix on macOS if dependencies allow.
5. Record build blockers instead of rewriting around them prematurely.

Exit criteria:

- working fork exists
- license attribution exists
- original or minimally renamed app build status is known
- no existing Hepta Control UI regression

### Phase 1 — Hepta-branded Robrix boot

Goal: boot the Robrix-derived Matrix-heart app as a Hepta-branded native desktop app without changing its heart.

Tasks:

1. Rename app/package labels:
   - Robrix → Hepta Native or Hepta Chat
2. Replace icons/assets where easy.
3. Replace top-level labels:
   - Rooms → Conversations
   - Spaces → Workspaces
   - Home → Hepta
4. Keep Matrix login path if necessary for immediate boot.
5. Add a local/offline fixture mode if login blocks UI testing.

Exit criteria:

- Hepta-branded native desktop window opens
- desktop and mobile code paths still compile or have documented blockers
- no Hepta runtime integration yet required

### Phase 2 — Hepta custom event renderers

Goal: render Hepta runtime objects inside existing Matrix timeline.

Tasks:

1. Add event parser/renderers for:
   - `m.hepta.runtime_event`
   - `m.hepta.tool_call`
   - `m.hepta.tool_result`
   - `m.hepta.approval_request`
   - `m.hepta.task`
   - `m.hepta.agent_run`
   - `m.hepta.memory_citation`
2. Keep fallback rendering as text if custom renderer fails.
3. Add fixture events to test rendering without live Matrix server.
4. Map agent identities to avatar/profile visuals.

Exit criteria:

- timeline can show Hepta event cards
- composer still works for ordinary messages
- custom event parsing is covered by tests/fixtures

### Phase 3 — Hepta Matrix bridge

Goal: feed live or semi-live Hepta runtime data into Matrix-shaped rooms/events.

Two possible bridge paths:

#### Path A: Local Matrix homeserver bridge

Use a local Matrix homeserver or compatible service. Hepta writes real Matrix events into local rooms.

Pros:

- maximum compatibility with Matrix SDK
- Robrix sync/timeline works naturally
- easier to reuse E2EE/media/profile behavior later

Cons:

- more moving parts
- local server lifecycle/config/security required

#### Path B: In-process Matrix-shaped event source

Keep Matrix-like types but bypass external homeserver for fixture/local runtime mode.

Pros:

- fewer external services
- easier offline dev
- safer for first Hepta-only runtime UI

Cons:

- may require more surgery in Robrix `sliding_sync.rs`
- less faithful to Matrix SDK assumptions

Recommended fast path:

1. Start with Path B for fixture/local runtime development.
2. Keep Path A as fallback if Matrix SDK integration resists in-process feeding.

Exit criteria:

- Hepta runtime snapshot can appear as one or more conversations
- tool/task/runtime events render in timeline
- no mutating actions enabled yet

### Phase 4 — Composer and action bridge

Goal: make the composer send Hepta-intended actions through a safe planner.

Tasks:

1. Add slash command recognition:
   - `/task`
   - `/agent`
   - `/tool`
   - `/approve`
   - `/status`
2. Add mentions:
   - `@main`
   - `@subagent`
   - `@tool`
3. Add context chips:
   - session
   - task
   - memory citation
   - artifact
4. Add dry-run planner for actions.
5. Require confirmation for mutations.

Exit criteria:

- composer can create Hepta draft actions
- dry-run previews render as Matrix/Hepta events
- real mutation remains disabled until explicit class-by-class gates are added

### Phase 5 — Controlled mutations

Goal: enable safe mutations without violating Hepta/OpenClaw safety boundaries.

Enable in this order:

1. local UI-only state
2. read-only runtime command
3. draft task plan creation
4. send message to current Hepta session
5. spawn/steer/kill subagent
6. approve tool/exec call
7. update task registry
8. config/gateway mutation, last and most gated

Each mutation needs:

- exact payload preview
- target display
- confirmation
- result readback
- redacted evidence
- regression test

Current implementation status:

- `hepta_action_bridge` defines the side-effect-free class gate and Phase 5 ordering.
- Approval card buttons now reuse the shared Robrix `ConfirmationModal` path to show an exact local payload preview before any decision is acknowledged.
- The approval confirmation callback intentionally only emits a local warning popup; Hepta native execution adapters, Matrix send, live tool approval, and task registry mutation remain disabled until policy/readback gates are implemented.

### Phase 6 — Mobile packaging

Goal: keep mobile on the same Matrix heart and Hepta native OpenClaw-parity event/action substrate.

Tasks:

1. Use Robrix `cargo-makepad` mobile instructions.
2. Build Android target.
3. Build iOS simulator target.
4. Validate mobile stack routes.
5. Validate composer keyboard behavior.
6. Keep mutations read-only/draft until mobile confirmation UX is proven.

Exit criteria:

- Android build/run smoke or documented blocker
- iOS simulator build/run smoke or documented blocker
- desktop and mobile share same event substrate

### Phase 7 — Optional de-Matrixification

Goal: only if necessary, extract a Hepta-native event store after product behavior is proven.

Do this only if Matrix heart creates concrete blockers:

- build complexity too high
- local/offline mode too hard
- performance unacceptable
- mobile packaging unacceptable
- semantic mismatch blocks core Hepta UX

If this phase happens, Matrix heart becomes the reference implementation for the Hepta-native event store.

## 11. Safety boundaries

Directly carrying Matrix heart must not weaken safety.

Rules:

1. No automatic external Matrix sends unless explicitly enabled.
2. Local fixture/offline mode must exist before Hepta runtime actions are wired.
3. Any network homeserver mode must be visibly labeled.
4. Any bridge write must be categorized:
   - local fixture event
   - local Hepta runtime event
   - local Matrix homeserver event
   - external Matrix/network event
5. Approval events must never execute automatically.
6. Tool/exec payloads must show exact command/script before approval.
7. Secrets must be redacted in event cards and source views.
8. Mobile mutation must stay disabled until confirmation UX is strong.
9. Current mobile-safe approval path may show exact local preview confirmation, but accepting it must remain preview-only until live mutation classes are explicitly enabled.

## 12. Quality gates

### 12.1 Existing Hepta gates

Keep current Hepta UI green:

```sh
cargo fmt --all --check
git diff --check
cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture
./scripts/hepta-control-ui-smoke.sh
cargo check -q -p hepta
```

### 12.2 Robrix fork gates

For the fork/spike:

```sh
cargo fmt --all --check
cargo check -q
cargo build --release
```

If full build is blocked by native dependencies, record the blocker exactly and create smaller compile gates for edited crates/modules.

### 12.3 Event renderer gates

Add tests/fixtures for:

- normal text message
- agent reply
- tool call
- tool result
- approval request
- task update
- runtime event
- memory citation
- redacted source view

### 12.4 Mobile gates

Once packaging starts:

```sh
cargo makepad android run -p hepta-native --release
cargo makepad apple ios run-sim -p hepta-native --release
```

If names differ, update commands in the implementation README.

## 13. Licensing and attribution

Because this plan now intentionally copies substantial Robrix code, attribution is mandatory.

Required actions:

1. Preserve Robrix MIT license text.
2. Add a Hepta third-party notice for Robrix / Project Robius.
3. Record source commit: `project-robius/robrix @ b2bb6cf`.
4. Track copied files in a manifest.
5. Keep any modified copied files clearly attributable.

Suggested manifest path:

```text
docs/architecture/third_party/ROBRIX_COPY_MANIFEST_2026-05-14.md
```

Manifest fields:

```text
source_repo
source_commit
source_license
copied_file
hepta_destination
modification_summary
status: copied | renamed | heavily_modified | retired
```

## 14. Risk register

| Risk | Impact | Fast-path response |
| --- | --- | --- |
| Matrix assumptions leak into Hepta UX | confusing model | accept temporarily; hide with Hepta labels and event mapping |
| Build dependency surface grows | slow iteration | isolate fork first; avoid breaking main Hepta crates |
| Local/offline mode fights Matrix SDK | delay | use fixture mode first; consider local homeserver path |
| External network actions unsafe | privacy/safety risk | default to offline/local; label network mode explicitly |
| Mobile packaging complexity | schedule risk | inherit Robrix mobile setup; document blockers early |
| License hygiene missed | legal/maintenance risk | create copy manifest before large code edits |
| De-Matrixification later becomes expensive | technical debt | postpone until product shape proven |
| Hepta Control UI regresses | current product breakage | keep existing Rust/no-JS smoke gates separate |

## 15. Immediate implementation checklist

1. Create copy manifest for Robrix.
2. Create a Hepta Robrix working fork/spike.
3. Preserve upstream commit and MIT license.
4. Try original Robrix build on macOS.
5. If build succeeds, rename app to Hepta Native.
6. If build fails, record exact blocker and compile smaller edited surface.
7. Add Hepta branding and labels.
8. Add fixture/local mode if login/server blocks UI work.
9. Add first custom event renderer:
   - `m.hepta.runtime_event`
10. Add second renderer:
   - `m.hepta.tool_call`
11. Add approval card renderer:
   - `m.hepta.approval_request`
12. Add bridge stub that emits fixture Hepta events into a Matrix-shaped timeline.
13. Only then wire live read-only Hepta runtime snapshots.

## 16. First milestone

Milestone name:

> M1: Hepta-branded Robrix fork boots with Matrix heart intact

Definition of done:

- Hepta has a tracked Robrix copy/fork.
- Attribution and copy manifest exist.
- Native desktop window boots or exact build blocker is documented.
- App is visibly Hepta-branded.
- Matrix heart is still intact.
- At least one Hepta custom event fixture renders in the timeline.
- Existing Hepta Control UI gates remain green.

## 17. Second milestone

Milestone name:

> M2: Hepta runtime events flow through Matrix-shaped timeline

Definition of done:

- Read-only Hepta runtime snapshot maps to one conversation.
- Timeline renders:
  - user message
  - assistant/agent reply
  - tool call
  - tool result
  - approval request
  - task update
  - runtime event
- Desktop shell shows inspector pane.
- Mobile shell shows drill-down detail.
- No mutation is enabled without confirmation.

## 18. Final direction

The fastest path is to stop fighting Robrix's Matrix core and use it.

Hepta should become:

> a native Rust multi-agent collaboration cockpit that initially uses Matrix as its event/timeline substrate, then gradually replaces or hides Matrix semantics wherever Hepta needs stronger agent/runtime concepts.

This keeps velocity high. We get a working desktop/mobile chat substrate first, then bend it into Hepta.

## 19. Implementation status — 2026-05-14 M4/Mobile pass

This pass advanced the Matrix-heart fast path beyond the M2 definition of done while keeping all runtime mutations local-only:

- Desktop shell now includes a persistent right-side `Inspector / Control` pane for runtime status, task policy, approval policy, context chips, and mobile mutation policy.
- New read-only `hepta_runtime_status` model and shared `HeptaRuntimeStatusPane` expose fixture readiness, composer/action preview mode, approval preview mode, action outbox readiness, and explicit mobile packaging gate evidence without OpenClaw Gateway calls.
- Mobile welcome/cockpit surface now includes a `Mobile drill-down detail` pane so the single-column shell exposes the same event substrate and draft-first safety model.
- Mobile room view now includes a persistent `HeptaMobileSafetyBar` above the Matrix-heart room screen, making inspect-payload / confirm-preview / live-blocked status visible while reading actual timelines.
- Composer bridge now recognizes both direct Hepta slash commands and `/hepta ...` commands:
  - `/task`
  - `/agent`
  - `/tool`
  - `/approve`
  - `/reject`
  - `/status`
- Composer plans collect context chips:
  - `@agent`
  - `#task`
  - `session:*`
  - `memory:*`
  - `artifact:*`
- New read-only `hepta_context_snapshot` model and shared `HeptaContextSnapshotPane` expose those chip classes on desktop and mobile without reading memory files, session stores, task registries, or external services.
- Composer plans convert into Matrix-shaped `m.hepta.*` preview events through the local bridge seam, with `external_mutation_enabled=false`.
- New `hepta_command_templates` model and shared `HeptaCommandTemplatesPane` expose validated quick-command templates for status, task, agent, tool, and approval drafts; every template must parse through the same dry-run planner before it appears in UI.
- Composer now shows an inline dry-run preview pane while reserved Hepta commands are being typed, and suppresses Matrix typing notices for `/task`, `/agent`, `/tool`, `/approve`, `/reject`, `/status`, and `/hepta ...` command prefixes.
- Side-effect-free `hepta_action_bridge` policy now classifies local-only, read-only preview, draft preview, confirmation-blocked, and policy-blocked mutation classes in the Phase 5 enablement order.
- New side-effect-free `hepta_action_queue` model and shared `HeptaActionOutboxPane` expose staged preview, exact confirmation, policy-blocked, and evidence lanes on both desktop Inspector and mobile detail surfaces; the pane is populated from sample queue items generated by the same composer/action policy model rather than static copy only.
- Every Hepta card now exposes `Inspect payload`, reusing the existing event source modal for raw Matrix/Hepta JSON inspection, and renders an inline policy badge derived from `bridge_policy` payload metadata or safe event-kind defaults.
- Approval cards route approve/reject clicks into shared confirmation modals with exact JSON payload preview and local-only acknowledgement popups.
- Fixture cockpit includes a composer dry-run preview event so the timeline/card surface demonstrates draft actions before controlled mutations are enabled.
- Mutation boundary remains unchanged: no OpenClaw Gateway call, no Matrix send, no external approval/tool execution, and no task-registry mutation is wired in this pass.

Validation for this pass:

```sh
cargo check --manifest-path apps/hepta-native/Cargo.toml
cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture
cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture
git diff --check
```

Known formatting note: `cargo fmt --manifest-path apps/hepta-native/Cargo.toml --all --check` still traverses Robrix-derived modules that rustfmt cannot parse under this toolchain because of existing let-chain syntax. For now, targeted rustfmt is used on Hepta-owned edited modules, and the compile/test gates above are the authoritative native UI gates.

### Mobile packaging tooling status

Current packaging status on this host:

```sh
cargo makepad --help
cargo makepad android --abi=aarch64 --package-name=ai.hepta.nativeapp --app-label='Hepta Native' --sdk-path=/Users/qianqi/.openclaw/workspace/Hepta/android_33_sdk build -p hepta-native --release
cargo makepad apple ios --org=ai.hepta --app=hepta-native build -p hepta-native --release
```

Results:

- `cargo-makepad v1.0.0` is installed from Makepad `dev` branch.
- Android SDK/NDK/JDK were materialized under `/Users/qianqi/.openclaw/workspace/Hepta/android_33_sdk` after a Makepad stripped-NDK installer path issue.
- Android APK build smoke passed with Java-safe package `ai.hepta.nativeapp`; using `ai.hepta.native` generated Java package errors because `native` is reserved.
- APK artifacts were produced at `apps/hepta-native/target/android/makepad-android-apk/hepta_native/apk/heptanative.apk` and `heptanative.unaligned.apk`.
- iOS release build reached the native/iOS asset phase; current local blocker is simulator runtime mismatch: `No simulator runtime version from ["23C54"] available to use with iphonesimulator SDK version 23F73`.

The UI now exposes these Phase 6 facts through local-only packaging status panes. Those panes are reporting surfaces only: they do not shell out, install toolchains, run `adb`, run a simulator, sign packages, call the OpenClaw Gateway, send Matrix events, or mutate task/tool/approval state.
