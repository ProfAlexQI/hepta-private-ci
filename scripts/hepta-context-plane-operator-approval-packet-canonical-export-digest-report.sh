#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
approval_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-report.sh"
negative_export_report="$repo_root/scripts/hepta-context-plane-operator-approval-packet-negative-export-report.sh"

line_count() {
  printf '%s\n' "$1" | wc -l | tr -d ' '
}

sha256_digest() {
  shasum -a 256 | awk '{print $1}'
}

approval_status="$(bash "$approval_report")"
negative_export_status="$(bash "$negative_export_report")"
combined_status="$(printf '%s\n%s' "$approval_status" "$negative_export_status")"

approval_line_count="$(line_count "$approval_status")"
negative_export_line_count="$(line_count "$negative_export_status")"
combined_line_count="$(line_count "$combined_status")"
approval_digest="$(printf '%s\n' "$approval_status" | sha256_digest)"
negative_export_digest="$(printf '%s\n' "$negative_export_status" | sha256_digest)"
combined_digest="$(printf '%s\n' "$combined_status" | sha256_digest)"

cat <<STATUS
context-plane-operator-approval-packet-canonical-export-digest=pass
context-plane-operator-approval-packet-canonical-export-digest.schema=1
context-plane-operator-approval-packet-canonical-export-digest.approval-report-lines=$approval_line_count
context-plane-operator-approval-packet-canonical-export-digest.approval-report-sha256=$approval_digest
context-plane-operator-approval-packet-canonical-export-digest.negative-export-report-lines=$negative_export_line_count
context-plane-operator-approval-packet-canonical-export-digest.negative-export-report-sha256=$negative_export_digest
context-plane-operator-approval-packet-canonical-export-digest.combined-report-lines=$combined_line_count
context-plane-operator-approval-packet-canonical-export-digest.combined-report-sha256=$combined_digest
context-plane-operator-approval-packet-canonical-export-digest.runtime-activation=disabled
context-plane-operator-approval-packet-canonical-export-digest.operator-activation=disabled
STATUS
