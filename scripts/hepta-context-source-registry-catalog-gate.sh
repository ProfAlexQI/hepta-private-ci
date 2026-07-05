#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
registry="$repo_root/codex-rs/CONTEXT_SOURCE_REGISTRY.tsv"
contracts="$repo_root/codex-rs/CONTEXT_DEBUG_CONTRACTS.md"
manifest_classifier="$repo_root/codex-rs/core/src/context_manager/manifest/classification.rs"
manifest_policy_candidate="$repo_root/codex-rs/core/src/context_manager/manifest/policy/candidate.rs"
manifest_policy_compression="$repo_root/codex-rs/core/src/context_manager/manifest/policy/compression.rs"
rust_registry="$repo_root/codex-rs/core/src/context_manager/source_registry.rs"
rust_registry_catalog="$repo_root/codex-rs/core/src/context_manager/source_registry/catalog.rs"
rust_registry_tests="$repo_root/codex-rs/core/src/context_manager/source_registry/tests.rs"
debug_gate="$repo_root/scripts/hepta-context-debug-gate.sh"
preflight_script="$repo_root/scripts/hepta-context-preflight.sh"

fail() {
  echo "hepta-context-source-registry-catalog-gate: $*" >&2
  exit 1
}

assert_file_contains() {
  local file_path="$1"
  local needle="$2"
  local label="$3"

  if ! grep -F "$needle" "$file_path" >/dev/null; then
    fail "$label must contain: $needle"
  fi
}

line_number_of() {
  local file_path="$1"
  local needle="$2"
  local line

  line="$(grep -n -F "$needle" "$file_path" | head -n 1 | cut -d: -f1 || true)"
  if [ -z "$line" ]; then
    fail "$file_path is missing required text: $needle"
  fi
  printf '%s\n' "$line"
}

assert_line_before() {
  local file_path="$1"
  local before_needle="$2"
  local after_needle="$3"
  local label="$4"
  local before_line
  local after_line

  before_line="$(line_number_of "$file_path" "$before_needle")"
  after_line="$(line_number_of "$file_path" "$after_needle")"
  if [ "$before_line" -ge "$after_line" ]; then
    fail "$label expected '$before_needle' before '$after_needle'"
  fi
}

normalized="$(mktemp -t hepta-context-source-registry.XXXXXX)"
expected="$(mktemp -t hepta-context-source-registry-expected.XXXXXX)"

cleanup() {
  rm -f "$normalized" "$expected"
}
trap cleanup EXIT

assert_file_contains "$registry" "# version=1" \
  "context source registry catalog version"

grep -v '^#' "$registry" >"$normalized"

cat >"$expected" <<'EOF'
source_id	tier	owner_lane	privacy_class	budget_class	ttl	volatility	trust_class	redaction_policy	quality_metric	activation_guard	rollback_policy	omit_priority	allowed_compression_actions
apps	tool	hepta-context	prompt_visible	tool_inventory	turn	medium	runtime_observed	prompt_hash_only	inventory_digest	candidate_only	rebuild_from_source	30	defragment
available_plugins	tool	hepta-context	prompt_visible	tool_inventory	turn	medium	system_owned	prompt_hash_only	inventory_digest	candidate_only	rebuild_from_source	20	defragment
available_skills	tool	hepta-context	prompt_visible	tool_inventory	turn	medium	system_owned	prompt_hash_only	inventory_digest	candidate_only	rebuild_from_source	40	defragment
collaboration_mode	developer	hepta-context	prompt_visible	protected_developer	session	low	system_owned	prompt_hash_only	policy_digest	protected	restore_previous	-	-
context	runtime	hepta-context	prompt_visible	fallback_context	turn	high	runtime_observed	metadata_only	freshness	candidate_only	drop_turn_fragment	-	-
contextual_user	user	hepta-context	prompt_visible	protected_user	turn	medium	user_owned	prompt_hash_only	presence	protected	drop_turn_fragment	-	-
developer_instructions	developer	hepta-context	prompt_visible	protected_developer	session	low	developer_owned	prompt_hash_only	policy_digest	protected	not_mutable	-	-
environment	runtime	hepta-context	prompt_visible	runtime_state	turn	high	runtime_observed	metadata_only	freshness	candidate_only	rebuild_from_source	-	-
extension_contextual_user	user	hepta-context	prompt_visible	protected_user	turn	medium	extension_owned	prompt_hash_only	extension_digest	protected	drop_turn_fragment	-	-
extension_developer_capabilities	tool	hepta-context	prompt_visible	tool_capability	turn	medium	extension_owned	prompt_hash_only	extension_digest	candidate_only	drop_turn_fragment	10	prune
extension_developer_policy	developer	hepta-context	prompt_visible	protected_developer	turn	medium	extension_owned	prompt_hash_only	extension_digest	protected	drop_turn_fragment	-	-
extension_separate_developer	developer	hepta-context	prompt_visible	protected_developer	turn	medium	extension_owned	prompt_hash_only	extension_digest	protected	drop_turn_fragment	-	-
model_switch	session_state	hepta-context	prompt_visible	session_state	turn	low	system_owned	metadata_only	presence	protected	restore_previous	-	-
non_text_content	runtime	hepta-context	prompt_visible	non_text_context	turn	high	runtime_observed	metadata_only	presence	protected	drop_turn_fragment	-	-
permissions	system	hepta-context	prompt_visible	protected_system	session	low	system_owned	prompt_hash_only	policy_digest	protected	not_mutable	-	-
personality	developer	hepta-context	prompt_visible	protected_developer	session	low	developer_owned	prompt_hash_only	policy_digest	protected	restore_previous	-	-
realtime	runtime	hepta-context	prompt_visible	runtime_state	turn	high	runtime_observed	metadata_only	freshness	candidate_only	rebuild_from_source	-	-
selected_context_recall	retrieved_snippets	hepta-context	bounded_recall_payload	bounded_recall	turn	high	retrieved_memory	guarded_envelope	recall_quality	operator_approval_required	rerun_recall	50	summary
user_instructions	user	hepta-context	prompt_visible	protected_user	session	low	user_owned	prompt_hash_only	policy_digest	protected	restore_previous	-	-
EOF

if ! diff -u "$expected" "$normalized"; then
  fail "context source registry catalog drifted from the stable read-only export"
fi

awk -F '\t' '
  NR == 1 {
    if ($0 != "source_id\ttier\towner_lane\tprivacy_class\tbudget_class\tttl\tvolatility\ttrust_class\tredaction_policy\tquality_metric\tactivation_guard\trollback_policy\tomit_priority\tallowed_compression_actions") {
      printf("bad header: %s\n", $0) > "/dev/stderr";
      exit 1;
    }
    next;
  }
  NF != 14 {
    printf("bad field count on line %d\n", NR) > "/dev/stderr";
    exit 1;
  }
  $1 !~ /^[a-z0-9_]+$/ {
    printf("bad source id on line %d\n", NR) > "/dev/stderr";
    exit 1;
  }
  seen[$1]++ {
    printf("duplicate source id: %s\n", $1) > "/dev/stderr";
    exit 1;
  }
  previous != "" && previous > $1 {
    printf("source ids are not sorted: %s before %s\n", previous, $1) > "/dev/stderr";
    exit 1;
  }
  $3 != "hepta-context" {
    printf("bad owner lane for %s: %s\n", $1, $3) > "/dev/stderr";
    exit 1;
  }
  $4 != "prompt_visible" && $4 != "bounded_recall_payload" {
    printf("bad privacy class for %s: %s\n", $1, $4) > "/dev/stderr";
    exit 1;
  }
  $6 != "turn" && $6 != "session" {
    printf("bad ttl for %s: %s\n", $1, $6) > "/dev/stderr";
    exit 1;
  }
  $7 != "low" && $7 != "medium" && $7 != "high" {
    printf("bad volatility for %s: %s\n", $1, $7) > "/dev/stderr";
    exit 1;
  }
  $8 != "system_owned" && $8 != "developer_owned" && $8 != "user_owned" && $8 != "extension_owned" && $8 != "runtime_observed" && $8 != "retrieved_memory" {
    printf("bad trust class for %s: %s\n", $1, $8) > "/dev/stderr";
    exit 1;
  }
  $9 != "prompt_hash_only" && $9 != "guarded_envelope" && $9 != "metadata_only" {
    printf("bad redaction policy for %s: %s\n", $1, $9) > "/dev/stderr";
    exit 1;
  }
  $10 != "presence" && $10 != "freshness" && $10 != "inventory_digest" && $10 != "extension_digest" && $10 != "recall_quality" && $10 != "policy_digest" {
    printf("bad quality metric for %s: %s\n", $1, $10) > "/dev/stderr";
    exit 1;
  }
  $11 != "protected" && $11 != "candidate_only" && $11 != "operator_approval_required" {
    printf("bad activation guard for %s: %s\n", $1, $11) > "/dev/stderr";
    exit 1;
  }
  $12 != "rebuild_from_source" && $12 != "restore_previous" && $12 != "drop_turn_fragment" && $12 != "rerun_recall" && $12 != "not_mutable" {
    printf("bad rollback policy for %s: %s\n", $1, $12) > "/dev/stderr";
    exit 1;
  }
  $13 != "-" && $13 !~ /^[0-9]+$/ {
    printf("bad omit priority for %s: %s\n", $1, $13) > "/dev/stderr";
    exit 1;
  }
  $14 != "-" && $14 !~ /^(summary|defragment|prune)(,(summary|defragment|prune))*$/ {
    printf("bad compression action list for %s: %s\n", $1, $14) > "/dev/stderr";
    exit 1;
  }
  {
    previous = $1;
    count++;
  }
  END {
    if (count != 19) {
      printf("expected 19 source registry entries, got %d\n", count) > "/dev/stderr";
      exit 1;
    }
  }
' "$normalized" || fail "context source registry schema validation failed"

while IFS=$'\t' read -r source_id tier owner_lane privacy_class budget_class omit_priority actions; do
  if [ "$source_id" = "source_id" ]; then
    continue
  fi

  assert_file_contains "$rust_registry_catalog" "source_id: \"$source_id\"" \
    "context source registry rust classification"
  assert_file_contains "$manifest_classifier" "\"$source_id\"" \
    "context source registry manifest classification"
done <"$normalized"

for term in \
  "context_source_registry_entry" \
  "source_aware_omit_priority" \
  "source_aware_compression_kind"; do
  assert_file_contains "$rust_registry" "$term" "context source registry rust resolver"
done

assert_file_contains "$rust_registry_tests" \
  "include_str!(\"../../../../CONTEXT_SOURCE_REGISTRY.tsv\")" \
  "context source registry TSV sync test"

for term in \
  "source_id: \"extension_developer_capabilities\"" \
  "source_id: \"available_plugins\"" \
  "source_id: \"apps\"" \
  "source_id: \"available_skills\"" \
  "source_id: \"selected_context_recall\"" \
  "omit_priority: Some(10)" \
  "omit_priority: Some(20)" \
  "omit_priority: Some(30)" \
  "omit_priority: Some(40)" \
  "omit_priority: Some(50)" \
  "TurnContextCompressionStageKind::Prune" \
  "TurnContextCompressionStageKind::Defragment" \
  "TurnContextCompressionStageKind::Summary"; do
  assert_file_contains "$rust_registry_catalog" "$term" "context source registry rust resolver"
done

assert_file_contains "$manifest_classifier" "registered_contribution_classification" \
  "context source registry manifest classifier"
assert_file_contains "$manifest_policy_candidate" "source_aware_omit_priority(" \
  "context source registry manifest omit priority lookup"
assert_file_contains "$manifest_policy_compression" "source_aware_compression_kind(" \
  "context source registry manifest compression lookup"

for term in \
  "Context source registry catalog" \
  "CONTEXT_SOURCE_REGISTRY.tsv" \
  "source_id" \
  "owner_lane" \
  "privacy_class" \
  "budget_class" \
  "ttl" \
  "volatility" \
  "trust_class" \
  "redaction_policy" \
  "quality_metric" \
  "activation_guard" \
  "rollback_policy" \
  "omit_priority" \
  "allowed_compression_actions" \
  "read-only catalog" \
  "must not become a runtime activation route"; do
  assert_file_contains "$contracts" "$term" "context source registry contract"
done

assert_file_contains "$debug_gate" "hepta-context-source-registry-catalog-gate.sh" \
  "context source registry debug gate"
assert_file_contains "$debug_gate" "hepta-context-source-registry-rust-gate.sh" \
  "context source registry rust debug gate"
assert_file_contains "$debug_gate" "hepta-context-source-registry-health-gate.sh" \
  "context source registry health debug gate"
assert_file_contains "$preflight_script" "context source registry catalog gate" \
  "context source registry preflight stage"
assert_file_contains "$preflight_script" "context source registry rust resolver gate" \
  "context source registry rust preflight stage"
assert_file_contains "$preflight_script" "context source registry health gate" \
  "context source registry health preflight stage"
assert_line_before \
  "$preflight_script" \
  "context source registry catalog gate" \
  "context source registry rust resolver gate" \
  "context source registry preflight stage order"
assert_line_before \
  "$preflight_script" \
  "context source registry rust resolver gate" \
  "context source registry health gate" \
  "context source registry preflight stage order"
assert_line_before \
  "$preflight_script" \
  "context source registry health gate" \
  "context adaptive budget allocation dry-run report gate" \
  "context source registry preflight stage order"

echo "context-source-registry=pass"
echo "context-source-registry.version=1"
echo "context-source-registry.entry-count=19"
echo "context-source-registry.owner-lane=hepta-context"
echo "context-source-registry.runtime-activation=disabled"
echo "Hepta context source registry catalog gate passed"
