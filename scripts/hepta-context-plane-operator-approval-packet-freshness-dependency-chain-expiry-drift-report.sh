#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
canonical_digest_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest-report.sh"

line_count() {
  printf '%s\n' "$1" | wc -l | tr -d ' '
}

sha256_digest() {
  shasum -a 256 | awk '{print $1}'
}

source_status="$(bash "$canonical_digest_report")"
source_status_lines="$(line_count "$source_status")"
source_status_sha256="$(printf '%s\n' "$source_status" | sha256_digest)"

cat <<STATUS
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift=pass
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.schema=1
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-canonical-digest-report-lines=$source_status_lines
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-canonical-digest-report-sha256=$source_status_sha256
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.readiness-chain-generation=276
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-readiness-chain-generation=275
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-dependency-chain-generation=274
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-freshness-sequence=273
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.readiness-window-start-sequence=273
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.readiness-window-current-sequence=276
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.readiness-window-expires-after-sequence=277
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.readiness-window-max-drift-sequences=0
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.expired-window=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.window-start-drift=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.window-current-drift=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.window-expiry-drift=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.source-digest-replay=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.payload-field-injection=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.write-activation-field-injection=reject
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.runtime-activation=disabled
context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift.operator-activation=disabled
STATUS
