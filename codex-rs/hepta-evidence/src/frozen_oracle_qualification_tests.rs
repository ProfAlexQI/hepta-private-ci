use std::sync::Arc;
use std::sync::OnceLock;

use codex_hepta_contracts::GovernanceMode;
use codex_hepta_contracts::Sha256Digest;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;
use sqlx::SqliteConnection;
use tempfile::TempDir;

use crate::AppendDisposition;
use crate::EvidenceError;
use crate::FrozenOracleConformanceRecord;
use crate::FrozenOracleConformanceStatus;
use crate::FrozenOracleQualificationTerminalRecord;
use crate::HeptaEvidenceStore;
use crate::VerifiedFrozenOracleCorpus;
use crate::frozen_oracle_qualification::FrozenOracleQualificationBinding;
use crate::pinned_frozen_oracle_corpus_bytes;

const FIXTURE_CANDIDATE_COMMIT: &str = "1111111111111111111111111111111111111111";
const FIXTURE_CANDIDATE_TREE: &str = "2222222222222222222222222222222222222222";
const PINNED_CORPUS_SHA256: &str =
    "6fbf5ef6eba851554f1c342fc6e262ff17c51dc58033ed2ca5d86dcafb7d804f";
const CASE_COUNT: u64 = 252;

fn sqlite_config(temp: &TempDir) -> SqliteConfig {
    SqliteConfig::new_for_testing(
        AbsolutePathBuf::try_from(temp.path().to_path_buf()).expect("absolute temp path"),
    )
}

fn digest(label: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(label.as_bytes())
}

fn corpus() -> VerifiedFrozenOracleCorpus {
    VerifiedFrozenOracleCorpus::load(pinned_frozen_oracle_corpus_bytes())
        .expect("library-pinned frozen-oracle corpus")
}

fn corpus_json() -> &'static Value {
    static CORPUS: OnceLock<Value> = OnceLock::new();
    CORPUS.get_or_init(|| {
        serde_json::from_slice(pinned_frozen_oracle_corpus_bytes()).expect("pinned corpus JSON")
    })
}

fn candidate_projection(ordinal: u64, diverges: bool) -> Value {
    let mut projection = corpus_json()["cases"]
        .as_array()
        .expect("pinned cases")
        .get(usize::try_from(ordinal - 1).expect("ordinal index"))
        .expect("pinned ordinal")["expected_lifecycle_projection"]
        .clone();
    if diverges {
        projection["audit_test_divergence"] = Value::Bool(true);
    }
    projection
}

fn plan(
    corpus: &VerifiedFrozenOracleCorpus,
    nonce_label: &str,
) -> crate::FrozenOracleQualificationPlan {
    corpus
        .qualification_plan(
            FIXTURE_CANDIDATE_COMMIT,
            FIXTURE_CANDIDATE_TREE,
            digest(nonce_label),
        )
        .expect("capability-derived qualification plan")
}

async fn append_case(
    store: &HeptaEvidenceStore,
    corpus: &VerifiedFrozenOracleCorpus,
    run_id: &crate::FrozenOracleQualificationRunId,
    ordinal: u64,
    diverges: bool,
) -> crate::FrozenOracleConformanceAppend {
    store
        .append_frozen_oracle_conformance_observation(
            run_id,
            corpus,
            ordinal,
            &candidate_projection(ordinal, diverges),
        )
        .await
        .expect("append capability-derived conformance observation")
}

async fn append_range(
    store: &HeptaEvidenceStore,
    corpus: &VerifiedFrozenOracleCorpus,
    run_id: &crate::FrozenOracleQualificationRunId,
    first: u64,
    last: u64,
) {
    for ordinal in first..=last {
        append_case(store, corpus, run_id, ordinal, false).await;
    }
}

fn frozen_oracle_conformance_observation_digest(payload: &[u8]) -> Sha256Digest {
    let domain = b"hepta-frozen-oracle-conformance-observation:v1";
    let mut hasher = Sha256::new();
    for part in [domain.as_slice(), payload] {
        hasher.update((part.len() as u64).to_be_bytes());
        hasher.update(part);
    }
    Sha256Digest::parse(format!("{:x}", hasher.finalize())).expect("observation digest")
}

async fn insert_raw_frozen_oracle_conformance_observation(
    connection: &mut SqliteConnection,
    observation: &FrozenOracleConformanceRecord,
    observation_sha256: &Sha256Digest,
    payload_json: &str,
    payload_sha256: &Sha256Digest,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO frozen_oracle_qualification_observations (
            qualification_run_id, binding_sha256, canonical_oracle_corpus_sha256,
            ordinal, sample_id_sha256, candidate_output_sha256,
            canonical_oracle_output_sha256, canonical_oracle_matched,
            previous_observation_sha256, observation_sha256, schema_version,
            qualification_only, promotion_authority_granted, payload_json,
            payload_sha256, recorded_at_ms
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(observation.qualification_run_id.as_str())
    .bind(observation.binding_sha256.as_str())
    .bind(observation.canonical_oracle_corpus_sha256.as_str())
    .bind(i64::try_from(observation.ordinal).expect("observation ordinal"))
    .bind(observation.sample_id_sha256.as_str())
    .bind(observation.candidate_output_sha256.as_str())
    .bind(observation.canonical_oracle_output_sha256.as_str())
    .bind(observation.canonical_oracle_matched)
    .bind(observation.previous_observation_sha256.as_str())
    .bind(observation_sha256.as_str())
    .bind(i64::from(observation.schema_version))
    .bind(observation.qualification_only)
    .bind(observation.promotion_authority_granted)
    .bind(payload_json)
    .bind(payload_sha256.as_str())
    .bind(1_i64)
    .execute(&mut *connection)
    .await?;
    Ok(())
}

#[test]
fn pinned_loader_is_byte_bounded_and_rejects_every_non_pinned_corpus() {
    let verified = corpus();
    assert_eq!(verified.corpus_sha256().as_str(), PINNED_CORPUS_SHA256);
    assert_eq!(verified.sample_count(), CASE_COUNT);

    let oversized = vec![b' '; 1_048_577];
    assert!(matches!(
        VerifiedFrozenOracleCorpus::load(&oversized),
        Err(EvidenceError::InvalidRecord(_))
    ));
    let mut tampered = pinned_frozen_oracle_corpus_bytes().to_vec();
    tampered[0] ^= 1;
    assert!(matches!(
        VerifiedFrozenOracleCorpus::load(&tampered),
        Err(EvidenceError::InvalidRecord(_))
    ));
}

#[tokio::test]
async fn official_sha_single_equal_projection_cannot_self_sign_conformant() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("evidence store");
    let corpus = corpus();
    let plan = plan(&corpus, "official-sha-count-one-attack");
    let registration = store
        .begin_frozen_oracle_qualification_run(&corpus, &plan)
        .await
        .expect("register pinned run");
    let first = append_case(
        &store,
        &corpus,
        &registration.qualification_run_id,
        1,
        false,
    )
    .await;
    assert!(first.stored.observation.canonical_oracle_matched);
    assert!(matches!(
        store
            .finish_frozen_oracle_qualification_run(&registration.qualification_run_id, &corpus,)
            .await
            .expect_err("one equal digest cannot self-sign conformant"),
        EvidenceError::InvalidRecord(_)
    ));
    let summary = store
        .get_frozen_oracle_qualification_run_summary(&registration.qualification_run_id, &corpus)
        .await
        .expect("summary")
        .expect("registered run");
    assert_eq!(summary.binding.required_sample_count(), CASE_COUNT);
    assert_eq!(
        summary.binding.canonical_oracle_corpus_sha256().as_str(),
        PINNED_CORPUS_SHA256
    );
    assert_eq!(summary.observation_count, 1);
    assert!(summary.terminal.is_none());
}

#[tokio::test]
async fn restart_continuity_terminal_anchor_and_exact_replays_are_durable() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let corpus = corpus();
    let plan = plan(&corpus, "restart-continuity");
    let first = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("first evidence store");
    let registration = first
        .begin_frozen_oracle_qualification_run(&corpus, &plan)
        .await
        .expect("register qualification run");
    assert_eq!(registration.disposition, AppendDisposition::Inserted);
    assert_eq!(
        first
            .begin_frozen_oracle_qualification_run(&corpus, &plan)
            .await
            .expect("exact registration replay")
            .disposition,
        AppendDisposition::AlreadyPresent
    );
    append_range(
        &first,
        &corpus,
        &registration.qualification_run_id,
        1,
        CASE_COUNT / 2,
    )
    .await;
    drop(first);

    let restarted = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("restart evidence store");
    assert_eq!(
        append_case(
            &restarted,
            &corpus,
            &registration.qualification_run_id,
            CASE_COUNT / 2,
            false,
        )
        .await
        .disposition,
        AppendDisposition::AlreadyPresent
    );
    append_range(
        &restarted,
        &corpus,
        &registration.qualification_run_id,
        CASE_COUNT / 2 + 1,
        CASE_COUNT,
    )
    .await;
    let terminal = restarted
        .finish_frozen_oracle_qualification_run(&registration.qualification_run_id, &corpus)
        .await
        .expect("terminal");
    assert_eq!(
        terminal.terminal.conformance_status,
        FrozenOracleConformanceStatus::Conformant
    );
    assert_eq!(terminal.terminal.observation_count, CASE_COUNT);
    assert_eq!(terminal.terminal.canonical_oracle_match_count, CASE_COUNT);
    assert_eq!(terminal.terminal.canonical_oracle_divergence_count, 0);
    assert!(
        terminal.terminal.qualification_run_finished_at_ms
            >= terminal.terminal.qualification_run_started_at_ms
    );
    assert_eq!(terminal.terminal.governance_mode, GovernanceMode::Shadow);
    assert!(!terminal.terminal.enforce_enabled);
    assert!(terminal.terminal.qualification_only);
    assert!(!terminal.terminal.promotion_authority_granted);
    assert_eq!(
        restarted
            .finish_frozen_oracle_qualification_run(&registration.qualification_run_id, &corpus,)
            .await
            .expect("exact terminal replay"),
        terminal
    );
    drop(restarted);

    let reopened = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("terminal restart");
    let summary = reopened
        .get_frozen_oracle_qualification_run_summary(&registration.qualification_run_id, &corpus)
        .await
        .expect("summary")
        .expect("registered qualification run");
    assert_eq!(summary.observation_count, CASE_COUNT);
    assert_eq!(
        summary.head_observation_sha256,
        terminal.terminal.head_observation_sha256
    );
    assert_eq!(summary.terminal, Some(terminal));
}

#[tokio::test]
async fn divergence_is_durable_qualification_evidence_without_authority() {
    let temp = TempDir::new().expect("temp dir");
    let store = HeptaEvidenceStore::open(&sqlite_config(&temp))
        .await
        .expect("evidence store");
    let corpus = corpus();
    let plan = plan(&corpus, "durable-divergence");
    let registration = store
        .begin_frozen_oracle_qualification_run(&corpus, &plan)
        .await
        .expect("register");
    let diverged = append_case(&store, &corpus, &registration.qualification_run_id, 1, true).await;
    assert!(!diverged.stored.observation.canonical_oracle_matched);
    append_range(
        &store,
        &corpus,
        &registration.qualification_run_id,
        2,
        CASE_COUNT,
    )
    .await;
    let terminal = store
        .finish_frozen_oracle_qualification_run(&registration.qualification_run_id, &corpus)
        .await
        .expect("diverged terminal");
    assert_eq!(
        terminal.terminal.conformance_status,
        FrozenOracleConformanceStatus::Diverged
    );
    assert_eq!(terminal.terminal.canonical_oracle_divergence_count, 1);
    assert!(!terminal.terminal.promotion_authority_granted);
}

#[tokio::test]
async fn canonical_payloads_and_sql_caps_reject_unbounded_or_unknown_json() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("evidence store");
    let corpus = corpus();
    let plan = plan(&corpus, "canonical-payloads");
    let registration = store
        .begin_frozen_oracle_qualification_run(&corpus, &plan)
        .await
        .expect("register");
    let observation = append_case(
        &store,
        &corpus,
        &registration.qualification_run_id,
        1,
        false,
    )
    .await
    .stored
    .observation;
    append_range(
        &store,
        &corpus,
        &registration.qualification_run_id,
        2,
        CASE_COUNT,
    )
    .await;
    let terminal = store
        .finish_frozen_oracle_qualification_run(&registration.qualification_run_id, &corpus)
        .await
        .expect("terminal")
        .terminal;

    let mut binding_json = serde_json::to_value(plan.binding.clone()).expect("binding JSON");
    binding_json["unknown"] = Value::Bool(true);
    assert!(serde_json::from_value::<FrozenOracleQualificationBinding>(binding_json).is_err());
    let mut observation_json = serde_json::to_value(observation).expect("observation JSON");
    observation_json["unknown"] = Value::Bool(true);
    assert!(serde_json::from_value::<FrozenOracleConformanceRecord>(observation_json).is_err());
    let mut terminal_json = serde_json::to_value(terminal).expect("terminal JSON");
    terminal_json["unknown"] = Value::Bool(true);
    assert!(
        serde_json::from_value::<FrozenOracleQualificationTerminalRecord>(terminal_json).is_err()
    );

    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    sqlx::query("DROP TRIGGER frozen_oracle_qualification_bindings_no_update")
        .execute(&raw)
        .await
        .expect("drop immutable trigger for SQL cap test");
    let oversized_json = format!("\"{}\"", "x".repeat(16_384));
    let error = sqlx::query(
        "UPDATE frozen_oracle_qualification_bindings SET binding_json = ?
         WHERE qualification_run_id = ?",
    )
    .bind(oversized_json)
    .bind(registration.qualification_run_id.as_str())
    .execute(&raw)
    .await
    .expect_err("SQL byte cap must reject oversized JSON before Rust parsing");
    assert!(error.to_string().contains("CHECK constraint failed"));
    sqlx::query(
        "CREATE TRIGGER frozen_oracle_qualification_bindings_no_update
         BEFORE UPDATE ON frozen_oracle_qualification_bindings
         BEGIN
             SELECT RAISE(ABORT, 'frozen-oracle qualification run bindings are immutable');
         END",
    )
    .execute(&raw)
    .await
    .expect("restore exact immutable trigger");
    raw.close().await;
}

#[tokio::test]
async fn no_op_trigger_with_required_words_cannot_spoof_exact_fingerprint() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("evidence store");
    let database_path = store.path().to_path_buf();
    drop(store);

    let raw = sqlite
        .open_durable_evidence_pool(&database_path)
        .await
        .expect("raw evidence pool");
    sqlx::query("DROP TRIGGER frozen_oracle_qualification_observations_chain_guard")
        .execute(&raw)
        .await
        .expect("drop exact chain trigger");
    sqlx::query(
        "CREATE TRIGGER frozen_oracle_qualification_observations_chain_guard
         BEFORE INSERT ON frozen_oracle_qualification_observations
         BEGIN
             /* Deliberately inert: all legacy fragment words are decorative. */
             SELECT 'frozen_oracle_qualification_heads observation_count + 1 = new.ordinal
                 new.ordinal > required_sample_count previous_observation_sha256
                 head_observation_sha256 = new.previous_observation_sha256 binding_sha256
                 raise(abort frozen-oracle qualification run chain is not contiguous';
         END",
    )
    .execute(&raw)
    .await
    .expect("install fragment-spoofing no-op trigger");
    raw.close().await;

    let error = match HeptaEvidenceStore::open(&sqlite).await {
        Ok(_) => panic!("exact trigger fingerprint accepted a no-op definition"),
        Err(error) => error,
    };
    match error {
        EvidenceError::Corrupt(detail) => {
            assert!(detail.contains("fingerprinted SQLite trigger"));
        }
        other => panic!("expected corrupt schema fingerprint, got {other:?}"),
    }
}

#[tokio::test]
async fn restored_schema_trigger_cannot_hide_a_broken_hash_chain_projection() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("evidence store");
    let corpus = corpus();
    let plan = plan(&corpus, "broken-chain");
    let registration = store
        .begin_frozen_oracle_qualification_run(&corpus, &plan)
        .await
        .expect("register");
    append_range(&store, &corpus, &registration.qualification_run_id, 1, 2).await;
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    sqlx::query("DROP TRIGGER frozen_oracle_qualification_observations_no_update")
        .execute(&raw)
        .await
        .expect("drop trigger for corruption simulation");
    sqlx::query(
        "UPDATE frozen_oracle_qualification_observations
         SET previous_observation_sha256 = ? WHERE qualification_run_id = ? AND ordinal = 2",
    )
    .bind("0".repeat(64))
    .bind(registration.qualification_run_id.as_str())
    .execute(&raw)
    .await
    .expect("corrupt chain projection");
    sqlx::query(
        "CREATE TRIGGER frozen_oracle_qualification_observations_no_update
         BEFORE UPDATE ON frozen_oracle_qualification_observations
         BEGIN
             SELECT RAISE(ABORT, 'frozen-oracle qualification run observations are immutable');
         END",
    )
    .execute(&raw)
    .await
    .expect("restore exact immutable trigger");
    raw.close().await;
    drop(store);

    assert!(matches!(
        HeptaEvidenceStore::open(&sqlite).await,
        Err(EvidenceError::Corrupt(_))
    ));
}

#[tokio::test]
async fn sql_and_reopen_both_reject_an_observation_beyond_the_pinned_count() {
    let temp = TempDir::new().expect("temp dir");
    let sqlite = sqlite_config(&temp);
    let store = HeptaEvidenceStore::open(&sqlite)
        .await
        .expect("evidence store");
    let corpus = corpus();
    let plan = plan(&corpus, "over-bound");
    let registration = store
        .begin_frozen_oracle_qualification_run(&corpus, &plan)
        .await
        .expect("register");
    append_case(
        &store,
        &corpus,
        &registration.qualification_run_id,
        1,
        false,
    )
    .await;
    let summary = store
        .get_frozen_oracle_qualification_run_summary(&registration.qualification_run_id, &corpus)
        .await
        .expect("summary")
        .expect("registered run");
    let output_sha256 = digest("raw-over-bound-output");
    let over_bound = FrozenOracleConformanceRecord {
        schema_version: crate::FROZEN_ORACLE_QUALIFICATION_SCHEMA_VERSION,
        qualification_run_id: registration.qualification_run_id.clone(),
        binding_sha256: summary.binding_sha256.clone(),
        canonical_oracle_corpus_sha256: summary.binding.canonical_oracle_corpus_sha256().clone(),
        ordinal: CASE_COUNT + 1,
        sample_id_sha256: digest("raw-over-bound-sample"),
        candidate_output_sha256: output_sha256.clone(),
        canonical_oracle_output_sha256: output_sha256,
        canonical_oracle_matched: true,
        previous_observation_sha256: summary.head_observation_sha256,
        qualification_only: true,
        promotion_authority_granted: false,
    };
    let payload = crate::canonical::canonical_json(&over_bound).expect("canonical observation");
    let payload_json = String::from_utf8(payload.clone()).expect("observation JSON");
    let payload_sha256 = Sha256Digest::for_bytes(&payload);
    let observation_sha256 = frozen_oracle_conformance_observation_digest(&payload);
    let raw = sqlite
        .open_durable_evidence_pool(store.path())
        .await
        .expect("raw evidence pool");
    let mut raw_connection = raw.acquire().await.expect("raw evidence connection");
    let error = insert_raw_frozen_oracle_conformance_observation(
        &mut raw_connection,
        &over_bound,
        &observation_sha256,
        &payload_json,
        &payload_sha256,
    )
    .await
    .expect_err("SQL trigger must reject an over-bound observation");
    assert!(error.to_string().contains("chain is not contiguous"));

    sqlx::query("DROP TRIGGER frozen_oracle_qualification_observations_chain_guard")
        .execute(&mut *raw_connection)
        .await
        .expect("drop chain trigger for corruption simulation");
    sqlx::query("DROP TRIGGER frozen_oracle_qualification_observations_advance_head")
        .execute(&mut *raw_connection)
        .await
        .expect("drop head trigger for corruption simulation");
    sqlx::query("PRAGMA ignore_check_constraints = ON")
        .execute(&mut *raw_connection)
        .await
        .expect("enable explicit corruption simulation");
    insert_raw_frozen_oracle_conformance_observation(
        &mut raw_connection,
        &over_bound,
        &observation_sha256,
        &payload_json,
        &payload_sha256,
    )
    .await
    .expect("simulate a legacy over-bound row");
    sqlx::query("PRAGMA ignore_check_constraints = OFF")
        .execute(&mut *raw_connection)
        .await
        .expect("restore check constraints");
    sqlx::query(
        "CREATE TRIGGER frozen_oracle_qualification_observations_chain_guard
         BEFORE INSERT ON frozen_oracle_qualification_observations
         WHEN NOT EXISTS (
                 SELECT 1
                 FROM frozen_oracle_qualification_heads
                 WHERE qualification_run_id = NEW.qualification_run_id
                   AND binding_sha256 = NEW.binding_sha256
                   AND observation_count + 1 = NEW.ordinal
                   AND head_observation_sha256 = NEW.previous_observation_sha256
             )
             OR NEW.ordinal > (
                 SELECT required_sample_count
                 FROM frozen_oracle_qualification_bindings
                 WHERE qualification_run_id = NEW.qualification_run_id
             )
         BEGIN
             SELECT RAISE(ABORT, 'frozen-oracle qualification run chain is not contiguous');
         END",
    )
    .execute(&mut *raw_connection)
    .await
    .expect("restore exact chain trigger");
    sqlx::query(
        "CREATE TRIGGER frozen_oracle_qualification_observations_advance_head
         AFTER INSERT ON frozen_oracle_qualification_observations
         BEGIN
             UPDATE frozen_oracle_qualification_heads
             SET observation_count = observation_count + 1,
                 canonical_oracle_match_count =
                     canonical_oracle_match_count + NEW.canonical_oracle_matched,
                 canonical_oracle_divergence_count =
                     canonical_oracle_divergence_count + (1 - NEW.canonical_oracle_matched),
                 head_observation_sha256 = NEW.observation_sha256
             WHERE qualification_run_id = NEW.qualification_run_id
               AND binding_sha256 = NEW.binding_sha256
               AND observation_count + 1 = NEW.ordinal
               AND head_observation_sha256 = NEW.previous_observation_sha256;
             SELECT CASE WHEN changes() <> 1
                 THEN RAISE(ABORT, 'frozen-oracle qualification append head did not advance')
             END;
         END",
    )
    .execute(&mut *raw_connection)
    .await
    .expect("restore exact advance trigger");
    drop(raw_connection);
    raw.close().await;
    drop(store);

    assert!(matches!(
        HeptaEvidenceStore::open(&sqlite).await,
        Err(EvidenceError::Corrupt(_))
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn summary_holds_one_snapshot_when_an_append_commits_between_selects() {
    let temp = TempDir::new().expect("temp dir");
    let other_temp = TempDir::new().expect("other temp dir");
    let corpus = corpus();
    let run_plan = plan(&corpus, "snapshot-hook-dedicated-qualification-nonce");
    let store = Arc::new(
        HeptaEvidenceStore::open(&sqlite_config(&temp))
            .await
            .expect("evidence store"),
    );
    let registration = store
        .begin_frozen_oracle_qualification_run(&corpus, &run_plan)
        .await
        .expect("register");
    append_case(
        &store,
        &corpus,
        &registration.qualification_run_id,
        1,
        false,
    )
    .await;
    let other_store = HeptaEvidenceStore::open(&sqlite_config(&other_temp))
        .await
        .expect("other evidence store");
    let other_registration = other_store
        .begin_frozen_oracle_qualification_run(&corpus, &run_plan)
        .await
        .expect("register same run identity in another database");
    assert_eq!(
        other_registration.qualification_run_id,
        registration.qualification_run_id
    );

    let hook =
        crate::frozen_oracle_qualification::install_frozen_oracle_qualification_load_snapshot_hook(
            &store,
            &registration.qualification_run_id,
        );
    tokio::time::timeout(
        std::time::Duration::from_secs(5),
        other_store.get_frozen_oracle_qualification_run_summary(
            &other_registration.qualification_run_id,
            &corpus,
        ),
    )
    .await
    .expect("another database must not consume the target store hook")
    .expect("other database summary")
    .expect("other registered run");
    let summary_store = Arc::clone(&store);
    let summary_run_id = registration.qualification_run_id.clone();
    let summary_corpus = corpus.clone();
    let summary_reader = tokio::spawn(async move {
        summary_store
            .get_frozen_oracle_qualification_run_summary(&summary_run_id, &summary_corpus)
            .await
            .expect("interleaved snapshot summary")
            .expect("registered qualification run")
    });
    hook.binding_loaded.notified().await;
    append_case(
        &store,
        &corpus,
        &registration.qualification_run_id,
        2,
        false,
    )
    .await;
    hook.resume.notify_one();
    let interleaved = summary_reader.await.expect("summary reader task");
    assert_eq!(interleaved.observation_count, 1);
    assert_eq!(interleaved.canonical_oracle_match_count, 1);
    assert_eq!(interleaved.canonical_oracle_divergence_count, 0);

    let after_commit = store
        .get_frozen_oracle_qualification_run_summary(&registration.qualification_run_id, &corpus)
        .await
        .expect("final summary")
        .expect("registered qualification run");
    assert_eq!(after_commit.observation_count, 2);
    assert_eq!(after_commit.canonical_oracle_match_count, 2);
    assert_eq!(after_commit.canonical_oracle_divergence_count, 0);
}
