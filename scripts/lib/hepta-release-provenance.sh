#!/usr/bin/env bash
HEPTA_BUILD_PROVENANCE_SCHEMA="hepta_build_provenance_v1"
HEPTA_BOUND_GATE_RECEIPTS_SCHEMA="hepta_preflight_bound_gate_receipts_v1"
hepta_release_sha256_file() {
  shasum -a 256 "$1" | awk '{print $1}'
}
hepta_release_sha256_text() {
  shasum -a 256 | awk '{print $1}'
}
hepta_release_canonical_preflight_artifact_path() {
  local target_dir="$1"
  local override="$2"
  [[ -n "$target_dir" && "$target_dir" == /* && "$target_dir" != *$'\n'* ]] || {
    echo "release preflight cargo target directory must be an absolute single-line path" >&2
    return 1
  }
  [[ -z "$override" ]] || {
    echo "release preflight artifact override is forbidden; the canonical cargo target artifact is mandatory" >&2
    return 1
  }
  printf '%s/release/hepta\n' "${target_dir%/}"
}
hepta_release_canonical_json() {
  jq -cS .
}
hepta_release_canonical_json_sha256() {
  hepta_release_canonical_json | hepta_release_sha256_text
}
hepta_release_canonicalize_json_file() {
  local file="$1"
  local staged
  [[ -f "$file" ]] || {
    echo "missing JSON receipt: $file" >&2
    return 1
  }
  staged="$(mktemp "${TMPDIR:-/tmp}/hepta-canonical-json.XXXXXX")"
  if ! jq -cS . "$file" >"$staged"; then
    rm -f "$staged"
    return 1
  fi
  mv -f "$staged" "$file"
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
  jq -cn --arg schema_version "$HEPTA_BUILD_PROVENANCE_SCHEMA" \
    --arg source_commit "$source_commit" --arg toolchain_aggregate_sha256 "$toolchain_aggregate_sha" \
    --arg dependency_aggregate_sha256 "$dependency_aggregate_sha" --arg artifact_sha256 "$artifact_sha" \
    --argjson toolchain_inputs "$toolchain_inputs" --argjson rustc_records "$rustc_records" \
    --argjson dependency_inputs "$dependency_inputs" \
    '{schema_version:$schema_version,source:{commit:$source_commit,commit_bound:true},toolchain:{bound:true,aggregate_sha256:$toolchain_aggregate_sha256,manifest_inputs:$toolchain_inputs,rustc_verbose_inputs:$rustc_records},dependencies:{bound:true,aggregate_sha256:$dependency_aggregate_sha256,lock_inputs:$dependency_inputs},artifact:{bound:true,sha256:$artifact_sha256}}'
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
      ((keys - ["artifact","bound_gate_receipts","dependencies","deployment_consistency","preflight_profiles","schema_version","source","toolchain","watchdog_gate_mode"]) | length) == 0
      and .schema_version == $schema
      and .source == {commit:$source_commit,commit_bound:true}
      and (.toolchain | keys | sort) == ["aggregate_sha256","bound","manifest_inputs","rustc_verbose_inputs"]
      and .toolchain.bound == true
      and (.toolchain.aggregate_sha256 | test("^[0-9a-f]{64}$"))
      and (.toolchain.manifest_inputs | map(.path)) == ["codex-rs/rust-toolchain.toml","apps/hepta-native/rust-toolchain.toml"]
      and (.toolchain.manifest_inputs | all((keys | sort) == ["path","sha256"] and (.sha256 | test("^[0-9a-f]{64}$"))))
      and (.toolchain.rustc_verbose_inputs | map(.workspace)) == ["codex-rs","apps/hepta-native"]
      and (.toolchain.rustc_verbose_inputs | all((keys | sort) == ["rustc_verbose_sha256","workspace"] and (.rustc_verbose_sha256 | test("^[0-9a-f]{64}$"))))
      and (.dependencies | keys | sort) == ["aggregate_sha256","bound","lock_inputs"]
      and .dependencies.bound == true
      and (.dependencies.aggregate_sha256 | test("^[0-9a-f]{64}$"))
      and (.dependencies.lock_inputs | map(.path)) == ["codex-rs/Cargo.lock","apps/hepta-native/Cargo.lock"]
      and (.dependencies.lock_inputs | all((keys | sort) == ["path","sha256"] and (.sha256 | test("^[0-9a-f]{64}$"))))
      and .artifact == {bound:true,sha256:$artifact_sha}
    ' <<<"$provenance_json" >/dev/null || {
      echo "build provenance shape or exact input set is invalid" >&2
      return 1
    }
  local manifest_inputs_sha rustc_inputs_sha expected_toolchain_sha
  local expected_dependency_sha actual_toolchain_sha actual_dependency_sha
  manifest_inputs_sha="$(
    jq -c '.toolchain.manifest_inputs' <<<"$provenance_json" \
      | hepta_release_records_aggregate_sha256
  )"
  rustc_inputs_sha="$(
    jq -r '.toolchain.rustc_verbose_inputs[] | [.workspace, .rustc_verbose_sha256] | @tsv' \
      <<<"$provenance_json" \
      | hepta_release_sha256_text
  )"
  expected_toolchain_sha="$(
    printf '%s\n%s\n' "$manifest_inputs_sha" "$rustc_inputs_sha" \
      | hepta_release_sha256_text
  )"
  expected_dependency_sha="$(
    jq -c '.dependencies.lock_inputs' <<<"$provenance_json" \
      | hepta_release_records_aggregate_sha256
  )"
  actual_toolchain_sha="$(jq -r '.toolchain.aggregate_sha256' <<<"$provenance_json")"
  actual_dependency_sha="$(jq -r '.dependencies.aggregate_sha256' <<<"$provenance_json")"
  [[ "$actual_toolchain_sha" == "$expected_toolchain_sha" ]] || {
    echo "build provenance toolchain aggregate SHA-256 mismatch" >&2
    return 1
  }
  [[ "$actual_dependency_sha" == "$expected_dependency_sha" ]] || {
    echo "build provenance dependency aggregate SHA-256 mismatch" >&2
    return 1
  }
}
hepta_release_validate_bound_gate_receipts_json() {
  local receipts_json="$1"
  local source_commit="$2"
  local codex_cargo_lock_sha="$3"
  local native_cargo_lock_sha="$4"
  local dependency_contract_sha="$5"
  [[ "$source_commit" =~ ^[0-9a-f]{40}$ \
    && "$codex_cargo_lock_sha" =~ ^[0-9a-f]{64}$ \
    && "$native_cargo_lock_sha" =~ ^[0-9a-f]{64}$ \
    && "$dependency_contract_sha" =~ ^[0-9a-f]{64}$ ]] || {
    echo "bound gate receipt validation requires canonical source, contract, and Cargo.lock hashes" >&2
    return 1
  }
  jq -e \
    --arg schema "$HEPTA_BOUND_GATE_RECEIPTS_SCHEMA" \
    --arg source_commit "$source_commit" \
    --arg codex_cargo_lock_sha "$codex_cargo_lock_sha" \
    --arg native_cargo_lock_sha "$native_cargo_lock_sha" \
    --arg dependency_contract_sha "$dependency_contract_sha" \
    'def finding: (keys|sort)==["advisory_id","kind","package","version"] and (.advisory_id==null or (.advisory_id|test("^RUSTSEC-[0-9]{4}-[0-9]{4}$"))) and (.kind|IN("vulnerability","unmaintained","unsound","yanked")) and (.package|type=="string" and length>0) and (.version|type=="string" and length>0);
      (.gates | INDEX(.id)) as $g
      | .schema == $schema
      and (. | keys | sort) == ["gates","schema","source"]
      and .source == {commit:$source_commit,commit_bound:true}
      and (.gates | type == "array" and length == 3)
      and ((.gates | map(.id) | sort) == ["dependency-security","gate-compat-debt","native-ingress-composition"])
      and ((.gates | map(.id) | unique | length) == 3)
      and (.gates | all((. | keys | sort) == ["id","receipt","receipt_sha256","self_test_receipt","self_test_sha256"]
        and (.receipt_sha256 | test("^[0-9a-f]{64}$"))
        and (.self_test_sha256 | test("^[0-9a-f]{64}$"))
        and .receipt.status == "ready"
        and .self_test_receipt.status == "ready"))
      and $g["native-ingress-composition"].receipt == {"authenticated_preference_context":"attached","classification":"typed_lifecycle_registry","control_ui_route_specs":283,"default_off_runtime_kernel_mutations":2,"get_bounded_ephemeral_verifications":3,"get_external_or_durable_effect_surfaces":0,"legacy_credentialed_network_reads":1,"mutation_routes":"plan_only_unless_specialized","operator_telegram_pipelines":1,"schema":"hepta.native-ingress-composition.v2","status":"ready","telegram_durable_intent_owner":"TelegramPipelineAuthority","unknown_ingress":"fail_closed"}
      and $g["native-ingress-composition"].self_test_receipt == {schema:"hepta.native-ingress-composition-self-test.v2",status:"ready",negative_fixtures:14}
      and ($g["dependency-security"].receipt as $d
        | $d.graphs as $graphs
        | ($d | keys | sort) == ["advisory_database_commit","contract_sha256","deployment","graphs","network_used_during_scan","publication","raw_unsafe_warning_count","raw_vulnerability_count","raw_warning_count","schema","status","tools","unsafe_warning_count","unsafe_warning_counts","unsafe_warning_exception_count","unsafe_warning_exceptions","vulnerability_count","vulnerability_exception_count","vulnerability_exceptions","warning_count"]
        and $d.schema == "hepta_dependency_security_receipt_v1"
        and $d.tools == {cargo_audit:"cargo-audit 0.22.2",cargo_deny:"cargo-deny 0.20.2"}
        and $d.advisory_database_commit == "1abf7a8c1822223a38e99f652bc232071c44a86d"
        and $d.contract_sha256 == $dependency_contract_sha
        and ($d.graphs | map(.id)) == ["codex-workspace","native-app"]
        and ($d.graphs | all((keys | sort) == ["cargo_audit_exit_code","cargo_audit_sha256","cargo_deny_error_count","cargo_deny_exit_code","cargo_deny_sha256","cargo_deny_warning_count","cargo_lock_path","cargo_lock_sha256","id","manifest_path","raw_unsafe_warning_count","raw_unsafe_warnings","raw_vulnerabilities","raw_vulnerability_count","raw_warning_count","status","unsafe_warning_count","unsafe_warning_counts","unsafe_warning_exception_count","unsafe_warning_exceptions","unsafe_warnings","vulnerabilities","vulnerability_count","vulnerability_exception_count","vulnerability_exceptions","warning_count"]
          and .status == "ready" and (.cargo_audit_sha256 | test("^[0-9a-f]{64}$")) and (.cargo_deny_sha256 | test("^[0-9a-f]{64}$"))
          and .cargo_audit_exit_code == (if .raw_vulnerability_count>0 then 1 else 0 end) and .cargo_deny_exit_code == 0 and .cargo_deny_error_count == 0 and (.cargo_deny_warning_count | type == "number" and . >= 0)
          and (.raw_vulnerabilities|all(finding and .kind=="vulnerability")) and (.raw_unsafe_warnings|all(finding and .kind!="vulnerability"))
          and .raw_vulnerabilities == .vulnerability_exceptions and .vulnerabilities == [] and .raw_unsafe_warnings == .unsafe_warning_exceptions and .unsafe_warnings == []
          and (.vulnerability_exceptions|all(.advisory_id as $id|["RUSTSEC-2026-0118","RUSTSEC-2026-0119","RUSTSEC-2026-0194","RUSTSEC-2026-0195"]|index($id)!=null))
          and (.unsafe_warning_exceptions|all(.advisory_id as $id|["RUSTSEC-2023-0089","RUSTSEC-2024-0320","RUSTSEC-2024-0388","RUSTSEC-2024-0436","RUSTSEC-2025-0057","RUSTSEC-2025-0141"]|index($id)!=null))
          and .raw_vulnerability_count == (.raw_vulnerabilities|length) and .vulnerability_exception_count == (.vulnerability_exceptions|length) and .raw_warning_count == (.raw_unsafe_warnings|length) and .raw_unsafe_warning_count == .raw_warning_count and .unsafe_warning_exception_count == (.unsafe_warning_exceptions|length)
          and .vulnerability_count == 0 and .warning_count == 0 and .unsafe_warning_count == 0
          and .unsafe_warning_counts == {unmaintained:0,unsound:0,yanked:0}))
        and ($d.graphs[0] | {id,manifest_path,cargo_lock_path,cargo_lock_sha256}) == {id:"codex-workspace",manifest_path:"codex-rs/Cargo.toml",cargo_lock_path:"codex-rs/Cargo.lock",cargo_lock_sha256:$codex_cargo_lock_sha}
        and ($d.graphs[1] | {id,manifest_path,cargo_lock_path,cargo_lock_sha256}) == {id:"native-app",manifest_path:"apps/hepta-native/Cargo.toml",cargo_lock_path:"apps/hepta-native/Cargo.lock",cargo_lock_sha256:$native_cargo_lock_sha}
        and $d.raw_vulnerability_count == ([$graphs[].raw_vulnerability_count]|add) and $d.vulnerability_exception_count == ([$graphs[].vulnerability_exception_count]|add) and $d.vulnerability_exceptions == [$graphs[] as $graph|$graph.vulnerability_exceptions[]|.+{graph_id:$graph.id}]
        and $d.raw_warning_count == ([$graphs[].raw_warning_count]|add) and $d.raw_unsafe_warning_count == $d.raw_warning_count and $d.unsafe_warning_exception_count == ([$graphs[].unsafe_warning_exception_count]|add) and $d.unsafe_warning_exceptions == [$graphs[] as $graph|$graph.unsafe_warning_exceptions[]|.+{graph_id:$graph.id}]
        and $d.vulnerability_count == 0 and $d.warning_count == 0 and $d.unsafe_warning_count == 0 and $d.unsafe_warning_counts == {unmaintained:0,unsound:0,yanked:0}
        and $d.network_used_during_scan == false and $d.deployment == false and $d.publication == false)
      and $g["dependency-security"].self_test_receipt == {schema:"hepta_dependency_security_self_test_v1",status:"ready",negative_fixtures:12,vendored_source_negative_fixtures:5}
      and $g["gate-compat-debt"].receipt == {"captured_shell_pair_count":1239,"compatibility_payload_count":2512,"counts_may_only_improve":true,"declarative_pair_count":35,"entrypoint_manifest_verified":true,"legacy_workgraph_pair_count":8,"pair_count":1282,"payload_bundle_verified":true,"physical_entrypoint_count":165,"schema":"hepta_gate_compat_debt_receipt_v2","status":"ready","unregistered_payload_count":26,"virtual_entrypoint_count":2399}
      and $g["gate-compat-debt"].self_test_receipt == {schema:"hepta_gate_compat_debt_self_test_v2",status:"ready",negative_fixtures:3}
    ' <<<"$receipts_json" >/dev/null || {
      echo "bound gate receipt structure or release claims are invalid" >&2
      return 1
    }
  local gate_id field sha_field expected_sha actual_sha raw_findings_sha
  raw_findings_sha="$(jq -cS '.gates[]|select(.id=="dependency-security")|.receipt.graphs|map({key:.id,value:{raw_vulnerabilities,raw_unsafe_warnings}})|from_entries' <<<"$receipts_json" | hepta_release_sha256_text)"
  [[ "$raw_findings_sha" == "79875483fa5a377ce389e7775556043c0dd90a319a9ecdc104c18bd4c8f0fe73" ]] || {
    echo "bound dependency receipt raw findings differ from the pinned lock/database contract" >&2
    return 1
  }
  for gate_id in native-ingress-composition dependency-security gate-compat-debt; do
    for field in receipt self_test_receipt; do
      case "$field" in
        receipt) sha_field="receipt_sha256" ;;
        self_test_receipt) sha_field="self_test_sha256" ;;
        *) return 1 ;;
      esac
      expected_sha="$(
        jq -r \
          --arg id "$gate_id" \
          --arg field "$sha_field" \
          '.gates[] | select(.id == $id) | .[$field]' \
          <<<"$receipts_json"
      )"
      actual_sha="$(
        jq -cS --arg id "$gate_id" --arg field "$field" \
          '.gates[] | select(.id == $id) | .[$field]' \
          <<<"$receipts_json" \
          | hepta_release_sha256_text
      )"
      [[ "$expected_sha" == "$actual_sha" ]] || {
        echo "bound gate embedded receipt SHA-256 mismatch: $gate_id/$field" >&2
        return 1
      }
    done
  done
}
hepta_release_fixture_bound_gate_receipts_json() {
  local root="$1"
  local provenance_json="$2"
  local source_commit="$3"
  local codex_lock_sha native_lock_sha contract_sha
  local ingress ingress_self dependency dependency_self compat compat_self
  local ingress_sha ingress_self_sha dependency_sha dependency_self_sha compat_sha compat_self_sha
  codex_lock_sha="$(
    jq -er '
      [.dependencies.lock_inputs[] | select(.path == "codex-rs/Cargo.lock") | .sha256]
      | if length == 1 then .[0] else error("missing codex lock") end
    ' <<<"$provenance_json"
  )"
  native_lock_sha="$(
    jq -er '
      [.dependencies.lock_inputs[] | select(.path == "apps/hepta-native/Cargo.lock") | .sha256]
      | if length == 1 then .[0] else error("missing native lock") end
    ' <<<"$provenance_json"
  )"
  contract_sha="$(jq -cS . "$root/scripts/hepta-dependency-security-v1.json" | hepta_release_sha256_text)"
  ingress='{"authenticated_preference_context":"attached","classification":"typed_lifecycle_registry","control_ui_route_specs":283,"default_off_runtime_kernel_mutations":2,"get_bounded_ephemeral_verifications":3,"get_external_or_durable_effect_surfaces":0,"legacy_credentialed_network_reads":1,"mutation_routes":"plan_only_unless_specialized","operator_telegram_pipelines":1,"schema":"hepta.native-ingress-composition.v2","status":"ready","telegram_durable_intent_owner":"TelegramPipelineAuthority","unknown_ingress":"fail_closed"}'
  ingress_self='{"negative_fixtures":14,"schema":"hepta.native-ingress-composition-self-test.v2","status":"ready"}'
  dependency="$(
    jq -cn --arg codex_lock_sha "$codex_lock_sha" --arg native_lock_sha "$native_lock_sha" --arg contract_sha "$contract_sha" \
      'def cv:[{advisory_id:"RUSTSEC-2026-0119",kind:"vulnerability",package:"hickory-proto",version:"0.25.2"},{advisory_id:"RUSTSEC-2026-0118",kind:"vulnerability",package:"hickory-proto",version:"0.25.2"},{advisory_id:"RUSTSEC-2026-0195",kind:"vulnerability",package:"quick-xml",version:"0.39.4"},{advisory_id:"RUSTSEC-2026-0194",kind:"vulnerability",package:"quick-xml",version:"0.39.4"}];def cw:[{advisory_id:"RUSTSEC-2023-0089",kind:"unmaintained",package:"atomic-polyfill",version:"1.0.3"},{advisory_id:"RUSTSEC-2025-0141",kind:"unmaintained",package:"bincode",version:"1.3.3"},{advisory_id:"RUSTSEC-2024-0388",kind:"unmaintained",package:"derivative",version:"2.2.0"},{advisory_id:"RUSTSEC-2025-0057",kind:"unmaintained",package:"fxhash",version:"0.2.1"},{advisory_id:"RUSTSEC-2024-0436",kind:"unmaintained",package:"paste",version:"1.0.15"},{advisory_id:"RUSTSEC-2024-0320",kind:"unmaintained",package:"yaml-rust",version:"0.4.5"}];def nv:[];def nw:[];def graph($id;$manifest;$lock;$sha;$audit;$deny;$v;$w):{id:$id,status:"ready",manifest_path:$manifest,cargo_lock_path:$lock,cargo_lock_sha256:$sha,cargo_audit_sha256:$audit,cargo_deny_sha256:$deny,cargo_audit_exit_code:(if ($v|length)>0 then 1 else 0 end),cargo_deny_exit_code:0,cargo_deny_error_count:0,cargo_deny_warning_count:0,raw_vulnerability_count:($v|length),raw_vulnerabilities:$v,vulnerability_exception_count:($v|length),vulnerability_exceptions:$v,vulnerabilities:[],vulnerability_count:0,raw_warning_count:($w|length),raw_unsafe_warning_count:($w|length),raw_unsafe_warnings:$w,unsafe_warning_exception_count:($w|length),unsafe_warning_exceptions:$w,unsafe_warnings:[],warning_count:0,unsafe_warning_count:0,unsafe_warning_counts:{unmaintained:0,unsound:0,yanked:0}};[graph("codex-workspace";"codex-rs/Cargo.toml";"codex-rs/Cargo.lock";$codex_lock_sha;("1"*64);("2"*64);cv;cw),graph("native-app";"apps/hepta-native/Cargo.toml";"apps/hepta-native/Cargo.lock";$native_lock_sha;("3"*64);("4"*64);nv;nw)] as $g|{schema:"hepta_dependency_security_receipt_v1",status:"ready",tools:{cargo_audit:"cargo-audit 0.22.2",cargo_deny:"cargo-deny 0.20.2"},advisory_database_commit:"1abf7a8c1822223a38e99f652bc232071c44a86d",contract_sha256:$contract_sha,graphs:$g,raw_vulnerability_count:([$g[].raw_vulnerability_count]|add),vulnerability_exception_count:([$g[].vulnerability_exception_count]|add),vulnerability_exceptions:[$g[] as $x|$x.vulnerability_exceptions[]|.+{graph_id:$x.id}],vulnerability_count:0,raw_warning_count:([$g[].raw_warning_count]|add),warning_count:0,raw_unsafe_warning_count:([$g[].raw_unsafe_warning_count]|add),unsafe_warning_exception_count:([$g[].unsafe_warning_exception_count]|add),unsafe_warning_exceptions:[$g[] as $x|$x.unsafe_warning_exceptions[]|.+{graph_id:$x.id}],unsafe_warning_count:0,unsafe_warning_counts:{unmaintained:0,unsound:0,yanked:0},network_used_during_scan:false,deployment:false,publication:false}'
  )"
  dependency_self='{"negative_fixtures":12,"schema":"hepta_dependency_security_self_test_v1","status":"ready","vendored_source_negative_fixtures":5}'
  compat='{"captured_shell_pair_count":1239,"compatibility_payload_count":2512,"counts_may_only_improve":true,"declarative_pair_count":35,"entrypoint_manifest_verified":true,"legacy_workgraph_pair_count":8,"pair_count":1282,"payload_bundle_verified":true,"physical_entrypoint_count":165,"schema":"hepta_gate_compat_debt_receipt_v2","status":"ready","unregistered_payload_count":26,"virtual_entrypoint_count":2399}'
  compat_self='{"negative_fixtures":3,"schema":"hepta_gate_compat_debt_self_test_v2","status":"ready"}'
  ingress_sha="$(hepta_release_canonical_json_sha256 <<<"$ingress")"
  ingress_self_sha="$(hepta_release_canonical_json_sha256 <<<"$ingress_self")"
  dependency_sha="$(hepta_release_canonical_json_sha256 <<<"$dependency")"
  dependency_self_sha="$(hepta_release_canonical_json_sha256 <<<"$dependency_self")"
  compat_sha="$(hepta_release_canonical_json_sha256 <<<"$compat")"
  compat_self_sha="$(hepta_release_canonical_json_sha256 <<<"$compat_self")"
  jq -cSn \
    --arg source_commit "$source_commit" \
    --argjson ingress "$ingress" --arg ingress_sha "$ingress_sha" \
    --argjson ingress_self "$ingress_self" --arg ingress_self_sha "$ingress_self_sha" \
    --argjson dependency "$dependency" --arg dependency_sha "$dependency_sha" \
    --argjson dependency_self "$dependency_self" --arg dependency_self_sha "$dependency_self_sha" \
    --argjson compat "$compat" --arg compat_sha "$compat_sha" \
    --argjson compat_self "$compat_self" --arg compat_self_sha "$compat_self_sha" \
    '{schema:"hepta_preflight_bound_gate_receipts_v1",source:{commit:$source_commit,commit_bound:true},gates:[{id:"native-ingress-composition",receipt:$ingress,receipt_sha256:$ingress_sha,self_test_receipt:$ingress_self,self_test_sha256:$ingress_self_sha},{id:"dependency-security",receipt:$dependency,receipt_sha256:$dependency_sha,self_test_receipt:$dependency_self,self_test_sha256:$dependency_self_sha},{id:"gate-compat-debt",receipt:$compat,receipt_sha256:$compat_sha,self_test_receipt:$compat_self,self_test_sha256:$compat_self_sha}]}'
}
hepta_release_fixture_complete_provenance_json() {
  local root="$1"
  local source_commit="$2"
  local artifact="$3"
  local provenance receipts
  provenance="$(hepta_release_build_provenance_json "$root" "$source_commit" "$artifact")"
  receipts="$(hepta_release_fixture_bound_gate_receipts_json "$root" "$provenance" "$source_commit")"
  jq -cS --argjson receipts "$receipts" \
    '. + {preflight_profiles:{backend:true,native:true,release:true},watchdog_gate_mode:"fixture",bound_gate_receipts:$receipts,deployment_consistency:{checked_during_candidate_preflight:false,required_before_activation:true}}' <<<"$provenance"
}
hepta_release_write_fixture_preflight_log() {
  local target="$1"
  local source_commit="$2"
  local provenance_json="$3"
  local canonical_provenance provenance_sha artifact_sha final_receipt
  canonical_provenance="$(jq -cS . <<<"$provenance_json")"
  provenance_sha="$(printf '%s' "$canonical_provenance" | hepta_release_sha256_text)"
  artifact_sha="$(jq -r '.artifact.sha256' <<<"$canonical_provenance")"
  final_receipt="$(
    jq -cSn --arg source_commit "$source_commit" --arg artifact_sha256 "$artifact_sha" \
      --arg build_provenance_sha256 "$provenance_sha" \
      '{schema:"hepta_preflight_final_receipt_v1",status:"passed",source_commit:$source_commit,artifact_sha256:$artifact_sha256,build_provenance_sha256:$build_provenance_sha256}'
  )"
  printf '%s\n' \
    "[hepta-preflight-resume] head=${source_commit:0:10} attempt=1 start_line=1 marker=<start> log=$target" \
    "{\"hepta_repo_head\":\"$source_commit\"}" \
    "[hepta-preflight-provenance] $canonical_provenance" \
    "[hepta-preflight-final] $final_receipt" \
    "Hepta preflight passed" >"$target"
}
