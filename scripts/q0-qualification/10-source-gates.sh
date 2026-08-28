# shellcheck shell=bash
gate_dir="$ARTIFACT_DIR/gates"
status_file=plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V2.json
canonical_status=$(mktemp)
cp "$status_file" "$canonical_status"
restore_status() {
  cp "$canonical_status" "$status_file"
  rm -f "$canonical_status"
}
trap restore_status EXIT

run_gate() {
  local name=$1
  shift
  set +e
  "$@" >"$gate_dir/$name.log" 2>&1
  local code=$?
  set -e
  printf '%s\n' "$code" >"$gate_dir/$name.exit"
  printf '%-36s exit=%s\n' "$name" "$code"
}

run_tranche_gate() {
  local tranche=$1
  local name=$2
  shift 2
  local code=0
  local restore_code=0
  set +e
  python3 scripts/hepta-intelligence-status-compat.py "$tranche" \
    >"$gate_dir/$name.status.log" 2>&1
  code=$?
  if test "$code" = "0"; then
    "$@" >"$gate_dir/$name.log" 2>&1
    code=$?
  else
    printf 'status compatibility projection failed for %s\n' "$tranche" \
      >"$gate_dir/$name.log"
  fi
  cp "$canonical_status" "$status_file"
  restore_code=$?
  if test "$code" = "0" && test "$restore_code" != "0"; then
    code=$restore_code
  fi
  set -e
  printf '%s\n' "$code" >"$gate_dir/$name.exit"
  printf '%-36s exit=%s tranche=%s\n' "$name" "$code" "$tranche"
}

master_plan_gate() {
  local actual
  actual=$(python3 scripts/verify-hepta-intelligence-master-plan.py)
  printf '%s\n' "$actual"
  test "$actual" = "PASS_HEPTA_INTELLIGENCE_MASTER_PLAN_V4_SOURCE_ONLY"
}

run_gate master-plan master_plan_gate
for tranche in P0.2 P0.3 P0.4a P0.4b P0.4c; do
  run_gate "status-$tranche" \
    python3 scripts/hepta-intelligence-status-compat.py "$tranche" --check-only
done
run_tranche_gate P0.2 grounding-ledger \
  python3 scripts/verify-hepta-intelligence-grounding-ledger.py
run_tranche_gate P0.3 grounding-gate \
  python3 scripts/verify-hepta-intelligence-grounding-gate.py
run_tranche_gate P0.4a mutation-state \
  python3 scripts/verify-hepta-intelligence-mutation-state.py
run_gate mutation-journal-sqlite \
  python3 scripts/hepta-intelligence-mutation-journal-sqlite-selftest.py
run_tranche_gate P0.4b mutation-journal \
  python3 scripts/verify-hepta-intelligence-mutation-journal.py
run_tranche_gate P0.4c shadow-host \
  python3 scripts/verify-hepta-intelligence-shadow-host.py

restore_status
trap - EXIT
git diff --check
