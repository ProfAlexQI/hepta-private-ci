#!/usr/bin/env bash

# Count only Rust tests where a #[test] or #[tokio::test(...)] attribute is
# paired with the immediately following target function. Blank lines and
# additional non-test attributes are allowed between the test attribute and
# function; any other intervening source is rejected as an invalid pair.
HEPTA_V2_RUNTIME_NEURON_TEST_PATTERN='(?:activate_neurons_bootstraps_direct_activation_from_recall_evidence|neuron_activation_overview_respects_zero_limit|neuron_activation_overview_uses_topic_routing_state|intuition_overview_reuses_single_routing_state_for_neuron_activation|intuition_overview_uses_durable_neuron_store_and_feedback_calibration|neuron_lookup_revalidates_stored_neuron_when_topic_evidence_changes|compress_topic_to_neuron_collects_provenance_and_component_links|compress_active_topics_to_neurons_returns_unique_active_topics)'

hepta_v2_test_pair_count() {
  local function_pattern="$1"
  shift
  local file count total=0

  for file in "$@"; do
    [[ -f "$file" ]] || {
      echo "Architecture V2 test inventory source is missing: $file" >&2
      return 1
    }
    count="$(
      HEPTA_V2_TEST_FUNCTION_PATTERN="$function_pattern" perl -ne '
        BEGIN {
          $target = qr/\A(?:$ENV{HEPTA_V2_TEST_FUNCTION_PATTERN})\z/;
          $pending = 0;
          $count = 0;
        }
        if (/^\s*#\[(?:tokio::)?test(?:\([^\]]*\))?\]\s*$/) {
          die "$ARGV:$.: nested test attribute without a function\n" if $pending;
          $pending = 1;
          next;
        }
        if ($pending) {
          next if /^\s*$/;
          next if /^\s*#\[(?!test(?:\(|\]))[^\]]+\]\s*$/;
          if (/^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\b/) {
            $count++ if $1 =~ $target;
            $pending = 0;
            next;
          }
          die "$ARGV:$.: test attribute is not paired with a function\n";
        }
        if (eof && $pending) {
          die "$ARGV:$.: test attribute reaches end of file without a function\n";
        }
        END {
          print "$count\n" unless $@;
        }
      ' "$file"
    )" || return 1
    total=$((total + count))
  done

  printf '%s\n' "$total"
}

hepta_v2_assert_test_inventory() {
  local label="$1"
  local expected="$2"
  local function_pattern="$3"
  shift 3
  local actual

  actual="$(hepta_v2_test_pair_count "$function_pattern" "$@")" || return 1
  if [[ "$actual" != "$expected" ]]; then
    echo "$label test inventory mismatch: expected $expected attributed function pair(s), got $actual" >&2
    return 1
  fi
}
