# Hepta Native downstream patch ledger

The immutable upstream baseline is Robrix commit
`a5a664da569c577ab1a3e5a33f45dcc9364954a0`, recorded in
`UPSTREAM_ROBRIX.lock.json` and `UPSTREAM_ROBRIX_FILES.tsv`.

This ledger is not permission to overwrite the upstream tree. Each downstream
change must stay narrow, preserve the real Robrix room-list/timeline/composer
product shell, and be independently reviewable. The read-only sync checker
reports current drift against the frozen baseline; it never fetches, copies,
resets, commits, pushes, or changes the worktree.

## Import exclusions

| Upstream path | Reason |
| --- | --- |
| `.github/**` | The Hepta monorepo owns CI and repository automation. |
| `AGENTS.md` | The Hepta workspace owns agent instructions. |
| `packaging/upload-release-secrets.sh` | Secret upload and release mutation require separate authority. |

## Active downstream patches

The immutable, byte-exact raw snapshot is
`7ac362f9690aa870591f4edcf533934af18921cb`. It is a provenance object, not an
active readiness implementation. The import in the current branch lineage is
`a9cf73726c892cec5e3fae12792f8b98ba93ea58`; every difference between the
locked manifest and the current tree is governed below, including files that
already differed at that lineage import. A row documents provenance but does
not itself prove correctness or readiness.

<!-- DOWNSTREAM_PATCHES_V1_BEGIN -->
| Local path or glob | Class | Purpose | Required verification |
| --- | --- | --- | --- |
| `.cargo/config.toml` | product identity | Set the stable cross-platform Hepta application identifier without changing Makepad's platform model. | `cargo check --locked`; package metadata audit |
| `Cargo.toml` | dependency and packaging policy | Keep the complete Robrix dependency graph, pin upstream Git revisions, add the narrow bridge/keyring dependencies and features, and declare Hepta package metadata. | `cargo check --locked`; dependency pin and package metadata audit |
| `Cargo.lock` | dependency policy | Freeze the exact dependency graph used by the downstream Native build. | `cargo check --locked`; lockfile source/revision audit |
| `rust-toolchain.toml` | reproducible toolchain | Pin the Native toolchain to Rust 1.95.0 instead of a moving `stable` channel. | feature-matrix gate; `rustc --version` receipt |
| `deny.toml` | dependency policy | Define the license, ban, and source checks used by the Native dependency audit. | dependency-policy self-test; `cargo deny` where installed |
| `dependency-source-policy-v1.json` | dependency provenance | Record exact lockfile, manifest, registry, Git, and vendored-source provenance for the resolved Native graph. | `hepta-native-dependency-policy verify` |
| `hepta-native-dependency-policy` | dependency provenance tooling | Verify pinned Git revisions, allowed registries, vendored provenance, licenses, and policy hashes without fetching or mutating dependencies. | `hepta-native-dependency-policy-self-test`; policy verify |
| `hepta-native-dependency-policy-self-test` | dependency provenance tooling | Exercise positive and deliberately negative dependency-policy cases. | execute self-test; shell/Ruby syntax check |
| `canonical-assets-v1.tsv` | asset deduplication policy | Declare the single canonical store-icon source and its generated packaging materialization after retiring the unused Control texture. | `scripts/hepta-native-canonical-assets verify` |
| `README.md` | product documentation and attribution | Document the upstream-first Hepta Native build, frozen Robrix baseline, downstream boundaries, and preserved upstream reference material. | copy review; provenance audit |
| `build.rs` | product identity and provenance | Embed Hepta desktop metadata while exposing Hepta and frozen Robrix revisions as separate build-time values. | `cargo check --locked`; revision and package metadata audit |
| `packaging/Info.plist` | packaging identity | Apply the Hepta macOS bundle name, identifier, executable, icon, deep link, and permission copy. | plist parse; unsigned local package inspection |
| `packaging/robrix.desktop` | packaging rename | Remove the upstream Linux desktop-entry filename as part of the explicit Hepta rename. | strict upstream sync check; package file inventory |
| `packaging/hepta-native.desktop` | packaging identity | Add the renamed Hepta Linux desktop entry. | desktop-entry validation; package file inventory |
| `packaging/rs.robius.robrix.metainfo.xml` | packaging rename | Remove the upstream AppStream filename as part of the explicit Hepta rename. | strict upstream sync check; package file inventory |
| `packaging/ai.hepta.nativeapp.metainfo.xml` | packaging identity and attribution | Add Hepta AppStream metadata while retaining Robrix/Matrix provenance and license attribution. | XML parse; attribution audit |
| `packaging/build-ios-testflight.sh` | packaging identity | Point the existing iOS packaging workflow at the Hepta identifiers and artifact names; it does not authorize upload. | `bash -n`; dry-run argument audit |
| `packaging/build-macos-dmg.sh` | packaging identity | Point the existing macOS packaging workflow at the Hepta app, icon, background, and artifact names; it does not authorize signing or publication. | `bash -n`; unsigned local packaging check |
| `packaging/fix-dmg-applications-icon.sh` | packaging identity | Update the DMG helper for the renamed Hepta image. | `bash -n`; local DMG layout check |
| `packaging/debian-copyright` | packaging identity and attribution | Update installed Hepta paths while preserving upstream and third-party notices. | package inventory; attribution audit |
| `packaging/icon_google_play_512.png` | brand asset | Replace the store artwork with the Hepta icon. | image decode and dimension check |
| `packaging/ios/icons/Assets.xcassets/AppIcon.appiconset/*.png` | brand assets | Replace the iOS application icons with the Hepta icon set. | asset-catalog and image dimension checks |
| `packaging/HeptaNative.icns` | brand asset | Add the Hepta macOS application icon. | icon decode; unsigned local package inspection |
| `packaging/Hepta Native macOS dmg background.png` | brand asset | Add the Hepta DMG background used by the local packaging recipe. | image decode and DMG layout check |
| `resources/icon*` | brand assets | Replace upstream product icons with the Hepta icon family while leaving Robrix attribution resources intact. | image decode and dimension checks |
| `resources/android/AndroidManifest.xml.template` | packaging identity | Document and retain the Hepta Android manifest template used by the Makepad packaging flow. | manifest parse; Android package audit |
| `src/app.rs` | product identity, theme, and diagnostic capture | Apply Hepta title/chrome to the real Robrix application root without replacing its desktop or mobile home shell; expose the delayed Makepad GPU-frame evidence hook only behind `developer-diagnostics`. | `cargo check --locked`; product-shell v2 gate; default/all-feature checks; desktop/mobile GPU screenshots |
| `src/main.rs` | product identity | Launch the renamed `hepta_native` library entry point. | `cargo check --locked`; launch smoke |
| `src/lib.rs` | product integration | Register the side-effect-free bridge contract and Hepta application identity/data namespace. | `cargo check --locked`; product-shell v2 gate |
| `src/sliding_sync.rs` | Matrix compatibility and identity | Preserve the upstream Sliding Sync path while changing only the product-owned SSO callback scheme. | Native focused tests; live Matrix remains a separate gate |
| `src/hepta_bridge/**` | typed bridge | Add a narrow, fail-closed, session-bound runtime/task/tool/approval presentation boundary; the product facade stays disabled until an authoritative adapter exists. | bridge unit tests; origin/session/correlation checks; no-live-receipt boundary check |
| `src/persistence/app_state.rs` | product identity | Restore persisted window state under the Hepta product title without changing upstream window-state behavior. | `cargo check --locked`; launch and restore smoke |
| `src/persistence/matrix_session_store/**` | security | Persist Matrix session material through authenticated metadata and platform credential/private-file abstractions; fail closed where a secure backend is unavailable and never fall back to plaintext. | secure-session unit tests; crash-consistency tests; platform credential review |
| `src/persistence/matrix_state.rs` | security | Replace upstream plaintext session persistence with the authenticated secure session store, bounded migration, authoritative same-user logout cleanup, and HTTPS production boundary. | secure-session unit tests; relogin/migration/logout/fail-closed checks |
| `src/persistence/mod.rs` | security | Register the secure Matrix session persistence module. | `cargo check --locked`; secure-session unit tests |
| `src/home/light_themed_dock.rs` | Hepta visual system | Apply shared light-glass chrome to the upstream desktop dock. | desktop screenshot gate; contrast review |
| `src/home/edited_indicator.rs` | upstream-first behavior restoration | Remove the former local cockpit-era edit-history contract overlay and restore the upstream edit-indicator behavior compatible with the current timeline API. | Native feature matrix; focused timeline tests |
| `src/home/loading_pane.rs` | Matrix SDK compatibility | Adapt backwards-pagination cancellation to the current typed timeline-request queue while removing obsolete cockpit boundary copy. | Native feature matrix; pagination focused tests |
| `src/home/search_messages.rs` | upstream-first behavior restoration | Restore the upstream local search surface after removing the former cockpit-era evidence and popup overlay. | Native feature matrix; search-surface smoke |
| `src/home/add_room.rs` | product identity and accessibility | Apply Hepta copy/surface styling and 44pt controls to the upstream room join/search surface. | copy review; touch-target review; focused tests |
| `src/home/location_preview.rs` | product identity | Replace residual upstream product-facing copy while preserving the Robrix location-preview behavior. | copy review; `cargo check --locked` |
| `src/home/navigation_tab_bar.rs` | Hepta visual system | Apply shared light-glass chrome to upstream mobile navigation. | mobile screenshot gate; touch-target review |
| `src/home/room_screen.rs` | Hepta visual system | Restyle the real upstream timeline surface; no live Hepta bridge hook is added here. | room/timeline focused tests; desktop/mobile screenshots |
| `src/home/rooms_list_entry.rs` | Hepta visual system | Restyle upstream room rows without changing room-list behavior. | room-list screenshot and selection-state checks |
| `src/home/rooms_sidebar.rs` | Hepta visual system | Restyle the real upstream sidebar and its navigation chrome. | desktop screenshot and resize checks |
| `src/home/welcome_screen.rs` | product identity and theme | Restyle the upstream empty state and explain the Matrix/Hepta boundary without presenting diagnostics as the product home. | empty-state screenshot and copy review |
| `src/login/login_screen.rs` | Hepta visual system | Apply Hepta identity and light-glass styling to the upstream login flow. | login screenshot and keyboard-flow checks |
| `src/join_leave_room_modal.rs` | product identity | Replace residual upstream product-facing copy while preserving the Matrix join/leave flow. | copy review; focused tests |
| `src/logout/logout_confirm_modal.rs` | product identity | Apply Hepta product copy to the existing logout confirmation flow. | copy review; logout focused tests |
| `src/logout/logout_state_machine.rs` | session security | Bind post-server logout cleanup to the authoritative current Matrix user and fail closed if local secure material cannot be removed. | logout state-machine tests; same-user cleanup audit |
| `src/room/room_input_bar.rs` | Hepta visual system | Restyle the real upstream Matrix composer; it remains a Matrix message composer and is not a live Hepta execution hook. | composer unit tests; message-send regression check; screenshots |
| `src/settings/about_settings.rs` | product identity | Replace Robrix-facing about copy with Hepta/upstream attribution. | attribution/license check; settings screenshot |
| `src/settings/account_settings.rs` | upstream-first account surface | Restore the real Robrix/Matrix account and device surface after removing the former marker-heavy cockpit contract overlay. | Native feature matrix; account/device focused tests; no live mutation claim |
| `src/settings/developer_diagnostics.rs` | developer-only diagnostics | Expose read-only provenance and disabled-bridge diagnostics, hidden unless the non-default developer feature is enabled. | default-feature hidden check; developer-feature test |
| `src/settings/mod.rs` | developer-only diagnostics | Register the diagnostics widget while preserving its runtime feature visibility boundary. | feature matrix check; product-shell v2 gate |
| `src/settings/settings_screen.rs` | developer-only diagnostics | Place the hidden-by-default diagnostics surface in settings rather than on the product home. | default-feature hidden check; settings route test |
| `src/shared/hepta_theme.rs` | Hepta visual system | Define shared downstream Native semantic theme tokens. | theme/token unit tests; screenshot review |
| `src/shared/icon_button.rs` | Hepta visual system | Apply shared control chrome to upstream icon buttons. | focus/touch/contrast screenshot checks |
| `src/shared/mention_popup.rs` | Hepta visual system | Apply restrained glass, readable secondary text, and shared radii to the upstream mention chooser. | popover screenshot; contrast review |
| `src/shared/mod.rs` | Hepta visual system | Register shared downstream theme primitives. | `cargo check --locked`; theme unit tests |
| `src/shared/navigation_bar_button.rs` | Hepta visual system | Apply shared navigation-button states to the upstream control. | selected/focus/touch screenshot checks |
| `src/shared/room_input_popup_menu.rs` | Hepta visual system and accessibility | Apply restrained glass and 44pt focusable menu rows to the upstream composer action menu. | popover screenshot; keyboard and touch-target review |
| `src/shared/styles.rs` | Hepta visual system | Map the upstream semantic style layer to the Hepta light-glass palette. | contrast tests; desktop/mobile screenshots |
| `src/tsp/mod.rs` | product identity | Apply the Hepta product user agent and user-facing TSP copy while retaining the upstream experimental TSP implementation. | `cargo check --locked`; TSP feature check |
| `third_party/aquamarine-0.6.0/**` | vendored compatibility dependency | Retain the minimal proc-macro compatibility crate required by the updated TSP/Matrix lock without network resolution. | `HEPTA_PROVENANCE.json`; license presence; dependency-policy verify/self-test |
| `third_party/sqlx-sqlite-only-0.8.6/**` | vendored compatibility dependency | Retain the SQLite-only SQLx compatibility subset required by the updated Matrix persistence graph. | `HEPTA_PROVENANCE.json`; MIT/Apache licenses; dependency-policy verify/self-test |
| `third_party/wasm_evt_listener-0.1.0/**` | vendored compatibility dependency | Retain the event-listener compatibility crate required by the updated cross-platform dependency graph. | `HEPTA_PROVENANCE.json`; license presence; dependency-policy verify/self-test |
<!-- DOWNSTREAM_PATCHES_V1_END -->

## Read-only verification tooling

The following repository-owned scripts are outside the imported application
prefix, so they are not upstream drift and do not appear in the table above:

- `scripts/hepta-native-robrix-upstream-sync-check-v2.sh` verifies the frozen
  commit/tree/manifest and requires every current application drift path to be
  declared here. It never fetches or modifies files.
- `scripts/hepta-native-product-shell-gate-v2.sh` verifies source structure
  only. It cannot promote live Matrix, live Hepta bridge, device-lab, signing,
  notarization, stapling, distribution, or full-product readiness.
- `scripts/hepta-native-product-shell-gate-v2-self-test.sh` exercises positive
  and deliberately negative source-gate cases.

## Non-negotiable boundaries

- The default product path remains the real Robrix desktop/mobile room shell.
- Developer diagnostics are feature-gated and cannot replace the default home.
- Matrix chat traffic is not an execution receipt and must not authorize Hepta actions.
- Runtime, task, tool, and approval integration stays behind the typed `hepta_bridge` boundary.
- Signing, notarization, stapling, store upload, and public release are not implied by this ledger.
