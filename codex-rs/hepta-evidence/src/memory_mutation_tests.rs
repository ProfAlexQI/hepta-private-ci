use codex_hepta_contracts::MEMORY_CONTRACT_SCHEMA_VERSION;
use codex_hepta_contracts::MemoryId;
use codex_hepta_contracts::MemoryLifecycle;
use codex_hepta_contracts::MemoryMutationDryRunDisposition;
use codex_hepta_contracts::MemoryMutationDryRunReason;
use codex_hepta_contracts::MemoryMutationProposal;
use codex_hepta_contracts::MemoryProvenance;
use codex_hepta_contracts::MemoryRevision;
use codex_hepta_contracts::MemoryScope;
use codex_hepta_contracts::MemorySourceKind;
use codex_hepta_contracts::RevisionStamp;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::canonical_memory_mutation_dry_run;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use tempfile::TempDir;

use crate::AppendDisposition;
use crate::EvidenceError;
use crate::HeptaEvidenceStore;
use crate::MEMORY_MUTATION_SHADOW_SCHEMA_VERSION;
use crate::MemoryMutationShadowObservation;

fn sqlite_config(temp: &TempDir) -> SqliteConfig {
    SqliteConfig::new_for_testing(
        AbsolutePathBuf::try_from(temp.path().to_path_buf()).expect("absolute temp path"),
    )
}

fn digest(value: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(value.as_bytes())
}

fn scope() -> MemoryScope {
    MemoryScope {
        installation_sha256: digest("installation"),
        workspace_sha256: digest("workspace"),
        thread_sha256: digest("thread"),
        principal_sha256: digest("principal"),
    }
}

fn candidate(content: &[u8], source_revision: u64) -> MemoryRevision {
    let scope = scope();
    MemoryRevision {
        schema_version: MEMORY_CONTRACT_SCHEMA_VERSION,
        memory_id: MemoryId::for_content(&scope, content),
        revision: RevisionStamp::new(1, content),
        scope,
        provenance: MemoryProvenance {
            source_kind: MemorySourceKind::ReviewedHeptaMemory,
            source_id_sha256: digest("source"),
            source_revision: RevisionStamp {
                revision: source_revision,
                content_sha256: digest(&format!("source-{source_revision}")),
            },
            observed_at_unix_seconds: 100,
        },
        lifecycle: MemoryLifecycle::Active,
        valid_until_unix_seconds: None,
    }
}

fn create_observation(content: &[u8]) -> MemoryMutationShadowObservation {
    let proposal = MemoryMutationProposal::create(
        "turn-1",
        digest("proposer"),
        candidate(content, 1),
        content,
    )
    .expect("valid proposal");
    let dry_run = canonical_memory_mutation_dry_run(
        proposal.proposal_id.clone(),
        digest("snapshot-absent"),
        MemoryMutationDryRunDisposition::WouldCreate,
        MemoryMutationDryRunReason::Ready,
        1,
    );
    MemoryMutationShadowObservation::new(proposal, dry_run).expect("valid observation")
}

#[tokio::test]
async fn shadow_observation_is_append_only_idempotent_and_restart_readable() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let observation = create_observation(b"private reviewed memory");

    assert_eq!(
        store
            .append_memory_mutation_shadow_observation(&observation)
            .await
            .expect("append observation"),
        AppendDisposition::Inserted,
    );
    assert_eq!(
        store
            .append_memory_mutation_shadow_observation(&observation)
            .await
            .expect("replay observation"),
        AppendDisposition::AlreadyPresent,
    );

    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    let payload_json: String = sqlx::query_scalar(
        "SELECT payload_json FROM memory_mutation_shadow_observations WHERE dry_run_id = ?",
    )
    .bind(observation.dry_run.dry_run_id.as_str())
    .fetch_one(&raw)
    .await
    .expect("stored payload");
    assert!(!payload_json.contains("private reviewed memory"));
    for forbidden in [
        "authority",
        "actual_memory_writes",
        "actual_kg_writes",
        "provider_calls",
        "network_calls",
    ] {
        assert!(!payload_json.contains(forbidden));
    }
    raw.close().await;
    drop(store);

    let reopened = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("reopen evidence");
    let stored = reopened
        .get_memory_mutation_shadow_observation(&observation.dry_run.dry_run_id)
        .await
        .expect("read observation")
        .expect("stored observation");
    assert_eq!(stored.observation, observation);
    assert!(stored.seq > 0);
}

#[tokio::test]
async fn snapshot_drift_records_distinct_shadow_observations() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("open evidence");
    let content = b"candidate";
    let current = candidate(content, 1);
    let proposal =
        MemoryMutationProposal::create("turn-1", digest("proposer"), current.clone(), content)
            .expect("valid proposal");
    let absent = MemoryMutationShadowObservation::new(
        proposal.clone(),
        canonical_memory_mutation_dry_run(
            proposal.proposal_id.clone(),
            digest("snapshot-absent"),
            MemoryMutationDryRunDisposition::WouldCreate,
            MemoryMutationDryRunReason::Ready,
            1,
        ),
    )
    .expect("absent observation");
    let present = MemoryMutationShadowObservation::new(
        proposal.clone(),
        canonical_memory_mutation_dry_run(
            proposal.proposal_id.clone(),
            digest("snapshot-present"),
            MemoryMutationDryRunDisposition::NoOp,
            MemoryMutationDryRunReason::ExactRevisionAlreadyPresent,
            0,
        ),
    )
    .expect("present observation");

    assert_eq!(
        absent.dry_run.disposition,
        MemoryMutationDryRunDisposition::WouldCreate
    );
    assert_eq!(
        present.dry_run.disposition,
        MemoryMutationDryRunDisposition::NoOp
    );
    assert!(absent.dry_run.has_integrity());
    assert!(present.dry_run.has_integrity());
    store
        .append_memory_mutation_shadow_observation(&absent)
        .await
        .expect("append absent");
    store
        .append_memory_mutation_shadow_observation(&present)
        .await
        .expect("append present");

    let stored = store
        .list_memory_mutation_shadow_observations(&proposal.proposal_id, 0, 10)
        .await
        .expect("list observations");
    assert_eq!(
        stored
            .into_iter()
            .map(|item| item.observation)
            .collect::<Vec<_>>(),
        vec![absent, present],
    );
}

#[tokio::test]
async fn concurrent_exact_append_has_one_insert_and_one_replay() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let first = HeptaEvidenceStore::open(&sqlite).await.expect("first pool");
    let second = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("second pool");
    let observation = create_observation(b"candidate");

    let (left, right) = tokio::join!(
        first.append_memory_mutation_shadow_observation(&observation),
        second.append_memory_mutation_shadow_observation(&observation),
    );
    let dispositions = [left.expect("left append"), right.expect("right append")];
    assert_eq!(
        dispositions
            .iter()
            .filter(|value| **value == AppendDisposition::Inserted)
            .count(),
        1,
    );
    assert_eq!(
        dispositions
            .iter()
            .filter(|value| **value == AppendDisposition::AlreadyPresent)
            .count(),
        1,
    );
}

#[tokio::test]
async fn invalid_projection_and_storage_mutation_fail_closed() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let observation = create_observation(b"candidate");
    let mut invalid = observation.clone();
    invalid.dry_run.projected_memory_writes = 2;
    assert!(matches!(
        store
            .append_memory_mutation_shadow_observation(&invalid)
            .await
            .expect_err("invalid projection must fail"),
        EvidenceError::InvalidRecord(_),
    ));

    store
        .append_memory_mutation_shadow_observation(&observation)
        .await
        .expect("append observation");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    sqlx::query(
        "UPDATE memory_mutation_shadow_observations SET recorded_at_ms = recorded_at_ms + 1
         WHERE dry_run_id = ?",
    )
    .bind(observation.dry_run.dry_run_id.as_str())
    .execute(&raw)
    .await
    .expect_err("append-only trigger must reject updates");
    sqlx::query("DELETE FROM memory_mutation_shadow_observations WHERE dry_run_id = ?")
        .bind(observation.dry_run.dry_run_id.as_str())
        .execute(&raw)
        .await
        .expect_err("append-only trigger must reject deletion");
    raw.close().await;
}

#[tokio::test]
async fn sqlite_rejects_invalid_projection_and_impossible_terminal_semantics() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    let digest_zero = "0".repeat(64);
    let digest_one = "1".repeat(64);
    let payload_json = "{}";
    let evidence_sha256 = Sha256Digest::for_bytes(payload_json.as_bytes());
    let insert = "INSERT INTO memory_mutation_shadow_observations (
            dry_run_id, proposal_id, turn_sha256, scope_sha256, snapshot_sha256,
            disposition, reason, projected_memory_writes, schema_version,
            payload_json, evidence_sha256, recorded_at_ms
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

    sqlx::query(insert)
        .bind("raw-dry-run-1")
        .bind("raw-proposal-1")
        .bind(&digest_zero)
        .bind(&digest_zero)
        .bind(&digest_zero)
        .bind("would_create")
        .bind("ready")
        .bind(2_i64)
        .bind(1_i64)
        .bind(payload_json)
        .bind(evidence_sha256.as_str())
        .bind(1_i64)
        .execute(&raw)
        .await
        .expect_err("SQL must reject an impossible projected write count");

    sqlx::query(insert)
        .bind("raw-dry-run-2")
        .bind("raw-proposal-2")
        .bind(&digest_one)
        .bind(&digest_one)
        .bind(&digest_one)
        .bind("blocked")
        .bind("ready")
        .bind(0_i64)
        .bind(1_i64)
        .bind(payload_json)
        .bind(evidence_sha256.as_str())
        .bind(2_i64)
        .execute(&raw)
        .await
        .expect_err("SQL must reject blocked-ready semantics");
    raw.close().await;
}

#[tokio::test]
async fn typed_readback_rejects_payload_substitution() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let observation = create_observation(b"candidate");
    store
        .append_memory_mutation_shadow_observation(&observation)
        .await
        .expect("append observation");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    sqlx::query("DROP TRIGGER memory_mutation_shadow_no_update")
        .execute(&raw)
        .await
        .expect("drop update trigger");
    let substituted = "{}";
    sqlx::query(
        "UPDATE memory_mutation_shadow_observations
         SET payload_json = ?, evidence_sha256 = ? WHERE dry_run_id = ?",
    )
    .bind(substituted)
    .bind(Sha256Digest::for_bytes(substituted.as_bytes()).as_str())
    .bind(observation.dry_run.dry_run_id.as_str())
    .execute(&raw)
    .await
    .expect("substitute payload");

    assert!(matches!(
        store
            .get_memory_mutation_shadow_observation(&observation.dry_run.dry_run_id)
            .await
            .expect_err("substituted payload must fail"),
        EvidenceError::Corrupt(_),
    ));
    raw.close().await;
}

#[tokio::test]
async fn typed_readback_rejects_projection_column_substitution() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let observation = create_observation(b"candidate");
    store
        .append_memory_mutation_shadow_observation(&observation)
        .await
        .expect("append observation");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    sqlx::query("DROP TRIGGER memory_mutation_shadow_no_update")
        .execute(&raw)
        .await
        .expect("drop update trigger");
    sqlx::query(
        "UPDATE memory_mutation_shadow_observations
         SET turn_sha256 = ? WHERE dry_run_id = ?",
    )
    .bind("0".repeat(64))
    .bind(observation.dry_run.dry_run_id.as_str())
    .execute(&raw)
    .await
    .expect("substitute projection column");

    assert!(matches!(
        store
            .get_memory_mutation_shadow_observation(&observation.dry_run.dry_run_id)
            .await
            .expect_err("substituted column must fail"),
        EvidenceError::Corrupt(_),
    ));
    raw.close().await;
}

#[tokio::test]
async fn open_rejects_missing_shadow_immutability_trigger() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    sqlx::query("DROP TRIGGER memory_mutation_shadow_no_update")
        .execute(&raw)
        .await
        .expect("drop trigger");
    raw.close().await;
    drop(store);

    let error = match HeptaEvidenceStore::open(&sqlite).await {
        Ok(_) => panic!("missing trigger must fail closed"),
        Err(error) => error,
    };
    assert!(matches!(error, EvidenceError::Corrupt(_)));
}

#[tokio::test]
async fn open_rejects_weakened_projection_schema_constraint() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("open evidence");
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    let mut connection = raw.acquire().await.expect("raw evidence connection");
    sqlx::query("PRAGMA writable_schema = ON")
        .execute(&mut *connection)
        .await
        .expect("enable writable schema");
    let update = sqlx::query(
        "UPDATE sqlite_schema
         SET sql = replace(
             sql,
             'projected_memory_writes BETWEEN 0 AND 2',
             'projected_memory_writes >= 0'
         )
         WHERE type = 'table' AND name = 'memory_mutation_shadow_observations'",
    )
    .execute(&mut *connection)
    .await
    .expect("weaken table constraint");
    assert_eq!(update.rows_affected(), 1);
    sqlx::query("PRAGMA writable_schema = OFF")
        .execute(&mut *connection)
        .await
        .expect("disable writable schema");
    drop(connection);
    raw.close().await;
    drop(store);

    let error = match HeptaEvidenceStore::open(&sqlite).await {
        Ok(_) => panic!("weakened schema must fail closed"),
        Err(error) => error,
    };
    assert!(matches!(error, EvidenceError::Corrupt(_)));
}

#[test]
fn serialized_shadow_observation_rejects_authority_fields() {
    let observation = create_observation(b"candidate");
    assert_eq!(
        observation.schema_version,
        MEMORY_MUTATION_SHADOW_SCHEMA_VERSION
    );

    let mut serialized = serde_json::to_value(observation).expect("serialize observation");
    serialized["authority_state"] = serde_json::Value::String("authorized".to_string());
    assert!(
        serde_json::from_value::<MemoryMutationShadowObservation>(serialized).is_err(),
        "the shadow schema has no authorized state",
    );
}
