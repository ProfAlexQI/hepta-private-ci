fn request_digest(request: &IntelligenceMutationTransitionRequest) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(&mut hasher, REQUEST_DOMAIN);
    frame_part(
        &mut hasher,
        &INTELLIGENCE_MUTATION_STATE_SCHEMA_VERSION.to_be_bytes(),
    );
    frame_part(
        &mut hasher,
        INTELLIGENCE_MUTATION_STATE_NAMESPACE.as_bytes(),
    );
    frame_binding(&mut hasher, &request.binding);
    frame_part(&mut hasher, &request.sequence.to_be_bytes());
    frame_part(
        &mut hasher,
        request.causal_parent_sha256.as_str().as_bytes(),
    );
    frame_action(&mut hasher, &request.action);
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn transition_digest(receipt: &IntelligenceMutationTransitionReceipt) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(&mut hasher, TRANSITION_DOMAIN);
    frame_part(&mut hasher, &receipt.schema_version.to_be_bytes());
    frame_part(&mut hasher, receipt.namespace.as_bytes());
    frame_part(&mut hasher, receipt.operation_id.as_str().as_bytes());
    frame_part(&mut hasher, &receipt.sequence.to_be_bytes());
    frame_part(&mut hasher, receipt.from_phase.as_str().as_bytes());
    frame_part(&mut hasher, receipt.to_phase.as_str().as_bytes());
    frame_part(&mut hasher, receipt.action.as_bytes());
    frame_part(&mut hasher, receipt.request_sha256.as_str().as_bytes());
    frame_part(
        &mut hasher,
        receipt.causal_parent_sha256.as_str().as_bytes(),
    );
    frame_part(
        &mut hasher,
        receipt.intent_disposition.as_str().as_bytes(),
    );
    frame_part(&mut hasher, &[receipt.memory_write_count]);
    frame_part(&mut hasher, &[receipt.projection_publish_count]);
    frame_part(&mut hasher, &[u8::from(receipt.outbox_settled)]);
    frame_part(
        &mut hasher,
        &receipt.last_published_generation.to_be_bytes(),
    );
    frame_optional_phase(&mut hasher, receipt.indeterminate_from);
    frame_optional_phase(&mut hasher, receipt.last_recovery_origin);
    frame_part(&mut hasher, &[receipt.reconciliation_count]);
    frame_part(&mut hasher, &[u8::from(receipt.runtime_wired)]);
    frame_part(&mut hasher, &[u8::from(receipt.qualified)]);
    frame_part(&mut hasher, &[u8::from(receipt.sqlite_persistence)]);
    frame_part(&mut hasher, &[u8::from(receipt.external_effects)]);
    frame_part(
        &mut hasher,
        &[u8::from(receipt.production_authority)],
    );
    frame_part(
        &mut hasher,
        &[u8::from(receipt.operator_acceptance)],
    );
    frame_part(&mut hasher, &[u8::from(receipt.promotion)]);
    frame_part(&mut hasher, &[u8::from(receipt.callers_ratchet)]);
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn verify_transition_receipt(
    receipt: &IntelligenceMutationTransitionReceipt,
) -> Result<(), IntelligenceMutationStateError> {
    if receipt.schema_version != INTELLIGENCE_MUTATION_STATE_SCHEMA_VERSION
        || receipt.namespace != INTELLIGENCE_MUTATION_STATE_NAMESPACE
    {
        return Err(IntelligenceMutationStateError::ReceiptSchemaMismatch);
    }
    if receipt.runtime_wired
        || receipt.qualified
        || receipt.sqlite_persistence
        || receipt.external_effects
        || receipt.production_authority
        || receipt.operator_acceptance
        || receipt.promotion
        || receipt.callers_ratchet
    {
        return Err(IntelligenceMutationStateError::AuthorityEscalation);
    }
    validate_digest(&receipt.operation_id, "receipt operation id")?;
    validate_digest(&receipt.request_sha256, "receipt request digest")?;
    validate_digest(
        &receipt.causal_parent_sha256,
        "receipt causal parent digest",
    )?;
    validate_digest(&receipt.transition_sha256, "transition digest")?;
    if transition_digest(receipt) != receipt.transition_sha256 {
        return Err(IntelligenceMutationStateError::ReceiptDigestMismatch);
    }
    Ok(())
}
