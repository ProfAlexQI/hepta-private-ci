#[cfg(test)]
mod edit_history_detail_surface_tests {
    use super::*;

    #[test]
    fn edit_history_detail_summary_is_read_only_and_event_scoped() {
        let event_id = EventId::parse("$edited:example.org").unwrap();

        let label = edit_history_detail_summary_label(
            &event_id,
            2,
            3,
            true,
            "$replacement:example.org",
            None,
        );

        assert!(label.contains("$edited:example.org"));
        assert!(label.contains("2 replacement event(s)"));
        assert!(label.contains("complete m.replace pagination exhausted after 3 relation page"));
        assert!(label.contains("$replacement:example.org"));
        assert!(label.contains("latest timestamp unavailable"));
        assert!(label.contains("local read-only"));
    }

    #[test]
    fn edit_history_detail_diff_and_metadata_keep_remaining_gaps_explicit() {
        let diff = edit_history_detail_diff_label("Original body", "Edited body");
        let metadata =
            edit_history_detail_metadata_label(1, 2, true, "Original body", "Edited body");

        assert!(diff.contains("differs"));
        assert!(diff.contains("Loaded original: Original body"));
        assert!(diff.contains("Latest replacement: Edited body"));
        assert!(metadata.contains("1 m.replace relation"));
        assert!(metadata.contains("complete m.replace pagination exhausted after 2 relation page"));
        assert!(metadata.contains("loaded original preview 13 chars"));
        assert!(metadata.contains("latest replacement preview 11 chars"));
        assert!(metadata.contains("No full modal"));
        assert!(metadata.contains("event-context fetch"));
        assert!(metadata.contains("timeline reload"));
        assert!(metadata.contains("gateway/runtime/auth"));
        assert!(metadata.contains("live mutation"));
    }

    #[test]
    fn edit_history_local_full_snapshot_renders_loaded_compact_state() {
        let label = edit_history_local_full_snapshot_label(
            "$edited:example.org",
            Some(2),
            "$replacement:example.org",
            None,
            "Original body",
            "Edited body",
            "",
            false,
        );

        assert!(label.contains("Local full snapshot"));
        assert!(label.contains("target event $edited:example.org"));
        assert!(label.contains("2 compact replacement relation"));
        assert!(label.contains("latest replacement event $replacement:example.org"));
        assert!(label.contains("latest timestamp unavailable"));
        assert!(label.contains("original preview 13 chars/13 bytes: Original body"));
        assert!(label.contains("latest preview 11 chars/11 bytes: Edited body"));
        assert!(label.contains("local delta differs"));
        assert!(label.contains("error cache empty"));
        assert!(label.contains("retry cache not used"));
        assert!(label.contains("Full opens this loaded local snapshot"));
        assert!(label.contains("Remote full history modal UI"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("timeline pagination/reload"));
        assert!(label.contains("replacement event source fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_EVIDENCE
                .contains("synthetic full snapshot JSON")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE
                .contains("Full opens the existing local EventSourceModal")
        );
    }

    #[test]
    fn edit_history_local_full_snapshot_modal_json_is_cached_read_only() {
        let json = edit_history_local_full_snapshot_modal_json(
            "$edited:example.org",
            Some(2),
            Some(3),
            true,
            "$replacement:example.org",
            None,
            "Original body",
            "Edited body",
            "{\"type\":\"m.room.message\"}",
            "",
            true,
            true,
        );

        assert!(json.contains("\"kind\": \"hepta.telegram.edit_history.local_full_snapshot\""));
        assert!(json.contains("\"source\": \"cached MatrixRequest::FetchEditHistory"));
        assert!(json.contains("\"target_event_id\": \"$edited:example.org\""));
        assert!(json.contains("\"replacement_count\": 2"));
        assert!(json.contains("\"relation_pages_fetched\": 3"));
        assert!(json.contains("\"pagination_exhausted\": true"));
        assert!(json.contains("\"latest_replacement_event_id\": \"$replacement:example.org\""));
        assert!(json.contains("\"loaded_original_preview\": \"Original body\""));
        assert!(json.contains("\"latest_replacement_preview\": \"Edited body\""));
        assert!(json.contains("\"latest_replacement_raw_json_available\": true"));
        assert!(json.contains("\"retry_cache_ready\": true"));
        assert!(json.contains("\"remote_full_history_modal_request\": false"));
        assert!(json.contains("\"event_context_fetch\": false"));
        assert!(json.contains("\"gateway_runtime_auth\": false"));
        assert!(
            MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_EVIDENCE.contains("EventSourceModal")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_EVIDENCE
                .contains("no extra MatrixRequest::FetchEditHistory")
        );
    }

    #[test]
    fn edit_history_local_full_snapshot_modal_label_reports_open_state() {
        let label = edit_history_local_full_snapshot_modal_label(
            "$edited:example.org",
            true,
            512,
            Some(2),
            Some(3),
            true,
            true,
            true,
        );

        assert!(label.contains("opened local full snapshot EventSourceModal"));
        assert!(label.contains("target event $edited:example.org"));
        assert!(label.contains("2 compact replacement relation"));
        assert!(label.contains("complete m.replace pagination exhausted after 3 relation page"));
        assert!(label.contains("loaded source available"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("snapshot JSON 512 bytes"));
        assert!(label.contains(MESSAGE_EDIT_HISTORY_LOCAL_FULL_SNAPSHOT_MODAL_LABEL));
        assert!(label.contains("No extra MatrixRequest::FetchEditHistory"));
        assert!(label.contains("no remote full-history modal request"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn edit_history_full_modal_boundary_label_keeps_remaining_gap_visible() {
        let label = edit_history_full_modal_boundary_label(
            "loaded compact m.replace summary",
            Some(3),
            Some(2),
            true,
            false,
        );

        assert!(label.contains("Full history boundary"));
        assert!(label.contains("loaded compact m.replace summary"));
        assert!(label.contains("3 compact replacement relation"));
        assert!(label.contains("complete m.replace pagination exhausted after 2 relation page"));
        assert!(label.contains("retry cache not used"));
        assert!(label.contains(MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_LABEL));
        assert!(label.contains("No remote full-history modal request"));
        assert!(label.contains("side-by-side full diff"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("timeline pagination/reload"));
        assert!(label.contains("replacement event source fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_EVIDENCE.contains("local blocked controls")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_FULL_MODAL_BOUNDARY_EVIDENCE
                .contains("MatrixRequest::FetchEditHistory")
        );
    }

    #[test]
    fn edit_history_full_modal_boundary_label_reports_retry_state() {
        let label =
            edit_history_full_modal_boundary_label("retry confirmation", None, None, false, true);

        assert!(label.contains("retry confirmation"));
        assert!(label.contains("replacement count waiting"));
        assert!(label.contains("m.replace pagination waiting"));
        assert!(label.contains("retry cache ready"));
    }

    #[test]
    fn edit_history_full_control_boundary_label_is_local_only() {
        let label = edit_history_full_control_boundary_label("Event context");

        assert!(label.contains("Event context is local only"));
        assert!(label.contains(MESSAGE_EDIT_HISTORY_FULL_CONTROLS_LABEL));
        assert!(label.contains("No remote full-history modal request"));
        assert!(label.contains("side-by-side full diff rendering"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("timeline pagination/reload"));
        assert!(label.contains("replacement event source fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE.contains("Full, Diff, Context"));
        assert!(
            MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE
                .contains("real loaded side-by-side preview diff handoff")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE
                .contains("real loaded edit-source modal handoff")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE.contains("MatrixRequest::FetchEditHistory")
        );
    }

    #[test]
    fn edit_history_loaded_source_modal_label_summarizes_loaded_original_source() {
        let label = edit_history_loaded_source_modal_label(
            "latest replacement",
            "$replacement:example.org",
            true,
            Some("{\n  \"type\": \"m.room.message\"\n}"),
            "Original body",
            "Edited body",
            Some(2),
            "$edited:example.org",
        );

        assert!(label.contains("opened latest replacement EventSourceModal"));
        assert!(label.contains("latest replacement event $replacement:example.org"));
        assert!(label.contains("source JSON 30 chars"));
        assert!(label.contains("2 replacement relation"));
        assert!(label.contains("$edited:example.org"));
        assert!(label.contains("original 13 chars"));
        assert!(label.contains("latest 11 chars"));
        assert!(label.contains("cached raw JSON returned by FetchEditHistory"));
        assert!(label.contains(MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_LABEL));
        assert!(label.contains("can request Matrix room.event/load_or_fetch_event"));
        assert!(label.contains("latest replacement source"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("timeline pagination/reload"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE.contains("EventSourceModal"));
        assert!(
            MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE.contains("latest replacement source")
        );
        assert!(MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE.contains("raw JSON"));
        assert!(MESSAGE_EDIT_HISTORY_LOADED_SOURCE_MODAL_EVIDENCE.contains("Missing event id"));
    }

    #[test]
    fn edit_history_loaded_source_modal_label_summarizes_original_fallback() {
        let label = edit_history_loaded_source_modal_label(
            "loaded original",
            "$edited:example.org",
            true,
            Some("{\n  \"type\": \"m.room.message\"\n}"),
            "Original body",
            "Edited body",
            Some(2),
            "$replacement:example.org",
        );

        assert!(label.contains("opened loaded original EventSourceModal"));
        assert!(label.contains("loaded original event $edited:example.org"));
        assert!(label.contains("source JSON 30 chars"));
    }

    #[test]
    fn edit_history_loaded_source_modal_label_reports_missing_loaded_source() {
        let label = edit_history_loaded_source_modal_label("", "", false, None, "", "", None, "");

        assert!(label.contains("loaded edit source unavailable"));
        assert!(label.contains("target event waiting"));
        assert!(label.contains("source JSON unavailable"));
        assert!(label.contains("replacement count waiting"));
        assert!(label.contains("latest replacement event waiting"));
    }

    #[test]
    fn edit_history_loaded_diff_detail_label_summarizes_loaded_state() {
        let label = edit_history_loaded_diff_detail_label(
            Some("Loaded side-by-side preview diff"),
            "$edited:example.org",
            Some(2),
            "$replacement:example.org",
            None,
            "Original body",
            "Edited body",
            false,
        );

        assert!(label.contains("Loaded side-by-side preview diff detail selected"));
        assert!(label.contains("target event $edited:example.org"));
        assert!(label.contains("2 replacement relation"));
        assert!(label.contains("latest replacement event $replacement:example.org"));
        assert!(label.contains("latest timestamp unavailable"));
        assert!(label.contains("original 13 chars, latest 11 chars"));
        assert!(label.contains("local delta differs"));
        assert!(label.contains("retry cache not used"));
        assert!(label.contains(MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_LABEL));
        assert!(label.contains("No remote full-history modal request"));
        assert!(label.contains("side-by-side full diff rendering"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("timeline pagination/reload"));
        assert!(label.contains("replacement event source fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_EVIDENCE
                .contains("MatrixRequest::FetchEditHistory")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_EVIDENCE.contains("Full/Diff/Context/Source")
        );
    }

    #[test]
    fn edit_history_loaded_side_by_side_diff_modal_json_is_cached_read_only() {
        let json = edit_history_loaded_side_by_side_diff_modal_json(
            "$edited:example.org",
            Some(2),
            Some(3),
            true,
            "$replacement:example.org",
            None,
            "Original body\nsame",
            "Edited body\nsame",
        )
        .unwrap();

        assert!(json.contains(
            "\"kind\": \"hepta.telegram.edit_history.loaded_side_by_side_preview_full_body_diff\""
        ));
        assert!(json.contains("\"target_event_id\": \"$edited:example.org\""));
        assert!(json.contains("\"replacement_count\": 2"));
        assert!(json.contains("\"relation_pages_fetched\": 3"));
        assert!(json.contains("\"pagination_exhausted\": true"));
        assert!(json.contains("\"latest_replacement_event_id\": \"$replacement:example.org\""));
        assert!(json.contains("\"rendering_scope\": \"loaded preview rows only"));
        assert!(json.contains("\"loaded_full_body_side_by_side_snapshot\": false"));
        assert!(json.contains("\"line\": 1"));
        assert!(json.contains("\"original\": \"Original body\""));
        assert!(json.contains("\"latest\": \"Edited body\""));
        assert!(json.contains("\"changed\": true"));
        assert!(json.contains("\"line\": 2"));
        assert!(json.contains("\"changed\": false"));
        assert!(json.contains("\"loaded_preview_side_by_side_snapshot\": true"));
        assert!(json.contains("\"loaded_full_body_side_by_side_snapshot\": false"));
        assert!(json.contains("\"server_backed_full_body_side_by_side_diff_rendering\": false"));
        assert!(json.contains("\"event_context_fetch\": false"));
        assert!(json.contains("\"write_side_live_mutation\": false"));
        assert!(
            MESSAGE_EDIT_HISTORY_LOADED_SIDE_BY_SIDE_DIFF_MODAL_EVIDENCE
                .contains("EventSourceModal")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_LOADED_SIDE_BY_SIDE_DIFF_MODAL_EVIDENCE
                .contains("no extra MatrixRequest::FetchEditHistory")
        );
    }

    #[test]
    fn edit_history_loaded_side_by_side_diff_modal_json_uses_cached_source_full_body() {
        let original_source_json = r#"{
  "type": "m.room.message",
  "content": {
    "msgtype": "m.text",
    "body": "Original full body\nsecond original line"
  }
}"#;
        let latest_source_json = r#"{
  "type": "m.room.message",
  "content": {
    "msgtype": "m.text",
    "body": "* Edited preview fallback",
    "m.new_content": {
      "msgtype": "m.text",
      "body": "Edited full body\nsecond edited line"
    }
  }
}"#;
        let snapshot = edit_history_loaded_side_by_side_diff_modal_json_with_sources(
            "$edited:example.org",
            Some(1),
            Some(1),
            true,
            "$replacement:example.org",
            None,
            "Original preview",
            "Edited preview",
            Some(original_source_json),
            latest_source_json,
        )
        .unwrap();

        assert!(snapshot.loaded_full_body);
        assert!(
            snapshot
                .json
                .contains("\"loaded full body rows from cached source JSON")
        );
        assert!(
            snapshot
                .json
                .contains("\"original_body_source\": \"loaded original latest_json body\"")
        );
        assert!(
            snapshot
                .json
                .contains("\"latest_body_source\": \"cached latest replacement raw JSON body\"")
        );
        assert!(
            snapshot
                .json
                .contains("\"original\": \"Original full body\"")
        );
        assert!(snapshot.json.contains("\"latest\": \"Edited full body\""));
        assert!(
            snapshot
                .json
                .contains("\"loaded_full_body_side_by_side_snapshot\": true")
        );
        assert!(
            snapshot
                .json
                .contains("\"server_backed_full_body_side_by_side_diff_rendering\": false")
        );
    }

    #[test]
    fn edit_history_loaded_side_by_side_diff_modal_json_requires_loaded_preview() {
        assert!(
            edit_history_loaded_side_by_side_diff_modal_json(
                "", None, None, false, "", None, "", "",
            )
            .is_none()
        );
        assert!(
            edit_history_loaded_side_by_side_diff_modal_json(
                "$edited:example.org",
                None,
                None,
                false,
                "",
                None,
                "",
                "",
            )
            .is_none()
        );
    }

    #[test]
    fn edit_history_loaded_diff_detail_label_reports_waiting_and_retry_state() {
        let label = edit_history_loaded_diff_detail_label(None, "", None, "", None, "", "", true);

        assert!(label.contains("No Full/Diff/Context/Source/Packet/Contract detail selected"));
        assert!(label.contains("target event waiting"));
        assert!(label.contains("replacement count waiting"));
        assert!(label.contains("latest replacement event waiting"));
        assert!(label.contains("preview chars waiting"));
        assert!(label.contains("delta waiting"));
        assert!(label.contains("retry cache ready"));
    }

    #[test]
    fn edit_history_loaded_diff_clipboard_payload_is_loaded_local_only() {
        let payload = edit_history_loaded_diff_clipboard_payload(
            "$edited:example.org",
            Some(2),
            "$replacement:example.org",
            None,
            "Original body",
            "Edited body",
        )
        .unwrap();

        assert!(payload.contains("Edit history compact diff"));
        assert!(payload.contains("Target: $edited:example.org"));
        assert!(payload.contains("2 compact replacement relation"));
        assert!(payload.contains("latest replacement event $replacement:example.org"));
        assert!(payload.contains("Original preview (13 chars, 13 bytes): Original body"));
        assert!(payload.contains("Latest preview (11 chars, 11 bytes): Edited body"));
        assert!(payload.contains(MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_LABEL));
        assert!(payload.contains("No remote full-history modal request"));
        assert!(payload.contains("side-by-side full diff rendering"));
        assert!(payload.contains("event-context fetch"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
        assert!(MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_EVIDENCE.contains("local clipboard"));
        assert!(
            MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_EVIDENCE
                .contains("real loaded side-by-side preview diff modal")
        );
    }

    #[test]
    fn edit_history_loaded_diff_clipboard_payload_requires_loaded_preview() {
        assert!(edit_history_loaded_diff_clipboard_payload("", None, "", None, "", "").is_none());
        assert!(
            edit_history_loaded_diff_clipboard_payload(
                "$edited:example.org",
                None,
                "",
                None,
                "",
                "",
            )
            .is_none()
        );
    }

    #[test]
    fn edit_history_loaded_diff_clipboard_label_reports_copied_and_unavailable_states() {
        let copied = edit_history_loaded_diff_clipboard_label(
            "$edited:example.org",
            true,
            true,
            Some(512),
            true,
            Some(1),
            "$replacement:example.org",
            "Original body",
            "Edited body",
        );
        let unavailable = edit_history_loaded_diff_clipboard_label(
            "", false, false, None, false, None, "", "", "",
        );

        assert!(copied.contains("copied loaded compact diff to local clipboard"));
        assert!(copied.contains("opened loaded full-body side-by-side diff modal 512 bytes"));
        assert!(copied.contains("target event $edited:example.org"));
        assert!(copied.contains("1 replacement relation"));
        assert!(copied.contains("$replacement:example.org"));
        assert!(copied.contains("original 13 chars/13 bytes, latest 11 chars/11 bytes"));
        assert!(copied.contains(MESSAGE_EDIT_HISTORY_LOADED_DIFF_CLIPBOARD_LABEL));
        assert!(copied.contains(MESSAGE_EDIT_HISTORY_LOADED_SIDE_BY_SIDE_DIFF_MODAL_LABEL));
        assert!(copied.contains("No remote full-history modal request"));
        assert!(copied.contains("event-context fetch"));
        assert!(unavailable.contains("diff clipboard unavailable"));
        assert!(unavailable.contains("side-by-side preview diff modal unavailable"));
        assert!(unavailable.contains("target event waiting"));
        assert!(unavailable.contains("preview data waiting"));
    }

    #[test]
    fn edit_history_full_diff_packet_payload_lists_remote_modal_acceptance() {
        let payload = edit_history_full_diff_packet_payload(
            "$edited:example.org",
            Some(2),
            "$replacement:example.org",
            None,
            "Original body",
            "Edited body",
            "",
            true,
            true,
            "Loaded diff detail cached",
            "Preflight detail cached",
            "Full boundary cached",
        );

        assert!(payload.contains("Edit history loaded/full diff packet"));
        assert!(payload.contains("Target: target event $edited:example.org"));
        assert!(payload.contains("2 compact replacement relation"));
        assert!(payload.contains("$replacement:example.org"));
        assert!(payload.contains("latest timestamp unavailable"));
        assert!(payload.contains("original 13 chars/13 bytes"));
        assert!(payload.contains("latest 11 chars/11 bytes"));
        assert!(payload.contains("Retry: retry cache ready"));
        assert!(payload.contains("Loaded source: loaded original source available"));
        assert!(payload.contains("remote full-history modal request/result/error"));
        assert!(payload.contains("complete replacement pagination is live"));
        assert!(payload.contains("side-by-side full diff rendering"));
        assert!(payload.contains("event context"));
        assert!(payload.contains("replacement event source"));
        assert!(payload.contains("loaded original source"));
        assert!(payload.contains("PositiveConfirmationModal"));
        assert!(payload.contains("no extra MatrixRequest::FetchEditHistory"));
        assert!(payload.contains("gateway/runtime/auth/provider"));
        assert!(payload.contains("Telegram delivery"));
        assert!(payload.contains("live mutation"));
        assert!(
            MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_EVIDENCE
                .contains("loaded/full diff remote modal acceptance contract")
        );
    }

    #[test]
    fn edit_history_full_diff_packet_label_reports_copy_state() {
        let label =
            edit_history_full_diff_packet_label("$edited:example.org", Some(2), true, false);

        assert!(label.contains("Edit-history Packet copied"));
        assert!(label.contains("target event $edited:example.org"));
        assert!(label.contains("2 replacement relation"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("loaded source unavailable"));
        assert!(label.contains(MESSAGE_EDIT_HISTORY_FULL_DIFF_PACKET_LABEL));
        assert!(label.contains("No extra FetchEditHistory"));
        assert!(label.contains("full modal request"));
        assert!(label.contains("side-by-side full diff rendering"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn edit_history_full_history_result_contract_packet_payload_lists_typed_contracts() {
        let payload = edit_history_full_history_result_contract_packet_payload(
            "$edited:example.org",
            Some(2),
            "$replacement:example.org",
            None,
            "Original body",
            "Edited body",
            "",
            true,
            true,
            "Loaded diff detail cached",
            "Preflight detail cached",
            "Full boundary cached",
        );

        assert!(payload.contains("Edit history typed full-history modal/result contract packet"));
        assert!(payload.contains("Target: target event $edited:example.org"));
        assert!(payload.contains("2 compact replacement relation"));
        assert!(payload.contains("$replacement:example.org"));
        assert!(payload.contains("full-history modal request slots"));
        assert!(payload.contains("replacement count cursor"));
        assert!(payload.contains("full-history modal result slots"));
        assert!(payload.contains("side-by-side diff slots"));
        assert!(payload.contains("event context slots"));
        assert!(payload.contains("replacement source slots"));
        assert!(payload.contains("source hash"));
        assert!(payload.contains("PositiveConfirmationModal"));
        assert!(payload.contains(MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_LABEL));
        assert!(payload.contains("no extra MatrixRequest::FetchEditHistory"));
        assert!(payload.contains("gateway/runtime/auth/provider"));
        assert!(payload.contains("Telegram delivery"));
        assert!(payload.contains("live mutation"));
        assert!(
            MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_EVIDENCE
                .contains("typed full-history modal/result contract packet")
        );
    }

    #[test]
    fn edit_history_full_history_result_contract_packet_label_reports_copy_state() {
        let label = edit_history_full_history_result_contract_packet_label(
            "$edited:example.org",
            Some(2),
            true,
            false,
        );

        assert!(label.contains("Edit-history Contract copied"));
        assert!(label.contains("target event $edited:example.org"));
        assert!(label.contains("2 replacement relation"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("loaded source unavailable"));
        assert!(label.contains(MESSAGE_EDIT_HISTORY_FULL_HISTORY_RESULT_CONTRACT_PACKET_LABEL));
        assert!(label.contains("No extra FetchEditHistory"));
        assert!(label.contains("full modal request"));
        assert!(label.contains("side-by-side full diff rendering"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn edit_history_remote_result_taxonomy_packet_payload_lists_blocked_slots() {
        let payload = edit_history_remote_result_taxonomy_packet_payload(
            "$edited:example.org",
            Some(2),
            Some(3),
            true,
            "$replacement:example.org",
            None,
            "Original body",
            "Edited body",
            "",
            true,
            true,
            "Loaded diff detail cached",
            "Preflight detail cached",
            "Full boundary cached",
        );

        assert!(payload.contains("Edit history remote full-history/source result taxonomy packet"));
        assert!(payload.contains("Target: target event $edited:example.org"));
        assert!(payload.contains("3 relation page"));
        assert!(
            payload.contains("Live result references: paginated MatrixRequest::FetchEditHistory")
        );
        assert!(payload.contains("Room::relations next_batch exhaustion"));
        assert!(payload.contains("source-only MatrixRequest::FetchEventSource"));
        assert!(payload.contains("Room::load_or_fetch_event"));
        assert!(payload.contains("Blocked remote_full_history_request_id: not_assigned"));
        assert!(payload.contains("Blocked full_history_cursor_id: not_assigned"));
        assert!(payload.contains("Blocked server_backed_full_diff_operation_id: not_assigned"));
        assert!(payload.contains("Blocked replacement_source_reconciliation_operation_id"));
        assert!(payload.contains("Blocked event_context_operation_id: not_assigned"));
        assert!(payload.contains("Blocked stale_target_result"));
        assert!(payload.contains("Source-hash policy"));
        assert!(payload.contains("Audit redaction"));
        assert!(payload.contains(MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(payload.contains("No extra MatrixRequest::FetchEditHistory"));
        assert!(payload.contains("server-backed side-by-side full diff rendering"));
        assert!(payload.contains("event-context fetch"));
        assert!(payload.contains("Telegram delivery"));
        assert!(payload.contains("live mutation"));
    }

    #[test]
    fn edit_history_remote_result_taxonomy_packet_label_reports_copy_state() {
        let label = edit_history_remote_result_taxonomy_packet_label(
            "$edited:example.org",
            Some(2),
            true,
            false,
        );

        assert!(label.contains("Edit-history Taxonomy copied"));
        assert!(label.contains("target event $edited:example.org"));
        assert!(label.contains("2 replacement relation"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("loaded source unavailable"));
        assert!(label.contains(MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(label.contains("No extra FetchEditHistory"));
        assert!(label.contains("server-backed full diff rendering"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn edit_history_remote_result_taxonomy_evidence_names_live_boundary() {
        assert!(
            MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("remote full-history/source reconciliation result taxonomy")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("MatrixRequest::FetchEditHistory")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("MatrixRequest::FetchEventSource")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("source-hash policy not-assigned/not-wired")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("no remote full-history modal request")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_REMOTE_RESULT_TAXONOMY_PACKET_EVIDENCE
                .contains("gateway/runtime/auth/provider")
        );
        assert!(MESSAGE_EDIT_HISTORY_FULL_CONTROLS_EVIDENCE.contains("Taxonomy"));
        assert!(
            MESSAGE_EDIT_HISTORY_LOADED_DIFF_DETAIL_EVIDENCE
                .contains("Full/Diff/Context/Source/Packet/Contract/Taxonomy")
        );
    }

    #[test]
    fn edit_history_preflight_detail_label_summarizes_cached_result() {
        let label = edit_history_preflight_detail_label(
            Some("Result"),
            "$edited:example.org",
            Some(2),
            "$replacement:example.org",
            None,
            "Original body",
            "Edited body",
            "",
            false,
            "Result metadata",
            "Full boundary",
        );

        assert!(label.contains("Result selected"));
        assert!(label.contains("controls Request, Result, Error, Retry, Source"));
        assert!(label.contains("result cached from compact m.replace summary"));
        assert!(label.contains("target event $edited:example.org"));
        assert!(label.contains("2 replacement relation"));
        assert!(label.contains("latest replacement event $replacement:example.org"));
        assert!(label.contains("original 13 chars, latest 11 chars"));
        assert!(label.contains("error cache empty"));
        assert!(label.contains("retry cache not ready"));
        assert!(label.contains("source metadata"));
        assert!(label.contains(MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(label.contains("No extra MatrixRequest::FetchEditHistory"));
        assert!(label.contains("no retry without PositiveConfirmationModal"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("replacement source fetch"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn edit_history_preflight_detail_label_reports_waiting_error_and_retry() {
        let label = edit_history_preflight_detail_label(
            Some("Error"),
            "$edited:example.org",
            None,
            "",
            None,
            "",
            "",
            "not found",
            true,
            "",
            "",
        );

        assert!(label.contains("Error selected"));
        assert!(label.contains("failed compact read cached"));
        assert!(label.contains("replacement result waiting"));
        assert!(label.contains("latest replacement event waiting"));
        assert!(label.contains("preview counts waiting"));
        assert!(label.contains("error cache 9 chars"));
        assert!(label.contains("retry cache ready"));
        assert!(label.contains("source metadata waiting"));
    }

    #[test]
    fn edit_history_preflight_detail_evidence_names_local_boundaries() {
        assert!(
            MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("Request, Result, Error, Retry, and Source")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("MatrixRequest::FetchEditHistory")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("PositiveConfirmationModal")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE.contains("event-context fetch")
        );
        assert!(
            MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("gateway/runtime/auth")
        );
        assert!(MESSAGE_EDIT_HISTORY_PREFLIGHT_DETAIL_CONTROLS_LABEL.contains("stay local"));
    }

    #[test]
    fn edit_history_retry_confirmation_is_confirmed_and_narrow() {
        let event_id = EventId::parse("$edited:example.org").unwrap();

        let label = edit_history_retry_confirmation_label(&event_id);

        assert!(label.contains("Retry compact edit history read"));
        assert!(label.contains("$edited:example.org"));
        assert!(label.contains("Retry confirms before FetchEditHistory"));
        assert!(label.contains("full-history modal"));
        assert!(label.contains("event-context fetch"));
        assert!(label.contains("timeline pagination/reload"));
        assert!(label.contains("event source open"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }
}

fn room_settings_live_write_validation_label(
    field: RoomSettingsMutationField,
    timeline_ready: bool,
) -> String {
    if timeline_ready {
        format!(
            "Room {} update blocked locally: enter a non-empty {} before submitting {}. {} No canonical alias, power-level, membership, gateway/runtime/auth, or Telegram delivery mutation was requested.",
            field.label(),
            field.matrix_event_type(),
            field.matrix_event_type(),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        )
    } else {
        format!(
            "Room {} update blocked locally: no loaded timeline is available for {}. {} No room-state write was requested.",
            field.label(),
            field.matrix_event_type(),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        )
    }
}

fn room_settings_live_write_avatar_validation_label(room_label: &str) -> String {
    format!(
        "Room Avatar removal blocked locally for {room_label}: loaded room-list avatar identity is missing, so no m.room.avatar removal was requested. {} No canonical alias, power-level, membership, gateway/runtime/auth, or Telegram delivery mutation was requested.",
        ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
    )
}

fn room_settings_tombstone_replacement_validation_label(
    room_label: &str,
    replacement_room_id: &str,
    error: &str,
) -> String {
    format!(
        "Room Tombstone update blocked locally for {room_label}: replacement room `{}` is not a valid Matrix room id: {}. {} No m.room.tombstone, power-level, membership, gateway/runtime/auth, or Telegram delivery mutation was requested.",
        compact_message_preview(replacement_room_id, "empty"),
        compact_message_preview(error, "unknown error"),
        ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
    )
}

fn room_settings_tombstone_body(room_label: &str, replacement_room_id: &str) -> String {
    let room_label = if room_label.trim().is_empty() {
        "This room"
    } else {
        room_label.trim()
    };
    format!("{room_label} has been replaced by {replacement_room_id}.")
}

fn room_settings_live_write_confirmation_label(
    room_label: &str,
    field: RoomSettingsMutationField,
    value: &str,
) -> String {
    if field == RoomSettingsMutationField::Avatar {
        format!(
            "Remove room avatar for {room_label}? This will send {} removal through the Matrix SDK after confirmation. {} Power-level, membership, gateway/runtime/auth, and Telegram delivery mutations remain blocked.",
            field.matrix_event_type(),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        )
    } else if field == RoomSettingsMutationField::CanonicalAlias {
        format!(
            "Save canonical alias for {room_label} as `{}`? This will send {} through the Matrix SDK after confirmation while preserving loaded alternative aliases. {} Power-level, membership, gateway/runtime/auth, and Telegram delivery mutations remain blocked.",
            compact_message_preview(value, "empty"),
            field.matrix_event_type(),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        )
    } else if field == RoomSettingsMutationField::Tombstone {
        format!(
            "Mark {room_label} as replaced by `{}`? This will send {} through the Matrix SDK after confirmation. {} Power-level, member moderation, gateway/runtime/auth, and Telegram delivery mutations remain blocked.",
            compact_message_preview(value, "empty"),
            field.matrix_event_type(),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        )
    } else {
        format!(
            "Save {} for {room_label} as `{}`? This will send {} through the Matrix SDK after confirmation. {} Power-level, membership, gateway/runtime/auth, and Telegram delivery mutations remain blocked.",
            field.label(),
            compact_message_preview(value, "empty"),
            field.matrix_event_type(),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        )
    }
}

fn room_settings_live_write_result_label(
    room_label: &str,
    field: RoomSettingsMutationField,
    value: &str,
    result: &Result<(), String>,
) -> String {
    let value = compact_message_preview(value, "empty");
    match result {
        Ok(())
            if field == RoomSettingsMutationField::Avatar
                && room_settings_avatar_upload_value(value.as_str()) =>
        {
            format!(
                "Room settings Avatar upload sent for {room_label}: Room::upload_avatar accepted by SDK for `{value}`. {} Room list/header will follow Matrix sync; canonical alias, power-level, membership, gateway/runtime/auth, and Telegram delivery remain blocked.",
                ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
            )
        }
        Ok(()) if field == RoomSettingsMutationField::Avatar => format!(
            "Room settings Avatar removal sent for {room_label}: {} accepted by SDK. {} Room list/header will follow Matrix sync; power-level, membership, gateway/runtime/auth, and Telegram delivery remain blocked.",
            field.matrix_event_type(),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        ),
        Ok(()) if field == RoomSettingsMutationField::CanonicalAlias => format!(
            "Room settings Canonical alias update sent for {room_label}: {} `{value}` accepted by SDK. {} Room list/header will follow Matrix sync; power-level, membership, gateway/runtime/auth, and Telegram delivery remain blocked.",
            field.matrix_event_type(),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        ),
        Ok(()) if field == RoomSettingsMutationField::Tombstone => format!(
            "Room settings Tombstone update sent for {room_label}: {} replacement `{value}` accepted by SDK. {} Room list/header will follow Matrix sync; power-level, member moderation, gateway/runtime/auth, and Telegram delivery remain blocked.",
            field.matrix_event_type(),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        ),
        Ok(()) => format!(
            "Room settings {} update sent for {room_label}: {} `{value}` accepted by SDK. {} Room list/header will follow Matrix sync; power-level, membership, gateway/runtime/auth, and Telegram delivery remain blocked.",
            field.label(),
            field.matrix_event_type(),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        ),
        Err(error)
            if field == RoomSettingsMutationField::Avatar
                && room_settings_avatar_upload_value(value.as_str()) =>
        {
            format!(
                "Update failed: room settings Avatar upload for {room_label} could not send Room::upload_avatar for `{value}`: {}. Retry is confirmation-gated. {} Canonical alias, power-level, membership, gateway/runtime/auth, and Telegram delivery remain blocked.",
                compact_message_preview(error, "unknown error"),
                ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
            )
        }
        Err(error) if field == RoomSettingsMutationField::Avatar => format!(
            "Update failed: room settings Avatar removal for {room_label} could not send {}: {}. Retry is confirmation-gated. {} Power-level, membership, gateway/runtime/auth, and Telegram delivery remain blocked.",
            field.matrix_event_type(),
            compact_message_preview(error, "unknown error"),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        ),
        Err(error) if field == RoomSettingsMutationField::CanonicalAlias => format!(
            "Update failed: room settings Canonical alias for {room_label} could not send {} `{value}`: {}. Retry is confirmation-gated and preserves loaded alternative aliases. {} Power-level, membership, gateway/runtime/auth, and Telegram delivery remain blocked.",
            field.matrix_event_type(),
            compact_message_preview(error, "unknown error"),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        ),
        Err(error) if field == RoomSettingsMutationField::Tombstone => format!(
            "Update failed: room settings Tombstone for {room_label} could not send {} replacement `{value}`: {}. Retry is confirmation-gated. {} Power-level, member moderation, gateway/runtime/auth, and Telegram delivery remain blocked.",
            field.matrix_event_type(),
            compact_message_preview(error, "unknown error"),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        ),
        Err(error) => format!(
            "Update failed: room settings {} for {room_label} could not send {} `{value}`: {}. Retry is confirmation-gated. {} Power-level, membership, gateway/runtime/auth, and Telegram delivery remain blocked.",
            field.label(),
            field.matrix_event_type(),
            compact_message_preview(error, "unknown error"),
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_LABEL
        ),
    }
}

fn room_settings_name_id_clipboard_payload(room_label: &str, room_id: &str) -> Option<String> {
    let room_id = room_id.trim();
    if room_id.is_empty() {
        return None;
    }
    let room_label = if room_label.trim().is_empty() {
        "this chat"
    } else {
        room_label.trim()
    };
    Some(format!(
        "Room settings name/id\nName: {room_label}\nRoom ID: {room_id}\nScope: loaded RoomScreen RoomNameId\nBoundary: no m.room.name/topic/avatar/canonical_alias/power_levels write"
    ))
}

fn room_settings_name_id_clipboard_label(
    copied: bool,
    room_label: &str,
    room_id: &str,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_levels_loaded: bool,
) -> String {
    let action_state = if copied {
        "copied loaded room name/id to local clipboard"
    } else {
        "name/id clipboard unavailable"
    };
    let label_state = if room_label.trim().is_empty() {
        "room label waiting".to_string()
    } else {
        format!(
            "room label `{}`",
            compact_message_preview(room_label, "this chat")
        )
    };
    let room_id_state = if room_id.trim().is_empty() {
        "room id waiting".to_string()
    } else {
        format!(
            "room id `{}`",
            compact_message_preview(room_id, "room id waiting")
        )
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata available"
    } else {
        "identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached member(s)"))
        .unwrap_or_else(|| "member cache waiting".to_string());
    let power_state = if power_levels_loaded {
        "power levels loaded"
    } else {
        "power levels waiting"
    };
    format!(
        "Room settings Name clipboard: {action_state}; {label_state}; {room_id_state}; {identity_state}; {member_state}; {power_state}. {ROOM_SETTINGS_NAME_ID_CLIPBOARD_LABEL} No m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.power_levels, membership write, invite, kick, ban, knock, notification rule, message mutation, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn allowed_blocked(allowed: bool) -> &'static str {
    if allowed { "allowed" } else { "blocked" }
}

fn room_settings_permissions_clipboard_payload(
    can_send: bool,
    can_react: bool,
    can_notify_room: bool,
) -> String {
    format!(
        "Room settings permissions\nSend messages: {}\nSend reactions: {}\nNotify @room: {}\nScope: loaded RoomScreen tl_state.user_power\nBoundary: no m.room.power_levels write",
        allowed_blocked(can_send),
        allowed_blocked(can_react),
        allowed_blocked(can_notify_room)
    )
}

fn room_settings_permissions_clipboard_label(
    copied: bool,
    room_label: &str,
    permissions: Option<(bool, bool, bool)>,
    identity_loaded: bool,
    member_count: Option<usize>,
) -> String {
    let action_state = if copied {
        "copied loaded permission summary to local clipboard"
    } else {
        "permissions clipboard unavailable"
    };
    let room_state = if room_label.trim().is_empty() {
        "room label waiting".to_string()
    } else {
        format!(
            "room label `{}`",
            compact_message_preview(room_label, "this chat")
        )
    };
    let permissions_state = permissions
        .map(|(can_send, can_react, can_notify_room)| {
            format!(
                "send {}; react {}; @room {}",
                allowed_blocked(can_send),
                allowed_blocked(can_react),
                allowed_blocked(can_notify_room)
            )
        })
        .unwrap_or_else(|| "power levels waiting".to_string());
    let identity_state = if identity_loaded {
        "loaded identity metadata available"
    } else {
        "identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached member(s)"))
        .unwrap_or_else(|| "member cache waiting".to_string());
    format!(
        "Room settings Permissions clipboard: {action_state}; {room_state}; {permissions_state}; {identity_state}; {member_state}. {ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_LABEL} No m.room.power_levels, m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, membership write, invite, kick, ban, knock, notification rule, message mutation, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn room_settings_members_cache_preview(members: &[RoomMember]) -> String {
    if members.is_empty() {
        return "no members in loaded cache".to_string();
    }

    let preview = members
        .iter()
        .take(6)
        .map(room_settings_member_cache_row)
        .collect::<Vec<_>>()
        .join("; ");
    if members.len() > 6 {
        format!(
            "{preview}; +{} more local cached member(s)",
            members.len() - 6
        )
    } else {
        preview
    }
}

fn room_settings_member_cache_row(member: &RoomMember) -> String {
    let display_name = member
        .display_name()
        .map(|display_name| compact_message_preview(display_name, "display name unavailable"))
        .unwrap_or_else(|| "display name unavailable".to_string());
    format!("{display_name} <{}>", member.user_id())
}

fn room_settings_members_clipboard_payload(member_count: usize, member_preview: &str) -> String {
    let preview = compact_message_preview(member_preview, "member cache sample waiting");
    format!(
        "Room settings members\nLoaded members: {member_count}\nSample: {preview}\nScope: loaded RoomScreen room_members local cache\nBoundary: no membership write"
    )
}

fn room_settings_members_clipboard_label(
    copied: bool,
    room_label: &str,
    member_count: Option<usize>,
    member_preview: &str,
    identity_loaded: bool,
    power_levels_loaded: bool,
) -> String {
    let action_state = if copied {
        "copied loaded member-cache summary to local clipboard"
    } else {
        "members clipboard unavailable"
    };
    let room_state = if room_label.trim().is_empty() {
        "room label waiting".to_string()
    } else {
        format!(
            "room label `{}`",
            compact_message_preview(room_label, "this chat")
        )
    };
    let member_state = member_count
        .map(|count| format!("{count} cached member(s)"))
        .unwrap_or_else(|| "member cache waiting".to_string());
    let preview_state = format!(
        "sample `{}`",
        compact_message_preview(member_preview, "member cache sample waiting")
    );
    let identity_state = if identity_loaded {
        "loaded identity metadata available"
    } else {
        "identity metadata waiting"
    };
    let power_state = if power_levels_loaded {
        "power levels loaded"
    } else {
        "power levels waiting"
    };
    format!(
        "Room settings Members clipboard: {action_state}; {room_state}; {member_state}; {preview_state}; {identity_state}; {power_state}. {ROOM_SETTINGS_MEMBERS_CLIPBOARD_LABEL} No membership write, invite, kick, ban, knock, m.room.member mutation, m.room.power_levels, m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, notification rule, message mutation, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn room_settings_identity_clipboard_payload(
    room_label: &str,
    room_id: &str,
    canonical_alias: Option<&str>,
    alt_alias_count: usize,
    room_avatar_loaded: bool,
    is_tombstoned: bool,
    member_count: Option<usize>,
) -> Option<String> {
    let room_id = room_id.trim();
    if room_id.is_empty() {
        return None;
    }
    let room_label = if room_label.trim().is_empty() {
        "this chat"
    } else {
        room_label.trim()
    };
    let alias_state = canonical_alias
        .map(str::trim)
        .filter(|alias| !alias.is_empty())
        .unwrap_or("none loaded");
    let avatar_state = if room_avatar_loaded {
        "avatar image cached"
    } else {
        "avatar fallback"
    };
    let tombstone_state = if is_tombstoned {
        "tombstoned"
    } else {
        "not tombstoned"
    };
    let member_state = member_count
        .map(|count| format!("{count} loaded member(s)"))
        .unwrap_or_else(|| "member cache waiting".to_string());
    Some(format!(
        "Room settings identity\nName: {room_label}\nRoom ID: {room_id}\nCanonical alias: {alias_state}\nAlternative aliases: {alt_alias_count}\nAvatar: {avatar_state}\nTombstone: {tombstone_state}\nMembers: {member_state}\nScope: loaded RoomsList RoomContextMenuDetails + RoomScreen member cache\nBoundary: no m.room.name/topic/avatar/canonical_alias/tombstone/power_levels write"
    ))
}

fn room_settings_identity_clipboard_label(
    copied: bool,
    room_label: &str,
    room_id: &str,
    canonical_alias: Option<&str>,
    alt_alias_count: Option<usize>,
    room_avatar_loaded: Option<bool>,
    is_tombstoned: Option<bool>,
    member_count: Option<usize>,
) -> String {
    let action_state = if copied {
        "copied loaded identity metadata to local clipboard"
    } else {
        "identity clipboard unavailable"
    };
    let room_state = if room_label.trim().is_empty() {
        "room label waiting".to_string()
    } else {
        format!(
            "room label `{}`",
            compact_message_preview(room_label, "this chat")
        )
    };
    let room_id_state = if room_id.trim().is_empty() {
        "room id waiting".to_string()
    } else {
        format!(
            "room id `{}`",
            compact_message_preview(room_id, "room id waiting")
        )
    };
    let alias_state = canonical_alias
        .map(str::trim)
        .filter(|alias| !alias.is_empty())
        .map(|alias| {
            format!(
                "canonical alias `{}`",
                compact_message_preview(alias, "alias waiting")
            )
        })
        .unwrap_or_else(|| "canonical alias missing".to_string());
    let alt_alias_state = alt_alias_count
        .map(|count| format!("{count} alternative alias(es)"))
        .unwrap_or_else(|| "alternative aliases waiting".to_string());
    let avatar_state = room_avatar_loaded
        .map(|loaded| {
            if loaded {
                "avatar image cached"
            } else {
                "avatar fallback"
            }
        })
        .unwrap_or("avatar metadata waiting");
    let tombstone_state = is_tombstoned
        .map(|tombstoned| {
            if tombstoned {
                "tombstoned"
            } else {
                "not tombstoned"
            }
        })
        .unwrap_or("tombstone metadata waiting");
    let member_state = member_count
        .map(|count| format!("{count} cached member(s)"))
        .unwrap_or_else(|| "member cache waiting".to_string());
    format!(
        "Room settings Identity clipboard: {action_state}; {room_state}; {room_id_state}; {alias_state}; {alt_alias_state}; {avatar_state}; {tombstone_state}; {member_state}. {ROOM_SETTINGS_IDENTITY_CLIPBOARD_LABEL} No m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.tombstone, m.room.power_levels, membership write, invite, kick, ban, knock, notification rule, message mutation, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn room_settings_close_metadata_label(
    room_label: &str,
    option_staged: bool,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_levels_loaded: bool,
) -> String {
    let option_state = if option_staged {
        "last option preview retained"
    } else {
        "no option preview staged"
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} members loaded"))
        .unwrap_or_else(|| "members waiting".to_string());
    let power_state = if power_levels_loaded {
        "power levels loaded"
    } else {
        "power levels waiting"
    };
    format!(
        "Room settings closed for {room_label}: {option_state}; {identity_state}; {member_state}; {power_state}; local strip hidden only. {ROOM_SETTINGS_CLOSE_METADATA_LABEL}"
    )
}

fn room_settings_refresh_metadata_label(
    room_label: &str,
    timeline_loaded: bool,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_levels_loaded: bool,
) -> String {
    let timeline_state = if timeline_loaded {
        "timeline ready for read refresh"
    } else {
        "timeline waiting for read refresh"
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members before refresh"))
        .unwrap_or_else(|| "cached members waiting before refresh".to_string());
    let power_state = if power_levels_loaded {
        "power-level display ready"
    } else {
        "power-level display waiting"
    };
    let live_read_state = room_settings_refresh_live_read_wiring_label(
        timeline_loaded,
        member_count,
        power_levels_loaded,
    );
    format!(
        "Room settings refresh for {room_label}: {timeline_state}; {identity_state}; {member_state}; {power_state}; {live_read_state}. {ROOM_SETTINGS_REFRESH_METADATA_LABEL}"
    )
}

fn room_settings_refresh_live_read_wiring_label(
    timeline_loaded: bool,
    member_count: Option<usize>,
    power_levels_loaded: bool,
) -> String {
    let request_state = if timeline_loaded {
        "live read refresh submits GetRoomPowerLevels plus GetRoomMembers(local_only=false, JOIN)"
    } else {
        "live read refresh waits for a loaded timeline"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached member(s) before refresh"))
        .unwrap_or_else(|| "member cache pending".to_string());
    let power_state = if power_levels_loaded {
        "power-level baseline present"
    } else {
        "power-level baseline pending"
    };
    format!(
        "{request_state}; {member_state}; {power_state}; editable room-state writes remain blocked"
    )
}

fn room_settings_edit_controls_boundary_label(
    room_label: &str,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_levels_loaded: bool,
) -> String {
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members"))
        .unwrap_or_else(|| "cached members waiting".to_string());
    let power_state = if power_levels_loaded {
        "power-level display ready"
    } else {
        "power-level display waiting"
    };
    format!(
        "Room settings edit-controls boundary for {room_label}: {identity_state}; {member_state}; {power_state}. {ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_LABEL} No m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.history_visibility, m.room.join_rules, m.room.power_levels, membership moderation, invite, kick, ban, knock, tombstone, notification-rule handoff, gateway/runtime/auth, or live mutation."
    )
}

fn room_settings_edit_intent_staging_label(
    room_label: &str,
    edit_intent: &str,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_levels_loaded: bool,
) -> String {
    let edit_intent = edit_intent.trim();
    let edit_intent = if edit_intent.is_empty() {
        "room-state edit"
    } else {
        edit_intent
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members"))
        .unwrap_or_else(|| "cached members waiting".to_string());
    let power_state = if power_levels_loaded {
        "power-level display ready"
    } else {
        "power-level display waiting"
    };
    format!(
        "Room settings edit intent staged locally for {room_label}: {edit_intent}; {identity_state}; {member_state}; {power_state}. {ROOM_SETTINGS_EDIT_INTENT_STAGING_LABEL} No m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.history_visibility, m.room.join_rules, m.room.power_levels, member moderation, invite, kick, ban, knock, tombstone, notification-rule handoff, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn room_settings_field_edit_intent_controls_label(
    room_label: &str,
    field_intent: &str,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_levels_loaded: bool,
) -> String {
    let field_intent = field_intent.trim();
    let field_intent = if field_intent.is_empty() {
        "room field"
    } else {
        field_intent
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members"))
        .unwrap_or_else(|| "cached members waiting".to_string());
    let power_state = if power_levels_loaded {
        "power-level display ready"
    } else {
        "power-level display waiting"
    };
    format!(
        "Room settings field edit intent staged locally for {room_label}: {field_intent}; {identity_state}; {member_state}; {power_state}. {ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_LABEL} No m.room.name, m.room.topic, m.room.avatar, m.room.power_levels, membership list write, invite, kick, ban, knock, canonical alias, history visibility, join rule, tombstone, notification-rule handoff, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn room_settings_refresh_result_detail_label(
    room_label: &str,
    action: Option<&str>,
    timeline_loaded: bool,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_summary: &str,
    local_status_chars: usize,
) -> String {
    let action = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .map(|action| format!("{action} detail selected"))
        .unwrap_or_else(|| "no refresh result detail selected".to_string());
    let timeline_state = if timeline_loaded {
        "timeline ready"
    } else {
        "timeline waiting"
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members"))
        .unwrap_or_else(|| "cached members waiting".to_string());
    let power_summary = power_summary.trim();
    let power_summary = if power_summary.is_empty() {
        "power result waiting"
    } else {
        power_summary
    };
    format!(
        "Room settings refresh result detail for {room_label}: {action}; {timeline_state}; {identity_state}; {member_state}; {power_summary}; local status {local_status_chars} chars. {ROOM_SETTINGS_REFRESH_RESULT_DETAIL_LABEL} Refresh is the only control that reuses GetRoomPowerLevels and GetRoomMembers(server-backed). Result, Members, Power, Failure, and Source send no extra read, no m.room.name, no m.room.topic, no m.room.avatar, no m.room.power_levels mutation, membership list write, invite, kick, ban, knock, canonical alias, history visibility, join rule, tombstone, notification-rule handoff, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn room_settings_mutation_preflight_detail_label(
    room_label: &str,
    action: Option<&str>,
    timeline_loaded: bool,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_summary: &str,
    local_status_chars: usize,
) -> String {
    let action = action
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .map(|action| format!("{action} preflight selected"))
        .unwrap_or_else(|| "no mutation preflight selected".to_string());
    let timeline_state = if timeline_loaded {
        "timeline ready"
    } else {
        "timeline waiting"
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members"))
        .unwrap_or_else(|| "cached members waiting".to_string());
    let power_summary = power_summary.trim();
    let power_summary = if power_summary.is_empty() {
        "power result waiting"
    } else {
        power_summary
    };
    format!(
        "Room settings room-state mutation preflight for {room_label}: {action}; {timeline_state}; {identity_state}; {member_state}; {power_summary}; local status {local_status_chars} chars. {ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_LABEL} Request, Result, Error, Retry, and Source submit no m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.history_visibility, m.room.join_rules, m.room.power_levels, membership list write, invite, kick, ban, knock, tombstone, notification-rule handoff, retry automation, room-state mutation contract call, message mutation, gateway/runtime/auth, or live mutation."
    )
}

fn room_settings_mutation_request_packet_snapshot_label(
    room_label: &str,
    timeline_loaded: bool,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_summary: &str,
    local_status_chars: usize,
) -> String {
    let timeline_state = if timeline_loaded {
        "timeline ready"
    } else {
        "timeline waiting"
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members"))
        .unwrap_or_else(|| "cached members waiting".to_string());
    let power_summary = power_summary.trim();
    let power_summary = if power_summary.is_empty() {
        "power result waiting"
    } else {
        power_summary
    };

    format!(
        "Local room-state mutation packet snapshot for {room_label}: Request selected. {timeline_state}; {identity_state}; {member_state}; {power_summary}; local status {local_status_chars} chars. Proposed request body, result slot, error slot, retry eligibility, source summary, m.room.name/topic/avatar/canonical_alias/history_visibility/join_rules/power_levels fields, membership list scope, invite/kick/ban/knock scope, tombstone scope, notification-rule handoff, and typed room-settings mutation contract target are represented as local metadata only. No m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.history_visibility, m.room.join_rules, m.room.power_levels, membership list write, invite, kick, ban, knock, tombstone, notification-rule handoff, retry automation, room-state mutation contract call, message mutation, gateway/runtime/auth, or live mutation was submitted. {ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_LABEL}"
    )
}

fn room_settings_field_mutation_packet_payload(
    room_label: &str,
    timeline_loaded: bool,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_summary: &str,
    local_status: &str,
    last_preflight_action: &str,
) -> String {
    let timeline_state = if timeline_loaded {
        "timeline ready"
    } else {
        "timeline waiting"
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members"))
        .unwrap_or_else(|| "cached members waiting".to_string());
    let power_summary = power_summary.trim();
    let power_summary = if power_summary.is_empty() {
        "power result waiting"
    } else {
        power_summary
    };
    let local_status = compact_message_preview(local_status, "no local settings status staged");
    let last_preflight_action = last_preflight_action.trim();
    let last_preflight_action = if last_preflight_action.is_empty() {
        "no prior preflight control"
    } else {
        last_preflight_action
    };

    format!(
        "Room settings field mutation packet\nRoom: {room_label}\nTimeline: {timeline_state}\nIdentity: {identity_state}\nMembers: {member_state}\nPower: {power_summary}\nLast preflight: {last_preflight_action}\nLocal status: {local_status}\nAcceptance matrix:\n- Name: requires explicit confirmation, m.room.name body, result slot, error slot, retry slot, and loaded-source summary before any write.\n- Topic: requires explicit confirmation, m.room.topic body, result slot, error slot, retry slot, and loaded-source summary before any write.\n- Avatar: requires selected media/crop handoff, explicit confirmation, m.room.avatar body, result slot, error slot, and no upload before review.\n- History visibility: preset writes require explicit confirmation, typed visibility enum, result/error/retry slots, and the existing SetRoomHistoryVisibility path.\n- Join rules: preset writes require explicit confirmation, typed join-rule payload, result/error/retry slots, and the existing SetRoomJoinRule path.\n- Aliases: canonical alias and alternative aliases require typed alias source, explicit confirmation, result/error/retry slots, and no alias write here.\n- Tombstone: requires a valid replacement room id, explicit confirmation, result/error/retry slots, and the existing SetRoomTombstone path.\n- Power levels: requires loaded baseline, typed delta, explicit confirmation, result/error/retry slots, and no m.room.power_levels write here.\n- Members: invite/kick/ban/knock/moderation require selected member target, explicit confirmation, result/error/retry slots, and no m.room.member write here.\n- Notification handoff: stays out of room settings mutation packet until a typed notification-rule contract exists.\nPromotion criteria: map remaining power/member room-state fields to typed room-settings mutation contracts with confirmation, result, error, retry, and source payload before implementation.\nBoundary: {ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_LABEL} No m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.history_visibility, m.room.join_rules, m.room.power_levels, membership write, invite, kick, ban, knock, notification-rule handoff, retry automation, room-state mutation contract call, message mutation, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn room_settings_field_mutation_packet_clipboard_label(
    copied: bool,
    room_label: &str,
    timeline_loaded: bool,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_summary: &str,
    local_status_chars: usize,
) -> String {
    let action_state = if copied {
        "copied field-by-field mutation packet to local clipboard"
    } else {
        "field mutation packet clipboard unavailable"
    };
    let timeline_state = if timeline_loaded {
        "timeline ready"
    } else {
        "timeline waiting"
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members"))
        .unwrap_or_else(|| "cached members waiting".to_string());
    let power_summary = power_summary.trim();
    let power_summary = if power_summary.is_empty() {
        "power result waiting"
    } else {
        power_summary
    };

    format!(
        "Room settings field mutation packet for {room_label}: {action_state}; {timeline_state}; {identity_state}; {member_state}; {power_summary}; local status {local_status_chars} chars. {ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_LABEL} No m.room.name/topic/avatar/canonical_alias/history_visibility/join_rules/power_levels write, membership write, invite, kick, ban, knock, notification-rule handoff, retry automation, room-state mutation contract call, gateway/runtime/auth, or live mutation."
    )
}

fn room_settings_field_mutation_contract_packet_payload(
    room_label: &str,
    timeline_loaded: bool,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_summary: &str,
    local_status: &str,
    last_preflight_action: &str,
) -> String {
    let timeline_state = if timeline_loaded {
        "timeline ready"
    } else {
        "timeline waiting"
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members"))
        .unwrap_or_else(|| "cached members waiting".to_string());
    let power_summary = power_summary.trim();
    let power_summary = if power_summary.is_empty() {
        "power result waiting"
    } else {
        power_summary
    };
    let local_status = compact_message_preview(local_status, "no local settings status staged");
    let last_preflight_action = last_preflight_action.trim();
    let last_preflight_action = if last_preflight_action.is_empty() {
        "no prior preflight control"
    } else {
        last_preflight_action
    };

    format!(
        "Room settings typed room-state mutation/result contract\nRoom: {room_label}\nTimeline: {timeline_state}\nIdentity: {identity_state}\nMembers: {member_state}\nPower: {power_summary}\nLast preflight: {last_preflight_action}\nLocal status: {local_status}\nContract matrix:\n- Baseline identity: loaded room id/name, canonical alias, avatar mxc, tombstone, member count, and power-level baseline must be stable before any write.\n- Name contract: typed m.room.name request body, confirmation id, result event id, stale-baseline detection, error taxonomy, retry eligibility, and source hash are required before a name write.\n- Topic contract: typed m.room.topic request body, confirmation id, result event id, stale-baseline detection, error taxonomy, retry eligibility, and source hash are required before a topic write.\n- Avatar contract: typed media source, upload result handoff, m.room.avatar request body, thumbnail/source metadata, result event id, error/retry/source slots, and rollback copy are required before avatar state writes.\n- Visibility and join-rule contracts: current preset writes use typed m.room.history_visibility and m.room.join_rules enums with confirmation, result/error/retry slots, and permission-error reporting; richer selection/source UX remains contract-first.\n- Alias contract: typed canonical alias and alternative aliases baseline, alias ownership result, error taxonomy, retry/source slots, and conflict handling are required before alias writes.\n- Tombstone contract: current replacement writes use a typed successor room id, PositiveConfirmationModal, result/error/retry slots, and RoomTombstoneEventContent; richer navigation fallback remains contract-first.\n- Power-level contract: loaded power-level baseline, typed delta, affected actions, confirmation id, result event id, stale baseline handling, and rollback/source slots are required before m.room.power_levels writes.\n- Member moderation contract: typed member target, invite/kick/ban/knock body, power permission result, membership result/error/retry/source slots, and audit copy are required before m.room.member moderation writes.\n- Notification handoff: room-settings mutation contracts must reference the typed notification rule contract instead of writing notification rules here.\nPromotion blocker: map remaining power/member room-state fields to typed mutation/result contracts before writes.\nBoundary: {ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_LABEL} No m.room.name, m.room.topic, m.room.avatar, m.room.canonical_alias, m.room.history_visibility, m.room.join_rules, m.room.power_levels, membership write, invite, kick, ban, knock, notification-rule handoff, retry automation, room-state mutation contract call, message mutation, account/profile, gateway/runtime/auth, or live mutation."
    )
}

fn room_settings_field_mutation_contract_packet_clipboard_label(
    copied: bool,
    room_label: &str,
    timeline_loaded: bool,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_summary: &str,
    local_status_chars: usize,
) -> String {
    let action_state = if copied {
        "contract copied"
    } else {
        "contract unavailable"
    };
    let timeline_state = if timeline_loaded {
        "timeline ready"
    } else {
        "timeline waiting"
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members"))
        .unwrap_or_else(|| "cached members waiting".to_string());
    let power_summary = power_summary.trim();
    let power_summary = if power_summary.is_empty() {
        "power result waiting"
    } else {
        power_summary
    };

    format!(
        "Room settings typed contract for {room_label}: {action_state}; {timeline_state}; {identity_state}; {member_state}; {power_summary}; local status {local_status_chars} chars. {ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_LABEL} No m.room.name/topic/avatar/canonical_alias/history_visibility/join_rules/power_levels write, membership write, invite, kick, ban, knock, tombstone, notification-rule handoff, retry automation, room-state mutation contract call, gateway/runtime/auth, or live mutation."
    )
}

fn room_settings_power_member_result_taxonomy_packet_payload(
    room_label: &str,
    timeline_loaded: bool,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_summary: &str,
    local_status: &str,
    last_preflight_action: &str,
) -> String {
    let timeline_state = if timeline_loaded {
        "timeline ready"
    } else {
        "timeline waiting"
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members"))
        .unwrap_or_else(|| "cached members waiting".to_string());
    let power_summary = power_summary.trim();
    let power_summary = if power_summary.is_empty() {
        "power result waiting"
    } else {
        power_summary
    };
    let local_status = compact_message_preview(local_status, "no local settings status staged");
    let last_preflight_action = last_preflight_action.trim();
    let last_preflight_action = if last_preflight_action.is_empty() {
        "no prior preflight control"
    } else {
        last_preflight_action
    };

    format!(
        "Room settings power/member result taxonomy packet\nRoom: {room_label}\nTimeline: {timeline_state}\nIdentity: {identity_state}\nMembers: {member_state}\nPower: {power_summary}\nLast preflight: {last_preflight_action}\nLocal status: {local_status}\nLive result references:\n- Existing room-state writes: confirmed MatrixRequest::SetRoomName, SetRoomTopic, SetRoomCanonicalAlias, UploadRoomAvatar, RemoveRoomAvatar, SetRoomHistoryVisibility, SetRoomJoinRule, and SetRoomTombstone result paths only.\n- Refresh reads: MatrixRequest::GetRoomPowerLevels and MatrixRequest::GetRoomMembers result paths only.\nBlocked power/member result slots:\n- power_levels_operation_id: not_assigned\n- power_levels_request_slot: not_built\n- power_levels_result: applied, permission_denied, forbidden, stale_baseline, invalid_delta, failed not_wired\n- power_levels_rollback_slot: not_built\n- member_moderation_operation_id: not_assigned\n- member_moderation_target_slot: no_selected_member_contract\n- invite_result: accepted, permission_denied, already_in_state, failed, stale_membership not_wired\n- kick_result: accepted, permission_denied, already_in_state, failed, stale_membership not_wired\n- ban_result: accepted, permission_denied, already_in_state, failed, stale_membership not_wired\n- knock_result: accepted, permission_denied, already_in_state, failed, stale_membership not_wired\n- membership_audit_event_slot: not_built\n- retry_policy: PositiveConfirmationModal backend_request_id_and_source_hash_required\n- cancel_policy: local_dismiss_no_request\n- stale_policy: room_id_power_event_source_hash_and_member_snapshot_required_before_live\n- audit_redaction: no access token, raw moderation reason, invite address, profile PII, or full power event JSON in local packet\nBoundary: {ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_LABEL} No m.room.power_levels write, m.room.member mutation, invite, kick, ban, knock, notification-rule handoff, retry automation, room-state mutation contract call, account/profile, gateway/runtime/auth, Telegram delivery, or live mutation."
    )
}

fn room_settings_power_member_result_taxonomy_packet_clipboard_label(
    copied: bool,
    room_label: &str,
    timeline_loaded: bool,
    identity_loaded: bool,
    member_count: Option<usize>,
    power_summary: &str,
    local_status_chars: usize,
) -> String {
    let action_state = if copied {
        "taxonomy copied"
    } else {
        "taxonomy unavailable"
    };
    let timeline_state = if timeline_loaded {
        "timeline ready"
    } else {
        "timeline waiting"
    };
    let identity_state = if identity_loaded {
        "loaded identity metadata ready"
    } else {
        "loaded identity metadata waiting"
    };
    let member_state = member_count
        .map(|count| format!("{count} cached members"))
        .unwrap_or_else(|| "cached members waiting".to_string());
    let power_summary = power_summary.trim();
    let power_summary = if power_summary.is_empty() {
        "power result waiting"
    } else {
        power_summary
    };

    format!(
        "Room settings taxonomy for {room_label}: {action_state}; {timeline_state}; {identity_state}; {member_state}; {power_summary}; local status {local_status_chars} chars. {ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_LABEL} No m.room.power_levels write, m.room.member mutation, invite, kick, ban, knock, retry automation, room-state mutation contract call, gateway/runtime/auth, Telegram delivery, or live mutation."
    )
}

fn notifications_close_refresh_metadata_label(
    room_label: &str,
    action: &str,
    mode_state: &str,
    local_status: Option<&str>,
    attention_loaded: bool,
    timeline_loaded: bool,
) -> String {
    let local_state = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| format!("local status {status}"))
        .unwrap_or_else(|| "no local status staged".to_string());
    let attention_state = if attention_loaded {
        "loaded attention metadata ready"
    } else {
        "loaded attention metadata waiting"
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded for mode read"
    } else {
        "timeline waiting for mode read"
    };
    format!(
        "Notification {action} metadata for {room_label}: {mode_state}; {local_state}; {attention_state}; {timeline_state}. {NOTIFICATIONS_CLOSE_REFRESH_METADATA_LABEL}"
    )
}

fn notifications_timed_global_boundary_label(
    mode_state: &str,
    loaded_attention: &str,
    local_status: Option<&str>,
) -> String {
    let local_state = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| format!("local status {status}"))
        .unwrap_or_else(|| "no local status staged".to_string());
    format!(
        "Boundary: {mode_state}; {loaded_attention}; {local_state}; timed mute, push gateway/device, pusher config, and sound/badge stay unwired; Global/Defaults can read the SDK default room mode live, and Default All/Mentions/Mute can write that default only after confirmation. {NOTIFICATIONS_TIMED_GLOBAL_BOUNDARY_LABEL}"
    )
}

fn notifications_pusher_keyword_boundary_label(
    mode_state: &str,
    loaded_attention: &str,
    retry_cache_ready: bool,
    local_status: Option<&str>,
) -> String {
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    let local_state = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| format!("local status {status}"))
        .unwrap_or_else(|| "no local status staged".to_string());
    format!(
        "Pusher/keyword boundary: {mode_state}; {loaded_attention}; {retry_state}; {local_state}. Keyword rules and Keyword list can read live Matrix notification settings; Add keyword and Remove keyword confirm before live SDK notification keyword writes; Device push and Pushers can read live homeserver push capability; Global/Defaults can read live SDK default room mode; Default All/Mentions/Mute confirm before live SDK default room-mode writes; timed mute presets, push gateway/device setup writes, pusher enable/disable mutations, sound/badge tuning, raw account-data edits, and room-list notification indication stay local blocked controls. {NOTIFICATIONS_PUSHER_KEYWORD_BOUNDARY_LABEL} No raw notification rule account-data edit, pusher mutation, unconfirmed keyword write, timed mute write, unconfirmed default mode write, room-state, membership, gateway/runtime/auth, or unrelated live mutation."
    )
}

fn notifications_mode_clipboard_payload(
    room_label: &str,
    mode_label: &str,
    loaded_attention: &str,
    local_status: Option<&str>,
) -> Option<String> {
    let mode_label = mode_label.trim();
    if mode_label.is_empty() {
        return None;
    }
    let room_label = room_label.trim();
    let room_label = if room_label.is_empty() {
        "this chat"
    } else {
        room_label
    };
    let attention = loaded_attention.trim();
    let attention = if attention.is_empty() {
        "Loaded attention: unavailable"
    } else {
        attention
    };
    let local_status = local_status
        .map(str::trim)
        .filter(|status| !status.is_empty())
        .unwrap_or("no local status staged");

    Some(format!(
        "Notification mode\nRoom: {room_label}\nMode: {mode_label}\nAttention: {attention}\nLocal status: {local_status}\nSource: loaded RoomScreen notification state"
    ))
}

fn notifications_mode_clipboard_label(
    copied: bool,
    room_label: &str,
    mode_label: Option<&str>,
    loaded_attention: &str,
    timeline_loaded: bool,
    local_status: Option<&str>,
) -> String {
    let payload_state = if copied {
        "clipboard payload copied"
    } else {
        "clipboard payload unavailable"
    };
    let room_label = room_label.trim();
    let room_label = if room_label.is_empty() {
        "this chat"
    } else {
        room_label
    };
    let mode_state = mode_label
        .map(str::trim)
        .filter(|mode| !mode.is_empty())
        .map(|mode| format!("loaded mode {mode}"))
        .unwrap_or_else(|| "loaded mode waiting".to_string());
    let attention = loaded_attention.trim();
    let attention_state = if attention.is_empty() {
        "Loaded attention: unavailable"
    } else {
        attention
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded"
    } else {
        "timeline waiting"
    };
    let local_state = local_status
        .map(str::trim)
        .filter(|status| !status.is_empty())
        .map(|status| format!("local status {status}"))
        .unwrap_or_else(|| "no local status staged".to_string());

    format!(
        "Notification mode clipboard for {room_label}: {payload_state}; {mode_state}; {attention_state}; {timeline_state}; {local_state}. {NOTIFICATIONS_MODE_CLIPBOARD_LABEL} No SetRoomNotificationMode, timed mute, global preference, keyword rule, push gateway/device, pusher, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_advanced_control_boundary_label(
    room_label: &str,
    control: &str,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let control = if control.trim().is_empty() {
        "Advanced notification control"
    } else {
        control.trim()
    };
    format!(
        "{control} staged for {room_label}: {mode_state}; {loaded_attention}. {NOTIFICATIONS_ADVANCED_CONTROLS_LABEL} Keywords may submit a live read-only GetNotificationKeywordRules request and Global may submit a live read-only GetDefaultRoomNotificationMode request; the separate Default All/Mentions/Mute row gates SetDefaultRoomNotificationMode behind confirmation; no raw notification rule account-data edit, pusher mutation, push gateway/device configuration, unconfirmed keyword write, unconfirmed default write, timed mute write, message mutation, room-state, membership, gateway/runtime/auth, or unrelated live mutation."
    )
}

fn notifications_advanced_detail_control_label(
    room_label: &str,
    control: &str,
    mode_state: &str,
    loaded_attention: &str,
    retry_cache_ready: bool,
) -> String {
    let control = if control.trim().is_empty() {
        "Advanced notification detail"
    } else {
        control.trim()
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    format!(
        "{control} detail staged for {room_label}: {mode_state}; {loaded_attention}; {retry_state}. {NOTIFICATIONS_ADVANCED_DETAIL_CONTROLS_LABEL} Keyword list may submit a live read-only GetNotificationKeywordRules request and Defaults may submit a live read-only GetDefaultRoomNotificationMode request; the separate Default All/Mentions/Mute row gates SetDefaultRoomNotificationMode behind confirmation; no notification rule account-data edit, push-rule write beyond SDK keyword/default APIs, pusher mutation, push gateway/device configuration, timed mute write, unconfirmed default write, sound/badge tuning, retry automation, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notification_keyword_rules_summary_label(summary: &NotificationKeywordRulesSummary) -> String {
    let count = summary.enabled_keywords.len();
    if count == 0 {
        return if summary.has_enabled_keywords {
            "SDK reports enabled keyword rules, but no keyword patterns were available".to_string()
        } else {
            "no enabled custom keyword rules".to_string()
        };
    }

    let shown = summary
        .enabled_keywords
        .iter()
        .take(5)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(", ");
    let more_count = count.saturating_sub(5);
    let suffix = if more_count > 0 {
        format!(" +{more_count} more")
    } else {
        String::new()
    };
    format!("{count} enabled custom keyword rule(s): {shown}{suffix}")
}

fn normalized_notification_keyword(keyword: &str) -> Option<String> {
    let keyword = keyword.trim();
    if keyword.is_empty() {
        None
    } else {
        Some(keyword.to_string())
    }
}

fn notifications_keyword_rules_live_read_requested_label(
    room_label: &str,
    control: &str,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let control = control.trim();
    let control = if control.is_empty() {
        "Keywords"
    } else {
        control
    };
    format!(
        "{control} live read submitted for {room_label}: {mode_state}; {loaded_attention}. {NOTIFICATIONS_KEYWORD_LIST_LIVE_READ_LABEL} MatrixRequest::GetNotificationKeywordRules is waiting for NotificationSettings::enabled_keywords; no unconfirmed keyword write, pusher mutation, timed mute, global preference, sound/badge tuning, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_keyword_rules_unavailable_label(
    room_label: &str,
    control: &str,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let control = control.trim();
    let control = if control.is_empty() {
        "Keywords"
    } else {
        control
    };
    format!(
        "{control} live read unavailable for {room_label}: timeline waiting; {mode_state}; {loaded_attention}. {NOTIFICATIONS_KEYWORD_LIST_LIVE_READ_LABEL} No MatrixRequest::GetNotificationKeywordRules, no keyword write, pusher mutation, timed mute, global preference, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_keyword_rules_live_result_label(
    room_label: &str,
    summary: &NotificationKeywordRulesSummary,
    mode_state: &str,
    loaded_attention: &str,
    local_status: Option<&str>,
) -> String {
    let keyword_summary = notification_keyword_rules_summary_label(summary);
    let local_status_chars = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| status.chars().count())
        .unwrap_or(0);
    format!(
        "Keyword list live result for {room_label}: {keyword_summary}; {mode_state}; {loaded_attention}; prior local status {local_status_chars} chars. {NOTIFICATIONS_KEYWORD_LIST_LIVE_READ_LABEL} Read through NotificationSettings::contains_keyword_rules and enabled_keywords; no unconfirmed add/remove keyword rule write, account-data mutation, pusher mutation, push gateway/device configuration, timed mute, global preference, sound/badge tuning, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_keyword_mutation_confirmation_label(
    room_label: &str,
    keyword: &str,
    mutation: NotificationKeywordMutation,
) -> String {
    let action = notification_keyword_mutation_action_label(mutation);
    format!(
        "{action} rule for {room_label}: {keyword}? {NOTIFICATIONS_KEYWORD_MUTATION_LABEL} PositiveConfirmationModal gates MatrixRequest::SetNotificationKeywordRule before NotificationSettings::add_keyword/remove_keyword."
    )
}

fn notifications_keyword_mutation_requested_label(
    room_label: &str,
    keyword: &str,
    mutation: NotificationKeywordMutation,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let action = notification_keyword_mutation_action_label(mutation);
    format!(
        "{action} confirmation opened for {room_label}: {keyword}; {mode_state}; {loaded_attention}. {NOTIFICATIONS_KEYWORD_MUTATION_LABEL} MatrixRequest::SetNotificationKeywordRule waits for PositiveConfirmationModal accept before NotificationSettings::add_keyword/remove_keyword; no timed mute, global preference, pusher mutation, push gateway/device configuration, sound/badge tuning, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_keyword_mutation_unavailable_label(
    room_label: &str,
    keyword: &str,
    mutation: NotificationKeywordMutation,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let action = notification_keyword_mutation_action_label(mutation);
    format!(
        "{action} unavailable for {room_label}: {keyword}; timeline waiting; {mode_state}; {loaded_attention}. {NOTIFICATIONS_KEYWORD_MUTATION_LABEL} No MatrixRequest::SetNotificationKeywordRule, pusher mutation, push gateway/device configuration, timed mute, global preference, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_keyword_mutation_result_label(
    room_label: &str,
    keyword: &str,
    mutation: NotificationKeywordMutation,
    result: &Result<(), String>,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let action = notification_keyword_mutation_action_label(mutation);
    match result {
        Ok(()) => {
            let verb = notification_keyword_mutation_verb(mutation);
            format!(
                "Keyword rule {verb} for {room_label}: {keyword}; {mode_state}; {loaded_attention}. {NOTIFICATIONS_KEYWORD_MUTATION_LABEL} NotificationSettings::add_keyword/remove_keyword succeeded; pusher mutation, push gateway/device configuration, timed mute, global preference, room-state, membership, gateway/runtime/auth, or unrelated live mutation was not requested."
            )
        }
        Err(error) => format!(
            "{action} failed for {room_label}: {keyword}; {mode_state}; {loaded_attention}; error {}. {NOTIFICATIONS_KEYWORD_MUTATION_LABEL} Failed-state Retry reopens PositiveConfirmationModal before resubmitting SetNotificationKeywordRule; no automatic retry, pusher mutation, timed mute, global preference, room-state, membership, gateway/runtime/auth, or unrelated live mutation.",
            compact_message_preview(error, "unknown error")
        ),
    }
}

fn notification_pusher_status_summary_label(summary: &NotificationPusherStatusSummary) -> String {
    match &summary.encrypted_event_to_device_push {
        Ok(true) => "homeserver supports encrypted push-to-device capability".to_string(),
        Ok(false) => {
            "homeserver does not advertise encrypted push-to-device capability".to_string()
        }
        Err(error) => format!(
            "homeserver push-to-device capability read failed: {}",
            compact_message_preview(error, "unknown error")
        ),
    }
}

fn notifications_pusher_status_live_read_requested_label(
    room_label: &str,
    control: &str,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let control = control.trim();
    let control = if control.is_empty() {
        "Pushers"
    } else {
        control
    };
    format!(
        "{control} live read submitted for {room_label}: {mode_state}; {loaded_attention}. {NOTIFICATIONS_PUSHER_STATUS_LIVE_READ_LABEL} MatrixRequest::GetNotificationPusherStatus is waiting for Client::can_homeserver_push_encrypted_event_to_device; no pusher set/delete mutation, push gateway/device configuration write, account-data mutation, timed mute, global preference, sound/badge tuning, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_pusher_status_unavailable_label(
    room_label: &str,
    control: &str,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let control = control.trim();
    let control = if control.is_empty() {
        "Pushers"
    } else {
        control
    };
    format!(
        "{control} live read unavailable for {room_label}: timeline waiting; {mode_state}; {loaded_attention}. {NOTIFICATIONS_PUSHER_STATUS_LIVE_READ_LABEL} No MatrixRequest::GetNotificationPusherStatus, pusher set/delete mutation, push gateway/device configuration write, account-data mutation, timed mute, global preference, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_pusher_status_live_result_label(
    room_label: &str,
    summary: &NotificationPusherStatusSummary,
    mode_state: &str,
    loaded_attention: &str,
    local_status: Option<&str>,
) -> String {
    let pusher_summary = notification_pusher_status_summary_label(summary);
    let local_status_chars = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| status.chars().count())
        .unwrap_or(0);
    format!(
        "Pusher status live result for {room_label}: {pusher_summary}; {mode_state}; {loaded_attention}; prior local status {local_status_chars} chars. {NOTIFICATIONS_PUSHER_STATUS_LIVE_READ_LABEL} Read through Client::can_homeserver_push_encrypted_event_to_device; no pusher set/delete mutation, push gateway/device configuration write, account-data mutation, push-rule write, keyword write, timed mute, global preference, sound/badge tuning, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notification_default_room_mode_summary_label(
    summary: &NotificationDefaultRoomModeSummary,
) -> String {
    let mode = telegram_notification_mode_action_label(summary.mode);
    let encryption = if summary.is_encrypted {
        "encrypted"
    } else {
        "unencrypted"
    };
    let room_class = if summary.is_one_to_one {
        "one-to-one"
    } else {
        "group"
    };
    format!(
        "{mode} default for {encryption} {room_class} rooms ({} active members in current room)",
        summary.active_members_count
    )
}

fn notifications_default_room_mode_live_read_requested_label(
    room_label: &str,
    control: &str,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let control = control.trim();
    let control = if control.is_empty() {
        "Defaults"
    } else {
        control
    };
    format!(
        "{control} live read submitted for {room_label}: {mode_state}; {loaded_attention}. MatrixRequest::GetDefaultRoomNotificationMode is waiting for NotificationSettings::get_default_room_notification_mode for the loaded room class; no default preference write, timed mute, pusher mutation, push gateway/device configuration, sound/badge tuning, room-state, membership, gateway/runtime/auth, or unrelated live mutation."
    )
}

fn notifications_default_room_mode_unavailable_label(
    room_label: &str,
    control: &str,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let control = control.trim();
    let control = if control.is_empty() {
        "Defaults"
    } else {
        control
    };
    format!(
        "{control} live read unavailable for {room_label}: timeline waiting; {mode_state}; {loaded_attention}. No MatrixRequest::GetDefaultRoomNotificationMode, no default preference write, timed mute, pusher mutation, push gateway/device configuration, room-state, membership, gateway/runtime/auth, or unrelated live mutation."
    )
}

fn notifications_default_room_mode_live_result_label(
    room_label: &str,
    result: &Result<NotificationDefaultRoomModeSummary, String>,
    mode_state: &str,
    loaded_attention: &str,
    local_status: Option<&str>,
) -> String {
    let local_status_chars = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| status.chars().count())
        .unwrap_or(0);
    match result {
        Ok(summary) => {
            let default_summary = notification_default_room_mode_summary_label(summary);
            format!(
                "Default notification mode live result for {room_label}: {default_summary}; {mode_state}; {loaded_attention}; prior local status {local_status_chars} chars. Read through NotificationSettings::get_default_room_notification_mode using the loaded room encryption and one-to-one class; no default preference write, pusher mutation, timed mute, sound/badge tuning, room-state, membership, gateway/runtime/auth, or unrelated live mutation."
            )
        }
        Err(error) => format!(
            "Default notification mode live read failed for {room_label}: {}; {mode_state}; {loaded_attention}; prior local status {local_status_chars} chars. No default preference write, pusher mutation, timed mute, room-state, membership, gateway/runtime/auth, or unrelated live mutation.",
            compact_message_preview(error, "unknown error")
        ),
    }
}

fn notifications_default_room_mode_write_confirmation_label(
    room_label: &str,
    mode: RoomNotificationMode,
) -> String {
    let mode_label = telegram_notification_mode_action_label(mode);
    format!(
        "Set default notification mode for {room_label} to {mode_label}? {NOTIFICATIONS_DEFAULT_ROOM_MODE_WRITE_LABEL} PositiveConfirmationModal gates MatrixRequest::SetDefaultRoomNotificationMode before NotificationSettings::set_default_room_notification_mode."
    )
}

fn notifications_default_room_mode_write_requested_label(
    room_label: &str,
    mode: RoomNotificationMode,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let mode_label = telegram_notification_mode_action_label(mode);
    format!(
        "Default {mode_label} confirmation opened for {room_label}: {mode_state}; {loaded_attention}. {NOTIFICATIONS_DEFAULT_ROOM_MODE_WRITE_LABEL} MatrixRequest::SetDefaultRoomNotificationMode waits for PositiveConfirmationModal accept before NotificationSettings::set_default_room_notification_mode for the loaded room class; no timed mute, pusher mutation, push gateway/device configuration, sound/badge tuning, room-state, membership, gateway/runtime/auth, or unrelated live mutation."
    )
}

fn notifications_default_room_mode_write_unavailable_label(
    room_label: &str,
    mode: RoomNotificationMode,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let mode_label = telegram_notification_mode_action_label(mode);
    format!(
        "Default {mode_label} write unavailable for {room_label}: timeline waiting; {mode_state}; {loaded_attention}. {NOTIFICATIONS_DEFAULT_ROOM_MODE_WRITE_LABEL} No MatrixRequest::SetDefaultRoomNotificationMode, timed mute, pusher mutation, push gateway/device configuration, room-state, membership, gateway/runtime/auth, or unrelated live mutation."
    )
}

fn notifications_default_room_mode_write_result_label(
    room_label: &str,
    mode: RoomNotificationMode,
    result: &Result<NotificationDefaultRoomModeSummary, String>,
    mode_state: &str,
    loaded_attention: &str,
) -> String {
    let mode_label = telegram_notification_mode_action_label(mode);
    match result {
        Ok(summary) => {
            let default_summary = notification_default_room_mode_summary_label(summary);
            format!(
                "Default notification mode write result for {room_label}: requested {mode_label}; {default_summary}; {mode_state}; {loaded_attention}. {NOTIFICATIONS_DEFAULT_ROOM_MODE_WRITE_LABEL} NotificationSettings::set_default_room_notification_mode succeeded and the SDK default room mode was read back; no timed mute, pusher mutation, push gateway/device configuration, sound/badge tuning, room-state, membership, gateway/runtime/auth, or unrelated live mutation was requested."
            )
        }
        Err(error) => format!(
            "Default {mode_label} write failed for {room_label}: {mode_state}; {loaded_attention}; error {}. {NOTIFICATIONS_DEFAULT_ROOM_MODE_WRITE_LABEL} Failed-state Retry reopens PositiveConfirmationModal before resubmitting SetDefaultRoomNotificationMode; no automatic retry, timed mute, pusher mutation, room-state, membership, gateway/runtime/auth, or unrelated live mutation.",
            compact_message_preview(error, "unknown error")
        ),
    }
}

fn notifications_default_room_mode_retry_confirmation_label(
    room_label: &str,
    mode: RoomNotificationMode,
) -> String {
    let mode_label = telegram_notification_mode_action_label(mode);
    format!(
        "Retry default notification mode update for {room_label}: {mode_label}? {NOTIFICATIONS_DEFAULT_ROOM_MODE_WRITE_LABEL} Failed-state Retry confirms before SetDefaultRoomNotificationMode; timed mute, pusher writes, sound/badge, room-state, membership, gateway/runtime/auth, and unrelated live mutation stay unwired."
    )
}

fn notifications_result_detail_control_label(
    room_label: &str,
    control: Option<&str>,
    mode_state: &str,
    loaded_attention: &str,
    requested_mode: Option<&str>,
    retry_cache_ready: bool,
    timeline_loaded: bool,
    local_status: Option<&str>,
) -> String {
    let control = control
        .map(str::trim)
        .filter(|control| !control.is_empty())
        .map(|control| format!("{control} detail selected"))
        .unwrap_or_else(|| "no notification result detail selected".to_string());
    let requested_state = requested_mode
        .filter(|mode| !mode.trim().is_empty())
        .map(|mode| format!("requested mode {mode}"))
        .unwrap_or_else(|| "no requested mode staged".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded"
    } else {
        "timeline waiting"
    };
    let local_status_chars = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| status.chars().count())
        .unwrap_or(0);
    format!(
        "Notification result detail for {room_label}: {control}; {mode_state}; {loaded_attention}; {requested_state}; {retry_state}; {timeline_state}; local status {local_status_chars} chars. {NOTIFICATIONS_RESULT_DETAIL_CONTROLS_LABEL} Result, Requested, Retry cache, Failure, and Source send no extra read, no unconfirmed SetRoomNotificationMode, no timed mute, global notification preference, keyword rule, push-rule, pusher mutation, push gateway/device configuration, sound/badge tuning, retry automation, cancel queue, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_preflight_detail_control_label(
    room_label: &str,
    control: Option<&str>,
    mode_state: &str,
    loaded_attention: &str,
    requested_mode: Option<&str>,
    retry_cache_ready: bool,
    timeline_loaded: bool,
    local_status: Option<&str>,
) -> String {
    let control = control
        .map(str::trim)
        .filter(|control| !control.is_empty())
        .map(|control| format!("{control} preflight selected"))
        .unwrap_or_else(|| "no notification preflight selected".to_string());
    let requested_state = requested_mode
        .filter(|mode| !mode.trim().is_empty())
        .map(|mode| format!("requested mode {mode}"))
        .unwrap_or_else(|| "no requested mode staged".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded for current mode"
    } else {
        "timeline waiting for current mode"
    };
    let local_status_chars = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| status.chars().count())
        .unwrap_or(0);
    format!(
        "Notification timed/global/pusher preflight for {room_label}: {control}; {mode_state}; {loaded_attention}; {requested_state}; {retry_state}; {timeline_state}; local status {local_status_chars} chars. {NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_LABEL} Keywords may submit a live read-only GetNotificationKeywordRules request; Pushers may submit a live read-only GetNotificationPusherStatus request; Defaults may submit a live read-only GetDefaultRoomNotificationMode request; Schedule, Packet, Contract, and Account data send no notification rule account-data write, no push-rule write beyond SDK reads, no pusher mutation, no push gateway/device configuration write, no timed mute write, no default preference write, no sound/badge tuning, no extra GetRoomNotificationMode, no unconfirmed SetRoomNotificationMode, no retry automation, cancel queue, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_schedule_local_snapshot_label(
    room_label: &str,
    mode_state: &str,
    loaded_attention: &str,
    requested_mode: Option<&str>,
    retry_cache_ready: bool,
    timeline_loaded: bool,
    local_status: Option<&str>,
) -> String {
    let requested_state = requested_mode
        .filter(|mode| !mode.trim().is_empty())
        .map(|mode| format!("requested mode {mode}"))
        .unwrap_or_else(|| "no requested mode staged".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded for current mode"
    } else {
        "timeline waiting for current mode"
    };
    let local_status_chars = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| status.chars().count())
        .unwrap_or(0);
    format!(
        "Local notification schedule snapshot for {room_label}: {mode_state}; {loaded_attention}; {requested_state}; {retry_state}; {timeline_state}; local status {local_status_chars} chars; timed mute window not selected; global schedule source not loaded. {NOTIFICATIONS_PREFLIGHT_DETAIL_CONTROLS_LABEL} Schedule renders this loaded local schedule packet only; it sends no notification rule account-data read or write, no push-rule write, no pusher mutation, no push gateway/device configuration, no timed mute write, no global notification preference write, no sound/badge tuning, no extra GetRoomNotificationMode, no unconfirmed SetRoomNotificationMode, no retry automation, cancel queue, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_rule_packet_payload(
    room_label: &str,
    mode_state: &str,
    loaded_attention: &str,
    requested_mode: Option<&str>,
    retry_cache_ready: bool,
    timeline_loaded: bool,
    local_status: Option<&str>,
) -> String {
    let room_label = room_label.trim();
    let room_label = if room_label.is_empty() {
        "this chat"
    } else {
        room_label
    };
    let mode_state = if mode_state.trim().is_empty() {
        "Current Matrix mode: unavailable"
    } else {
        mode_state.trim()
    };
    let attention = if loaded_attention.trim().is_empty() {
        "Loaded attention: unavailable"
    } else {
        loaded_attention.trim()
    };
    let requested_state = requested_mode
        .filter(|mode| !mode.trim().is_empty())
        .map(|mode| format!("requested mode {mode}"))
        .unwrap_or_else(|| "no requested mode staged".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded for current mode"
    } else {
        "timeline waiting for current mode"
    };
    let local_status = local_status
        .map(str::trim)
        .filter(|status| !status.is_empty())
        .unwrap_or("no local status staged");

    format!(
        "Notification rule packet\nRoom: {room_label}\nMode state: {mode_state}\nAttention: {attention}\nRequested: {requested_state}\nRetry: {retry_state}\nTimeline: {timeline_state}\nLocal status: {local_status}\nAcceptance matrix:\n- Room mode request/result/error/retry: only confirmed All/Mentions/Mute and failed-state Retry may submit SetRoomNotificationMode.\n- Timed mute request/result/error/retry: slot persisted; no timed mute write is wired.\n- Default room-mode read/write request/result/error/retry: Global/Defaults may read NotificationSettings::get_default_room_notification_mode; Default All/Mentions/Mute and failed-state Retry may submit confirmed SetDefaultRoomNotificationMode.\n- Keyword rules request/result/error/retry: SDK keyword Add/Remove owns the confirmed write path; no raw push-rule or account-data write is wired.\n- Pusher/device request/result/error/retry: Pushers/Device push may read Client::can_homeserver_push_encrypted_event_to_device; no pusher mutation or push gateway/device configuration is wired.\n- Sound/badge request/result/error/retry: slot persisted; no sound or badge tuning write is wired.\n- Promotion criteria: map remaining timed mute, raw account-data, pusher, sound/badge, and notification-result contracts before implementation.\nBoundary: {NOTIFICATIONS_RULE_PACKET_DRILLDOWN_LABEL} No Matrix notification rule account-data read or write outside SDK keyword/default APIs, no raw push-rule write beyond SDK keyword/default writes, no pusher mutation, no push gateway/device configuration, no timed mute write, no sound/badge tuning, no extra GetRoomNotificationMode, no unconfirmed SetRoomNotificationMode or SetDefaultRoomNotificationMode, no retry automation, cancel queue, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_rule_packet_clipboard_label(
    room_label: &str,
    copied: bool,
    mode_state: &str,
    loaded_attention: &str,
    requested_mode: Option<&str>,
    retry_cache_ready: bool,
    timeline_loaded: bool,
    local_status: Option<&str>,
) -> String {
    let copied_state = if copied {
        "packet copied"
    } else {
        "packet unavailable"
    };
    let requested_state = requested_mode
        .filter(|mode| !mode.trim().is_empty())
        .map(|mode| format!("requested mode {mode}"))
        .unwrap_or_else(|| "no requested mode staged".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded"
    } else {
        "timeline waiting"
    };
    let local_status_chars = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| status.chars().count())
        .unwrap_or(0);
    format!(
        "Notification rule packet for {room_label}: {copied_state}; {mode_state}; {loaded_attention}; {requested_state}; {retry_state}; {timeline_state}; local status {local_status_chars} chars. {NOTIFICATIONS_RULE_PACKET_DRILLDOWN_LABEL} No notification rule account-data read/write, push-rule write, pusher mutation, timed mute write, global preference write, sound/badge tuning, unconfirmed SetRoomNotificationMode, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_rule_contract_packet_payload(
    room_label: &str,
    mode_state: &str,
    loaded_attention: &str,
    requested_mode: Option<&str>,
    retry_cache_ready: bool,
    timeline_loaded: bool,
    local_status: Option<&str>,
) -> String {
    let room_label = room_label.trim();
    let room_label = if room_label.is_empty() {
        "this chat"
    } else {
        room_label
    };
    let mode_state = if mode_state.trim().is_empty() {
        "Current Matrix mode: unavailable"
    } else {
        mode_state.trim()
    };
    let attention = if loaded_attention.trim().is_empty() {
        "Loaded attention: unavailable"
    } else {
        loaded_attention.trim()
    };
    let requested_state = requested_mode
        .filter(|mode| !mode.trim().is_empty())
        .map(|mode| format!("requested mode {mode}"))
        .unwrap_or_else(|| "no requested mode staged".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded for current mode"
    } else {
        "timeline waiting for current mode"
    };
    let local_status = local_status
        .map(str::trim)
        .filter(|status| !status.is_empty())
        .unwrap_or("no local status staged");

    format!(
        "Notification account-data/pusher typed contract\nRoom: {room_label}\nMode state: {mode_state}\nAttention: {attention}\nRequested: {requested_state}\nRetry: {retry_state}\nTimeline: {timeline_state}\nLocal status: {local_status}\nContract matrix:\n- Room mode: confirmed All/Mentions/Mute SetRoomNotificationMode request/result/error/retry/source is live for current rooms.\n- Default room mode: confirmed Default All/Mentions/Mute SetDefaultRoomNotificationMode request/result/error/retry/source is live for the loaded room class.\n- Account-data rules: typed account-data read body, write body, result, error, retry, stale baseline, and source hash must exist before raw rule edits.\n- Push-rule keywords: typed keyword match rule, enable/disable result, ordering delta, error taxonomy, retry eligibility, and source snapshot support the current SDK keyword writes.\n- Pusher/device: typed pusher list, device target, enable/disable mutation, gateway/device push result, error taxonomy, and retry/source slots must exist before pusher changes.\n- Timed mute: typed global preference baseline, timed window, mute expiry, result/error/retry/source, and schedule reconciliation must exist before timed writes.\n- Sound/badge/result reconciliation: typed sound, badge, notification count reconciliation, stale requested-mode detection, failed confirmed write mapping, and loaded attention refresh criteria must exist before promotion.\n- Promotion blocker: coordinate remaining raw account-data, pusher, sound/badge, timed mute, and result contracts outside the UI lane before wiring them.\nBoundary: {NOTIFICATIONS_RULE_CONTRACT_PACKET_LABEL} No Matrix notification rule account-data read or write outside SDK keyword/default APIs, no pusher mutation, no push gateway/device configuration, no timed mute write, no sound/badge tuning, no extra GetRoomNotificationMode, no unconfirmed SetRoomNotificationMode or SetDefaultRoomNotificationMode, no retry automation, cancel queue, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_rule_contract_packet_clipboard_label(
    room_label: &str,
    copied: bool,
    mode_state: &str,
    loaded_attention: &str,
    requested_mode: Option<&str>,
    retry_cache_ready: bool,
    timeline_loaded: bool,
    local_status: Option<&str>,
) -> String {
    let copied_state = if copied {
        "contract copied"
    } else {
        "contract unavailable"
    };
    let requested_state = requested_mode
        .filter(|mode| !mode.trim().is_empty())
        .map(|mode| format!("requested mode {mode}"))
        .unwrap_or_else(|| "no requested mode staged".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded"
    } else {
        "timeline waiting"
    };
    let local_status_chars = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| status.chars().count())
        .unwrap_or(0);
    format!(
        "Notification rule contract for {room_label}: {copied_state}; {mode_state}; {loaded_attention}; {requested_state}; {retry_state}; {timeline_state}; local status {local_status_chars} chars. {NOTIFICATIONS_RULE_CONTRACT_PACKET_LABEL} No notification account-data read/write, push-rule write, pusher mutation, push gateway/device configuration, timed mute write, global preference write, sound/badge tuning, unconfirmed SetRoomNotificationMode, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_result_taxonomy_packet_payload(
    room_label: &str,
    mode_state: &str,
    loaded_attention: &str,
    requested_mode: Option<&str>,
    retry_cache_ready: bool,
    timeline_loaded: bool,
    local_status: Option<&str>,
) -> String {
    let room_label = room_label.trim();
    let room_label = if room_label.is_empty() {
        "this chat"
    } else {
        room_label
    };
    let mode_state = if mode_state.trim().is_empty() {
        "Current Matrix mode: unavailable"
    } else {
        mode_state.trim()
    };
    let attention = if loaded_attention.trim().is_empty() {
        "Loaded attention: unavailable"
    } else {
        loaded_attention.trim()
    };
    let requested_state = requested_mode
        .filter(|mode| !mode.trim().is_empty())
        .map(|mode| format!("requested mode {mode}"))
        .unwrap_or_else(|| "no requested mode staged".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded for current mode"
    } else {
        "timeline waiting for current mode"
    };
    let local_status = local_status
        .map(str::trim)
        .filter(|status| !status.is_empty())
        .unwrap_or("no local status staged");

    format!(
        "Notification timed/global/pusher result taxonomy packet\nRoom: {room_label}\nMode state: {mode_state}\nAttention: {attention}\nRequested: {requested_state}\nRetry: {retry_state}\nTimeline: {timeline_state}\nLocal status: {local_status}\nLive result references:\n- Room mode: existing confirmed MatrixRequest::SetRoomNotificationMode result path only.\n- Keyword rules: existing confirmed MatrixRequest::SetNotificationKeywordRule result path only.\n- Default mode: existing confirmed MatrixRequest::SetDefaultRoomNotificationMode result path only.\nBlocked result slots:\n- timed_mute_operation_id: not_assigned\n- timed_mute_result: scheduled, applied, expired, failed, stale not_wired\n- raw_account_data_operation_id: not_assigned\n- raw_account_data_result: applied, failed, stale not_wired\n- pusher_device_operation_id: not_assigned\n- pusher_device_result: enabled, disabled, failed, stale not_wired\n- sound_badge_operation_id: not_assigned\n- sound_badge_result: applied, failed, stale not_wired\n- retry_policy: confirmation_required_backend_request_id_required\n- cancel_policy: local_dismiss_no_request\n- source_hash_policy: backend_generation_required_before_raw_rule_or_pusher_write\n- audit_redaction: no pushkey, token, gateway secret, or raw pusher payload in local packet\nBoundary: {NOTIFICATIONS_RESULT_TAXONOMY_PACKET_LABEL} No Matrix notification rule account-data read or write outside SDK keyword/default APIs, no pusher mutation, no push gateway/device configuration, no timed mute write, no sound/badge tuning, no extra GetRoomNotificationMode, no unconfirmed SetRoomNotificationMode or SetDefaultRoomNotificationMode, no retry automation, cancel queue, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

fn notifications_result_taxonomy_packet_clipboard_label(
    room_label: &str,
    copied: bool,
    mode_state: &str,
    loaded_attention: &str,
    requested_mode: Option<&str>,
    retry_cache_ready: bool,
    timeline_loaded: bool,
    local_status: Option<&str>,
) -> String {
    let copied_state = if copied {
        "taxonomy copied"
    } else {
        "taxonomy unavailable"
    };
    let requested_state = requested_mode
        .filter(|mode| !mode.trim().is_empty())
        .map(|mode| format!("requested mode {mode}"))
        .unwrap_or_else(|| "no requested mode staged".to_string());
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded"
    } else {
        "timeline waiting"
    };
    let local_status_chars = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| status.chars().count())
        .unwrap_or(0);
    format!(
        "Notification result taxonomy for {room_label}: {copied_state}; {mode_state}; {loaded_attention}; {requested_state}; {retry_state}; {timeline_state}; local status {local_status_chars} chars. {NOTIFICATIONS_RESULT_TAXONOMY_PACKET_LABEL} No notification account-data read/write, pusher mutation, push gateway/device configuration, timed mute write, sound/badge tuning, unconfirmed SetRoomNotificationMode, gateway/runtime/auth, or live mutation."
    )
}

pub(crate) fn notifications_mode_target_metadata_label(
    room_label: &str,
    current_mode_state: &str,
    requested_mode: Option<&str>,
    attention_loaded: bool,
    retry_cache_ready: bool,
    timeline_loaded: bool,
    local_status: Option<&str>,
) -> String {
    let requested_state = requested_mode
        .filter(|mode| !mode.trim().is_empty())
        .map(|mode| format!("requested mode {mode}"))
        .unwrap_or_else(|| "no requested mode staged".to_string());
    let attention_state = if attention_loaded {
        "loaded attention ready"
    } else {
        "loaded attention waiting"
    };
    let retry_state = if retry_cache_ready {
        "retry cache ready"
    } else {
        "retry cache empty"
    };
    let timeline_state = if timeline_loaded {
        "timeline loaded"
    } else {
        "timeline waiting"
    };
    let local_state = local_status
        .filter(|status| !status.trim().is_empty())
        .map(|status| format!("local status {status}"))
        .unwrap_or_else(|| "no local status staged".to_string());

    format!(
        "Notification target metadata for {room_label}: {current_mode_state}; {requested_state}; {attention_state}; {retry_state}; {timeline_state}; {local_state}. {NOTIFICATIONS_MODE_TARGET_METADATA_LABEL}"
    )
}

fn notifications_retry_confirmation_label(room_label: &str, mode_label: &str) -> String {
    format!(
        "Retry notification mode update for {room_label}: {mode_label}? {NOTIFICATIONS_RETRY_CONFIRMATION_LABEL} No timed mute, global preference, keyword rule, push gateway/device, pusher, cancel queue, room-state, membership, gateway/runtime/auth, or live mutation."
    )
}

#[cfg(test)]
mod room_settings_close_metadata_tests {
    use super::*;

    #[test]
    fn room_settings_name_id_clipboard_payload_is_loaded_local_only() {
        let payload =
            room_settings_name_id_clipboard_payload("Hepta Runtime Cockpit", "!roomid:example.org")
                .expect("loaded room id should produce a clipboard payload");

        assert!(payload.contains("Room settings name/id"));
        assert!(payload.contains("Name: Hepta Runtime Cockpit"));
        assert!(payload.contains("Room ID: !roomid:example.org"));
        assert!(payload.contains("loaded RoomScreen RoomNameId"));
        assert!(payload.contains("no m.room.name"));
        assert!(ROOM_SETTINGS_NAME_ID_CLIPBOARD_EVIDENCE.contains("local clipboard"));
        assert!(ROOM_SETTINGS_NAME_ID_CLIPBOARD_EVIDENCE.contains("RoomNameId"));
        assert!(ROOM_SETTINGS_NAME_ID_CLIPBOARD_EVIDENCE.contains("gateway/runtime/auth"));
    }

    #[test]
    fn room_settings_name_id_clipboard_payload_requires_room_id() {
        assert!(room_settings_name_id_clipboard_payload("Hepta", "   ").is_none());
        let payload = room_settings_name_id_clipboard_payload("", "!roomid:example.org")
            .expect("empty labels fall back to this chat");
        assert!(payload.contains("Name: this chat"));
    }

    #[test]
    fn room_settings_name_id_clipboard_label_reports_copy_state() {
        let copied = room_settings_name_id_clipboard_label(
            true,
            "Hepta Runtime Cockpit",
            "!roomid:example.org",
            true,
            Some(8),
            true,
        );
        let unavailable = room_settings_name_id_clipboard_label(false, "", "", false, None, false);

        assert!(copied.contains("copied loaded room name/id to local clipboard"));
        assert!(copied.contains("room label `Hepta Runtime Cockpit`"));
        assert!(copied.contains("room id `!roomid:example.org`"));
        assert!(copied.contains("8 cached member(s)"));
        assert!(copied.contains(ROOM_SETTINGS_NAME_ID_CLIPBOARD_LABEL));
        assert!(copied.contains("No m.room.name"));
        assert!(unavailable.contains("name/id clipboard unavailable"));
        assert!(unavailable.contains("room id waiting"));
        assert!(unavailable.contains("member cache waiting"));
    }

    #[test]
    fn room_settings_permissions_clipboard_payload_is_loaded_local_only() {
        let payload = room_settings_permissions_clipboard_payload(true, false, true);

        assert!(payload.contains("Room settings permissions"));
        assert!(payload.contains("Send messages: allowed"));
        assert!(payload.contains("Send reactions: blocked"));
        assert!(payload.contains("Notify @room: allowed"));
        assert!(payload.contains("loaded RoomScreen tl_state.user_power"));
        assert!(payload.contains("no m.room.power_levels write"));
        assert!(ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_EVIDENCE.contains("GetRoomPowerLevels"));
        assert!(ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_EVIDENCE.contains("tl_state.user_power"));
        assert!(ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_EVIDENCE.contains("local clipboard"));
        assert!(ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_EVIDENCE.contains("m.room.power_levels"));
        assert!(ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_EVIDENCE.contains("gateway/runtime/auth"));
    }

    #[test]
    fn room_settings_permissions_clipboard_label_reports_copy_state() {
        let copied = room_settings_permissions_clipboard_label(
            true,
            "Hepta Runtime Cockpit",
            Some((true, false, true)),
            true,
            Some(8),
        );
        let unavailable = room_settings_permissions_clipboard_label(false, "", None, false, None);

        assert!(copied.contains("copied loaded permission summary to local clipboard"));
        assert!(copied.contains("room label `Hepta Runtime Cockpit`"));
        assert!(copied.contains("send allowed; react blocked; @room allowed"));
        assert!(copied.contains("loaded identity metadata available"));
        assert!(copied.contains("8 cached member(s)"));
        assert!(copied.contains(ROOM_SETTINGS_PERMISSIONS_CLIPBOARD_LABEL));
        assert!(copied.contains("No m.room.power_levels"));
        assert!(unavailable.contains("permissions clipboard unavailable"));
        assert!(unavailable.contains("power levels waiting"));
        assert!(unavailable.contains("member cache waiting"));
    }

    #[test]
    fn room_settings_members_clipboard_payload_is_loaded_local_only() {
        let payload = room_settings_members_clipboard_payload(
            3,
            "Alice <@alice:example.org>; Bob <@bob:example.org>",
        );

        assert!(payload.contains("Room settings members"));
        assert!(payload.contains("Loaded members: 3"));
        assert!(payload.contains("Alice <@alice:example.org>"));
        assert!(payload.contains("loaded RoomScreen room_members local cache"));
        assert!(payload.contains("no membership write"));
        assert!(ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE.contains("room_members cache"));
        assert!(ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE.contains("GetRoomMembers(server-backed"));
        assert!(ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE.contains("local clipboard"));
        assert!(ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE.contains("m.room.member"));
        assert!(ROOM_SETTINGS_MEMBERS_CLIPBOARD_EVIDENCE.contains("gateway/runtime/auth"));
    }

    #[test]
    fn room_settings_members_clipboard_label_reports_copy_state() {
        let copied = room_settings_members_clipboard_label(
            true,
            "Hepta Runtime Cockpit",
            Some(8),
            "Alice <@alice:example.org>; Bob <@bob:example.org>",
            true,
            true,
        );
        let unavailable = room_settings_members_clipboard_label(false, "", None, "", false, false);

        assert!(copied.contains("copied loaded member-cache summary to local clipboard"));
        assert!(copied.contains("room label `Hepta Runtime Cockpit`"));
        assert!(copied.contains("8 cached member(s)"));
        assert!(copied.contains("Alice <@alice:example.org>"));
        assert!(copied.contains("loaded identity metadata available"));
        assert!(copied.contains("power levels loaded"));
        assert!(copied.contains(ROOM_SETTINGS_MEMBERS_CLIPBOARD_LABEL));
        assert!(copied.contains("No membership write"));
        assert!(unavailable.contains("members clipboard unavailable"));
        assert!(unavailable.contains("member cache waiting"));
        assert!(unavailable.contains("power levels waiting"));
    }

    #[test]
    fn room_settings_identity_clipboard_payload_is_loaded_local_only() {
        let payload = room_settings_identity_clipboard_payload(
            "Hepta Runtime Cockpit",
            "!roomid:example.org",
            Some("#hepta:example.org"),
            2,
            true,
            false,
            Some(8),
        )
        .expect("loaded room identity should produce a clipboard payload");

        assert!(payload.contains("Room settings identity"));
        assert!(payload.contains("Name: Hepta Runtime Cockpit"));
        assert!(payload.contains("Room ID: !roomid:example.org"));
        assert!(payload.contains("Canonical alias: #hepta:example.org"));
        assert!(payload.contains("Alternative aliases: 2"));
        assert!(payload.contains("avatar image cached"));
        assert!(payload.contains("not tombstoned"));
        assert!(payload.contains("8 loaded member(s)"));
        assert!(payload.contains("loaded RoomsList RoomContextMenuDetails"));
        assert!(payload.contains("no m.room.name"));
        assert!(ROOM_SETTINGS_IDENTITY_CLIPBOARD_EVIDENCE.contains("RoomContextMenuDetails"));
        assert!(ROOM_SETTINGS_IDENTITY_CLIPBOARD_EVIDENCE.contains("local clipboard"));
        assert!(ROOM_SETTINGS_IDENTITY_CLIPBOARD_EVIDENCE.contains("m.room.canonical_alias"));
        assert!(ROOM_SETTINGS_IDENTITY_CLIPBOARD_EVIDENCE.contains("gateway/runtime/auth"));
    }

    #[test]
    fn room_settings_identity_clipboard_payload_requires_room_id() {
        assert!(
            room_settings_identity_clipboard_payload("Hepta", "   ", None, 0, false, false, None)
                .is_none()
        );
        let payload = room_settings_identity_clipboard_payload(
            "",
            "!roomid:example.org",
            None,
            0,
            false,
            true,
            None,
        )
        .expect("empty labels fall back to this chat");
        assert!(payload.contains("Name: this chat"));
        assert!(payload.contains("Canonical alias: none loaded"));
        assert!(payload.contains("tombstoned"));
        assert!(payload.contains("member cache waiting"));
    }

    #[test]
    fn room_settings_identity_clipboard_label_reports_copy_state() {
        let copied = room_settings_identity_clipboard_label(
            true,
            "Hepta Runtime Cockpit",
            "!roomid:example.org",
            Some("#hepta:example.org"),
            Some(2),
            Some(true),
            Some(false),
            Some(8),
        );
        let unavailable =
            room_settings_identity_clipboard_label(false, "", "", None, None, None, None, None);

        assert!(copied.contains("copied loaded identity metadata to local clipboard"));
        assert!(copied.contains("room label `Hepta Runtime Cockpit`"));
        assert!(copied.contains("room id `!roomid:example.org`"));
        assert!(copied.contains("canonical alias `#hepta:example.org`"));
        assert!(copied.contains("2 alternative alias(es)"));
        assert!(copied.contains("avatar image cached"));
        assert!(copied.contains("not tombstoned"));
        assert!(copied.contains("8 cached member(s)"));
        assert!(copied.contains(ROOM_SETTINGS_IDENTITY_CLIPBOARD_LABEL));
        assert!(copied.contains("No m.room.name"));
        assert!(unavailable.contains("identity clipboard unavailable"));
        assert!(unavailable.contains("room id waiting"));
        assert!(unavailable.contains("alternative aliases waiting"));
        assert!(unavailable.contains("member cache waiting"));
    }

    #[test]
    fn room_settings_close_metadata_label_summarizes_loaded_state() {
        let label = room_settings_close_metadata_label("Hepta UI", true, true, Some(12), true);

        assert!(label.contains("Room settings closed for Hepta UI"));
        assert!(label.contains("last option preview retained"));
        assert!(label.contains("loaded identity metadata ready"));
        assert!(label.contains("12 members loaded"));
        assert!(label.contains("power levels loaded"));
        assert!(label.contains("local strip hidden only"));
        assert!(label.contains(ROOM_SETTINGS_CLOSE_METADATA_LABEL));
    }

    #[test]
    fn room_settings_close_metadata_label_reports_waiting_state() {
        let label = room_settings_close_metadata_label("this chat", false, false, None, false);

        assert!(label.contains("no option preview staged"));
        assert!(label.contains("loaded identity metadata waiting"));
        assert!(label.contains("members waiting"));
        assert!(label.contains("power levels waiting"));
        assert!(label.contains("no Matrix room-state request"));
    }

    #[test]
    fn room_settings_refresh_metadata_label_summarizes_read_refresh() {
        let label = room_settings_refresh_metadata_label("Hepta UI", true, true, Some(7), true);

        assert!(label.contains("Room settings refresh for Hepta UI"));
        assert!(label.contains("timeline ready for read refresh"));
        assert!(label.contains("loaded identity metadata ready"));
        assert!(label.contains("7 cached members before refresh"));
        assert!(label.contains("power-level display ready"));
        assert!(label.contains("live read refresh submits GetRoomPowerLevels plus GetRoomMembers"));
        assert!(label.contains(ROOM_SETTINGS_REFRESH_METADATA_LABEL));
    }

    #[test]
    fn room_settings_refresh_metadata_label_keeps_waiting_state_local() {
        let label = room_settings_refresh_metadata_label("this chat", false, false, None, false);

        assert!(label.contains("timeline waiting for read refresh"));
        assert!(label.contains("loaded identity metadata waiting"));
        assert!(label.contains("cached members waiting before refresh"));
        assert!(label.contains("power-level display waiting"));
        assert!(label.contains("live read refresh waits for a loaded timeline"));
        assert!(label.contains(
            "Name/Topic/avatar/alias/history/join-rule/tombstone writes use confirmed live room-state path"
        ));
    }

    #[test]
    fn room_settings_refresh_live_read_wiring_label_reports_partial_live_state() {
        let ready = room_settings_refresh_live_read_wiring_label(true, Some(3), true);
        let waiting = room_settings_refresh_live_read_wiring_label(false, None, false);

        assert!(ready.contains("GetRoomPowerLevels"));
        assert!(ready.contains("GetRoomMembers(local_only=false, JOIN)"));
        assert!(ready.contains("3 cached member(s) before refresh"));
        assert!(ready.contains("power-level baseline present"));
        assert!(ready.contains("editable room-state writes remain blocked"));
        assert!(waiting.contains("waits for a loaded timeline"));
        assert!(waiting.contains("member cache pending"));
        assert!(waiting.contains("power-level baseline pending"));
        assert!(
            ROOM_SETTINGS_REFRESH_LIVE_READ_WIRING_EVIDENCE
                .contains("TimelineUpdate::UserPowerLevels")
        );
        assert!(ROOM_SETTINGS_REFRESH_LIVE_READ_WIRING_EVIDENCE.contains("RoomMembersListFetched"));
    }

    #[test]
    fn room_settings_edit_controls_boundary_label_lists_blocked_controls() {
        let label = room_settings_edit_controls_boundary_label("Hepta UI", true, Some(12), true);

        assert!(label.contains("Room settings edit-controls boundary for Hepta UI"));
        assert!(label.contains("loaded identity metadata ready"));
        assert!(label.contains("12 cached members"));
        assert!(label.contains("power-level display ready"));
        assert!(label.contains(ROOM_SETTINGS_EDIT_CONTROLS_BOUNDARY_LABEL));
        assert!(label.contains("m.room.name"));
        assert!(label.contains("m.room.topic"));
        assert!(label.contains("m.room.avatar"));
        assert!(label.contains("m.room.history_visibility"));
        assert!(label.contains("m.room.join_rules"));
        assert!(label.contains("m.room.power_levels"));
        assert!(label.contains("membership moderation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn room_settings_tombstone_live_write_labels_confirm_validate_and_retry() {
        let replacement = "!replacement:example.org";
        let confirmation = room_settings_live_write_confirmation_label(
            "Hepta UI",
            RoomSettingsMutationField::Tombstone,
            replacement,
        );
        let validation = room_settings_tombstone_replacement_validation_label(
            "Hepta UI",
            "not-a-room-id",
            "invalid room id",
        );
        let sent = room_settings_live_write_result_label(
            "Hepta UI",
            RoomSettingsMutationField::Tombstone,
            replacement,
            &Ok(()),
        );
        let failed = room_settings_live_write_result_label(
            "Hepta UI",
            RoomSettingsMutationField::Tombstone,
            replacement,
            &Err("M_FORBIDDEN".to_string()),
        );

        assert!(confirmation.contains("Mark Hepta UI as replaced"));
        assert!(confirmation.contains("m.room.tombstone"));
        assert!(confirmation.contains("PositiveConfirmationModal"));
        assert!(
            validation.contains("replacement room `not-a-room-id` is not a valid Matrix room id")
        );
        assert!(validation.contains("No m.room.tombstone"));
        assert!(sent.contains("Tombstone update sent"));
        assert!(sent.contains("replacement `!replacement:example.org` accepted by SDK"));
        assert!(failed.contains("Retry is confirmation-gated"));
        assert_eq!(
            room_settings_tombstone_body("Hepta UI", replacement),
            "Hepta UI has been replaced by !replacement:example.org."
        );
        assert!(
            ROOM_SETTINGS_NAME_TOPIC_LIVE_WRITE_EVIDENCE
                .contains("MatrixRequest::SetRoomTombstone")
        );
    }

    #[test]
    fn room_settings_edit_controls_boundary_label_keeps_waiting_state_local() {
        let label = room_settings_edit_controls_boundary_label("this chat", false, None, false);

        assert!(label.contains("loaded identity metadata waiting"));
        assert!(label.contains("cached members waiting"));
        assert!(label.contains("power-level display waiting"));
        assert!(label.contains("No m.room.name"));
    }

    #[test]
    fn room_settings_edit_intent_staging_label_lists_blocked_writes() {
        let label =
            room_settings_edit_intent_staging_label("Hepta UI", "Avatar", true, Some(12), true);

        assert!(label.contains("Room settings edit intent staged locally for Hepta UI"));
        assert!(label.contains("Avatar"));
        assert!(label.contains("loaded identity metadata ready"));
        assert!(label.contains("12 cached members"));
        assert!(label.contains("power-level display ready"));
        assert!(label.contains(ROOM_SETTINGS_EDIT_INTENT_STAGING_LABEL));
        assert!(label.contains("m.room.avatar"));
        assert!(label.contains("m.room.canonical_alias"));
        assert!(label.contains("m.room.power_levels"));
        assert!(label.contains("member moderation"));
        assert!(label.contains("notification-rule handoff"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn room_settings_edit_intent_staging_label_uses_safe_fallbacks() {
        let label = room_settings_edit_intent_staging_label("this chat", "   ", false, None, false);

        assert!(label.contains("room-state edit"));
        assert!(label.contains("loaded identity metadata waiting"));
        assert!(label.contains("cached members waiting"));
        assert!(label.contains("power-level display waiting"));
        assert!(label.contains("No m.room.name"));
    }

    #[test]
    fn room_settings_field_edit_intent_controls_label_lists_visible_fields() {
        let label = room_settings_field_edit_intent_controls_label(
            "Hepta UI",
            "Name edit",
            true,
            Some(12),
            true,
        );

        assert!(label.contains("Room settings field edit intent staged locally for Hepta UI"));
        assert!(label.contains("Name edit"));
        assert!(label.contains("loaded identity metadata ready"));
        assert!(label.contains("12 cached members"));
        assert!(label.contains("power-level display ready"));
        assert!(label.contains(ROOM_SETTINGS_FIELD_EDIT_INTENT_CONTROLS_LABEL));
        assert!(label.contains("m.room.name"));
        assert!(label.contains("m.room.topic"));
        assert!(label.contains("m.room.avatar"));
        assert!(label.contains("m.room.power_levels"));
        assert!(label.contains("membership list write"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn room_settings_field_edit_intent_controls_label_uses_safe_fallbacks() {
        let label =
            room_settings_field_edit_intent_controls_label("this chat", "   ", false, None, false);

        assert!(label.contains("room field"));
        assert!(label.contains("loaded identity metadata waiting"));
        assert!(label.contains("cached members waiting"));
        assert!(label.contains("power-level display waiting"));
        assert!(label.contains("No m.room.name"));
    }

    #[test]
    fn room_settings_refresh_result_detail_label_summarizes_loaded_results() {
        let label = room_settings_refresh_result_detail_label(
            "Hepta UI",
            Some("Members"),
            true,
            true,
            Some(18),
            "power result send allowed, react allowed, @room blocked",
            144,
        );

        assert!(label.contains("Room settings refresh result detail for Hepta UI"));
        assert!(label.contains("Members detail selected"));
        assert!(label.contains("timeline ready"));
        assert!(label.contains("loaded identity metadata ready"));
        assert!(label.contains("18 cached members"));
        assert!(label.contains("power result send allowed"));
        assert!(label.contains("local status 144 chars"));
        assert!(label.contains(ROOM_SETTINGS_REFRESH_RESULT_DETAIL_LABEL));
        assert!(label.contains("GetRoomPowerLevels"));
        assert!(label.contains("GetRoomMembers(server-backed)"));
        assert!(label.contains("m.room.name"));
        assert!(label.contains("m.room.topic"));
        assert!(label.contains("m.room.avatar"));
        assert!(label.contains("m.room.power_levels"));
        assert!(label.contains("membership list write"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn room_settings_refresh_result_detail_label_uses_waiting_fallbacks() {
        let label = room_settings_refresh_result_detail_label(
            "this chat",
            Some("   "),
            false,
            false,
            None,
            "   ",
            0,
        );

        assert!(label.contains("no refresh result detail selected"));
        assert!(label.contains("timeline waiting"));
        assert!(label.contains("loaded identity metadata waiting"));
        assert!(label.contains("cached members waiting"));
        assert!(label.contains("power result waiting"));
        assert!(label.contains("local status 0 chars"));
        assert!(label.contains("no extra read"));
    }

    #[test]
    fn room_settings_mutation_preflight_detail_label_summarizes_blocked_writes() {
        let label = room_settings_mutation_preflight_detail_label(
            "Hepta UI",
            Some("Request"),
            true,
            true,
            Some(21),
            "power result send allowed, react blocked, @room allowed",
            188,
        );

        assert!(label.contains("Room settings room-state mutation preflight for Hepta UI"));
        assert!(label.contains("Request preflight selected"));
        assert!(label.contains("timeline ready"));
        assert!(label.contains("loaded identity metadata ready"));
        assert!(label.contains("21 cached members"));
        assert!(label.contains("power result send allowed"));
        assert!(label.contains("local status 188 chars"));
        assert!(label.contains(ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_LABEL));
        assert!(label.contains("m.room.name"));
        assert!(label.contains("m.room.topic"));
        assert!(label.contains("m.room.avatar"));
        assert!(label.contains("m.room.canonical_alias"));
        assert!(label.contains("m.room.history_visibility"));
        assert!(label.contains("m.room.join_rules"));
        assert!(label.contains("m.room.power_levels"));
        assert!(label.contains("membership list write"));
        assert!(label.contains("retry automation"));
        assert!(label.contains("room-state mutation contract call"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(
            ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_EVIDENCE
                .contains("local room-state mutation packet snapshot")
        );
    }

    #[test]
    fn room_settings_mutation_preflight_detail_label_uses_waiting_fallbacks() {
        let label = room_settings_mutation_preflight_detail_label(
            "this chat",
            Some("   "),
            false,
            false,
            None,
            "   ",
            0,
        );

        assert!(label.contains("no mutation preflight selected"));
        assert!(label.contains("timeline waiting"));
        assert!(label.contains("loaded identity metadata waiting"));
        assert!(label.contains("cached members waiting"));
        assert!(label.contains("power result waiting"));
        assert!(label.contains("local status 0 chars"));
        assert!(label.contains("submit no m.room.name"));
    }

    #[test]
    fn room_settings_mutation_request_packet_snapshot_label_summarizes_local_packet() {
        let label = room_settings_mutation_request_packet_snapshot_label(
            "Hepta UI",
            true,
            true,
            Some(21),
            "power result send allowed, react blocked, @room allowed",
            188,
        );

        assert!(label.contains("Local room-state mutation packet snapshot"));
        assert!(label.contains("Request selected"));
        assert!(label.contains("timeline ready"));
        assert!(label.contains("loaded identity metadata ready"));
        assert!(label.contains("21 cached members"));
        assert!(label.contains("power result send allowed"));
        assert!(label.contains("local status 188 chars"));
        assert!(label.contains("Proposed request body"));
        assert!(label.contains("result slot"));
        assert!(label.contains("retry eligibility"));
        assert!(label.contains("m.room.name/topic/avatar"));
        assert!(label.contains("membership list scope"));
        assert!(label.contains("typed room-settings mutation contract target"));
        assert!(label.contains("No m.room.name"));
        assert!(label.contains("m.room.power_levels"));
        assert!(label.contains("room-state mutation contract call"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
        assert!(label.contains(ROOM_SETTINGS_MUTATION_PREFLIGHT_DETAIL_CONTROLS_LABEL));
    }

    #[test]
    fn room_settings_field_mutation_packet_payload_lists_field_acceptance() {
        let payload = room_settings_field_mutation_packet_payload(
            "Hepta UI",
            true,
            true,
            Some(21),
            "power result send allowed, react blocked, @room allowed",
            "Name edit staged",
            "Request",
        );

        assert!(payload.contains("Room settings field mutation packet"));
        assert!(payload.contains("Timeline: timeline ready"));
        assert!(payload.contains("Identity: loaded identity metadata ready"));
        assert!(payload.contains("21 cached members"));
        assert!(payload.contains("Name edit staged"));
        assert!(payload.contains("Last preflight: Request"));
        assert!(payload.contains("m.room.name body"));
        assert!(payload.contains("m.room.topic body"));
        assert!(payload.contains("m.room.avatar body"));
        assert!(payload.contains("canonical alias and alternative aliases"));
        assert!(payload.contains("m.room.history_visibility"));
        assert!(payload.contains("m.room.join_rules"));
        assert!(payload.contains("m.room.power_levels"));
        assert!(payload.contains("invite/kick/ban/knock/moderation"));
        assert!(payload.contains("Tombstone:"));
        assert!(payload.contains("typed room-settings mutation contract"));
        assert!(payload.contains("No m.room.name"));
        assert!(payload.contains("retry automation"));
        assert!(payload.contains("room-state mutation contract call"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
        assert!(payload.contains(ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_LABEL));
    }

    #[test]
    fn room_settings_field_mutation_packet_clipboard_label_is_local_only() {
        let label = room_settings_field_mutation_packet_clipboard_label(
            true,
            "Hepta UI",
            true,
            true,
            Some(21),
            "power result send allowed, react blocked, @room allowed",
            188,
        );

        assert!(label.contains("field mutation packet for Hepta UI"));
        assert!(label.contains("copied field-by-field mutation packet to local clipboard"));
        assert!(label.contains("timeline ready"));
        assert!(label.contains("loaded identity metadata ready"));
        assert!(label.contains("21 cached members"));
        assert!(label.contains("power result send allowed"));
        assert!(label.contains("local status 188 chars"));
        assert!(label.contains(ROOM_SETTINGS_FIELD_MUTATION_PACKET_DRILLDOWN_LABEL));
        assert!(label.contains("No m.room.name/topic/avatar"));
        assert!(label.contains("membership write"));
        assert!(label.contains("retry automation"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn room_settings_field_mutation_contract_packet_payload_lists_typed_contracts() {
        let payload = room_settings_field_mutation_contract_packet_payload(
            "Hepta UI",
            true,
            true,
            Some(21),
            "power result send allowed, react blocked, @room allowed",
            "Field packet copied",
            "Packet",
        );

        assert!(payload.contains("Room settings typed room-state mutation/result contract"));
        assert!(payload.contains("Timeline: timeline ready"));
        assert!(payload.contains("Identity: loaded identity metadata ready"));
        assert!(payload.contains("21 cached members"));
        assert!(payload.contains("Field packet copied"));
        assert!(payload.contains("Last preflight: Packet"));
        assert!(payload.contains("Baseline identity"));
        assert!(payload.contains("Name contract"));
        assert!(payload.contains("m.room.name request body"));
        assert!(payload.contains("Topic contract"));
        assert!(payload.contains("m.room.topic request body"));
        assert!(payload.contains("Avatar contract"));
        assert!(payload.contains("m.room.avatar request body"));
        assert!(payload.contains("Alias contract"));
        assert!(payload.contains("Visibility and join-rule contracts"));
        assert!(payload.contains("Power-level contract"));
        assert!(payload.contains("m.room.power_levels"));
        assert!(payload.contains("Member moderation contract"));
        assert!(payload.contains("Tombstone contract"));
        assert!(payload.contains("RoomTombstoneEventContent"));
        assert!(payload.contains("typed notification rule contract"));
        assert!(payload.contains("typed room-state mutation/result contracts"));
        assert!(payload.contains(ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_LABEL));
        assert!(payload.contains("No m.room.name"));
        assert!(payload.contains("retry automation"));
        assert!(payload.contains("room-state mutation contract call"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("live mutation"));
    }

    #[test]
    fn room_settings_field_mutation_contract_packet_clipboard_label_is_local_only() {
        let label = room_settings_field_mutation_contract_packet_clipboard_label(
            true,
            "Hepta UI",
            true,
            true,
            Some(21),
            "power result send allowed, react blocked, @room allowed",
            188,
        );

        assert!(label.contains("typed contract for Hepta UI"));
        assert!(label.contains("contract copied"));
        assert!(label.contains("timeline ready"));
        assert!(label.contains("loaded identity metadata ready"));
        assert!(label.contains("21 cached members"));
        assert!(label.contains("power result send allowed"));
        assert!(label.contains("local status 188 chars"));
        assert!(label.contains(ROOM_SETTINGS_FIELD_MUTATION_CONTRACT_PACKET_LABEL));
        assert!(label.contains("No m.room.name/topic/avatar"));
        assert!(label.contains("membership write"));
        assert!(label.contains("retry automation"));
        assert!(label.contains("room-state mutation contract call"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("live mutation"));
    }

    #[test]
    fn room_settings_power_member_result_taxonomy_packet_payload_lists_blocked_slots() {
        let payload = room_settings_power_member_result_taxonomy_packet_payload(
            "Hepta UI",
            true,
            true,
            Some(21),
            "power result send allowed, react blocked, @room allowed",
            "Contract copied",
            "Contract",
        );

        assert!(payload.contains("Room settings power/member result taxonomy packet"));
        assert!(payload.contains("Timeline: timeline ready"));
        assert!(payload.contains("Identity: loaded identity metadata ready"));
        assert!(payload.contains("21 cached members"));
        assert!(payload.contains("Contract copied"));
        assert!(payload.contains("Last preflight: Contract"));
        assert!(payload.contains("Existing room-state writes"));
        assert!(payload.contains("Refresh reads"));
        assert!(payload.contains("power_levels_operation_id: not_assigned"));
        assert!(payload.contains("power_levels_result: applied, permission_denied"));
        assert!(payload.contains("member_moderation_operation_id: not_assigned"));
        assert!(payload.contains("member_moderation_target_slot"));
        assert!(payload.contains("invite_result: accepted, permission_denied"));
        assert!(payload.contains("kick_result: accepted, permission_denied"));
        assert!(payload.contains("ban_result: accepted, permission_denied"));
        assert!(payload.contains("knock_result: accepted, permission_denied"));
        assert!(payload.contains("stale_policy"));
        assert!(payload.contains("audit_redaction"));
        assert!(payload.contains(ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(payload.contains("No m.room.power_levels write"));
        assert!(payload.contains("m.room.member mutation"));
        assert!(payload.contains("gateway/runtime/auth"));
        assert!(payload.contains("Telegram delivery"));
        assert!(payload.contains("live mutation"));
    }

    #[test]
    fn room_settings_power_member_result_taxonomy_packet_clipboard_label_is_local_only() {
        let label = room_settings_power_member_result_taxonomy_packet_clipboard_label(
            true,
            "Hepta UI",
            true,
            true,
            Some(21),
            "power result send allowed, react blocked, @room allowed",
            188,
        );

        assert!(label.contains("taxonomy for Hepta UI"));
        assert!(label.contains("taxonomy copied"));
        assert!(label.contains("timeline ready"));
        assert!(label.contains("loaded identity metadata ready"));
        assert!(label.contains("21 cached members"));
        assert!(label.contains("power result send allowed"));
        assert!(label.contains("local status 188 chars"));
        assert!(label.contains(ROOM_SETTINGS_POWER_MEMBER_RESULT_TAXONOMY_PACKET_LABEL));
        assert!(label.contains("No m.room.power_levels write"));
        assert!(label.contains("m.room.member mutation"));
        assert!(label.contains("retry automation"));
        assert!(label.contains("room-state mutation contract call"));
        assert!(label.contains("gateway/runtime/auth"));
        assert!(label.contains("Telegram delivery"));
        assert!(label.contains("live mutation"));
    }
}
