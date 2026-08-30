# Hepta Native downstream patch ledger v2

The immutable upstream baseline is Robrix commit
`a5a664da569c577ab1a3e5a33f45dcc9364954a0`, recorded in
`UPSTREAM_ROBRIX.lock.json` and `UPSTREAM_ROBRIX_FILES.tsv`.

This replacement ledger consolidates the accumulated downstream rows into
non-overlapping ownership boundaries. A boundary is intentionally scoped to one
file, one file family, or one product module. It does not grant release,
deployment, device-lab, signing, notarization, or publication authority.

The read-only sync checker compares the current worktree with the frozen Robrix
manifest. Every modified, removed, or downstream-only path must match exactly
one row below; every row must match at least one current drift path. The checker
remains fail-closed on undeclared paths, ambiguous matches, unused rows, parse
errors, unexpected import exclusions, or provenance mismatches.

## Import exclusions

| Upstream path | Reason |
| --- | --- |
| `.github/**` | The Hepta monorepo owns CI and repository automation. |
| `AGENTS.md` | The Hepta workspace owns agent instructions. |
| `packaging/upload-release-secrets.sh` | Secret upload and release mutation require separate authority. |

## Active downstream patches

The immutable raw snapshot is `7ac362f9690aa870591f4edcf533934af18921cb`.
The active lineage import is `a9cf73726c892cec5e3fae12792f8b98ba93ea58`.
Rows are grouped by non-overlapping product ownership boundaries so a path can
never be justified by two independent claims.

<!-- DOWNSTREAM_PATCHES_V1_BEGIN -->
| Local path or glob | Class | Purpose | Required verification |
| --- | --- | --- | --- |
| `.cargo/config.toml` | product identity | Bind the stable Hepta application identifier while preserving the Makepad platform model. | dependency and package metadata gates |
| `Cargo.toml` | dependency policy | Preserve the Robrix dependency graph while declaring bounded Hepta bridge, accessibility, packaging, and probe targets. | locked Cargo metadata, check, test, and deny gates |
| `Cargo.lock` | dependency policy | Lock the exact dependency graph used by both hosted and local qualification. | locked fetch, audit, deny, and reproducibility gates |
| `README.md` | product documentation | Document the Hepta-native product boundary and truthful qualification limits. | documentation and source-contract review |
| `build.rs` | build integration | Generate only bounded platform resources and preserve reproducible build behavior. | locked build and platform package gates |
| `canonical-assets-v1.tsv` | asset governance | Bind canonical source assets used by deterministic packaging copies. | canonical asset gate |
| `deny.toml` | dependency policy | Enforce the native dependency license, source, and duplicate policy. | cargo-deny gate |
| `dependency-source-policy-v1.json` | dependency policy | Record exact permitted Git and registry dependency sources. | dependency policy verifier |
| `hepta-live-bridge-backend-contract-v1.json` | backend boundary | Define the disabled-by-default native bridge contract without claiming a live backend. | bridge contract and source-only product-shell gates |
| `hepta-native-dependency-policy` | dependency policy | Verify native dependency source and vendored patch integrity. | dependency policy self-test and CI contract |
| `hepta-native-dependency-policy-self-test` | dependency policy | Reject malformed, broadened, or unpinned native dependency policy. | executable self-test |
| `licenses/ATTRIBUTIONS.md` | attribution | Add Hepta-owned and vendored component attribution without removing upstream notices. | attribution and license gates |
| `mobile-readiness-policy-v1.json` | mobile boundary | Keep emulator and real-device claims separate and fail closed. | mobile readiness gate |
| `promotion-trust-policy-v1.json` | authority boundary | Keep build evidence separate from promotion and release authority. | promotion trust gate |
| `rust-toolchain.toml` | toolchain policy | Pin the native Rust toolchain used by source and package qualification. | exact toolchain gate |
| `packaging/*.desktop` | package identity | Replace retired Robrix desktop metadata with Hepta-owned desktop identity. | Linux package metadata gate |
| `packaging/*.icns` | package identity | Carry the canonical Hepta macOS icon set. | icon and app-bundle fingerprint gates |
| `packaging/*.plist` | package identity | Bind Hepta bundle identity while preserving bounded entitlements. | plist and macOS package gates |
| `packaging/*.png` | package identity | Replace the retired Robrix DMG and store artwork with canonical Hepta assets. | canonical asset and package visual gates |
| `packaging/*.rb` | package integrity | Fingerprint bundles and validate release approval receipts without mutating authority. | Ruby self-tests and package integrity gates |
| `packaging/*.sh` | package integration | Build unsigned, signed, notarized, and TestFlight candidates under explicit authority boundaries. | shell syntax, package, signing, and release gates |
| `packaging/*.swift` | package integration | Resolve Finder bookmark metadata for deterministic macOS packaging. | macOS package gate |
| `packaging/*.xml` | package identity | Replace retired Robrix metainfo with Hepta application metadata. | metainfo validation |
| `packaging/android-emulator-login-template-v1/**` | emulator evidence | Maintain deterministic Android emulator login templates without claiming real-device evidence. | emulator template integrity gate |
| `packaging/debian-copyright` | attribution | Preserve upstream copyright while adding Hepta and vendored notices. | Debian package and attribution gates |
| `packaging/ios/**` | package identity | Carry canonical iOS icon and asset-catalog inputs. | iOS package asset gate |
| `packaging/native-fixture-contract-v1.json` | fixture boundary | Bind source-only native fixture generation without claiming a live session. | fixture contract gate |
| `packaging/release-execution-approval-trust-v1.json` | authority boundary | Bind release execution to independently verified approval evidence. | release approval trust gate |
| `packaging/release-secrets.env.example` | secret boundary | Document names only and keep secret values outside the repository. | secret scan and release policy review |
| `resources/android/AndroidManifest.xml` | Android integration | Bind the Hepta activity, theme, and platform bridge without broadening runtime permissions. | Android manifest contract gate |
| `resources/android/res/**` | Android integration | Carry Hepta Material 3 resources and canonical Android assets. | Android resource and package gates |
| `resources/icons/**` | canonical assets | Store Hepta-owned application icons under deterministic paths. | canonical asset gate |
| `resources/icon*.png` | canonical assets | Replace upstream root icon variants with Hepta-owned equivalents. | canonical asset and package gates |
| `resources/img/**` | asset retirement | Remove retired Robrix imagery and retain only canonical generated copies. | asset retirement and canonical asset gates |
| `resources/robrix_logo_alpha.png` | asset retirement | Remove the retired Robrix alpha logo. | asset retirement gate |
| `src/*.rs` | native shell integration | Adapt top-level Robrix application, accessibility, cache, verification, lifecycle, and startup modules to the Hepta product shell. | fmt, clippy, unit tests, source contracts, and product-shell gate |
| `src/bin/hepta-ui-v4-*.rs` | bounded probes | Provide source-bound filter and Windows material probes that cannot issue promotion authority. | probe self-tests and exact-source workflow gates |
| `src/hepta_bridge/**` | backend boundary | Implement authenticated, fail-closed native bridge contracts while keeping live activation disabled by default. | bridge unit tests and backend adapter contracts |
| `src/home/**` | product surface | Preserve the real Robrix room-list, timeline, composer, and navigation shell while applying Hepta v4 layout, theme, filtering, and state semantics. | native UI contract, widget tree, filter lifecycle, and visual source gates |
| `src/login/**` | authentication surface | Apply Hepta identity and semantic accessibility to the real Robrix login flow. | login tree and accessibility gates |
| `src/logout/**` | authentication surface | Keep logout confirmation, error, and state-machine behavior fail closed. | logout state-machine tests |
| `src/persistence/**` | persistence boundary | Bind Hepta session and TSP persistence to explicit storage contracts. | persistence and session-store tests |
| `src/profile/**` | profile surface | Apply Hepta profile identity while retaining Robrix profile behavior and cache boundaries. | profile and cache tests |
| `src/room/**` | composer surface | Preserve room input, reply, typing, filtering, and display semantics under Hepta styling. | room/composer unit and UI contract gates |
| `src/settings/**` | settings surface | Apply Hepta identity, preferences, diagnostics, and accessibility to settings. | settings and diagnostics source gates |
| `src/shared/**` | shared UI and platform material | Consolidate Hepta theme, accessible controls, platform material adapters, window acknowledgement, and shared Robrix widgets. | component matrix, platform adapter, z-index, and accessibility gates |
| `src/sliding_sync/**` | Matrix synchronization | Preserve Robrix synchronization semantics while isolating queue and worker responsibilities. | sliding-sync tests and source contracts |
| `src/tsp/**` | TSP surface | Preserve bounded wallet, DID, signing, and verification UI flows. | TSP unit and UI contract gates |
| `tests/android_manifest_contract.rs` | Android contract | Assert the exact Android manifest authority and component boundary. | native test suite |
| `third_party/robius-authentication/**` | vendored dependency | Vendor the exact reviewed authentication revision required by the native package. | vendored tree hash and dependency policy gates |
| `third_party/robius-directories/**` | vendored dependency | Vendor the exact reviewed directory revision required by the native package. | vendored tree hash and dependency policy gates |
| `third_party/robius-open/**` | vendored dependency | Vendor the exact reviewed open-handler revision required by the native package. | vendored tree hash and dependency policy gates |
<!-- DOWNSTREAM_PATCHES_V1_END -->

## Non-claims

This ledger proves only that current repository drift is explicitly classified.
It does not prove a live homeserver session, a live Hepta backend connection,
real-device behavior, signing, notarization, stapling, store upload, publication,
promotion, release readiness, or general availability.
