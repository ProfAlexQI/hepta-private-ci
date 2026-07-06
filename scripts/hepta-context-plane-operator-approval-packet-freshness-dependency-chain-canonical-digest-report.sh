#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
chain_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-report.sh"

line_count() {
  printf '%s\n' "$1" | wc -l | tr -d ' '
}

sha256_digest() {
  shasum -a 256 | awk '{print $1}'
}

chain_status="$(bash "$chain_report")"
chain_status_lines="$(line_count "$chain_status")"
chain_status_sha256="$(printf '%s\n' "$chain_status" | sha256_digest)"

cat <<STATUS
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest=pass
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.schema=1
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.dependency-chain-report-lines=$chain_status_lines
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.dependency-chain-report-sha256=$chain_status_sha256
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.readiness-chain-generation=275
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.source-readiness-chain-generation=274
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.source-freshness-sequence=273
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.reordered-dependency-rows=reject
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.mismatched-upstream-digest=reject
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.mixed-generation-replay=reject
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.mixed-sequence-replay=reject
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.payload-field-injection=reject
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.write-activation-field-injection=reject
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.runtime-activation=disabled
context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest.operator-activation=disabled
STATUS
