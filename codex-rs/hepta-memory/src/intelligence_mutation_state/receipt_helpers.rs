fn frame_binding(hasher: &mut Sha256, binding: &IntelligenceMutationBinding) {
    frame_part(hasher, binding.operation_id.as_str().as_bytes());
    frame_part(hasher, binding.lease_id.as_bytes());
    frame_part(hasher, &binding.lease_epoch.to_be_bytes());
    frame_optional_u64(hasher, binding.expected_revision);
    frame_part(
        hasher,
        &binding.starting_projection_generation.to_be_bytes(),
    );
    frame_part(hasher, binding.causal_root_sha256.as_str().as_bytes());
}

fn frame_action(hasher: &mut Sha256, action: &IntelligenceMutationAction) {
    frame_part(hasher, action.kind().as_bytes());
    match action {
        IntelligenceMutationAction::WitnessSource { source_sha256 } => {
            frame_part(hasher, source_sha256.as_str().as_bytes());
        }
        IntelligenceMutationAction::ValidateGrounding {
            grounding_receipt_sha256,
        } => frame_part(hasher, grounding_receipt_sha256.as_str().as_bytes()),
        IntelligenceMutationAction::RejectPreCommit { reason_sha256 }
        | IntelligenceMutationAction::CancelPreCommit { reason_sha256 }
        | IntelligenceMutationAction::MarkIndeterminate { reason_sha256 }
        | IntelligenceMutationAction::Quarantine { reason_sha256 } => {
            frame_part(hasher, reason_sha256.as_str().as_bytes());
        }
        IntelligenceMutationAction::AppendDurableIntent { intent_sha256 } => {
            frame_part(hasher, intent_sha256.as_str().as_bytes());
        }
        IntelligenceMutationAction::CommitMemoryFacts {
            write_receipt_sha256,
        } => frame_part(hasher, write_receipt_sha256.as_str().as_bytes()),
        IntelligenceMutationAction::PublishProjection {
            expected_previous_generation,
            new_generation,
            projection_receipt_sha256,
        } => {
            frame_part(hasher, &expected_previous_generation.to_be_bytes());
            frame_part(hasher, &new_generation.to_be_bytes());
            frame_part(hasher, projection_receipt_sha256.as_str().as_bytes());
        }
        IntelligenceMutationAction::SettleOutbox { outcome_sha256 } => {
            frame_part(hasher, outcome_sha256.as_str().as_bytes());
        }
        IntelligenceMutationAction::Reconcile { observation } => {
            frame_reconciliation_observation(hasher, observation);
        }
        IntelligenceMutationAction::Terminalize => {}
    }
}

fn frame_reconciliation_observation(
    hasher: &mut Sha256,
    observation: &IntelligenceMutationReconciliationObservation,
) {
    frame_part(hasher, observation.kind().as_bytes());
    match observation {
        IntelligenceMutationReconciliationObservation::NotApplied { outcome_sha256 } => {
            frame_part(hasher, outcome_sha256.as_str().as_bytes());
        }
        IntelligenceMutationReconciliationObservation::MemoryFactsCommitted {
            write_receipt_sha256,
        } => frame_part(hasher, write_receipt_sha256.as_str().as_bytes()),
        IntelligenceMutationReconciliationObservation::ProjectionPublished {
            write_receipt_sha256,
            expected_previous_generation,
            new_generation,
            projection_receipt_sha256,
        } => {
            frame_part(hasher, write_receipt_sha256.as_str().as_bytes());
            frame_part(hasher, &expected_previous_generation.to_be_bytes());
            frame_part(hasher, &new_generation.to_be_bytes());
            frame_part(hasher, projection_receipt_sha256.as_str().as_bytes());
        }
        IntelligenceMutationReconciliationObservation::OutboxSettled {
            write_receipt_sha256,
            expected_previous_generation,
            new_generation,
            projection_receipt_sha256,
            outcome_sha256,
        } => {
            frame_part(hasher, write_receipt_sha256.as_str().as_bytes());
            frame_part(hasher, &expected_previous_generation.to_be_bytes());
            frame_part(hasher, &new_generation.to_be_bytes());
            frame_part(hasher, projection_receipt_sha256.as_str().as_bytes());
            frame_part(hasher, outcome_sha256.as_str().as_bytes());
        }
    }
}

fn frame_optional_u64(hasher: &mut Sha256, value: Option<u64>) {
    match value {
        Some(value) => {
            frame_part(hasher, b"some");
            frame_part(hasher, &value.to_be_bytes());
        }
        None => frame_part(hasher, b"none"),
    }
}

fn frame_optional_phase(hasher: &mut Sha256, value: Option<IntelligenceMutationPhase>) {
    match value {
        Some(value) => {
            frame_part(hasher, b"some");
            frame_part(hasher, value.as_str().as_bytes());
        }
        None => frame_part(hasher, b"none"),
    }
}

fn validate_id(value: &str, label: &str) -> Result<(), IntelligenceMutationStateError> {
    if value.trim().is_empty() || value.len() > MAX_ID_BYTES || value.as_bytes().contains(&0) {
        return Err(IntelligenceMutationStateError::Invalid(format!(
            "{label} must contain 1..={MAX_ID_BYTES} non-NUL bytes"
        )));
    }
    Ok(())
}

fn validate_lease_and_revision(
    lease_epoch: u64,
    expected_revision: Option<u64>,
) -> Result<(), IntelligenceMutationStateError> {
    if lease_epoch == 0 {
        return Err(IntelligenceMutationStateError::Invalid(
            "lease epoch must be positive".to_string(),
        ));
    }
    if expected_revision == Some(0) {
        return Err(IntelligenceMutationStateError::Invalid(
            "expected revision must be positive when present".to_string(),
        ));
    }
    Ok(())
}

fn validate_projection_step(
    expected_previous_generation: u64,
    new_generation: u64,
) -> Result<(), IntelligenceMutationStateError> {
    let expected_new = expected_previous_generation
        .checked_add(1)
        .ok_or(IntelligenceMutationStateError::ProjectionGenerationOverflow)?;
    if new_generation != expected_new {
        return Err(IntelligenceMutationStateError::StaleProjectionGeneration {
            expected: expected_new,
            received: new_generation,
        });
    }
    Ok(())
}

fn validate_digest(
    value: &Sha256Digest,
    label: &str,
) -> Result<(), IntelligenceMutationStateError> {
    Sha256Digest::parse(value.as_str().to_string()).map_err(|error| {
        IntelligenceMutationStateError::Invalid(format!("invalid {label}: {error}"))
    })?;
    Ok(())
}
