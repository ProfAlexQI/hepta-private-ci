use super::*;

fn completed(model_now: u64) -> VerifiedCommittedCurrentTipPreparationV1 {
    crate::install_epoch_completion_v1::test_only_completed_install_epoch_preparation_v1(model_now)
}

fn completed_after_retry(model_now: u64) -> VerifiedCommittedCurrentTipPreparationV1 {
    crate::install_epoch_completion_v1::test_only_completed_install_epoch_preparation_after_retry_v1(
        model_now,
    )
}

fn assert_digest(value: &str) {
    assert_eq!(value.len(), 64);
    assert_ne!(value, "0".repeat(64));
    assert!(
        value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    );
}

fn different_digest(value: &str) -> String {
    let replacement = if value.starts_with('a') { 'b' } else { 'a' };
    replacement.to_string().repeat(64)
}

fn assert_cross_bound_successor_mutation_is_rejected(
    mutate: impl FnOnce(&mut InstallEpochDurableProjectionSourceV1),
) {
    let mut source = completed(1_070).durable_projection_source_v1();
    mutate(&mut source);
    source.successor_tip_sha256 = external_watermark_record_sha256_v1(&source.successor_record);
    assert!(projection_from_source_v1(source).is_err());
}

#[test]
fn completed_model_projects_exact_initial_claim_bundle_and_phase_bindings() {
    let projection = project_install_epoch_completion_for_durable_bridge_v1(completed(1_070))
        .expect("completed model should project");

    assert_eq!(projection.claim_domain_id(), EXPECTED_CLAIM_DOMAIN_V1);
    assert_eq!(
        projection.authority_claim().scope(),
        EXPECTED_AUTHORITY_CLAIM_SCOPE_V1
    );
    assert_eq!(
        projection.lease_claim().scope(),
        EXPECTED_LEASE_CLAIM_SCOPE_V1
    );
    assert_eq!(
        projection.commit_claim().scope(),
        EXPECTED_COMMIT_CLAIM_SCOPE_V1
    );
    assert_eq!(
        projection.initial_query_claim().scope(),
        EXPECTED_QUERY_CLAIM_SCOPE_V1
    );
    assert_eq!(
        projection.active_query_claim().nonce(),
        projection.initial_query_claim().nonce()
    );
    assert!(projection.retry_query_claim().is_none());
    assert!(projection.active_query_bundle().is_none());
    assert_eq!(projection.phase().cas_intent_revision(), 1);
    assert_eq!(projection.phase().cas_issue_revision(), 2);
    assert_eq!(projection.phase().cas_receipt_revision(), 3);
    assert_eq!(projection.phase().active_query_revision(), 4);
    assert_eq!(projection.phase().active_query_sequence(), 1);
    assert_eq!(projection.phase().final_revision(), 5);
    assert_eq!(
        projection.completion_bundle().binding_sha256(),
        projection.phase().cas_intent_state_sha256()
    );
    assert_eq!(
        projection.current_tip_query_nonce(),
        projection.initial_query_claim().nonce()
    );
    assert_eq!(
        projection.successor_tip_sha256(),
        external_watermark_record_sha256_v1(projection.successor_record())
    );
    for digest in [
        projection.job_id_sha256(),
        projection.projection_sha256(),
        projection.preparation_bundle().id_sha256(),
        projection.preparation_bundle().binding_sha256(),
        projection.completion_bundle().id_sha256(),
        projection.phase().head_id_sha256(),
        projection.phase().final_state_sha256(),
    ] {
        assert_digest(digest);
    }
}

#[test]
fn projection_is_inert_and_grants_no_live_or_privileged_authority() {
    let projection = project_install_epoch_completion_for_durable_bridge_v1(completed(1_070))
        .expect("completed model should project");

    assert!(!projection.durable_publication_complete());
    assert!(!projection.provider_io_allowed());
    assert!(!projection.root_install_execution_allowed());
    assert!(!projection.daemon_reload_enable_or_start_allowed());
    assert!(!projection.trusted_state_root_established());
    assert!(!projection.fresh_attempt_allowed());
}

#[test]
fn local_model_completion_time_and_freshness_do_not_fork_durable_identity() {
    let first = project_install_epoch_completion_for_durable_bridge_v1(completed(1_070)).unwrap();
    let later = project_install_epoch_completion_for_durable_bridge_v1(completed(1_071)).unwrap();

    assert_eq!(first.job_id_sha256(), later.job_id_sha256());
    assert_eq!(first.projection_sha256(), later.projection_sha256());
}

#[test]
fn retry_projection_exports_the_active_claim_bundle_and_monotonic_phase() {
    let projection =
        project_install_epoch_completion_for_durable_bridge_v1(completed_after_retry(1_070))
            .expect("completed retry model should project");
    let retry = projection.retry_query_claim().expect("retry claim");
    let bundle = projection.active_query_bundle().expect("retry bundle");

    assert_eq!(projection.phase().active_query_sequence(), 2);
    assert_eq!(projection.phase().active_query_revision(), 5);
    assert_eq!(projection.phase().final_revision(), 6);
    assert_eq!(projection.active_query_claim().nonce(), retry.nonce());
    assert_eq!(projection.current_tip_query_nonce(), retry.nonce());
    assert_ne!(retry.nonce(), projection.initial_query_claim().nonce());
    assert_eq!(bundle.id_sha256(), retry.binding_sha256());
    assert_eq!(
        bundle.binding_sha256(),
        projection.phase().active_query_state_sha256()
    );
}

#[test]
fn every_receipt_or_phase_identity_change_changes_projection_digest() {
    let verified = completed(1_070);
    let source = verified.durable_projection_source_v1();
    let job_id = projection_job_id_sha256_v1(&source);
    let baseline = projection_binding_sha256_v1(&source, &job_id);

    let mut changed_commit = completed(1_070).durable_projection_source_v1();
    changed_commit.commit_signature_sha256 = "a".repeat(64);
    assert_ne!(
        baseline,
        projection_binding_sha256_v1(&changed_commit, &job_id)
    );

    let mut changed_query = completed(1_070).durable_projection_source_v1();
    changed_query.active_query_state_sha256 = "b".repeat(64);
    assert_ne!(
        baseline,
        projection_binding_sha256_v1(&changed_query, &job_id)
    );

    let mut changed_final = completed(1_070).durable_projection_source_v1();
    changed_final.finalized_state_sha256 = "c".repeat(64);
    assert_ne!(
        baseline,
        projection_binding_sha256_v1(&changed_final, &job_id)
    );

    let mut changed_tip = completed(1_070).durable_projection_source_v1();
    changed_tip.successor_tip_sha256 = "d".repeat(64);
    assert_ne!(
        baseline,
        projection_binding_sha256_v1(&changed_tip, &job_id)
    );
}

#[test]
fn job_identity_is_stable_for_receipts_but_changes_for_operation_identity() {
    let source = completed(1_070).durable_projection_source_v1();
    let baseline = projection_job_id_sha256_v1(&source);

    let mut changed_receipt = completed(1_070).durable_projection_source_v1();
    changed_receipt.current_tip_signature_sha256 = "a".repeat(64);
    assert_eq!(baseline, projection_job_id_sha256_v1(&changed_receipt));

    let mut changed_operation = completed(1_070).durable_projection_source_v1();
    changed_operation.completion_operation_binding_sha256 = "b".repeat(64);
    assert_ne!(baseline, projection_job_id_sha256_v1(&changed_operation));
}

#[test]
fn malformed_phase_or_cross_bound_successor_is_rejected_fail_closed() {
    let mut phase_splice = completed(1_070).durable_projection_source_v1();
    phase_splice.final_phase_revision = phase_splice.active_query_revision;
    assert!(projection_from_source_v1(phase_splice).is_err());

    assert_cross_bound_successor_mutation_is_rejected(|source| {
        source.successor_record.machine_id_sha256 =
            different_digest(&source.successor_record.machine_id_sha256);
    });
    assert_cross_bound_successor_mutation_is_rejected(|source| {
        source.successor_record.prepared_epoch_binding_sha256 =
            different_digest(&source.successor_record.prepared_epoch_binding_sha256);
    });
    assert_cross_bound_successor_mutation_is_rejected(|source| {
        source.successor_record.provider_profile_sha256 =
            different_digest(&source.successor_record.provider_profile_sha256);
    });
    assert_cross_bound_successor_mutation_is_rejected(|source| {
        source.successor_record.state_root_profile_sha256 =
            different_digest(&source.successor_record.state_root_profile_sha256);
    });
    assert_cross_bound_successor_mutation_is_rejected(|source| {
        source.successor_record.stream_id_sha256 =
            different_digest(&source.successor_record.stream_id_sha256);
    });
    assert_cross_bound_successor_mutation_is_rejected(|source| {
        source.successor_record.successor_revision = source
            .successor_record
            .successor_revision
            .checked_add(1)
            .expect("fixture revision must not overflow");
    });

    let mut retry_splice = completed_after_retry(1_070).durable_projection_source_v1();
    retry_splice.active_query_bundle = None;
    assert!(projection_from_source_v1(retry_splice).is_err());

    let mut revision_overflow = completed(1_070).durable_projection_source_v1();
    match &mut revision_overflow.predecessor {
        ExternalWatermarkPredecessorV1::GenesisPinnedSentinel { revision, .. }
        | ExternalWatermarkPredecessorV1::Successor { revision, .. } => *revision = u64::MAX,
    }
    revision_overflow.successor_record.predecessor = revision_overflow.predecessor.clone();
    revision_overflow.successor_tip_sha256 =
        external_watermark_record_sha256_v1(&revision_overflow.successor_record);
    assert!(projection_from_source_v1(revision_overflow).is_err());
}

#[test]
fn golden_projection_identities_are_stable() {
    let projection = project_install_epoch_completion_for_durable_bridge_v1(completed(1_070))
        .expect("completed model should project");

    assert_eq!(
        projection.job_id_sha256(),
        "8b5d08912c844c864ddf8218f4c7ab9c4601402de7699d3a0ba5f3fc43565d4b"
    );
    assert_eq!(
        projection.projection_sha256(),
        "62b6801c2fbd9e73e89469ff95126ff0386edb494f01cfd2df9569fcfbc1b61e"
    );
}
