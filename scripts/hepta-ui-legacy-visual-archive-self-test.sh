#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="scripts/archive/hepta-ui-top-design-v1/manifest.json"
ARCHIVE_GATE_DIR="scripts/archive/hepta-ui-top-design-v1/gates"
ARCHIVE_FIXTURE="scripts/archive/hepta-ui-top-design-v1/native-fixture/hepta-native-fixture-visual-smoke.sh"
COMPAT_FIXTURE="scripts/hepta-native-fixture-visual-smoke.sh"
CURRENT_FIXTURE_CONTRACT="apps/hepta-native/packaging/native-fixture-contract-v1.json"

for command_name in jq rg readlink sed sort diff mktemp find cmp seq wc; do
  if ! command -v "$command_name" >/dev/null 2>&1; then
    printf 'archive self-test requires %s\n' "$command_name" >&2
    exit 2
  fi
done

if [[ ! -s "$MANIFEST" ]]; then
  printf 'missing historical UI archive manifest: %s\n' "$MANIFEST" >&2
  exit 1
fi

jq -e '
  .schema_version == 1
  and .kind == "hepta-ui-historical-top-design-archive"
  and .status == "archived_historical_compatibility"
  and .canonical_current_readiness.invokes_historical_gates == false
  and .canonical_current_readiness.invokes_html_native_fixture == false
  and .historical_gates.count == 40
  and .historical_gates.first_stage == 2
  and .historical_gates.last_stage == 41
  and .historical_gates.standalone_roots == [2, 3]
  and .historical_gates.fixture_execution_stages == [12]
  and .historical_gates.compatibility_strategy == "relative_symlink"
  and .native_fixture.compatibility_strategy == "relative_symlink"
  and .native_fixture.current_contract_path == "apps/hepta-native/packaging/native-fixture-contract-v1.json"
  and .native_fixture.active_static_source_consumers == []
  and .native_fixture.active_metadata_only_consumers == []
  and (.native_fixture.migrated_static_contract_consumers | length) == 2
  and (.native_fixture.migrated_metadata_contract_consumers | length) == 2
  and .retired_root_report_generators.status == "retired_no_runtime_consumers"
  and .retired_root_report_generators.paths == [
    "scripts/hepta-ui-demo-evidence-gate.sh",
    "scripts/hepta-ui-top-design-referee-refresh-gate.sh"
  ]
  and .retired_root_report_generators.replacement == "scripts/hepta-ui-current-readiness.sh"
  and .retired_root_report_generators.historical_catalog_identifiers_preserved == true
  and .retired_root_report_generators.current_readiness_or_ci_consumers_removed == 0
  and (.retired_root_report_generators.preserved_receipt_consumers | length) == 8
  and .receipt_compatibility.implementation_storage_paths_changed == true
  and .receipt_compatibility.compatibility_entrypoint_paths_changed == false
  and .receipt_compatibility.receipt_paths_changed == false
  and .receipt_compatibility.schemas_changed == false
  and .receipt_compatibility.execution_semantics_changed == false
  and .receipt_compatibility.current_readiness_authority_changed == false
' "$MANIFEST" >/dev/null

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-legacy-archive-self-test.XXXXXX")"
stage_actual="$tmp_dir/stages.actual"
stage_expected="$tmp_dir/stages.expected"
gate_refs_actual="$tmp_dir/gate-refs.actual"
gate_refs_expected="$tmp_dir/gate-refs.expected"
fixture_refs_actual="$tmp_dir/fixture-refs.actual"
fixture_refs_expected="$tmp_dir/fixture-refs.expected"
receipt_refs_actual="$tmp_dir/receipt-refs.actual"
receipt_refs_expected="$tmp_dir/receipt-refs.expected"
retired_receipt_consumers_actual="$tmp_dir/retired-receipt-consumers.actual"
retired_receipt_consumers_expected="$tmp_dir/retired-receipt-consumers.expected"
cleanup() {
  rm -f \
    "$stage_actual" \
    "$stage_expected" \
    "$gate_refs_actual" \
    "$gate_refs_expected" \
    "$fixture_refs_actual" \
    "$fixture_refs_expected" \
    "$receipt_refs_actual" \
    "$receipt_refs_expected" \
    "$retired_receipt_consumers_actual" \
    "$retired_receipt_consumers_expected"
  rmdir "$tmp_dir" 2>/dev/null || true
}
trap cleanup EXIT

: >"$stage_actual"
gate_count=0
for implementation in "$ARCHIVE_GATE_DIR"/hepta-ui-harsh-top-design-referee-v*-gate.sh; do
  if [[ ! -f "$implementation" || -L "$implementation" ]]; then
    printf 'archived gate is not a regular implementation: %s\n' "$implementation" >&2
    exit 1
  fi

  gate_name="$(basename "$implementation")"
  stage="$(printf '%s\n' "$gate_name" | sed -E 's/^hepta-ui-harsh-top-design-referee-v([0-9]+).*/\1/')"
  compat_path="scripts/$gate_name"
  expected_target="archive/hepta-ui-top-design-v1/gates/$gate_name"

  if [[ ! "$stage" =~ ^[0-9]+$ ]]; then
    printf 'cannot parse historical gate stage: %s\n' "$gate_name" >&2
    exit 1
  fi
  if [[ ! -L "$compat_path" ]]; then
    printf 'historical compatibility entrypoint is not a symlink: %s\n' "$compat_path" >&2
    exit 1
  fi
  if [[ "$(readlink "$compat_path")" != "$expected_target" ]]; then
    printf 'historical compatibility target mismatch: %s -> %s\n' "$compat_path" "$(readlink "$compat_path")" >&2
    exit 1
  fi
  if ! cmp -s "$compat_path" "$implementation"; then
    printf 'historical compatibility content mismatch: %s\n' "$compat_path" >&2
    exit 1
  fi

  bash -n "$implementation"
  bash -n "$compat_path"
  printf '%s\n' "$stage" >>"$stage_actual"
  gate_count=$((gate_count + 1))
done

while IFS= read -r retired_generator; do
  if [[ -e "$retired_generator" || -L "$retired_generator" ]]; then
    printf 'retired historical root report generator reappeared: %s\n' "$retired_generator" >&2
    exit 1
  fi
done < <(jq -r '.retired_root_report_generators.paths[]' "$MANIFEST")

jq -e '
  [.entries[] | select(
    .gate_path == "scripts/hepta-ui-demo-evidence-gate.sh"
    or .gate_path == "scripts/hepta-ui-top-design-referee-refresh-gate.sh"
  )] | length == 2
' docs/architecture/HEPTA_SHELL_GATE_MIGRATION_INPUT_V1.json >/dev/null

rg -l 'ui-demo-evidence-gate\.json|ui-top-design-referee-refresh-gate\.json' \
  scripts --glob '*.sh' \
  | rg -v 'scripts/hepta-ui-legacy-visual-archive-self-test\.sh$' \
  | sort >"$retired_receipt_consumers_actual"
jq -r '.retired_root_report_generators.preserved_receipt_consumers[]' "$MANIFEST" \
  | sort >"$retired_receipt_consumers_expected"
if ! diff -u "$retired_receipt_consumers_expected" "$retired_receipt_consumers_actual"; then
  printf 'retired root report receipt-consumer inventory is stale\n' >&2
  exit 1
fi

seq 2 41 >"$stage_expected"
sort -n "$stage_actual" -o "$stage_actual"
if ! diff -u "$stage_expected" "$stage_actual"; then
  printf 'historical stage set is not exactly v2-v41\n' >&2
  exit 1
fi
if [[ "$gate_count" -ne 40 ]]; then
  printf 'historical implementation count mismatch: %s\n' "$gate_count" >&2
  exit 1
fi

for stage in $(seq 2 41); do
  implementation="$(find "$ARCHIVE_GATE_DIR" -maxdepth 1 -type f -name "hepta-ui-harsh-top-design-referee-v${stage}-*-gate.sh" -o -maxdepth 1 -type f -name "hepta-ui-harsh-top-design-referee-v${stage}-gate.sh")"
  implementation_count="$(printf '%s\n' "$implementation" | sed '/^$/d' | wc -l | tr -d ' ')"
  if [[ "$implementation_count" -ne 1 ]]; then
    printf 'expected exactly one implementation for v%s, found %s\n' "$stage" "$implementation_count" >&2
    exit 1
  fi

  script_refs="$(rg -o 'scripts/hepta-ui-harsh-top-design-referee-v[0-9][^"[:space:]]*\.sh' "$implementation" | sort -u || true)"
  if [[ "$stage" -le 3 ]]; then
    if [[ -n "$script_refs" ]]; then
      printf 'standalone historical root v%s unexpectedly invokes another stage: %s\n' "$stage" "$script_refs" >&2
      exit 1
    fi
  else
    predecessor=$((stage - 1))
    predecessor_count="$(printf '%s\n' "$script_refs" | sed '/^$/d' | rg -c "referee-v${predecessor}(-|[.]sh)" || true)"
    total_ref_count="$(printf '%s\n' "$script_refs" | sed '/^$/d' | wc -l | tr -d ' ')"
    if [[ "$predecessor_count" -ne 1 || "$total_ref_count" -ne 1 ]]; then
      printf 'historical dependency mismatch for v%s; expected only v%s, found: %s\n' "$stage" "$predecessor" "$script_refs" >&2
      exit 1
    fi
    while IFS= read -r script_ref; do
      [[ -z "$script_ref" ]] && continue
      if [[ ! -L "$script_ref" || ! -s "$script_ref" ]]; then
        printf 'historical dependency does not resolve through compatibility entrypoint: %s\n' "$script_ref" >&2
        exit 1
      fi
    done <<<"$script_refs"
  fi

  fixture_ref_count="$(rg -c 'scripts/hepta-native-fixture-visual-smoke\.sh' "$implementation" || true)"
  if [[ "$stage" -eq 12 ]]; then
    if [[ "$fixture_ref_count" -ne 1 ]]; then
      printf 'historical v12 fixture dependency is missing or duplicated\n' >&2
      exit 1
    fi
  elif [[ "$fixture_ref_count" -ne 0 ]]; then
    printf 'unexpected Native fixture execution dependency in v%s\n' "$stage" >&2
    exit 1
  fi
done

if [[ ! -f "$ARCHIVE_FIXTURE" || -L "$ARCHIVE_FIXTURE" ]]; then
  printf 'archived Native fixture is not a regular implementation: %s\n' "$ARCHIVE_FIXTURE" >&2
  exit 1
fi
if [[ ! -L "$COMPAT_FIXTURE" ]]; then
  printf 'Native fixture compatibility entrypoint is not a symlink: %s\n' "$COMPAT_FIXTURE" >&2
  exit 1
fi
if [[ ! -x "$COMPAT_FIXTURE" ]]; then
  printf 'Native fixture compatibility entrypoint is not executable: %s\n' "$COMPAT_FIXTURE" >&2
  exit 1
fi
if [[ "$(readlink "$COMPAT_FIXTURE")" != "archive/hepta-ui-top-design-v1/native-fixture/hepta-native-fixture-visual-smoke.sh" ]]; then
  printf 'Native fixture compatibility target mismatch: %s\n' "$(readlink "$COMPAT_FIXTURE")" >&2
  exit 1
fi
cmp -s "$COMPAT_FIXTURE" "$ARCHIVE_FIXTURE"
bash -n "$ARCHIVE_FIXTURE"
bash -n "$COMPAT_FIXTURE"

jq -e '
  .schema == "hepta_native_fixture_contract_v1"
  and .status == "ready"
  and .grants_product_claims == false
  and (.backend_alignment_markers | length) == 21
  and (.spaces_room_membership_edge_markers | length) == 6
' "$CURRENT_FIXTURE_CONTRACT" >/dev/null

for marker in \
  'native_telegram_static_fixture_smoke_only:true' \
  'data-native-telegram-rooms-list-load-more-pagination-packet="loaded-counts-cursor-result-slots-local"' \
  'data-native-telegram-space-lobby-reknock-cancel-prior-packet="tree-action-cancel-slot-local"'; do
  if ! grep -Fq "$marker" "$COMPAT_FIXTURE"; then
    printf 'Native fixture compatibility entrypoint hides required marker: %s\n' "$marker" >&2
    exit 1
  fi
done

for manifest_path in $(jq -r '
  .native_fixture.active_static_source_consumers[],
  .native_fixture.active_metadata_only_consumers[],
  .native_fixture.migrated_static_contract_consumers[],
  .native_fixture.migrated_metadata_contract_consumers[],
  .native_fixture.historical_report_consumers[],
  .external_dependency_inventory.rust_catalog_only_consumers[],
  .external_dependency_inventory.historical_catalogs[],
  .external_dependency_inventory.historical_evidence_assets[],
  .external_dependency_inventory.historical_docs[]
' "$MANIFEST"); do
  if [[ ! -e "$manifest_path" ]]; then
    printf 'archived dependency inventory path is stale: %s\n' "$manifest_path" >&2
    exit 1
  fi
done

while IFS= read -r candidate; do
  case "$candidate" in
    scripts/archive/hepta-ui-top-design-v1/gates/* | \
    scripts/archive/hepta-ui-top-design-v1/README.md | \
    scripts/archive/hepta-ui-top-design-v1/manifest.json | \
    scripts/hepta-ui-legacy-visual-archive-self-test.sh | \
    scripts/hepta-ui-harsh-top-design-referee-v*-gate.sh)
      ;;
    *)
      printf '%s\n' "$candidate"
      ;;
  esac
done < <(rg -l 'scripts/hepta-ui-harsh-top-design-referee-v[0-9][^"`[:space:]]*\.sh' scripts .github codex-rs apps docs | sort) >"$gate_refs_actual"
jq -r '.direct_reference_inventory.historical_gate_script_reference_files[]' "$MANIFEST" | sort >"$gate_refs_expected"
if ! diff -u "$gate_refs_expected" "$gate_refs_actual"; then
  printf 'historical gate direct-reference inventory is stale\n' >&2
  exit 1
fi

while IFS= read -r candidate; do
  case "$candidate" in
    scripts/archive/hepta-ui-top-design-v1/README.md | \
    scripts/archive/hepta-ui-top-design-v1/manifest.json | \
    scripts/hepta-ui-legacy-visual-archive-self-test.sh | \
    scripts/hepta-ui-legacy-visual-gates.json | \
    scripts/hepta-native-fixture-visual-smoke.sh)
      ;;
    *)
      printf '%s\n' "$candidate"
      ;;
  esac
done < <(rg -l 'scripts/hepta-native-fixture-visual-smoke\.sh' scripts .github codex-rs apps docs | sort) >"$fixture_refs_actual"
jq -r '.direct_reference_inventory.native_fixture_script_reference_files[]' "$MANIFEST" | sort >"$fixture_refs_expected"
if ! diff -u "$fixture_refs_expected" "$fixture_refs_actual"; then
  printf 'Native fixture direct-reference inventory is stale\n' >&2
  exit 1
fi

while IFS= read -r candidate; do
  case "$candidate" in
    scripts/archive/hepta-ui-top-design-v1/gates/* | \
    scripts/archive/hepta-ui-top-design-v1/native-fixture/* | \
    scripts/archive/hepta-ui-top-design-v1/README.md | \
    scripts/archive/hepta-ui-top-design-v1/manifest.json | \
    scripts/hepta-ui-legacy-visual-archive-self-test.sh | \
    scripts/hepta-ui-harsh-top-design-referee-v*-gate.sh | \
    scripts/hepta-native-fixture-visual-smoke.sh)
      ;;
    *)
      printf '%s\n' "$candidate"
      ;;
  esac
done < <(rg -l 'ui-harsh-top-design-referee-v[0-9][^"`[:space:]]*gate\.json|native-fixture-visual-smoke\.json|native-fixture/hepta-native-fixture\.html' scripts .github codex-rs apps docs | sort) >"$receipt_refs_actual"
jq -r '.direct_reference_inventory.historical_receipt_reference_files[]' "$MANIFEST" | sort >"$receipt_refs_expected"
if ! diff -u "$receipt_refs_expected" "$receipt_refs_actual"; then
  printf 'historical receipt direct-reference inventory is stale\n' >&2
  exit 1
fi

if rg -n 'scripts/hepta-ui-harsh-top-design-referee-v[0-9].*\.sh|scripts/hepta-native-fixture-visual-smoke\.sh' \
  scripts/hepta-ui-current-readiness.sh \
  scripts/hepta-ui-current-readiness-self-test.sh \
  .github \
  codex-rs --glob '*.rs' --glob '*.yml' --glob '*.yaml' >/dev/null; then
  printf 'current readiness, CI, or Rust directly invokes an archived UI gate\n' >&2
  exit 1
fi

if rg -n 'scripts/hepta-ui-(demo-evidence|top-design-referee-refresh)-gate\.sh' \
  scripts/hepta-ui-current-readiness.sh \
  scripts/hepta-ui-current-readiness-self-test.sh \
  .github \
  codex-rs --glob '*.rs' --glob '*.yml' --glob '*.yaml' >/dev/null; then
  printf 'current readiness, CI, or Rust references a retired UI root report generator\n' >&2
  exit 1
fi

printf 'Hepta UI legacy visual archive self-test passed: 40 gates + 1 Native fixture\n'
