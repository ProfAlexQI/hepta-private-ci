#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readiness_report="$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-readiness-report.sh"
negative_rehearsal_report="$repo_root/scripts/hepta-context-memory-shadow-canary-promotion-negative-rehearsal-report.sh"

line_count() {
  printf '%s\n' "$1" | wc -l | tr -d ' '
}

sha256_digest() {
  shasum -a 256 | awk '{print $1}'
}

readiness_status="$(bash "$readiness_report")"
negative_rehearsal_status="$(bash "$negative_rehearsal_report")"
combined_status="$(printf '%s\n%s' "$readiness_status" "$negative_rehearsal_status")"

cat <<STATUS
memory-shadow-canary-promotion-audit-digest=pass
memory-shadow-canary-promotion-audit-digest.schema=1
memory-shadow-canary-promotion-audit-digest.payload-light=pass
memory-shadow-canary-promotion-audit-digest.readiness-report-lines=$(line_count "$readiness_status")
memory-shadow-canary-promotion-audit-digest.readiness-report-sha256=$(printf '%s\n' "$readiness_status" | sha256_digest)
memory-shadow-canary-promotion-audit-digest.negative-rehearsal-report-lines=$(line_count "$negative_rehearsal_status")
memory-shadow-canary-promotion-audit-digest.negative-rehearsal-report-sha256=$(printf '%s\n' "$negative_rehearsal_status" | sha256_digest)
memory-shadow-canary-promotion-audit-digest.combined-report-lines=$(line_count "$combined_status")
memory-shadow-canary-promotion-audit-digest.combined-report-sha256=$(printf '%s\n' "$combined_status" | sha256_digest)
memory-shadow-canary-promotion-audit-digest.runtime-activation=disabled
memory-shadow-canary-promotion-audit-digest.operator-activation=disabled
STATUS
