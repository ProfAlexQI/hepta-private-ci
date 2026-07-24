use crate::Authorization;
use crate::AuthorizationDecision;
use crate::AuthorizationRef;
use crate::CandidateRef;
use crate::ContentHash;
use crate::ContractError;
use crate::PrincipalId;
use crate::ReceiptId;

/// Terminal execution status recorded by an outcome receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OutcomeStatus {
    /// The authorized execution completed successfully.
    Succeeded,
    /// Execution failed under the named machine-readable code.
    Failed {
        /// Stable failure code owned by the executing layer.
        error_code: String,
    },
    /// Execution was cancelled under the named machine-readable code.
    Cancelled {
        /// Stable cancellation code owned by the executing layer.
        reason_code: String,
    },
}

/// Exact reference to one immutable outcome receipt.
///
/// Receipt identity alone is never authoritative. `receipt_hash` binds the
/// complete canonical envelope, including authority, candidate, payload set,
/// executor, terminal status, and outcome digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptRef {
    id: ReceiptId,
    receipt_hash: ContentHash,
}

impl ReceiptRef {
    /// Creates an exact outcome-receipt reference.
    pub fn new(id: ReceiptId, receipt_hash: ContentHash) -> Self {
        Self { id, receipt_hash }
    }

    /// Returns the receipt identity.
    pub fn id(&self) -> &ReceiptId {
        &self.id
    }

    /// Returns the digest of the complete canonical receipt envelope.
    pub fn receipt_hash(&self) -> &ContentHash {
        &self.receipt_hash
    }
}

/// Immutable receipt binding an execution outcome to its exact authority chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutcomeReceipt {
    id: ReceiptId,
    receipt_hash: ContentHash,
    candidate: CandidateRef,
    authorization: AuthorizationRef,
    payload_set_hash: ContentHash,
    executed_by: PrincipalId,
    outcome_hash: ContentHash,
    status: OutcomeStatus,
}

/// Complete data parts required to rehydrate one persisted outcome receipt.
///
/// This is a data-recovery boundary, not an execution witness. In particular,
/// it carries only an [`AuthorizationRef`], cannot authorize execution, and
/// cannot prove that the referenced authorization remains current. The
/// producer-owned `receipt_hash` remains opaque: rehydration preserves it but
/// does not claim to recalculate a digest whose canonicalization belongs to
/// another layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutcomeReceiptParts {
    id: ReceiptId,
    receipt_hash: ContentHash,
    candidate: CandidateRef,
    authorization: AuthorizationRef,
    payload_set_hash: ContentHash,
    executed_by: PrincipalId,
    outcome_hash: ContentHash,
    status: OutcomeStatus,
}

impl OutcomeReceiptParts {
    /// Creates untrusted parts for fallible receipt rehydration.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ReceiptId,
        receipt_hash: ContentHash,
        candidate: CandidateRef,
        authorization: AuthorizationRef,
        payload_set_hash: ContentHash,
        executed_by: PrincipalId,
        outcome_hash: ContentHash,
        status: OutcomeStatus,
    ) -> Self {
        Self {
            id,
            receipt_hash,
            candidate,
            authorization,
            payload_set_hash,
            executed_by,
            outcome_hash,
            status,
        }
    }

    /// Returns the receipt identity.
    pub fn id(&self) -> &ReceiptId {
        &self.id
    }

    /// Returns the producer-owned opaque receipt-envelope digest.
    pub fn receipt_hash(&self) -> &ContentHash {
        &self.receipt_hash
    }

    /// Returns the exact executed candidate.
    pub fn candidate(&self) -> &CandidateRef {
        &self.candidate
    }

    /// Returns the exact referenced commit-time authorization.
    pub fn authorization(&self) -> &AuthorizationRef {
        &self.authorization
    }

    /// Returns the digest of the exact executed payload set.
    pub fn payload_set_hash(&self) -> &ContentHash {
        &self.payload_set_hash
    }

    /// Returns the accountable executing principal.
    pub fn executed_by(&self) -> &PrincipalId {
        &self.executed_by
    }

    /// Returns the digest of the canonical execution outcome.
    pub fn outcome_hash(&self) -> &ContentHash {
        &self.outcome_hash
    }

    /// Returns the terminal execution status.
    pub fn status(&self) -> &OutcomeStatus {
        &self.status
    }
}

impl OutcomeReceipt {
    /// Creates an immutable outcome receipt from a commit-time authorization.
    ///
    /// The producer computes `receipt_hash` over the complete canonical receipt
    /// envelope. Candidate, authorization, and payload bindings are copied from
    /// the authorization so they cannot disagree inside the constructed value.
    /// A denied authorization cannot produce an execution receipt.
    pub fn try_new(
        id: ReceiptId,
        receipt_hash: ContentHash,
        authorization: &Authorization,
        executed_by: PrincipalId,
        outcome_hash: ContentHash,
        status: OutcomeStatus,
    ) -> Result<Self, ContractError> {
        if !matches!(
            authorization.decision(),
            AuthorizationDecision::Authorized { .. }
        ) {
            return Err(ContractError::AuthorizationNotAuthorized);
        }
        validate_terminal_status(&status)?;

        Ok(Self {
            id,
            receipt_hash,
            candidate: authorization.candidate().clone(),
            authorization: authorization.reference(),
            payload_set_hash: authorization.payload_set_hash().clone(),
            executed_by,
            outcome_hash,
            status,
        })
    }

    /// Rehydrates an immutable receipt from untrusted persisted data parts.
    ///
    /// This validates every relationship available without the original
    /// execution witness: the candidate and receipt must bind the same payload
    /// set, and failure/cancellation statuses must carry non-empty stable
    /// codes. The producer-owned `receipt_hash` is intentionally not
    /// recalculated here; a persistence layer must separately protect and
    /// verify its own canonical row representation.
    pub fn try_rehydrate(parts: OutcomeReceiptParts) -> Result<Self, ContractError> {
        if parts.candidate.payload_set_hash() != &parts.payload_set_hash {
            return Err(ContractError::OutcomePayloadSetMismatch);
        }
        validate_terminal_status(&parts.status)?;

        Ok(Self {
            id: parts.id,
            receipt_hash: parts.receipt_hash,
            candidate: parts.candidate,
            authorization: parts.authorization,
            payload_set_hash: parts.payload_set_hash,
            executed_by: parts.executed_by,
            outcome_hash: parts.outcome_hash,
            status: parts.status,
        })
    }

    /// Returns complete data parts suitable for a persistence codec.
    ///
    /// The returned value is not an authorization or execution witness.
    pub fn rehydration_parts(&self) -> OutcomeReceiptParts {
        OutcomeReceiptParts::new(
            self.id.clone(),
            self.receipt_hash.clone(),
            self.candidate.clone(),
            self.authorization.clone(),
            self.payload_set_hash.clone(),
            self.executed_by.clone(),
            self.outcome_hash.clone(),
            self.status.clone(),
        )
    }

    /// Returns the receipt identity.
    pub fn id(&self) -> &ReceiptId {
        &self.id
    }

    /// Returns the digest of the complete canonical receipt envelope.
    pub fn receipt_hash(&self) -> &ContentHash {
        &self.receipt_hash
    }

    /// Returns the exact executed candidate.
    pub fn candidate(&self) -> &CandidateRef {
        &self.candidate
    }

    /// Returns the exact commit-time authorization used for execution.
    pub fn authorization(&self) -> &AuthorizationRef {
        &self.authorization
    }

    /// Returns the digest of the exact executed payload set.
    pub fn payload_set_hash(&self) -> &ContentHash {
        &self.payload_set_hash
    }

    /// Returns the accountable executing principal.
    pub fn executed_by(&self) -> &PrincipalId {
        &self.executed_by
    }

    /// Returns the digest of the canonical execution outcome.
    pub fn outcome_hash(&self) -> &ContentHash {
        &self.outcome_hash
    }

    /// Returns the terminal outcome status.
    pub fn status(&self) -> &OutcomeStatus {
        &self.status
    }

    /// Returns an exact reference to this immutable receipt.
    pub fn reference(&self) -> ReceiptRef {
        ReceiptRef::new(self.id.clone(), self.receipt_hash.clone())
    }
}

fn validate_terminal_status(status: &OutcomeStatus) -> Result<(), ContractError> {
    match status {
        OutcomeStatus::Succeeded => Ok(()),
        OutcomeStatus::Failed { error_code } if error_code.trim().is_empty() => {
            Err(ContractError::OutcomeFailureCodeEmpty)
        }
        OutcomeStatus::Cancelled { reason_code } if reason_code.trim().is_empty() => {
            Err(ContractError::OutcomeCancellationCodeEmpty)
        }
        OutcomeStatus::Failed { .. } | OutcomeStatus::Cancelled { .. } => Ok(()),
    }
}
