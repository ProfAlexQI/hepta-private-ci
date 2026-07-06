#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${HEPTA_RECONSTRUCTION_REPO_ROOT:-$(cd "$(dirname "$0")/.." && pwd -P)}"
SESSION_ROOT="${HEPTA_AGENT_SESSION_ROOT:-$HOME/.openclaw/agents/hepta/agent/codex-home/sessions}"
REPO_PREFIX="$REPO_ROOT/"

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required for the Hepta session reconstruction report" >&2
  exit 127
fi

git_branch="$(git -C "$REPO_ROOT" rev-parse --abbrev-ref HEAD 2>/dev/null || printf 'unknown')"
git_status_short="$(git -C "$REPO_ROOT" status --short --untracked-files=all 2>/dev/null || true)"
git_status_clean=false
if [[ -z "$git_status_short" ]]; then
  git_status_clean=true
fi

if [[ ! -d "$SESSION_ROOT" ]]; then
  jq -n \
    --arg runtime "hepta" \
    --arg status "blocked" \
    --arg repo_root "$REPO_ROOT" \
    --arg session_root "$SESSION_ROOT" \
    --arg git_branch "$git_branch" \
    --argjson git_status_clean "$git_status_clean" \
    '{
      runtime:$runtime,
      surface:"hepta_systems_session_reconstruction_map",
      status:$status,
      repo_root:$repo_root,
      session_root:$session_root,
      git_branch:$git_branch,
      git_status_clean:$git_status_clean,
      side_effect_free:true,
      report_only:true,
      replay_applied:false,
      blocker:"missing_agent_session_root"
    }'
  exit 0
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

calls_jsonl="$tmpdir/calls.jsonl"
entries_jsonl="$tmpdir/entries.jsonl"
statuses_jsonl="$tmpdir/statuses.jsonl"
: >"$calls_jsonl"
: >"$entries_jsonl"
: >"$statuses_jsonl"

session_file_count="$(
  find "$SESSION_ROOT" -type f -name '*.jsonl' 2>/dev/null | wc -l | tr -d ' '
)"

while IFS= read -r -d '' session_file; do
  jq -c \
    --arg repo_prefix "$REPO_PREFIX" \
    --arg session_root "$SESSION_ROOT" \
    '
      select(
        .type == "response_item"
        and .payload.type == "custom_tool_call"
        and .payload.name == "apply_patch"
        and ((.payload.input // "") | contains($repo_prefix))
      )
      | {
          timestamp,
          session:(input_filename | ltrimstr($session_root + "/")),
          call_id:.payload.call_id,
          patch_line_count:(.payload.input | split("\n") | length),
          touched_file_count:(
            .payload.input
            | split("\n")
            | map(select(test("^\\*\\*\\* (Add|Update|Delete) File: ")))
            | length
          )
        }
    ' "$session_file" >>"$calls_jsonl"

  jq -c \
    --arg repo_prefix "$REPO_PREFIX" \
    --arg session_root "$SESSION_ROOT" \
    '
      def file_entry:
        if test("^\\*\\*\\* Add File: ") then
          {operation:"add", path:(sub("^\\*\\*\\* Add File: "; ""))}
        elif test("^\\*\\*\\* Update File: ") then
          {operation:"update", path:(sub("^\\*\\*\\* Update File: "; ""))}
        elif test("^\\*\\*\\* Delete File: ") then
          {operation:"delete", path:(sub("^\\*\\*\\* Delete File: "; ""))}
        else
          empty
        end;

      select(
        .type == "response_item"
        and .payload.type == "custom_tool_call"
        and .payload.name == "apply_patch"
        and ((.payload.input // "") | contains($repo_prefix))
      )
      | (.payload.input | split("\n")[] | select(test("^\\*\\*\\* (Add|Update|Delete) File: ")) | file_entry) as $file
      | select($file.path | startswith($repo_prefix))
      | {
          timestamp,
          session:(input_filename | ltrimstr($session_root + "/")),
          call_id:.payload.call_id,
          operation:$file.operation,
          absolute_path:$file.path,
          repo_relative_path:($file.path | ltrimstr($repo_prefix))
        }
    ' "$session_file" >>"$entries_jsonl"
done < <(find "$SESSION_ROOT" -type f -name '*.jsonl' -print0 2>/dev/null)

while IFS= read -r rel_path; do
  [[ -n "$rel_path" ]] || continue
  abs_path="$REPO_ROOT/$rel_path"
  present=false
  tracked=false
  current_status="missing"

  if [[ -e "$abs_path" ]]; then
    present=true
  fi
  if git -C "$REPO_ROOT" ls-files --error-unmatch -- "$rel_path" >/dev/null 2>&1; then
    tracked=true
  fi

  if [[ "$present" == "true" && "$tracked" == "true" ]]; then
    current_status="present_tracked"
  elif [[ "$present" == "true" ]]; then
    current_status="present_untracked"
  fi

  jq -cn \
    --arg path "$rel_path" \
    --arg absolute_path "$abs_path" \
    --arg current_status "$current_status" \
    --argjson present "$present" \
    --argjson tracked "$tracked" \
    '{
      path:$path,
      absolute_path:$absolute_path,
      present:$present,
      tracked:$tracked,
      current_status:$current_status
    }'
done < <(jq -r '.repo_relative_path' "$entries_jsonl" | LC_ALL=C sort -u) >"$statuses_jsonl"

git_status_lines="$(
  printf '%s\n' "$git_status_short" | jq -R -s 'split("\n") | map(select(length > 0))'
)"

jq -n \
  --arg runtime "hepta" \
  --arg surface "hepta_systems_session_reconstruction_map" \
  --arg repo_root "$REPO_ROOT" \
  --arg session_root "$SESSION_ROOT" \
  --arg git_branch "$git_branch" \
  --argjson git_status_clean "$git_status_clean" \
  --argjson git_status_lines "$git_status_lines" \
  --argjson session_file_count "$session_file_count" \
  --slurpfile calls "$calls_jsonl" \
  --slurpfile entries "$entries_jsonl" \
  --slurpfile statuses "$statuses_jsonl" \
  '
    ($statuses | map({key:.path, value:.}) | from_entries) as $status_by_path
    | (
        $entries
        | sort_by(.repo_relative_path)
        | group_by(.repo_relative_path)
        | map({
            path:.[0].repo_relative_path,
            operations:(map(.operation) | unique | sort),
            add_patch_count:(map(select(.operation == "add")) | length),
            update_patch_count:(map(select(.operation == "update")) | length),
            delete_patch_count:(map(select(.operation == "delete")) | length),
            first_timestamp:(map(.timestamp) | min),
            last_timestamp:(map(.timestamp) | max),
            sessions:(map(.session) | unique | sort),
            call_ids:(map(.call_id) | unique | sort),
            current:($status_by_path[.[0].repo_relative_path] // {
              path:.[0].repo_relative_path,
              present:false,
              tracked:false,
              current_status:"unknown"
            })
          })
      ) as $path_summaries
    | ($path_summaries | map({key:.path, value:.}) | from_entries) as $path_by_path
    | [
        {
          id:"plugin_contribution_point_abi",
          description:"typed plugin contribution-point ABI plus report/gate/doc",
          paths:[
            "codex-rs/core-plugins/src/contribution_point_abi.rs",
            "scripts/hepta-systems-plugin-contribution-point-abi-report.sh",
            "scripts/hepta-systems-plugin-contribution-point-abi-gate.sh",
            "docs/architecture/HEPTA_SYSTEMS_PLUGIN_CONTRIBUTION_POINT_ABI_2026-06-19.md"
          ]
        },
        {
          id:"tool_registry_router_lookup_shadow",
          description:"disabled ToolRegistry router lookup shadow surface plus report/gate/doc",
          paths:[
            "codex-rs/tools/src/tool_registry_router_lookup_shadow.rs",
            "scripts/hepta-systems-tool-registry-router-lookup-shadow-report.sh",
            "scripts/hepta-systems-tool-registry-router-lookup-shadow-gate.sh",
            "docs/architecture/HEPTA_SYSTEMS_TOOL_REGISTRY_ROUTER_LOOKUP_SHADOW_2026-06-20.md"
          ]
        },
        {
          id:"workflow_durable_store_replay_proof",
          description:"read-only workflow durable-store replay proof and projection report",
          paths:[
            "codex-rs/hepta-runtime/src/workflow_durable_store_replay_proof.rs",
            "scripts/hepta-systems-workflow-durable-store-replay-proof-report.sh",
            "scripts/hepta-systems-workflow-durable-store-replay-proof-gate.sh",
            "scripts/hepta-systems-work-graph-durable-store-replay-proof-projection-report.sh"
          ]
        },
        {
          id:"compact_capability_matrix",
          description:"compact Hepta systems capability matrix and canonical gate wiring",
          paths:[
            "scripts/hepta-systems-capability-matrix-report.sh",
            "scripts/hepta-systems-capability-matrix-gate.sh",
            "scripts/hepta-systems-canonical-gate.sh",
            "docs/architecture/HEPTA_SYSTEMS_CAPABILITY_MATRIX_2026-06-19.md"
          ]
        },
        {
          id:"scheduler_cutover_preview_chain",
          description:"scheduler cutover preview chain evidence from replay proof toward operator approval",
          path_prefixes:[
            "codex-rs/hepta-runtime/src/work_graph_scheduler_",
            "scripts/hepta-systems-work-graph-scheduler-",
            "docs/architecture/HEPTA_SYSTEMS_CAPABILITY_MATRIX_2026-06-19.md"
          ]
        }
      ] as $anchors
    | (
        $anchors
        | map(
            . as $anchor
            | if has("paths") then
                . + {
                  evidence_paths:(
                    $anchor.paths
                    | map($path_by_path[.] // empty)
                  ),
                  evidence_path_count:(
                    $anchor.paths
                    | map(select($path_by_path[.] != null))
                    | length
                  ),
                  missing_current_path_count:(
                    $anchor.paths
                    | map(select(($path_by_path[.] // {current:{current_status:"missing"}}).current.current_status == "missing"))
                    | length
                  )
                }
              else
                . + {
                  evidence_paths:(
                    $path_summaries
                    | map(select(
                        (.path | startswith($anchor.path_prefixes[0]))
                        or (.path | startswith($anchor.path_prefixes[1]))
                        or (.path == $anchor.path_prefixes[2])
                      ))
                  ),
                  evidence_path_count:(
                    $path_summaries
                    | map(select(
                        (.path | startswith($anchor.path_prefixes[0]))
                        or (.path | startswith($anchor.path_prefixes[1]))
                        or (.path == $anchor.path_prefixes[2])
                      ))
                    | length
                  ),
                  missing_current_path_count:(
                    $path_summaries
                    | map(select(
                        (
                          (.path | startswith($anchor.path_prefixes[0]))
                          or (.path | startswith($anchor.path_prefixes[1]))
                          or (.path == $anchor.path_prefixes[2])
                        )
                        and .current.current_status == "missing"
                      ))
                    | length
                  )
                }
              end
          )
      ) as $recovery_anchors
    | {
        runtime:$runtime,
        surface:$surface,
        status:(if (($calls | length) > 0 and ($entries | length) > 0) then "ready" else "blocked" end),
        repo_root:$repo_root,
        session_root:$session_root,
        git_branch:$git_branch,
        git_status_clean:$git_status_clean,
        git_status_lines:$git_status_lines,
        session_file_count:$session_file_count,
        hepta_apply_patch_call_count:($calls | length),
        hepta_apply_patch_file_entry_count:($entries | length),
        touched_path_count:($path_summaries | length),
        present_path_count:($path_summaries | map(select(.current.current_status != "missing")) | length),
        missing_recoverable_path_count:($path_summaries | map(select(.current.current_status == "missing")) | length),
        add_file_path_count:($path_summaries | map(select(.add_patch_count > 0)) | length),
        update_file_path_count:($path_summaries | map(select(.update_patch_count > 0)) | length),
        delete_file_path_count:($path_summaries | map(select(.delete_patch_count > 0)) | length),
        recovery_anchor_count:($recovery_anchors | length),
        recovery_anchor_evidence_ready_count:($recovery_anchors | map(select(.evidence_path_count > 0)) | length),
        recovery_anchor_missing_current_path_count:($recovery_anchors | map(.missing_current_path_count) | add),
        side_effect_free:true,
        report_only:true,
        replay_applied:false,
        live_mutation_enabled:false,
        patch_replay_enabled:false,
        recommended_next_local_step:"extract_ordered_hepta_patch_queue_and_apply_only_selected_phase0_recovery",
        recovery_anchors:$recovery_anchors,
        top_missing_recoverable_paths:(
          $path_summaries
          | map(select(.current.current_status == "missing"))
          | sort_by(-(.add_patch_count + .update_patch_count), .path)
          | .[0:40]
        ),
        top_present_paths:(
          $path_summaries
          | map(select(.current.current_status != "missing"))
          | sort_by(-(.add_patch_count + .update_patch_count), .path)
          | .[0:20]
        )
      }
  '
