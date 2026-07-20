#!/usr/bin/env bash

# Shared, side-effect-free build provenance helpers for Hepta release evidence.
# This file is sourced by preflight, immutable release materialization, and
# focused fixture tests. Callers retain responsibility for set -euo pipefail.

HEPTA_BUILD_PROVENANCE_SCHEMA="hepta_build_provenance_v1"

hepta_release_sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

hepta_release_sha256_text() {
  shasum -a 256 | awk '{print $1}'
}

hepta_release_file_records_json() {
  local root="$1"
  shift

  local records="[]"
  local relative_path file_sha
  for relative_path in "$@"; do
    [[ -f "$root/$relative_path" ]] || {
      echo "missing release provenance input: $root/$relative_path" >&2
      return 1
    }
    file_sha="$(hepta_release_sha256_file "$root/$relative_path")"
    records="$(
      jq -cn \
        --argjson records "$records" \
        --arg path "$relative_path" \
        --arg sha256 "$file_sha" \
        '$records + [{path:$path,sha256:$sha256}]'
    )"
  done

  printf '%s\n' "$records"
}

hepta_release_records_aggregate_sha256() {
  jq -r '.[] | [.path, .sha256] | @tsv' | hepta_release_sha256_text
}

hepta_release_rustc_record_json() {
  local root="$1"
  local workspace="$2"
  local verbose rustc_sha

  verbose="$(cd "$root/$workspace" && rustc -vV)"
  rustc_sha="$(printf '%s' "$verbose" | hepta_release_sha256_text)"
  jq -cn \
    --arg workspace "$workspace" \
    --arg sha256 "$rustc_sha" \
    '{workspace:$workspace,rustc_verbose_sha256:$sha256}'
}

hepta_release_build_provenance_json() {
  local root="$1"
  local source_commit="$2"
  local artifact="$3"

  [[ "$source_commit" =~ ^[0-9a-f]{40}$ ]] || {
    echo "release provenance source commit must be a full Git SHA" >&2
    return 1
  }
  [[ -f "$artifact" ]] || {
    echo "missing release provenance artifact: $artifact" >&2
    return 1
  }

  local toolchain_inputs dependency_inputs rustc_records
  local toolchain_inputs_sha dependency_inputs_sha rustc_records_sha
  local toolchain_aggregate_sha dependency_aggregate_sha artifact_sha

  toolchain_inputs="$(
    hepta_release_file_records_json \
      "$root" \
      codex-rs/rust-toolchain.toml \
      apps/hepta-native/rust-toolchain.toml
  )"
  dependency_inputs="$(
    hepta_release_file_records_json \
      "$root" \
      codex-rs/Cargo.lock \
      apps/hepta-native/Cargo.lock
  )"
  rustc_records="$(
    jq -cn \
      --argjson codex "$(hepta_release_rustc_record_json "$root" codex-rs)" \
      --argjson native "$(hepta_release_rustc_record_json "$root" apps/hepta-native)" \
      '[$codex,$native]'
  )"

  toolchain_inputs_sha="$(printf '%s\n' "$toolchain_inputs" | hepta_release_records_aggregate_sha256)"
  dependency_inputs_sha="$(printf '%s\n' "$dependency_inputs" | hepta_release_records_aggregate_sha256)"
  rustc_records_sha="$(
    jq -r '.[] | [.workspace, .rustc_verbose_sha256] | @tsv' <<<"$rustc_records" \
      | hepta_release_sha256_text
  )"
  toolchain_aggregate_sha="$(
    printf '%s\n%s\n' "$toolchain_inputs_sha" "$rustc_records_sha" \
      | hepta_release_sha256_text
  )"
  dependency_aggregate_sha="$dependency_inputs_sha"
  artifact_sha="$(hepta_release_sha256_file "$artifact")"

  jq -cn \
    --arg schema_version "$HEPTA_BUILD_PROVENANCE_SCHEMA" \
    --arg source_commit "$source_commit" \
    --arg toolchain_aggregate_sha256 "$toolchain_aggregate_sha" \
    --arg dependency_aggregate_sha256 "$dependency_aggregate_sha" \
    --arg artifact_sha256 "$artifact_sha" \
    --argjson toolchain_inputs "$toolchain_inputs" \
    --argjson rustc_records "$rustc_records" \
    --argjson dependency_inputs "$dependency_inputs" \
    '{
      schema_version:$schema_version,
      source:{commit:$source_commit,commit_bound:true},
      toolchain:{
        bound:true,
        aggregate_sha256:$toolchain_aggregate_sha256,
        manifest_inputs:$toolchain_inputs,
        rustc_verbose_inputs:$rustc_records
      },
      dependencies:{
        bound:true,
        aggregate_sha256:$dependency_aggregate_sha256,
        lock_inputs:$dependency_inputs
      },
      artifact:{bound:true,sha256:$artifact_sha256}
    }'
}

hepta_release_validate_build_provenance_json() {
  local provenance_json="$1"
  local source_commit="$2"
  local artifact_sha="$3"

  jq -e \
    --arg schema "$HEPTA_BUILD_PROVENANCE_SCHEMA" \
    --arg source_commit "$source_commit" \
    --arg artifact_sha "$artifact_sha" \
    '
      .schema_version == $schema
      and .source.commit_bound == true
      and .source.commit == $source_commit
      and .toolchain.bound == true
      and (.toolchain.aggregate_sha256 | test("^[0-9a-f]{64}$"))
      and (.toolchain.manifest_inputs | type == "array" and length >= 2)
      and (.toolchain.manifest_inputs | all(
        (.path | type == "string" and length > 0)
        and (.sha256 | test("^[0-9a-f]{64}$"))
      ))
      and (.toolchain.rustc_verbose_inputs | type == "array" and length >= 2)
      and (.toolchain.rustc_verbose_inputs | all(
        (.workspace | type == "string" and length > 0)
        and (.rustc_verbose_sha256 | test("^[0-9a-f]{64}$"))
      ))
      and .dependencies.bound == true
      and (.dependencies.aggregate_sha256 | test("^[0-9a-f]{64}$"))
      and (.dependencies.lock_inputs | type == "array" and length >= 2)
      and (.dependencies.lock_inputs | all(
        (.path | type == "string" and length > 0)
        and (.sha256 | test("^[0-9a-f]{64}$"))
      ))
      and .artifact.bound == true
      and .artifact.sha256 == $artifact_sha
    ' <<<"$provenance_json" >/dev/null
}
