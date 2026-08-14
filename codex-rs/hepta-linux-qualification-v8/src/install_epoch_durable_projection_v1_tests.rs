use super::*;

fn completed(model_now: u64) -> VerifiedCommittedCurrentTipPreparationV1 {
    crate::install_epoch_completion_v1::test_only_completed_install_epoch_preparation_v1(model_now)
}

fn completed_after_retry(model_now: u64) -> VerifiedCommittedCurrentTipPreparationV1 {
    crate::install_epoch_completion_v1::test_only_completed_install_epoch_preparation_after_retry_v1(
        model_now,
    )
}

fn completed_after_retries(
    model_now: u64,
    retry_count: u64,
) -> VerifiedCommittedCurrentTipPreparationV1 {
    crate::install_epoch_completion_v1::test_only_completed_install_epoch_preparation_after_retries_v1(
        model_now,
        retry_count,
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
    mutate: impl FnOnce(&mut InstallEpochDurableProjectionSourceV2),
) {
    let mut source = completed(1_070).durable_projection_source_v2();
    mutate(&mut source);
    source.successor_tip_sha256 = external_watermark_record_sha256_v1(&source.successor_record);
    assert!(projection_from_source_v2(source).is_err());
}

fn assert_projection_source_is_rejected(
    source: InstallEpochDurableProjectionSourceV2,
    expected_message: &str,
) {
    let error = projection_from_source_v2(source).expect_err("tampered source must fail closed");
    assert!(
        error.to_string().contains(expected_message),
        "unexpected projection error: {error}"
    );
}

#[test]
fn completed_model_projects_exact_initial_claim_bundle_and_phase_bindings() {
    let projection = project_install_epoch_completion_for_durable_bridge_v2(completed(1_070))
        .expect("completed model should project");

    assert_eq!(
        projection.schema(),
        INSTALL_EPOCH_DURABLE_PROJECTION_SCHEMA_V2
    );
    assert_eq!(
        projection.profile(),
        INSTALL_EPOCH_DURABLE_PROJECTION_PROFILE_V2
    );
    #[allow(deprecated)]
    {
        assert_eq!(
            INSTALL_EPOCH_DURABLE_PROJECTION_SCHEMA_V1,
            "hepta_linux_v8_install_epoch_durable_projection_v1"
        );
        assert_ne!(
            INSTALL_EPOCH_DURABLE_PROJECTION_SCHEMA_V1,
            INSTALL_EPOCH_DURABLE_PROJECTION_SCHEMA_V2
        );
    }
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
    let projection = project_install_epoch_completion_for_durable_bridge_v2(completed(1_070))
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
    let first = project_install_epoch_completion_for_durable_bridge_v2(completed(1_070)).unwrap();
    let later = project_install_epoch_completion_for_durable_bridge_v2(completed(1_071)).unwrap();

    assert_eq!(first.job_id_sha256(), later.job_id_sha256());
    assert_eq!(first.projection_sha256(), later.projection_sha256());
}

#[test]
fn retry_projection_exports_the_active_claim_bundle_and_monotonic_phase() {
    let projection =
        project_install_epoch_completion_for_durable_bridge_v2(completed_after_retry(1_070))
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
fn every_bounded_retry_count_exports_complete_ordered_attempt_and_closure_history() {
    for retry_count in 0..=MAX_EXTERNAL_WATERMARK_CURRENT_TIP_RETRY_COUNT_V1 {
        let projection = project_install_epoch_completion_for_durable_bridge_v2(
            completed_after_retries(1_070, retry_count),
        )
        .expect("every bounded retry history should project");
        let attempts = projection.current_tip_attempts();
        let expected_attempt_count = retry_count.checked_add(1).unwrap();

        assert_eq!(attempts.len() as u64, expected_attempt_count);
        assert_eq!(
            projection.phase().active_query_sequence(),
            expected_attempt_count
        );
        assert_eq!(
            projection.phase().active_query_revision(),
            expected_attempt_count.checked_add(3).unwrap()
        );
        assert_eq!(
            projection.phase().final_revision(),
            projection
                .phase()
                .active_query_revision()
                .checked_add(1)
                .unwrap()
        );

        let mut predecessor_revision = projection.phase().cas_receipt_revision();
        let mut predecessor_state = projection.phase().cas_receipt_state_sha256();
        for (index, attempt) in attempts.iter().enumerate() {
            let expected_sequence = u64::try_from(index).unwrap().checked_add(1).unwrap();
            assert_eq!(attempt.query_sequence(), expected_sequence);
            assert_eq!(attempt.phase_predecessor_revision(), predecessor_revision);
            assert_eq!(attempt.phase_predecessor_state_sha256(), predecessor_state);
            assert_eq!(
                attempt.phase_successor_revision(),
                predecessor_revision.checked_add(1).unwrap()
            );
            assert_eq!(
                attempt.query_bundle().id_sha256(),
                attempt.query_claim().binding_sha256()
            );
            assert_eq!(
                attempt.query_bundle().binding_sha256(),
                attempt.phase_successor_state_sha256()
            );

            match (index.checked_sub(1), attempt.predecessor_closure()) {
                (None, None) => {}
                (Some(predecessor_index), Some(closure)) => {
                    let predecessor = &attempts[predecessor_index];
                    assert_eq!(closure.query_sequence(), predecessor.query_sequence());
                    assert_eq!(closure.query_nonce(), predecessor.query_claim().nonce());
                    assert_eq!(
                        closure.query_claim_binding_sha256(),
                        predecessor.query_claim().binding_sha256()
                    );
                    assert_eq!(
                        closure.query_phase_revision(),
                        predecessor.phase_successor_revision()
                    );
                    assert_eq!(
                        closure.query_state_sha256(),
                        predecessor.phase_successor_state_sha256()
                    );
                    assert_eq!(
                        closure.completion_operation_binding_sha256(),
                        projection.completion_operation_binding_sha256()
                    );
                    assert_eq!(
                        closure.phase_head_id_sha256(),
                        projection.phase().head_id_sha256()
                    );
                    assert_eq!(
                        closure.provider_transaction_sha256(),
                        projection.provider_transaction_sha256()
                    );
                    for digest in [
                        closure.closure_binding_sha256(),
                        closure.closure_evidence_sha256(),
                        closure.closure_profile_sha256(),
                    ] {
                        assert_digest(digest);
                    }
                }
                disposition => panic!("invalid closure disposition: {disposition:?}"),
            }

            predecessor_revision = attempt.phase_successor_revision();
            predecessor_state = attempt.phase_successor_state_sha256();
        }

        assert_eq!(
            projection.initial_query_claim().nonce(),
            attempts.first().unwrap().query_claim().nonce()
        );
        assert_eq!(
            projection.active_query_claim().nonce(),
            attempts.last().unwrap().query_claim().nonce()
        );
        assert_eq!(
            projection.phase().active_query_state_sha256(),
            attempts.last().unwrap().phase_successor_state_sha256()
        );
    }
}

#[test]
fn truncated_reordered_and_duplicated_attempt_histories_are_rejected() {
    let mut truncated = completed_after_retries(1_070, 3).durable_projection_source_v2();
    let mut attempts = std::mem::replace(
        &mut truncated.current_tip_attempts,
        Vec::new().into_boxed_slice(),
    )
    .into_vec();
    attempts.pop().expect("terminal attempt");
    truncated.current_tip_attempts = attempts.into_boxed_slice();
    assert_projection_source_is_rejected(truncated, "attempt count is not exact");

    let mut reordered = completed_after_retries(1_070, 3).durable_projection_source_v2();
    reordered.current_tip_attempts.swap(1, 2);
    assert_projection_source_is_rejected(reordered, "attempt phase edge is not exact");

    let mut duplicated = completed_after_retries(1_070, 2).durable_projection_source_v2();
    let donor = completed_after_retries(1_070, 0).durable_projection_source_v2();
    let duplicate_initial = donor.current_tip_attempts.into_vec().remove(0);
    duplicated.current_tip_attempts[1] = duplicate_initial;
    assert_projection_source_is_rejected(duplicated, "attempt phase edge is not exact");
}

#[test]
fn spliced_closure_and_self_consistent_closure_or_nonce_tampering_are_rejected() {
    let mut spliced = completed_after_retries(1_070, 3).durable_projection_source_v2();
    let mut donor = completed_after_retries(1_070, 3).durable_projection_source_v2();
    let closure_from_later_attempt = donor.current_tip_attempts[3]
        .predecessor_closure
        .take()
        .expect("later predecessor closure");
    spliced.current_tip_attempts[1].predecessor_closure = Some(closure_from_later_attempt);
    assert_projection_source_is_rejected(spliced, "query closure is not exactly cross-bound");

    let mut closure_tampered = completed_after_retries(1_070, 1).durable_projection_source_v2();
    let closure = closure_tampered.current_tip_attempts[1]
        .predecessor_closure
        .as_mut()
        .expect("retry predecessor closure");
    closure.closure_evidence_sha256 = different_digest(&closure.closure_evidence_sha256);
    closure.closure_binding_sha256 = query_closure_binding_sha256_v1(closure);
    assert_projection_source_is_rejected(
        closure_tampered,
        "current-tip claim/state binding is not exact",
    );

    let mut nonce_tampered = completed_after_retries(1_070, 2).durable_projection_source_v2();
    let active = nonce_tampered
        .current_tip_attempts
        .last_mut()
        .expect("active attempt");
    let replacement_nonce = "f".repeat(64);
    assert_ne!(replacement_nonce, active.query_claim.nonce);
    active.query_claim.nonce = replacement_nonce.clone();
    nonce_tampered.current_tip_query_nonce = replacement_nonce;
    assert_projection_source_is_rejected(
        nonce_tampered,
        "current-tip claim/state binding is not exact",
    );
}

#[test]
fn every_receipt_or_phase_identity_change_changes_projection_digest() {
    let verified = completed(1_070);
    let source = verified.durable_projection_source_v2();
    let job_id = projection_job_id_sha256_v2(&source);
    let baseline = projection_binding_sha256_v2(&source, &job_id);

    let mut changed_commit = completed(1_070).durable_projection_source_v2();
    changed_commit.commit_signature_sha256 = "a".repeat(64);
    assert_ne!(
        baseline,
        projection_binding_sha256_v2(&changed_commit, &job_id)
    );

    let mut changed_query = completed(1_070).durable_projection_source_v2();
    changed_query.active_query_state_sha256 = "b".repeat(64);
    assert_ne!(
        baseline,
        projection_binding_sha256_v2(&changed_query, &job_id)
    );

    let mut changed_final = completed(1_070).durable_projection_source_v2();
    changed_final.finalized_state_sha256 = "c".repeat(64);
    assert_ne!(
        baseline,
        projection_binding_sha256_v2(&changed_final, &job_id)
    );

    let mut changed_tip = completed(1_070).durable_projection_source_v2();
    changed_tip.successor_tip_sha256 = "d".repeat(64);
    assert_ne!(
        baseline,
        projection_binding_sha256_v2(&changed_tip, &job_id)
    );
}

#[test]
fn job_identity_is_stable_for_receipts_but_changes_for_operation_identity() {
    let source = completed(1_070).durable_projection_source_v2();
    let baseline = projection_job_id_sha256_v2(&source);

    let mut changed_receipt = completed(1_070).durable_projection_source_v2();
    changed_receipt.current_tip_signature_sha256 = "a".repeat(64);
    assert_eq!(baseline, projection_job_id_sha256_v2(&changed_receipt));

    let mut changed_operation = completed(1_070).durable_projection_source_v2();
    changed_operation.completion_operation_binding_sha256 = "b".repeat(64);
    assert_ne!(baseline, projection_job_id_sha256_v2(&changed_operation));
}

#[test]
fn malformed_phase_or_cross_bound_successor_is_rejected_fail_closed() {
    let mut phase_splice = completed(1_070).durable_projection_source_v2();
    phase_splice.final_phase_revision = phase_splice.active_query_revision;
    assert!(projection_from_source_v2(phase_splice).is_err());

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

    let mut retry_splice = completed_after_retry(1_070).durable_projection_source_v2();
    retry_splice
        .current_tip_attempts
        .last_mut()
        .expect("retry attempt")
        .query_bundle
        .binding_sha256 = "e".repeat(64);
    assert!(projection_from_source_v2(retry_splice).is_err());

    let mut revision_overflow = completed(1_070).durable_projection_source_v2();
    match &mut revision_overflow.predecessor {
        ExternalWatermarkPredecessorV1::GenesisPinnedSentinel { revision, .. }
        | ExternalWatermarkPredecessorV1::Successor { revision, .. } => *revision = u64::MAX,
    }
    revision_overflow.successor_record.predecessor = revision_overflow.predecessor.clone();
    revision_overflow.successor_tip_sha256 =
        external_watermark_record_sha256_v1(&revision_overflow.successor_record);
    assert!(projection_from_source_v2(revision_overflow).is_err());
}

#[test]
fn golden_projection_identities_are_stable() {
    let projection = project_install_epoch_completion_for_durable_bridge_v2(completed(1_070))
        .expect("completed model should project");

    assert_eq!(
        projection.job_id_sha256(),
        "4ec08de21f8bd1bfd0afaf1a87d0fdb6ad1055cb9f3c6d06a8eb64aaeb80452e"
    );
    assert_eq!(
        projection.projection_sha256(),
        "fa75ee5b665fe18d25cc97eecba32460f653b1b1dd588177fc6d80144f249895"
    );
}
