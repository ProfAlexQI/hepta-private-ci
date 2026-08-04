#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
GENERATOR="$REPO_ROOT/scripts/hepta-native-route-catalog"
CATALOG="$REPO_ROOT/codex-rs/hepta-native-gateway/routes/control_ui_route_catalog_v1.jsonl"
EVIDENCE="$REPO_ROOT/codex-rs/hepta-native-gateway/routes/control_ui_evidence_definition_registry_v1.jsonl"
TARGET_ROUTE="/api/hepta-memory-intelligence-kg-full-enablement-runtime-readiness"
TMP_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/hepta-evidence-separation.XXXXXX")"
trap 'rm -rf "$TMP_ROOT"' EXIT

fixture_catalog="$TMP_ROOT/route-catalog-without-target.jsonl"
ruby -rjson - "$CATALOG" "$fixture_catalog" "$TARGET_ROUTE" <<'RUBY'
source, destination, target = ARGV
records = File.readlines(source, chomp: true).reject(&:empty?).map { |line| JSON.parse(line) }
index = records.index { |record| record["kind"] == "control_ui_route" && record["path"] == target }
abort "target HTTP route missing from source catalog" unless index
records.delete_at(index)
routes = records.select { |record| record["kind"] == "control_ui_route" }
records.first["route_count"] = routes.length
routes.each_with_index do |route, ordinal|
  route["ordinal"] = ordinal
  route["key"] = format("control_ui_%03d", ordinal)
end
File.write(destination, records.map { |record| JSON.generate(record) }.join("\n") + "\n")
RUBY

receipt="$TMP_ROOT/independence.json"
HEPTA_NATIVE_ROUTE_CATALOG="$fixture_catalog" \
HEPTA_EVIDENCE_DEFINITION_REGISTRY="$EVIDENCE" \
HEPTA_EVIDENCE_EXPECT_MISSING_HTTP_ROUTE="$TARGET_ROUTE" \
  "$GENERATOR" verify-evidence-independence >"$receipt"
jq -e '
  .schema == "hepta_evidence_definition_registry_independence_v1"
  and .status == "ready"
  and .http_route_count == 283
  and .evidence_definition_count == 207
  and .removed_http_selector_preserved == true
  and .stable_renderer_key_preserved == true
' "$receipt" >/dev/null

missing_evidence="$TMP_ROOT/evidence-without-target.jsonl"
ruby -rjson - "$EVIDENCE" "$missing_evidence" "$TARGET_ROUTE" <<'RUBY'
source, destination, target = ARGV
records = File.readlines(source, chomp: true).reject(&:empty?).map { |line| JSON.parse(line) }
index = records.index { |record| record["kind"] == "evidence_definition" && record["route_selector"] == target }
abort "target evidence definition missing" unless index
records.delete_at(index)
definitions = records.drop(1)
records.first["evidence_definition_count"] = definitions.length
records.first["renderable_evidence_definition_count"] = definitions.count { |record| record["renderer_key"] }
records.first["legacy_evidence_definition_count"] = definitions.count { |record| record["legacy_compatibility_route"] }
definitions.each_with_index { |record, ordinal| record["definition_key"] = format("evidence_%03d", ordinal) }
File.write(destination, records.map { |record| JSON.generate(record) }.join("\n") + "\n")
RUBY
if HEPTA_NATIVE_ROUTE_CATALOG="$fixture_catalog" \
  HEPTA_EVIDENCE_DEFINITION_REGISTRY="$missing_evidence" \
  HEPTA_EVIDENCE_EXPECT_MISSING_HTTP_ROUTE="$TARGET_ROUTE" \
  "$GENERATOR" verify-evidence-independence >/dev/null 2>&1; then
  echo "evidence separation accepted deletion from both typed sources" >&2
  exit 1
fi

bad_renderer="$TMP_ROOT/evidence-with-unknown-renderer.jsonl"
ruby -rjson - "$EVIDENCE" "$bad_renderer" "$TARGET_ROUTE" <<'RUBY'
source, destination, target = ARGV
records = File.readlines(source, chomp: true).reject(&:empty?).map { |line| JSON.parse(line) }
definition = records.find { |record| record["kind"] == "evidence_definition" && record["route_selector"] == target }
abort "target evidence definition missing" unless definition
definition["renderer_key"] = "native_report_999"
File.write(destination, records.map { |record| JSON.generate(record) }.join("\n") + "\n")
RUBY
if HEPTA_NATIVE_ROUTE_CATALOG="$fixture_catalog" \
  HEPTA_EVIDENCE_DEFINITION_REGISTRY="$bad_renderer" \
  HEPTA_EVIDENCE_EXPECT_MISSING_HTTP_ROUTE="$TARGET_ROUTE" \
  "$GENERATOR" verify-evidence-independence >/dev/null 2>&1; then
  echo "evidence separation accepted an unknown renderer key" >&2
  exit 1
fi

echo "hepta-native-route-catalog-evidence-separation-self-test: PASS"
