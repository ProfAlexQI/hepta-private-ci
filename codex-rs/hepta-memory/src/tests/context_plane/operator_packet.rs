use super::*;

#[test]
fn store_snapshot_context_plane_operator_approval_packet_is_payload_light() {
    let snapshot = StoreSnapshot {
        sessions: vec![],
        memories: vec![
            memory_record("memory-1", MemoryScope::LongTerm, "timeout retry guidance"),
            memory_record("memory-2", MemoryScope::Session, "session timeout summary"),
        ],
        transcripts: vec![
            transcript_entry(
                "session-1",
                1,
                TranscriptEntryKind::Message,
                "timeout surfaced during tool run",
            ),
            transcript_entry(
                "session-1",
                2,
                TranscriptEntryKind::Summary,
                "timeout retried successfully",
            ),
        ],
    };
    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };

    let packet = snapshot.context_plane_operator_approval_packet(&request);

    assert!(packet.has_packet_integrity());
    assert!(packet.dry_run_only);
    assert!(packet.approval_required);
    assert!(!packet.activation_command_present);
    assert_eq!(packet.matrix_row_count, 12);
    assert_eq!(packet.threshold_satisfied_count, 9);
    assert_eq!(packet.blocker_count, 3);
    assert_eq!(packet.threshold_snapshot.required_ready_count, 11);
    assert_eq!(packet.threshold_snapshot.required_shadow_count, 1);
    assert_eq!(packet.required_scope_count(), 6);
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(
            ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled
        ),
        Some(1)
    );
    assert_eq!(
        packet.blocker_reason_count(ContextPlaneActivationBlockerReason::OperatorApprovalMissing),
        Some(1)
    );
    assert_eq!(packet.recall_quality_blocking_reason_count, 0);
    assert_eq!(packet.recall_quality_blocking_reason_count_total(), 0);

    let json = serde_json::to_string(&packet).expect("operator approval packet should serialize");
    assert!(json.contains("adaptive_budget_allocation_runtime"));
    assert!(json.contains("source_aware_runtime_activation"));
    assert!(json.contains("operator_activation"));
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reason_counts"));
    assert!(json.contains("adaptive_budget_allocation_shadow_only"));
    assert!(json.contains("source_aware_front_door_disabled"));
    assert!(json.contains("operator_approval_missing"));
    assert!(!json.contains("timeout surfaced during tool run"));
    assert!(!json.contains("timeout retried successfully"));
    assert!(!json.contains("session-1"));
    assert!(!json.contains("memory-1"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("\"activation_command_present\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[tokio::test]
async fn store_context_plane_operator_approval_packet_matches_snapshot_helper() {
    let store = InMemoryStore::default();
    store
        .put(memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "timeout retry guidance",
        ))
        .await
        .expect("put should succeed");
    store
        .put(memory_record(
            "memory-2",
            MemoryScope::Session,
            "session timeout summary",
        ))
        .await
        .expect("put should succeed");
    store
        .append(transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "timeout surfaced during tool run",
        ))
        .await
        .expect("append should succeed");
    store
        .append(transcript_entry(
            "session-1",
            2,
            TranscriptEntryKind::Summary,
            "timeout retried successfully",
        ))
        .await
        .expect("append should succeed");

    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };
    let snapshot = store.snapshot().expect("snapshot should load");
    let from_store = store
        .context_plane_operator_approval_packet(request.clone())
        .expect("context plane operator approval packet should succeed");

    assert_eq!(
        from_store,
        snapshot.context_plane_operator_approval_packet(&request)
    );
    assert!(from_store.has_packet_integrity());
    assert!(from_store.dry_run_only);
    assert!(from_store.approval_required);
    assert!(!from_store.activation_command_present);
    assert_eq!(from_store.matrix_row_count, 12);
    assert_eq!(from_store.threshold_satisfied_count, 9);
    assert_eq!(from_store.blocker_count, 3);
    assert_eq!(from_store.threshold_snapshot.required_ready_count, 11);
    assert_eq!(from_store.threshold_snapshot.required_shadow_count, 1);
    assert_eq!(from_store.required_scope_count(), 6);
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly
        ),
        Some(1)
    );
    assert_eq!(
        from_store.blocker_reason_count(
            ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled
        ),
        Some(1)
    );
    assert_eq!(
        from_store
            .blocker_reason_count(ContextPlaneActivationBlockerReason::OperatorApprovalMissing),
        Some(1)
    );
    assert_eq!(from_store.recall_quality_blocking_reason_count, 0);
    assert_eq!(from_store.recall_quality_blocking_reason_count_total(), 0);
    assert!(!from_store.production_write);
    assert!(!from_store.graph_write);
    assert!(!from_store.runtime_activation);
    assert!(!from_store.adaptive_allocator_runtime_activation);
    assert!(!from_store.source_aware_runtime_activation);
    assert!(!from_store.prompt_assembly_change);
    assert!(!from_store.operator_activation_allowed);
}

#[tokio::test]
async fn store_context_plane_operator_approval_packet_rejects_activation_shaped_rehydration() {
    let store = InMemoryStore::default();
    store
        .put(memory_record(
            "memory-1",
            MemoryScope::LongTerm,
            "timeout retry guidance",
        ))
        .await
        .expect("put should succeed");
    store
        .append(transcript_entry(
            "session-1",
            1,
            TranscriptEntryKind::Message,
            "timeout surfaced during tool run",
        ))
        .await
        .expect("append should succeed");

    let request = ContextRecallRequest {
        session_id: SessionId("session-1".into()),
        query_text: Some("timeout".into()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };
    let packet = store
        .context_plane_operator_approval_packet(request)
        .expect("context plane operator approval packet should succeed");
    assert!(packet.has_packet_integrity());

    let mut command_shaped =
        serde_json::to_value(&packet).expect("packet should convert to json value");
    command_shaped
        .as_object_mut()
        .expect("packet json should be an object")
        .insert(
            "activation_command".into(),
            serde_json::Value::String("hepta context activate --now".into()),
        );
    assert!(serde_json::from_value::<ContextPlaneOperatorApprovalPacket>(command_shaped).is_err());

    let mut payload_shaped =
        serde_json::to_value(&packet).expect("packet should convert to json value");
    payload_shaped["threshold_snapshot"]
        .as_object_mut()
        .expect("threshold snapshot should be an object")
        .insert("tool_args".into(), serde_json::json!({"cmd": "activate"}));
    assert!(serde_json::from_value::<ContextPlaneOperatorApprovalPacket>(payload_shaped).is_err());

    let mut enabled_flag =
        serde_json::to_value(&packet).expect("packet should convert to json value");
    enabled_flag
        .as_object_mut()
        .expect("packet json should be an object")
        .insert("runtime_activation".into(), serde_json::Value::Bool(true));
    let parsed: ContextPlaneOperatorApprovalPacket =
        serde_json::from_value(enabled_flag).expect("known side-effect flag should deserialize");
    assert!(!parsed.has_packet_integrity());
}
