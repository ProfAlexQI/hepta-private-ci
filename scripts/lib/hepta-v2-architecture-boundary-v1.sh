#!/usr/bin/env bash
set -euo pipefail

ROOT="${HEPTA_V2_ARCH_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)}"
source "$ROOT/scripts/lib/hepta-v2-test-inventory.sh"

usage() {
  echo "usage: scripts/hepta-v2-architecture-boundary <verify|self-test>" >&2
  exit 2
}

require_tool() {
  command -v "$1" >/dev/null || {
    echo "missing Hepta Architecture V2 boundary tool: $1" >&2
    return 1
  }
}

package_dependency_names() {
  local manifest="$1"
  python3 - "$ROOT/$manifest" "$ROOT/codex-rs/Cargo.toml" <<'PY'
import sys
import tomllib

with open(sys.argv[1], "rb") as handle:
    manifest = tomllib.load(handle)
with open(sys.argv[2], "rb") as handle:
    workspace_dependencies = tomllib.load(handle)["workspace"]["dependencies"]

def emit(table):
    for alias, specification in table.items():
        if isinstance(specification, dict):
            inherited = workspace_dependencies.get(alias, {})
            inherited_package = inherited.get("package") if isinstance(inherited, dict) else None
            print(specification.get("package", inherited_package or alias))
        else:
            print(alias)

for section in ("dependencies", "dev-dependencies", "build-dependencies"):
    emit(manifest.get(section, {}))

for target in manifest.get("target", {}).values():
    for section in ("dependencies", "dev-dependencies", "build-dependencies"):
        emit(target.get(section, {}))
PY
}

workspace_has_contract_boundary() {
  python3 - "$ROOT/codex-rs/Cargo.toml" <<'PY'
import sys
import tomllib

with open(sys.argv[1], "rb") as handle:
    manifest = tomllib.load(handle)

workspace = manifest["workspace"]
members = workspace["members"]
dependency = workspace["dependencies"].get("hepta-contracts")
valid_dependency = (
    isinstance(dependency, dict)
    and dependency.get("path") == "hepta-contracts"
)
raise SystemExit(0 if "hepta-contracts" in members and valid_dependency else 1)
PY
}

require_dependency() {
  local manifest="$1"
  local required="$2"
  local dependencies
  if ! dependencies="$(package_dependency_names "$manifest")"; then
    echo "Architecture V2 could not parse dependencies: $manifest" >&2
    return 1
  fi
  if ! grep -Fxq "$required" <<<"$dependencies"; then
    echo "Architecture V2 required dependency missing: $manifest -> $required" >&2
    return 1
  fi
}

deny_dependencies() {
  local manifest="$1"
  shift
  local dependencies dependency
  if ! dependencies="$(package_dependency_names "$manifest")"; then
    echo "Architecture V2 could not parse dependencies: $manifest" >&2
    return 1
  fi
  while IFS= read -r dependency; do
    local denied
    for denied in "$@"; do
      if [[ "$dependency" == "$denied" ]]; then
        echo "Architecture V2 forbidden dependency: $manifest -> $dependency" >&2
        return 1
      fi
    done
  done <<<"$dependencies"
}

verify_non_live_preference_authority() {
  local source_root output scan_rc
  for source_root in \
    codex-rs/hepta-runtime/src \
    codex-rs/hepta-gateway/src \
    codex-rs/hepta-native-gateway/src \
    codex-rs/hepta-cli/src \
    codex-rs/hepta-kernel/src
  do
    [[ -f "$ROOT/$source_root/lib.rs" || -f "$ROOT/$source_root/main.rs" ]] || {
      echo "Architecture V2 preference authority source entry is missing: $source_root" >&2
      return 1
    }
  done

  if output="$(
    rg -n -w --glob '*.rs' \
      -e PreferenceEvidenceRef \
      -e PreferenceTransition \
      -e PreferenceStateDocument \
      -e InMemoryPreferenceStore \
      -e reduce_explicit_preference \
      -e commit_evidenced \
      "$ROOT/codex-rs/hepta-runtime" \
      "$ROOT/codex-rs/hepta-gateway" \
      "$ROOT/codex-rs/hepta-native-gateway" \
      "$ROOT/codex-rs/hepta-cli" \
      "$ROOT/codex-rs/hepta-kernel" 2>&1
  )"; then
    echo "Architecture V2 non-live preference authority consumer detected:" >&2
    echo "$output" >&2
    return 1
  else
    scan_rc=$?
    if [[ "$scan_rc" != 1 ]]; then
      echo "Architecture V2 preference authority scan failed: $output" >&2
      return 1
    fi
  fi

  local context_freezer="codex-rs/hepta-runtime/src/runtime_kernel/context_freezer.rs"
  [[ -f "$ROOT/$context_freezer" ]] || {
    echo "Architecture V2 runtime context freezer is missing: $context_freezer" >&2
    return 1
  }
  if ! grep -Fqx \
    '    let preference_stamp = revisions.stamp(session_id, "preference:unattached", preference_hash)?;' \
    "$ROOT/$context_freezer"; then
    echo "Architecture V2 runtime preference must remain explicitly unattached" >&2
    return 1
  fi
}

verify_trusted_preference_feedback_boundary() {
  local memory_authority="codex-rs/hepta-memory/src/preference_authority.rs"
  local memory_types="codex-rs/hepta-memory/src/preference_cas/preference_authority_types.rs"
  local memory_canonical="codex-rs/hepta-memory/src/preference_cas/preference_authority_canonical.rs"
  local memory_tests="codex-rs/hepta-memory/src/preference_authority/tests.rs"
  local intelligence_authority="codex-rs/hepta-intelligence/src/trusted_preference_feedback.rs"
  local intelligence_tests="codex-rs/hepta-intelligence/src/trusted_preference_feedback/tests.rs"
  local source requirement

  for source in \
    "$memory_authority" "$memory_types" "$memory_canonical" "$memory_tests" \
    "$intelligence_authority" "$intelligence_tests"
  do
    [[ -f "$ROOT/$source" ]] || {
      echo "Architecture V2 trusted preference source is missing: $source" >&2
      return 1
    }
  done

  for requirement in \
    'pub fn advance_preference_with_authority<A, R>(' \
    'pub async fn advance_preference_with_authority<A, R>(' \
    'ensure_expected_previous(&request, &current)?' \
    'let challenge = PreferenceFeedbackChallenge::new(request, source.clone(), reducer_ref.clone());' \
    '.authenticate(&challenge)' \
    'let feedback = challenge.into_authenticated();' \
    'let commit = self.commit_evidenced(advance.transition, advance.document)?;' \
    '.commit_evidenced(advance.transition, advance.document)'
  do
    grep -Fq "$requirement" "$ROOT/$memory_authority" || {
      echo "Architecture V2 preference authority is incomplete: $requirement" >&2
      return 1
    }
  done

  for requirement in \
    'pub struct PreferenceFeedbackRequest {' \
    'transition_id: PreferenceTransitionId' \
    'evidence_id: PreferenceEvidenceId' \
    'receipt: ReceiptRef' \
    'session_binding_hash: ContentHash' \
    'subject: PrincipalId' \
    'preference: PreferenceId' \
    'target_binding_hash: ContentHash' \
    'expected_previous: PreferenceState' \
    'pub struct PreferenceFeedbackChallenge {' \
    'source: PreferenceFeedbackSourceRef' \
    'reducer: PreferenceReducerRef' \
    'pub struct AuthenticatedPreferenceFeedback {' \
    'pub trait PreferenceFeedbackAuthenticator' \
    'pub trait PreferenceDomainReducer'
  do
    grep -Fq "$requirement" "$ROOT/$memory_types" || {
      echo "Architecture V2 preference challenge binding is incomplete: $requirement" >&2
      return 1
    }
  done

  for requirement in \
    'const AUTHORITY_EVIDENCE_HASH_DOMAIN: &str = "hepta.memory.preference-authority.evidence.v1"' \
    'hash.text("source.identity", source.identity().as_str())' \
    'hash.text("reducer.identity", reducer.identity())' \
    'hash.text("transition.id", request.transition_id().as_str())' \
    'hash.text("receipt.id", request.receipt().id().as_str())' \
    'hash.text("subject", request.subject().as_str())' \
    '"target_binding_hash"' \
    '"expected_previous.revision"'
  do
    grep -Fq "$requirement" "$ROOT/$memory_canonical" || {
      echo "Architecture V2 preference evidence digest is incomplete: $requirement" >&2
      return 1
    }
  done

  for requirement in \
    'pub trait TrustedPreferenceFeedbackSource' \
    'pub struct TrustedExplicitPreferenceReducer' \
    'pub struct DurableTrustedPreferenceFeedbackAuthority<S>' \
    'DurablePreferenceStore::bootstrap_new_keyed(path, integrity_key)' \
    'DurablePreferenceStore::open_existing_keyed(path, integrity_key)' \
    'self.ensure_source_binding()?' \
    'pinned_source: Some(&self.source_binding)' \
    "pinned_source: Option<&'a PreferenceFeedbackSourceRef>" \
    'self.ensure_pinned_source()?' \
    'pub fn advance_trusted_explicit_preference<S>(' \
    'pub async fn advance_trusted_explicit_preference_durable<S>(' \
    'challenge.request().target_binding_hash() != &self.target.binding_hash()' \
    '.authenticate(&TrustedPreferenceFeedbackChallenge::new('
  do
    grep -Fq "$requirement" "$ROOT/$intelligence_authority" || {
      echo "Architecture V2 trusted preference adapter is incomplete: $requirement" >&2
      return 1
    }
  done
  if grep -Fq 'impl TrustedPreferenceFeedbackSource for' "$ROOT/$intelligence_authority"; then
    echo "Architecture V2 trusted preference adapter exposes a production source implementation" >&2
    return 1
  fi

  hepta_v2_assert_test_inventory "Architecture V2 preference authority" 6 '.*' \
    "$ROOT/$memory_tests"
  hepta_v2_assert_test_inventory "Architecture V2 trusted preference feedback" 5 '.*' \
    "$ROOT/$intelligence_tests"
}

verify_intuition_feedback_ownership() {
  local intelligence_source="codex-rs/hepta-intelligence/src/intuition_feedback_learning.rs"
  local runtime_source="codex-rs/hepta-runtime/src/query.rs"
  [[ -f "$ROOT/$intelligence_source" ]] || {
    echo "Architecture V2 intuition feedback owner is missing: $intelligence_source" >&2
    return 1
  }
  [[ -f "$ROOT/$runtime_source" ]] || {
    echo "Architecture V2 runtime feedback adapter is missing: $runtime_source" >&2
    return 1
  }

  local required
  for required in \
    'pub fn intuition_feedback_weight_delta' \
    'pub fn estimate_intuition_feedback_confidence' \
    'pub fn apply_intuition_feedback_to_topic_sessions' \
    'pub fn reduce_intuition_feedback_neurons'
  do
    grep -Fq "$required" "$ROOT/$intelligence_source" || {
      echo "Architecture V2 intelligence feedback reducer is incomplete: $required" >&2
      return 1
    }
    grep -Fq "${required#pub fn }" "$ROOT/$runtime_source" || {
      echo "Architecture V2 runtime feedback delegation is missing: ${required#pub fn }" >&2
      return 1
    }
  done

  local output scan_rc
  if output="$(
    rg -n \
      -e 'IntuitionFeedbackOutcome::Accepted => 0\.12' \
      -e 'feedback-learning:' \
      -e 'feedback-confirmed:' \
      -e 'feedback-review:' \
      "$ROOT/$runtime_source" 2>&1
  )"; then
    echo "Architecture V2 runtime-owned intuition feedback cognition detected:" >&2
    echo "$output" >&2
    return 1
  else
    scan_rc=$?
    [[ "$scan_rc" == 1 ]] || {
      echo "Architecture V2 intuition feedback ownership scan failed: $output" >&2
      return 1
    }
  fi
}

verify_intuition_planner_ownership() {
  local intelligence_source="codex-rs/hepta-intelligence/src/intuition_planner.rs"
  local runtime_source="codex-rs/hepta-runtime/src/query.rs"
  [[ -f "$ROOT/$intelligence_source" ]] || {
    echo "Architecture V2 intuition planner owner is missing: $intelligence_source" >&2
    return 1
  }
  grep -Fq 'pub fn plan_intuition' "$ROOT/$intelligence_source" || {
    echo "Architecture V2 intelligence planner entry point is missing" >&2
    return 1
  }
  grep -Fq 'pub struct IntuitionPlanInput' "$ROOT/$intelligence_source" || {
    echo "Architecture V2 intelligence planner input boundary is missing" >&2
    return 1
  }
  local delegation_count
  delegation_count="$(
    grep -Fc 'plan_intuition(IntuitionPlanInput {' "$ROOT/$runtime_source"
  )"
  if [[ "$delegation_count" != 1 ]]; then
    echo "Architecture V2 runtime intuition planner delegation count is $delegation_count, expected 1" >&2
    return 1
  fi
  if grep -Fq \
    'action_mode: IntuitionActionMode::ExecuteAllowed' \
    "$ROOT/$intelligence_source"; then
    echo "Architecture V2 intuition planner constructed execution authority" >&2
    return 1
  fi
  grep -Fq 'fn planner_never_emits_execution_authority' \
    "$ROOT/codex-rs/hepta-intelligence/src/intuition_planner/tests.rs" || {
      echo "Architecture V2 intuition planner authority regression test is missing" >&2
      return 1
    }

  local output scan_rc
  if output="$(
    rg -n \
      -e 'fn build_bootstrap_workflow_priors' \
      -e 'fn rank_intuition_workflow_candidate' \
      -e 'fn build_bootstrap_skill_decisions' \
      -e 'fn rank_intuition_skill_candidate' \
      -e 'fn default_intuition_workflow_registry' \
      -e 'fn score_registered_capability_for_intent' \
      "$ROOT/$runtime_source" 2>&1
  )"; then
    echo "Architecture V2 runtime-owned intuition planner cognition detected:" >&2
    echo "$output" >&2
    return 1
  else
    scan_rc=$?
    [[ "$scan_rc" == 1 ]] || {
      echo "Architecture V2 intuition planner ownership scan failed: $output" >&2
      return 1
    }
  fi
}

verify_capability_manifest_ownership() {
  local context_source="codex-rs/hepta-runtime/src/runtime_kernel/context_freezer.rs"
  local approval_source="codex-rs/hepta-runtime/src/runtime_kernel/approval_state.rs"
  local attempt_source="codex-rs/hepta-runtime/src/runtime_kernel/execution_attempt.rs"
  local descriptor_source="codex-rs/hepta-runtime/src/runtime_kernel/types.rs"
  local regression_source="codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_capability_descriptor.rs"
  local source
  for source in \
    "$context_source" "$approval_source" "$attempt_source" \
    "$descriptor_source" "$regression_source"
  do
    [[ -f "$ROOT/$source" ]] || {
      echo "Architecture V2 capability binding source is missing: $source" >&2
      return 1
    }
  done

  grep -Fq 'pub executor_provider: String' "$ROOT/$descriptor_source" &&
    grep -Fq 'pub operation: String' "$ROOT/$descriptor_source" &&
    grep -Fq 'CapabilityDescriptor::new(' "$ROOT/$context_source" &&
    grep -Fq 'capability_descriptor: CapabilityDescriptor' "$ROOT/$approval_source" &&
    grep -Fq 'current_capability_descriptor(' "$ROOT/$attempt_source" &&
    grep -Fq 'validate_capability_material(' "$ROOT/$attempt_source" &&
    grep -Fq \
      'fn architecture_v2_capability_descriptor_provider_and_operation_drift_fail_closed' \
      "$ROOT/$regression_source" || {
        echo "Architecture V2 exact capability descriptor binding is incomplete" >&2
        return 1
      }

  if rg -n -F 'CapabilityManifestRef::new' "$ROOT/$context_source" >/dev/null; then
    echo "Architecture V2 runtime synthesized a manifest reference without a descriptor" >&2
    return 1
  fi
}

verify_exact_admission_boundary() {
  local kernel_policy="codex-rs/hepta-kernel/src/safety_gate/policy_evidence.rs"
  local kernel_admission="codex-rs/hepta-kernel/src/safety_gate/admission.rs"
  local kernel_regression="codex-rs/hepta-kernel/src/safety_gate/admission_tests.rs"
  local runtime_adapter="codex-rs/hepta-runtime/src/runtime_kernel/safety_gate_client/admission.rs"
  local runtime_regression="codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_exact_safety.rs"
  local source requirement output scan_rc
  for source in \
    "$kernel_policy" "$kernel_admission" "$kernel_regression" \
    "$runtime_adapter" "$runtime_regression"
  do
    [[ -f "$ROOT/$source" ]] || {
      echo "Architecture V2 exact admission source is missing: $source" >&2
      return 1
    }
  done

  for requirement in \
    'pub struct HeptaKernelPolicyEvidence' \
    'default_rules: Vec<PolicyRule>' \
    'custom_rules: Vec<PolicyRule>' \
    'fn assess(' \
    'policy_snapshot_hash(&self.default_rules, &self.custom_rules)' \
    'fn evaluate(&self) -> Option<PolicyDecision>' \
    'expected != self.presented_decision' \
    'Some(candidate.metacontrol_hash())'
  do
    grep -Fq "$requirement" "$ROOT/$kernel_policy" || {
      echo "Architecture V2 kernel policy evidence replay is incomplete: $requirement" >&2
      return 1
    }
  done
  grep -Fq 'evidence.policy.assess(candidate, &evidence.capability)' \
    "$ROOT/$kernel_admission" || {
      echo "Architecture V2 kernel candidate admission does not assess replayed policy evidence" >&2
      return 1
    }

  for requirement in \
    'HeptaKernelPolicyEvidence::new(' \
    'runtime.policy.default_rules()' \
    '.custom_rules()' \
    'gate.admit_candidate(' \
    'admission.rejection_reason_code()'
  do
    grep -Fq "$requirement" "$ROOT/$runtime_adapter" || {
      echo "Architecture V2 runtime exact-admission adapter is incomplete: $requirement" >&2
      return 1
    }
  done

  if output="$(rg -n -F 'AdmissionDecision::Admitted' "$ROOT/codex-rs/hepta-runtime/src" 2>&1)"; then
    echo "Architecture V2 runtime minted an admitted candidate outside the kernel:" >&2
    echo "$output" >&2
    return 1
  else
    scan_rc=$?
    [[ "$scan_rc" == 1 ]] || {
      echo "Architecture V2 runtime self-admission scan failed: $output" >&2
      return 1
    }
  fi

  grep -Fq 'fn architecture_v2_exact_admission_rejects_deny_before_approval' \
    "$ROOT/$runtime_regression" &&
    grep -Fq 'fn deny_and_inconsistent_policy_evidence_fail_closed' \
      "$ROOT/$kernel_regression" &&
    grep -Fq 'let fabricated_allow = candidate_with_requirements(' \
      "$ROOT/$kernel_regression" &&
    grep -Fq 'Some(admission_reason::POLICY_DECISION_MISMATCH)' \
      "$ROOT/$kernel_regression" || {
        echo "Architecture V2 exact-admission deny/decision-forgery regressions are incomplete" >&2
        return 1
      }
}

verify_v2_test_inventories() {
  local runtime_tests="codex-rs/hepta-runtime/src/runtime_kernel/tests"
  local memory_tests="codex-rs/hepta-memory/src/tests"

  hepta_v2_assert_test_inventory "Architecture V2 stable contracts" 11 '.*' \
    "$ROOT/codex-rs/hepta-contracts/tests/stable_contracts.rs"
  hepta_v2_assert_test_inventory "Architecture V2 intelligence neuron activation" 4 '.*' \
    "$ROOT/codex-rs/hepta-intelligence/src/neuron_activation/tests.rs"
  hepta_v2_assert_test_inventory "Architecture V2 tool candidate" 4 '.*' \
    "$ROOT/codex-rs/hepta-intelligence/src/tool_candidate.rs"
  hepta_v2_assert_test_inventory "Architecture V2 intuition feedback learner" 4 '.*' \
    "$ROOT/codex-rs/hepta-intelligence/src/intuition_feedback_learning/tests.rs"
  hepta_v2_assert_test_inventory "Architecture V2 intuition planner" 8 '.*' \
    "$ROOT/codex-rs/hepta-intelligence/src/intuition_planner/tests.rs"
  hepta_v2_assert_test_inventory "Architecture V2 explicit-preference reducer" 8 '.*' \
    "$ROOT/codex-rs/hepta-intelligence/src/preference_feedback/tests.rs"
  hepta_v2_assert_test_inventory "Architecture V2 trusted preference feedback" 5 '.*' \
    "$ROOT/codex-rs/hepta-intelligence/src/trusted_preference_feedback/tests.rs"
  hepta_v2_assert_test_inventory "Architecture V2 preference authority" 6 '.*' \
    "$ROOT/codex-rs/hepta-memory/src/preference_authority/tests.rs"
  hepta_v2_assert_test_inventory "Architecture V2 kernel safety gate" 12 '.*' \
    "$ROOT/codex-rs/hepta-kernel/src/safety_gate/tests.rs" \
    "$ROOT/codex-rs/hepta-kernel/src/safety_gate/admission_tests.rs"
  hepta_v2_assert_test_inventory "Architecture V2 preference-CAS" 35 '.*' \
    "$ROOT/$memory_tests/preference_cas.rs" \
    "$ROOT/$memory_tests/preference_cas/document.rs" \
    "$ROOT/$memory_tests/preference_cas/durable.rs" \
    "$ROOT/$memory_tests/preference_cas/durable_concurrency.rs" \
    "$ROOT/$memory_tests/preference_cas/durable_opening.rs" \
    "$ROOT/$memory_tests/preference_cas/durable_opening_security.rs" \
    "$ROOT/$memory_tests/preference_cas/fixtures.rs" \
    "$ROOT/$memory_tests/preference_cas/legacy.rs"
  hepta_v2_assert_test_inventory "Architecture V2 durable preference-CAS" 19 '.*' \
    "$ROOT/$memory_tests/preference_cas/durable.rs" \
    "$ROOT/$memory_tests/preference_cas/durable_concurrency.rs" \
    "$ROOT/$memory_tests/preference_cas/durable_opening.rs" \
    "$ROOT/$memory_tests/preference_cas/durable_opening_security.rs"
  hepta_v2_assert_test_inventory "Architecture V2 durable opening security" 7 '.*' \
    "$ROOT/$memory_tests/preference_cas/durable_opening_security.rs"
  hepta_v2_assert_test_inventory "Architecture V2 durable sidecar lifecycle" 2 \
    'unlinked_open_sidecar_.*' \
    "$ROOT/codex-rs/hepta-memory/src/durable/opening/filesystem.rs"
  hepta_v2_assert_test_inventory "Architecture V2 outcome-store" 50 '.*' \
    "$ROOT/$memory_tests/outcome_store.rs" \
    "$ROOT/$memory_tests/outcome_store/durable.rs" \
    "$ROOT/$memory_tests/outcome_store/effect_ack.rs" \
    "$ROOT/$memory_tests/outcome_store/execution_intent.rs" \
    "$ROOT/$memory_tests/outcome_store/pending_intent.rs" \
    "$ROOT/$memory_tests/outcome_store/sync_writer.rs"
  hepta_v2_assert_test_inventory "Architecture V2 durable outcome-store" 29 '.*' \
    "$ROOT/$memory_tests/outcome_store/durable.rs" \
    "$ROOT/$memory_tests/outcome_store/effect_ack.rs" \
    "$ROOT/$memory_tests/outcome_store/execution_intent.rs" \
    "$ROOT/$memory_tests/outcome_store/pending_intent.rs"
  hepta_v2_assert_test_inventory "Architecture V2 durable effect ACK" 4 '.*' \
    "$ROOT/$memory_tests/outcome_store/effect_ack.rs"
  hepta_v2_assert_test_inventory "Architecture V2 durable execution intent" 9 '.*' \
    "$ROOT/$memory_tests/outcome_store/execution_intent.rs"
  hepta_v2_assert_test_inventory "Architecture V2 sync durable outcome writer" 13 '.*' \
    "$ROOT/$memory_tests/outcome_store/sync_writer.rs"
  hepta_v2_assert_test_inventory "Architecture V2 runtime neuron hydration" 8 \
    "$HEPTA_V2_RUNTIME_NEURON_TEST_PATTERN" \
    "$ROOT/codex-rs/hepta-runtime/src/query/tests.rs"
  hepta_v2_assert_test_inventory "Architecture V2 exact-safety" 10 \
    'architecture_v2_exact_(safety|admission)_.*' \
    "$ROOT/$runtime_tests/architecture_v2_exact_safety.rs"
  hepta_v2_assert_test_inventory "Architecture V2 execution-lease" 5 \
    'architecture_v2_execution_lease_.*' \
    "$ROOT/$runtime_tests/architecture_v2_execution_lease.rs"
  hepta_v2_assert_test_inventory "Architecture V2 outcome-receipt" 4 \
    'architecture_v2_.*' \
    "$ROOT/$runtime_tests/architecture_v2_terminal_outcome.rs"
  hepta_v2_assert_test_inventory "Architecture V2 outcome-flow" 8 \
    'architecture_v2_.*' \
    "$ROOT/$runtime_tests/architecture_v2_outcome_flow.rs"
  hepta_v2_assert_test_inventory "Architecture V2 runtime outcome sink" 19 '.*' \
    "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink/tests.rs"
  hepta_v2_assert_test_inventory "Architecture V2 provider idempotency" 2 \
    'architecture_v2_provider_idempotency_.*' \
    "$ROOT/$runtime_tests/architecture_v2_provider_idempotency.rs"
  hepta_v2_assert_test_inventory "Architecture V2 resource-reservation" 4 \
    'architecture_v2_resource_reservation_.*' \
    "$ROOT/$runtime_tests/architecture_v2_resource_reservation.rs"
  hepta_v2_assert_test_inventory "Architecture V2 capability-descriptor" 4 \
    'architecture_v2_capability_descriptor_.*' \
    "$ROOT/$runtime_tests/architecture_v2_capability_descriptor.rs"
  hepta_v2_assert_test_inventory "Architecture V2 symlink-reservation" 4 \
    'architecture_v2_symlink_reservation_.*' \
    "$ROOT/$runtime_tests/architecture_v2_symlink_reservation.rs"
  hepta_v2_assert_test_inventory "Architecture V2 process-reservation" 8 \
    'architecture_v2_process_reservation_.*' \
    "$ROOT/$runtime_tests/architecture_v2_process_reservation.rs"
  hepta_v2_assert_test_inventory "Architecture V2 cross-process write lock" 4 '.*' \
    "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/cross_process_write_lock.rs"
  hepta_v2_assert_test_inventory "Architecture V2 dispatch-selector" 3 \
    'architecture_v2_dispatch_selector_.*' \
    "$ROOT/$runtime_tests/architecture_v2_dispatch_selector.rs"
  hepta_v2_assert_test_inventory "Architecture V2 native-mutation" 8 \
    'architecture_v2_native_mutation_.*' \
    "$ROOT/$runtime_tests/architecture_v2_native_mutation.rs"
  hepta_v2_assert_test_inventory "Architecture V2 provider-effect ACK" 2 \
    'architecture_v2_provider_effect_.*' \
    "$ROOT/$runtime_tests/architecture_v2_provider_effect.rs"
  hepta_v2_assert_test_inventory "Architecture V2 sealed-read" 9 \
    'architecture_v2_sealed_read_.*' \
    "$ROOT/$runtime_tests/architecture_v2_sealed_read.rs"
  hepta_v2_assert_test_inventory "Architecture V2 process-control" 2 \
    'architecture_v2_process_control_.*' \
    "$ROOT/$runtime_tests/architecture_v2_process_control.rs"
  hepta_v2_assert_test_inventory "Architecture V2 maintenance-mutation" 7 \
    'architecture_v2_maintenance_.*' \
    "$ROOT/$runtime_tests/architecture_v2_maintenance_mutation.rs"
}

verify_sealed_execution_boundaries() {
  local runtime_root="codex-rs/hepta-runtime/src/runtime_kernel" requirement source token minimum count family expected regression
  for requirement in \
    'types.rs|anchor_directory: fs::File|1' 'transaction_ops.rs|libc::openat(|1' \
    'transaction_ops.rs|libc::O_NOFOLLOW|1' 'tool_support.rs|write_file requires an identity-bound execution reservation|2' \
    'execution_attempt.rs|fn validate_dispatch_selector(|1' 'execution_bus.rs|.invoke_authorized(|1' \
    'execution_bus.rs|execution.executor()|1' 'execution_bus.rs|execution.capability()|1' \
    'types.rs|struct SealedWriteIdentity|1' 'types.rs|process_reservation_id: Option<String>|1' \
    'types.rs|struct PreparedWriteReservationSet|1' \
    'transaction_ops.rs|fn process_write_reservation_registry()|1' \
    'transaction_ops.rs|fn acquire_sealed_write_target_reservation(|1' \
    'transaction_ops.rs|fn sealed_write_identities_conflict(|1' \
    'transaction_ops.rs|fn preflight_prepared_native_mutation(|1' \
    'transaction_ops.rs|fn invoke_prepared_native_mutation(|1' \
    'execution_lease.rs|fn prepared_write_transactions(|1'
  do
    source="${requirement%%|*}"; requirement="${requirement#*|}"
    minimum="${requirement##*|}"; token="${requirement%|*}"
    count="$(grep -Fc "$token" "$ROOT/$runtime_root/$source" || true)"
    (( count >= minimum )) || {
      echo "Architecture V2 sealed execution boundary is incomplete:" \
        "$source -> $token (expected at least $minimum, got $count)" >&2
      return 1
    }
  done
  if rg -q '^fn native_compat_(write|edit|apply_patch|tts)\(' \
    "$ROOT/$runtime_root/tool_support.rs"; then
    echo "Architecture V2 native mutation retained an unsealed provider path" >&2
    return 1
  fi
  for requirement in \
    'symlink_reservation|4' 'process_reservation|8' 'dispatch_selector|3' \
    'native_mutation|8' 'sealed_read|9' 'provider_effect|2'
  do
    family="${requirement%%|*}"
    expected="${requirement##*|}"
    regression="$runtime_root/tests/architecture_v2_${family}.rs"
    count="$(
      hepta_v2_test_pair_count "architecture_v2_${family}_.*" "$ROOT/$regression"
    )" || return 1
    [[ "$count" == "$expected" ]] || {
      echo "Architecture V2 sealed execution boundary is incomplete:" \
        "$family attributed test inventory expected $expected, got ${count:-0}" >&2
      return 1
    }
  done
}

verify_quarantined_native_process_surface() {
  local source="codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  local tests="codex-rs/hepta-runtime/src/runtime_kernel/tests.rs"
  local registry_block production_tools premodel_block offering_block
  registry_block="$(sed -n '/^    fn new() -> Self {/,/^    #\[cfg(test)\]/p' "$ROOT/$source")"
  production_tools="$(sed -n '/^fn native_openclaw_compatible_tools()/,/^#\[cfg(test)\]/p' "$ROOT/$source")"
  premodel_block="$(sed -n '/^fn native_pre_model_tool_call(/,/^fn should_offer_model_tools_for_turn(/p' "$ROOT/$source")"
  offering_block="$(sed -n '/^fn should_offer_model_tools_for_turn(/,/^fn requests_quarantined_native_tool(/p' "$ROOT/$source")"

  if grep -Eq 'quarantined_exec_process|"(exec|process)"|B::(Exec|Process)' \
    <<<"$registry_block$production_tools"; then
    echo "Architecture V2 production registry exposes quarantined exec/process" >&2
    return 1
  fi
  grep -Fq 'if requests_quarantined_native_tool(input) {' <<<"$premodel_block" &&
    grep -Fq 'return None;' <<<"$premodel_block" &&
    ! grep -Eq 'extract_explicit_(exec|process)_tool_call' <<<"$premodel_block" &&
    grep -Fq 'if requests_quarantined_native_tool(user_text) {' <<<"$offering_block" &&
    grep -Fq 'return false;' <<<"$offering_block" || {
      echo "Architecture V2 pre-model routing exposes quarantined exec/process" >&2
      return 1
    }
  for requirement in \
    'fn new_with_quarantined_exec_process_for_test()' \
    'fn quarantined_exec_process_tools_for_test()' \
    'assert_eq!(tools.len(), 42);' \
    'explicit_exec_intent_is_quarantined_before_model_routing' \
    'explicit_process_intent_is_quarantined_before_model_routing'
  do
    grep -Fq "$requirement" "$ROOT/$source" "$ROOT/$tests" || {
      echo "Architecture V2 exec/process quarantine regression is incomplete: $requirement" >&2
      return 1
    }
  done
  perl -0e '
    local $/;
    my $source = <>;
    exit(
      $source =~ /native_tool\(\s*"process"\s*,\s*"[^"]*"\s*,\s*RiskTier::High\s*,\s*false\s*,\s*true\s*,\s*false\s*,\s*B::Process\s*,?\s*\)/s
        ? 0
        : 1
    );
  ' "$ROOT/$source" || {
    echo "Architecture V2 quarantined process descriptor must remain High, non-read-only, and destructive" >&2
    return 1
  }
}

verify_backup_prune_mutation_boundary() {
  local runtime_root="codex-rs/hepta-runtime/src/runtime_kernel"
  local source="$runtime_root/transaction_ops.rs"
  local session_source="$runtime_root/session_ops.rs"
  local prune_block
  prune_block="$(
    sed -n '/^    fn plan_backup_prune(/,/^    }/p' "$ROOT/$source"
  )"
  [[ -n "$prune_block" ]] || {
    echo "Architecture V2 backup prune boundary is missing" >&2
    return 1
  }
  if grep -Fq 'fs::remove_file(' <<<"$prune_block"; then
    echo "Architecture V2 backup prune bypasses identity-sealed deletion" >&2
    return 1
  fi
  perl -0e '
    local $/;
    my $source = <>;
    exit(
      $source =~ /#\[cfg\(not\(test\)\)\]\s+pub fn prune_backups\(.*?backup prune is quarantined until rollback references have a durable cross-process pin catalog.*?\n    \}/s
        ? 0
        : 1
    );
  ' "$ROOT/$session_source" || {
    echo "Architecture V2 release backup prune is not quarantined" >&2
    return 1
  }
}

verify_mutation_transaction_evidence_boundary() {
  local runtime_root="codex-rs/hepta-runtime/src/runtime_kernel"
  local bus="$runtime_root/execution_bus.rs"
  local lease="$runtime_root/execution_lease.rs"
  local recorder="$runtime_root/outcome_recorder.rs"
  local transactions="$runtime_root/transaction_ops.rs"
  local requirement source token

  for requirement in \
    'execution_bus.rs|record_mutation_transactions_from_tool_result(' \
    'execution_bus.rs|execution.prepared_write_transactions()' \
    'execution_lease.rs|pub(super) fn prepared_write_transactions(&self) -> &[PreparedWriteTransaction]' \
    'transaction_ops.rs|fn record_mutation_transactions_from_tool_result(' \
    'transaction_ops.rs|prepared: &[PreparedWriteTransaction]' \
    'outcome_recorder.rs|!execution.prepared_write_transactions().is_empty()'
  do
    source="${requirement%%|*}"
    token="${requirement#*|}"
    grep -Fq "$token" "$ROOT/$runtime_root/$source" || {
      echo "Architecture V2 generalized mutation transaction evidence is incomplete: $source -> $token" >&2
      return 1
    }
  done

  if rg -n -F 'prepared_write_transaction()' \
    "$ROOT/$bus" "$ROOT/$lease" "$ROOT/$recorder" "$ROOT/$transactions" >/dev/null
  then
    echo "Architecture V2 mutation transaction evidence retained a singular write-only getter" >&2
    return 1
  fi
  if grep -Fq 'execution.tool_name() == "write_file"' "$ROOT/$recorder"; then
    echo "Architecture V2 mutation transaction evidence retained a write_file-only terminal path" >&2
    return 1
  fi
}

verify_mutation_durability_boundary() {
  local runtime_root="codex-rs/hepta-runtime/src/runtime_kernel"
  local transactions="$runtime_root/transaction_ops.rs"
  local regression="$runtime_root/tests/architecture_v2_native_mutation.rs"
  local requirement

  for requirement in \
    'enum StagedFileInstallOutcome {' \
    'DurabilityAmbiguous(HeptaError)' \
    'fn install_staged_file(' \
    'fn committed_mutation_observed_after_error(' \
    'fn inject_atomic_install_post_commit_failure_for_test()' \
    '"mutation_durability_ambiguous: {}"' \
    '"applied_effect_observed_after_tool_error"'
  do
    grep -Fq "$requirement" "$ROOT/$transactions" || {
      echo "Architecture V2 atomic mutation durability boundary is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'inject_atomic_install_post_commit_failure_for_test();' \
    'starts_with("mutation_durability_ambiguous:")' \
    '["effect.disposition","recorded"]'
  do
    grep -Fq "$requirement" "$ROOT/$regression" || {
      echo "Architecture V2 atomic mutation ambiguity regression is incomplete: $requirement" >&2
      return 1
    }
  done
}

verify_execution_intent_boundary() {
  local runtime_root="codex-rs/hepta-runtime/src/runtime_kernel"
  local memory_root="codex-rs/hepta-memory/src/outcome_store"
  local attempt="$runtime_root/execution_attempt.rs"
  local bus="$runtime_root/execution_bus.rs"
  local tools="$runtime_root/tool_support.rs"
  local transactions="$runtime_root/transaction_ops.rs"
  local intent="$memory_root/execution_intent.rs"
  local durable="$memory_root/durable/execution_intent.rs"
  local terminal="$memory_root/durable/execution_intent/terminal_evidence.rs"
  local commit="$memory_root/durable/intent.rs"
  local regression="codex-rs/hepta-memory/src/tests/outcome_store/execution_intent.rs"
  local requirement terminal_field_count

  for source in \
    "$attempt" "$bus" "$tools" "$transactions" "$intent" "$durable" \
    "$terminal" "$commit" "$regression"
  do
    [[ -f "$ROOT/$source" ]] || {
      echo "Architecture V2 exact execution-intent source is missing: $source" >&2
      return 1
    }
  done

  perl -0e '
    local $/;
    my $source = <>;
    exit(
      $source =~ /AuthorizedToolExecution::stage_execution_intent.*?captured\.provider_invocation_started = true;.*?\.invoke_authorized\(/s
        ? 0
        : 1
    );
  ' "$ROOT/$bus" || {
    echo "Architecture V2 provider invocation is not ordered after durable intent staging" >&2
    return 1
  }
  for requirement in \
    'self.outcome_sink.stage_execution_intent(&intent)' \
    'pending_execution_intent(intent.attempt_id())' \
    'Ok(Some(recovered)) if recovered == intent' \
    'provider dispatch blocked because execution intent was not durably staged'
  do
    grep -Fq "$requirement" "$ROOT/$attempt" || {
      echo "Architecture V2 runtime execution-intent staging is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'const IDEMPOTENCY_KEY_DOMAIN: &str = "hepta.memory.execution-intent.idempotency-key.v3"' \
    'canonical_effect_plan: Option<String>' \
    'candidate_reference_hash: ContentHash'
  do
    grep -Fq "$requirement" "$ROOT/$intent" || {
      echo "Architecture V2 durable intent digest binding is incomplete: $requirement" >&2
      return 1
    }
  done
  grep -Fq 'candidate_reference_hash: candidate_reference_hash(authorization.candidate())' \
    "$ROOT/$attempt" || {
      echo "Architecture V2 runtime intent lacks an exact candidate-reference digest" >&2
      return 1
    }

  for requirement in \
    'ProviderExecutionIdentity::from_exact_context(' \
    'expected_attempt_id' \
    'expected_idempotency_key' \
    'HEPTA_EXECUTION_ATTEMPT_ID' \
    'HEPTA_EXECUTION_IDEMPOTENCY_KEY'
  do
    grep -Fq "$requirement" "$ROOT/$tools" || {
      echo "Architecture V2 provider attempt/idempotency binding is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'expected_attempt_id: &str' \
    'expected_idempotency_key: &str' \
    'presented_attempt_id: &str' \
    'presented_idempotency_key: &str' \
    'presented provider execution identity differs from the staged execution intent'
  do
    grep -Fq "$requirement" "$ROOT/$transactions" || {
      echo "Architecture V2 exact provider identity validation is incomplete: $requirement" >&2
      return 1
    }
  done

  perl -0e '
    local $/;
    my $source = <>;
    exit(
      $source =~ /pub\(crate\) async fn commit_staged_intent_and_resolve_execution\(.*?resolve_execution_intent_in_transaction\(.*?commit_intent_transaction\(/s
        ? 0
        : 1
    );
  ' "$ROOT/$commit" || {
    echo "Architecture V2 outcome record and execution-intent resolve are not one transaction" >&2
    return 1
  }
  for requirement in \
    'const EXECUTION_INTENT_ROW_SCHEMA_VERSION: u32 = 4' \
    'candidate_reference_hash: String' \
    'WHERE attempt_id = ? AND idempotency_key = ?' \
    'validate_terminal_binding(&intent, outcome, effect_ack.as_ref())?' \
    'terminal_evidence::validate(intent, outcome, effect_ack)'
  do
    grep -Fq "$requirement" "$ROOT/$durable" || {
      echo "Architecture V2 durable execution-intent resolution is incomplete: $requirement" >&2
      return 1
    }
  done

  terminal_field_count="$(
    sed -n '/^const TERMINAL_FIELDS:/,/^];/p' "$ROOT/$terminal" |
      grep -Ec 'FieldKind::(Text|Number)'
  )"
  [[ "$terminal_field_count" == 80 ]] || {
    echo "Architecture V2 strict terminal evidence field inventory drifted: expected 80, got $terminal_field_count" >&2
    return 1
  }
  for requirement in \
    'candidate_reference_hash(candidate) != *intent.candidate_reference_hash()' \
    'object.len() != 2' \
    'values.len() != TERMINAL_FIELDS.len()' \
    'canonical != canonical_evidence' \
    '&evidence.evidence_hash != outcome.canonical_evidence_hash()' \
    '&evidence.evidence_hash != outcome.receipt().outcome_hash()' \
    'validate_intent(intent, effect_ack, &evidence)?' \
    'validate_receipt(intent, outcome.receipt(), &evidence)?' \
    'validate_terminal_shape(intent, outcome.receipt(), &evidence)' \
    'receipt.receipt_hash().as_str() != computed_hash.as_str()'
  do
    grep -Fq "$requirement" "$ROOT/$terminal" || {
      echo "Architecture V2 strict terminal evidence verification is incomplete: $requirement" >&2
      return 1
    }
  done
  hepta_v2_assert_test_inventory "Architecture V2 durable execution intent" 9 '.*' \
    "$ROOT/$regression"
}

verify_provider_effect_boundary() {
  local runtime_root="codex-rs/hepta-runtime/src/runtime_kernel"
  local memory_root="codex-rs/hepta-memory/src"
  local provider="$runtime_root/provider_effect.rs"
  local bus="$runtime_root/execution_bus.rs"
  local sink="$runtime_root/outcome_sink.rs"
  local transaction="$runtime_root/transaction_ops.rs"
  local database="$memory_root/durable/schema.rs"
  local integrity="$memory_root/durable/integrity.rs"
  local effect_ack="$memory_root/outcome_store/effect_ack.rs"
  local durable_ack="$memory_root/outcome_store/durable/effect_ack.rs"
  local durable_intent="$memory_root/outcome_store/durable/execution_intent.rs"
  local terminal="$memory_root/outcome_store/durable/execution_intent/terminal_evidence.rs"
  local memory_regression="$memory_root/tests/outcome_store/effect_ack.rs"
  local runtime_regression="$runtime_root/tests/architecture_v2_provider_effect.rs"
  local native_regression="$runtime_root/tests/architecture_v2_native_mutation.rs"
  local source requirement

  for source in \
    "$provider" "$bus" "$sink" "$transaction" "$database" "$integrity" "$effect_ack" "$durable_ack" \
    "$durable_intent" "$terminal" "$memory_regression" "$runtime_regression" \
    "$native_regression"
  do
    [[ -f "$ROOT/$source" ]] || {
      echo "Architecture V2 provider effect source is missing: $source" >&2
      return 1
    }
  done

  for requirement in \
    'CREATE TABLE IF NOT EXISTS hepta_v2_execution_effect_acks (' \
    'effect_plan_hash TEXT NOT NULL'
  do
    grep -Fq "$requirement" "$ROOT/$database" || {
      echo "Architecture V2 provider effect durable schema is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'pub struct ExecutionEffectAckParts' \
    'pub struct ExecutionEffectAck' \
    'const EFFECT_ACK_DOMAIN: &str = "hepta.memory.execution-effect-ack.v1"' \
    'strip_prefix(&expected_prefix)' \
    'canonical_provider_ack'
  do
    grep -Fq "$requirement" "$ROOT/$effect_ack" || {
      echo "Architecture V2 provider effect ACK contract is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'pub async fn record_execution_effect_ack(' \
    'pub async fn execution_effect_ack(' \
    'pub(super) async fn execution_effect_ack_for_intent(' \
    'ExecutionEffectAckIntentMissing' \
    'ExecutionEffectAckBindingMismatch'
  do
    grep -Fq "$requirement" "$ROOT/$durable_ack" || {
      echo "Architecture V2 provider effect ACK persistence is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'pub(crate) fn canonical_effect_plan_for(' \
    'pub(crate) fn acknowledge_provider_invocation(' \
    'match sink.record_execution_effect_ack(&ack)' \
    'pub(crate) fn confirm_provider_effect_ack(' \
    'pub(crate) fn inspect_pending_effect(' \
    'prepared.staged_after_bytes.clone()' \
    'live tts effect plan lacks exact privately staged audio bytes' \
    'fn read_regular_single_link_file(' \
    'libc::O_CLOEXEC | libc::O_NOFOLLOW' \
    'metadata.nlink() != 1' \
    'ExecutionEffectInspectionState::AppliedAcknowledged' \
    'ExecutionEffectInspectionState::AppliedUnacknowledged' \
    'ExecutionEffectInspectionState::InDoubt'
  do
    grep -Fq "$requirement" "$ROOT/$provider" || {
      echo "Architecture V2 provider effect ACK boundary is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'transaction.staged_after_bytes =' \
    'stage_native_tts_audio(input_json)' \
    'std::process::Command::new("/usr/bin/say")' \
    '.arg("--")' \
    'live tts provider lacks the exact audio bytes staged before durable intent' \
    'write_prepared_target(prepared, &prepared.mode_requested, audio)' \
    '"synthesis_staged_before_intent": true' \
    '"provider_installed_staged_bytes": true'
  do
    grep -Fq "$requirement" "$ROOT/$transaction" || {
      echo "Architecture V2 staged TTS provider effect is incomplete: $requirement" >&2
      return 1
    }
  done

  perl -0e '
    local $/;
    my $source = <>;
    exit(
      $source =~ /\.invoke_authorized\(.*?provider_invocation_completed = true;.*?confirm_provider_effect_ack.*?capture_invocation_result\(/s
        ? 0
        : 1
    );
  ' "$ROOT/$bus" || {
    echo "Architecture V2 provider effect ACK is not confirmed before terminal capture" >&2
    return 1
  }
  for requirement in \
    'provider effect is in doubt and requires reconciliation' \
    'execution.trip_outcome_breaker(reason)' \
    'execution.disarm_receipt_guard()' \
    'execution.release_execution_lease()'
  do
    grep -Fq "$requirement" "$ROOT/$bus" || {
      echo "Architecture V2 provider effect fail-closed path is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'fn record_execution_effect_ack(' \
    'fn execution_effect_ack(' \
    'pub fn pending_execution_effect_inspections('
  do
    grep -Fq "$requirement" "$ROOT/$sink" || {
      echo "Architecture V2 provider effect sink boundary is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'execution_effect_ack_for_intent(database, transaction, &intent).await?' \
    'effect_ack.as_ref()'
  do
    grep -Fq "$requirement" "$ROOT/$durable_intent" || {
      echo "Architecture V2 terminal resolution does not require the exact effect ACK: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    '"execution.effect_plan_hash", FieldKind::Text' \
    '"execution.effect_ack_hash", FieldKind::Text'
  do
    grep -Fq "$requirement" "$ROOT/$terminal" || {
      echo "Architecture V2 terminal evidence omits provider effect binding: $requirement" >&2
      return 1
    }
  done

  hepta_v2_assert_test_inventory "Architecture V2 durable effect ACK" 4 '.*' \
    "$ROOT/$memory_regression"
  hepta_v2_assert_test_inventory "Architecture V2 provider-effect ACK" 2 \
    'architecture_v2_provider_effect_.*' "$ROOT/$runtime_regression"
  hepta_v2_assert_test_inventory "Architecture V2 native-mutation" 8 \
    'architecture_v2_native_mutation_.*' "$ROOT/$native_regression"
}

verify_release_mutation_quarantines() {
  local source="codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  local regression="codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_provider_idempotency.rs"
  local function block live_case_count

  perl -0e '
    local $/;
    my $source = <>;
    exit(
      $source =~ /#\[cfg\(test\)\]\s+RegisteredTool::DiskJunkAudit\(DiskJunkAuditTool\)/
      && $source =~ /#\[cfg\(not\(test\)\)\]\s+fn looks_like_disk_junk_audit_intent\(_input: &str\) -> bool \{.*?\bfalse\s*\}/s
        ? 0
        : 1
    );
  ' "$ROOT/$source" || {
    echo "Architecture V2 release disk-junk audit surface is not test-only" >&2
    return 1
  }

  for function in \
    native_compat_message \
    native_compat_sessions_send \
    native_compat_sessions_spawn \
    native_compat_subagents \
    native_compat_feishu
  do
    block="$(sed -n "/^fn $function(/,/^}/p" "$ROOT/$source")"
    [[ -n "$block" ]] &&
      grep -Fq 'reject_native_live_without_idempotency_receipt(tool, provider_identity)' \
        <<<"$block" || {
          echo "Architecture V2 live native mutation bypasses quarantine: $function" >&2
          return 1
        }
  done
  block="$(sed -n '/^fn native_compat_live_surface(/,/^}/p' "$ROOT/$source")"
  grep -Fq \
    '"canvas" => reject_native_live_without_idempotency_receipt(tool, provider_identity)' \
    <<<"$block" &&
    grep -Fq \
      '| "feishu_bitable_create_field" => native_compat_feishu(tool, input, provider_identity)' \
      <<<"$block" || {
        echo "Architecture V2 canvas/Feishu live mutation quarantine dispatch is incomplete" >&2
        return 1
      }
  grep -Fq \
    'live provider action remains quarantined until its adapter durably deduplicates the exact idempotency key and returns a binding receipt' \
    "$ROOT/$source" || {
      echo "Architecture V2 live native quarantine lacks an exact receipt requirement" >&2
      return 1
    }
  hepta_v2_assert_test_inventory "Architecture V2 provider idempotency" 2 \
    'architecture_v2_provider_idempotency_.*' "$ROOT/$regression"
  live_case_count="$(
    sed -n '/let cases = \[/,/^    ];/p' "$ROOT/$regression" |
      grep -Fc 'json!('
  )"
  [[ "$live_case_count" == 20 ]] || {
    echo "Architecture V2 live native mutation quarantine inventory drifted: expected 20, got $live_case_count" >&2
    return 1
  }
}

verify_cross_process_write_lock_boundary() {
  local runtime_lib="codex-rs/hepta-runtime/src/lib.rs"
  local runtime_root="codex-rs/hepta-runtime/src/runtime_kernel"
  local lock_source="$runtime_root/cross_process_write_lock.rs"
  local requirement source token minimum count

  for requirement in \
    'lib.rs|pub(super) mod cross_process_write_lock;|1' \
    'cross_process_write_lock.rs|pub(crate) struct CrossProcessWriteLease {|1' \
    'cross_process_write_lock.rs|_files: Vec<fs::File>|1' \
    'cross_process_write_lock.rs|pub(crate) fn acquire_cross_process_target_lease(|1' \
    'cross_process_write_lock.rs|hepta.runtime.cross-process-namespace.v1|1' \
    'cross_process_write_lock.rs|hepta.runtime.cross-process-anchor.v1|1' \
    'cross_process_write_lock.rs|hepta.runtime.cross-process-inode.v1|1' \
    'cross_process_write_lock.rs|hepta.runtime.cross-process-workspace.v1|1' \
    'cross_process_write_lock.rs|.custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)|1' \
    'cross_process_write_lock.rs|metadata.uid() != expected_uid|1' \
    'cross_process_write_lock.rs|metadata.nlink() != 1|1' \
    'cross_process_write_lock.rs|metadata.mode() & 0o077 != 0|2' \
    'cross_process_write_lock.rs|CrossProcessLockMode::Shared => libc::LOCK_SH|1' \
    'cross_process_write_lock.rs|CrossProcessLockMode::Exclusive => libc::LOCK_EX|1' \
    'cross_process_write_lock.rs|| libc::LOCK_NB|1' \
    'cross_process_write_lock.rs|libc::flock(file.as_raw_fd(), operation)|1' \
    'cross_process_write_lock.rs|live filesystem mutation requires supported cross-process advisory locks|1' \
    'types.rs|cross_process_lease:|1' \
    'types.rs|_cross_process_lease:|1' \
    'types.rs|Arc<runtime_kernel::cross_process_write_lock::CrossProcessWriteLease>|1' \
    'types.rs|Option<runtime_kernel::cross_process_write_lock::CrossProcessWriteLease>|1' \
    'transaction_ops.rs|acquire_cross_process_target_lease(|2' \
    'transaction_ops.rs|Arc::clone(&group_cross_process_lease)|1' \
    'transaction_ops.rs|fn cross_process_target_identity(|1'
  do
    source="${requirement%%|*}"; requirement="${requirement#*|}"
    minimum="${requirement##*|}"; token="${requirement%|*}"
    if [[ "$source" == "lib.rs" ]]; then
      count="$(grep -Fc "$token" "$ROOT/$runtime_lib" || true)"
    else
      count="$(grep -Fc "$token" "$ROOT/$runtime_root/$source" || true)"
    fi
    (( count >= minimum )) || {
      echo "Architecture V2 cross-process write lock is incomplete:" \
        "$source -> $token (expected at least $minimum, got $count)" >&2
      return 1
    }
  done

  if grep -Eq '#\[derive\([^]]*Clone[^]]*\)\][[:space:]]*$' "$ROOT/$lock_source" &&
    perl -0e '
      local $/;
      my $source = <>;
      exit($source =~ /#\[derive\([^]]*Clone[^]]*\)\]\s+pub\(crate\) struct CrossProcessWriteLease/s ? 0 : 1);
    ' "$ROOT/$lock_source"
  then
    echo "Architecture V2 cross-process write lease must remain non-cloneable" >&2
    return 1
  fi
  hepta_v2_assert_test_inventory "Architecture V2 cross-process write lock" 4 '.*' \
    "$ROOT/$lock_source"
  grep -Fq 'fn separate_process_holds_exact_identity_until_drop()' \
    "$ROOT/$lock_source" || {
      echo "Architecture V2 cross-process write lock lacks a separate-process regression" >&2
      return 1
    }
}

verify_sealed_read_boundary() {
  local runtime_root="codex-rs/hepta-runtime/src/runtime_kernel"
  local source token requirement
  for requirement in \
    'types.rs|struct PreparedReadCapability {' \
    'types.rs|anchor_directory: fs::File' \
    'types.rs|retained_file: fs::File' \
    'types.rs|content_hash: String' \
    'types.rs|bytes: Vec<u8>' \
    'execution_lease.rs|runtime.prepare_read_capability(' \
    'execution_lease.rs|prepared_read: Option<PreparedReadCapability>' \
    'execution_attempt.rs|fn prepared_read_capability(' \
    'execution_bus.rs|let prepared_read = execution.prepared_read_capability();' \
    'execution_bus.rs|prepared_read,' \
    'transaction_ops.rs|fn seal_read_capability(' \
    'transaction_ops.rs|fn ensure_single_link_read_target(' \
    'transaction_ops.rs|fn verify_namespace_unchanged(&self)' \
    'transaction_ops.rs|fn preflight_prepared_read(' \
    'transaction_ops.rs|libc::O_RDONLY | libc::O_NONBLOCK | libc::O_NOFOLLOW' \
    'tool_support.rs|invoke_prepared_read_file(prepared, identity)' \
    'tool_support.rs|invoke_prepared_native_read(' \
    'tool_support.rs|read_file requires a sealed read capability'
  do
    source="${requirement%%|*}"
    token="${requirement#*|}"
    grep -Fq "$token" "$ROOT/$runtime_root/$source" || {
      echo "Architecture V2 sealed read capability is incomplete: $source -> $token" >&2
      return 1
    }
  done

  local tool_source="$runtime_root/tool_support.rs"
  local registry_block production_tools provider_block
  registry_block="$(sed -n '/^    fn new() -> Self {/,/^    #\[cfg(test)\]/p' "$ROOT/$tool_source")"
  production_tools="$(sed -n '/^fn native_openclaw_compatible_tools()/,/^#\[cfg(test)\]/p' "$ROOT/$tool_source")"
  if grep -Eq \
    'quarantined_native_read_and_generator|new_with_all_quarantined_tools|"(list_dir|search_text|memory_search|image|pdf|image_generate|music_generate|video_generate)"|RegisteredTool::(ListDir|SearchText)' \
    <<<"$registry_block$production_tools"
  then
    echo "Architecture V2 production registry exposes an unsealed read or generator surface" >&2
    return 1
  fi
  for requirement in \
    'fn new_with_all_quarantined_tools_for_test()' \
    'fn quarantined_native_read_and_generator_tools_for_test(' \
    'architecture_v2_sealed_read_quarantines_directory_media_and_generator_surfaces'
  do
    grep -Fq "$requirement" \
      "$ROOT/$tool_source" \
      "$ROOT/$runtime_root/tests/architecture_v2_sealed_read.rs" || {
        echo "Architecture V2 sealed read quarantine regression is incomplete: $requirement" >&2
        return 1
      }
  done
  provider_block="$(
    sed -n \
      '/^fn invoke_prepared_read_file(/,/^enum NativePatchOp/p' \
      "$ROOT/$tool_source"
  )"
  [[ "$(grep -Fc 'std::str::from_utf8(&prepared.bytes)' <<<"$provider_block")" == 2 ]] || {
    echo "Architecture V2 sealed read provider does not consume captured bytes exclusively" >&2
    return 1
  }
  if grep -Eq 'fs::(read|read_to_string)|File::open' <<<"$provider_block"; then
    echo "Architecture V2 sealed read provider reopened an authorized path" >&2
    return 1
  fi
}

verify_production_durable_composition() {
  local runtime_root="codex-rs/hepta-runtime/src/runtime_kernel"
  local session_source="$runtime_root/session_ops.rs"
  local provider_source="$runtime_root/provider_support.rs"
  local sink_source="$runtime_root/outcome_sink.rs"

  perl -0e '
    local $/;
    my $source = <>;
    exit(
      $source =~ /#\[cfg\(test\)\]\s+pub fn new\(\) -> Self \{/s
      && $source =~ /```compile_fail.*RuntimeKernel::new\(\);/s
      && $source =~ /```compile_fail.*Default::default\(\);/s
        ? 0
        : 1
    );
  ' "$ROOT/$session_source" || {
    echo "Architecture V2 production runtime exposes an ephemeral constructor" >&2
    return 1
  }
  perl -0e '
    local $/;
    my $source = <>;
    exit($source =~ /\A#\[cfg\(test\)\]\s+impl Default for RuntimeKernel \{/s ? 0 : 1);
  ' "$ROOT/$provider_source" || {
    echo "Architecture V2 production runtime exposes an ephemeral Default implementation" >&2
    return 1
  }
  perl -0e '
    local $/;
    my $source = <>;
    exit(
      $source =~ /#\[cfg\(test\)\]\s+use hepta_memory::InMemoryOutcomeStore;/s
      && $source =~ /#\[cfg\(test\)\]\s+use hepta_memory::OutcomeStoreError;/s
      && $source =~ /#\[cfg\(test\)\]\s+struct InMemoryOutcomeReceiptSink/s
      && $source =~ /#\[cfg\(test\)\]\s+impl Default for InMemoryOutcomeReceiptSink/s
      && $source =~ /#\[cfg\(test\)\]\s+pub\(crate\) fn in_memory_outcome_sink\(\)/s
        ? 0
        : 1
    );
  ' "$ROOT/$sink_source" || {
    echo "Architecture V2 release build exposes an in-memory outcome sink" >&2
    return 1
  }
}

verify_durable_outcome_boundary() {
  local runtime_root="codex-rs/hepta-runtime/src/runtime_kernel"
  local memory_root="codex-rs/hepta-memory/src"
  local types_source="$runtime_root/types.rs"
  local sink_source="$runtime_root/outcome_sink.rs"
  local sink_breaker_source="$runtime_root/outcome_sink/breaker.rs"
  local session_source="$runtime_root/session_ops.rs"
  local attempt_source="$runtime_root/execution_attempt.rs"
  local recorder_source="$runtime_root/outcome_recorder.rs"
  local runtime_tests="$runtime_root/outcome_sink/tests.rs"
  local database_source="$memory_root/durable.rs"
  local database_schema_source="$memory_root/durable/schema.rs"
  local integrity_source="$memory_root/durable/integrity.rs"
  local opening_source="$memory_root/durable/opening.rs"
  local opening_filesystem_source="$memory_root/durable/opening/filesystem.rs"
  local store_source="$memory_root/outcome_store/durable.rs"
  local effect_ack_source="$memory_root/outcome_store/effect_ack.rs"
  local durable_effect_ack_source="$memory_root/outcome_store/durable/effect_ack.rs"
  local execution_intent_source="$memory_root/outcome_store/durable/execution_intent.rs"
  local terminal_evidence_source="$memory_root/outcome_store/durable/execution_intent/terminal_evidence.rs"
  local intent_source="$memory_root/outcome_store/durable/intent.rs"
  local store_tests="$memory_root/tests/outcome_store/durable.rs"
  local effect_ack_tests="$memory_root/tests/outcome_store/effect_ack.rs"
  local execution_intent_tests="$memory_root/tests/outcome_store/execution_intent.rs"
  local intent_tests="$memory_root/tests/outcome_store/pending_intent.rs"
  local writer_source="$memory_root/outcome_store/sync_writer.rs"
  local writer_intent_source="$memory_root/outcome_store/sync_writer/intent.rs"
  local writer_tests="$memory_root/tests/outcome_store/sync_writer.rs"
  local preference_source="$memory_root/preference_cas/durable.rs"
  local preference_tests="$memory_root/tests/preference_cas/durable_opening.rs"
  local preference_security_tests="$memory_root/tests/preference_cas/durable_opening_security.rs"
  local source requirement trait_block opening_existing_block opening_reserve_block count

  for source in \
    "$types_source" "$sink_source" "$session_source" "$attempt_source" \
    "$recorder_source" "$runtime_tests" "$database_source" "$database_schema_source" \
    "$integrity_source" "$opening_source" "$opening_filesystem_source" "$store_source" \
    "$execution_intent_source" "$sink_breaker_source" \
    "$effect_ack_source" "$durable_effect_ack_source" "$terminal_evidence_source" \
    "$intent_source" "$store_tests" "$effect_ack_tests" "$execution_intent_tests" "$intent_tests" \
    "$writer_source" "$writer_intent_source" "$writer_tests" \
    "$preference_source" "$preference_tests" "$preference_security_tests"
  do
    [[ -f "$ROOT/$source" ]] || {
      echo "Architecture V2 durable outcome source is missing: $source" >&2
      return 1
    }
  done

  if grep -Fq 'InMemoryOutcomeStore' "$ROOT/$types_source"; then
    echo "Architecture V2 RuntimeKernel outcome field is fixed to InMemoryOutcomeStore" >&2
    return 1
  fi
  grep -Fq \
    'outcome_sink: runtime_kernel::outcome_sink::SharedOutcomeReceiptSink' \
    "$ROOT/$types_source" || {
      echo "Architecture V2 RuntimeKernel must store SharedOutcomeReceiptSink" >&2
      return 1
    }

  trait_block="$(
    sed -n \
      '/pub(crate) trait OutcomeReceiptSink: Send + Sync {/,/^}/p' \
      "$ROOT/$sink_source"
  )"
  for requirement in \
    'pub(crate) trait OutcomeReceiptSink: Send + Sync {' \
    'fn record(' \
    '&self,' \
    'exact: &ExactOutcomeRecord' \
    'fn read_by_attempt(' \
    'attempt_id: &str'
  do
    grep -Fq "$requirement" <<<"$trait_block" || {
      echo "Architecture V2 object-safe outcome sink record/read boundary is incomplete: $requirement" >&2
      return 1
    }
  done

  for requirement in \
    'pub fn bootstrap_with_durable_outcomes(' \
    'pub fn open_with_durable_outcomes(' \
    'integrity_key: hepta_memory::DurableIntegrityKey' \
    'bootstrap_new_durable_outcome_sink(' \
    'open_existing_durable_outcome_sink(' \
    'Result<Self, hepta_memory::DurableOutcomeWriterError>' \
    'This fails if the target already exists' \
    'This constructor never creates a file or falls back to memory.'
  do
    grep -Fq "$requirement" "$ROOT/$session_source" || {
      echo "Architecture V2 fallible durable outcome constructor is incomplete: $requirement" >&2
      return 1
    }
  done

  for requirement in \
    'pub struct DurableIntegrityKey' \
    'type HmacSha256 = Hmac<Sha256>' \
    'hepta.memory.durable-integrity.row-mac.v1' \
    'const KEYED_SCHEMA_VERSION: i64 = 5' \
    'hmac-sha256-v1' \
    'hmac-sha256:'
  do
    grep -Fq "$requirement" "$ROOT/$integrity_source" || {
      echo "Architecture V2 keyed durable integrity boundary is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'CREATE TABLE IF NOT EXISTS hepta_v2_integrity' \
    'verify_integrity_binding'
  do
    grep -Fq "$requirement" "$ROOT/$database_schema_source" || {
      echo "Architecture V2 keyed durable database binding is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'self.integrity.protect(&payload_json)?' \
    '.verify(payload_json, expected_storage_hash, row_kind)?'
  do
    grep -Fq "$requirement" "$ROOT/$database_source" || {
      echo "Architecture V2 keyed durable database binding is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'SyncDurableOutcomeWriter::bootstrap_new_keyed' \
    'SyncDurableOutcomeWriter::open_existing_keyed'
  do
    grep -Fq "$requirement" "$ROOT/$sink_source" || {
      echo "Architecture V2 production outcome composition is not keyed: $requirement" >&2
      return 1
    }
  done
  if grep -Eq 'SyncDurableOutcomeWriter::(bootstrap_new|open_existing)\(' "$ROOT/$sink_source"; then
    echo "Architecture V2 production outcome composition downgraded to unkeyed compatibility storage" >&2
    return 1
  fi

  for requirement in \
    'pub(crate) struct ExactOutcomeRecord' \
    'pub(crate) fn record_first_outcome(' \
    'PendingOutcomeKind::SafeRetry' \
    'PendingOutcomeKind::CommitAmbiguous' \
    'reconciliation_in_flight' \
    'pub fn reconcile_pending_outcome(' \
    'fn pending_intent(' \
    'fn first_pending_intent(' \
    'pub(super) fn durable_pending_outcome_reason(' \
    'DurableOutcomeWriterError::WorkerUnavailable' \
    'DurableOutcomeWriterError::AcknowledgementTimeout' \
    'DurableOutcomeWriterError::CommitAmbiguous' \
    'pub(crate) fn bootstrap_new_durable_outcome_sink(' \
    'pub(crate) fn open_existing_durable_outcome_sink(' \
    '.reopen_existing_bound()'
  do
    grep -Fq "$requirement" "$ROOT/$sink_source" "$ROOT/$sink_breaker_source" || {
      echo "Architecture V2 record-first exact outcome reconciliation is incomplete: $requirement" >&2
      return 1
    }
  done
  grep -Fq 'record_first_outcome(' "$ROOT/$recorder_source" || {
    echo "Architecture V2 outcome recorder bypasses record-first exact persistence" >&2
    return 1
  }
  grep -Fq '.outcome_sink.read_by_attempt' "$ROOT/$attempt_source" || {
    echo "Architecture V2 execution attempt reservation bypasses durable outcome reads" >&2
    return 1
  }
  grep -Fq 'self.durable_pending_outcome_reason()?' "$ROOT/$attempt_source" || {
    echo "Architecture V2 execution admission bypasses unresolved durable outcome intents" >&2
    return 1
  }
  if grep -Fq 'does not provide a cross-process pending-intent journal' \
    "$ROOT/$sink_source"
  then
    echo "Architecture V2 runtime still claims durable pending intents are process-local" >&2
    return 1
  fi

  grep -Fq 'CREATE TABLE IF NOT EXISTS hepta_v2_outcome_intents' \
    "$ROOT/$database_schema_source" || {
      echo "Architecture V2 durable producer-intent journal schema is missing" >&2
      return 1
    }
  for requirement in \
    'pub async fn stage_intent(' \
    'pub(crate) async fn commit_staged_intent(' \
    'pub(crate) async fn acknowledge_intent(' \
    'pub async fn pending_intents(' \
    'pub async fn pending_intent(' \
    'verify_intent_recovery' \
    'fn post_commit_error(' \
    '.map_err(|error| post_commit_error(error, operation))?'
  do
    grep -Fq "$requirement" "$ROOT/$intent_source" || {
      echo "Architecture V2 durable producer-intent journal is incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'pub fn stage_intent(' \
    'pub fn pending_intents(' \
    'pub fn pending_intent(' \
    'pub(super) fn acknowledge_intent('
  do
    grep -Fq "$requirement" "$ROOT/$writer_intent_source" || {
      echo "Architecture V2 synchronous producer-intent boundary is incomplete: $requirement" >&2
      return 1
    }
  done

  for requirement in \
    'pub(crate) async fn bootstrap_new(' \
    'pub(crate) async fn open_existing(' \
    'pub(crate) async fn bootstrap_new_with_integrity(' \
    'pub(crate) async fn open_existing_with_integrity(' \
    'pub(crate) async fn open_existing_bound_with_integrity(' \
    'let pool = connect_pool(&path, false).await?' \
    'durable database path was deleted or replaced'
  do
    grep -Fq "$requirement" "$ROOT/$opening_source" || {
      echo "Architecture V2 durable database open modes are incomplete: $requirement" >&2
      return 1
    }
  done
  for requirement in \
    'const PRIVATE_FILE_MODE: u32 = 0o600' \
    'const PRIVATE_DIRECTORY_MODE: u32 = 0o700' \
    'const MAX_SIDECAR_VALIDATION_ATTEMPTS: usize = 4' \
    'enum SidecarValidationProgress {' \
    'recheck_unlinked_sidecar_namespace(' \
    '.custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)' \
    'metadata.uid() != expected_uid' \
    'metadata.nlink() != 1' \
    'validate_existing_sidecars('
  do
    grep -Fq "$requirement" "$ROOT/$opening_filesystem_source" || {
      echo "Architecture V2 durable database filesystem opening is incomplete: $requirement" >&2
      return 1
    }
  done
  opening_reserve_block="$(
    sed -n \
      '/^pub(super) fn reserve_new_database_file(/,/^#\[cfg(not(unix))\]/p' \
      "$ROOT/$opening_filesystem_source"
  )"
  for requirement in \
    '.create_new(true)' \
    '.mode(PRIVATE_FILE_MODE)' \
    '.custom_flags(libc::O_CLOEXEC | libc::O_NOFOLLOW)'
  do
    grep -Fq "$requirement" <<<"$opening_reserve_block" || {
      echo "Architecture V2 durable database exclusive reservation is incomplete: $requirement" >&2
      return 1
    }
  done
  opening_existing_block="$(
    sed -n \
      '/pub(crate) async fn open_existing_with_integrity(/,/pub(crate) async fn open_existing_bound_with_integrity/p' \
      "$ROOT/$opening_source"
  )"
  if grep -Eq 'create_dir_all|create_if_missing\(true\)|CREATE (TABLE|INDEX)' \
    <<<"$opening_existing_block"
  then
    echo "Architecture V2 open-existing path may create or migrate durable storage" >&2
    return 1
  fi

  for requirement in \
    'pub async fn bootstrap_new(' \
    'pub async fn open_existing(' \
    'pub async fn bootstrap_new_keyed(' \
    'pub async fn open_existing_keyed(' \
    'DurableDatabase::bootstrap_new_with_integrity(path, integrity)' \
    'DurableDatabase::open_existing_with_integrity(path, integrity)' \
    'validate_database_identity()'
  do
    grep -Fq "$requirement" "$ROOT/$store_source" || {
      echo "Architecture V2 durable outcome store open/identity boundary is incomplete: $requirement" >&2
      return 1
    }
  done
  if grep -Fq 'open_or_create_legacy' \
    "$ROOT/$opening_source" "$ROOT/$store_source" "$ROOT/$writer_source" \
    "$ROOT/$preference_source" "$ROOT/$sink_source" "$ROOT/$session_source"
  then
    echo "Architecture V2 production outcome path reaches legacy open-or-create storage" >&2
    return 1
  fi
  if grep -Fq 'open_durable_outcome_sink(' "$ROOT/$sink_source" "$ROOT/$session_source"; then
    echo "Architecture V2 runtime outcome path exposes an ambiguous durable open adapter" >&2
    return 1
  fi

  for requirement in \
    'pub fn bootstrap_new(' \
    'pub fn open_existing(' \
    'pub fn bootstrap_new_keyed(' \
    'pub fn open_existing_keyed(' \
    'pub fn reopen_existing_bound(' \
    'DurableIntegrityContext' \
    'WriterOpenMode::OpenExistingBound' \
    'pub fn record(' \
    'pub fn read_by_attempt(' \
    'DurableOutcomeWriterError::PendingIntent' \
    'DurableOutcomeWriterError::QueueFull' \
    'DurableOutcomeWriterError::WorkerUnavailable' \
    'DurableOutcomeWriterError::AcknowledgementTimeout' \
    'DurableOutcomeWriterError::CommitAmbiguous'
  do
    grep -Fq "$requirement" "$ROOT/$writer_source" || {
      echo "Architecture V2 synchronous durable outcome writer is incomplete: $requirement" >&2
      return 1
    }
  done
  if grep -Fq 'pub fn open(' "$ROOT/$writer_source"; then
    echo "Architecture V2 synchronous outcome writer exposes an ambiguous open API" >&2
    return 1
  fi

  hepta_v2_assert_test_inventory "Architecture V2 runtime outcome sink" 19 '.*' \
    "$ROOT/$runtime_tests"
  hepta_v2_assert_test_inventory "Architecture V2 durable outcome store" 29 '.*' \
    "$ROOT/$store_tests" \
    "$ROOT/$effect_ack_tests" \
    "$ROOT/$execution_intent_tests" \
    "$ROOT/$intent_tests"
  hepta_v2_assert_test_inventory "Architecture V2 durable effect ACK" 4 '.*' \
    "$ROOT/$effect_ack_tests"
  hepta_v2_assert_test_inventory "Architecture V2 durable execution intent" 9 '.*' \
    "$ROOT/$execution_intent_tests"
  hepta_v2_assert_test_inventory "Architecture V2 sync durable outcome writer" 13 '.*' \
    "$ROOT/$writer_tests"

  for requirement in \
    'pub async fn bootstrap_new(' \
    'pub async fn open_existing(' \
    'pub async fn bootstrap_new_keyed(' \
    'pub async fn open_existing_keyed(' \
    'DurableDatabase::bootstrap_new(path)' \
    'DurableDatabase::open_existing(path)' \
    'validate_database_identity()'
  do
    grep -Fq "$requirement" "$ROOT/$preference_source" || {
      echo "Architecture V2 durable preference open boundary is incomplete: $requirement" >&2
      return 1
    }
  done
  if grep -Fq 'pub async fn open(' "$ROOT/$preference_source"; then
    echo "Architecture V2 durable preference store exposes an ambiguous open API" >&2
    return 1
  fi
  hepta_v2_assert_test_inventory "Architecture V2 durable preference opening" 4 '.*' \
    "$ROOT/$preference_tests"
  hepta_v2_assert_test_inventory "Architecture V2 durable opening security" 7 '.*' \
    "$ROOT/$preference_security_tests"
  hepta_v2_assert_test_inventory "Architecture V2 durable sidecar lifecycle" 2 \
    'unlinked_open_sidecar_.*' "$ROOT/$opening_filesystem_source"
}

verify() {
  require_tool grep
  require_tool perl
  require_tool python3
  require_tool rg

  local contracts_manifest="codex-rs/hepta-contracts/Cargo.toml"
  [[ -f "$ROOT/$contracts_manifest" ]] || {
    echo "Architecture V2 contract boundary is missing: $contracts_manifest" >&2
    return 1
  }
  workspace_has_contract_boundary || {
    echo "Architecture V2 contract boundary is not registered in the Cargo workspace" >&2
    return 1
  }

  local contract_dependencies dependency
  if ! contract_dependencies="$(package_dependency_names "$contracts_manifest")"; then
    echo "Architecture V2 could not parse dependencies: $contracts_manifest" >&2
    return 1
  fi
  while IFS= read -r dependency; do
    if [[ -n "$dependency" ]]; then
      echo "Architecture V2 contracts must remain dependency-free: $dependency" >&2
      return 1
    fi
  done <<<"$contract_dependencies"

  require_dependency "codex-rs/hepta-core/Cargo.toml" "hepta-contracts"
  require_dependency "codex-rs/hepta-intelligence/Cargo.toml" "hepta-contracts"
  require_dependency "codex-rs/hepta-kernel/Cargo.toml" "hepta-contracts"
  require_dependency "codex-rs/hepta-memory/Cargo.toml" "hepta-contracts"
  require_dependency "codex-rs/hepta-runtime/Cargo.toml" "hepta-contracts"
  local compatibility_source="codex-rs/hepta-core/src/hepta_contracts.rs"
  if ! grep -Fqx 'pub use ::hepta_contracts::*;' "$ROOT/$compatibility_source"; then
    echo "Architecture V2 compatibility re-export missing: $compatibility_source" >&2
    return 1
  fi

  deny_dependencies "codex-rs/hepta-core/Cargo.toml" \
    hepta-intelligence hepta-kernel hepta-memory hepta-kg hepta-plugins \
    hepta-runtime hepta-gateway hepta-native-gateway hepta-cli
  deny_dependencies "codex-rs/hepta-memory/Cargo.toml" \
    hepta-intelligence hepta-kernel hepta-plugins hepta-runtime \
    hepta-gateway hepta-native-gateway hepta-cli
  deny_dependencies "codex-rs/hepta-kg/Cargo.toml" \
    hepta-intelligence hepta-kernel hepta-memory hepta-plugins \
    hepta-runtime hepta-gateway hepta-native-gateway hepta-cli
  deny_dependencies "codex-rs/hepta-intelligence/Cargo.toml" \
    hepta-kernel hepta-plugins hepta-runtime hepta-gateway \
    hepta-native-gateway hepta-cli
  deny_dependencies "codex-rs/hepta-kernel/Cargo.toml" \
    hepta-intelligence hepta-memory hepta-kg hepta-plugins \
    hepta-runtime hepta-gateway hepta-native-gateway hepta-cli
  deny_dependencies "codex-rs/hepta-plugins/Cargo.toml" \
    hepta-intelligence hepta-kernel hepta-memory hepta-runtime \
    hepta-gateway hepta-native-gateway hepta-cli
  deny_dependencies "codex-rs/hepta-runtime/Cargo.toml" \
    hepta-plugins

  verify_non_live_preference_authority
  verify_trusted_preference_feedback_boundary
  verify_intuition_feedback_ownership
  verify_intuition_planner_ownership
  verify_capability_manifest_ownership
  verify_exact_admission_boundary
  verify_sealed_execution_boundaries
  verify_v2_test_inventories
  verify_quarantined_native_process_surface
  verify_backup_prune_mutation_boundary
  verify_mutation_transaction_evidence_boundary
  verify_mutation_durability_boundary
  verify_execution_intent_boundary
  verify_provider_effect_boundary
  verify_release_mutation_quarantines
  verify_cross_process_write_lock_boundary
  verify_sealed_read_boundary
  verify_production_durable_composition
  verify_durable_outcome_boundary

  echo '{"schema":"hepta_architecture_v2_dependency_boundary_v1","status":"ready","contract_boundary":"hepta-contracts","contract_dependencies":0,"compatibility_shell":"hepta-core","forbidden_reverse_edges":46,"live_preference_authority":"unattached","trusted_preference_feedback_authority":"keyed-composed-transport-unattached","trusted_preference_transport":"unattached","trusted_preference_authority_composition":"keyed-durable-pinned-source","legacy_intuition_feedback_owner":"hepta-intelligence","legacy_intuition_planner_owner":"hepta-intelligence","capability_manifest_owner":"runtime-catalog-adapter","exact_admission_owner":"hepta-kernel","runtime_self_admission":"forbidden","identity_sealed_write_path":"retained-openat","process_write_reservation":"identity-global","cross_process_write_reservation":"advisory-prefix-identity-lock","exact_dispatch_selector":"executor-capability","production_tool_descriptor_inventory":41,"test_tool_descriptor_inventory":42,"production_exec_process":"quarantined","quarantined_process_descriptor":"high-non-read-only-destructive","production_disk_junk_audit":"test-only","backup_prune_deletion":"release-quarantined","mutation_transaction_evidence":"generalized-set","mutation_install_durability":"atomic-ambiguous-effect-recorded","provider_effect_plan":"pre-dispatch-canonical","provider_effect_ack":"durable-exact-before-terminal","provider_effect_recovery":"read-only-fail-closed","live_tts_effect":"private-byte-stage-before-intent-durable-ack","sealed_read_capability":"retained-fd-captured-bytes","production_unsealed_reads":"quarantined","production_live_native_mutations":"exact-receipt-quarantined","production_live_native_mutation_inventory":20,"production_outcome_composition":"durable-keyed-only","durable_schema":5,"durable_compatibility_schema":4,"durable_row_integrity":"hmac-sha256-v1","durable_integrity_key":"external-caller-supplied","durable_rollback_resistance":"external-monotonic-anchor-required","execution_intent_stage":"before-provider-invocation","execution_intent_terminal_resolution":"atomic-with-outcome-record","execution_intent_idempotency_domain":"v3","execution_intent_row_schema":4,"strict_terminal_evidence_field_inventory":80,"durable_sidecar_validation":"bounded-identity-recheck","durable_sidecar_validation_attempts":4,"test_inventory_binding":"test-attribute-plus-target-function","contracts_test_inventory":11,"intelligence_neuron_test_inventory":4,"intelligence_tool_candidate_test_inventory":4,"intelligence_feedback_test_inventory":4,"intelligence_planner_test_inventory":8,"intelligence_preference_test_inventory":8,"intelligence_trusted_preference_test_inventory":5,"memory_preference_authority_test_inventory":6,"memory_preference_test_inventory":35,"memory_durable_preference_test_inventory":19,"memory_durable_opening_security_test_inventory":7,"memory_durable_sidecar_lifecycle_test_inventory":2,"memory_outcome_test_inventory":50,"memory_durable_outcome_test_inventory":29,"memory_effect_ack_test_inventory":4,"memory_execution_intent_test_inventory":9,"memory_pending_outcome_intent_test_inventory":3,"memory_sync_outcome_writer_test_inventory":13,"runtime_neuron_test_inventory":8,"runtime_exact_safety_test_inventory":10,"runtime_execution_lease_test_inventory":5,"runtime_outcome_receipt_test_inventory":4,"runtime_outcome_flow_test_inventory":8,"runtime_outcome_sink_test_inventory":19,"runtime_provider_idempotency_test_inventory":2,"runtime_provider_effect_test_inventory":2,"runtime_resource_reservation_test_inventory":4,"runtime_capability_descriptor_test_inventory":4,"runtime_symlink_reservation_test_inventory":4,"runtime_process_reservation_test_inventory":8,"runtime_dispatch_selector_test_inventory":3,"runtime_native_mutation_test_inventory":8,"runtime_sealed_read_test_inventory":9,"runtime_cross_process_write_lock_test_inventory":4,"runtime_process_control_test_inventory":2,"runtime_maintenance_test_inventory":7,"durable_outcome_sink":"keyed-producer-intent-journal-exact-reconciliation","durable_outcome_database_open":"keyed-bootstrap-new-or-identity-bound-existing-private-filesystem","durable_preference_database_open":"bootstrap-new-or-existing-private-filesystem-keyed-capable"}'
}

expect_fixture_denied() {
  local fixture="$1"
  local expected_diagnostic="${2:-}"
  local output
  if output="$(HEPTA_V2_ARCH_ROOT="$fixture" "$0" verify 2>&1)"; then
    echo "Architecture V2 boundary accepted a drifted fixture: $fixture" >&2
    return 1
  fi
  if [[ -n "$expected_diagnostic" ]] && ! grep -Fq "$expected_diagnostic" <<<"$output"; then
    echo "Architecture V2 boundary emitted the wrong denial: $output" >&2
    return 1
  fi
}

self_test() (
  verify >/dev/null

  local tmp fixture manifest regression drift source original replacement
  tmp="$(mktemp -d "${TMPDIR:-/tmp}/hepta-v2-architecture.XXXXXX")"
  trap 'rm -rf "$tmp"' EXIT
  fixture="$tmp/fixture"
  mkdir -p "$fixture/codex-rs" "$fixture/scripts/lib"
  cp "$ROOT/scripts/lib/hepta-v2-test-inventory.sh" \
    "$fixture/scripts/lib/hepta-v2-test-inventory.sh"
  cp "$ROOT/codex-rs/Cargo.toml" "$fixture/codex-rs/Cargo.toml"
  for manifest in \
    hepta-contracts \
    hepta-core \
    hepta-memory \
    hepta-kg \
    hepta-intelligence \
    hepta-kernel \
    hepta-plugins \
    hepta-runtime
  do
    mkdir -p "$fixture/codex-rs/$manifest"
    cp "$ROOT/codex-rs/$manifest/Cargo.toml" "$fixture/codex-rs/$manifest/Cargo.toml"
  done
  mkdir -p "$fixture/codex-rs/hepta-core/src"
  cp "$ROOT/codex-rs/hepta-core/src/hepta_contracts.rs" \
    "$fixture/codex-rs/hepta-core/src/hepta_contracts.rs"
  mkdir -p "$fixture/codex-rs/hepta-contracts/tests"
  cp "$ROOT/codex-rs/hepta-contracts/tests/stable_contracts.rs" \
    "$fixture/codex-rs/hepta-contracts/tests/stable_contracts.rs"
  for source_root in hepta-runtime hepta-gateway hepta-native-gateway hepta-cli hepta-kernel
  do
    mkdir -p "$fixture/codex-rs/$source_root/src"
    : >"$fixture/codex-rs/$source_root/src/lib.rs"
  done
  cp "$ROOT/codex-rs/hepta-runtime/src/lib.rs" \
    "$fixture/codex-rs/hepta-runtime/src/lib.rs"
  mkdir -p "$fixture/codex-rs/hepta-runtime/src/runtime_kernel"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tests.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tests.rs"
  for source in \
    context_freezer.rs approval_state.rs cross_process_write_lock.rs execution_attempt.rs execution_bus.rs execution_lease.rs \
    outcome_recorder.rs outcome_sink.rs provider_effect.rs provider_support.rs safety_gate_client.rs session_ops.rs \
    tool_support.rs transaction_ops.rs types.rs
  do
    cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/$source" \
      "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/$source"
  done
  mkdir -p "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/safety_gate_client"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/safety_gate_client/admission.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/safety_gate_client/admission.rs"
  mkdir -p "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tests"
  for regression in \
    capability_descriptor dispatch_selector exact_safety execution_lease \
    maintenance_mutation native_mutation outcome_flow process_control sealed_read \
    process_reservation provider_effect provider_idempotency resource_reservation symlink_reservation \
    terminal_outcome
  do
    cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_${regression}.rs" "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tests/"
  done
  mkdir -p "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink/breaker.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink/breaker.rs"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink/tests.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink/tests.rs"
  mkdir -p \
    "$fixture/codex-rs/hepta-memory/src/durable/opening" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/execution_intent" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/sync_writer" \
    "$fixture/codex-rs/hepta-memory/src/preference_cas" \
    "$fixture/codex-rs/hepta-memory/src/preference_authority" \
    "$fixture/codex-rs/hepta-memory/src/tests/outcome_store" \
    "$fixture/codex-rs/hepta-memory/src/tests/preference_cas"
  cp "$ROOT/codex-rs/hepta-memory/src/durable.rs" \
    "$fixture/codex-rs/hepta-memory/src/durable.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/durable/schema.rs" \
    "$fixture/codex-rs/hepta-memory/src/durable/schema.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/durable/integrity.rs" \
    "$fixture/codex-rs/hepta-memory/src/durable/integrity.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/durable/opening.rs" \
    "$fixture/codex-rs/hepta-memory/src/durable/opening.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/durable/opening/filesystem.rs" \
    "$fixture/codex-rs/hepta-memory/src/durable/opening/filesystem.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/execution_intent.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/execution_intent.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/effect_ack.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/effect_ack.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/durable.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/durable/execution_intent.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/execution_intent.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/durable/effect_ack.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/effect_ack.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/durable/execution_intent/terminal_evidence.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/execution_intent/terminal_evidence.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/durable/intent.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/intent.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/sync_writer.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/sync_writer.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/sync_writer/intent.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/sync_writer/intent.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/tests/outcome_store.rs" \
    "$fixture/codex-rs/hepta-memory/src/tests/outcome_store.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/tests/outcome_store/durable.rs" \
    "$fixture/codex-rs/hepta-memory/src/tests/outcome_store/durable.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/tests/outcome_store/effect_ack.rs" \
    "$fixture/codex-rs/hepta-memory/src/tests/outcome_store/effect_ack.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/tests/outcome_store/execution_intent.rs" \
    "$fixture/codex-rs/hepta-memory/src/tests/outcome_store/execution_intent.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/tests/outcome_store/pending_intent.rs" \
    "$fixture/codex-rs/hepta-memory/src/tests/outcome_store/pending_intent.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/tests/outcome_store/sync_writer.rs" \
    "$fixture/codex-rs/hepta-memory/src/tests/outcome_store/sync_writer.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/preference_cas/durable.rs" \
    "$fixture/codex-rs/hepta-memory/src/preference_cas/durable.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/preference_authority.rs" \
    "$fixture/codex-rs/hepta-memory/src/preference_authority.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/preference_cas/preference_authority_canonical.rs" \
    "$fixture/codex-rs/hepta-memory/src/preference_cas/preference_authority_canonical.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/preference_cas/preference_authority_types.rs" \
    "$fixture/codex-rs/hepta-memory/src/preference_cas/preference_authority_types.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/preference_authority/tests.rs" \
    "$fixture/codex-rs/hepta-memory/src/preference_authority/tests.rs"
  cp "$ROOT/codex-rs/hepta-memory/src/tests/preference_cas.rs" \
    "$fixture/codex-rs/hepta-memory/src/tests/preference_cas.rs"
  for regression in \
    document durable durable_concurrency durable_opening durable_opening_security fixtures legacy
  do
    cp "$ROOT/codex-rs/hepta-memory/src/tests/preference_cas/$regression.rs" \
      "$fixture/codex-rs/hepta-memory/src/tests/preference_cas/$regression.rs"
  done
  mkdir -p "$fixture/codex-rs/hepta-kernel/src/safety_gate"
  cp "$ROOT/codex-rs/hepta-kernel/src/safety_gate/policy_evidence.rs" \
    "$fixture/codex-rs/hepta-kernel/src/safety_gate/policy_evidence.rs"
  cp "$ROOT/codex-rs/hepta-kernel/src/safety_gate/admission.rs" \
    "$fixture/codex-rs/hepta-kernel/src/safety_gate/admission.rs"
  cp "$ROOT/codex-rs/hepta-kernel/src/safety_gate/tests.rs" \
    "$fixture/codex-rs/hepta-kernel/src/safety_gate/tests.rs"
  cp "$ROOT/codex-rs/hepta-kernel/src/safety_gate/admission_tests.rs" \
    "$fixture/codex-rs/hepta-kernel/src/safety_gate/admission_tests.rs"
  cp "$ROOT/codex-rs/hepta-runtime/src/query.rs" \
    "$fixture/codex-rs/hepta-runtime/src/query.rs"
  mkdir -p "$fixture/codex-rs/hepta-runtime/src/query"
  cp "$ROOT/codex-rs/hepta-runtime/src/query/tests.rs" \
    "$fixture/codex-rs/hepta-runtime/src/query/tests.rs"
  mkdir -p \
    "$fixture/codex-rs/hepta-intelligence/src/neuron_activation" \
    "$fixture/codex-rs/hepta-intelligence/src/intuition_feedback_learning" \
    "$fixture/codex-rs/hepta-intelligence/src/intuition_planner" \
    "$fixture/codex-rs/hepta-intelligence/src/preference_feedback" \
    "$fixture/codex-rs/hepta-intelligence/src/trusted_preference_feedback"
  cp "$ROOT/codex-rs/hepta-intelligence/src/neuron_activation/tests.rs" \
    "$fixture/codex-rs/hepta-intelligence/src/neuron_activation/tests.rs"
  cp "$ROOT/codex-rs/hepta-intelligence/src/tool_candidate.rs" \
    "$fixture/codex-rs/hepta-intelligence/src/tool_candidate.rs"
  cp "$ROOT/codex-rs/hepta-intelligence/src/intuition_feedback_learning.rs" \
    "$fixture/codex-rs/hepta-intelligence/src/intuition_feedback_learning.rs"
  cp "$ROOT/codex-rs/hepta-intelligence/src/intuition_feedback_learning/tests.rs" \
    "$fixture/codex-rs/hepta-intelligence/src/intuition_feedback_learning/tests.rs"
  cp "$ROOT/codex-rs/hepta-intelligence/src/intuition_planner.rs" \
    "$fixture/codex-rs/hepta-intelligence/src/intuition_planner.rs"
  cp "$ROOT/codex-rs/hepta-intelligence/src/intuition_planner/tests.rs" \
    "$fixture/codex-rs/hepta-intelligence/src/intuition_planner/tests.rs"
  cp "$ROOT/codex-rs/hepta-intelligence/src/preference_feedback/tests.rs" \
    "$fixture/codex-rs/hepta-intelligence/src/preference_feedback/tests.rs"
  cp "$ROOT/codex-rs/hepta-intelligence/src/trusted_preference_feedback.rs" \
    "$fixture/codex-rs/hepta-intelligence/src/trusted_preference_feedback.rs"
  cp "$ROOT/codex-rs/hepta-intelligence/src/trusted_preference_feedback/tests.rs" \
    "$fixture/codex-rs/hepta-intelligence/src/trusted_preference_feedback/tests.rs"

  HEPTA_V2_ARCH_ROOT="$fixture" "$0" verify >/dev/null

  perl -0pi -e 's/^hepta-contracts\.workspace = true\n//m' \
    "$fixture/codex-rs/hepta-kernel/Cargo.toml"
  expect_fixture_denied "$fixture" "Architecture V2 required dependency missing"
  cp "$ROOT/codex-rs/hepta-kernel/Cargo.toml" \
    "$fixture/codex-rs/hepta-kernel/Cargo.toml"

  : >"$fixture/codex-rs/hepta-core/src/hepta_contracts.rs"
  expect_fixture_denied "$fixture" "Architecture V2 compatibility re-export missing"
  cp "$ROOT/codex-rs/hepta-core/src/hepta_contracts.rs" \
    "$fixture/codex-rs/hepta-core/src/hepta_contracts.rs"

  printf '\nhepta-core.workspace = true\n' \
    >>"$fixture/codex-rs/hepta-contracts/Cargo.toml"
  expect_fixture_denied "$fixture"

  cp "$ROOT/codex-rs/hepta-contracts/Cargo.toml" \
    "$fixture/codex-rs/hepta-contracts/Cargo.toml"
  printf '\nhepta-runtime.workspace = true\n' \
    >>"$fixture/codex-rs/hepta-intelligence/Cargo.toml"
  expect_fixture_denied "$fixture"

  cp "$ROOT/codex-rs/hepta-intelligence/Cargo.toml" \
    "$fixture/codex-rs/hepta-intelligence/Cargo.toml"
  printf '\nhepta-intelligence.workspace = true\n' \
    >>"$fixture/codex-rs/hepta-kernel/Cargo.toml"
  expect_fixture_denied "$fixture"

  cp "$ROOT/codex-rs/hepta-kernel/Cargo.toml" \
    "$fixture/codex-rs/hepta-kernel/Cargo.toml"
  python3 - "$fixture/codex-rs/Cargo.toml" <<'PY'
from pathlib import Path
import sys

manifest = Path(sys.argv[1])
contents = manifest.read_text(encoding="utf-8")
needle = 'hepta-intelligence = { path = "hepta-intelligence" }\n'
manifest.write_text(
    contents.replace(
        needle,
        needle + 'v2-brain = { package = "hepta-intelligence", path = "hepta-intelligence" }\n',
        1,
    ),
    encoding="utf-8",
)
PY
  printf '\nv2-brain.workspace = true\n' \
    >>"$fixture/codex-rs/hepta-kernel/Cargo.toml"
  expect_fixture_denied "$fixture"

  cp "$ROOT/codex-rs/hepta-kernel/Cargo.toml" \
    "$fixture/codex-rs/hepta-kernel/Cargo.toml"
  printf '\nhepta-plugins.workspace = true\n' \
    >>"$fixture/codex-rs/hepta-runtime/Cargo.toml"
  expect_fixture_denied "$fixture"

  cp "$ROOT/codex-rs/hepta-runtime/Cargo.toml" \
    "$fixture/codex-rs/hepta-runtime/Cargo.toml"
  printf '\n[dependencies]\n' >>"$fixture/codex-rs/hepta-memory/Cargo.toml"
  expect_fixture_denied "$fixture" \
    "Architecture V2 could not parse dependencies: codex-rs/hepta-memory/Cargo.toml"

  cp "$ROOT/codex-rs/hepta-memory/Cargo.toml" \
    "$fixture/codex-rs/hepta-memory/Cargo.toml"
  printf '\nuse hepta_contracts::PreferenceTransition;\n' \
    >>"$fixture/codex-rs/hepta-gateway/src/lib.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 non-live preference authority consumer detected"

  : >"$fixture/codex-rs/hepta-gateway/src/lib.rs"
  perl -0pi -e 's/preference:unattached/preference:attached/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/context_freezer.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 runtime preference must remain explicitly unattached"

  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/context_freezer.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/context_freezer.rs"
  printf '\nconst DRIFTED_FEEDBACK_RULE: &str = "feedback-learning:";\n' \
    >>"$fixture/codex-rs/hepta-runtime/src/query.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 runtime-owned intuition feedback cognition detected"

  cp "$ROOT/codex-rs/hepta-runtime/src/query.rs" \
    "$fixture/codex-rs/hepta-runtime/src/query.rs"
  printf '\nfn build_bootstrap_workflow_priors() {}\n' \
    >>"$fixture/codex-rs/hepta-runtime/src/query.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 runtime-owned intuition planner cognition detected"

  cp "$ROOT/codex-rs/hepta-runtime/src/query.rs" \
    "$fixture/codex-rs/hepta-runtime/src/query.rs"
  printf '\n// CapabilityManifestRef::new drift fixture.\n' \
    >>"$fixture/codex-rs/hepta-runtime/src/runtime_kernel/context_freezer.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 runtime synthesized a manifest reference without a descriptor"

  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/context_freezer.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/context_freezer.rs"
  perl -0pi -e 's/HeptaKernelPolicyEvidence::new\(/HeptaKernelPolicyEvidence::from_unreplayed\(/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/safety_gate_client/admission.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 runtime exact-admission adapter is incomplete"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/safety_gate_client/admission.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/safety_gate_client/admission.rs"

  printf '\nconst DRIFTED_ADMISSION: hepta_contracts::AdmissionDecision = hepta_contracts::AdmissionDecision::Admitted;\n' \
    >>"$fixture/codex-rs/hepta-runtime/src/runtime_kernel/safety_gate_client/admission.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 runtime minted an admitted candidate outside the kernel"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/safety_gate_client/admission.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/safety_gate_client/admission.rs"

  perl -0pi -e 's/let fabricated_allow =/let fabricated_decision_removed =/' \
    "$fixture/codex-rs/hepta-kernel/src/safety_gate/admission_tests.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 exact-admission deny/decision-forgery regressions are incomplete"
  cp "$ROOT/codex-rs/hepta-kernel/src/safety_gate/admission_tests.rs" \
    "$fixture/codex-rs/hepta-kernel/src/safety_gate/admission_tests.rs"

  for drift in \
    'types.rs|anchor_directory: fs::File|anchor_directory: ()' 'execution_attempt.rs|validate_dispatch_selector|validate_unbound_selector' \
    'tests/architecture_v2_dispatch_selector.rs|architecture_v2_dispatch_selector_|architecture_v2_dispatch_selektorr_' 'tests/architecture_v2_symlink_reservation.rs|architecture_v2_symlink_reservation_|architecture_v2_symlink_selektorr_' \
    'transaction_ops.rs|fn process_write_reservation_registry()|fn isolated_write_reservation_registry()' 'tests/architecture_v2_process_reservation.rs|architecture_v2_process_reservation_|architecture_v2_process_rezervation_'
  do
    IFS='|' read -r source original replacement <<<"$drift"
    perl -0pi -e "s/\\Q${original}\\E/${replacement}/g" "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/$source"
    expect_fixture_denied "$fixture" "Architecture V2 sealed execution boundary is incomplete"
    cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/$source" "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/$source"
  done

  perl -0pi -e \
    's/#\[tokio::test\]\nasync fn architecture_v2_dispatch_selector_/#\[allow(dead_code)\]\nasync fn architecture_v2_dispatch_selector_/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_dispatch_selector.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 sealed execution boundary is incomplete"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_dispatch_selector.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_dispatch_selector.rs"

  perl -0pi -e \
    's/outcome_sink: runtime_kernel::outcome_sink::SharedOutcomeReceiptSink/outcome_store: hepta_memory::InMemoryOutcomeStore/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/types.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 RuntimeKernel outcome field is fixed to InMemoryOutcomeStore"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/types.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/types.rs"

  perl -0pi -e \
    's/#\[cfg\(test\)\]\n    pub fn new\(\) -> Self/    pub fn new() -> Self/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/session_ops.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 production runtime exposes an ephemeral constructor"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/session_ops.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/session_ops.rs"

  perl -0pi -e \
    's/open_with_durable_outcomes/open_without_durable_outcomes/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/session_ops.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 fallible durable outcome constructor is incomplete"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/session_ops.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/session_ops.rs"

  perl -0pi -e \
    's/SyncDurableOutcomeWriter::bootstrap_new_keyed/SyncDurableOutcomeWriter::bootstrap_new/g; s/SyncDurableOutcomeWriter::open_existing_keyed/SyncDurableOutcomeWriter::open_existing/g' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 production outcome composition is not keyed"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink.rs"

  perl -0pi -e 's/KEYED_SCHEMA_VERSION: i64 = 5/KEYED_SCHEMA_VERSION: i64 = 4/' \
    "$fixture/codex-rs/hepta-memory/src/durable/integrity.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 keyed durable integrity boundary is incomplete"
  cp "$ROOT/codex-rs/hepta-memory/src/durable/integrity.rs" \
    "$fixture/codex-rs/hepta-memory/src/durable/integrity.rs"

  perl -0pi -e \
    's/PendingOutcomeKind::SafeRetry/PendingOutcomeKind::UnsafeRetry/g' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink/breaker.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 record-first exact outcome reconciliation is incomplete"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink.rs"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink/breaker.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink/breaker.rs"

  perl -0pi -e 's/pub async fn stage_intent/pub async fn stage_unbound/' \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/intent.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 durable producer-intent journal is incomplete"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/durable/intent.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/intent.rs"

  perl -0pi -e 's/\.create_new\(true\)/.create(true)/' \
    "$fixture/codex-rs/hepta-memory/src/durable/opening/filesystem.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 durable database exclusive reservation is incomplete"
  cp "$ROOT/codex-rs/hepta-memory/src/durable/opening/filesystem.rs" \
    "$fixture/codex-rs/hepta-memory/src/durable/opening/filesystem.rs"

  perl -0pi -e 's/MAX_SIDECAR_VALIDATION_ATTEMPTS: usize = 4/MAX_SIDECAR_VALIDATION_ATTEMPTS: usize = 1/' \
    "$fixture/codex-rs/hepta-memory/src/durable/opening/filesystem.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 durable database filesystem opening is incomplete"
  cp "$ROOT/codex-rs/hepta-memory/src/durable/opening/filesystem.rs" \
    "$fixture/codex-rs/hepta-memory/src/durable/opening/filesystem.rs"

  perl -0pi -e 's/\.reopen_existing_bound\(\)/.reopen_unbound()/g' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 record-first exact outcome reconciliation is incomplete"

  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink.rs"
  perl -0pi -e 's/pub async fn open_existing/pub async fn open/' \
    "$fixture/codex-rs/hepta-memory/src/preference_cas/durable.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 durable preference open boundary is incomplete"
  cp "$ROOT/codex-rs/hepta-memory/src/preference_cas/durable.rs" \
    "$fixture/codex-rs/hepta-memory/src/preference_cas/durable.rs"

  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  perl -0pi -e \
    's/native_openclaw_compatible_tools\(\)/quarantined_exec_process_tools_for_test()/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 production registry exposes quarantined exec/process"

  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  perl -0pi -e \
    's/if requests_quarantined_native_tool\(input\) \{/if false {/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 pre-model routing exposes quarantined exec/process"

  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  perl -0pi -e \
    's/(native_tool\(\s*"process".*?RiskTier::)High/${1}Medium/s' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 quarantined process descriptor must remain High, non-read-only, and destructive"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"

  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs"
  perl -0pi -e \
    's/(    fn plan_backup_prune\([^{]+\{)/$1\n        let _ = fs::remove_file("drifted-backup");/s' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 backup prune bypasses identity-sealed deletion"

  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs"
  perl -0pi -e 's/backup prune is quarantined until rollback references have a durable cross-process pin catalog/backup prune enabled without durable pins/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/session_ops.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 release backup prune is not quarantined"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/session_ops.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/session_ops.rs"

  perl -0pi -e 's/"mutation_durability_ambiguous: \{\}"/"mutation_failed: {}"/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 atomic mutation durability boundary is incomplete"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/execution_bus.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/execution_bus.rs"

  perl -0pi -e 's/ \\| libc::LOCK_NB/ | libc::LOCK_UN/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/cross_process_write_lock.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 cross-process write lock is incomplete"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/cross_process_write_lock.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/cross_process_write_lock.rs"
  perl -0pi -e \
    's/record_mutation_transactions_from_tool_result/record_write_transaction_from_tool_result/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/execution_bus.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 generalized mutation transaction evidence is incomplete"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/execution_bus.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/execution_bus.rs"

  perl -0pi -e 's/AuthorizedToolExecution::stage_execution_intent/AuthorizedToolExecution::stage_after_provider/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/execution_bus.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 provider invocation is not ordered after durable intent staging"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/execution_bus.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/execution_bus.rs"

  perl -0pi -e 's/resolve_execution_intent_in_transaction/leave_execution_intent_pending/' \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/intent.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 outcome record and execution-intent resolve are not one transaction"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/durable/intent.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/intent.rs"

  perl -0pi -e 's/execution-intent\.idempotency-key\.v3/execution-intent.idempotency-key.v1/' \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/execution_intent.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 durable intent digest binding is incomplete"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/execution_intent.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/execution_intent.rs"

  perl -0pi -e 's/let feedback = challenge\.into_authenticated\(\);/let feedback = request.into_authenticated();/' \
    "$fixture/codex-rs/hepta-memory/src/preference_authority.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 preference authority is incomplete"
  cp "$ROOT/codex-rs/hepta-memory/src/preference_authority.rs" \
    "$fixture/codex-rs/hepta-memory/src/preference_authority.rs"

  perl -0pi -e 's/DurablePreferenceStore::bootstrap_new_keyed/DurablePreferenceStore::bootstrap_new/' \
    "$fixture/codex-rs/hepta-intelligence/src/trusted_preference_feedback.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 trusted preference adapter is incomplete"
  cp "$ROOT/codex-rs/hepta-intelligence/src/trusted_preference_feedback.rs" \
    "$fixture/codex-rs/hepta-intelligence/src/trusted_preference_feedback.rs"

  perl -0pi -e 's/pinned_source: Some\(&self\.source_binding\)/pinned_source: None/' \
    "$fixture/codex-rs/hepta-intelligence/src/trusted_preference_feedback.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 trusted preference adapter is incomplete"
  cp "$ROOT/codex-rs/hepta-intelligence/src/trusted_preference_feedback.rs" \
    "$fixture/codex-rs/hepta-intelligence/src/trusted_preference_feedback.rs"

  perl -0pi -e 's/match sink\.record_execution_effect_ack\(&ack\)/match sink.record_untracked_effect(&ack)/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/provider_effect.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 provider effect ACK boundary is incomplete"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/provider_effect.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/provider_effect.rs"

  perl -0pi -e 's#std::process::Command::new\("/usr/bin/say"\)#std::process::Command::new("say")#' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 staged TTS provider effect is incomplete"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs"

  perl -0pi -e 's/\n        \.arg\("--"\)//' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 staged TTS provider effect is incomplete"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/transaction_ops.rs"

  perl -0pi -e 's/prepared\.staged_after_bytes\.clone\(\)/prepared.before_bytes.clone()/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/provider_effect.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 provider effect ACK boundary is incomplete"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/provider_effect.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/provider_effect.rs"

  perl -0pi -e 's/candidate_reference_hash: candidate_reference_hash\(authorization\.candidate\(\)\)/candidate_reference_hash: authorization.candidate().content_hash().clone()/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/execution_attempt.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 runtime intent lacks an exact candidate-reference digest"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/execution_attempt.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/execution_attempt.rs"

  perl -0pi -e 's/ProviderExecutionIdentity::from_exact_context/ProviderExecutionIdentity::from_context/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 provider attempt/idempotency binding is incomplete"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"

  perl -0pi -e 's/^    \("transaction\.error_hash", FieldKind::Text\),\n//m' \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/execution_intent/terminal_evidence.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 strict terminal evidence field inventory drifted"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/durable/execution_intent/terminal_evidence.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/execution_intent/terminal_evidence.rs"

  perl -0pi -e 's/outcome\.receipt\(\)\.outcome_hash\(\)/outcome.receipt().unbound_hash()/g' \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/execution_intent/terminal_evidence.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 strict terminal evidence verification is incomplete"
  cp "$ROOT/codex-rs/hepta-memory/src/outcome_store/durable/execution_intent/terminal_evidence.rs" \
    "$fixture/codex-rs/hepta-memory/src/outcome_store/durable/execution_intent/terminal_evidence.rs"

  perl -0pi -e 's/#\[cfg\(test\)\]\n            RegisteredTool::DiskJunkAudit/            RegisteredTool::DiskJunkAudit/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 release disk-junk audit surface is not test-only"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"

  perl -0pi -e 's/(fn native_compat_sessions_send\(.*?)(reject_native_live_without_idempotency_receipt)/${1}allow_native_live_without_receipt/s' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 live native mutation bypasses quarantine"
  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"

  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  perl -0pi -e \
    's/invoke_prepared_native_read/invoke_unsealed_native_read/g' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 sealed read capability is incomplete"

  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  perl -0pi -e \
    's/native_openclaw_compatible_tools\(\)/quarantined_native_read_and_generator_tools_for_test()/' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 production registry exposes an unsealed read or generator surface"

  cp "$ROOT/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs" \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  perl -0pi -e \
    's/(fn invoke_prepared_read_file\([^{]+\{)/$1\n    let _ = fs::read_to_string(&prepared.resolved_path);/s' \
    "$fixture/codex-rs/hepta-runtime/src/runtime_kernel/tool_support.rs"
  expect_fixture_denied "$fixture" \
    "Architecture V2 sealed read provider reopened an authorized path"

  echo '{"schema":"hepta_architecture_v2_dependency_boundary_self_test_v1","status":"ready","negative_fixtures":60,"test_attribute_function_pairing_enforced":true,"production_exec_process_quarantine_enforced":true,"process_metadata_enforced":true,"release_backup_prune_quarantine_enforced":true,"generalized_mutation_transaction_evidence_enforced":true,"atomic_mutation_ambiguity_enforced":true,"execution_intent_before_provider_enforced":true,"atomic_outcome_intent_resolution_enforced":true,"exact_provider_identity_enforced":true,"candidate_reference_digest_enforced":true,"trusted_preference_authority_enforced":true,"trusted_preference_composition_enforced":true,"trusted_preference_race_pin_enforced":true,"provider_effect_ack_enforced":true,"live_tts_staging_enforced":true,"fixed_tts_adapter_path_enforced":true,"tts_option_terminator_enforced":true,"strict_terminal_field_inventory_enforced":true,"strict_terminal_hash_enforced":true,"release_disk_audit_quarantine_enforced":true,"live_native_mutation_quarantine_enforced":true,"cross_process_write_lock_enforced":true,"sealed_read_capability_enforced":true,"sealed_read_production_quarantine_enforced":true,"sealed_read_provider_reopen_denied":true,"production_durable_composition_enforced":true,"durable_keyed_integrity_enforced":true,"durable_producer_intent_journal_enforced":true,"durable_opening_filesystem_enforced":true,"durable_sidecar_identity_recheck_enforced":true}'
)

[[ $# -eq 1 ]] || usage
case "$1" in
  verify)
    verify
    ;;
  self-test)
    self_test
    ;;
  *)
    usage
    ;;
esac
