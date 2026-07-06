use super::*;
use pretty_assertions::assert_eq;

fn assert_compression_stage_evidence(
    stage: &TurnContextCompressionStage,
    rollback_source_text_hash: &str,
) {
    assert_eq!(
        stage.loss_check_status,
        Some(TurnContextCompressionLossCheckStatus::MarkerBoundaryOnly)
    );
    assert_eq!(
        stage.rollback_source_text_hash.as_deref(),
        Some(rollback_source_text_hash)
    );
    assert_eq!(
        stage.protected_tier_invariant,
        Some(TurnContextCompressionProtectedTierInvariant::Preserved)
    );
}

#[test]
fn turn_context_manifest_hashes_context_items_without_payload_text() {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: "secret policy body".to_string(),
        }],
        phase: None,
    }];

    let manifest =
        build_turn_context_manifest(&context_items).expect("context items should produce manifest");
    let manifest_json = serde_json::to_string(&manifest).expect("manifest should serialize");

    assert!(manifest.has_replay_integrity());
    assert_eq!(manifest.entries.len(), 1);
    assert_eq!(manifest.decision_ledger.len(), 2);
    assert_eq!(manifest.budget_tokens, Some(manifest.estimated_tokens));
    assert_eq!(manifest.omitted_entries, 0);
    assert!(manifest.omitted_sources.is_empty());
    assert!(!manifest.truncated);
    assert_eq!(manifest.entries[0].role, "developer");
    assert_eq!(manifest.entries[0].tier, TurnContextTier::Developer);
    assert_eq!(
        manifest.entries[0].source,
        "turn_context:developer:developer_instructions:0"
    );
    assert_eq!(
        manifest.decision_ledger[0].source,
        manifest.entries[0].source
    );
    assert_eq!(
        manifest.decision_ledger[0].decision,
        "included:always_include_developer"
    );
    assert_eq!(
        manifest.decision_ledger[1].source,
        "turn_context:assembly_policy"
    );
    assert_eq!(
        manifest.decision_ledger[1].decision,
        "policy:non_omitting_replay_baseline:within_budget"
    );
    assert!(manifest.decision_ledger[0].reason_hash.is_some());
    assert!(manifest.decision_ledger[1].reason_hash.is_some());
    assert!(manifest.decision_ledger_hash.is_some());
    assert!(!manifest_json.contains("secret policy body"));
    assert!(manifest_json.contains("\"tier\":\"developer\""));
}

#[test]
fn turn_context_manifest_entries_record_context_tiers() {
    let context_items = vec![
        ResponseItem::Message {
            id: None,
            role: "developer".to_string(),
            content: vec![
                ContentItem::InputText {
                    text: "<permissions instructions>policy</permissions instructions>".to_string(),
                },
                ContentItem::InputText {
                    text: format!("{APPS_INSTRUCTIONS_OPEN_TAG}\napps\n</apps_instructions>"),
                },
                ContentItem::InputText {
                    text: format!(
                        "{ENVIRONMENT_CONTEXT_OPEN_TAG}\n<cwd>/tmp</cwd>\n</environment_context>"
                    ),
                },
                ContentItem::InputText {
                    text: "plain developer instructions".to_string(),
                },
                ContentItem::InputText {
                    text: format!(
                        "{LIVE_RECALL_SELECTED_SNIPPETS_HEADER}\nsummary\n{LIVE_RECALL_SELECTED_SNIPPETS_FOOTER}"
                    ),
                },
            ],
            phase: None,
        },
        ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText {
                text: "workspace user instructions".to_string(),
            }],
            phase: None,
        },
    ];

    let manifest =
        build_turn_context_manifest(&context_items).expect("context items should produce manifest");
    let tier_for = |source_id: &str| {
        manifest
            .entries
            .iter()
            .find(|entry| entry.source.contains(source_id))
            .map(|entry| entry.tier)
            .expect("source tier should be present")
    };

    assert_eq!(tier_for(":permissions:"), TurnContextTier::System);
    assert_eq!(tier_for(":apps:"), TurnContextTier::Tool);
    assert_eq!(tier_for(":environment:"), TurnContextTier::Runtime);
    assert_eq!(
        tier_for(":developer_instructions:"),
        TurnContextTier::Developer
    );
    assert_eq!(
        tier_for(":selected_context_recall:"),
        TurnContextTier::RetrievedSnippets
    );
    assert_eq!(tier_for(":contextual_user:"), TurnContextTier::User);
    assert!(manifest.has_replay_integrity());
}

#[test]
fn turn_context_manifest_records_budget_pressure_without_prompt_truncation() {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: "secret policy body".to_string(),
        }],
        phase: None,
    }];
    let assembly_policy = ContextAssemblyPolicy::from_model_context_window(Some(1));

    let manifest = build_turn_context_manifest_with_policy(&context_items, &assembly_policy)
        .expect("context items should produce manifest");
    let manifest_json = serde_json::to_string(&manifest).expect("manifest should serialize");

    assert!(manifest.has_replay_integrity());
    assert!(manifest.estimated_tokens > 1);
    assert_eq!(manifest.budget_tokens, Some(1));
    assert_eq!(manifest.omitted_entries, 0);
    assert!(manifest.omitted_sources.is_empty());
    assert!(!manifest.truncated);
    assert!(manifest.decision_ledger.iter().any(|entry| {
        entry.source == "turn_context:assembly_policy"
            && matches!(
                entry.kind(),
                TurnContextDecisionKind::Policy {
                    strategy,
                    budget_state
                } if strategy == "non_omitting_replay_baseline"
                    && budget_state == "budget_exceeded"
            )
            && entry.reason_hash.is_some()
    }));
    assert!(!manifest_json.contains("secret policy body"));
}

#[test]
fn turn_context_manifest_records_source_aware_budget_candidates_without_prompt_mutation() {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![
            ContentItem::InputText {
                text: "<permissions instructions>policy</permissions instructions>".to_string(),
            },
            ContentItem::InputText {
                text: format!(
                    "{EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG}\ncapability payload should not serialize\n</extension_developer_capabilities>"
                ),
            },
            ContentItem::InputText {
                text: format!(
                    "{PLUGINS_INSTRUCTIONS_OPEN_TAG}\ntool inventory payload should not serialize\n</plugins_instructions>"
                ),
            },
            ContentItem::InputText {
                text: format!(
                    "{APPS_INSTRUCTIONS_OPEN_TAG}\napp inventory payload should not serialize\n</apps_instructions>"
                ),
            },
            ContentItem::InputText {
                text: format!("{SKILLS_INSTRUCTIONS_OPEN_TAG}\nskills\n</skills_instructions>"),
            },
        ],
        phase: None,
    }];
    let assembly_policy = ContextAssemblyPolicy::from_model_context_window(Some(1));

    let manifest = build_turn_context_manifest_with_policy(&context_items, &assembly_policy)
        .expect("context items should produce manifest");
    let candidate_decisions = manifest
        .decision_ledger
        .iter()
        .filter_map(|entry| {
            let kind = entry.kind();
            let is_candidate = matches!(
                &kind,
                TurnContextDecisionKind::CandidateOmit { .. }
                    | TurnContextDecisionKind::CandidateTruncate { .. }
            );
            is_candidate.then_some((entry.source.as_str(), kind))
        })
        .collect::<Vec<_>>();

    assert!(manifest.has_replay_integrity());
    assert_eq!(manifest.entries.len(), 5);
    assert_eq!(manifest.omitted_entries, 0);
    assert!(manifest.omitted_sources.is_empty());
    assert!(!manifest.truncated);
    assert_eq!(
        candidate_decisions
            .iter()
            .map(|(source, _)| *source)
            .collect::<Vec<_>>(),
        vec![
            "turn_context:developer:extension_developer_capabilities:0:1",
            "turn_context:developer:available_plugins:0:2",
            "turn_context:developer:apps:0:3",
            "turn_context:developer:available_skills:0:4",
        ]
    );
    assert!(matches!(
        &candidate_decisions[0].1,
        TurnContextDecisionKind::CandidateOmit {
            source_id,
            priority,
            tokens
        } if source_id == "extension_developer_capabilities"
            && *priority == 10
            && *tokens > 0
    ));
    assert!(matches!(
        &candidate_decisions[1].1,
        TurnContextDecisionKind::CandidateOmit {
            source_id,
            priority,
            tokens
        } if source_id == "available_plugins" && *priority == 20 && *tokens > 0
    ));
    assert!(matches!(
        &candidate_decisions[2].1,
        TurnContextDecisionKind::CandidateOmit {
            source_id,
            priority,
            tokens
        } if source_id == "apps" && *priority == 30 && *tokens > 0
    ));
    assert!(matches!(
        &candidate_decisions[3].1,
        TurnContextDecisionKind::CandidateOmit {
            source_id,
            priority,
            tokens
        } if source_id == "available_skills" && *priority == 40 && *tokens > 0
    ));
    assert!(
        !manifest
            .decision_ledger
            .iter()
            .any(|entry| entry.kind().is_candidate_truncation())
    );
}

#[test]
fn turn_context_manifest_records_adaptive_budget_allocations_without_prompt_mutation() {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![
            ContentItem::InputText {
                text: "<permissions instructions>policy</permissions instructions>".to_string(),
            },
            ContentItem::InputText {
                text: format!(
                    "{EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG}\nextension caps\n</extension_developer_capabilities>"
                ),
            },
            ContentItem::InputText {
                text: format!("{PLUGINS_INSTRUCTIONS_OPEN_TAG}\nplugins\n</plugins_instructions>"),
            },
            ContentItem::InputText {
                text: format!("{APPS_INSTRUCTIONS_OPEN_TAG}\napps\n</apps_instructions>"),
            },
        ],
        phase: None,
    }];
    let ledger = ContextContributionLedger::from_response_items(&context_items);
    let tokens_for = |source_id: &str| {
        ledger
            .contributions()
            .iter()
            .filter(|contribution| contribution.source_id == source_id)
            .fold(0_u32, |tokens, contribution| {
                tokens.saturating_add(contribution.estimated_tokens)
            })
    };
    let permissions_tokens = tokens_for("permissions");
    let extension_capability_tokens = tokens_for("extension_developer_capabilities");
    let extension_capability_reserve = estimated_compression_output_tokens(
        TurnContextCompressionStageKind::Prune,
        extension_capability_tokens,
    );
    let budget_tokens = permissions_tokens.saturating_add(extension_capability_reserve);
    let assembly_policy =
        ContextAssemblyPolicy::from_model_context_window(Some(i64::from(budget_tokens)));

    let result = assemble_turn_context_with_policy(
        &context_items,
        None,
        &TurnContextManifestOptions::default(),
        &assembly_policy,
    );
    let manifest = result
        .context_manifest
        .expect("context items should produce manifest");
    let manifest_json = serde_json::to_string(&manifest).expect("manifest should serialize");
    let allocation_for = |source_id: &str| {
        manifest
            .adaptive_budget_allocations
            .iter()
            .find(|allocation| allocation.source_id == source_id)
            .expect("allocation should be present")
    };

    assert_eq!(result.context_items, context_items);
    assert!(manifest.has_replay_integrity());
    assert!(manifest.estimated_tokens > budget_tokens);
    assert_eq!(manifest.budget_tokens, Some(budget_tokens));
    assert_eq!(manifest.omitted_entries, 0);
    assert!(manifest.omitted_sources.is_empty());
    assert!(!manifest.truncated);
    assert_eq!(manifest.adaptive_budget_allocations.len(), 4);
    assert!(manifest.adaptive_budget_allocations_have_integrity());

    let permissions = allocation_for("permissions");
    assert_eq!(
        permissions.proposed_action,
        TurnContextBudgetAllocationAction::Keep
    );
    assert_eq!(permissions.proposed_budget_tokens, permissions.input_tokens);
    assert_eq!(permissions.overflow_tokens, 0);
    assert!(!permissions.would_drop);
    assert!(!permissions.would_compress);

    let extension_capabilities = allocation_for("extension_developer_capabilities");
    assert_eq!(
        extension_capabilities.current_heuristic_action,
        TurnContextBudgetAllocationAction::Drop
    );
    assert_eq!(
        extension_capabilities.proposed_action,
        TurnContextBudgetAllocationAction::Compress
    );
    assert_eq!(
        extension_capabilities.compression_kind,
        Some(TurnContextCompressionStageKind::Prune)
    );
    assert_eq!(
        extension_capabilities.proposed_budget_tokens,
        extension_capability_reserve
    );
    assert!(extension_capabilities.overflow_tokens > 0);
    assert!(!extension_capabilities.would_drop);
    assert!(extension_capabilities.would_compress);

    let plugins = allocation_for("available_plugins");
    assert_eq!(
        plugins.proposed_action,
        TurnContextBudgetAllocationAction::Drop
    );
    assert_eq!(plugins.proposed_budget_tokens, 0);
    assert!(plugins.overflow_tokens > 0);
    assert!(plugins.would_drop);
    assert!(!plugins.would_compress);

    assert!(manifest_json.contains("\"adaptive_budget_allocations\""));
    assert!(!manifest_json.contains("capability payload should not serialize"));
    assert!(!manifest_json.contains("tool inventory payload should not serialize"));
    assert!(!manifest_json.contains("app inventory payload should not serialize"));
    assert!(!manifest_json.contains("policy</permissions"));
}

#[test]
fn source_aware_budget_candidate_priority_is_tier_guarded() {
    let mut contribution = ContextContribution {
        role: "developer".to_string(),
        slot: "developer".to_string(),
        source_id: "available_plugins",
        source: "turn_context:developer:available_plugins:0".to_string(),
        replay_key: "turn_context:developer:available_plugins:0:aaaaaaaaaaaaaaaa".to_string(),
        text_hash: "aaaaaaaaaaaaaaaa".to_string(),
        estimated_tokens: 11,
        policy_class: "capability_inventory",
        include_reason: "available_plugins",
        tier: TurnContextTier::Tool,
    };

    assert_eq!(
        source_aware_budget_candidate_priority(&contribution),
        Some(20)
    );

    contribution.tier = TurnContextTier::Developer;
    assert_eq!(source_aware_budget_candidate_priority(&contribution), None);

    contribution.source_id = "selected_context_recall";
    contribution.tier = TurnContextTier::RetrievedSnippets;
    assert_eq!(
        source_aware_budget_candidate_priority(&contribution),
        Some(50)
    );

    contribution.tier = TurnContextTier::Tool;
    assert_eq!(source_aware_budget_candidate_priority(&contribution), None);

    contribution.source_id = "permissions";
    contribution.tier = TurnContextTier::System;
    assert_eq!(source_aware_budget_candidate_priority(&contribution), None);
}

#[test]
fn turn_context_manifest_records_compression_candidates_without_prompt_mutation() {
    let mut selected_snippets = test_selected_snippet_envelope();
    selected_snippets.snippets[0].text =
        "bounded memory summary with repeated project context and recent decisions".into();
    let selected_snippets = ContextRecallSelectedSnippetEnvelope {
        envelope: selected_snippets,
    };
    let mut context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![
            ContentItem::InputText {
                text: "<permissions instructions>policy</permissions instructions>".to_string(),
            },
            ContentItem::InputText {
                text: format!(
                    "{EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG}\nextension capabilities with repeated tool details\n</extension_developer_capabilities>"
                ),
            },
            ContentItem::InputText {
                text: format!(
                    "{PLUGINS_INSTRUCTIONS_OPEN_TAG}\nplugins registry with repeated capability details\n</plugins_instructions>"
                ),
            },
        ],
        phase: None,
    }];
    context_items.push(
        build_recall_selected_snippets_live_context_item(Some(&selected_snippets))
            .expect("selected snippets should build live context item"),
    );
    let assembly_policy = ContextAssemblyPolicy::from_model_context_window(Some(1));

    let manifest = build_turn_context_manifest_with_policy(&context_items, &assembly_policy)
        .expect("context items should produce manifest");

    assert!(manifest.has_replay_integrity());
    assert_eq!(manifest.entries.len(), 4);
    assert_eq!(manifest.omitted_entries, 0);
    assert!(manifest.omitted_sources.is_empty());
    assert!(!manifest.truncated);
    assert!(manifest.compression_stages.is_empty());
    assert_eq!(manifest.compression_candidates.len(), 3);
    assert_eq!(
        manifest
            .compression_candidates
            .iter()
            .map(|candidate| candidate.source_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "extension_developer_capabilities",
            "available_plugins",
            "selected_context_recall",
        ]
    );
    assert_eq!(
        manifest
            .compression_candidates
            .iter()
            .map(|candidate| candidate.kind)
            .collect::<Vec<_>>(),
        vec![
            TurnContextCompressionStageKind::Prune,
            TurnContextCompressionStageKind::Defragment,
            TurnContextCompressionStageKind::Summary,
        ]
    );
    assert_eq!(
        manifest
            .compression_candidates
            .iter()
            .map(|candidate| candidate.tier)
            .collect::<Vec<_>>(),
        vec![
            TurnContextTier::Tool,
            TurnContextTier::Tool,
            TurnContextTier::RetrievedSnippets,
        ]
    );
    assert!(manifest.compression_candidates.iter().all(|candidate| {
        candidate.estimated_output_tokens < candidate.input_tokens
            && candidate.estimated_tokens_saved() > 0
            && candidate.affected_entries == 1
            && candidate.not_executed_reason
                == TurnContextCompressionCandidateReason::BudgetPressureDryRun
    }));
    assert!(
        !manifest
            .compression_candidates
            .iter()
            .any(|candidate| candidate.source_id == "permissions")
    );
}

#[test]
fn turn_context_decision_entries_parse_typed_kinds() {
    let included = TurnContextDecisionEntry {
        source: "turn_context:developer:permissions:0".to_string(),
        decision: "included:always_include_safety_policy".to_string(),
        reason_hash: None,
    };
    let policy = TurnContextDecisionEntry {
        source: "turn_context:assembly_policy".to_string(),
        decision: "policy:source_aware_omission:budget_exceeded".to_string(),
        reason_hash: None,
    };
    let omitted = TurnContextDecisionEntry {
        source: "turn_context:developer:apps:0:3".to_string(),
        decision: "omitted:apps:priority:30:tokens:9".to_string(),
        reason_hash: None,
    };
    let candidate_omit = TurnContextDecisionEntry {
        source: "turn_context:developer:available_plugins:0:2".to_string(),
        decision: "candidate_omit:available_plugins:priority:20:tokens:11".to_string(),
        reason_hash: None,
    };
    let candidate_truncate = TurnContextDecisionEntry {
        source: "turn_context:developer:selected_context_recall:0".to_string(),
        decision: "candidate_truncate:selected_context_recall:remaining_over_budget:4:tokens:13"
            .to_string(),
        reason_hash: None,
    };
    let truncated = TurnContextDecisionEntry {
        source: "turn_context:developer:selected_context_recall:0".to_string(),
        decision: "truncated:selected_context_recall:original_tokens:24:tokens:3".to_string(),
        reason_hash: None,
    };
    let malformed = TurnContextDecisionEntry {
        source: "turn_context:developer:selected_context_recall:0".to_string(),
        decision: "truncated:selected_context_recall:original_tokens:foo:tokens:3".to_string(),
        reason_hash: None,
    };

    assert!(matches!(
        included.kind(),
        TurnContextDecisionKind::Included { policy_class } if policy_class == "always_include_safety_policy"
    ));
    assert!(matches!(
        policy.kind(),
        TurnContextDecisionKind::Policy {
            strategy,
            budget_state
        } if strategy == "source_aware_omission" && budget_state == "budget_exceeded"
    ));
    assert!(matches!(
        omitted.kind(),
        TurnContextDecisionKind::Omitted {
            source_id,
            priority,
            tokens
        } if source_id == "apps" && priority == 30 && tokens == 9
    ));
    assert!(matches!(
        candidate_omit.kind(),
        TurnContextDecisionKind::CandidateOmit {
            source_id,
            priority,
            tokens
        } if source_id == "available_plugins" && priority == 20 && tokens == 11
    ));
    assert!(matches!(
        candidate_truncate.kind(),
        TurnContextDecisionKind::CandidateTruncate {
            source_id,
            remaining_over_budget,
            tokens
        } if source_id == "selected_context_recall"
            && remaining_over_budget == 4
            && tokens == 13
    ));
    assert!(matches!(
        truncated.kind(),
        TurnContextDecisionKind::Truncated {
            source_id,
            original_tokens,
            tokens
        } if source_id == "selected_context_recall"
            && original_tokens == 24
            && tokens == 3
    ));
    assert!(matches!(
        malformed.kind(),
        TurnContextDecisionKind::Unknown { .. }
    ));
}

#[test]
fn turn_context_manifest_source_aware_omission_policy_omits_low_priority_sources() {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![
            ContentItem::InputText {
                text: "<permissions instructions>policy</permissions instructions>".to_string(),
            },
            ContentItem::InputText {
                text: format!(
                    "{EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG}\nextension caps\n</extension_developer_capabilities>"
                ),
            },
            ContentItem::InputText {
                text: format!("{PLUGINS_INSTRUCTIONS_OPEN_TAG}\nplugins\n</plugins_instructions>"),
            },
            ContentItem::InputText {
                text: format!("{APPS_INSTRUCTIONS_OPEN_TAG}\napps\n</apps_instructions>"),
            },
            ContentItem::InputText {
                text: format!("{SKILLS_INSTRUCTIONS_OPEN_TAG}\nskills\n</skills_instructions>"),
            },
        ],
        phase: None,
    }];
    let ledger = ContextContributionLedger::from_response_items(&context_items);
    let permissions_tokens = ledger
        .contributions()
        .iter()
        .find(|contribution| contribution.source_id == "permissions")
        .expect("permissions contribution should be present")
        .estimated_tokens;
    let assembly_policy = ContextAssemblyPolicy::source_aware_omission_for_model_context_window(
        Some(i64::from(permissions_tokens)),
    );

    let manifest = build_turn_context_manifest_with_policy(&context_items, &assembly_policy)
        .expect("context items should produce manifest");
    let included_decision_sources = manifest
        .decision_ledger
        .iter()
        .filter(|entry| matches!(entry.kind(), TurnContextDecisionKind::Included { .. }))
        .map(|entry| entry.source.as_str())
        .collect::<Vec<_>>();
    let omitted_decisions = manifest
        .decision_ledger
        .iter()
        .filter_map(|entry| {
            let kind = entry.kind();
            matches!(&kind, TurnContextDecisionKind::Omitted { .. })
                .then_some((entry.source.as_str(), kind))
        })
        .collect::<Vec<_>>();

    assert!(manifest.has_replay_integrity());
    assert_eq!(manifest.estimated_tokens, permissions_tokens);
    assert_eq!(manifest.budget_tokens, Some(permissions_tokens));
    assert_eq!(manifest.omitted_entries, 4);
    assert_eq!(
        manifest.omitted_sources,
        vec![
            "turn_context:developer:extension_developer_capabilities:0:1",
            "turn_context:developer:available_plugins:0:2",
            "turn_context:developer:apps:0:3",
            "turn_context:developer:available_skills:0:4",
        ]
    );
    assert!(!manifest.truncated);
    assert_eq!(manifest.entries.len(), 1);
    assert_eq!(
        manifest.entries[0].source,
        "turn_context:developer:permissions:0:0"
    );
    assert_eq!(
        included_decision_sources,
        vec!["turn_context:developer:permissions:0:0"]
    );
    assert!(manifest.decision_ledger.iter().any(|entry| {
        entry.source == "turn_context:assembly_policy"
            && matches!(
                entry.kind(),
                TurnContextDecisionKind::Policy {
                    strategy,
                    budget_state
                } if strategy == "source_aware_omission"
                    && budget_state == "budget_exceeded"
            )
            && entry.reason_hash.is_some()
    }));
    assert_eq!(
        omitted_decisions
            .iter()
            .map(|(source, _)| *source)
            .collect::<Vec<_>>(),
        manifest
            .omitted_sources
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert!(matches!(
        &omitted_decisions[0].1,
        TurnContextDecisionKind::Omitted {
            source_id,
            priority,
            tokens
        } if source_id == "extension_developer_capabilities"
            && *priority == 10
            && *tokens > 0
    ));
    assert!(matches!(
        &omitted_decisions[1].1,
        TurnContextDecisionKind::Omitted {
            source_id,
            priority,
            tokens
        } if source_id == "available_plugins" && *priority == 20 && *tokens > 0
    ));
    assert!(matches!(
        &omitted_decisions[2].1,
        TurnContextDecisionKind::Omitted {
            source_id,
            priority,
            tokens
        } if source_id == "apps" && *priority == 30 && *tokens > 0
    ));
    assert!(matches!(
        &omitted_decisions[3].1,
        TurnContextDecisionKind::Omitted {
            source_id,
            priority,
            tokens
        } if source_id == "available_skills" && *priority == 40 && *tokens > 0
    ));
    assert!(
        !manifest
            .decision_ledger
            .iter()
            .any(|entry| entry.kind().is_candidate_truncation())
    );
}

#[test]
fn assemble_turn_context_with_policy_filters_omitted_prompt_fragments() {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![
            ContentItem::InputText {
                text: "<permissions instructions>policy</permissions instructions>".to_string(),
            },
            ContentItem::InputText {
                text: format!(
                    "{EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG}\nextension caps\n</extension_developer_capabilities>"
                ),
            },
            ContentItem::InputText {
                text: format!("{PLUGINS_INSTRUCTIONS_OPEN_TAG}\nplugins\n</plugins_instructions>"),
            },
            ContentItem::InputText {
                text: format!("{APPS_INSTRUCTIONS_OPEN_TAG}\napps\n</apps_instructions>"),
            },
            ContentItem::InputText {
                text: format!("{SKILLS_INSTRUCTIONS_OPEN_TAG}\nskills\n</skills_instructions>"),
            },
        ],
        phase: None,
    }];
    let permissions_tokens = ContextContributionLedger::from_response_items(&context_items)
        .contributions()
        .iter()
        .find(|contribution| contribution.source_id == "permissions")
        .expect("permissions contribution should be present")
        .estimated_tokens;
    let assembly_policy = ContextAssemblyPolicy::source_aware_omission_for_model_context_window(
        Some(i64::from(permissions_tokens)),
    );

    let result = assemble_turn_context_with_policy(
        &context_items,
        None,
        &TurnContextManifestOptions::default(),
        &assembly_policy,
    );
    let manifest = result
        .context_manifest
        .as_ref()
        .expect("context items should produce manifest");
    let filtered_text = result
        .context_items
        .iter()
        .flat_map(|item| match item {
            ResponseItem::Message { content, .. } => content
                .iter()
                .filter_map(manifest_content_text)
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .collect::<Vec<_>>()
        .join("\n");

    assert!(manifest.has_replay_integrity());
    assert_eq!(result.context_items.len(), 1);
    let ResponseItem::Message { content, .. } = &result.context_items[0] else {
        panic!("filtered context item should remain a message");
    };
    assert_eq!(content.len(), 1);
    assert!(filtered_text.contains("<permissions instructions>policy</permissions instructions>"));
    assert!(!filtered_text.contains("extension caps"));
    assert!(!filtered_text.contains("plugins"));
    assert!(!filtered_text.contains("apps"));
    assert!(!filtered_text.contains("skills"));
    assert_eq!(
        manifest.entries[0].source,
        "turn_context:developer:permissions:0:0"
    );
    assert_eq!(manifest.omitted_entries, 4);
    assert_eq!(
        manifest.omitted_sources,
        vec![
            "turn_context:developer:extension_developer_capabilities:0:1",
            "turn_context:developer:available_plugins:0:2",
            "turn_context:developer:apps:0:3",
            "turn_context:developer:available_skills:0:4",
        ]
    );
}

#[test]
fn assemble_turn_context_with_policy_truncates_prompt_and_manifest_text_hash_together() {
    let mut selected_snippets = test_selected_snippet_envelope();
    selected_snippets.snippets[0].text =
        "bounded memory summary with project preference and recent context details repeated".into();
    let selected_snippets = ContextRecallSelectedSnippetEnvelope {
        envelope: selected_snippets,
    };
    let context_items = vec![
        build_recall_selected_snippets_live_context_item(Some(&selected_snippets))
            .expect("selected snippets should build live context item"),
    ];
    let original_manifest = build_turn_context_manifest(&context_items)
        .expect("selected snippet context should produce manifest");
    let assembly_policy =
        ContextAssemblyPolicy::source_aware_omission_and_truncation_for_model_context_window(Some(
            1,
        ));

    let result = assemble_turn_context_with_policy(
        &context_items,
        None,
        &TurnContextManifestOptions {
            recall_provider_rollup: None,
            recall_selected_snippets: Some(selected_snippets),
            memory_taxonomy: Vec::new(),
            memory_formation_receipts: Vec::new(),
            memory_temporal_facts: Vec::new(),
        },
        &assembly_policy,
    );
    let manifest = result
        .context_manifest
        .as_ref()
        .expect("truncated context should produce manifest");
    let ResponseItem::Message { content, .. } = &result.context_items[0] else {
        panic!("truncated context item should remain a message");
    };
    let ContentItem::InputText {
        text: truncated_text,
    } = &content[0]
    else {
        panic!("truncated context should stay text");
    };
    let expected_text_hash =
        stable_turn_context_manifest_replay_hash(&format!("text:{truncated_text}\n"));

    assert!(manifest.has_replay_integrity());
    assert_eq!(result.context_items.len(), 1);
    assert_eq!(content.len(), 1);
    assert!(truncated_text.contains(LIVE_RECALL_SELECTED_SNIPPETS_HEADER));
    assert!(truncated_text.contains("[context truncated for budget]"));
    assert!(truncated_text.contains(LIVE_RECALL_SELECTED_SNIPPETS_FOOTER));
    assert!(!truncated_text.contains("fedcba9876543210"));
    assert!(!truncated_text.contains("bounded memory summary"));
    assert!(manifest.truncated);
    assert_eq!(manifest.omitted_entries, 0);
    assert!(manifest.omitted_sources.is_empty());
    assert_eq!(manifest.entries.len(), 1);
    assert_eq!(
        manifest.entries[0].source,
        "turn_context:developer:selected_context_recall:0"
    );
    assert_eq!(manifest.entries[0].text_hash, expected_text_hash);
    assert_ne!(
        manifest.entries[0].text_hash,
        original_manifest.entries[0].text_hash
    );
    assert!(manifest.estimated_tokens < original_manifest.estimated_tokens);
    assert!(manifest.decision_ledger.iter().any(|entry| {
        entry.source == manifest.entries[0].source
            && matches!(
                entry.kind(),
                TurnContextDecisionKind::Truncated {
                    source_id,
                    original_tokens,
                    tokens
                } if source_id == "selected_context_recall" && original_tokens > tokens
            )
            && entry.reason_hash.is_some()
    }));
    assert!(!manifest.decision_ledger.iter().any(|entry| {
        entry.source == manifest.entries[0].source && entry.kind().is_candidate_truncation()
    }));
}

#[test]
fn assemble_turn_context_with_policy_summarizes_selected_recall_prompt_and_manifest_text_hash_together()
 {
    let mut selected_snippets = test_selected_snippet_envelope();
    selected_snippets.snippets[0].text =
            "bounded memory summary with project preference, recent context details, and durable recall notes"
                .into();
    let selected_snippets = ContextRecallSelectedSnippetEnvelope {
        envelope: selected_snippets,
    };
    let context_items = vec![
        ResponseItem::Message {
            id: None,
            role: "developer".to_string(),
            content: vec![ContentItem::InputText {
                text: format!(
                    "{PLUGINS_INSTRUCTIONS_OPEN_TAG}\nplugins registry with repeated capability details\n</plugins_instructions>"
                ),
            }],
            phase: None,
        },
        build_recall_selected_snippets_live_context_item(Some(&selected_snippets))
            .expect("selected snippets should build live context item"),
    ];
    let original_manifest = build_turn_context_manifest(&context_items)
        .expect("selected snippet context should produce manifest");
    let assembly_policy =
        ContextAssemblyPolicy::source_aware_summary_for_model_context_window(Some(1));

    let result = assemble_turn_context_with_policy(
        &context_items,
        None,
        &TurnContextManifestOptions {
            recall_provider_rollup: None,
            recall_selected_snippets: Some(selected_snippets),
            memory_taxonomy: Vec::new(),
            memory_formation_receipts: Vec::new(),
            memory_temporal_facts: Vec::new(),
        },
        &assembly_policy,
    );
    let manifest = result
        .context_manifest
        .as_ref()
        .expect("summarized context should produce manifest");
    let summarized_text = result
        .context_items
        .iter()
        .flat_map(|item| match item {
            ResponseItem::Message { content, .. } => content
                .iter()
                .filter_map(manifest_content_text)
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .find(|text| text.starts_with(LIVE_RECALL_SELECTED_SNIPPETS_HEADER))
        .expect("selected context recall should remain as summarized text");
    let expected_text_hash =
        stable_turn_context_manifest_replay_hash(&format!("text:{summarized_text}\n"));
    let selected_entry = manifest
        .entries
        .iter()
        .find(|entry| entry.source == "turn_context:developer:selected_context_recall:1")
        .expect("selected recall manifest entry should remain");
    let original_selected_entry = original_manifest
        .entries
        .iter()
        .find(|entry| entry.source == selected_entry.source)
        .expect("original selected recall manifest entry should exist");

    assert!(manifest.has_replay_integrity());
    assert_eq!(result.context_items.len(), 2);
    assert!(summarized_text.contains(LIVE_RECALL_SELECTED_SNIPPETS_HEADER));
    assert!(summarized_text.contains("[context summarized for budget]"));
    assert!(summarized_text.contains(LIVE_RECALL_SELECTED_SNIPPETS_FOOTER));
    assert!(!summarized_text.contains("fedcba9876543210"));
    assert!(!summarized_text.contains("bounded memory summary"));
    assert!(!manifest.truncated);
    assert_eq!(manifest.omitted_entries, 0);
    assert!(manifest.omitted_sources.is_empty());
    assert_eq!(manifest.compression_stages.len(), 1);
    assert_eq!(
        manifest.compression_stages[0].kind,
        TurnContextCompressionStageKind::Summary
    );
    assert_eq!(
        manifest.compression_stages[0].input_tokens,
        original_selected_entry.estimated_tokens
    );
    assert_eq!(
        manifest.compression_stages[0].output_tokens,
        selected_entry.estimated_tokens
    );
    assert_eq!(manifest.compression_stages[0].affected_entries, 1);
    assert!(manifest.compression_stages[0].tokens_saved() > 0);
    assert_compression_stage_evidence(
        &manifest.compression_stages[0],
        &original_selected_entry.text_hash,
    );
    assert_eq!(selected_entry.text_hash, expected_text_hash);
    assert_ne!(selected_entry.text_hash, original_selected_entry.text_hash);
    assert!(manifest.estimated_tokens < original_manifest.estimated_tokens);
    assert!(
        !manifest
            .compression_candidates
            .iter()
            .any(|candidate| candidate.source_id == "selected_context_recall")
    );
    assert_eq!(
        manifest
            .compression_candidates
            .iter()
            .map(|candidate| (candidate.source_id.as_str(), candidate.kind))
            .collect::<Vec<_>>(),
        vec![(
            "available_plugins",
            TurnContextCompressionStageKind::Defragment
        )]
    );
}

#[test]
fn assemble_turn_context_with_policy_defragments_tool_inventory_prompt_and_manifest_text_hash_together()
 {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: format!(
                "{PLUGINS_INSTRUCTIONS_OPEN_TAG}\n\
                    plugin alpha: repeated capability metadata for shell, files, docs, and search\n\
                    plugin beta: repeated capability metadata for shell, files, docs, and search\n\
                    plugin gamma: repeated capability metadata for shell, files, docs, and search\n\
                    {PLUGINS_INSTRUCTIONS_CLOSE_TAG}"
            ),
        }],
        phase: None,
    }];
    let original_manifest = build_turn_context_manifest(&context_items)
        .expect("tool inventory context should produce manifest");
    let assembly_policy =
        ContextAssemblyPolicy::source_aware_tool_defragment_for_model_context_window(Some(1));

    let result = assemble_turn_context_with_policy(
        &context_items,
        None,
        &TurnContextManifestOptions::default(),
        &assembly_policy,
    );
    let manifest = result
        .context_manifest
        .as_ref()
        .expect("defragmented context should produce manifest");
    let defragmented_text = result
        .context_items
        .iter()
        .flat_map(|item| match item {
            ResponseItem::Message { content, .. } => content
                .iter()
                .filter_map(manifest_content_text)
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .find(|text| text.starts_with(PLUGINS_INSTRUCTIONS_OPEN_TAG))
        .expect("available plugins should remain as defragmented text");
    let expected_text_hash =
        stable_turn_context_manifest_replay_hash(&format!("text:{defragmented_text}\n"));
    let plugins_entry = manifest
        .entries
        .iter()
        .find(|entry| entry.source == "turn_context:developer:available_plugins:0")
        .expect("plugins manifest entry should remain");
    let original_plugins_entry = original_manifest
        .entries
        .iter()
        .find(|entry| entry.source == plugins_entry.source)
        .expect("original plugins manifest entry should exist");

    assert!(manifest.has_replay_integrity());
    assert_eq!(result.context_items.len(), 1);
    assert!(defragmented_text.contains(PLUGINS_INSTRUCTIONS_OPEN_TAG));
    assert!(defragmented_text.contains("[context defragmented for budget]"));
    assert!(defragmented_text.contains(PLUGINS_INSTRUCTIONS_CLOSE_TAG));
    assert!(!defragmented_text.contains("plugin alpha"));
    assert!(!manifest.truncated);
    assert_eq!(manifest.omitted_entries, 0);
    assert!(manifest.omitted_sources.is_empty());
    assert_eq!(manifest.compression_stages.len(), 1);
    assert_eq!(
        manifest.compression_stages[0].kind,
        TurnContextCompressionStageKind::Defragment
    );
    assert_eq!(
        manifest.compression_stages[0].input_tokens,
        original_plugins_entry.estimated_tokens
    );
    assert_eq!(
        manifest.compression_stages[0].output_tokens,
        plugins_entry.estimated_tokens
    );
    assert_eq!(manifest.compression_stages[0].affected_entries, 1);
    assert!(manifest.compression_stages[0].tokens_saved() > 0);
    assert_compression_stage_evidence(
        &manifest.compression_stages[0],
        &original_plugins_entry.text_hash,
    );
    assert_eq!(plugins_entry.text_hash, expected_text_hash);
    assert_ne!(plugins_entry.text_hash, original_plugins_entry.text_hash);
    assert!(manifest.estimated_tokens < original_manifest.estimated_tokens);
    assert!(
        !manifest
            .compression_candidates
            .iter()
            .any(|candidate| candidate.source_id == "available_plugins")
    );
}

#[test]
fn assemble_turn_context_with_policy_prunes_extension_developer_capabilities_prompt_and_manifest_text_hash_together()
 {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: format!(
                "{EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG}\n\
                    extension capability alpha: repeated tool dispatch metadata and routing hints\n\
                    extension capability beta: repeated tool dispatch metadata and routing hints\n\
                    extension capability gamma: repeated tool dispatch metadata and routing hints\n\
                    {EXTENSION_DEVELOPER_CAPABILITIES_CLOSE_TAG}"
            ),
        }],
        phase: None,
    }];
    let original_manifest = build_turn_context_manifest(&context_items)
        .expect("extension capabilities context should produce manifest");
    let assembly_policy =
        ContextAssemblyPolicy::source_aware_tool_prune_for_model_context_window(Some(1));

    let result = assemble_turn_context_with_policy(
        &context_items,
        None,
        &TurnContextManifestOptions::default(),
        &assembly_policy,
    );
    let manifest = result
        .context_manifest
        .as_ref()
        .expect("pruned context should produce manifest");
    let pruned_text = result
        .context_items
        .iter()
        .flat_map(|item| match item {
            ResponseItem::Message { content, .. } => content
                .iter()
                .filter_map(manifest_content_text)
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .find(|text| text.starts_with(EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG))
        .expect("extension capabilities should remain as pruned text");
    let expected_text_hash =
        stable_turn_context_manifest_replay_hash(&format!("text:{pruned_text}\n"));
    let capabilities_entry = manifest
        .entries
        .iter()
        .find(|entry| entry.source == "turn_context:developer:extension_developer_capabilities:0")
        .expect("extension capabilities manifest entry should remain");
    let original_capabilities_entry = original_manifest
        .entries
        .iter()
        .find(|entry| entry.source == capabilities_entry.source)
        .expect("original extension capabilities manifest entry should exist");

    assert!(manifest.has_replay_integrity());
    assert_eq!(result.context_items.len(), 1);
    assert!(pruned_text.contains(EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG));
    assert!(pruned_text.contains("[context pruned for budget]"));
    assert!(pruned_text.contains(EXTENSION_DEVELOPER_CAPABILITIES_CLOSE_TAG));
    assert!(!pruned_text.contains("extension capability alpha"));
    assert!(!manifest.truncated);
    assert_eq!(manifest.omitted_entries, 0);
    assert!(manifest.omitted_sources.is_empty());
    assert_eq!(manifest.compression_stages.len(), 1);
    assert_eq!(
        manifest.compression_stages[0].kind,
        TurnContextCompressionStageKind::Prune
    );
    assert_eq!(
        manifest.compression_stages[0].input_tokens,
        original_capabilities_entry.estimated_tokens
    );
    assert_eq!(
        manifest.compression_stages[0].output_tokens,
        capabilities_entry.estimated_tokens
    );
    assert_eq!(manifest.compression_stages[0].affected_entries, 1);
    assert!(manifest.compression_stages[0].tokens_saved() > 0);
    assert_compression_stage_evidence(
        &manifest.compression_stages[0],
        &original_capabilities_entry.text_hash,
    );
    assert_eq!(capabilities_entry.text_hash, expected_text_hash);
    assert_ne!(
        capabilities_entry.text_hash,
        original_capabilities_entry.text_hash
    );
    assert!(manifest.estimated_tokens < original_manifest.estimated_tokens);
    assert!(
        !manifest
            .compression_candidates
            .iter()
            .any(|candidate| { candidate.source_id == "extension_developer_capabilities" })
    );
}

#[test]
fn assemble_turn_context_with_policy_executes_summary_defragment_and_prune_together() {
    let mut selected_snippets = test_selected_snippet_envelope();
    selected_snippets.snippets[0].text =
            "bounded memory summary with repeated project context, recent decisions, durable recall notes, and handoff details"
                .into();
    let selected_snippets = ContextRecallSelectedSnippetEnvelope {
        envelope: selected_snippets,
    };
    let context_items = vec![
        build_recall_selected_snippets_live_context_item(Some(&selected_snippets))
            .expect("selected snippets should build live context item"),
        ResponseItem::Message {
            id: None,
            role: "developer".to_string(),
            content: vec![
                ContentItem::InputText {
                    text: format!(
                        "{PLUGINS_INSTRUCTIONS_OPEN_TAG}\n\
                            plugin alpha: repeated capability metadata for shell, files, docs, and search\n\
                            plugin beta: repeated capability metadata for shell, files, docs, and search\n\
                            plugin gamma: repeated capability metadata for shell, files, docs, and search\n\
                            {PLUGINS_INSTRUCTIONS_CLOSE_TAG}"
                    ),
                },
                ContentItem::InputText {
                    text: format!(
                        "{EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG}\n\
                            extension capability alpha: repeated tool dispatch metadata and routing hints\n\
                            extension capability beta: repeated tool dispatch metadata and routing hints\n\
                            extension capability gamma: repeated tool dispatch metadata and routing hints\n\
                            {EXTENSION_DEVELOPER_CAPABILITIES_CLOSE_TAG}"
                    ),
                },
            ],
            phase: None,
        },
    ];
    let original_manifest = build_turn_context_manifest(&context_items)
        .expect("compressible context should produce manifest");
    let assembly_policy =
        ContextAssemblyPolicy::source_aware_compression_for_model_context_window(Some(1));

    let result = assemble_turn_context_with_policy(
        &context_items,
        None,
        &TurnContextManifestOptions {
            recall_provider_rollup: None,
            recall_selected_snippets: Some(selected_snippets),
            memory_taxonomy: Vec::new(),
            memory_formation_receipts: Vec::new(),
            memory_temporal_facts: Vec::new(),
        },
        &assembly_policy,
    );
    let manifest = result
        .context_manifest
        .as_ref()
        .expect("compressed context should produce manifest");
    let rewritten_texts = result
        .context_items
        .iter()
        .flat_map(|item| match item {
            ResponseItem::Message { content, .. } => content
                .iter()
                .filter_map(manifest_content_text)
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .collect::<Vec<_>>();
    let summarized_text = rewritten_texts
        .iter()
        .find(|text| text.starts_with(LIVE_RECALL_SELECTED_SNIPPETS_HEADER))
        .expect("selected recall should remain as summarized text");
    let defragmented_text = rewritten_texts
        .iter()
        .find(|text| text.starts_with(PLUGINS_INSTRUCTIONS_OPEN_TAG))
        .expect("plugins should remain as defragmented text");
    let pruned_text = rewritten_texts
        .iter()
        .find(|text| text.starts_with(EXTENSION_DEVELOPER_CAPABILITIES_OPEN_TAG))
        .expect("extension capabilities should remain as pruned text");
    let selected_entry = manifest
        .entries
        .iter()
        .find(|entry| entry.source.contains(":selected_context_recall:"))
        .expect("selected recall entry should remain");
    let plugins_entry = manifest
        .entries
        .iter()
        .find(|entry| entry.source.contains(":available_plugins:"))
        .expect("plugins entry should remain");
    let capabilities_entry = manifest
        .entries
        .iter()
        .find(|entry| entry.source.contains(":extension_developer_capabilities:"))
        .expect("extension capabilities entry should remain");
    let original_selected_entry = original_manifest
        .entries
        .iter()
        .find(|entry| entry.source == selected_entry.source)
        .expect("original selected recall entry should exist");
    let original_plugins_entry = original_manifest
        .entries
        .iter()
        .find(|entry| entry.source == plugins_entry.source)
        .expect("original plugins entry should exist");
    let original_capabilities_entry = original_manifest
        .entries
        .iter()
        .find(|entry| entry.source == capabilities_entry.source)
        .expect("original capabilities entry should exist");

    assert!(manifest.has_replay_integrity());
    assert_eq!(result.context_items.len(), 2);
    assert!(summarized_text.contains("[context summarized for budget]"));
    assert!(defragmented_text.contains("[context defragmented for budget]"));
    assert!(pruned_text.contains("[context pruned for budget]"));
    assert!(!summarized_text.contains("bounded memory summary"));
    assert!(!defragmented_text.contains("plugin alpha"));
    assert!(!pruned_text.contains("extension capability alpha"));
    assert!(!manifest.truncated);
    assert_eq!(manifest.omitted_entries, 0);
    assert!(manifest.omitted_sources.is_empty());
    assert_eq!(manifest.compression_stages.len(), 3);
    assert_eq!(
        manifest
            .compression_stages
            .iter()
            .map(|stage| stage.kind)
            .collect::<Vec<_>>(),
        vec![
            TurnContextCompressionStageKind::Summary,
            TurnContextCompressionStageKind::Defragment,
            TurnContextCompressionStageKind::Prune,
        ]
    );
    assert_eq!(
        manifest.compression_stages[0].input_tokens,
        original_selected_entry.estimated_tokens
    );
    assert_eq!(
        manifest.compression_stages[0].output_tokens,
        selected_entry.estimated_tokens
    );
    assert_eq!(
        manifest.compression_stages[1].input_tokens,
        original_plugins_entry.estimated_tokens
    );
    assert_eq!(
        manifest.compression_stages[1].output_tokens,
        plugins_entry.estimated_tokens
    );
    assert_eq!(
        manifest.compression_stages[2].input_tokens,
        original_capabilities_entry.estimated_tokens
    );
    assert_eq!(
        manifest.compression_stages[2].output_tokens,
        capabilities_entry.estimated_tokens
    );
    assert!(
        manifest
            .compression_stages
            .iter()
            .all(|stage| { stage.affected_entries == 1 && stage.tokens_saved() > 0 })
    );
    for (stage, original_entry) in manifest.compression_stages.iter().zip([
        original_selected_entry,
        original_plugins_entry,
        original_capabilities_entry,
    ]) {
        assert_compression_stage_evidence(stage, &original_entry.text_hash);
    }
    assert_eq!(
        selected_entry.text_hash,
        stable_turn_context_manifest_replay_hash(&format!("text:{summarized_text}\n"))
    );
    assert_eq!(
        plugins_entry.text_hash,
        stable_turn_context_manifest_replay_hash(&format!("text:{defragmented_text}\n"))
    );
    assert_eq!(
        capabilities_entry.text_hash,
        stable_turn_context_manifest_replay_hash(&format!("text:{pruned_text}\n"))
    );
    assert!(manifest.estimated_tokens < original_manifest.estimated_tokens);
    assert!(manifest.compression_candidates.is_empty());
}

#[test]
fn assemble_turn_context_with_policy_does_not_compress_protected_tiers() {
    let context_items = vec![
            ResponseItem::Message {
                id: None,
                role: "developer".to_string(),
                content: vec![
                    ContentItem::InputText {
                        text: "<permissions instructions>\nprotected system policy with repeated permission and approval guidance\n</permissions instructions>".to_string(),
                    },
                    ContentItem::InputText {
                        text: format!(
                            "{COLLABORATION_MODE_OPEN_TAG}\nprotected collaboration-mode developer guidance\n</collaboration_mode>"
                        ),
                    },
                    ContentItem::InputText {
                        text: "<personality_spec>\nprotected personality guidance\n</personality_spec>".to_string(),
                    },
                    ContentItem::InputText {
                        text: format!(
                            "{EXTENSION_DEVELOPER_POLICY_OPEN_TAG}\nprotected extension developer policy\n</extension_developer_policy>"
                        ),
                    },
                    ContentItem::InputText {
                        text: format!(
                            "{EXTENSION_SEPARATE_DEVELOPER_OPEN_TAG}\nprotected extension separate developer instructions\n</extension_separate_developer>"
                        ),
                    },
                    ContentItem::InputText {
                        text: "protected plain developer instructions".to_string(),
                    },
                    ContentItem::InputText {
                        text: "<model_switch>\nprotected session-state model switch\n</model_switch>".to_string(),
                    },
                    ContentItem::InputText {
                        text: format!(
                            "{REALTIME_CONVERSATION_OPEN_TAG}\nprotected runtime realtime state\n</realtime_conversation>"
                        ),
                    },
                ],
                phase: None,
            },
            ResponseItem::Message {
                id: None,
                role: "user".to_string(),
                content: vec![
                    ContentItem::InputText {
                        text: "# AGENTS.md instructions for /tmp\nprotected workspace user instructions".to_string(),
                    },
                    ContentItem::InputText {
                        text: format!(
                            "{EXTENSION_CONTEXTUAL_USER_OPEN_TAG}\nprotected extension contextual user instructions\n</extension_contextual_user>"
                        ),
                    },
                    ContentItem::InputText {
                        text: format!(
                            "{ENVIRONMENT_CONTEXT_OPEN_TAG}\n<cwd>/tmp</cwd>\n</environment_context>"
                        ),
                    },
                    ContentItem::InputImage {
                        image_url: "data:image/png;base64,AAAA".to_string(),
                        detail: None,
                    },
                ],
                phase: None,
            },
        ];
    let original_manifest =
        build_turn_context_manifest(&context_items).expect("protected context has manifest");
    let assembly_policy =
        ContextAssemblyPolicy::source_aware_compression_for_model_context_window(Some(1));

    let result = assemble_turn_context_with_policy(
        &context_items,
        None,
        &TurnContextManifestOptions::default(),
        &assembly_policy,
    );
    let manifest = result
        .context_manifest
        .as_ref()
        .expect("protected context should keep manifest");
    let rendered_context =
        serde_json::to_string(&result.context_items).expect("context should serialize");

    assert_eq!(result.context_items, context_items);
    assert!(manifest.has_replay_integrity());
    assert_eq!(manifest.entries, original_manifest.entries);
    assert_eq!(
        manifest.estimated_tokens,
        original_manifest.estimated_tokens
    );
    assert_eq!(manifest.budget_tokens, Some(1));
    assert_eq!(manifest.omitted_entries, 0);
    assert!(manifest.omitted_sources.is_empty());
    assert!(!manifest.truncated);
    assert!(manifest.compression_candidates.is_empty());
    assert!(manifest.compression_stages.is_empty());
    assert!(manifest.decision_ledger.iter().any(|entry| {
        matches!(
            entry.kind(),
            TurnContextDecisionKind::Policy {
                strategy,
                budget_state,
            } if strategy == "source_aware_compression" && budget_state == "budget_exceeded"
        )
    }));
    assert!(manifest.decision_ledger.iter().all(|entry| {
        !matches!(
            entry.kind(),
            TurnContextDecisionKind::CandidateOmit { .. }
                | TurnContextDecisionKind::CandidateTruncate { .. }
                | TurnContextDecisionKind::Omitted { .. }
                | TurnContextDecisionKind::Truncated { .. }
        )
    }));
    assert!(
        manifest
            .entries
            .iter()
            .any(|entry| entry.tier == TurnContextTier::System)
            && manifest
                .entries
                .iter()
                .any(|entry| entry.tier == TurnContextTier::Developer)
            && manifest
                .entries
                .iter()
                .any(|entry| entry.tier == TurnContextTier::User)
            && manifest
                .entries
                .iter()
                .any(|entry| entry.tier == TurnContextTier::Runtime)
            && manifest
                .entries
                .iter()
                .any(|entry| entry.tier == TurnContextTier::SessionState)
    );
    assert!(!rendered_context.contains("[context summarized for budget]"));
    assert!(!rendered_context.contains("[context defragmented for budget]"));
    assert!(!rendered_context.contains("[context pruned for budget]"));
}

#[test]
fn turn_context_manifest_builds_contribution_ledger_per_visible_fragment() {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![
            ContentItem::InputText {
                text: "first policy body".to_string(),
            },
            ContentItem::InputText {
                text: "second policy body".to_string(),
            },
        ],
        phase: None,
    }];

    let ledger = ContextContributionLedger::from_response_items(&context_items);
    let manifest =
        build_turn_context_manifest(&context_items).expect("context items should produce manifest");
    let manifest_json = serde_json::to_string(&manifest).expect("manifest should serialize");

    assert!(manifest.has_replay_integrity());
    assert_eq!(ledger.contributions().len(), 2);
    assert_eq!(manifest.entries.len(), 2);
    assert_eq!(manifest.decision_ledger.len(), 3);
    assert_eq!(
        manifest.entries[0].source,
        "turn_context:developer:developer_instructions:0:0"
    );
    assert_eq!(
        manifest.entries[1].source,
        "turn_context:developer:developer_instructions:0:1"
    );
    assert_eq!(
        manifest
            .decision_ledger
            .iter()
            .filter(|entry| entry.source != "turn_context:assembly_policy")
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>()
    );
    assert!(
        ledger
            .contributions()
            .iter()
            .all(|entry| entry.policy_class == "always_include_developer")
    );
    assert!(!manifest_json.contains("first policy body"));
    assert!(!manifest_json.contains("second policy body"));
}

#[test]
fn turn_context_manifest_classifies_key_context_contribution_sources() {
    let context_items = vec![
            ResponseItem::Message {
                id: None,
                role: "developer".to_string(),
                content: vec![ContentItem::InputText {
                    text: "<permissions instructions>policy</permissions instructions>"
                        .to_string(),
                }],
                phase: None,
            },
            ResponseItem::Message {
                id: None,
                role: "user".to_string(),
                content: vec![ContentItem::InputText {
                    text: "<environment_context>\n  <cwd>/tmp</cwd>\n</environment_context>"
                        .to_string(),
                }],
                phase: None,
            },
            ResponseItem::Message {
                id: None,
                role: "developer".to_string(),
                content: vec![ContentItem::InputText {
                    text: "<selected_context_recall>\n- snippet_hash=fedcba9876543210 text: bounded memory\n</selected_context_recall>"
                        .to_string(),
                }],
                phase: None,
            },
        ];

    let ledger = ContextContributionLedger::from_response_items(&context_items);
    let manifest =
        build_turn_context_manifest(&context_items).expect("context items should produce manifest");

    assert_eq!(
        ledger
            .contributions()
            .iter()
            .map(|entry| (entry.source_id, entry.policy_class))
            .collect::<Vec<_>>(),
        vec![
            ("permissions", "always_include_safety_policy"),
            ("environment", "turn_environment"),
            ("selected_context_recall", "bounded_recall"),
        ]
    );
    assert_eq!(
        manifest
            .entries
            .iter()
            .map(|entry| entry.source.as_str())
            .collect::<Vec<_>>(),
        vec![
            "turn_context:developer:permissions:0",
            "turn_context:contextual_user:environment:1",
            "turn_context:developer:selected_context_recall:2",
        ]
    );
    assert_eq!(
        manifest
            .decision_ledger
            .iter()
            .map(|entry| entry.decision.as_str())
            .collect::<Vec<_>>(),
        vec![
            "included:always_include_safety_policy",
            "included:turn_environment",
            "included:bounded_recall",
            "policy:non_omitting_replay_baseline:within_budget",
        ]
    );
}

#[test]
fn turn_context_manifest_resolves_recall_provider_rollup_without_payload_text() {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: "secret policy body".to_string(),
        }],
        phase: None,
    }];
    let previous_manifest =
        build_turn_context_manifest(&context_items).expect("manifest should build");
    let previous_ledger = previous_manifest.ledger_hash.clone();
    let recall_selection = TurnContextRecallSelectionSummary {
        returned_source_count: 2,
        selected_source_count: 2,
        ranked_source_count: 0,
        returned_unselected_source_count: 0,
        source_diversity_met: true,
        source_diversity_target: 2,
        max_per_source: 2,
        ranked_item_count: 0,
        omitted_by_budget_count: 0,
        memory_control_omitted_count: 1,
        low_trust_ranked_item_count: 0,
        low_recency_ranked_item_count: 0,
    };
    let options = TurnContextManifestOptions {
        recall_provider_rollup: Some(ContextRecallProviderRollup {
            recall_selection: recall_selection.clone(),
        }),
        recall_selected_snippets: None,
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: Vec::new(),
    };

    let resolved = resolve_turn_context_manifest(&[], Some(&previous_manifest), &options)
        .expect("previous manifest should carry forward");
    let resolved_json = serde_json::to_string(&resolved).expect("manifest should serialize");

    assert!(resolved.has_replay_integrity());
    assert_eq!(resolved.recall_selection.as_ref(), Some(&recall_selection));
    assert_ne!(resolved.ledger_hash, previous_ledger);
    assert!(!resolved_json.contains("secret policy body"));
}

#[test]
fn turn_context_manifest_resolves_selected_snippets_as_guarded_payload() {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: "secret policy body".to_string(),
        }],
        phase: None,
    }];
    let previous_manifest =
        build_turn_context_manifest(&context_items).expect("manifest should build");
    let previous_ledger = previous_manifest.ledger_hash.clone();
    let selected_snippets = test_selected_snippet_envelope();
    let options = TurnContextManifestOptions {
        recall_provider_rollup: None,
        recall_selected_snippets: Some(ContextRecallSelectedSnippetEnvelope {
            envelope: selected_snippets.clone(),
        }),
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: Vec::new(),
    };

    let resolved = resolve_turn_context_manifest(&[], Some(&previous_manifest), &options)
        .expect("previous manifest should carry forward");
    let resolved_json = serde_json::to_string(&resolved).expect("manifest should serialize");

    assert!(resolved.has_replay_integrity());
    assert_eq!(
        resolved.recall_selected_snippets.as_ref(),
        Some(&selected_snippets)
    );
    assert_ne!(resolved.ledger_hash, previous_ledger);
    assert!(!resolved_json.contains("secret policy body"));
    assert!(!resolved_json.contains("source-memory-id"));
    assert!(!resolved_json.contains("[hepta-memory:"));
    assert!(!resolved_json.contains("needle"));
}

#[test]
fn turn_context_manifest_resolves_memory_taxonomy_without_payload_text() {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: "secret policy body".to_string(),
        }],
        phase: None,
    }];
    let previous_manifest =
        build_turn_context_manifest(&context_items).expect("manifest should build");
    let previous_ledger = previous_manifest.ledger_hash.clone();
    let memory_taxonomy = vec![
        TurnContextMemoryTaxonomyBucket {
            class: TurnContextMemoryTaxonomyClass::Semantic,
            source_count: 1,
            returned_count: 2,
            available_count: 3,
            omitted_count: 1,
            provenance_span_count: 0,
        },
        TurnContextMemoryTaxonomyBucket {
            class: TurnContextMemoryTaxonomyClass::Episodic,
            source_count: 1,
            returned_count: 1,
            available_count: 1,
            omitted_count: 0,
            provenance_span_count: 0,
        },
        TurnContextMemoryTaxonomyBucket {
            class: TurnContextMemoryTaxonomyClass::Control,
            source_count: 1,
            returned_count: 0,
            available_count: 2,
            omitted_count: 2,
            provenance_span_count: 0,
        },
        TurnContextMemoryTaxonomyBucket {
            class: TurnContextMemoryTaxonomyClass::Transcript,
            source_count: 2,
            returned_count: 3,
            available_count: 5,
            omitted_count: 2,
            provenance_span_count: 2,
        },
    ];
    let options = TurnContextManifestOptions {
        recall_provider_rollup: None,
        recall_selected_snippets: None,
        memory_taxonomy: memory_taxonomy.clone(),
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: Vec::new(),
    };

    let resolved = resolve_turn_context_manifest(&[], Some(&previous_manifest), &options)
        .expect("previous manifest should carry forward");
    let resolved_json = serde_json::to_string(&resolved).expect("manifest should serialize");

    assert!(resolved.has_replay_integrity());
    assert_eq!(resolved.memory_taxonomy, memory_taxonomy);
    assert_ne!(resolved.ledger_hash, previous_ledger);
    assert!(!resolved_json.contains("secret policy body"));
    assert!(!resolved_json.contains("memory_id"));
    assert!(!resolved_json.contains("source_id"));
    assert!(!resolved_json.contains("query"));
}

#[test]
fn turn_context_manifest_resolves_memory_formation_receipts_without_payload_text() {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: "secret policy body".to_string(),
        }],
        phase: None,
    }];
    let previous_manifest =
        build_turn_context_manifest(&context_items).expect("manifest should build");
    let previous_ledger = previous_manifest.ledger_hash.clone();
    let receipts = vec![TurnContextMemoryFormationReceipt {
        candidate_type: TurnContextMemoryFormationCandidateType::Fact,
        transcript_span_count: 2,
        provenance_span_count: 2,
        confidence_basis_points: 6400,
        idempotency_key_hash: "0123456789abcdef".into(),
        privacy_class: "user_private".into(),
        queued_for_background: true,
        production_write: false,
    }];
    let options = TurnContextManifestOptions {
        recall_provider_rollup: None,
        recall_selected_snippets: None,
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: receipts.clone(),
        memory_temporal_facts: Vec::new(),
    };

    let resolved = resolve_turn_context_manifest(&[], Some(&previous_manifest), &options)
        .expect("previous manifest should carry forward");
    let resolved_json = serde_json::to_string(&resolved).expect("manifest should serialize");

    assert!(resolved.has_replay_integrity());
    assert_eq!(resolved.memory_formation_receipts, receipts);
    assert_ne!(resolved.ledger_hash, previous_ledger);
    assert!(!resolved_json.contains("secret policy body"));
    assert!(!resolved_json.contains("transcript_text"));
    assert!(!resolved_json.contains("memory_id"));
    assert!(!resolved_json.contains("source_id"));
    assert!(!resolved_json.contains("query"));
}

#[test]
fn turn_context_manifest_resolves_memory_temporal_facts_without_payload_text() {
    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: "secret policy body".to_string(),
        }],
        phase: None,
    }];
    let previous_manifest =
        build_turn_context_manifest(&context_items).expect("manifest should build");
    let previous_ledger = previous_manifest.ledger_hash.clone();
    let temporal_facts = vec![TurnContextMemoryTemporalFact {
        fact_type: TurnContextMemoryTemporalFactType::Attribute,
        entity_hash: "0123456789abcdef".into(),
        provenance_span_count: 2,
        valid_from_sequence: 8,
        invalid_at_sequence: None,
        confidence_basis_points: 6200,
        supersedes_fact_hash: None,
        privacy_class: "user_private".into(),
        dry_run_only: true,
        production_write: false,
    }];
    let options = TurnContextManifestOptions {
        recall_provider_rollup: None,
        recall_selected_snippets: None,
        memory_taxonomy: Vec::new(),
        memory_formation_receipts: Vec::new(),
        memory_temporal_facts: temporal_facts.clone(),
    };

    let resolved = resolve_turn_context_manifest(&[], Some(&previous_manifest), &options)
        .expect("previous manifest should carry forward");
    let resolved_json = serde_json::to_string(&resolved).expect("manifest should serialize");

    assert!(resolved.has_replay_integrity());
    assert_eq!(resolved.memory_temporal_facts, temporal_facts);
    assert_ne!(resolved.ledger_hash, previous_ledger);
    assert!(!resolved_json.contains("secret policy body"));
    assert!(!resolved_json.contains("fact_text"));
    assert!(!resolved_json.contains("transcript_text"));
    assert!(!resolved_json.contains("memory_text"));
    assert!(!resolved_json.contains("source_id"));
    assert!(!resolved_json.contains("memory_id"));
    assert!(!resolved_json.contains("query"));
}

#[test]
fn turn_context_manifest_builds_guarded_selected_snippets_live_context_item() {
    let selected_snippets = ContextRecallSelectedSnippetEnvelope {
        envelope: test_selected_snippet_envelope(),
    };

    let item = build_recall_selected_snippets_live_context_item(Some(&selected_snippets))
        .expect("valid selected snippets should produce live context item");
    let ResponseItem::Message { role, content, .. } = item else {
        panic!("expected selected snippets context to be a message");
    };
    let ContentItem::InputText { text } = &content[0] else {
        panic!("expected selected snippets context text");
    };

    assert_eq!(role, "developer");
    assert!(text.contains(LIVE_RECALL_SELECTED_SNIPPETS_HEADER));
    assert!(text.contains("fedcba9876543210"));
    assert!(text.contains("[redacted-query] bounded memory"));
    assert!(!text.contains("source-memory-id"));
    assert!(!text.contains("source_id"));
    assert!(!text.contains("[hepta-memory:"));
    assert!(!text.contains("needle"));
}

#[test]
fn turn_context_manifest_rejects_selected_snippets_live_context_item_with_forbidden_payload() {
    let extension_data = ExtensionData::new("turn-1");
    let mut selected_snippets = test_selected_snippet_envelope();
    selected_snippets.snippets[0].text = "source_id leaked into bounded snippet".into();
    extension_data.insert(selected_snippets.clone());

    let item = build_recall_selected_snippets_live_context_item(Some(
        &ContextRecallSelectedSnippetEnvelope {
            envelope: selected_snippets,
        },
    ));
    let options = turn_context_manifest_options_from_extension_data(&extension_data);

    assert!(item.is_none());
    assert_eq!(options.recall_selected_snippets, None);
}

#[test]
fn turn_context_manifest_options_read_valid_recall_rollup_from_extension_data() {
    let extension_data = ExtensionData::new("turn-1");
    let recall_selection = TurnContextRecallSelectionSummary {
        returned_source_count: 3,
        selected_source_count: 2,
        ranked_source_count: 2,
        returned_unselected_source_count: 1,
        source_diversity_met: true,
        source_diversity_target: 2,
        max_per_source: 2,
        ranked_item_count: 3,
        omitted_by_budget_count: 1,
        memory_control_omitted_count: 1,
        low_trust_ranked_item_count: 1,
        low_recency_ranked_item_count: 0,
    };
    let selected_snippets = test_selected_snippet_envelope();
    extension_data.insert(recall_selection.clone());
    extension_data.insert(selected_snippets.clone());

    let options = turn_context_manifest_options_from_extension_data(&extension_data);

    assert_eq!(
        options
            .recall_provider_rollup
            .map(|rollup| rollup.recall_selection),
        Some(recall_selection)
    );
    assert_eq!(
        options
            .recall_selected_snippets
            .map(|selected| selected.envelope),
        Some(selected_snippets)
    );
}

#[test]
fn turn_context_manifest_selected_recall_controller_filters_and_applies_payload_light_inputs() {
    let extension_data = ExtensionData::new("turn-1");
    let recall_selection = TurnContextRecallSelectionSummary {
        returned_source_count: 3,
        selected_source_count: 2,
        ranked_source_count: 2,
        returned_unselected_source_count: 1,
        source_diversity_met: true,
        source_diversity_target: 2,
        max_per_source: 2,
        ranked_item_count: 3,
        omitted_by_budget_count: 1,
        memory_control_omitted_count: 1,
        low_trust_ranked_item_count: 1,
        low_recency_ranked_item_count: 0,
    };
    let selected_snippets = test_selected_snippet_envelope();
    extension_data.insert(recall_selection.clone());
    extension_data.insert(selected_snippets.clone());

    let decision =
        selected_recall::selected_recall_controller_decision_from_extension_data(&extension_data);

    assert_eq!(
        decision
            .recall_provider_rollup
            .as_ref()
            .map(|rollup| &rollup.recall_selection),
        Some(&recall_selection)
    );
    assert_eq!(
        decision
            .recall_selected_snippets
            .as_ref()
            .map(|selected| &selected.envelope),
        Some(&selected_snippets)
    );
    assert_eq!(
        decision.canary_readiness,
        selected_recall::SelectedRecallControllerCanaryReadiness {
            shadow_vs_live_required: true,
            token_saved_metric_required: true,
            latency_delta_metric_required: true,
            quality_delta_metric_required: true,
            rollback_readback_required: true,
            prompt_input_proof_required: true,
            response_debug_proof_payload_light: true,
            operator_approval_required: true,
            production_route_enabled: false,
            runtime_activation_enabled: false,
        }
    );
    assert!(decision.canary_readiness.has_payload_light_integrity());
    assert_eq!(
        decision.canary_metrics,
        selected_recall::SelectedRecallControllerCanaryMetrics {
            token_saved_min_basis_points:
                selected_recall::SELECTED_RECALL_CONTROLLER_TOKEN_SAVED_MIN_BASIS_POINTS,
            latency_delta_max_ms: selected_recall::SELECTED_RECALL_CONTROLLER_LATENCY_DELTA_MAX_MS,
            quality_delta_min_basis_points:
                selected_recall::SELECTED_RECALL_CONTROLLER_QUALITY_DELTA_MIN_BASIS_POINTS,
            rollback_readback_fixture_count:
                selected_recall::SELECTED_RECALL_CONTROLLER_ROLLBACK_READBACK_FIXTURE_COUNT,
            prompt_input_proof_covered: true,
            response_debug_proof_payload_light: true,
            production_route_enabled: false,
            runtime_activation_enabled: false,
        }
    );
    assert!(decision.canary_metrics.has_payload_light_integrity());
    assert_eq!(
        decision.readback_proofs,
        selected_recall::SelectedRecallControllerReadbackProofs {
            proofs: vec![
                selected_recall::SelectedRecallControllerReadbackProof {
                    surface: selected_recall::SelectedRecallControllerReadbackSurface::PromptInput,
                    label:
                        selected_recall::SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_MANIFEST_CONSUMED_PROOF,
                    covered: true,
                    payload_light: true,
                },
                selected_recall::SelectedRecallControllerReadbackProof {
                    surface: selected_recall::SelectedRecallControllerReadbackSurface::PromptInput,
                    label:
                        selected_recall::SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_SHADOW_METADATA_OMITTED_PROOF,
                    covered: true,
                    payload_light: true,
                },
                selected_recall::SelectedRecallControllerReadbackProof {
                    surface: selected_recall::SelectedRecallControllerReadbackSurface::PromptInput,
                    label:
                        selected_recall::SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_LIVE_SNIPPET_GUARDED_PROOF,
                    covered: true,
                    payload_light: true,
                },
                selected_recall::SelectedRecallControllerReadbackProof {
                    surface: selected_recall::SelectedRecallControllerReadbackSurface::ResponseDebug,
                    label:
                        selected_recall::SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_MANIFEST_SUMMARY_PROOF,
                    covered: true,
                    payload_light: true,
                },
                selected_recall::SelectedRecallControllerReadbackProof {
                    surface: selected_recall::SelectedRecallControllerReadbackSurface::ResponseDebug,
                    label:
                        selected_recall::SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_PAYLOAD_LIGHT_PROOF,
                    covered: true,
                    payload_light: true,
                },
                selected_recall::SelectedRecallControllerReadbackProof {
                    surface: selected_recall::SelectedRecallControllerReadbackSurface::Rollback,
                    label:
                        selected_recall::SELECTED_RECALL_CONTROLLER_ROLLBACK_FIXTURE_COVERED_PROOF,
                    covered: true,
                    payload_light: true,
                },
                selected_recall::SelectedRecallControllerReadbackProof {
                    surface: selected_recall::SelectedRecallControllerReadbackSurface::Rollback,
                    label:
                        selected_recall::SELECTED_RECALL_CONTROLLER_ROLLBACK_HASH_OMITTED_PROOF,
                    covered: true,
                    payload_light: true,
                },
            ],
            production_route_enabled: false,
            runtime_activation_enabled: false,
        }
    );
    assert!(decision.readback_proofs.has_prompt_input_readback_proofs());
    assert!(
        decision
            .readback_proofs
            .has_response_debug_readback_proofs()
    );
    assert!(decision.readback_proofs.has_rollback_readback_proofs());
    assert!(decision.readback_proofs.has_payload_light_integrity());
    assert_eq!(
        decision
            .readback_proofs
            .proofs
            .iter()
            .map(|proof| (proof.surface.as_str(), proof.label))
            .collect::<Vec<_>>(),
        vec![
            (
                "prompt-input",
                selected_recall::SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_MANIFEST_CONSUMED_PROOF,
            ),
            (
                "prompt-input",
                selected_recall::SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_SHADOW_METADATA_OMITTED_PROOF,
            ),
            (
                "prompt-input",
                selected_recall::SELECTED_RECALL_CONTROLLER_PROMPT_INPUT_LIVE_SNIPPET_GUARDED_PROOF,
            ),
            (
                "response-debug",
                selected_recall::SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_MANIFEST_SUMMARY_PROOF,
            ),
            (
                "response-debug",
                selected_recall::SELECTED_RECALL_CONTROLLER_RESPONSE_DEBUG_PAYLOAD_LIGHT_PROOF,
            ),
            (
                "rollback",
                selected_recall::SELECTED_RECALL_CONTROLLER_ROLLBACK_FIXTURE_COVERED_PROOF,
            ),
            (
                "rollback",
                selected_recall::SELECTED_RECALL_CONTROLLER_ROLLBACK_HASH_OMITTED_PROOF,
            ),
        ]
    );

    let context_items = vec![ResponseItem::Message {
        id: None,
        role: "developer".to_string(),
        content: vec![ContentItem::InputText {
            text: "secret policy body".to_string(),
        }],
        phase: None,
    }];
    let mut manifest = build_turn_context_manifest(&context_items).expect("manifest should build");
    let previous_ledger = manifest.ledger_hash.clone();

    selected_recall::apply_selected_recall_controller_decision(&mut manifest, &decision);
    let manifest_json = serde_json::to_string(&manifest).expect("manifest should serialize");

    assert!(manifest.has_replay_integrity());
    assert_eq!(manifest.recall_selection.as_ref(), Some(&recall_selection));
    assert_eq!(
        manifest.recall_selected_snippets.as_ref(),
        Some(&selected_snippets)
    );
    assert_ne!(manifest.ledger_hash, previous_ledger);
    assert!(!manifest_json.contains("secret policy body"));
    assert!(!manifest_json.contains("source-memory-id"));
    assert!(!manifest_json.contains("[hepta-memory:"));

    let unsafe_extension_data = ExtensionData::new("turn-unsafe");
    let mut unsafe_selected_snippets = test_selected_snippet_envelope();
    unsafe_selected_snippets.snippets[0].text = "source_id leaked into bounded snippet".into();
    unsafe_extension_data.insert(recall_selection);
    unsafe_extension_data.insert(unsafe_selected_snippets);

    let unsafe_decision = selected_recall::selected_recall_controller_decision_from_extension_data(
        &unsafe_extension_data,
    );

    assert!(unsafe_decision.recall_provider_rollup.is_some());
    assert!(unsafe_decision.recall_selected_snippets.is_none());
}

#[test]
fn turn_context_manifest_options_read_valid_memory_taxonomy_from_extension_data() {
    let extension_data = ExtensionData::new("turn-1");
    let memory_taxonomy = vec![
        TurnContextMemoryTaxonomyBucket {
            class: TurnContextMemoryTaxonomyClass::Semantic,
            source_count: 1,
            returned_count: 2,
            available_count: 3,
            omitted_count: 1,
            provenance_span_count: 0,
        },
        TurnContextMemoryTaxonomyBucket {
            class: TurnContextMemoryTaxonomyClass::Transcript,
            source_count: 1,
            returned_count: 2,
            available_count: 2,
            omitted_count: 0,
            provenance_span_count: 2,
        },
    ];
    extension_data.insert(memory_taxonomy.clone());

    let options = turn_context_manifest_options_from_extension_data(&extension_data);

    assert_eq!(options.memory_taxonomy, memory_taxonomy);
}

#[test]
fn turn_context_manifest_options_read_valid_memory_formation_receipts_from_extension_data() {
    let extension_data = ExtensionData::new("turn-1");
    let receipts = vec![
        TurnContextMemoryFormationReceipt {
            candidate_type: TurnContextMemoryFormationCandidateType::Fact,
            transcript_span_count: 2,
            provenance_span_count: 2,
            confidence_basis_points: 6400,
            idempotency_key_hash: "0123456789abcdef".into(),
            privacy_class: "user_private".into(),
            queued_for_background: true,
            production_write: false,
        },
        TurnContextMemoryFormationReceipt {
            candidate_type: TurnContextMemoryFormationCandidateType::Summary,
            transcript_span_count: 2,
            provenance_span_count: 1,
            confidence_basis_points: 7000,
            idempotency_key_hash: "fedcba9876543210".into(),
            privacy_class: "user_private".into(),
            queued_for_background: true,
            production_write: false,
        },
    ];
    extension_data.insert(receipts.clone());

    let options = turn_context_manifest_options_from_extension_data(&extension_data);

    assert_eq!(options.memory_formation_receipts, receipts);
}

#[test]
fn turn_context_manifest_options_read_valid_memory_temporal_facts_from_extension_data() {
    let extension_data = ExtensionData::new("turn-1");
    let temporal_facts = vec![
        TurnContextMemoryTemporalFact {
            fact_type: TurnContextMemoryTemporalFactType::Attribute,
            entity_hash: "0123456789abcdef".into(),
            provenance_span_count: 2,
            valid_from_sequence: 8,
            invalid_at_sequence: None,
            confidence_basis_points: 6200,
            supersedes_fact_hash: None,
            privacy_class: "user_private".into(),
            dry_run_only: true,
            production_write: false,
        },
        TurnContextMemoryTemporalFact {
            fact_type: TurnContextMemoryTemporalFactType::Summary,
            entity_hash: "fedcba9876543210".into(),
            provenance_span_count: 1,
            valid_from_sequence: 9,
            invalid_at_sequence: Some(12),
            confidence_basis_points: 7000,
            supersedes_fact_hash: Some("aaaaaaaaaaaaaaaa".into()),
            privacy_class: "user_private".into(),
            dry_run_only: true,
            production_write: false,
        },
    ];
    extension_data.insert(temporal_facts.clone());

    let options = turn_context_manifest_options_from_extension_data(&extension_data);

    assert_eq!(options.memory_temporal_facts, temporal_facts);
}

#[test]
fn turn_context_manifest_options_ignore_invalid_recall_rollup_from_extension_data() {
    let extension_data = ExtensionData::new("turn-1");
    let recall_selection = TurnContextRecallSelectionSummary {
        returned_source_count: 1,
        selected_source_count: 2,
        ranked_source_count: 2,
        returned_unselected_source_count: 0,
        source_diversity_met: true,
        source_diversity_target: 2,
        max_per_source: 2,
        ranked_item_count: 2,
        omitted_by_budget_count: 0,
        memory_control_omitted_count: 0,
        low_trust_ranked_item_count: 0,
        low_recency_ranked_item_count: 0,
    };
    extension_data.insert(recall_selection);

    let options = turn_context_manifest_options_from_extension_data(&extension_data);

    assert_eq!(options.recall_provider_rollup, None);
}

#[test]
fn turn_context_manifest_options_ignore_invalid_memory_taxonomy_from_extension_data() {
    let extension_data = ExtensionData::new("turn-1");
    extension_data.insert(vec![TurnContextMemoryTaxonomyBucket {
        class: TurnContextMemoryTaxonomyClass::Semantic,
        source_count: 1,
        returned_count: 2,
        available_count: 3,
        omitted_count: 0,
        provenance_span_count: 0,
    }]);

    let options = turn_context_manifest_options_from_extension_data(&extension_data);

    assert!(options.memory_taxonomy.is_empty());
}

#[test]
fn turn_context_manifest_options_ignore_invalid_memory_formation_receipts_from_extension_data() {
    let extension_data = ExtensionData::new("turn-1");
    extension_data.insert(vec![TurnContextMemoryFormationReceipt {
        candidate_type: TurnContextMemoryFormationCandidateType::Fact,
        transcript_span_count: 2,
        provenance_span_count: 2,
        confidence_basis_points: 6400,
        idempotency_key_hash: "0123456789abcdef".into(),
        privacy_class: "user_private".into(),
        queued_for_background: true,
        production_write: true,
    }]);

    let options = turn_context_manifest_options_from_extension_data(&extension_data);

    assert!(options.memory_formation_receipts.is_empty());
}

#[test]
fn turn_context_manifest_options_ignore_invalid_memory_temporal_facts_from_extension_data() {
    let extension_data = ExtensionData::new("turn-1");
    extension_data.insert(vec![TurnContextMemoryTemporalFact {
        fact_type: TurnContextMemoryTemporalFactType::Attribute,
        entity_hash: "0123456789abcdef".into(),
        provenance_span_count: 2,
        valid_from_sequence: 8,
        invalid_at_sequence: None,
        confidence_basis_points: 6200,
        supersedes_fact_hash: Some("raw-fact-id".into()),
        privacy_class: "user_private".into(),
        dry_run_only: true,
        production_write: false,
    }]);

    let options = turn_context_manifest_options_from_extension_data(&extension_data);

    assert!(options.memory_temporal_facts.is_empty());
}

#[test]
fn turn_context_manifest_options_ignore_invalid_selected_snippets_from_extension_data() {
    let extension_data = ExtensionData::new("turn-1");
    let mut selected_snippets = test_selected_snippet_envelope();
    selected_snippets.selected_snippet_count = 2;
    extension_data.insert(selected_snippets);

    let options = turn_context_manifest_options_from_extension_data(&extension_data);

    assert_eq!(options.recall_selected_snippets, None);
}

#[test]
fn turn_context_assembly_policy_requires_named_source_aware_compression_marker_injection_path() {
    let extension_data = ExtensionData::new("turn-1");
    let baseline_policy = ContextAssemblyPolicy::from_model_context_window(Some(1));

    assert_eq!(
        turn_context_assembly_policy_from_extension_data(
            &extension_data,
            Some(1),
            TurnContextAssemblyPolicyOptInGate::SourceAwareCompressionCanary,
        ),
        baseline_policy
    );

    insert_source_aware_compression_policy_opt_in_marker(&extension_data);

    assert_eq!(
        turn_context_assembly_policy_from_extension_data(
            &extension_data,
            Some(1),
            TurnContextAssemblyPolicyOptInGate::Disabled,
        ),
        baseline_policy
    );

    let canary_policy = turn_context_assembly_policy_from_extension_data(
        &extension_data,
        Some(1),
        TurnContextAssemblyPolicyOptInGate::SourceAwareCompressionCanary,
    );
    assert_eq!(
        canary_policy,
        ContextAssemblyPolicy::source_aware_compression_for_model_context_window(Some(1))
    );
    assert_eq!(canary_policy.strategy, "source_aware_compression");
    assert!(canary_policy.summarize_retrieved_snippets);
    assert!(canary_policy.defragment_tool_context);
    assert!(canary_policy.prune_tool_context);
}

fn test_selected_snippet_envelope() -> TurnContextRecallSelectedSnippetEnvelope {
    TurnContextRecallSelectedSnippetEnvelope {
        version: codex_protocol::protocol::TURN_CONTEXT_RECALL_SELECTED_SNIPPET_ENVELOPE_VERSION,
        max_snippets: 4,
        max_snippet_chars: 120,
        selected_snippet_count: 1,
        omitted_snippet_count: 2,
        redacted_snippet_count: 1,
        truncated_snippet_count: 0,
        snippets: vec![codex_protocol::protocol::TurnContextRecallSelectedSnippet {
            snippet_hash: "fedcba9876543210".into(),
            text: "[redacted-query] bounded memory".into(),
            estimated_tokens: 8,
            redacted: true,
            truncated: false,
        }],
        safety: codex_protocol::protocol::TurnContextRecallSelectedSnippetSafety {
            ready_for_shadow_handoff: true,
            bounded: true,
            origin_identifiers_exposed: false,
            raw_ranked_payload_exposed: false,
            rank_explanation_exposed: false,
            control_marker_exposed: false,
            query_payload_exposed: false,
            per_origin_list_exposed: false,
        },
    }
}
