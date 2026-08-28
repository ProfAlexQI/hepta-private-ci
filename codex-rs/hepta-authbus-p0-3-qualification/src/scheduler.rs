use std::collections::BTreeMap;

use codex_hepta_contracts::QuotaVector;
use codex_hepta_contracts::Sha256Digest;

use crate::AUTHBUS_B4_P0_3_AUTHORITY;

const MAX_TEXT_BYTES: usize = 512;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CanonicalQuotaVector {
    pub request_count: u64,
    pub rpm: u64,
    pub tpm: u64,
    pub concurrency: u64,
    pub day_budget: u64,
    pub context: u64,
}

impl CanonicalQuotaVector {
    pub const fn new(
        request_count: u64,
        rpm: u64,
        tpm: u64,
        concurrency: u64,
        day_budget: u64,
        context: u64,
    ) -> Self {
        Self {
            request_count,
            rpm,
            tpm,
            concurrency,
            day_budget,
            context,
        }
    }

    pub fn from_legacy_b4(value: QuotaVector) -> Self {
        Self {
            request_count: 1,
            rpm: value.rpm,
            tpm: value.tpm,
            concurrency: value.concurrency,
            day_budget: value.day_budget,
            context: value.context,
        }
    }

    pub const fn to_legacy_b4(self) -> QuotaVector {
        QuotaVector::new(
            self.rpm,
            self.tpm,
            self.concurrency,
            self.day_budget,
            self.context,
        )
    }

    pub fn is_zero(self) -> bool {
        self.request_count == 0
            && self.rpm == 0
            && self.tpm == 0
            && self.concurrency == 0
            && self.day_budget == 0
            && self.context == 0
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self {
            request_count: self.request_count.checked_add(other.request_count)?,
            rpm: self.rpm.checked_add(other.rpm)?,
            tpm: self.tpm.checked_add(other.tpm)?,
            concurrency: self.concurrency.checked_add(other.concurrency)?,
            day_budget: self.day_budget.checked_add(other.day_budget)?,
            context: self.context.checked_add(other.context)?,
        })
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        Some(Self {
            request_count: self.request_count.checked_sub(other.request_count)?,
            rpm: self.rpm.checked_sub(other.rpm)?,
            tpm: self.tpm.checked_sub(other.tpm)?,
            concurrency: self.concurrency.checked_sub(other.concurrency)?,
            day_budget: self.day_budget.checked_sub(other.day_budget)?,
            context: self.context.checked_sub(other.context)?,
        })
    }

    pub fn fits_within(self, limit: Self) -> bool {
        self.request_count <= limit.request_count
            && self.rpm <= limit.rpm
            && self.tpm <= limit.tpm
            && self.concurrency <= limit.concurrency
            && self.day_budget <= limit.day_budget
            && self.context <= limit.context
    }

    pub fn terminal_usage(self) -> Self {
        Self {
            concurrency: 0,
            ..self
        }
    }

    pub fn digest(self) -> Sha256Digest {
        let mut bytes = Vec::new();
        push_text(&mut bytes, "hepta.authbus.quota-vector.v1");
        push_u64(&mut bytes, self.request_count);
        push_u64(&mut bytes, self.rpm);
        push_u64(&mut bytes, self.tpm);
        push_u64(&mut bytes, self.concurrency);
        push_u64(&mut bytes, self.day_budget);
        push_u64(&mut bytes, self.context);
        Sha256Digest::for_bytes(&bytes)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalQuotaLimits {
    pub request_count: Option<u64>,
    pub rpm: Option<u64>,
    pub tpm: Option<u64>,
    pub concurrency: Option<u64>,
    pub day_budget: Option<u64>,
    pub context: Option<u64>,
}

impl CanonicalQuotaLimits {
    pub const fn known(value: CanonicalQuotaVector) -> Self {
        Self {
            request_count: Some(value.request_count),
            rpm: Some(value.rpm),
            tpm: Some(value.tpm),
            concurrency: Some(value.concurrency),
            day_budget: Some(value.day_budget),
            context: Some(value.context),
        }
    }

    pub const fn unknown_request_count(value: CanonicalQuotaVector) -> Self {
        Self {
            request_count: None,
            rpm: Some(value.rpm),
            tpm: Some(value.tpm),
            concurrency: Some(value.concurrency),
            day_budget: Some(value.day_budget),
            context: Some(value.context),
        }
    }

    pub fn is_fully_known(self) -> bool {
        self.request_count.is_some()
            && self.rpm.is_some()
            && self.tpm.is_some()
            && self.concurrency.is_some()
            && self.day_budget.is_some()
            && self.context.is_some()
    }

    fn can_hold(
        self,
        used: CanonicalQuotaVector,
        held: CanonicalQuotaVector,
        requested: CanonicalQuotaVector,
    ) -> bool {
        fn dimension(limit: Option<u64>, used: u64, held: u64, requested: u64) -> bool {
            let Some(limit) = limit else {
                return false;
            };
            used.checked_add(held)
                .and_then(|committed| committed.checked_add(requested))
                .is_some_and(|total| total <= limit)
        }

        dimension(
            self.request_count,
            used.request_count,
            held.request_count,
            requested.request_count,
        ) && dimension(self.rpm, used.rpm, held.rpm, requested.rpm)
            && dimension(self.tpm, used.tpm, held.tpm, requested.tpm)
            && dimension(
                self.concurrency,
                used.concurrency,
                held.concurrency,
                requested.concurrency,
            )
            && dimension(
                self.day_budget,
                used.day_budget,
                held.day_budget,
                requested.day_budget,
            )
            && dimension(self.context, used.context, held.context, requested.context)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P03Fence {
    pub authority_epoch: u64,
    pub owner_epoch: u64,
    pub generation: u64,
    pub fencing_token_sha256: Sha256Digest,
}

impl P03Fence {
    pub fn validate(&self) -> Result<(), P03SchedulerError> {
        if self.authority_epoch == 0
            || self.owner_epoch == 0
            || self.generation == 0
            || !valid_digest(&self.fencing_token_sha256)
        {
            return Err(P03SchedulerError::InvalidRequest);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum P03ResourceState {
    Available,
    Draining,
    Unavailable,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P03SchedulerResource {
    pub resource_id: String,
    pub resource_sha256: Sha256Digest,
    pub fence: P03Fence,
    pub quota: CanonicalQuotaLimits,
    pub state: P03ResourceState,
}

impl P03SchedulerResource {
    pub fn validate(&self) -> Result<(), P03SchedulerError> {
        validate_text(&self.resource_id)?;
        if !valid_digest(&self.resource_sha256) {
            return Err(P03SchedulerError::InvalidRequest);
        }
        self.fence.validate()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P03ReservationRequest {
    pub request_id: String,
    pub command_id: String,
    pub idempotency_key: String,
    pub payload_sha256: Sha256Digest,
    pub policy_sha256: Sha256Digest,
    pub resource_id: String,
    pub resource_sha256: Sha256Digest,
    pub estimate: CanonicalQuotaVector,
    pub safety_margin: CanonicalQuotaVector,
    pub fence: P03Fence,
    pub expected_revision: u64,
    pub created_at_ms: u64,
    pub expires_at_ms: u64,
}

impl P03ReservationRequest {
    fn validate_static(&self) -> Result<(), P03SchedulerError> {
        for value in [
            self.request_id.as_str(),
            self.command_id.as_str(),
            self.idempotency_key.as_str(),
            self.resource_id.as_str(),
        ] {
            validate_text(value)?;
        }
        for digest in [
            &self.payload_sha256,
            &self.policy_sha256,
            &self.resource_sha256,
        ] {
            if !valid_digest(digest) {
                return Err(P03SchedulerError::InvalidRequest);
            }
        }
        self.fence.validate()?;
        if self.expected_revision == 0
            || self.created_at_ms == 0
            || self.expires_at_ms <= self.created_at_ms
            || self.estimate.is_zero()
            || self.estimate.request_count == 0
            || self.estimate.checked_add(self.safety_margin).is_none()
        {
            return Err(P03SchedulerError::InvalidRequest);
        }
        Ok(())
    }

    pub fn binding_digest(&self) -> Result<Sha256Digest, P03SchedulerError> {
        self.validate_static()?;
        let mut bytes = Vec::new();
        push_text(&mut bytes, "hepta.authbus.b4.p0.3.reservation.v1");
        push_text(&mut bytes, &self.request_id);
        push_text(&mut bytes, &self.command_id);
        push_text(&mut bytes, &self.idempotency_key);
        push_digest(&mut bytes, &self.payload_sha256);
        push_digest(&mut bytes, &self.policy_sha256);
        push_text(&mut bytes, &self.resource_id);
        push_digest(&mut bytes, &self.resource_sha256);
        push_quota(&mut bytes, self.estimate);
        push_quota(&mut bytes, self.safety_margin);
        push_fence(&mut bytes, &self.fence);
        push_u64(&mut bytes, self.created_at_ms);
        push_u64(&mut bytes, self.expires_at_ms);
        Ok(Sha256Digest::for_bytes(&bytes))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P03SchedulerPermit {
    pub permit_id: String,
    pub request_id: String,
    pub command_id: String,
    pub idempotency_key: String,
    pub payload_sha256: Sha256Digest,
    pub resource_id: String,
    pub resource_sha256: Sha256Digest,
    pub reserved: CanonicalQuotaVector,
    pub fence: P03Fence,
    pub issued_at_ms: u64,
    pub expires_at_ms: u64,
    pub authority: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum P03ReservationState {
    ActiveReserved,
    DispatchStarted,
    OutcomeUnknown,
    Completed,
    Released,
    ExpiredPreDispatch,
}

impl P03ReservationState {
    fn is_active(self) -> bool {
        matches!(
            self,
            Self::ActiveReserved | Self::DispatchStarted | Self::OutcomeUnknown
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P03ReservationSnapshot {
    pub permit: P03SchedulerPermit,
    pub state: P03ReservationState,
    pub record_revision: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum P03AdmissionDisposition {
    Inserted(P03ReservationSnapshot),
    AlreadyPresent(P03ReservationSnapshot),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum P03WriteDisposition {
    Applied,
    AlreadyPresent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum P03ReconcileOutcome {
    VerifiedConsumed { actual: CanonicalQuotaVector },
    VerifiedNoEffect,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P03OldPermitReconcileRequest {
    pub permit_id: String,
    pub old_fence: P03Fence,
    pub current_fence: P03Fence,
    pub provider_status_receipt_sha256: Sha256Digest,
    pub owner_evidence_sha256: Sha256Digest,
    pub expected_revision: u64,
    pub observed_at_ms: u64,
    pub outcome: P03ReconcileOutcome,
}

impl P03OldPermitReconcileRequest {
    fn validate(&self) -> Result<(), P03SchedulerError> {
        validate_text(&self.permit_id)?;
        self.old_fence.validate()?;
        self.current_fence.validate()?;
        if self.old_fence == self.current_fence
            || self.expected_revision == 0
            || self.observed_at_ms == 0
            || !valid_digest(&self.provider_status_receipt_sha256)
            || !valid_digest(&self.owner_evidence_sha256)
        {
            return Err(P03SchedulerError::InvalidRequest);
        }
        if let P03ReconcileOutcome::VerifiedConsumed { actual } = self.outcome
            && actual.is_zero()
        {
            return Err(P03SchedulerError::InvalidRequest);
        }
        Ok(())
    }

    pub fn digest(&self) -> Result<Sha256Digest, P03SchedulerError> {
        self.validate()?;
        let mut bytes = Vec::new();
        push_text(&mut bytes, "hepta.authbus.b4.p0.3.old-permit-reconcile.v1");
        push_text(&mut bytes, &self.permit_id);
        push_fence(&mut bytes, &self.old_fence);
        push_fence(&mut bytes, &self.current_fence);
        push_digest(&mut bytes, &self.provider_status_receipt_sha256);
        push_digest(&mut bytes, &self.owner_evidence_sha256);
        push_u64(&mut bytes, self.expected_revision);
        push_u64(&mut bytes, self.observed_at_ms);
        match self.outcome {
            P03ReconcileOutcome::VerifiedConsumed { actual } => {
                push_text(&mut bytes, "VERIFIED_CONSUMED");
                push_quota(&mut bytes, actual);
            }
            P03ReconcileOutcome::VerifiedNoEffect => {
                push_text(&mut bytes, "VERIFIED_NO_EFFECT");
            }
            P03ReconcileOutcome::Unknown => {
                push_text(&mut bytes, "UNKNOWN");
            }
        }
        Ok(Sha256Digest::for_bytes(&bytes))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum P03ReconcileResolution {
    Consumed,
    NoEffect,
    HeldUnknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P03OldPermitReconcileReceipt {
    pub request_sha256: Sha256Digest,
    pub permit_id: String,
    pub resolution: P03ReconcileResolution,
    pub actual: Option<CanonicalQuotaVector>,
    pub before_revision: u64,
    pub after_revision: u64,
    pub held_after: CanonicalQuotaVector,
    pub used_after: CanonicalQuotaVector,
    pub observed_at_ms: u64,
    pub authority: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum P03ReconcileDisposition {
    Applied(P03OldPermitReconcileReceipt),
    AlreadyPresent(P03OldPermitReconcileReceipt),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P03ExpiryReport {
    pub released_pre_dispatch: Vec<P03ReservationSnapshot>,
    pub held_for_reconcile: Vec<P03ReservationSnapshot>,
    pub revision: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum P03SchedulerError {
    InvalidRequest,
    RequestConflict,
    IdempotencyConflict,
    BindingConflict,
    UnknownQuota,
    QuotaExceeded,
    ResourceUnavailable,
    StaleRevision,
    StaleFence,
    UnknownPermit,
    InvalidTransition,
    UsageOverrun,
    ObservationConflict,
    ReconcileConflict,
    TerminalImmutable,
    CorruptState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct IdempotencyRecord {
    request_id: String,
    request_sha256: Sha256Digest,
    payload_sha256: Sha256Digest,
    snapshot: P03ReservationSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ActivePermit {
    permit: P03SchedulerPermit,
    state: P03ReservationState,
    last_unknown_evidence_sha256: Option<Sha256Digest>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct P03LocalScheduler {
    resource: P03SchedulerResource,
    revision: u64,
    next_permit_nonce: u64,
    used: CanonicalQuotaVector,
    held: CanonicalQuotaVector,
    request_bindings: BTreeMap<String, String>,
    idempotency: BTreeMap<String, IdempotencyRecord>,
    active: BTreeMap<String, ActivePermit>,
    reconcile_history: BTreeMap<String, P03OldPermitReconcileReceipt>,
    terminal_reconcile_by_permit: BTreeMap<String, String>,
}

impl P03LocalScheduler {
    pub fn new(resource: P03SchedulerResource) -> Result<Self, P03SchedulerError> {
        resource.validate()?;
        Ok(Self {
            resource,
            revision: 1,
            next_permit_nonce: 1,
            used: CanonicalQuotaVector::default(),
            held: CanonicalQuotaVector::default(),
            request_bindings: BTreeMap::new(),
            idempotency: BTreeMap::new(),
            active: BTreeMap::new(),
            reconcile_history: BTreeMap::new(),
            terminal_reconcile_by_permit: BTreeMap::new(),
        })
    }

    pub fn resource(&self) -> &P03SchedulerResource {
        &self.resource
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn used(&self) -> CanonicalQuotaVector {
        self.used
    }

    pub fn held(&self) -> CanonicalQuotaVector {
        self.held
    }

    pub fn active_permit_count(&self) -> usize {
        self.active.len()
    }

    pub fn reservation_by_idempotency(
        &self,
        idempotency_key: &str,
    ) -> Option<&P03ReservationSnapshot> {
        self.idempotency
            .get(idempotency_key)
            .map(|record| &record.snapshot)
    }

    fn transactional<T>(
        &mut self,
        operation: impl FnOnce(&mut Self) -> Result<T, P03SchedulerError>,
    ) -> Result<T, P03SchedulerError> {
        let before = self.clone();
        let result = operation(self);
        if result.is_err() {
            *self = before;
        }
        result
    }

    pub fn reserve(
        &mut self,
        request: P03ReservationRequest,
    ) -> Result<P03AdmissionDisposition, P03SchedulerError> {
        self.transactional(|scheduler| scheduler.reserve_inner(request))
    }

    fn reserve_inner(
        &mut self,
        request: P03ReservationRequest,
    ) -> Result<P03AdmissionDisposition, P03SchedulerError> {
        request.validate_static()?;
        let request_sha256 = request.binding_digest()?;

        if let Some(existing) = self.idempotency.get(&request.idempotency_key) {
            if existing.payload_sha256 != request.payload_sha256 {
                return Err(P03SchedulerError::IdempotencyConflict);
            }
            if existing.request_id != request.request_id
                || existing.request_sha256 != request_sha256
            {
                return Err(P03SchedulerError::BindingConflict);
            }
            return Ok(P03AdmissionDisposition::AlreadyPresent(
                existing.snapshot.clone(),
            ));
        }

        if self.request_bindings.contains_key(&request.request_id) {
            return Err(P03SchedulerError::RequestConflict);
        }
        if request.expected_revision != self.revision {
            return Err(P03SchedulerError::StaleRevision);
        }
        if request.resource_id != self.resource.resource_id
            || request.resource_sha256 != self.resource.resource_sha256
            || request.fence != self.resource.fence
        {
            return Err(P03SchedulerError::StaleFence);
        }
        if self.resource.state != P03ResourceState::Available {
            return Err(P03SchedulerError::ResourceUnavailable);
        }
        if !self.resource.quota.is_fully_known() {
            return Err(P03SchedulerError::UnknownQuota);
        }
        let reserved = request
            .estimate
            .checked_add(request.safety_margin)
            .ok_or(P03SchedulerError::QuotaExceeded)?;
        if !self.resource.quota.can_hold(self.used, self.held, reserved) {
            return Err(P03SchedulerError::QuotaExceeded);
        }

        let next_revision = self
            .revision
            .checked_add(1)
            .ok_or(P03SchedulerError::InvalidRequest)?;
        let next_nonce = self
            .next_permit_nonce
            .checked_add(1)
            .ok_or(P03SchedulerError::InvalidRequest)?;
        let next_held = self
            .held
            .checked_add(reserved)
            .ok_or(P03SchedulerError::QuotaExceeded)?;
        let mut permit_preimage = Vec::new();
        push_text(&mut permit_preimage, "hepta.authbus.b4.p0.3.permit.v1");
        push_digest(&mut permit_preimage, &request_sha256);
        push_u64(&mut permit_preimage, self.next_permit_nonce);
        let permit_id = format!(
            "p03-permit:{}",
            Sha256Digest::for_bytes(&permit_preimage).as_str()
        );
        let permit = P03SchedulerPermit {
            permit_id: permit_id.clone(),
            request_id: request.request_id.clone(),
            command_id: request.command_id,
            idempotency_key: request.idempotency_key.clone(),
            payload_sha256: request.payload_sha256.clone(),
            resource_id: request.resource_id,
            resource_sha256: request.resource_sha256,
            reserved,
            fence: request.fence,
            issued_at_ms: request.created_at_ms,
            expires_at_ms: request.expires_at_ms,
            authority: AUTHBUS_B4_P0_3_AUTHORITY,
        };
        let snapshot = P03ReservationSnapshot {
            permit: permit.clone(),
            state: P03ReservationState::ActiveReserved,
            record_revision: next_revision,
        };
        self.next_permit_nonce = next_nonce;
        self.held = next_held;
        self.revision = next_revision;
        self.request_bindings
            .insert(request.request_id.clone(), request.idempotency_key.clone());
        self.idempotency.insert(
            request.idempotency_key,
            IdempotencyRecord {
                request_id: request.request_id,
                request_sha256,
                payload_sha256: permit.payload_sha256.clone(),
                snapshot: snapshot.clone(),
            },
        );
        self.active.insert(
            permit_id,
            ActivePermit {
                permit,
                state: P03ReservationState::ActiveReserved,
                last_unknown_evidence_sha256: None,
            },
        );
        Ok(P03AdmissionDisposition::Inserted(snapshot))
    }

    pub fn mark_dispatch_started(
        &mut self,
        permit_id: &str,
        current_fence: &P03Fence,
        expected_revision: u64,
        observed_at_ms: u64,
    ) -> Result<(P03WriteDisposition, P03ReservationSnapshot), P03SchedulerError> {
        self.transactional(|scheduler| {
            scheduler.mark_dispatch_started_inner(
                permit_id,
                current_fence,
                expected_revision,
                observed_at_ms,
            )
        })
    }

    fn mark_dispatch_started_inner(
        &mut self,
        permit_id: &str,
        current_fence: &P03Fence,
        expected_revision: u64,
        observed_at_ms: u64,
    ) -> Result<(P03WriteDisposition, P03ReservationSnapshot), P03SchedulerError> {
        validate_text(permit_id)?;
        current_fence.validate()?;
        if observed_at_ms == 0 {
            return Err(P03SchedulerError::InvalidRequest);
        }
        let active = self
            .active
            .get(permit_id)
            .cloned()
            .ok_or(P03SchedulerError::UnknownPermit)?;
        if active.state == P03ReservationState::DispatchStarted
            || active.state == P03ReservationState::OutcomeUnknown
        {
            return Ok((
                P03WriteDisposition::AlreadyPresent,
                self.snapshot_for_permit(&active.permit)?,
            ));
        }
        if active.state != P03ReservationState::ActiveReserved {
            return Err(P03SchedulerError::InvalidTransition);
        }
        self.validate_current_callback(&active.permit, current_fence, expected_revision)?;
        let next_revision = self.next_revision()?;
        let mut next = active;
        next.state = P03ReservationState::DispatchStarted;
        self.active.insert(permit_id.to_string(), next.clone());
        let snapshot = self.update_record_state(
            &next.permit,
            P03ReservationState::DispatchStarted,
            next_revision,
        )?;
        self.revision = next_revision;
        Ok((P03WriteDisposition::Applied, snapshot))
    }

    pub fn mark_outcome_unknown(
        &mut self,
        permit_id: &str,
        current_fence: &P03Fence,
        expected_revision: u64,
        evidence_sha256: Sha256Digest,
        observed_at_ms: u64,
    ) -> Result<(P03WriteDisposition, P03ReservationSnapshot), P03SchedulerError> {
        self.transactional(|scheduler| {
            scheduler.mark_outcome_unknown_inner(
                permit_id,
                current_fence,
                expected_revision,
                evidence_sha256,
                observed_at_ms,
            )
        })
    }

    fn mark_outcome_unknown_inner(
        &mut self,
        permit_id: &str,
        current_fence: &P03Fence,
        expected_revision: u64,
        evidence_sha256: Sha256Digest,
        observed_at_ms: u64,
    ) -> Result<(P03WriteDisposition, P03ReservationSnapshot), P03SchedulerError> {
        validate_text(permit_id)?;
        current_fence.validate()?;
        if observed_at_ms == 0 || !valid_digest(&evidence_sha256) {
            return Err(P03SchedulerError::InvalidRequest);
        }
        let active = self
            .active
            .get(permit_id)
            .cloned()
            .ok_or(P03SchedulerError::UnknownPermit)?;
        if active.state == P03ReservationState::OutcomeUnknown {
            if active.last_unknown_evidence_sha256.as_ref() == Some(&evidence_sha256) {
                return Ok((
                    P03WriteDisposition::AlreadyPresent,
                    self.snapshot_for_permit(&active.permit)?,
                ));
            }
            return Err(P03SchedulerError::ObservationConflict);
        }
        if active.state != P03ReservationState::DispatchStarted {
            return Err(P03SchedulerError::InvalidTransition);
        }
        self.validate_current_callback(&active.permit, current_fence, expected_revision)?;
        let next_revision = self.next_revision()?;
        let mut next = active;
        next.state = P03ReservationState::OutcomeUnknown;
        next.last_unknown_evidence_sha256 = Some(evidence_sha256);
        self.active.insert(permit_id.to_string(), next.clone());
        let snapshot = self.update_record_state(
            &next.permit,
            P03ReservationState::OutcomeUnknown,
            next_revision,
        )?;
        self.revision = next_revision;
        Ok((P03WriteDisposition::Applied, snapshot))
    }

    pub fn rebind(
        &mut self,
        current_fence: P03Fence,
        expected_revision: u64,
        observed_at_ms: u64,
    ) -> Result<u64, P03SchedulerError> {
        self.transactional(|scheduler| {
            scheduler.rebind_inner(current_fence, expected_revision, observed_at_ms)
        })
    }

    fn rebind_inner(
        &mut self,
        current_fence: P03Fence,
        expected_revision: u64,
        observed_at_ms: u64,
    ) -> Result<u64, P03SchedulerError> {
        current_fence.validate()?;
        if expected_revision != self.revision || observed_at_ms == 0 {
            return Err(P03SchedulerError::StaleRevision);
        }
        let old = &self.resource.fence;
        if current_fence.authority_epoch < old.authority_epoch
            || current_fence.owner_epoch <= old.owner_epoch
            || current_fence.generation <= old.generation
            || current_fence.fencing_token_sha256 == old.fencing_token_sha256
        {
            return Err(P03SchedulerError::StaleFence);
        }
        let next_revision = self.next_revision()?;
        self.resource.fence = current_fence;
        self.revision = next_revision;
        Ok(next_revision)
    }

    pub fn reconcile_old_permit(
        &mut self,
        request: P03OldPermitReconcileRequest,
    ) -> Result<P03ReconcileDisposition, P03SchedulerError> {
        self.transactional(|scheduler| scheduler.reconcile_old_permit_inner(request))
    }

    fn reconcile_old_permit_inner(
        &mut self,
        request: P03OldPermitReconcileRequest,
    ) -> Result<P03ReconcileDisposition, P03SchedulerError> {
        request.validate()?;
        let request_sha256 = request.digest()?;
        let request_key = request_sha256.to_string();

        if let Some(terminal_request) = self
            .terminal_reconcile_by_permit
            .get(&request.permit_id)
        {
            if terminal_request == &request_key {
                let receipt = self
                    .reconcile_history
                    .get(&request_key)
                    .cloned()
                    .ok_or(P03SchedulerError::CorruptState)?;
                return Ok(P03ReconcileDisposition::AlreadyPresent(receipt));
            }
            return Err(P03SchedulerError::TerminalImmutable);
        }
        if let Some(receipt) = self.reconcile_history.get(&request_key) {
            return Ok(P03ReconcileDisposition::AlreadyPresent(receipt.clone()));
        }
        if request.expected_revision != self.revision {
            return Err(P03SchedulerError::StaleRevision);
        }
        if request.current_fence != self.resource.fence {
            return Err(P03SchedulerError::StaleFence);
        }
        let active = self
            .active
            .get(&request.permit_id)
            .cloned()
            .ok_or(P03SchedulerError::UnknownPermit)?;
        if active.permit.fence != request.old_fence
            || active.permit.fence == request.current_fence
            || request.old_fence.owner_epoch >= request.current_fence.owner_epoch
            || request.old_fence.generation >= request.current_fence.generation
        {
            return Err(P03SchedulerError::StaleFence);
        }
        if !matches!(
            active.state,
            P03ReservationState::DispatchStarted | P03ReservationState::OutcomeUnknown
        ) {
            return Err(P03SchedulerError::InvalidTransition);
        }

        let before_revision = self.revision;
        let after_revision = self.next_revision()?;
        let (resolution, actual) = match request.outcome {
            P03ReconcileOutcome::VerifiedConsumed { actual } => {
                if !actual.fits_within(active.permit.reserved) {
                    return Err(P03SchedulerError::UsageOverrun);
                }
                let next_held = self
                    .held
                    .checked_sub(active.permit.reserved)
                    .ok_or(P03SchedulerError::CorruptState)?;
                let next_used = self
                    .used
                    .checked_add(actual.terminal_usage())
                    .ok_or(P03SchedulerError::QuotaExceeded)?;
                self.held = next_held;
                self.used = next_used;
                self.active.remove(&request.permit_id);
                self.update_record_state(
                    &active.permit,
                    P03ReservationState::Completed,
                    after_revision,
                )?;
                self.terminal_reconcile_by_permit
                    .insert(request.permit_id.clone(), request_key.clone());
                (P03ReconcileResolution::Consumed, Some(actual))
            }
            P03ReconcileOutcome::VerifiedNoEffect => {
                self.held = self
                    .held
                    .checked_sub(active.permit.reserved)
                    .ok_or(P03SchedulerError::CorruptState)?;
                self.active.remove(&request.permit_id);
                self.update_record_state(
                    &active.permit,
                    P03ReservationState::Released,
                    after_revision,
                )?;
                self.terminal_reconcile_by_permit
                    .insert(request.permit_id.clone(), request_key.clone());
                (P03ReconcileResolution::NoEffect, None)
            }
            P03ReconcileOutcome::Unknown => {
                let mut next = active;
                next.state = P03ReservationState::OutcomeUnknown;
                self.active.insert(request.permit_id.clone(), next.clone());
                self.update_record_state(
                    &next.permit,
                    P03ReservationState::OutcomeUnknown,
                    after_revision,
                )?;
                (P03ReconcileResolution::HeldUnknown, None)
            }
        };
        self.revision = after_revision;
        let receipt = P03OldPermitReconcileReceipt {
            request_sha256,
            permit_id: request.permit_id,
            resolution,
            actual,
            before_revision,
            after_revision,
            held_after: self.held,
            used_after: self.used,
            observed_at_ms: request.observed_at_ms,
            authority: AUTHBUS_B4_P0_3_AUTHORITY,
        };
        self.reconcile_history
            .insert(request_key, receipt.clone());
        Ok(P03ReconcileDisposition::Applied(receipt))
    }

    pub fn expire_active_permits(
        &mut self,
        now_ms: u64,
        expected_revision: u64,
    ) -> Result<P03ExpiryReport, P03SchedulerError> {
        self.transactional(|scheduler| {
            scheduler.expire_active_permits_inner(now_ms, expected_revision)
        })
    }

    fn expire_active_permits_inner(
        &mut self,
        now_ms: u64,
        expected_revision: u64,
    ) -> Result<P03ExpiryReport, P03SchedulerError> {
        if now_ms == 0 {
            return Err(P03SchedulerError::InvalidRequest);
        }
        if expected_revision != self.revision {
            return Err(P03SchedulerError::StaleRevision);
        }
        let mut pre_dispatch = Vec::new();
        let mut held_for_reconcile = Vec::new();
        let permit_ids: Vec<String> = self.active.keys().cloned().collect();
        for permit_id in permit_ids {
            let active = self
                .active
                .get(&permit_id)
                .cloned()
                .ok_or(P03SchedulerError::CorruptState)?;
            if active.permit.expires_at_ms > now_ms {
                continue;
            }
            match active.state {
                P03ReservationState::ActiveReserved => pre_dispatch.push(active),
                P03ReservationState::DispatchStarted | P03ReservationState::OutcomeUnknown => {
                    held_for_reconcile.push(self.snapshot_for_permit(&active.permit)?);
                }
                _ => return Err(P03SchedulerError::CorruptState),
            }
        }

        if pre_dispatch.is_empty() {
            return Ok(P03ExpiryReport {
                released_pre_dispatch: Vec::new(),
                held_for_reconcile,
                revision: self.revision,
            });
        }

        let next_revision = self.next_revision()?;
        let mut released = Vec::with_capacity(pre_dispatch.len());
        for active in pre_dispatch {
            self.held = self
                .held
                .checked_sub(active.permit.reserved)
                .ok_or(P03SchedulerError::CorruptState)?;
            self.active.remove(&active.permit.permit_id);
            released.push(self.update_record_state(
                &active.permit,
                P03ReservationState::ExpiredPreDispatch,
                next_revision,
            )?);
        }
        self.revision = next_revision;
        Ok(P03ExpiryReport {
            released_pre_dispatch: released,
            held_for_reconcile,
            revision: next_revision,
        })
    }

    pub fn verify_invariants(&self) -> Result<(), P03SchedulerError> {
        self.resource.validate()?;
        if self.revision == 0 || self.next_permit_nonce == 0 {
            return Err(P03SchedulerError::CorruptState);
        }
        let mut recomputed_held = CanonicalQuotaVector::default();
        for active in self.active.values() {
            if !active.state.is_active() || active.permit.authority {
                return Err(P03SchedulerError::CorruptState);
            }
            recomputed_held = recomputed_held
                .checked_add(active.permit.reserved)
                .ok_or(P03SchedulerError::CorruptState)?;
            let record = self
                .idempotency
                .get(&active.permit.idempotency_key)
                .ok_or(P03SchedulerError::CorruptState)?;
            if record.snapshot.permit != active.permit
                || record.snapshot.state != active.state
                || !record.snapshot.state.is_active()
            {
                return Err(P03SchedulerError::CorruptState);
            }
        }
        if recomputed_held != self.held {
            return Err(P03SchedulerError::CorruptState);
        }
        for (request_id, idempotency_key) in &self.request_bindings {
            let record = self
                .idempotency
                .get(idempotency_key)
                .ok_or(P03SchedulerError::CorruptState)?;
            if &record.request_id != request_id
                || record.snapshot.permit.request_id != *request_id
                || record.snapshot.permit.idempotency_key != *idempotency_key
            {
                return Err(P03SchedulerError::CorruptState);
            }
            if record.snapshot.state.is_active()
                != self.active.contains_key(&record.snapshot.permit.permit_id)
            {
                return Err(P03SchedulerError::CorruptState);
            }
        }
        if !self.resource.quota.can_hold(
            CanonicalQuotaVector::default(),
            CanonicalQuotaVector::default(),
            self.used,
        ) {
            return Err(P03SchedulerError::CorruptState);
        }
        Ok(())
    }

    fn validate_current_callback(
        &self,
        permit: &P03SchedulerPermit,
        current_fence: &P03Fence,
        expected_revision: u64,
    ) -> Result<(), P03SchedulerError> {
        if expected_revision != self.revision {
            return Err(P03SchedulerError::StaleRevision);
        }
        if current_fence != &self.resource.fence
            || permit.fence != self.resource.fence
            || permit.resource_id != self.resource.resource_id
            || permit.resource_sha256 != self.resource.resource_sha256
        {
            return Err(P03SchedulerError::StaleFence);
        }
        Ok(())
    }

    fn next_revision(&self) -> Result<u64, P03SchedulerError> {
        self.revision
            .checked_add(1)
            .ok_or(P03SchedulerError::InvalidRequest)
    }

    fn snapshot_for_permit(
        &self,
        permit: &P03SchedulerPermit,
    ) -> Result<P03ReservationSnapshot, P03SchedulerError> {
        self.idempotency
            .get(&permit.idempotency_key)
            .map(|record| record.snapshot.clone())
            .ok_or(P03SchedulerError::CorruptState)
    }

    fn update_record_state(
        &mut self,
        permit: &P03SchedulerPermit,
        state: P03ReservationState,
        revision: u64,
    ) -> Result<P03ReservationSnapshot, P03SchedulerError> {
        let record = self
            .idempotency
            .get_mut(&permit.idempotency_key)
            .ok_or(P03SchedulerError::CorruptState)?;
        if record.snapshot.permit != *permit {
            return Err(P03SchedulerError::CorruptState);
        }
        record.snapshot.state = state;
        record.snapshot.record_revision = revision;
        Ok(record.snapshot.clone())
    }
}

fn validate_text(value: &str) -> Result<(), P03SchedulerError> {
    if value.trim().is_empty()
        || value.len() > MAX_TEXT_BYTES
        || value.as_bytes().contains(&0)
        || value.chars().any(char::is_control)
    {
        return Err(P03SchedulerError::InvalidRequest);
    }
    Ok(())
}

fn valid_digest(value: &Sha256Digest) -> bool {
    Sha256Digest::parse(value.as_str().to_string()).is_ok()
}

fn push_bytes(buffer: &mut Vec<u8>, value: &[u8]) {
    buffer.extend_from_slice(&(value.len() as u64).to_be_bytes());
    buffer.extend_from_slice(value);
}

fn push_text(buffer: &mut Vec<u8>, value: &str) {
    push_bytes(buffer, value.as_bytes());
}

fn push_digest(buffer: &mut Vec<u8>, value: &Sha256Digest) {
    push_text(buffer, value.as_str());
}

fn push_u64(buffer: &mut Vec<u8>, value: u64) {
    push_bytes(buffer, &value.to_be_bytes());
}

fn push_quota(buffer: &mut Vec<u8>, value: CanonicalQuotaVector) {
    push_u64(buffer, value.request_count);
    push_u64(buffer, value.rpm);
    push_u64(buffer, value.tpm);
    push_u64(buffer, value.concurrency);
    push_u64(buffer, value.day_budget);
    push_u64(buffer, value.context);
}

fn push_fence(buffer: &mut Vec<u8>, value: &P03Fence) {
    push_u64(buffer, value.authority_epoch);
    push_u64(buffer, value.owner_epoch);
    push_u64(buffer, value.generation);
    push_digest(buffer, &value.fencing_token_sha256);
}
