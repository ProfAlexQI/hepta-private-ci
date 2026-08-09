use codex_hepta_contracts::ActionId;
use codex_hepta_contracts::GovernanceDecisionRecord;
use codex_hepta_contracts::GovernanceReceipt;
use codex_hepta_contracts::PolicyPhase;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::ToolAction;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use sqlx::Row;
use tempfile::TempDir;

use crate::EvidenceError;
use crate::HeptaEvidenceStore;
use crate::live_product_shadow::VerifiedLiveProductOracleV2;
use crate::live_product_shadow::normalize_live_product_receipt_v2;
use crate::live_product_shadow::pinned_live_product_oracle_v2_bytes;
use crate::live_product_shadow::test_support;

fn sqlite_config(temp: &TempDir) -> SqliteConfig {
    SqliteConfig::new_for_testing(
        AbsolutePathBuf::try_from(temp.path().to_path_buf()).expect("absolute temp path"),
    )
}

fn dynamically_identified_receipt() -> GovernanceReceipt {
    let pinned = test_support::pinned_receipt();
    let source_authorization = pinned.authorization.as_ref().expect("pinned authorization");
    let action_id = ActionId::for_tool_call("dynamic-thread", "dynamic-turn", "dynamic-call");
    let action = ToolAction {
        schema_version: pinned.admission.action.schema_version,
        action_id,
        thread_id: "dynamic-thread".to_string(),
        turn_id: "dynamic-turn".to_string(),
        call_id: "dynamic-call".to_string(),
        tool_name: pinned.admission.action.tool_name.clone(),
        source: pinned.admission.action.source.clone(),
        payload_sha256: pinned.admission.action.payload_sha256.clone(),
    };
    let admission = GovernanceDecisionRecord::new(
        action.clone(),
        PolicyPhase::Admission,
        pinned.admission.mode,
        pinned.admission.policy.clone(),
        pinned.admission.decision.clone(),
    );
    let authorization = GovernanceDecisionRecord::new(
        action,
        PolicyPhase::Authorization,
        source_authorization.mode,
        source_authorization.policy.clone(),
        source_authorization.decision.clone(),
    );
    GovernanceReceipt::new(
        admission,
        Some(authorization),
        pinned.host_accepted,
        pinned.outcome,
    )
}

#[test]
fn exact_v2_corpus_loader_pins_bytes_shape_and_all_vectors() {
    let tracked = include_bytes!("../fixtures/live_product_oracle_v2_2f704.json");
    assert_eq!(tracked.len(), 5_195);
    assert_eq!(tracked.last(), Some(&b'\n'));
    assert_eq!(
        Sha256Digest::for_bytes(tracked).as_str(),
        test_support::expected_tracked_corpus_sha256()
    );

    let official = pinned_live_product_oracle_v2_bytes().expect("pinned official bytes");
    assert_eq!(official.len(), 5_194);
    assert_eq!(official.last(), Some(&b'}'));
    assert_eq!(
        Sha256Digest::for_bytes(official).as_str(),
        test_support::expected_corpus_sha256()
    );
    let oracle = VerifiedLiveProductOracleV2::load(official).expect("strict pinned loader");
    assert_eq!(
        test_support::normalization_digest(&test_support::pinned_receipt()),
        test_support::expected_normalized_receipt_sha256()
    );

    let document: serde_json::Value = serde_json::from_slice(official).expect("pinned JSON");
    let case = &document["cases"][0];
    assert_eq!(case["ordinal"], 1);
    assert_eq!(case["tool_name"], "shell_command");
    assert_eq!(
        case["function_arguments_raw"],
        r#"{"command":"/usr/bin/printf hepta-shadow-probe","login":false,"timeout_ms":5000}"#
    );
    assert_eq!(
        case["payload_sha256"],
        "0918708543060974ab1e37c2b08d0ea688838f4ec54477eb9945d62478e07cbf"
    );
    assert_eq!(
        case["sample_id_sha256"],
        test_support::expected_sample_id_sha256()
    );
    assert_eq!(
        case["expected_normalized_receipt_sha256"],
        test_support::expected_normalized_receipt_sha256()
    );
    drop(oracle);
}

#[test]
fn corpus_mutations_representation_lf_and_oversize_all_fail_closed() {
    let official = pinned_live_product_oracle_v2_bytes().expect("pinned official bytes");
    assert!(matches!(
        VerifiedLiveProductOracleV2::load(include_bytes!(
            "../fixtures/live_product_oracle_v2_2f704.json"
        )),
        Err(EvidenceError::InvalidRecord(_))
    ));

    let mut byte_mutation = official.to_vec();
    byte_mutation[100] ^= 1;
    assert!(matches!(
        VerifiedLiveProductOracleV2::load(&byte_mutation),
        Err(EvidenceError::InvalidRecord(_))
    ));

    let mut unknown_field = official.to_vec();
    unknown_field.pop();
    unknown_field.extend_from_slice(b",\"unknown\":false}");
    assert!(matches!(
        VerifiedLiveProductOracleV2::load(&unknown_field),
        Err(EvidenceError::InvalidRecord(_))
    ));

    let mut over_cap = official.to_vec();
    over_cap.push(b' ');
    assert!(matches!(
        VerifiedLiveProductOracleV2::load(&over_cap),
        Err(EvidenceError::InvalidRecord(_))
    ));
}

#[test]
fn v2_normalization_rebinds_only_dynamic_identity_and_rejects_semantic_mutation() {
    let dynamic = dynamically_identified_receipt();
    assert_eq!(
        test_support::normalization_digest(&dynamic),
        test_support::expected_normalized_receipt_sha256()
    );

    let normalized = normalize_live_product_receipt_v2(&dynamic).expect("normalize dynamic id");
    assert_eq!(normalized, test_support::pinned_receipt());

    let mut broken_identity = dynamic.clone();
    broken_identity.action_id = ActionId::for_tool_call("other", "other", "other");
    assert!(matches!(
        normalize_live_product_receipt_v2(&broken_identity),
        Err(EvidenceError::InvalidRecord(_))
    ));

    let mut wrong_tool = dynamic;
    wrong_tool.admission.action.tool_name = "exec_command".to_string();
    wrong_tool
        .authorization
        .as_mut()
        .expect("authorization")
        .action
        .tool_name = "exec_command".to_string();
    assert!(matches!(
        normalize_live_product_receipt_v2(&wrong_tool),
        Err(EvidenceError::InvalidRecord(_))
    ));
}

#[tokio::test]
async fn migration_has_only_the_exact_v2_object_and_trigger_sets() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("evidence store");
    let rows = sqlx::query(
        "SELECT type, name, tbl_name FROM sqlite_schema
         WHERE name LIKE 'live_product_shadow_v2_%'
           AND name NOT LIKE 'sqlite_autoindex_%'
         ORDER BY type, name",
    )
    .fetch_all(&store.pool)
    .await
    .expect("v2 schema objects");
    assert_eq!(rows.len(), 37);
    assert_eq!(
        rows.iter()
            .filter(|row| row.get::<String, _>("type") == "table")
            .count(),
        5
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.get::<String, _>("type") == "index")
            .count(),
        11
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.get::<String, _>("type") == "trigger")
            .count(),
        21
    );
}

#[tokio::test]
async fn append_only_foundation_enforces_intent_order_and_refuses_clean_terminal_state() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("evidence store");
    let seeded = test_support::seed_pending_foundation(&store.pool, true)
        .await
        .expect("seed pending foundation");
    let intent = seeded.intent_id.as_ref().expect("seeded intent");

    assert!(
        sqlx::query(
            "REPLACE INTO live_product_shadow_v2_runs
             SELECT * FROM live_product_shadow_v2_runs WHERE run_id = ?",
        )
        .bind(&seeded.run_id)
        .execute(&store.pool)
        .await
        .is_err()
    );
    assert!(
        sqlx::query(
            "INSERT OR REPLACE INTO live_product_shadow_v2_pre_send_intents
             SELECT * FROM live_product_shadow_v2_pre_send_intents WHERE intent_id = ?",
        )
        .bind(intent)
        .execute(&store.pool)
        .await
        .is_err()
    );
    assert!(
        sqlx::query("UPDATE live_product_shadow_v2_runs SET recorded_at_ms = recorded_at_ms + 1 WHERE run_id = ?")
            .bind(&seeded.run_id)
            .execute(&store.pool)
            .await
            .is_err()
    );
    assert!(
        sqlx::query("DELETE FROM live_product_shadow_v2_pre_send_intents WHERE intent_id = ?")
            .bind(intent)
            .execute(&store.pool)
            .await
            .is_err()
    );

    let digest = test_support::digest("wrong second intent");
    assert!(
        sqlx::query(
            "INSERT INTO live_product_shadow_v2_pre_send_intents (
                intent_id, run_id, segment_id, schema_version, intent_ordinal,
                previous_intent_sha256, sample_token_sha256,
                provider_request_semantic_sha256, intent_sha256, recorded_at_ms
             ) VALUES (?, ?, ?, 2, 2, ?, ?, ?, ?, 2000)",
        )
        .bind(&digest)
        .bind(&seeded.run_id)
        .bind(&seeded.app_segment_id)
        .bind(test_support::digest("wrong previous intent"))
        .bind(test_support::digest("sample"))
        .bind(test_support::digest("request"))
        .bind(test_support::digest("chain"))
        .execute(&store.pool)
        .await
        .is_err()
    );

    let arbitrary = test_support::digest("terminal");
    assert!(
        sqlx::query(
            "INSERT INTO live_product_shadow_v2_terminals (
                terminal_id, run_id, schema_version, terminal_status,
                observed_intent_count, observed_import_count,
                evidence_set_sha256, strict_imports_complete,
                canonical_oracle_all_matched, clean_qualified,
                duration_claimed, exact_verified, promotion_authority_granted,
                operator_acceptance_recorded, enforce_enabled, outbound_enabled,
                retirement_authority_granted, terminal_sha256, recorded_at_ms
             ) VALUES (?, ?, 2, 'incomplete', 1, 0, ?, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, ?, 2000)",
        )
        .bind(&arbitrary)
        .bind(&seeded.run_id)
        .bind(&arbitrary)
        .bind(&arbitrary)
        .execute(&store.pool)
        .await
        .is_err()
    );

    assert!(
        sqlx::query(
            "INSERT INTO live_product_shadow_v2_terminals (
                terminal_id, run_id, schema_version, terminal_status,
                observed_intent_count, observed_import_count,
                evidence_set_sha256, strict_imports_complete,
                canonical_oracle_all_matched, clean_qualified,
                duration_claimed, exact_verified, promotion_authority_granted,
                operator_acceptance_recorded, enforce_enabled, outbound_enabled,
                retirement_authority_granted, terminal_sha256, recorded_at_ms
             ) VALUES (?, ?, 2, 'strict_artifact_import_complete', 4, 4, ?, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, ?, 2000)",
        )
        .bind(&arbitrary)
        .bind(&seeded.run_id)
        .bind(&arbitrary)
        .bind(&arbitrary)
        .execute(&store.pool)
        .await
        .is_err()
    );

    let terminal_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM live_product_shadow_v2_terminals WHERE run_id = ?",
    )
    .bind(&seeded.run_id)
    .fetch_one(&store.pool)
    .await
    .expect("terminal count");
    assert_eq!(terminal_count, 0);
}

#[tokio::test]
async fn replace_guards_reject_isolated_primary_and_unique_identity_collisions() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("evidence store");

    // A run with no children proves that the run identity guard, rather than
    // a foreign key or a later state guard, rejects both REPLACE spellings.
    let run = test_support::seed_run_only(&store.pool, "isolated replace run")
        .await
        .expect("seed isolated run");
    assert!(
        sqlx::query(
            "REPLACE INTO live_product_shadow_v2_runs
             SELECT * FROM live_product_shadow_v2_runs WHERE run_id = ?",
        )
        .bind(&run.run_id)
        .execute(&store.pool)
        .await
        .is_err()
    );
    let colliding_run = test_support::digest("different run primary key");
    let colliding_binding = test_support::digest("different run binding");
    assert!(
        sqlx::query(
            "INSERT OR REPLACE INTO live_product_shadow_v2_runs (
                run_id, schema_version, run_binding_sha256, run_nonce_sha256,
                oracle_commit, oracle_tree, oracle_manifest_sha256,
                oracle_corpus_sha256, oracle_generator_sha256, oracle_profile,
                source_identity_status, exact_verified, oracle_live_reachable,
                actual_live_trial_closure_required,
                strict_artifact_import_required, qualification_status,
                governance_mode, enforce_enabled, promotion_authority_granted,
                outbound_enabled, retirement_authority_granted,
                operator_acceptance_recorded, started_at_ms, recorded_at_ms
             )
             SELECT ?, schema_version, ?, run_nonce_sha256, oracle_commit,
                    oracle_tree, oracle_manifest_sha256, oracle_corpus_sha256,
                    oracle_generator_sha256, oracle_profile,
                    source_identity_status, exact_verified,
                    oracle_live_reachable, actual_live_trial_closure_required,
                    strict_artifact_import_required, qualification_status,
                    governance_mode, enforce_enabled,
                    promotion_authority_granted, outbound_enabled,
                    retirement_authority_granted,
                    operator_acceptance_recorded, started_at_ms, recorded_at_ms
             FROM live_product_shadow_v2_runs WHERE run_id = ?",
        )
        .bind(&colliding_run)
        .bind(&colliding_binding)
        .bind(&run.run_id)
        .execute(&store.pool)
        .await
        .is_err()
    );
    let persisted_run_binding: String = sqlx::query_scalar(
        "SELECT run_binding_sha256 FROM live_product_shadow_v2_runs WHERE run_id = ?",
    )
    .bind(&run.run_id)
    .fetch_one(&store.pool)
    .await
    .expect("original run survives both collisions");
    assert_eq!(persisted_run_binding, run.run_binding_sha256);
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM live_product_shadow_v2_runs WHERE run_id = ?",
        )
        .bind(&colliding_run)
        .fetch_one(&store.pool)
        .await
        .expect("colliding run count"),
        0
    );

    // A segment with no intents or terminal likewise has no downstream guard
    // that could accidentally make this regression pass.
    let segment = test_support::insert_app_segment(
        &store.pool,
        &run,
        "isolated segment database nonce",
        1_002,
    )
    .await
    .expect("seed isolated segment");
    let original_segment_binding: String = sqlx::query_scalar(
        "SELECT segment_binding_sha256 FROM live_product_shadow_v2_segments
         WHERE segment_id = ?",
    )
    .bind(&segment)
    .fetch_one(&store.pool)
    .await
    .expect("original segment binding");
    assert!(
        sqlx::query(
            "INSERT OR REPLACE INTO live_product_shadow_v2_segments
             SELECT * FROM live_product_shadow_v2_segments WHERE segment_id = ?",
        )
        .bind(&segment)
        .execute(&store.pool)
        .await
        .is_err()
    );
    let second_run = test_support::seed_run_only(&store.pool, "second isolated replace run")
        .await
        .expect("seed second isolated run");
    let colliding_segment = test_support::digest("different segment primary key");
    let colliding_segment_binding = test_support::digest("different segment binding");
    assert!(
        sqlx::query(
            "INSERT OR REPLACE INTO live_product_shadow_v2_segments (
                segment_id, run_id, schema_version, segment_ordinal, surface,
                source_database_nonce_sha256, source_database_fresh,
                segment_binding_sha256, opened_at_ms, recorded_at_ms
             )
             SELECT ?, ?, schema_version, segment_ordinal, surface,
                    source_database_nonce_sha256, source_database_fresh, ?,
                    opened_at_ms, recorded_at_ms
             FROM live_product_shadow_v2_segments WHERE segment_id = ?",
        )
        .bind(&colliding_segment)
        .bind(&second_run.run_id)
        .bind(&colliding_segment_binding)
        .bind(&segment)
        .execute(&store.pool)
        .await
        .is_err()
    );
    let persisted_segment_binding: String = sqlx::query_scalar(
        "SELECT segment_binding_sha256 FROM live_product_shadow_v2_segments
         WHERE segment_id = ?",
    )
    .bind(&segment)
    .fetch_one(&store.pool)
    .await
    .expect("original segment survives both collisions");
    assert_eq!(persisted_segment_binding, original_segment_binding);

    // A pending import has no terminal and no children, so its collision guard
    // is the only append-only mechanism that can reject these replacements.
    let pending = test_support::seed_pending_foundation(&store.pool, true)
        .await
        .expect("seed pending import foundation");
    let import_id = test_support::digest("isolated rejected import");
    let transcript = test_support::digest("isolated rejected transcript");
    sqlx::query(
        "INSERT INTO live_product_shadow_v2_artifact_imports (
            import_id, run_id, segment_id, intent_id, schema_version,
            importer_schema, import_status, artifact_path_sha256,
            stable_bundle_manifest_sha256, verification_snapshot_sha256,
            transcript_sha256, normalized_receipt_sha256,
            oracle_sample_id_sha256, strict_artifact_validated,
            canonical_oracle_matched, qualification_authority_granted,
            import_sha256, imported_at_ms
         ) VALUES (?, ?, ?, ?, 2,
                   'hepta_live_product_shadow_strict_artifact_import_v2',
                   'rejected', ?, ?, ?, ?, NULL, NULL, 0, 0, 0, ?, 1010)",
    )
    .bind(&import_id)
    .bind(&pending.run_id)
    .bind(&pending.app_segment_id)
    .bind(pending.intent_id.as_ref().expect("pending intent"))
    .bind(test_support::digest("isolated artifact path"))
    .bind(test_support::digest("isolated stable bundle"))
    .bind(test_support::digest("isolated verification snapshot"))
    .bind(&transcript)
    .bind(&import_id)
    .execute(&store.pool)
    .await
    .expect("seed isolated rejected import");
    assert!(
        sqlx::query(
            "REPLACE INTO live_product_shadow_v2_artifact_imports
             SELECT * FROM live_product_shadow_v2_artifact_imports WHERE import_id = ?",
        )
        .bind(&import_id)
        .execute(&store.pool)
        .await
        .is_err()
    );
    let colliding_import = test_support::digest("different import primary key");
    assert!(
        sqlx::query(
            "INSERT OR REPLACE INTO live_product_shadow_v2_artifact_imports (
                import_id, run_id, segment_id, intent_id, schema_version,
                importer_schema, import_status, artifact_path_sha256,
                stable_bundle_manifest_sha256, verification_snapshot_sha256,
                transcript_sha256, normalized_receipt_sha256,
                oracle_sample_id_sha256, strict_artifact_validated,
                canonical_oracle_matched, qualification_authority_granted,
                import_sha256, imported_at_ms
             )
             SELECT ?, run_id, segment_id, intent_id, schema_version,
                    importer_schema, import_status, artifact_path_sha256,
                    stable_bundle_manifest_sha256,
                    verification_snapshot_sha256, transcript_sha256,
                    normalized_receipt_sha256, oracle_sample_id_sha256,
                    strict_artifact_validated, canonical_oracle_matched,
                    qualification_authority_granted, ?, imported_at_ms
             FROM live_product_shadow_v2_artifact_imports WHERE import_id = ?",
        )
        .bind(&colliding_import)
        .bind(&colliding_import)
        .bind(&import_id)
        .execute(&store.pool)
        .await
        .is_err()
    );
    let persisted_transcript: String = sqlx::query_scalar(
        "SELECT transcript_sha256 FROM live_product_shadow_v2_artifact_imports
         WHERE import_id = ?",
    )
    .bind(&import_id)
    .fetch_one(&store.pool)
    .await
    .expect("original import survives both collisions");
    assert_eq!(persisted_transcript, transcript);
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM live_product_shadow_v2_artifact_imports WHERE import_id = ?",
        )
        .bind(&colliding_import)
        .fetch_one(&store.pool)
        .await
        .expect("colliding import count"),
        0
    );

    let terminal = test_support::seed_incomplete_terminal(&store.pool, &second_run.run_id, 1_001)
        .await
        .expect("seed isolated terminal");
    assert!(
        sqlx::query(
            "REPLACE INTO live_product_shadow_v2_terminals
             SELECT * FROM live_product_shadow_v2_terminals WHERE terminal_id = ?",
        )
        .bind(&terminal)
        .execute(&store.pool)
        .await
        .is_err()
    );
    let colliding_terminal = test_support::digest("different terminal primary key");
    assert!(
        sqlx::query(
            "INSERT OR REPLACE INTO live_product_shadow_v2_terminals (
                terminal_id, run_id, schema_version, terminal_status,
                observed_intent_count, observed_import_count,
                evidence_set_sha256, strict_imports_complete,
                canonical_oracle_all_matched, clean_qualified,
                duration_claimed, exact_verified, promotion_authority_granted,
                operator_acceptance_recorded, enforce_enabled, outbound_enabled,
                retirement_authority_granted, terminal_sha256, recorded_at_ms
             )
             SELECT ?, run_id, schema_version, terminal_status,
                    observed_intent_count, observed_import_count,
                    evidence_set_sha256, strict_imports_complete,
                    canonical_oracle_all_matched, clean_qualified,
                    duration_claimed, exact_verified,
                    promotion_authority_granted,
                    operator_acceptance_recorded, enforce_enabled,
                    outbound_enabled, retirement_authority_granted, ?,
                    recorded_at_ms
             FROM live_product_shadow_v2_terminals WHERE terminal_id = ?",
        )
        .bind(&colliding_terminal)
        .bind(&colliding_terminal)
        .bind(&terminal)
        .execute(&store.pool)
        .await
        .is_err()
    );
    let persisted_terminal_sha: String = sqlx::query_scalar(
        "SELECT terminal_sha256 FROM live_product_shadow_v2_terminals
         WHERE terminal_id = ?",
    )
    .bind(&terminal)
    .fetch_one(&store.pool)
    .await
    .expect("original terminal survives both collisions");
    assert_eq!(persisted_terminal_sha, terminal);
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM live_product_shadow_v2_terminals WHERE terminal_id = ?",
        )
        .bind(&colliding_terminal)
        .fetch_one(&store.pool)
        .await
        .expect("colliding terminal count"),
        0
    );
}

#[tokio::test]
async fn terminal_chronology_cannot_precede_the_latest_durable_intent() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("evidence store");
    let seeded = test_support::seed_pending_foundation(&store.pool, true)
        .await
        .expect("seed pending foundation");

    assert!(
        sqlx::query(
            "INSERT INTO live_product_shadow_v2_pre_send_intents (
                intent_id, run_id, segment_id, schema_version, intent_ordinal,
                previous_intent_sha256, sample_token_sha256,
                provider_request_semantic_sha256, intent_sha256, recorded_at_ms
             ) VALUES (?, ?, ?, 2, 2, ?, ?, ?, ?, 1009)",
        )
        .bind(test_support::digest("backward intent id"))
        .bind(&seeded.run_id)
        .bind(&seeded.app_segment_id)
        .bind(seeded.intent_sha256.as_ref().expect("intent chain"))
        .bind(test_support::digest("backward sample"))
        .bind(test_support::digest("backward provider"))
        .bind(test_support::digest("backward chain"))
        .execute(&store.pool)
        .await
        .is_err()
    );
    let early_import = test_support::digest("early rejected import");
    assert!(
        sqlx::query(
            "INSERT INTO live_product_shadow_v2_artifact_imports (
                import_id, run_id, segment_id, intent_id, schema_version,
                importer_schema, import_status, artifact_path_sha256,
                stable_bundle_manifest_sha256,
                verification_snapshot_sha256, transcript_sha256,
                normalized_receipt_sha256, oracle_sample_id_sha256,
                strict_artifact_validated, canonical_oracle_matched,
                qualification_authority_granted, import_sha256, imported_at_ms
             ) VALUES (?, ?, ?, ?, 2,
                       'hepta_live_product_shadow_strict_artifact_import_v2',
                       'rejected', ?, ?, ?, ?, NULL, NULL, 0, 0, 0, ?, 1009)",
        )
        .bind(&early_import)
        .bind(&seeded.run_id)
        .bind(&seeded.app_segment_id)
        .bind(seeded.intent_id.as_ref().expect("intent"))
        .bind(test_support::digest("early artifact path"))
        .bind(test_support::digest("early stable bundle"))
        .bind(test_support::digest("early snapshot"))
        .bind(test_support::digest("early transcript"))
        .bind(&early_import)
        .execute(&store.pool)
        .await
        .is_err()
    );
    assert!(
        test_support::seed_incomplete_terminal(&store.pool, &seeded.run_id, 1_009)
            .await
            .is_err()
    );
    test_support::seed_incomplete_terminal(&store.pool, &seeded.run_id, 1_010)
        .await
        .expect("terminal at the latest durable evidence time");
}

#[tokio::test]
async fn strict_four_import_terminal_reopens_without_clean_or_authority_and_rejects_replace() {
    let temp = TempDir::new().expect("temp dir");
    let config = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&config)
        .await
        .expect("evidence store");
    let seeded = test_support::seed_strict_complete_foundation(&store.pool)
        .await
        .expect("seed strict-complete non-authoritative state");

    assert!(
        sqlx::query(
            "REPLACE INTO live_product_shadow_v2_runs
             SELECT * FROM live_product_shadow_v2_runs WHERE run_id = ?",
        )
        .bind(&seeded.run_id)
        .execute(&store.pool)
        .await
        .is_err()
    );
    assert!(
        sqlx::query(
            "INSERT OR REPLACE INTO live_product_shadow_v2_segments
             SELECT * FROM live_product_shadow_v2_segments WHERE segment_id = ?",
        )
        .bind(&seeded.segment_ids[0])
        .execute(&store.pool)
        .await
        .is_err()
    );
    assert!(
        sqlx::query(
            "REPLACE INTO live_product_shadow_v2_pre_send_intents
             SELECT * FROM live_product_shadow_v2_pre_send_intents WHERE intent_id = ?",
        )
        .bind(&seeded.intent_ids[0])
        .execute(&store.pool)
        .await
        .is_err()
    );
    assert!(
        sqlx::query(
            "INSERT OR REPLACE INTO live_product_shadow_v2_artifact_imports
             SELECT * FROM live_product_shadow_v2_artifact_imports WHERE import_id = ?",
        )
        .bind(&seeded.import_ids[0])
        .execute(&store.pool)
        .await
        .is_err()
    );
    assert!(
        sqlx::query(
            "REPLACE INTO live_product_shadow_v2_terminals
             SELECT * FROM live_product_shadow_v2_terminals WHERE terminal_id = ?",
        )
        .bind(&seeded.terminal_id)
        .execute(&store.pool)
        .await
        .is_err()
    );

    let second_run = test_support::seed_run_only(&store.pool, "nonce-replay-second-run")
        .await
        .expect("seed second run");
    assert!(
        test_support::insert_app_segment(
            &store.pool,
            &second_run,
            "strict app_server database nonce",
            1_002,
        )
        .await
        .is_err()
    );

    store.pool.close().await;
    drop(store);
    let reopened = HeptaEvidenceStore::open(&config)
        .await
        .expect("reopen strict-complete non-authoritative state");
    let row = sqlx::query(
        "SELECT terminal_status, clean_qualified, duration_claimed,
                exact_verified, promotion_authority_granted,
                operator_acceptance_recorded, enforce_enabled,
                outbound_enabled, retirement_authority_granted
         FROM live_product_shadow_v2_terminals WHERE terminal_id = ?",
    )
    .bind(&seeded.terminal_id)
    .fetch_one(&reopened.pool)
    .await
    .expect("strict terminal");
    assert_eq!(
        row.get::<String, _>("terminal_status"),
        "strict_artifact_import_complete"
    );
    for name in [
        "clean_qualified",
        "duration_claimed",
        "exact_verified",
        "promotion_authority_granted",
        "operator_acceptance_recorded",
        "enforce_enabled",
        "outbound_enabled",
        "retirement_authority_granted",
    ] {
        assert!(!row.get::<bool, _>(name));
    }
}

#[tokio::test]
async fn open_time_scan_rejects_strict_import_mutation_after_exact_trigger_restore() {
    let temp = TempDir::new().expect("temp dir");
    let config = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&config)
        .await
        .expect("evidence store");
    let seeded = test_support::seed_strict_complete_foundation(&store.pool)
        .await
        .expect("seed strict-complete state");
    let mut connection = store.pool.acquire().await.expect("SQLite connection");
    sqlx::query("DROP TRIGGER live_product_shadow_v2_imports_no_update")
        .execute(&mut *connection)
        .await
        .expect("drop import update guard");
    sqlx::query(
        "UPDATE live_product_shadow_v2_artifact_imports
         SET transcript_sha256 = ? WHERE import_id = ?",
    )
    .bind(test_support::digest("mutated transcript"))
    .bind(&seeded.import_ids[0])
    .execute(&mut *connection)
    .await
    .expect("mutate import binding");
    sqlx::query(
        "CREATE TRIGGER live_product_shadow_v2_imports_no_update
         BEFORE UPDATE ON live_product_shadow_v2_artifact_imports
         BEGIN
             SELECT RAISE(ABORT, 'live product-Shadow v2 artifact imports are immutable');
         END",
    )
    .execute(&mut *connection)
    .await
    .expect("restore exact import guard");
    drop(connection);
    store.pool.close().await;
    drop(store);

    assert!(matches!(
        HeptaEvidenceStore::open(&config).await,
        Err(EvidenceError::Corrupt(_))
    ));
}

#[tokio::test]
async fn negative_terminal_blocks_late_segment_and_segment_set_is_rechecked_on_open() {
    let temp = TempDir::new().expect("temp dir");
    let config = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&config)
        .await
        .expect("evidence store");
    let run = test_support::seed_run_only(&store.pool, "terminal-before-segment")
        .await
        .expect("seed run only");
    test_support::seed_incomplete_terminal(&store.pool, &run.run_id, 1_002)
        .await
        .expect("seed negative terminal");

    assert!(
        test_support::insert_app_segment(&store.pool, &run, "late segment nonce", 1_002)
            .await
            .is_err()
    );

    let mut connection = store.pool.acquire().await.expect("SQLite connection");
    sqlx::query("DROP TRIGGER live_product_shadow_v2_segments_before_terminal")
        .execute(&mut *connection)
        .await
        .expect("drop terminal guard");
    test_support::insert_app_segment(&store.pool, &run, "late segment nonce", 1_002)
        .await
        .expect("simulate mutation with guard absent");
    sqlx::query(
        "CREATE TRIGGER live_product_shadow_v2_segments_before_terminal
         BEFORE INSERT ON live_product_shadow_v2_segments
         WHEN EXISTS (
             SELECT 1 FROM live_product_shadow_v2_terminals
             WHERE run_id = NEW.run_id
         )
         BEGIN
             SELECT RAISE(ABORT, 'live product-Shadow v2 run is already terminal');
         END",
    )
    .execute(&mut *connection)
    .await
    .expect("restore exact terminal guard");
    drop(connection);
    store.pool.close().await;
    drop(store);

    assert!(matches!(
        HeptaEvidenceStore::open(&config).await,
        Err(EvidenceError::Corrupt(_))
    ));
}

#[tokio::test]
async fn simulated_crash_preserves_pending_intent_without_minting_terminal_or_authority() {
    let temp = TempDir::new().expect("temp dir");
    let config = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&config)
        .await
        .expect("evidence store");
    let seeded = test_support::seed_pending_foundation(&store.pool, true)
        .await
        .expect("seed pending foundation");
    store.pool.close().await;
    drop(store);

    let reopened = HeptaEvidenceStore::open(&config)
        .await
        .expect("reopen pending foundation");
    let row = sqlx::query(
        "SELECT qualification_status, exact_verified, oracle_live_reachable,
                promotion_authority_granted, operator_acceptance_recorded
         FROM live_product_shadow_v2_runs WHERE run_id = ?",
    )
    .bind(&seeded.run_id)
    .fetch_one(&reopened.pool)
    .await
    .expect("pending run row");
    assert_eq!(
        row.get::<String, _>("qualification_status"),
        "pending_strict_artifact_import"
    );
    assert!(!row.get::<bool, _>("exact_verified"));
    assert!(!row.get::<bool, _>("oracle_live_reachable"));
    assert!(!row.get::<bool, _>("promotion_authority_granted"));
    assert!(!row.get::<bool, _>("operator_acceptance_recorded"));
    let terminal_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM live_product_shadow_v2_terminals")
            .fetch_one(&reopened.pool)
            .await
            .expect("terminal count");
    assert_eq!(terminal_count, 0);
}

#[tokio::test]
async fn open_time_scan_rejects_chain_mutation_even_after_exact_trigger_is_restored() {
    let temp = TempDir::new().expect("temp dir");
    let config = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&config)
        .await
        .expect("evidence store");
    let seeded = test_support::seed_pending_foundation(&store.pool, true)
        .await
        .expect("seed pending foundation");
    let mut connection = store.pool.acquire().await.expect("SQLite connection");
    sqlx::query("DROP TRIGGER live_product_shadow_v2_intents_no_update")
        .execute(&mut *connection)
        .await
        .expect("drop update guard");
    sqlx::query(
        "UPDATE live_product_shadow_v2_pre_send_intents
         SET intent_sha256 = ? WHERE intent_id = ?",
    )
    .bind(test_support::digest("mutated chain"))
    .bind(seeded.intent_id.expect("intent"))
    .execute(&mut *connection)
    .await
    .expect("mutate chain");
    sqlx::query(
        "CREATE TRIGGER live_product_shadow_v2_intents_no_update
         BEFORE UPDATE ON live_product_shadow_v2_pre_send_intents
         BEGIN
             SELECT RAISE(ABORT, 'live product-Shadow v2 pre-send intents are immutable');
         END",
    )
    .execute(&mut *connection)
    .await
    .expect("restore exact trigger");
    drop(connection);
    store.pool.close().await;
    drop(store);

    assert!(matches!(
        HeptaEvidenceStore::open(&config).await,
        Err(EvidenceError::Corrupt(_))
    ));
}

#[tokio::test]
async fn schema_fingerprint_rejects_plausible_noop_trigger_replacement() {
    let temp = TempDir::new().expect("temp dir");
    let config = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&config)
        .await
        .expect("evidence store");
    let mut connection = store.pool.acquire().await.expect("SQLite connection");
    sqlx::query("DROP TRIGGER live_product_shadow_v2_runs_no_delete")
        .execute(&mut *connection)
        .await
        .expect("drop trigger");
    sqlx::query(
        "CREATE TRIGGER live_product_shadow_v2_runs_no_delete
         BEFORE DELETE ON live_product_shadow_v2_runs
         BEGIN SELECT 1; END",
    )
    .execute(&mut *connection)
    .await
    .expect("replace trigger");
    drop(connection);
    store.pool.close().await;
    drop(store);

    assert!(matches!(
        HeptaEvidenceStore::open(&config).await,
        Err(EvidenceError::Corrupt(_))
    ));
}

#[tokio::test]
async fn schema_fingerprint_preserves_string_literal_case() {
    let temp = TempDir::new().expect("temp dir");
    let config = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&config)
        .await
        .expect("evidence store");
    let mut connection = store.pool.acquire().await.expect("SQLite connection");
    sqlx::query("DROP TRIGGER live_product_shadow_v2_runs_no_delete")
        .execute(&mut *connection)
        .await
        .expect("drop trigger");
    sqlx::query(
        "CREATE TRIGGER live_product_shadow_v2_runs_no_delete
         BEFORE DELETE ON live_product_shadow_v2_runs
         BEGIN
             SELECT RAISE(ABORT, 'live product-shadow v2 runs cannot be deleted');
         END",
    )
    .execute(&mut *connection)
    .await
    .expect("replace only string-literal case");
    drop(connection);
    store.pool.close().await;
    drop(store);

    assert!(matches!(
        HeptaEvidenceStore::open(&config).await,
        Err(EvidenceError::Corrupt(_))
    ));
}

#[test]
fn live_v2_has_no_public_self_attestation_or_terminal_api() {
    let library = include_str!("lib.rs");
    let module = include_str!("live_product_shadow.rs");
    assert!(!library.contains("pub use live_product_shadow"));
    assert!(!module.contains("pub async fn"));
    assert!(!module.contains("pub fn begin_"));
    assert!(!module.contains("pub fn finish_"));
    assert!(!module.contains("DurationSoak"));
    assert!(!module.contains("ObservedCounts"));
    assert!(!module.contains("FinishObservation"));
    assert!(!module.contains("std::path::Path"));
}
