#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
BASE_FILE="$ROOT/.hepta-suffix-ladder-freeze-base"

[[ -f "$BASE_FILE" ]] || {
  echo "missing suffix-ladder freeze base: $BASE_FILE" >&2
  exit 1
}

base_commit="$(tr -d '[:space:]' <"$BASE_FILE")"
git -C "$ROOT" cat-file -e "$base_commit^{commit}" 2>/dev/null || {
  echo "suffix-ladder freeze base is not a commit: $base_commit" >&2
  exit 1
}

added_file_list="$(mktemp "${TMPDIR:-/tmp}/hepta-suffix-ladder-added.XXXXXX")"
violation_list="$(mktemp "${TMPDIR:-/tmp}/hepta-suffix-ladder-violations.XXXXXX")"
trap 'rm -f "$added_file_list" "$violation_list"' EXIT

{
  git -C "$ROOT" diff --name-only --diff-filter=A "$base_commit"..HEAD -- \
    codex-rs/hepta-runtime/src scripts docs/architecture
  git -C "$ROOT" diff --name-only --diff-filter=A HEAD -- \
    codex-rs/hepta-runtime/src scripts docs/architecture
  git -C "$ROOT" ls-files --others --exclude-standard -- \
    codex-rs/hepta-runtime/src scripts docs/architecture
} | LC_ALL=C sort -u >"$added_file_list"

while IFS= read -r file; do
  [[ -n "$file" ]] || continue
  filename="${file##*/}"
  case "$file" in
    codex-rs/hepta-runtime/src/*)
      if [[ "$filename" =~ (report|readback|preview|denial|receipt|evidence) ]]; then
        printf '%s\t%s\n' "runtime_suffix_module" "$file" >>"$violation_list"
      fi
      ;;
    scripts/*)
      if [[ "$filename" == *-gate.sh || "$filename" == *-report.sh ]]; then
        printf '%s\t%s\n' "shell_gate_report_pair_surface" "$file" >>"$violation_list"
      fi
      ;;
    docs/architecture/*)
      if [[ "$filename" =~ (REPORT|READBACK|PREVIEW|DENIAL|RECEIPT|EVIDENCE) ]]; then
        printf '%s\t%s\n' "architecture_suffix_document" "$file" >>"$violation_list"
      fi
      ;;
  esac
done <"$added_file_list"

violation_count="$(wc -l <"$violation_list" | tr -d '[:space:]')"
violations_json="$(
  if [[ "$violation_count" == "0" ]]; then
    printf '[]\n'
  else
    jq -Rn \
      '[inputs | split("\t") | {category:.[0], file:.[1]}]' \
      <"$violation_list"
  fi
)"

jq -n \
  --arg status "$([[ "$violation_count" == "0" ]] && echo ready || echo blocked)" \
  --arg base_commit "$base_commit" \
  --argjson violation_count "$violation_count" \
  --argjson violations "$violations_json" \
  '{
    status:$status,
    check:"hepta_suffix_ladder_freeze",
    base_commit:$base_commit,
    policy:"new suffix-expanded Runtime modules, shell gate/report files, and architecture evidence documents are frozen",
    existing_surfaces_preserved:true,
    deletion_requires_parity_snapshot:true,
    violation_count:$violation_count,
    violations:$violations
  }'

if [[ "$violation_count" != "0" ]]; then
  echo "new suffix-ladder surfaces are frozen; migrate the capability into GateSpec/ReceiptStateMachine or an existing generic runner" >&2
  exit 1
fi
