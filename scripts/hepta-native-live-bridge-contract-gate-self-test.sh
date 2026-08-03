#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

GATE="$REPO_ROOT/scripts/hepta-native-live-bridge-contract-gate.sh"
VALIDATOR_DIR="$REPO_ROOT/scripts/lib"
TEST_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/hepta-native-live-bridge-contract.XXXXXX")"
trap 'rm -rf "$TEST_ROOT"' EXIT

"$GATE" --output "$TEST_ROOT/blocker.json"

jq -e '
  .schema == "hepta-native-live-bridge-blocker-receipt-v1"
  and .status == "blocked"
  and .contract_gate_ready == true
  and .backend_handoff_ready == true
  and .hepta_live_bridge_ready == false
  and .canonical_endpoint.method == "GET"
  and .canonical_endpoint.path == "/api/hepta-native-bridge/v1/snapshot"
  and .canonical_endpoint.registered == false
  and .canonical_endpoint.loopback_only == true
  and .canonical_endpoint.explicit_opt_in_required == true
  and .canonical_endpoint.matrix_login_required == true
  and .canonical_endpoint.authenticated_session_binding_available == false
  and .transport_seam.snapshot_get_only == true
  and .transport_seam.request_body_allowed == false
  and .transport_seam.redirect_allowed == false
  and .transport_seam.response_size_bounded == true
  and .transport_seam.session_and_correlation_binding_required == true
  and .transport_seam.authenticated_http_executor_available == false
  and .transport_seam.wired_to_product_lifecycle == false
  and (.evidence.live_adapter_path | endswith("/src/hepta_bridge/live_adapter.rs"))
  and (.candidate_endpoint_audit | length) == 6
  and (.candidate_endpoint_audit | all(.authoritative_bridge_snapshot == false))
  and .actual_request.performed == false
  and .actual_request.request_descriptor_sha256 == null
  and .actual_request.response_sha256 == null
  and .capabilities.snapshot == false
  and .capabilities.subscribe == false
  and .capabilities.prepare == false
  and .capabilities.confirm == false
  and .capabilities.reject == false
  and .capabilities.cancel == false
  and (.blockers | index("authenticated_http_executor_not_available") != null)
  and (.blockers | index("snapshot_transport_not_wired_to_product_lifecycle") != null)
  and (.blockers | index("snapshot_only_live_adapter_not_implemented") == null)
  and .side_effects.network_request_performed == false
  and .side_effects.matrix_login_performed == false
  and .side_effects.provider_invoked == false
  and .side_effects.channel_delivery_performed == false
  and .side_effects.cursor_written == false
  and .side_effects.gateway_mutation_performed == false
  and .side_effects.external_mutation_performed == false
  and .claim_boundaries.source_audit_is_live_receipt == false
  and .claim_boundaries.synthetic_envelope_is_live_receipt == false
  and .claim_boundaries.legacy_report_endpoint_is_task_truth == false
  and .claim_boundaries.matrix_event_is_hepta_receipt == false
  and .claim_boundaries.public_or_live_readiness_claimed == false
' "$TEST_ROOT/blocker.json" >/dev/null

jq -n '{
  metadata:{
    schema_version:1,
    stable_id:"snapshot-4",
    revision:4,
    cursor:null,
    timestamp:1785688800000,
    session_id:"session-7",
    correlation_id:"correlation-11",
    origin:{kind:"bridge_adapter",component:"hepta-native-gateway"},
    redaction:{status:"redacted",policy:"hepta-native-bridge-v1",removed_fields:[]},
    provenance:{
      source:"hepta-runtime-snapshot-store",
      source_entity_id:"snapshot-4",
      source_revision:4,
      observed_at:1785688800000
    }
  },
  binding:{
    matrix_room_id:null,
    hepta_session_id:"session-7",
    revision:4,
    mirror_policy:"local_only"
  },
  update:{
    type:"snapshot",
    data:{
      snapshot:{
        revision:4,
        cursor:null,
        runtime:{
          metadata:{
            schema_version:1,
            stable_id:"runtime-local",
            revision:4,
            cursor:null,
            timestamp:1785688800000,
            session_id:"session-7",
            correlation_id:"correlation-11",
            origin:{kind:"hepta_runtime"},
            redaction:{status:"not_required",policy:null},
            provenance:{
              source:"hepta-runtime",
              source_entity_id:"runtime-local",
              source_revision:4,
              observed_at:1785688800000
            }
          },
          state:"ready",
          title:"Local runtime",
          summary:"Read-only snapshot"
        },
        tasks:[],
        tool_invocations:[],
        approvals:[],
        activities:[]
      }
    }
  }
}' > "$TEST_ROOT/valid-envelope.json"

jq -e \
  -L "$VALIDATOR_DIR" \
  --arg session_id 'session-7' \
  --arg correlation_id 'correlation-11' \
  'include "hepta-native-live-bridge-envelope-v1";
   hepta_native_live_bridge_envelope_v1_valid($session_id; $correlation_id)' \
  "$TEST_ROOT/valid-envelope.json" >/dev/null

jq 'del(.metadata.correlation_id)' \
  "$TEST_ROOT/valid-envelope.json" > "$TEST_ROOT/missing-correlation.json"
if jq -e \
  -L "$VALIDATOR_DIR" \
  --arg session_id 'session-7' \
  --arg correlation_id 'correlation-11' \
  'include "hepta-native-live-bridge-envelope-v1";
   hepta_native_live_bridge_envelope_v1_valid($session_id; $correlation_id)' \
  "$TEST_ROOT/missing-correlation.json" >/dev/null
then
  printf '%s\n' 'envelope without correlation unexpectedly passed' >&2
  exit 1
fi

jq '.update.data.snapshot.runtime.metadata.redaction.status = "unredacted"' \
  "$TEST_ROOT/valid-envelope.json" > "$TEST_ROOT/unredacted.json"
if jq -e \
  -L "$VALIDATOR_DIR" \
  --arg session_id 'session-7' \
  --arg correlation_id 'correlation-11' \
  'include "hepta-native-live-bridge-envelope-v1";
   hepta_native_live_bridge_envelope_v1_valid($session_id; $correlation_id)' \
  "$TEST_ROOT/unredacted.json" >/dev/null
then
  printf '%s\n' 'unredacted record unexpectedly passed' >&2
  exit 1
fi

jq -n '{
  product:"Hepta",
  runtime:"hepta",
  status:"ready",
  side_effect_free:true,
  active_gateway_replacement_ready:false,
  route_matrix_ready:true
}' > "$TEST_ROOT/operator-snapshot-shape.json"
if jq -e \
  -L "$VALIDATOR_DIR" \
  --arg session_id 'session-7' \
  --arg correlation_id 'correlation-11' \
  'include "hepta-native-live-bridge-envelope-v1";
   hepta_native_live_bridge_envelope_v1_valid($session_id; $correlation_id)' \
  "$TEST_ROOT/operator-snapshot-shape.json" >/dev/null
then
  printf '%s\n' 'aggregate operator snapshot unexpectedly passed as bridge truth' >&2
  exit 1
fi

cp apps/hepta-native/src/hepta_bridge/mod.rs "$TEST_ROOT/bridge-mod.rs"
ruby -e '
  path = ARGV.fetch(0)
  source = File.binread(path)
  source = source.gsub("DisabledBridgeAdapter", "UnreviewedLiveBridgeAdapter")
  File.binwrite(path, source)
' "$TEST_ROOT/bridge-mod.rs"
set +e
HEPTA_NATIVE_LIVE_BRIDGE_MOD_PATH="$TEST_ROOT/bridge-mod.rs" \
  "$GATE" --output "$TEST_ROOT/dishonest-live.json" >/dev/null 2>&1
dishonest_live_exit=$?
set -e
if [[ "$dishonest_live_exit" -eq 0 ]]; then
  printf '%s\n' 'gate accepted replacement of the disabled production adapter' >&2
  exit 1
fi

jq '.current_implementation.canonical_endpoint_registered = true' \
  apps/hepta-native/hepta-live-bridge-backend-contract-v1.json \
  > "$TEST_ROOT/dishonest-contract.json"
set +e
HEPTA_NATIVE_LIVE_BRIDGE_CONTRACT_PATH="$TEST_ROOT/dishonest-contract.json" \
  "$GATE" --output "$TEST_ROOT/dishonest-contract-receipt.json" >/dev/null 2>&1
dishonest_contract_exit=$?
set -e
if [[ "$dishonest_contract_exit" -eq 0 ]]; then
  printf '%s\n' 'gate accepted a source-unverified canonical endpoint claim' >&2
  exit 1
fi

jq '.current_implementation.native_snapshot_transport_seam_available = false' \
  apps/hepta-native/hepta-live-bridge-backend-contract-v1.json \
  > "$TEST_ROOT/missing-seam-contract.json"
set +e
HEPTA_NATIVE_LIVE_BRIDGE_CONTRACT_PATH="$TEST_ROOT/missing-seam-contract.json" \
  "$GATE" --output "$TEST_ROOT/missing-seam-receipt.json" >/dev/null 2>&1
missing_seam_exit=$?
set -e
if [[ "$missing_seam_exit" -eq 0 ]]; then
  printf '%s\n' 'gate accepted a contract that denied the compiled snapshot seam' >&2
  exit 1
fi

jq '.current_implementation.authenticated_http_executor_available = true' \
  apps/hepta-native/hepta-live-bridge-backend-contract-v1.json \
  > "$TEST_ROOT/dishonest-executor-contract.json"
set +e
HEPTA_NATIVE_LIVE_BRIDGE_CONTRACT_PATH="$TEST_ROOT/dishonest-executor-contract.json" \
  "$GATE" --output "$TEST_ROOT/dishonest-executor-receipt.json" >/dev/null 2>&1
dishonest_executor_exit=$?
set -e
if [[ "$dishonest_executor_exit" -eq 0 ]]; then
  printf '%s\n' 'gate accepted a source-unverified authenticated HTTP executor claim' >&2
  exit 1
fi

printf '%s\n' 'hepta-native live bridge contract gate self-test: PASS'
