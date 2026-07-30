#!/usr/bin/env bash

HEPTA_IMMUTABLE_WATCHDOG_CLOSURE_SCHEMA="hepta_immutable_watchdog_closure_v1"
HEPTA_IMMUTABLE_WATCHDOG_ENTRYPOINT="scripts/hepta-watchdog.sh"
HEPTA_IMMUTABLE_WATCHDOG_PRODUCT_BOUNDARY="docs/decisions/hepta-product-boundary-v1.json"
HEPTA_IMMUTABLE_WATCHDOG_VERIFY_TOOL="scripts/hepta-immutable-release-tree"
HEPTA_IMMUTABLE_WATCHDOG_LEGACY_K_SOURCE_COMMIT="7e9d073a7ddb68d1f78666ebbf889c73d54d254a"

hepta_immutable_watchdog_closure_spec() {
  cat <<'EOF'
scripts/hepta-watchdog.sh	0555	true
scripts/hepta-live-soak.sh	0555	true
scripts/hepta-immutable-release-tree	0555	true
scripts/hepta-generation-pointer	0555	true
scripts/hepta-off-device-archive	0555	true
scripts/hepta-log-rotate	0555	true
scripts/hepta-final-evidence-index	0555	true
scripts/hepta-stage-identity-chain	0555	true
scripts/hepta-preflight-evidence-pack	0555	true
scripts/hepta-install-live-watchdog	0555	true
scripts/hepta-install-live-gateway	0555	true
scripts/lib/hepta-watchdog-release-evidence-v1.sh	0444	false
scripts/lib/hepta-watchdog-product-boundary-v1.sh	0444	false
scripts/lib/hepta-release-provenance.sh	0444	false
scripts/lib/hepta-immutable-watchdog-closure-v1.sh	0444	false
scripts/hepta-dependency-security-v1.json	0444	false
scripts/hepta-dependency-exception-policy-v1.json	0444	false
docs/decisions/hepta-product-boundary-v1.json	0444	false
EOF
}

hepta_immutable_watchdog_sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}

hepta_immutable_watchdog_sha256_text() {
  shasum -a 256 | awk '{print $1}'
}

hepta_immutable_watchdog_expected_paths_json() {
  local source_commit="${1:-}"
  hepta_immutable_watchdog_closure_spec \
    | cut -f1 \
    | if [[ "$source_commit" == "$HEPTA_IMMUTABLE_WATCHDOG_LEGACY_K_SOURCE_COMMIT" ]]; then
        grep -v -E '^scripts/hepta-(generation-pointer|off-device-archive|log-rotate|final-evidence-index)$'
      else
        cat
      fi \
    | jq -R . \
    | jq -cs .
}

hepta_immutable_watchdog_records_aggregate_sha256() {
  jq -r '.[] | [.relative_path,.sha256,.size_bytes,.mode,.executable] | @tsv' \
    | hepta_immutable_watchdog_sha256_text
}

hepta_immutable_watchdog_materialize() {
  local source_root="$1"
  local staging_root="$2"
  local source_commit="$3"
  local source_bound="$4"
  local records="[]"
  local relative_path mode executable source_file destination_file file_sha file_size

  while IFS=$'\t' read -r relative_path mode executable; do
    [[ -n "$relative_path" ]] || continue
    source_file="$source_root/$relative_path"
    destination_file="$staging_root/$relative_path"
    [[ -f "$source_file" && ! -L "$source_file" ]] || {
      echo "immutable watchdog source is missing or not regular: $relative_path" >&2
      return 1
    }
    mkdir -p "$(dirname "$destination_file")"
    cp -p "$source_file" "$destination_file"
    chmod "$mode" "$destination_file"
    file_sha="$(hepta_immutable_watchdog_sha256_file "$destination_file")"
    file_size="$(stat -f '%z' "$destination_file")"
    records="$(
      jq -cn \
        --argjson records "$records" \
        --arg relative_path "$relative_path" \
        --arg sha256 "$file_sha" \
        --argjson size_bytes "$file_size" \
        --arg mode "$mode" \
        --argjson executable "$executable" \
        '$records + [{
          relative_path:$relative_path,
          sha256:$sha256,
          size_bytes:$size_bytes,
          mode:$mode,
          executable:$executable
        }]'
    )"
  done < <(hepta_immutable_watchdog_closure_spec)

  find "$staging_root/scripts" "$staging_root/docs" -type d -exec chmod 0555 {} +

  local aggregate_sha
  aggregate_sha="$(
    printf '%s\n' "$records" \
      | hepta_immutable_watchdog_records_aggregate_sha256
  )"
  jq -cn \
    --arg schema "$HEPTA_IMMUTABLE_WATCHDOG_CLOSURE_SCHEMA" \
    --arg source_commit "$source_commit" \
    --argjson source_bound "$source_bound" \
    --arg entrypoint "$HEPTA_IMMUTABLE_WATCHDOG_ENTRYPOINT" \
    --arg product_boundary "$HEPTA_IMMUTABLE_WATCHDOG_PRODUCT_BOUNDARY" \
    --arg verify_tool "$HEPTA_IMMUTABLE_WATCHDOG_VERIFY_TOOL" \
    --arg aggregate_sha256 "$aggregate_sha" \
    --argjson records "$records" \
    '{
      schema:$schema,
      immutable_release_bound:true,
      source:{
        commit:(if $source_bound then $source_commit else null end),
        commit_bound:$source_bound
      },
      entrypoint:$entrypoint,
      product_boundary:$product_boundary,
      verify_tool:$verify_tool,
      aggregate_sha256:$aggregate_sha256,
      file_count:($records|length),
      files:$records
    }'
}

hepta_immutable_watchdog_verify() {
  local release_root="$1"
  local source_commit="$2"
  local source_bound="$3"
  local closure_json="$4"
  local expected_paths
  expected_paths="$(hepta_immutable_watchdog_expected_paths_json "$source_commit")"

  jq -e \
    --arg schema "$HEPTA_IMMUTABLE_WATCHDOG_CLOSURE_SCHEMA" \
    --arg source_commit "$source_commit" \
    --argjson source_bound "$source_bound" \
    --arg entrypoint "$HEPTA_IMMUTABLE_WATCHDOG_ENTRYPOINT" \
    --arg product_boundary "$HEPTA_IMMUTABLE_WATCHDOG_PRODUCT_BOUNDARY" \
    --arg verify_tool "$HEPTA_IMMUTABLE_WATCHDOG_VERIFY_TOOL" \
    --argjson expected_paths "$expected_paths" \
    '
      .schema == $schema
      and .immutable_release_bound == true
      and .source.commit_bound == $source_bound
      and .source.commit == (if $source_bound then $source_commit else null end)
      and .entrypoint == $entrypoint
      and .product_boundary == $product_boundary
      and .verify_tool == $verify_tool
      and (.aggregate_sha256 | test("^[0-9a-f]{64}$"))
      and .file_count == ($expected_paths|length)
      and (.files|map(.relative_path)) == $expected_paths
      and (.files|all(
        (keys|sort) == ["executable","mode","relative_path","sha256","size_bytes"]
        and (.sha256|test("^[0-9a-f]{64}$"))
        and (.size_bytes|type == "number" and . > 0)
        and (.mode == "0444" or .mode == "0555")
        and (.executable|type == "boolean")
        and ((.executable and .mode == "0555") or ((.executable|not) and .mode == "0444"))
      ))
    ' <<<"$closure_json" >/dev/null || {
    echo "immutable watchdog closure shape or exact file set is invalid" >&2
    return 1
  }

  local relative_path expected_sha expected_size expected_mode expected_executable
  local candidate actual_sha actual_size actual_mode
  while IFS=$'\t' read -r relative_path expected_sha expected_size expected_mode expected_executable; do
    candidate="$release_root/$relative_path"
    [[ -f "$candidate" && ! -L "$candidate" ]] || {
      echo "immutable watchdog file is missing, non-regular, or a symlink: $relative_path" >&2
      return 1
    }
    actual_sha="$(hepta_immutable_watchdog_sha256_file "$candidate")"
    actual_size="$(stat -f '%z' "$candidate")"
    actual_mode="$(stat -f '%Lp' "$candidate")"
    [[ "$actual_sha" == "$expected_sha" \
      && "$actual_size" == "$expected_size" \
      && "0$actual_mode" == "$expected_mode" ]] || {
      echo "immutable watchdog file evidence mismatch: $relative_path" >&2
      return 1
    }
    if [[ "$expected_executable" == "true" ]]; then
      [[ -x "$candidate" ]] || {
        echo "immutable watchdog executable bit is missing: $relative_path" >&2
        return 1
      }
    else
      [[ ! -x "$candidate" ]] || {
        echo "immutable watchdog data/helper unexpectedly executable: $relative_path" >&2
        return 1
      }
    fi
  done < <(
    jq -r '.files[] | [.relative_path,.sha256,.size_bytes,.mode,.executable] | @tsv' \
      <<<"$closure_json"
  )

  local expected_aggregate actual_aggregate
  expected_aggregate="$(jq -r '.aggregate_sha256' <<<"$closure_json")"
  actual_aggregate="$(
    jq -c '.files' <<<"$closure_json" \
      | hepta_immutable_watchdog_records_aggregate_sha256
  )"
  [[ "$actual_aggregate" == "$expected_aggregate" ]] || {
    echo "immutable watchdog aggregate SHA-256 mismatch" >&2
    return 1
  }
}
