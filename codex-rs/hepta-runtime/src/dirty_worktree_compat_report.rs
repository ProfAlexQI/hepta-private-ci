use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::{Map, Value};

use crate::*;

pub const DIRTY_WORKTREE_TYPED_COMPAT_REPORT_IDS: &[&str] = &[
    "hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-packet-readback",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback",
    "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback",
    "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback",
    "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan",
    "hepta-systems-dirty-worktree-release-boundary-inventory",
    "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording",
    "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-rehearsal",
    "hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot",
    "hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal",
    "hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback",
];

pub const RETIRED_DIRTY_WORKTREE_COMPAT_REPORT_ID: &str = "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-decision-recording-boundary-readback-without-recording";

const INVENTORY_SAMPLE_LIMIT: usize = 80;

const BUSINESS_ENVELOPE_FIELDS: &[&str] = &[
    "runtime",
    "surface",
    "status",
    "gate",
    "schema_version",
    "plugin_id",
    "next_actions",
    "next_migration_step",
    "local_gate",
    "architecture_note",
    "gate_script",
    "report_script",
    "doc_path",
    "side_effect_free",
    "side_effects",
    "recommended_next_gate",
    "production_authority_granted",
    "write_authority_granted",
    "observation_provenance",
    "legacy_business_fields",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DirtyWorktreeObservationEntry {
    pub status_code: String,
    pub index_status: char,
    pub worktree_status: char,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_path: Option<String>,
    pub top_level: String,
    pub change_kind: &'static str,
    pub scope_bucket: &'static str,
    pub operator_visible: bool,
    pub queryable: bool,
    pub release_boundary_route: String,
    pub git_mutation_allowed: bool,
    pub cleanup_allowed: bool,
    pub release_cutover_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirtyWorktreeObservation {
    pub entries: Vec<DirtyWorktreeObservationEntry>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
struct BucketCounts {
    count: usize,
    tracked_count: usize,
    untracked_count: usize,
    hepta_systems_owned_count: usize,
    cross_lane_or_unowned_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct TopLevelBucket {
    top_level: String,
    #[serde(flatten)]
    counts: BucketCounts,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ScopeBucket {
    scope_bucket: &'static str,
    count: usize,
    tracked_count: usize,
    untracked_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ObservedGroup {
    group_type: &'static str,
    source_bucket: String,
    counts: BucketCounts,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct OwnedGroupingEntry {
    group_type: &'static str,
    group_key: String,
    group_route: String,
    source_bucket: String,
    source_entry_count: usize,
    tracked_count: usize,
    untracked_count: usize,
    hepta_systems_owned_count: usize,
    cross_lane_or_unowned_count: usize,
    owner_hint: &'static str,
    review_lane: &'static str,
    freeze_state: &'static str,
    evidence_state: &'static str,
    operator_visible: bool,
    queryable: bool,
    diffable: bool,
    freeze_plan_ready: bool,
    freeze_applied: bool,
    git_mutation_allowed: bool,
    cleanup_allowed: bool,
    evidence_recording_allowed: bool,
    release_cutover_allowed: bool,
}

impl DirtyWorktreeObservation {
    pub fn from_porcelain_v1_z(input: &[u8]) -> Result<Self, String> {
        let mut records = input.split(|byte| *byte == b'\0').peekable();
        let mut entries = Vec::new();
        while let Some(record) = records.next() {
            if record.is_empty() {
                if records.peek().is_none() {
                    break;
                }
                return Err("git porcelain contains an empty record".to_string());
            }
            if record.len() < 4 || record[2] != b' ' {
                return Err("git porcelain record lacks an XY status and path".to_string());
            }
            let index = record[0] as char;
            let worktree = record[1] as char;
            if !valid_status(index) || !valid_status(worktree) {
                return Err("git porcelain record exposes an unknown status code".to_string());
            }
            let path = std::str::from_utf8(&record[3..])
                .map_err(|_| "git porcelain path is not valid UTF-8".to_string())?
                .to_string();
            if path.is_empty() {
                return Err("git porcelain record exposes an empty path".to_string());
            }
            let original_path = if matches!(index, 'R' | 'C') || matches!(worktree, 'R' | 'C') {
                let Some(original) = records.next() else {
                    return Err("renamed git porcelain record lacks its original path".to_string());
                };
                if original.is_empty() {
                    return Err(
                        "renamed git porcelain record exposes an empty original path".to_string(),
                    );
                }
                Some(
                    std::str::from_utf8(original)
                        .map_err(|_| "git porcelain original path is not valid UTF-8".to_string())?
                        .to_string(),
                )
            } else {
                None
            };
            let top_level = path.split('/').next().unwrap_or(path.as_str()).to_string();
            let scope_bucket = if is_hepta_systems_owned(&path) {
                "hepta_systems_owned"
            } else {
                "cross_lane_or_unowned"
            };
            entries.push(DirtyWorktreeObservationEntry {
                status_code: format!("{index}{worktree}"),
                index_status: index,
                worktree_status: worktree,
                release_boundary_route: format!(
                    "readback://release-boundary/dirty-worktree/path/{path}"
                ),
                path,
                original_path,
                top_level,
                change_kind: change_kind(index, worktree),
                scope_bucket,
                operator_visible: true,
                queryable: true,
                git_mutation_allowed: false,
                cleanup_allowed: false,
                release_cutover_allowed: false,
            });
        }
        Ok(Self { entries })
    }

    fn counts(&self) -> DirtyWorktreeReleaseBoundaryInventoryCounts {
        DirtyWorktreeReleaseBoundaryInventoryCounts {
            inventory_entry_count: self.entries.len(),
            tracked_change_count: self
                .entries
                .iter()
                .filter(|entry| !entry.is_untracked())
                .count(),
            untracked_change_count: self
                .entries
                .iter()
                .filter(|entry| entry.is_untracked())
                .count(),
            staged_change_count: self
                .entries
                .iter()
                .filter(|entry| entry.is_staged())
                .count(),
            unstaged_change_count: self
                .entries
                .iter()
                .filter(|entry| entry.is_unstaged())
                .count(),
            modified_change_count: self
                .entries
                .iter()
                .filter(|entry| entry.has_status('M'))
                .count(),
            deleted_change_count: self
                .entries
                .iter()
                .filter(|entry| entry.has_status('D'))
                .count(),
            added_change_count: self
                .entries
                .iter()
                .filter(|entry| entry.has_status('A'))
                .count(),
            renamed_change_count: self
                .entries
                .iter()
                .filter(|entry| entry.has_status('R'))
                .count(),
            unmerged_change_count: self
                .entries
                .iter()
                .filter(|entry| entry.is_unmerged())
                .count(),
            hepta_systems_owned_count: self
                .entries
                .iter()
                .filter(|entry| entry.scope_bucket == "hepta_systems_owned")
                .count(),
            cross_lane_or_unowned_count: self
                .entries
                .iter()
                .filter(|entry| entry.scope_bucket == "cross_lane_or_unowned")
                .count(),
            top_level_bucket_count: self.top_level_buckets().len(),
            sample_entry_count: self.entries.len().min(INVENTORY_SAMPLE_LIMIT),
        }
    }

    fn top_level_buckets(&self) -> Vec<TopLevelBucket> {
        let mut buckets = BTreeMap::<String, BucketCounts>::new();
        for entry in &self.entries {
            add_bucket_entry(buckets.entry(entry.top_level.clone()).or_default(), entry);
        }
        buckets
            .into_iter()
            .map(|(top_level, counts)| TopLevelBucket { top_level, counts })
            .collect()
    }

    fn scope_buckets(&self) -> Vec<ScopeBucket> {
        let mut buckets = BTreeMap::<&'static str, BucketCounts>::new();
        for entry in &self.entries {
            add_bucket_entry(buckets.entry(entry.scope_bucket).or_default(), entry);
        }
        buckets
            .into_iter()
            .map(|(scope_bucket, counts)| ScopeBucket {
                scope_bucket,
                count: counts.count,
                tracked_count: counts.tracked_count,
                untracked_count: counts.untracked_count,
            })
            .collect()
    }

    fn observed_groups(&self) -> Vec<ObservedGroup> {
        let mut groups = self
            .top_level_buckets()
            .into_iter()
            .map(|bucket| ObservedGroup {
                group_type: "top_level",
                source_bucket: bucket.top_level,
                counts: bucket.counts,
            })
            .collect::<Vec<_>>();
        groups.extend(
            self.scope_buckets()
                .into_iter()
                .map(|bucket| ObservedGroup {
                    group_type: "scope",
                    source_bucket: bucket.scope_bucket.to_string(),
                    counts: BucketCounts {
                        count: bucket.count,
                        tracked_count: bucket.tracked_count,
                        untracked_count: bucket.untracked_count,
                        hepta_systems_owned_count: usize::from(
                            bucket.scope_bucket == "hepta_systems_owned",
                        ) * bucket.count,
                        cross_lane_or_unowned_count: usize::from(
                            bucket.scope_bucket == "cross_lane_or_unowned",
                        ) * bucket.count,
                    },
                }),
        );
        groups
    }
}

impl DirtyWorktreeObservationEntry {
    fn is_untracked(&self) -> bool {
        self.index_status == '?' && self.worktree_status == '?'
    }

    fn is_staged(&self) -> bool {
        !self.is_untracked() && self.index_status != ' '
    }

    fn is_unstaged(&self) -> bool {
        !self.is_untracked() && self.worktree_status != ' '
    }

    fn has_status(&self, status: char) -> bool {
        self.index_status == status || self.worktree_status == status
    }

    fn is_unmerged(&self) -> bool {
        matches!(
            (self.index_status, self.worktree_status),
            ('A', 'A')
                | ('D', 'D')
                | ('A', 'U')
                | ('U', 'A')
                | ('D', 'U')
                | ('U', 'D')
                | ('U', 'U')
        )
    }
}

pub fn is_dirty_worktree_typed_compat_report(id: &str) -> bool {
    DIRTY_WORKTREE_TYPED_COMPAT_REPORT_IDS.contains(&id)
}

pub fn dirty_worktree_typed_compat_report(
    id: &str,
    observation: &DirtyWorktreeObservation,
) -> Result<Value, String> {
    dirty_worktree_report_impl(id, observation, false)
}

pub fn retired_dirty_worktree_owner_decision_source_report(
    observation: &DirtyWorktreeObservation,
) -> Result<Value, String> {
    dirty_worktree_report_impl(RETIRED_DIRTY_WORKTREE_COMPAT_REPORT_ID, observation, true)
}

fn dirty_worktree_report_impl(
    id: &str,
    observation: &DirtyWorktreeObservation,
    allow_retired_source: bool,
) -> Result<Value, String> {
    if !is_dirty_worktree_typed_compat_report(id)
        && !(allow_retired_source && id == RETIRED_DIRTY_WORKTREE_COMPAT_REPORT_ID)
    {
        return Err(format!(
            "unknown dirty-worktree typed compatibility report: {id}"
        ));
    }

    let groups = observation.observed_groups();
    let canary = controlled_canary_readiness_plan_report();
    let inventory =
        dirty_worktree_release_boundary_inventory_report_from_counts(&canary, observation.counts());
    let plan =
        dirty_worktree_release_boundary_grouping_freeze_plan_report_from_inventory_and_entries(
            &inventory,
            groups.iter().map(proxy_grouping_entry).collect(),
        );
    let readback =
        dirty_worktree_release_boundary_grouping_freeze_operator_readback_report_from_plan(&plan);
    let strategy =
        dirty_worktree_release_boundary_actionable_clean_worktree_strategy_report_from_readback(
            &readback,
        );
    let packet =
        dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_report_from_strategy(
            &strategy,
        );
    let non_send = dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback_report_from_packet(&packet);
    let git_boundary = dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback_report_from_non_send_readback(&non_send);
    let checklist = dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_report_from_git_boundary(&git_boundary);
    let checklist_readback = dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback_report_from_checklist(&checklist);
    let decision_boundary = dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback_report_from_packet_readback(&checklist_readback);
    let approval_boundary = dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback_report_from_decision_recording_boundary(&decision_boundary);
    let evidence_boundary = dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback_report_from_approval_acceptance_boundary(&approval_boundary);
    let risk = dirty_worktree_release_boundary_release_risk_snapshot_report_from_evidence_recording_boundary(&evidence_boundary);
    let rehearsal = dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal_report_from_release_risk_snapshot(&risk);
    let outcome =
        dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback_report_from_rehearsal(
            &rehearsal,
        );
    let owner_rehearsal =
        dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_report_from_outcome(
            &outcome,
        );
    let owner_outcome = dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_report_from_rehearsal(&owner_rehearsal);
    let owner_packet = dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send_report_from_outcome(&owner_outcome);
    let owner_git_boundary = dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_report_from_operator_packet(&owner_packet);
    let owner_checklist = dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_report_from_git_boundary(&owner_git_boundary);
    let owner_checklist_readback = dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_report_from_checklist(&owner_checklist);
    let owner_decision_boundary = dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_report_from_packet_readback(&owner_checklist_readback);
    let owner_approval_boundary = dirty_worktree_release_boundary_owner_freeze_classification_operator_approval_acceptance_boundary_readback_report_from_decision_recording_boundary(&owner_decision_boundary);
    let owner_evidence_boundary = dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_report_from_approval_acceptance_boundary(&owner_approval_boundary);

    let mut value = match id {
        "hepta-systems-dirty-worktree-release-boundary-inventory" => serialize(&inventory)?,
        "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan" => serialize(&plan)?,
        "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback" => {
            serialize(&readback)?
        }
        "hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy" => {
            serialize(&strategy)?
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet" => {
            serialize(&packet)?
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback" => {
            serialize(&non_send)?
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback" => {
            serialize(&git_boundary)?
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist" => {
            serialize(&checklist)?
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-packet-readback" => {
            serialize(&checklist_readback)?
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback" => {
            serialize(&decision_boundary)?
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback" => {
            serialize(&approval_boundary)?
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback" => {
            serialize(&evidence_boundary)?
        }
        "hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot" => serialize(&risk)?,
        "hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal" => {
            serialize(&rehearsal)?
        }
        "hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback" => {
            serialize(&outcome)?
        }
        "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-rehearsal" => {
            serialize(&owner_rehearsal)?
        }
        "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording" => {
            serialize(&owner_evidence_boundary)?
        }
        RETIRED_DIRTY_WORKTREE_COMPAT_REPORT_ID if allow_retired_source => {
            serialize(&owner_decision_boundary)?
        }
        _ => {
            return Err(format!(
                "unknown dirty-worktree typed compatibility report: {id}"
            ));
        }
    };
    materialize_observed_report(id, &mut value, observation, &groups)?;
    finish_report(value)
}

fn serialize<T: Serialize>(report: &T) -> Result<Value, String> {
    serde_json::to_value(report)
        .map_err(|error| format!("cannot serialize dirty-worktree report: {error}"))
}

fn materialize_observed_report(
    id: &str,
    value: &mut Value,
    observation: &DirtyWorktreeObservation,
    groups: &[ObservedGroup],
) -> Result<(), String> {
    let object = value
        .as_object_mut()
        .ok_or_else(|| "dirty-worktree report must serialize as an object".to_string())?;
    object.insert("lib_export_present".to_string(), Value::Bool(true));
    object.insert(
        "observation_provenance".to_string(),
        serde_json::json!({
            "source_command": "git status --porcelain=v1 -z --untracked-files=all",
            "framing": "nul_delimited_porcelain_v1",
            "parser": "hepta_owned_utf8_fail_closed_v1",
            "explicit_repository_root_required": true,
            "observed_entry_count": observation.entries.len(),
            "observed_group_count": groups.len(),
            "read_only": true
        }),
    );

    if id.ends_with("-inventory") {
        materialize_inventory(object, observation)?;
    } else if id.ends_with("-grouping-freeze-plan") {
        materialize_grouping_plan(object, observation, groups)?;
    } else {
        materialize_entries(id, object, groups)?;
    }

    if id.ends_with("operator-evidence-recording-boundary-readback") {
        object.insert(
            "blockers".to_string(),
            serde_json::json!([
                "evidence_recording_blocked",
                "evidence_persistence_blocked",
                "evidence_receipt_persistence_blocked",
                "approval_request_blocked",
                "approval_acceptance_blocked",
                "approval_recording_blocked",
                "approval_receipt_persistence_blocked",
                "operator_decision_recording_blocked",
                "operator_decision_recording_persistence_blocked",
                "operator_decision_receipt_persistence_blocked",
                "operator_evidence_recording_boundary_readback_persistence_blocked",
                "git_add_blocked",
                "git_index_mutation_blocked",
                "cleanup_and_delete_blocked",
                "strategy_application_blocked",
                "release_cutover_blocked",
                "canary_activation_blocked",
                "live_activation_blocked"
            ]),
        );
    }
    if id.ends_with("owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording") {
        object.insert(
            "evidence_receipt_blocked_count".to_string(),
            Value::from(groups.len()),
        );
    }
    apply_stable_entry_counts(id, object);
    apply_stage_report_compatibility(id, object, !groups.is_empty());
    Ok(())
}

fn apply_stable_entry_counts(id: &str, object: &mut Map<String, Value>) {
    let Some(entries) = object.get("entries").and_then(Value::as_array) else {
        return;
    };
    let mappings: &[(&str, &str)] = match id {
        "hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy" => &[
            ("stable_strategy_key_count", "strategy_key"),
            ("strategy_route_count", "strategy_route"),
        ],
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback" => {
            &[
                ("stable_boundary_key_count", "approval_boundary_key"),
                ("boundary_route_count", "approval_boundary_route"),
            ]
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback" => {
            &[
                ("stable_boundary_key_count", "boundary_key"),
                ("boundary_route_count", "boundary_route"),
            ]
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback"
        | "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording" => {
            &[
                ("stable_boundary_key_count", "evidence_boundary_key"),
                ("boundary_route_count", "evidence_boundary_route"),
            ]
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-packet-readback" => {
            &[
                ("stable_packet_key_count", "packet_key"),
                ("stable_readback_key_count", "readback_key"),
                ("packet_route_count", "packet_route"),
                ("readback_route_count", "readback_route"),
            ]
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist" => {
            &[
                ("stable_checklist_key_count", "checklist_key"),
                ("checklist_route_count", "checklist_route"),
            ]
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback" => {
            &[
                ("stable_readback_key_count", "git_boundary_readback_key"),
                ("readback_route_count", "git_boundary_readback_route"),
            ]
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback" => {
            &[
                ("stable_readback_key_count", "non_send_readback_key"),
                ("readback_route_count", "non_send_readback_route"),
            ]
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet" => {
            &[
                ("stable_packet_key_count", "packet_key"),
                ("packet_route_count", "packet_route"),
            ]
        }
        "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback" => &[
            ("stable_readback_key_count", "readback_key"),
            ("diff_key_count", "diff_key"),
        ],
        "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-rehearsal" => &[
            ("stable_classification_key_count", "classification_key"),
            ("classification_route_count", "classification_route"),
        ],
        "hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot" => &[
            ("stable_snapshot_key_count", "snapshot_key"),
            ("snapshot_route_count", "snapshot_route"),
        ],
        "hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal" => {
            &[
                ("stable_rehearsal_key_count", "rehearsal_key"),
                ("rehearsal_route_count", "rehearsal_route"),
            ]
        }
        "hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback" => &[
            ("stable_outcome_key_count", "outcome_key"),
            ("outcome_route_count", "outcome_route"),
        ],
        _ => &[],
    };
    let counts = mappings
        .iter()
        .map(|(report_field, entry_field)| {
            let count = entries
                .iter()
                .filter_map(|entry| entry.get(*entry_field).and_then(Value::as_str))
                .collect::<std::collections::BTreeSet<_>>()
                .len();
            ((*report_field).to_string(), Value::from(count))
        })
        .collect::<Vec<_>>();
    for (field, count) in counts {
        object.insert(field, count);
    }
}

fn apply_stage_report_compatibility(id: &str, object: &mut Map<String, Value>, active: bool) {
    match id {
        "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback" => {
            set_bool(object, "source_grouping_freeze_plan_ready", active);
            set_bool(object, "operator_readback_ready", active);
        }
        "hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy" => {
            set_bool(object, "source_operator_readback_ready", active);
            set_bool(object, "strategy_ready", active);
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet" => {
            set_bool(object, "source_strategy_ready", active);
            set_bool(object, "operator_packet_ready", active);
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback" =>
        {
            set_bool(object, "source_operator_packet_ready", active);
            set_bool(object, "operator_packet_visible", active);
            set_bool(object, "non_send_readback_ready", active);
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback" =>
        {
            set_bool(object, "source_non_send_readback_ready", active);
            set_bool(object, "source_operator_packet_visible", active);
            set_bool(object, "operator_packet_visible", active);
            set_bool(object, "git_mutation_boundary_readback_ready", active);
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist" =>
        {
            set_bool(object, "source_git_boundary_readback_ready", active);
            set_bool(object, "source_operator_packet_visible", active);
            set_bool(object, "decision_checklist_visible", active);
            set_bool(object, "operator_decision_checklist_ready", active);
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-packet-readback" =>
        {
            set_bool(object, "source_operator_decision_checklist_ready", active);
            set_bool(object, "source_decision_checklist_visible", active);
            set_bool(object, "packet_readback_visible", active);
            set_bool(
                object,
                "operator_decision_checklist_packet_readback_ready",
                active,
            );
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback" =>
        {
            set_bool(object, "source_packet_readback_ready", active);
            set_bool(object, "source_packet_readback_visible", active);
            set_bool(
                object,
                "decision_recording_boundary_readback_visible",
                active,
            );
            set_bool(
                object,
                "operator_decision_recording_boundary_readback_ready",
                active,
            );
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback" =>
        {
            set_bool(object, "source_decision_recording_boundary_ready", active);
            set_bool(object, "source_decision_recording_boundary_visible", active);
            set_bool(
                object,
                "approval_acceptance_boundary_readback_visible",
                active,
            );
            set_bool(
                object,
                "operator_approval_acceptance_boundary_readback_ready",
                active,
            );
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback" =>
        {
            set_bool(object, "source_approval_acceptance_boundary_ready", active);
            set_bool(
                object,
                "source_approval_acceptance_boundary_visible",
                active,
            );
            set_bool(
                object,
                "evidence_recording_boundary_readback_visible",
                active,
            );
            set_bool(
                object,
                "operator_evidence_recording_boundary_readback_ready",
                active,
            );
        }
        "hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot" => {
            set_bool(object, "source_evidence_recording_boundary_ready", active);
            set_bool(object, "source_evidence_recording_boundary_visible", active);
            set_bool(object, "release_risk_snapshot_ready", false);
            set_bool(object, "risk_snapshot_visible", false);
        }
        "hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal" =>
        {
            set_bool(object, "source_release_risk_snapshot_ready", false);
            set_bool(object, "source_release_risk_snapshot_visible", false);
            set_bool(
                object,
                "test_only_clean_worktree_strategy_rehearsal_ready",
                false,
            );
            set_bool(object, "test_only_rehearsal_visible", false);
        }
        "hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback" => {
            set_bool(object, "source_rehearsal_ready", false);
            set_bool(object, "source_test_only_rehearsal_visible", false);
        }
        _ => {}
    }
}

fn materialize_inventory(
    object: &mut Map<String, Value>,
    observation: &DirtyWorktreeObservation,
) -> Result<(), String> {
    let counts = observation.counts();
    object.insert(
        "source_canary_cache_consumed".to_string(),
        Value::Bool(true),
    );
    for (field, count) in [
        ("inventory_entry_count", counts.inventory_entry_count),
        ("tracked_change_count", counts.tracked_change_count),
        ("untracked_change_count", counts.untracked_change_count),
        ("staged_change_count", counts.staged_change_count),
        ("unstaged_change_count", counts.unstaged_change_count),
        ("modified_change_count", counts.modified_change_count),
        ("deleted_change_count", counts.deleted_change_count),
        ("added_change_count", counts.added_change_count),
        ("renamed_change_count", counts.renamed_change_count),
        ("unmerged_change_count", counts.unmerged_change_count),
        (
            "hepta_systems_owned_count",
            counts.hepta_systems_owned_count,
        ),
        (
            "cross_lane_or_unowned_count",
            counts.cross_lane_or_unowned_count,
        ),
        ("top_level_bucket_count", counts.top_level_bucket_count),
        ("scope_bucket_count", observation.scope_buckets().len()),
        (
            "inventory_sample_count",
            observation.entries.len().min(INVENTORY_SAMPLE_LIMIT),
        ),
    ] {
        object.insert(field.to_string(), Value::from(count));
    }
    object.insert(
        "top_level_buckets".to_string(),
        serialize(&observation.top_level_buckets())?,
    );
    object.insert(
        "scope_buckets".to_string(),
        serialize(&observation.scope_buckets())?,
    );
    object.insert(
        "entries_sample".to_string(),
        serialize(
            &observation
                .entries
                .iter()
                .take(INVENTORY_SAMPLE_LIMIT)
                .collect::<Vec<_>>(),
        )?,
    );
    object.insert(
        "inventory_sample_limit".to_string(),
        Value::from(INVENTORY_SAMPLE_LIMIT),
    );
    object.insert(
        "inventory_truncated".to_string(),
        Value::Bool(observation.entries.len() > INVENTORY_SAMPLE_LIMIT),
    );
    Ok(())
}

fn materialize_grouping_plan(
    object: &mut Map<String, Value>,
    observation: &DirtyWorktreeObservation,
    groups: &[ObservedGroup],
) -> Result<(), String> {
    let counts = observation.counts();
    object.insert(
        "entries".to_string(),
        serialize(&groups.iter().map(owned_grouping_entry).collect::<Vec<_>>())?,
    );
    for (field, count) in [
        (
            "top_level_group_count",
            groups
                .iter()
                .filter(|group| group.group_type == "top_level")
                .count(),
        ),
        (
            "scope_group_count",
            groups
                .iter()
                .filter(|group| group.group_type == "scope")
                .count(),
        ),
        ("group_entry_count", groups.len()),
        ("freeze_plan_ready_count", groups.len()),
        ("planned_not_applied_count", groups.len()),
        ("release_evidence_bucket_count", groups.len()),
        (
            "hepta_systems_owned_count",
            counts.hepta_systems_owned_count,
        ),
        (
            "cross_lane_or_unowned_count",
            counts.cross_lane_or_unowned_count,
        ),
    ] {
        object.insert(field.to_string(), Value::from(count));
    }
    object.insert(
        "source_dirty_worktree_release_boundary_resolved".to_string(),
        Value::Bool(observation.entries.is_empty()),
    );
    Ok(())
}

fn materialize_entries(
    id: &str,
    object: &mut Map<String, Value>,
    groups: &[ObservedGroup],
) -> Result<(), String> {
    let entries = object
        .get_mut("entries")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| format!("dirty-worktree report {id} lacks entries"))?;
    if entries.len() != groups.len() {
        return Err(format!(
            "dirty-worktree report {id} projected {} entries for {} groups",
            entries.len(),
            groups.len()
        ));
    }
    for (entry, group) in entries.iter_mut().zip(groups) {
        rewrite_group_entry(id, entry, group)?;
    }
    Ok(())
}

fn rewrite_group_entry(id: &str, entry: &mut Value, group: &ObservedGroup) -> Result<(), String> {
    let object = entry
        .as_object_mut()
        .ok_or_else(|| format!("dirty-worktree report {id} entry is not an object"))?;
    let proxy = proxy_source_bucket(group);
    for (field, value) in object.iter_mut() {
        if let Value::String(text) = value {
            rewrite_structural_string(id, field, text, proxy, group);
        }
    }
    set_string(object, "group_type", group.group_type);
    set_string(object, "source_bucket", &group.source_bucket);
    set_usize(object, "source_entry_count", group.counts.count);
    set_usize(object, "tracked_count", group.counts.tracked_count);
    set_usize(object, "untracked_count", group.counts.untracked_count);
    set_usize(
        object,
        "hepta_systems_owned_count",
        group.counts.hepta_systems_owned_count,
    );
    set_usize(
        object,
        "cross_lane_or_unowned_count",
        group.counts.cross_lane_or_unowned_count,
    );
    set_string(object, "owner_hint", owner_hint(group.counts));
    set_string(object, "review_lane", review_lane(group.counts));
    rewrite_stage_semantics(object, &group.source_bucket, owner_hint(group.counts));
    if matches!(
        id,
        "hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy"
            | "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet"
            | "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback"
            | "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback"
    ) {
        object.insert(
            "hepta_systems_owned_count".to_string(),
            Value::from(group.counts.hepta_systems_owned_count),
        );
        object.insert(
            "cross_lane_or_unowned_count".to_string(),
            Value::from(group.counts.cross_lane_or_unowned_count),
        );
    }
    match id {
        "hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal" =>
        {
            set_string(
                object,
                "operator_action",
                "review_test_only_rehearsal_before_clean_worktree_strategy",
            );
        }
        "hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback" => {
            set_string(
                object,
                "operator_action",
                "review_outcome_readback_before_any_probe_or_git_mutation",
            );
        }
        "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-rehearsal" => {
            object.remove("local_gate");
            set_string(
                object,
                "operator_action",
                "review_owner_freeze_classification_before_any_probe_or_git_mutation",
            );
        }
        "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording" =>
        {
            object.insert(
                "decision_persistence_state".to_string(),
                Value::String("decision_persistence_blocked".to_string()),
            );
        }
        _ => {}
    }
    Ok(())
}

fn rewrite_structural_string(
    id: &str,
    field: &str,
    text: &mut String,
    proxy: &str,
    group: &ObservedGroup,
) {
    if !is_structural_entry_field(field) {
        return;
    }
    let key_proxy = key_safe(proxy);
    let key_actual = key_safe(&group.source_bucket);
    let route_proxy = proxy.replace('_', "-");
    let route_actual = route_safe(&group.source_bucket);
    *text = text
        .replace(
            &format!(".{}.{key_proxy}", group.group_type),
            &format!(".{}.{key_actual}", group.group_type),
        )
        .replace(
            &format!("/{}/{route_proxy}", route_group_type(group.group_type)),
            &format!("/{}/{route_actual}", route_group_type(group.group_type)),
        );
    if proxy == "unknown" {
        *text = text
            .replace(".unknown", &format!(".{}.{key_actual}", group.group_type))
            .replace(
                "/unknown",
                &format!("/{}/{route_actual}", route_group_type(group.group_type)),
            );
    }

    let early_scope_route = id.ends_with("grouping-freeze-operator-readback")
        || id.ends_with("actionable-clean-worktree-strategy")
        || id.ends_with("clean-worktree-strategy-operator-packet")
        || (id.ends_with("operator-packet-non-send-readback") && field == "source_packet_route");
    if group.group_type == "scope" && early_scope_route && field.ends_with("route") {
        *text = text.replace(
            &format!("/scope/{route_actual}"),
            &format!("/scope/{}", group.source_bucket),
        );
    }
}

fn is_structural_entry_field(field: &str) -> bool {
    field.ends_with("_key")
        || field.ends_with("_route")
        || field.ends_with("_checkpoint")
        || matches!(
            field,
            "group_key"
                | "group_route"
                | "comparison_anchor"
                | "classification_key"
                | "classification_route"
                | "snapshot_key"
                | "snapshot_route"
                | "rehearsal_key"
                | "rehearsal_route"
                | "outcome_key"
                | "outcome_route"
        )
}

fn rewrite_stage_semantics(
    object: &mut Map<String, Value>,
    source_bucket: &str,
    owner_hint_value: &str,
) {
    for field in ["release_risk_tier", "source_release_risk_tier"] {
        set_string(object, field, release_risk_tier(source_bucket));
    }
    for field in ["release_risk_reason", "source_release_risk_reason"] {
        set_string(object, field, release_risk_reason(source_bucket));
    }
    for field in ["release_blocker", "source_release_blocker"] {
        set_string(object, field, release_blocker(source_bucket));
    }
    for field in ["source_rehearsal_action"] {
        set_string(object, field, risk_rehearsal_action(source_bucket));
    }
    set_string(object, "rehearsal_probe", rehearsal_probe(source_bucket));
    set_string(
        object,
        "source_rehearsal_probe",
        rehearsal_probe(source_bucket),
    );
    set_string(
        object,
        "required_local_gate",
        required_local_gate(source_bucket),
    );
    set_string(
        object,
        "source_required_local_gate",
        required_local_gate(source_bucket),
    );
    set_string(
        object,
        "convergence_state",
        convergence_state(source_bucket),
    );
    set_string(
        object,
        "source_convergence_state",
        convergence_state(source_bucket),
    );
    set_string(object, "outcome_state", outcome_state(source_bucket));
    set_string(object, "source_outcome_state", outcome_state(source_bucket));
    set_string(object, "outcome_action", outcome_action(source_bucket));
    set_string(
        object,
        "source_outcome_action",
        outcome_action(source_bucket),
    );
    set_string(
        object,
        "owner_state",
        owner_state(source_bucket, owner_hint_value),
    );
    set_string(object, "freeze_state", owner_freeze_state(source_bucket));
    set_string(
        object,
        "classification_state",
        classification_state(source_bucket),
    );
    set_string(
        object,
        "release_disposition",
        release_disposition(source_bucket),
    );
    if object.contains_key("snapshot_key") {
        set_string(
            object,
            "rehearsal_action",
            risk_rehearsal_action(source_bucket),
        );
    } else {
        set_string(
            object,
            "rehearsal_action",
            owner_rehearsal_action(source_bucket),
        );
    }
    if object.contains_key("owner_route") {
        object.insert(
            "owner_route".to_string(),
            Value::String(format!(
                "owner://release-boundary/{}",
                owner_hint_value.replace('_', "-")
            )),
        );
    }
}

fn finish_report(mut value: Value) -> Result<Value, String> {
    verify_read_only_report(&value)?;
    let projection = legacy_business_projection(&value)?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| "dirty-worktree typed report must be an object".to_string())?;
    object.insert("legacy_business_fields".to_string(), projection);
    object.insert(
        "production_authority_granted".to_string(),
        Value::Bool(false),
    );
    object.insert("write_authority_granted".to_string(), Value::Bool(false));
    verify_read_only_report(&value)?;
    Ok(value)
}

fn legacy_business_projection(value: &Value) -> Result<Value, String> {
    let mut object = value
        .as_object()
        .cloned()
        .ok_or_else(|| "dirty-worktree typed report must be an object".to_string())?;
    for field in BUSINESS_ENVELOPE_FIELDS {
        object.remove(*field);
    }
    Ok(Value::Object(object))
}

fn verify_read_only_report(value: &Value) -> Result<(), String> {
    let object = value
        .as_object()
        .ok_or_else(|| "dirty-worktree typed report must be an object".to_string())?;
    for field in [
        "runtime",
        "status",
        "gate",
        "schema_version",
        "side_effects",
    ] {
        if !object.contains_key(field) {
            return Err(format!("dirty-worktree typed report lacks {field}"));
        }
    }
    verify_value_is_read_only(value)
}

fn verify_value_is_read_only(value: &Value) -> Result<(), String> {
    match value {
        Value::Array(values) => {
            for value in values {
                verify_value_is_read_only(value)?;
            }
        }
        Value::Object(object) => {
            for (key, value) in object {
                if denied_true_field(key) && value == &Value::Bool(true) {
                    return Err(format!(
                        "dirty-worktree typed report grants forbidden {key}"
                    ));
                }
                if key == "side_effects" {
                    let effects = value.as_object().ok_or_else(|| {
                        "dirty-worktree side_effects must be an object".to_string()
                    })?;
                    if effects.values().any(|effect| effect != &Value::Bool(false)) {
                        return Err("dirty-worktree typed report records a side effect".to_string());
                    }
                }
                verify_value_is_read_only(value)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn denied_true_field(field: &str) -> bool {
    field.ends_with("_allowed")
        || field.ends_with("_granted")
        || field.ends_with("_enabled")
        || field.ends_with("_performed")
        || field.ends_with("_persisted")
        || field.ends_with("_recorded")
        || field.ends_with("_started")
        || matches!(
            field,
            "freeze_applied"
                | "git_index_mutated"
                | "strategy_applied"
                | "test_probe_executed"
                | "approval_accepted"
                | "decision_recorded"
                | "evidence_recorded"
        )
        || field.contains("production_authority")
        || field.contains("write_authority")
        || field.contains("activation_authority")
}

fn proxy_grouping_entry(
    group: &ObservedGroup,
) -> DirtyWorktreeReleaseBoundaryGroupingFreezePlanEntry {
    let source_bucket = proxy_source_bucket(group);
    DirtyWorktreeReleaseBoundaryGroupingFreezePlanEntry {
        group_type: group.group_type,
        group_key: proxy_group_key(group.group_type, source_bucket),
        group_route: proxy_group_route(group.group_type, source_bucket),
        source_bucket,
        source_entry_count: group.counts.count,
        tracked_count: group.counts.tracked_count,
        untracked_count: group.counts.untracked_count,
        hepta_systems_owned_count: group.counts.hepta_systems_owned_count,
        cross_lane_or_unowned_count: group.counts.cross_lane_or_unowned_count,
        owner_hint: owner_hint(group.counts),
        review_lane: review_lane(group.counts),
        freeze_state: "planned_not_applied",
        evidence_state: "not_recorded",
        operator_visible: true,
        queryable: true,
        diffable: true,
        freeze_plan_ready: true,
        freeze_applied: false,
        git_mutation_allowed: false,
        cleanup_allowed: false,
        evidence_recording_allowed: false,
        release_cutover_allowed: false,
    }
}

fn owned_grouping_entry(group: &ObservedGroup) -> OwnedGroupingEntry {
    OwnedGroupingEntry {
        group_type: group.group_type,
        group_key: format!(
            "dirty_worktree.group.{}.{}",
            group.group_type,
            component_safe(&group.source_bucket)
        ),
        group_route: format!(
            "readback://release-boundary/dirty-worktree/group/{}/{}",
            route_group_type(group.group_type),
            component_safe(&group.source_bucket)
        ),
        source_bucket: group.source_bucket.clone(),
        source_entry_count: group.counts.count,
        tracked_count: group.counts.tracked_count,
        untracked_count: group.counts.untracked_count,
        hepta_systems_owned_count: group.counts.hepta_systems_owned_count,
        cross_lane_or_unowned_count: group.counts.cross_lane_or_unowned_count,
        owner_hint: owner_hint(group.counts),
        review_lane: review_lane(group.counts),
        freeze_state: "planned_not_applied",
        evidence_state: "not_recorded",
        operator_visible: true,
        queryable: true,
        diffable: true,
        freeze_plan_ready: true,
        freeze_applied: false,
        git_mutation_allowed: false,
        cleanup_allowed: false,
        evidence_recording_allowed: false,
        release_cutover_allowed: false,
    }
}

fn proxy_source_bucket(group: &ObservedGroup) -> &'static str {
    if group.group_type == "scope" {
        return if group.source_bucket == "hepta_systems_owned" {
            "hepta_systems_owned"
        } else {
            "cross_lane_or_unowned"
        };
    }
    match group.source_bucket.as_str() {
        "artifacts" => "artifacts",
        "codex-rs" => "codex-rs",
        "docs" => "docs",
        "plugins" => "plugins",
        "scripts" => "scripts",
        _ => "unknown",
    }
}

fn proxy_group_key(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => "dirty_worktree.group.top_level.artifacts",
        ("top_level", "codex-rs") => "dirty_worktree.group.top_level.codex_rs",
        ("top_level", "docs") => "dirty_worktree.group.top_level.docs",
        ("top_level", "plugins") => "dirty_worktree.group.top_level.plugins",
        ("top_level", "scripts") => "dirty_worktree.group.top_level.scripts",
        ("top_level", _) => "dirty_worktree.group.top_level.unknown",
        ("scope", "hepta_systems_owned") => "dirty_worktree.group.scope.hepta_systems_owned",
        ("scope", _) => "dirty_worktree.group.scope.cross_lane_or_unowned",
        _ => "dirty_worktree.group.unknown",
    }
}

fn proxy_group_route(group_type: &str, source_bucket: &str) -> &'static str {
    match (group_type, source_bucket) {
        ("top_level", "artifacts") => {
            "readback://release-boundary/dirty-worktree/group/top-level/artifacts"
        }
        ("top_level", "codex-rs") => {
            "readback://release-boundary/dirty-worktree/group/top-level/codex-rs"
        }
        ("top_level", "docs") => "readback://release-boundary/dirty-worktree/group/top-level/docs",
        ("top_level", "plugins") => {
            "readback://release-boundary/dirty-worktree/group/top-level/plugins"
        }
        ("top_level", "scripts") => {
            "readback://release-boundary/dirty-worktree/group/top-level/scripts"
        }
        ("top_level", _) => "readback://release-boundary/dirty-worktree/group/top-level/unknown",
        ("scope", "hepta_systems_owned") => {
            "readback://release-boundary/dirty-worktree/group/scope/hepta_systems_owned"
        }
        ("scope", _) => {
            "readback://release-boundary/dirty-worktree/group/scope/cross_lane_or_unowned"
        }
        _ => "readback://release-boundary/dirty-worktree/group/unknown",
    }
}

fn owner_hint(counts: BucketCounts) -> &'static str {
    if counts.cross_lane_or_unowned_count > counts.hepta_systems_owned_count {
        "cross-lane-review"
    } else {
        "hepta-systems"
    }
}

fn review_lane(counts: BucketCounts) -> &'static str {
    if counts.cross_lane_or_unowned_count > 0 && counts.hepta_systems_owned_count > 0 {
        "mixed-hepta-and-cross-lane"
    } else if counts.cross_lane_or_unowned_count > 0 {
        "cross-lane-review"
    } else {
        "hepta-systems"
    }
}

fn release_risk_tier(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "critical",
        "codex-rs" | "plugins" | "scripts" | "hepta_systems_owned" => "high",
        "artifacts" | "docs" => "medium",
        _ => "high",
    }
}

fn release_risk_reason(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => {
            "cross-lane or unowned changes need owner attribution before release"
        }
        "codex-rs" => "runtime and crate changes require targeted Rust gates before release",
        "plugins" => "plugin surface changes can affect runtime/tool contribution boundaries",
        "scripts" => "automation and gate scripts can change release evidence",
        "hepta_systems_owned" => "owned Hepta systems changes still need freeze and rehearsal",
        "artifacts" => "generated or local artifacts need classification before release evidence",
        "docs" => {
            "architecture and evidence notes affect operator readback but not runtime execution"
        }
        _ => "dirty worktree bucket requires release-risk review",
    }
}

fn release_blocker(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "cross_lane_or_unowned_changes",
        "codex-rs" => "runtime_crate_changes",
        "plugins" => "plugin_surface_changes",
        "scripts" => "automation_gate_changes",
        "hepta_systems_owned" => "hepta_systems_owned_changes",
        "artifacts" => "generated_or_local_artifacts",
        "docs" => "documentation_evidence_changes",
        _ => "dirty_worktree_changes",
    }
}

fn risk_rehearsal_action(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "test_only_owner_attribution_and_freeze_rehearsal",
        "codex-rs" => "test_only_targeted_rust_gate_rehearsal",
        "plugins" => "test_only_plugin_surface_rehearsal",
        "scripts" => "test_only_script_gate_rehearsal",
        "hepta_systems_owned" => "test_only_owned_lane_freeze_rehearsal",
        "artifacts" => "test_only_artifact_classification_rehearsal",
        "docs" => "test_only_doc_evidence_rehearsal",
        _ => "test_only_dirty_worktree_rehearsal",
    }
}

fn rehearsal_probe(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "owner_attribution_and_freeze_probe",
        "codex-rs" => "targeted_rust_gate_probe",
        "plugins" => "plugin_surface_gate_probe",
        "scripts" => "script_syntax_and_gate_probe",
        "hepta_systems_owned" => "owned_lane_freeze_probe",
        "artifacts" => "artifact_classification_probe",
        "docs" => "doc_evidence_consistency_probe",
        _ => "general_dirty_worktree_review_probe",
    }
}

fn required_local_gate(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "owner_attribution_freeze_gate",
        "codex-rs" => "targeted_rust_gate",
        "plugins" => "plugin_surface_gate",
        "scripts" => "script_syntax_gate",
        "hepta_systems_owned" => "owned_lane_freeze_gate",
        "artifacts" => "artifact_classification_gate",
        "docs" => "doc_evidence_consistency_gate",
        _ => "general_dirty_worktree_review_gate",
    }
}

fn convergence_state(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "blocked_until_owner_attribution",
        "codex-rs" => "candidate_after_targeted_rust_gate",
        "plugins" => "candidate_after_plugin_surface_gate",
        "scripts" => "candidate_after_script_gate",
        "hepta_systems_owned" => "candidate_after_owned_lane_freeze",
        "artifacts" => "candidate_after_artifact_classification",
        "docs" => "candidate_after_doc_evidence_check",
        _ => "candidate_after_general_dirty_worktree_review",
    }
}

fn outcome_state(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "blocked_until_owner_attribution",
        "codex-rs" => "ready_for_targeted_rust_gate_rehearsal",
        "plugins" => "ready_for_plugin_surface_gate_rehearsal",
        "scripts" => "ready_for_script_syntax_gate_rehearsal",
        "hepta_systems_owned" => "ready_for_owned_lane_freeze_rehearsal",
        "artifacts" => "ready_for_artifact_classification_rehearsal",
        "docs" => "ready_for_doc_evidence_consistency_rehearsal",
        _ => "unknown",
    }
}

fn outcome_action(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "attribute_owner_before_any_clean_worktree_action",
        "codex-rs" => "run_targeted_rust_gate_probe_later_without_git_mutation",
        "plugins" => "run_plugin_surface_gate_probe_later_without_git_mutation",
        "scripts" => "run_script_syntax_gate_probe_later_without_git_mutation",
        "hepta_systems_owned" => "freeze_owned_lane_changes_later_without_git_mutation",
        "artifacts" => "classify_artifacts_later_without_delete",
        "docs" => "check_doc_evidence_consistency_later_without_persistence",
        _ => "review_dirty_worktree_bucket_later_without_git_mutation",
    }
}

fn owner_state(bucket: &str, owner: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "owner_attribution_required",
        _ if owner == "hepta-systems" => "owner_hint_hepta_systems_projected",
        _ if owner == "cross-lane-review" => "cross_lane_owner_review_required",
        _ => "unknown",
    }
}

fn owner_freeze_state(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "freeze_blocked_until_owner_attribution",
        "hepta_systems_owned" => "owned_lane_freeze_candidate",
        "artifacts" => "freeze_deferred_until_artifact_classification",
        "codex-rs" | "plugins" | "scripts" | "docs" => "freeze_deferred_until_targeted_gate",
        _ => "unknown",
    }
}

fn classification_state(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "owner_attribution_required",
        "codex-rs" => "targeted_rust_gate_required",
        "plugins" => "plugin_surface_gate_required",
        "scripts" => "script_syntax_gate_required",
        "hepta_systems_owned" => "owned_lane_freeze_required",
        "artifacts" => "artifact_classification_required",
        "docs" => "doc_evidence_consistency_required",
        _ => "unknown",
    }
}

fn release_disposition(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "blocked_until_owner_attribution",
        "artifacts" => "blocked_until_artifact_classification",
        "hepta_systems_owned" => "blocked_until_owned_lane_freeze",
        "codex-rs" | "plugins" | "scripts" | "docs" => "blocked_until_targeted_gate",
        _ => "blocked_until_bucket_review",
    }
}

fn owner_rehearsal_action(bucket: &str) -> &'static str {
    match bucket {
        "cross_lane_or_unowned" => "project_owner_attribution_without_git_mutation_or_persistence",
        "codex-rs" => "project_targeted_rust_gate_without_execution_or_git_mutation",
        "plugins" => "project_plugin_surface_gate_without_execution_or_git_mutation",
        "scripts" => "project_script_syntax_gate_without_execution_or_git_mutation",
        "hepta_systems_owned" => "project_owned_lane_freeze_without_applying_freeze",
        "artifacts" => "project_artifact_classification_without_delete_or_relocation",
        "docs" => "project_doc_evidence_consistency_without_evidence_persistence",
        _ => "project_bucket_review_without_git_mutation",
    }
}

fn set_string(object: &mut Map<String, Value>, field: &str, value: &str) {
    if object.contains_key(field) {
        object.insert(field.to_string(), Value::String(value.to_string()));
    }
}

fn set_usize(object: &mut Map<String, Value>, field: &str, value: usize) {
    if object.contains_key(field) {
        object.insert(field.to_string(), Value::from(value));
    }
}

fn set_bool(object: &mut Map<String, Value>, field: &str, value: bool) {
    if object.contains_key(field) {
        object.insert(field.to_string(), Value::Bool(value));
    }
}

fn key_safe(value: &str) -> String {
    component_safe(value).replace('-', "_")
}

fn route_safe(value: &str) -> String {
    component_safe(value).replace('_', "-")
}

fn component_safe(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-') {
                character
            } else {
                '-'
            }
        })
        .collect()
}

fn route_group_type(group_type: &str) -> &'static str {
    match group_type {
        "top_level" => "top-level",
        "scope" => "scope",
        _ => "unknown",
    }
}

fn valid_status(status: char) -> bool {
    matches!(
        status,
        ' ' | 'M' | 'T' | 'A' | 'D' | 'R' | 'C' | 'U' | '?' | '!'
    )
}

fn change_kind(index: char, worktree: char) -> &'static str {
    if index == '?' && worktree == '?' {
        "untracked"
    } else if matches!(
        (index, worktree),
        ('A', 'A') | ('D', 'D') | ('A', 'U') | ('U', 'A') | ('D', 'U') | ('U', 'D') | ('U', 'U')
    ) {
        "unmerged"
    } else if index != ' ' && worktree != ' ' {
        "index_and_worktree"
    } else if index != ' ' {
        "index_only"
    } else if worktree != ' ' {
        "worktree_only"
    } else {
        "unknown"
    }
}

fn is_hepta_systems_owned(path: &str) -> bool {
    [
        "apps/hepta-native/",
        "codex-rs/hepta-runtime/",
        "codex-rs/core-plugins/",
        "codex-rs/hepta-plugins/",
        "codex-rs/plugin/",
        "codex-rs/utils/plugins/",
        "codex-rs/tools/",
        ".hepta/",
        "plugins/hepta-system/",
        "scripts/",
    ]
    .iter()
    .any(|prefix| path.starts_with(prefix))
        || path.starts_with("docs/architecture/HEPTA_SYSTEMS")
}

fn add_bucket_entry(counts: &mut BucketCounts, entry: &DirtyWorktreeObservationEntry) {
    counts.count += 1;
    counts.tracked_count += usize::from(!entry.is_untracked());
    counts.untracked_count += usize::from(entry.is_untracked());
    counts.hepta_systems_owned_count += usize::from(entry.scope_bucket == "hepta_systems_owned");
    counts.cross_lane_or_unowned_count +=
        usize::from(entry.scope_bucket == "cross_lane_or_unowned");
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    const LEGACY_ORACLE: &[(&str, usize, &str, usize, &str)] = &[
        (
            "hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy",
            50,
            "bedfdd8713726c407f11a700c43e86724ab7a50f88a4365ed9075ede6b80a8f5",
            81,
            "cc4e12d373565fc05475e2cd1930e1929527aad58fe01ef80d80ea9807e502d5",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback",
            81,
            "8f18119613676019e72c887dd478e8df1d2f7f27166e43d0502cfe0dc4333649",
            141,
            "ed2fcc5a22fd15cd91a0e361845a9bbab0b3b64168985f46776667c412d4477f",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-packet-readback",
            72,
            "b33430a26d900957e6562cc7f45b4cbba37a162a58a3fa1e15677b53ff9dc8fa",
            118,
            "d8833535ec793053f64cad2ac60062f7ae5ba1dabee46e96e806cc36b2a748cf",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist",
            65,
            "ed8284d524ec2218db6d138e1dab7801d57c241dc24e9ccaa5c5519d9c68b7df",
            106,
            "06b5358c256bca2f9d75568d4829c23eeb7984ed621ac09bc145ff6f35e88f70",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback",
            75,
            "34bf9763c86b725d06c738c1a08ae812f08e7db6add9d0c6f2e95155f70c483d",
            125,
            "d1c7ea617de42a39713b8f801cfaae983d1a60b98d368ef74c19cb6b82096f25",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback",
            88,
            "661516f0a15c2c5df0cfe7a0428b73ae92fe3aa249b64b3c0cd5437b4b4bc706",
            153,
            "77e5c5713386b32bd254a02bed2728c496dc197b8243453403752a7288cb83e5",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback",
            61,
            "9930144de8efabaf3de26c44ef2c9a3bde0da5f62049b65b39e4aa6a7ba7faef",
            102,
            "5fd0c71fe360c79a385288c990b413c035e23d0958064219d1a0279f078bce53",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback",
            58,
            "71c1ffa2d67397315fea7fbf4004a25b871361134a96101c7d4b9dcf535125e1",
            99,
            "d9bf8ef987ec36e789c942022b3d650db9468e61068083a976ff0ad71a039f90",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet",
            61,
            "ef9aa8a51a9a6b3031e652787e7d66881db9ca3ca53655a65c36ac9ffbb80f99",
            93,
            "bec34b7431fe24fccf9ac9a139a30935073854a9d9bf1d7a786a3edf4ccda80d",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback",
            49,
            "5b78ab84a22f59502379998f35ea595c36d8f5336264d0c99ef88af1b09bc073",
            81,
            "a9d662a357ae81da87703d32834d1b160f300f9867b65fd726f176d688d5aeff",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan",
            49,
            "11bf9868e42fb4ff950e152cf51e936282102d19a2f4c954c42509ee5fdb6dd3",
            71,
            "45f8c6cf9ea42beb0caddf69521dbb459da3f93bd09c9b88caa8b03f3e00e742",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-inventory",
            75,
            "3ad9334d6839628e7d1f7f002a8b042129dff322fa2cac66318246bc2aac0ebe",
            98,
            "d57c9b50d5356e71049060079b9c045c308b900adbfc52fc3f37d02dcc4e98cb",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-operator-evidence-recording-boundary-readback-without-recording",
            107,
            "66d9118663121bfc0a085345c2469df09e7d10c40ea365e8dc0f239be6c54067",
            180,
            "1fea987c94273acd7d4be08ce74254e1e929a4e697f037d85ac311f99566f114",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-owner-freeze-classification-rehearsal",
            83,
            "8fb13937d593d0d49bd0058e0cdc30a341ef74082ad3415f0f14eea7410b389d",
            140,
            "99c3c9f942f6ba0af642b1cba3b0ae83ce7bf25e877f1ba7a89b0c61e4421962",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot",
            73,
            "a88ecd2032d812bb013bef283995fb4ba96faa640e5c7ab834f04efad7a11d3e",
            117,
            "783ab45cb9cb9ec3e698fb136bd201b23652793a700d5745a863f6823a5c9327",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal",
            79,
            "1b5ac0210098f8f760acad9139af1b43941eb71c10ce233a696e49789d6081a1",
            130,
            "7a6025b38c1734a7e0e48178987abbe31b6542757869869147c1e2faaf7b03f3",
        ),
        (
            "hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback",
            79,
            "cd8c9c35268d98157f2f0a60a6ccaf428f36558d6cd4935deccdcffeca3623c2",
            131,
            "83199586d5710a705d92891740296dc7cf70000bfe3adcd245f6b3b1a2a47ac7",
        ),
    ];

    #[test]
    fn porcelain_v1_z_parser_preserves_spaces_and_rename_records() {
        let observation = DirtyWorktreeObservation::from_porcelain_v1_z(
            b" M scripts/with space.sh\0R  codex-rs/hepta-runtime/src/new.rs\0codex-rs/hepta-runtime/src/old.rs\0?? docs/new note.md\0",
        )
        .expect("porcelain input should parse");
        assert_eq!(observation.entries.len(), 3);
        assert_eq!(observation.entries[0].path, "scripts/with space.sh");
        assert_eq!(
            observation.entries[1].original_path.as_deref(),
            Some("codex-rs/hepta-runtime/src/old.rs")
        );
        assert_eq!(observation.counts().renamed_change_count, 1);
    }

    #[test]
    fn porcelain_parser_fails_closed_on_truncated_rename_and_invalid_utf8() {
        assert!(
            DirtyWorktreeObservation::from_porcelain_v1_z(
                b"R  codex-rs/hepta-runtime/src/new.rs\0",
            )
            .expect_err("truncated rename must fail")
            .contains("original path")
        );
        assert!(
            DirtyWorktreeObservation::from_porcelain_v1_z(b"?? \xff\0")
                .expect_err("invalid UTF-8 must fail")
                .contains("valid UTF-8")
        );
    }

    #[test]
    fn reports_are_read_only_for_clean_and_dirty_observations() {
        for bytes in [
            b"".as_slice(),
            b" M README.md\0?? scripts/dirty-worktree-compat-oracle-untracked.txt\0".as_slice(),
        ] {
            let observation =
                DirtyWorktreeObservation::from_porcelain_v1_z(bytes).expect("fixture should parse");
            for id in DIRTY_WORKTREE_TYPED_COMPAT_REPORT_IDS {
                let report = dirty_worktree_typed_compat_report(id, &observation)
                    .unwrap_or_else(|error| panic!("{id} failed: {error}"));
                verify_read_only_report(&report)
                    .unwrap_or_else(|error| panic!("{id} is not read-only: {error}"));
                assert!(report["legacy_business_fields"].is_object());
                assert_eq!(report["production_authority_granted"], false);
                assert_eq!(report["write_authority_granted"], false);
            }
        }
    }

    #[test]
    fn clean_and_fixed_dirty_business_projections_match_legacy_oracle() {
        let clean =
            DirtyWorktreeObservation::from_porcelain_v1_z(b"").expect("clean fixture should parse");
        let dirty = DirtyWorktreeObservation::from_porcelain_v1_z(
            b" M README.md\0?? scripts/dirty-worktree-compat-oracle-untracked.txt\0",
        )
        .expect("dirty fixture should parse");
        for (id, clean_fields, clean_digest, dirty_fields, dirty_digest) in LEGACY_ORACLE {
            let clean_report = dirty_worktree_typed_compat_report(id, &clean)
                .unwrap_or_else(|error| panic!("clean {id} failed: {error}"));
            let dirty_report = dirty_worktree_typed_compat_report(id, &dirty)
                .unwrap_or_else(|error| panic!("dirty {id} failed: {error}"));
            assert_projection_oracle(
                id,
                "clean",
                &clean_report["legacy_business_fields"],
                *clean_fields,
                clean_digest,
            );
            assert_projection_oracle(
                id,
                "dirty",
                &dirty_report["legacy_business_fields"],
                *dirty_fields,
                dirty_digest,
            );
        }
    }

    fn assert_projection_oracle(
        id: &str,
        fixture: &str,
        projection: &Value,
        expected_field_count: usize,
        expected_digest: &str,
    ) {
        let mut fields = std::collections::BTreeSet::new();
        collect_field_paths(projection, "", &mut fields);
        assert_eq!(
            fields.len(),
            expected_field_count,
            "{id} {fixture} field set drifted"
        );
        let canonical = canonical_value(projection);
        let bytes = serde_json::to_vec(&canonical).expect("canonical projection should serialize");
        assert_eq!(
            format!("{:x}", Sha256::digest(bytes)),
            expected_digest,
            "{id} {fixture} recursive business JSON drifted"
        );
    }

    fn collect_field_paths(
        value: &Value,
        prefix: &str,
        fields: &mut std::collections::BTreeSet<String>,
    ) {
        match value {
            Value::Object(object) => {
                for (key, value) in object {
                    let path = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{prefix}.{key}")
                    };
                    fields.insert(path.clone());
                    collect_field_paths(value, &path, fields);
                }
            }
            Value::Array(values) => {
                let path = format!("{prefix}[]");
                fields.insert(path.clone());
                for value in values {
                    collect_field_paths(value, &path, fields);
                }
            }
            _ => {}
        }
    }

    fn canonical_value(value: &Value) -> Value {
        match value {
            Value::Object(object) => {
                let mut canonical = Map::new();
                let mut keys = object.keys().collect::<Vec<_>>();
                keys.sort();
                for key in keys {
                    canonical.insert(key.clone(), canonical_value(&object[key]));
                }
                Value::Object(canonical)
            }
            Value::Array(values) => Value::Array(values.iter().map(canonical_value).collect()),
            _ => value.clone(),
        }
    }

    #[test]
    fn hostile_top_level_names_only_affect_owned_structural_fields() {
        let observation = DirtyWorktreeObservation::from_porcelain_v1_z(
            b"?? unknown/a\0?? scope/b\0?? space name/c\0?? dash-name/d\0",
        )
        .expect("hostile names should remain observable");
        let readback = dirty_worktree_typed_compat_report(
            "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback",
            &observation,
        )
        .expect("readback should render");
        let entries = readback["entries"]
            .as_array()
            .expect("entries should be an array");
        let by_bucket = |bucket: &str| {
            entries
                .iter()
                .find(|entry| {
                    entry["group_type"] == "top_level" && entry["source_bucket"] == bucket
                })
                .unwrap_or_else(|| panic!("missing hostile bucket {bucket}"))
        };
        assert_eq!(
            by_bucket("unknown")["readback_key"],
            "dirty_worktree.readback.top_level.unknown"
        );
        assert_eq!(
            by_bucket("scope")["readback_key"],
            "dirty_worktree.readback.top_level.scope"
        );
        assert_eq!(
            by_bucket("space name")["readback_key"],
            "dirty_worktree.readback.top_level.space_name"
        );
        assert_eq!(
            by_bucket("dash-name")["readback_route"],
            "readback://release-boundary/dirty-worktree/grouping-freeze/operator/top-level/dash-name"
        );
        assert!(entries.iter().all(|entry| {
            entry["operator_status"] == "blocked_pending_clean_worktree_strategy"
        }));

        let outcome = dirty_worktree_typed_compat_report(
            "hepta-systems-dirty-worktree-release-boundary-test-only-rehearsal-outcome-readback",
            &observation,
        )
        .expect("outcome should render");
        let unknown = outcome["entries"]
            .as_array()
            .and_then(|entries| {
                entries
                    .iter()
                    .find(|entry| entry["source_bucket"] == "unknown")
            })
            .expect("unknown outcome entry should exist");
        assert_eq!(unknown["outcome_state"], "unknown");
        assert_eq!(
            unknown["operator_action"],
            "review_outcome_readback_before_any_probe_or_git_mutation"
        );
    }

    #[test]
    fn retired_pair_is_internal_only() {
        let observation =
            DirtyWorktreeObservation::from_porcelain_v1_z(b"").expect("clean fixture should parse");
        assert!(
            dirty_worktree_typed_compat_report(
                RETIRED_DIRTY_WORKTREE_COMPAT_REPORT_ID,
                &observation,
            )
            .is_err()
        );
        let report = retired_dirty_worktree_owner_decision_source_report(&observation)
            .expect("internal source must remain available");
        assert_eq!(report["decision_recorded"], false);
    }
}
