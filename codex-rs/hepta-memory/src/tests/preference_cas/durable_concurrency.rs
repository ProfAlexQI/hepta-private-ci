use super::*;

use crate::DurablePreferenceStore;

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_reads_and_recovery_observe_one_sqlite_snapshot() -> TestResult {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("v2-memory.sqlite3");
    let writer = DurablePreferenceStore::bootstrap_new(&database_path).await?;
    let reader = DurablePreferenceStore::open_existing(&database_path).await?;
    let subject = PrincipalId::new("subject-durable-snapshot");

    for index in 0..24 {
        let preference = PreferenceId::new(format!("preference-concurrent-genesis-{index}"));
        let genesis = preference_document(0, "sha256:g", "reducer.v1", "{}");
        let (initialized, observed) = tokio::join!(
            writer.get_or_init_genesis(preference.clone(), subject.clone(), genesis.clone(),),
            reader.read_document(&preference, &subject),
        );
        assert_eq!(initialized?, PreferenceGenesisOutcome::Initialized);
        let observed = observed?;
        assert!(observed.is_none() || observed.as_ref() == Some(&genesis));
    }

    let preference = PreferenceId::new("preference-concurrent-genesis-0");
    let genesis = preference_document(0, "sha256:g", "reducer.v1", "{}");
    let next = preference_document(1, "sha256:state-1", "reducer.v1", r#"{"revision":1}"#);
    let receipt = outcome_receipt("receipt-snapshot-1", "sha256:receipt-snapshot-1")?;
    let change = transition(
        "transition-snapshot-1",
        preference.clone(),
        subject.clone(),
        genesis.state().clone(),
        next.state().clone(),
        &receipt,
    )?;
    let observer_path = database_path.clone();
    let observer_preference = preference.clone();
    let observer_subject = subject.clone();
    let (committed, observed) = tokio::join!(
        async {
            tokio::task::yield_now().await;
            writer.commit_evidenced(change, next.clone()).await
        },
        async {
            for _ in 0..40 {
                let recovered = DurablePreferenceStore::open_existing(&observer_path).await?;
                assert!(
                    recovered
                        .read_document(&observer_preference, &observer_subject)
                        .await?
                        .is_some()
                );
                tokio::task::yield_now().await;
            }
            Ok::<_, PreferenceCasError>(())
        }
    );
    assert!(committed?.committed_now());
    observed?;

    let reopened = DurablePreferenceStore::open_existing(&database_path).await?;
    assert_eq!(
        reopened.read_document(&preference, &subject).await?,
        Some(next)
    );
    Ok(())
}
