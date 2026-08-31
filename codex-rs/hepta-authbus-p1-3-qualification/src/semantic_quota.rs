//! Executable semantic closure for the AuthBus P1.3 quota model.
//!
//! This module is compiled only by the explicit `p1-3-qualification` feature.
//! It supplies window-keyed accounting, per-request context enforcement,
//! active-only concurrency, reservation conservation, state-transition checks,
//! and an append-only digest chain. It grants no product or effect authority.

use std::collections::BTreeMap;

use codex_hepta_contracts::CanonicalQuotaLimits;
use codex_hepta_contracts::CanonicalQuotaVector;
use codex_hepta_contracts::QuotaDimension;
use codex_hepta_contracts::Sha256Digest;

const MINUTE_SECONDS: u64 = 60;
const DAY_SECONDS: u64 = 86_400;
const GENESIS_REVISION: u64 = 1;
const MAX_TEXT_BYTES: usize = 512;

/// Semantic closure errors. Every invalid or stale condition fails closed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemanticQuotaError {
    /// A text, time, vector, or window field is invalid.
    InvalidRequest,
    /// One or more canonical limits are unknown.
    UnknownLimit,
    /// A cumulative, windowed, active, or per-request limit would be exceeded.
    QuotaExceeded,
    /// The supplied ledger revision is stale.
    StaleRevision,
    /// A window does not match its dimension, interval, policy, or observation.
    InvalidWindow,
    /// An idempotency key or reservation identifier was reused with drift.
    BindingConflict,
    /// The requested reservation does not exist.
    UnknownReservation,
    /// The requested state transition is forbidden.
    InvalidTransition,
    /// Consumed and remaining vectors do not conserve the initial hold.
    ConservationViolation,
    /// A terminal usage vector exceeds the admitted hold.
    UsageOverrun,
    /// The append-only state digest chain is invalid.
    DigestChainMismatch,
    /// Checked arithmetic failed or stored state is internally inconsistent.
    CorruptState,
}

/// Canonical quota-window class.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum QuotaWindowKind {
    /// One exact UTC minute, used by RPM and TPM.
    MinuteUtc,
    /// One exact UTC day, used by daily budget.
    DayUtc,
}

impl QuotaWindowKind {
    const fn duration_seconds(self) -> u64 {
        match self {
            Self::MinuteUtc => MINUTE_SECONDS,
            Self::DayUtc => DAY_SECONDS,
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::MinuteUtc => "minute_utc",
            Self::DayUtc => "day_utc",
        }
    }
}

/// Durable identity for one windowed quota dimension.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct QuotaWindowKey {
    /// Canonical quota domain, such as provider/profile/resource.
    pub quota_domain: String,
    /// Windowed quota dimension.
    pub dimension: QuotaDimension,
    /// Exact interval class.
    pub kind: QuotaWindowKind,
    /// Inclusive UTC Unix-second boundary.
    pub starts_at_s: u64,
    /// Exclusive UTC Unix-second boundary.
    pub ends_at_s: u64,
    /// Non-zero policy/accounting revision.
    pub policy_revision: u64,
}

impl QuotaWindowKey {
    /// Validate canonical interval shape and observation membership.
    pub fn validate(&self, observed_at_s: u64) -> Result<(), SemanticQuotaError> {
        validate_text(&self.quota_domain)?;
        if self.policy_revision == 0 {
            return Err(SemanticQuotaError::InvalidWindow);
        }
        let expected_kind = match self.dimension {
            QuotaDimension::Rpm | QuotaDimension::Tpm => QuotaWindowKind::MinuteUtc,
            QuotaDimension::DayBudget => QuotaWindowKind::DayUtc,
            QuotaDimension::RequestCount
            | QuotaDimension::Concurrency
            | QuotaDimension::Context => return Err(SemanticQuotaError::InvalidWindow),
        };
        if self.kind != expected_kind {
            return Err(SemanticQuotaError::InvalidWindow);
        }
        let duration = self.kind.duration_seconds();
        if !self.starts_at_s.is_multiple_of(duration)
            || self.starts_at_s.checked_add(duration) != Some(self.ends_at_s)
            || observed_at_s < self.starts_at_s
            || observed_at_s >= self.ends_at_s
        {
            return Err(SemanticQuotaError::InvalidWindow);
        }
        Ok(())
    }

    fn append_digest_material(&self, bytes: &mut Vec<u8>) {
        push_text(bytes, &self.quota_domain);
        push_text(bytes, self.dimension.descriptor().canonical_key);
        push_text(bytes, self.kind.as_str());
        bytes.extend_from_slice(&self.starts_at_s.to_be_bytes());
        bytes.extend_from_slice(&self.ends_at_s.to_be_bytes());
        bytes.extend_from_slice(&self.policy_revision.to_be_bytes());
    }
}

/// Complete window identity required by one reservation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuotaWindowBindings {
    /// Requests-per-minute window.
    pub rpm: QuotaWindowKey,
    /// Tokens-per-minute window.
    pub tpm: QuotaWindowKey,
    /// Daily-budget window.
    pub day_budget: QuotaWindowKey,
}

impl QuotaWindowBindings {
    /// Validate exact dimension, domain, policy, and interval consistency.
    pub fn validate(
        &self,
        quota_domain: &str,
        policy_revision: u64,
        observed_at_s: u64,
    ) -> Result<(), SemanticQuotaError> {
        validate_text(quota_domain)?;
        if policy_revision == 0 {
            return Err(SemanticQuotaError::InvalidWindow);
        }
        self.rpm.validate(observed_at_s)?;
        self.tpm.validate(observed_at_s)?;
        self.day_budget.validate(observed_at_s)?;
        if self.rpm.dimension != QuotaDimension::Rpm
            || self.tpm.dimension != QuotaDimension::Tpm
            || self.day_budget.dimension != QuotaDimension::DayBudget
            || self.rpm.quota_domain != quota_domain
            || self.tpm.quota_domain != quota_domain
            || self.day_budget.quota_domain != quota_domain
            || self.rpm.policy_revision != policy_revision
            || self.tpm.policy_revision != policy_revision
            || self.day_budget.policy_revision != policy_revision
            || self.rpm.starts_at_s != self.tpm.starts_at_s
            || self.rpm.ends_at_s != self.tpm.ends_at_s
        {
            return Err(SemanticQuotaError::InvalidWindow);
        }
        Ok(())
    }

    /// Return the exact key for one windowed dimension.
    pub fn key_for(&self, dimension: QuotaDimension) -> Option<&QuotaWindowKey> {
        match dimension {
            QuotaDimension::Rpm => Some(&self.rpm),
            QuotaDimension::Tpm => Some(&self.tpm),
            QuotaDimension::DayBudget => Some(&self.day_budget),
            QuotaDimension::RequestCount
            | QuotaDimension::Concurrency
            | QuotaDimension::Context => None,
        }
    }

    fn digest(&self) -> Sha256Digest {
        let mut bytes = Vec::new();
        push_text(&mut bytes, "hepta.authbus.p1.3.window-bindings.v1");
        self.rpm.append_digest_material(&mut bytes);
        self.tpm.append_digest_material(&mut bytes);
        self.day_budget.append_digest_material(&mut bytes);
        Sha256Digest::for_bytes(&bytes)
    }
}

/// Request to reserve canonical quota under exact windows and revision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticReservationRequest {
    /// Stable reservation identity.
    pub reservation_id: String,
    /// Stable idempotency identity.
    pub idempotency_key: String,
    /// Canonical resource/provider quota domain.
    pub quota_domain: String,
    /// Exact final request payload digest.
    pub payload_sha256: Sha256Digest,
    /// Exact policy/configuration digest.
    pub policy_sha256: Sha256Digest,
    /// Non-zero policy revision shared by all window keys.
    pub policy_revision: u64,
    /// Estimated usage before margin.
    pub estimated: CanonicalQuotaVector,
    /// Explicit safety margin.
    pub safety_margin: CanonicalQuotaVector,
    /// Exact window identities.
    pub windows: QuotaWindowBindings,
    /// Request observation time in UTC Unix seconds.
    pub issued_at_s: u64,
    /// Reservation expiry in UTC Unix seconds.
    pub expires_at_s: u64,
    /// Required current ledger revision.
    pub expected_revision: u64,
}

impl SemanticReservationRequest {
    /// Validate exact binding and return the admitted hold vector.
    pub fn validate(&self) -> Result<CanonicalQuotaVector, SemanticQuotaError> {
        validate_text(&self.reservation_id)?;
        validate_text(&self.idempotency_key)?;
        validate_text(&self.quota_domain)?;
        if self.policy_revision == 0
            || self.expected_revision == 0
            || self.expires_at_s <= self.issued_at_s
        {
            return Err(SemanticQuotaError::InvalidRequest);
        }
        self.windows
            .validate(&self.quota_domain, self.policy_revision, self.issued_at_s)?;
        let hold = self
            .estimated
            .checked_add(self.safety_margin)
            .ok_or(SemanticQuotaError::InvalidRequest)?;
        if hold.is_zero() || hold.request_count == 0 {
            return Err(SemanticQuotaError::InvalidRequest);
        }
        Ok(hold)
    }

    fn binding_digest(&self) -> Result<Sha256Digest, SemanticQuotaError> {
        let hold = self.validate()?;
        let mut bytes = Vec::new();
        push_text(
            &mut bytes,
            "hepta.authbus.p1.3.semantic-reservation-request.v1",
        );
        push_text(&mut bytes, &self.reservation_id);
        push_text(&mut bytes, &self.idempotency_key);
        push_text(&mut bytes, &self.quota_domain);
        push_text(&mut bytes, self.payload_sha256.as_str());
        push_text(&mut bytes, self.policy_sha256.as_str());
        bytes.extend_from_slice(&self.policy_revision.to_be_bytes());
        append_vector(&mut bytes, self.estimated);
        append_vector(&mut bytes, self.safety_margin);
        append_vector(&mut bytes, hold);
        push_text(&mut bytes, self.windows.digest().as_str());
        bytes.extend_from_slice(&self.issued_at_s.to_be_bytes());
        bytes.extend_from_slice(&self.expires_at_s.to_be_bytes());
        Ok(Sha256Digest::for_bytes(&bytes))
    }
}

/// Durable reservation state used by the semantic qualification kernel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemanticReservationState {
    /// Hold is durable and no physical boundary has been attempted.
    Held,
    /// A physical dispatch attempt was durably recorded.
    DispatchAttempted,
    /// The physical outcome is unknown and only lookup/reconcile may proceed.
    Indeterminate,
    /// Verified terminal usage was committed.
    Completed,
    /// Verified no-effect released the complete hold.
    Released,
    /// A pre-dispatch expiry released the complete hold.
    ExpiredPreDispatch,
}

impl SemanticReservationState {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Held => "held",
            Self::DispatchAttempted => "dispatch_attempted",
            Self::Indeterminate => "indeterminate",
            Self::Completed => "completed",
            Self::Released => "released",
            Self::ExpiredPreDispatch => "expired_pre_dispatch",
        }
    }

    const fn is_active(self) -> bool {
        matches!(
            self,
            Self::Held | Self::DispatchAttempted | Self::Indeterminate
        )
    }
}

/// Current semantic reservation projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticReservationRecord {
    /// Stable reservation identity.
    pub reservation_id: String,
    /// Stable idempotency identity.
    pub idempotency_key: String,
    /// Exact quota domain.
    pub quota_domain: String,
    /// Exact request binding digest.
    pub request_sha256: Sha256Digest,
    /// Exact final payload digest.
    pub payload_sha256: Sha256Digest,
    /// Exact policy digest.
    pub policy_sha256: Sha256Digest,
    /// Exact policy revision.
    pub policy_revision: u64,
    /// Initial admitted hold.
    pub held: CanonicalQuotaVector,
    /// Verified terminal usage. Context is retained here as request evidence but
    /// is never accumulated into ledger spend.
    pub consumed: CanonicalQuotaVector,
    /// Unconsumed portion of the initial hold. `held = consumed + remaining`.
    pub remaining: CanonicalQuotaVector,
    /// Exact window identities.
    pub windows: QuotaWindowBindings,
    /// Request observation time.
    pub issued_at_s: u64,
    /// Reservation expiry.
    pub expires_at_s: u64,
    /// Current state.
    pub state: SemanticReservationState,
    /// Revision of the latest state transition.
    pub revision: u64,
    /// Prior global transition-chain digest.
    pub prior_state_sha256: Sha256Digest,
    /// Current global transition-chain digest.
    pub state_sha256: Sha256Digest,
}

impl SemanticReservationRecord {
    /// Validate conservation and state-specific vector rules.
    pub fn validate_semantics(&self) -> Result<(), SemanticQuotaError> {
        validate_text(&self.reservation_id)?;
        validate_text(&self.idempotency_key)?;
        validate_text(&self.quota_domain)?;
        if self.policy_revision == 0
            || self.revision <= GENESIS_REVISION
            || self.expires_at_s <= self.issued_at_s
        {
            return Err(SemanticQuotaError::CorruptState);
        }
        self.windows
            .validate(&self.quota_domain, self.policy_revision, self.issued_at_s)?;
        let conserved = self
            .consumed
            .checked_add(self.remaining)
            .ok_or(SemanticQuotaError::ConservationViolation)?;
        if conserved != self.held {
            return Err(SemanticQuotaError::ConservationViolation);
        }
        match self.state {
            SemanticReservationState::Held
            | SemanticReservationState::DispatchAttempted
            | SemanticReservationState::Indeterminate => {
                if !self.consumed.is_zero() || self.remaining != self.held {
                    return Err(SemanticQuotaError::ConservationViolation);
                }
            }
            SemanticReservationState::Completed => {
                if !self.consumed.fits_within(self.held) || self.consumed.concurrency != 0 {
                    return Err(SemanticQuotaError::UsageOverrun);
                }
            }
            SemanticReservationState::Released | SemanticReservationState::ExpiredPreDispatch => {
                if !self.consumed.is_zero() || self.remaining != self.held {
                    return Err(SemanticQuotaError::ConservationViolation);
                }
            }
        }
        Ok(())
    }
}

/// One append-only transition-chain receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticTransitionReceipt {
    /// Reservation whose state changed.
    pub reservation_id: String,
    /// Previous reservation state, or `None` for insertion.
    pub from_state: Option<SemanticReservationState>,
    /// New reservation state.
    pub to_state: SemanticReservationState,
    /// Global monotonic ledger revision.
    pub revision: u64,
    /// Initial hold vector.
    pub held: CanonicalQuotaVector,
    /// Verified usage vector.
    pub consumed: CanonicalQuotaVector,
    /// Unconsumed hold vector.
    pub remaining: CanonicalQuotaVector,
    /// Exact window binding digest.
    pub windows_sha256: Sha256Digest,
    /// Exact payload digest.
    pub payload_sha256: Sha256Digest,
    /// Exact policy digest.
    pub policy_sha256: Sha256Digest,
    /// Prior global digest.
    pub prior_sha256: Sha256Digest,
    /// Digest of this transition.
    pub transition_sha256: Sha256Digest,
}

impl SemanticTransitionReceipt {
    fn expected_digest(&self) -> Sha256Digest {
        let mut bytes = Vec::new();
        push_text(&mut bytes, "hepta.authbus.p1.3.semantic-transition.v1");
        push_text(&mut bytes, &self.reservation_id);
        push_text(
            &mut bytes,
            self.from_state
                .map_or("none", SemanticReservationState::as_str),
        );
        push_text(&mut bytes, self.to_state.as_str());
        bytes.extend_from_slice(&self.revision.to_be_bytes());
        append_vector(&mut bytes, self.held);
        append_vector(&mut bytes, self.consumed);
        append_vector(&mut bytes, self.remaining);
        push_text(&mut bytes, self.windows_sha256.as_str());
        push_text(&mut bytes, self.payload_sha256.as_str());
        push_text(&mut bytes, self.policy_sha256.as_str());
        push_text(&mut bytes, self.prior_sha256.as_str());
        Sha256Digest::for_bytes(&bytes)
    }

    fn validate_conservation(&self) -> Result<(), SemanticQuotaError> {
        let conserved = self
            .consumed
            .checked_add(self.remaining)
            .ok_or(SemanticQuotaError::ConservationViolation)?;
        if conserved != self.held {
            return Err(SemanticQuotaError::ConservationViolation);
        }
        match self.to_state {
            SemanticReservationState::Held
            | SemanticReservationState::DispatchAttempted
            | SemanticReservationState::Indeterminate
            | SemanticReservationState::Released
            | SemanticReservationState::ExpiredPreDispatch => {
                if !self.consumed.is_zero() || self.remaining != self.held {
                    return Err(SemanticQuotaError::ConservationViolation);
                }
            }
            SemanticReservationState::Completed => {
                if !self.consumed.fits_within(self.held) || self.consumed.concurrency != 0 {
                    return Err(SemanticQuotaError::UsageOverrun);
                }
            }
        }
        Ok(())
    }
}

/// Result of an idempotent reservation admission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SemanticAdmissionDisposition {
    /// New hold inserted.
    Inserted(SemanticReservationRecord),
    /// Exact binding already exists; no counters or revision changed.
    AlreadyPresent(SemanticReservationRecord),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct WindowCounter {
    used: u64,
    held: u64,
}

/// Window-aware, revision-bound P1.3 semantic qualification kernel.
#[derive(Clone, Debug)]
pub struct WindowedQuotaLedger {
    limits: CanonicalQuotaLimits,
    revision: u64,
    lifetime_request_count_used: u64,
    lifetime_request_count_held: u64,
    active_concurrency_held: u64,
    windows: BTreeMap<QuotaWindowKey, WindowCounter>,
    reservations: BTreeMap<String, SemanticReservationRecord>,
    idempotency: BTreeMap<String, (Sha256Digest, String)>,
    transitions: Vec<SemanticTransitionReceipt>,
    head_sha256: Sha256Digest,
}

impl WindowedQuotaLedger {
    /// Open an empty ledger under fully known canonical limits.
    pub fn open(limits: CanonicalQuotaLimits) -> Result<Self, SemanticQuotaError> {
        if !limits.is_fully_known() {
            return Err(SemanticQuotaError::UnknownLimit);
        }
        Ok(Self {
            limits,
            revision: GENESIS_REVISION,
            lifetime_request_count_used: 0,
            lifetime_request_count_held: 0,
            active_concurrency_held: 0,
            windows: BTreeMap::new(),
            reservations: BTreeMap::new(),
            idempotency: BTreeMap::new(),
            transitions: Vec::new(),
            head_sha256: genesis_digest(),
        })
    }

    /// Return the current global revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Return the current transition-chain head.
    #[must_use]
    pub fn head_sha256(&self) -> &Sha256Digest {
        &self.head_sha256
    }

    /// Return an immutable reservation projection.
    #[must_use]
    pub fn reservation(&self, reservation_id: &str) -> Option<&SemanticReservationRecord> {
        self.reservations.get(reservation_id)
    }

    /// Return retained append-only transition receipts.
    #[must_use]
    pub fn transition_receipts(&self) -> &[SemanticTransitionReceipt] {
        &self.transitions
    }

    /// Return used and held values for one exact window key.
    #[must_use]
    pub fn window_usage(&self, key: &QuotaWindowKey) -> (u64, u64) {
        self.windows
            .get(key)
            .map_or((0, 0), |counter| (counter.used, counter.held))
    }

    /// Return active-only concurrency holds.
    #[must_use]
    pub const fn active_concurrency(&self) -> u64 {
        self.active_concurrency_held
    }

    /// Reserve quota atomically. Exact replay returns the original projection.
    pub fn reserve(
        &mut self,
        request: SemanticReservationRequest,
    ) -> Result<SemanticAdmissionDisposition, SemanticQuotaError> {
        self.transactional(|ledger| ledger.reserve_inner(request))
    }

    fn reserve_inner(
        &mut self,
        request: SemanticReservationRequest,
    ) -> Result<SemanticAdmissionDisposition, SemanticQuotaError> {
        let hold = request.validate()?;
        let request_sha256 = request.binding_digest()?;
        if let Some((prior_sha256, reservation_id)) = self.idempotency.get(&request.idempotency_key)
        {
            if prior_sha256 != &request_sha256 || reservation_id != &request.reservation_id {
                return Err(SemanticQuotaError::BindingConflict);
            }
            let record = self
                .reservations
                .get(reservation_id)
                .cloned()
                .ok_or(SemanticQuotaError::CorruptState)?;
            return Ok(SemanticAdmissionDisposition::AlreadyPresent(record));
        }
        if self.reservations.contains_key(&request.reservation_id) {
            return Err(SemanticQuotaError::BindingConflict);
        }
        self.require_revision(request.expected_revision)?;
        self.check_capacity(hold, &request.windows)?;
        self.apply_hold(hold, &request.windows)?;

        let zero = CanonicalQuotaVector::default();
        let mut record = SemanticReservationRecord {
            reservation_id: request.reservation_id.clone(),
            idempotency_key: request.idempotency_key.clone(),
            quota_domain: request.quota_domain,
            request_sha256: request_sha256.clone(),
            payload_sha256: request.payload_sha256,
            policy_sha256: request.policy_sha256,
            policy_revision: request.policy_revision,
            held: hold,
            consumed: zero,
            remaining: hold,
            windows: request.windows,
            issued_at_s: request.issued_at_s,
            expires_at_s: request.expires_at_s,
            state: SemanticReservationState::Held,
            revision: 0,
            prior_state_sha256: genesis_digest(),
            state_sha256: genesis_digest(),
        };
        self.append_transition(None, &mut record)?;
        record.validate_semantics()?;
        self.idempotency.insert(
            request.idempotency_key,
            (request_sha256, request.reservation_id.clone()),
        );
        self.reservations
            .insert(request.reservation_id, record.clone());
        Ok(SemanticAdmissionDisposition::Inserted(record))
    }

    /// Record the physical-boundary attempt before crossing it.
    pub fn mark_dispatch_attempted(
        &mut self,
        reservation_id: &str,
        expected_revision: u64,
    ) -> Result<SemanticReservationRecord, SemanticQuotaError> {
        self.transactional(|ledger| {
            ledger.transition_state(
                reservation_id,
                expected_revision,
                SemanticReservationState::DispatchAttempted,
                None,
            )
        })
    }

    /// Mark a post-crossing unknown outcome. Recovery is lookup-only.
    pub fn mark_indeterminate(
        &mut self,
        reservation_id: &str,
        expected_revision: u64,
    ) -> Result<SemanticReservationRecord, SemanticQuotaError> {
        self.transactional(|ledger| {
            ledger.transition_state(
                reservation_id,
                expected_revision,
                SemanticReservationState::Indeterminate,
                None,
            )
        })
    }

    /// Commit verified terminal usage and release every unused hold.
    pub fn finalize_consumed(
        &mut self,
        reservation_id: &str,
        expected_revision: u64,
        actual: CanonicalQuotaVector,
    ) -> Result<SemanticReservationRecord, SemanticQuotaError> {
        self.transactional(|ledger| {
            ledger.transition_state(
                reservation_id,
                expected_revision,
                SemanticReservationState::Completed,
                Some(actual),
            )
        })
    }

    /// Commit verified no-effect and release the complete hold.
    pub fn finalize_no_effect(
        &mut self,
        reservation_id: &str,
        expected_revision: u64,
    ) -> Result<SemanticReservationRecord, SemanticQuotaError> {
        self.transactional(|ledger| {
            ledger.transition_state(
                reservation_id,
                expected_revision,
                SemanticReservationState::Released,
                None,
            )
        })
    }

    /// Expire only a pre-dispatch hold.
    pub fn expire_pre_dispatch(
        &mut self,
        reservation_id: &str,
        expected_revision: u64,
        now_s: u64,
    ) -> Result<SemanticReservationRecord, SemanticQuotaError> {
        self.transactional(|ledger| {
            let record = ledger
                .reservations
                .get(reservation_id)
                .ok_or(SemanticQuotaError::UnknownReservation)?;
            if now_s < record.expires_at_s {
                return Err(SemanticQuotaError::InvalidRequest);
            }
            ledger.transition_state(
                reservation_id,
                expected_revision,
                SemanticReservationState::ExpiredPreDispatch,
                None,
            )
        })
    }

    fn transition_state(
        &mut self,
        reservation_id: &str,
        expected_revision: u64,
        next_state: SemanticReservationState,
        actual: Option<CanonicalQuotaVector>,
    ) -> Result<SemanticReservationRecord, SemanticQuotaError> {
        validate_text(reservation_id)?;
        self.require_revision(expected_revision)?;
        let mut record = self
            .reservations
            .get(reservation_id)
            .cloned()
            .ok_or(SemanticQuotaError::UnknownReservation)?;
        if !allowed_transition(Some(record.state), next_state) {
            return Err(SemanticQuotaError::InvalidTransition);
        }
        let prior_state = record.state;
        match next_state {
            SemanticReservationState::DispatchAttempted
            | SemanticReservationState::Indeterminate => {}
            SemanticReservationState::Completed => {
                let actual = actual.ok_or(SemanticQuotaError::InvalidRequest)?;
                if !actual.fits_within(record.held) || actual.concurrency != 0 {
                    return Err(SemanticQuotaError::UsageOverrun);
                }
                self.release_hold(record.held, &record.windows)?;
                self.apply_terminal_usage(actual, &record.windows)?;
                record.consumed = actual;
                record.remaining = record
                    .held
                    .checked_sub(actual)
                    .ok_or(SemanticQuotaError::ConservationViolation)?;
            }
            SemanticReservationState::Released | SemanticReservationState::ExpiredPreDispatch => {
                if actual.is_some() {
                    return Err(SemanticQuotaError::InvalidRequest);
                }
                self.release_hold(record.held, &record.windows)?;
                record.consumed = CanonicalQuotaVector::default();
                record.remaining = record.held;
            }
            SemanticReservationState::Held => return Err(SemanticQuotaError::InvalidTransition),
        }
        record.state = next_state;
        self.append_transition(Some(prior_state), &mut record)?;
        record.validate_semantics()?;
        self.reservations
            .insert(reservation_id.to_string(), record.clone());
        Ok(record)
    }

    fn check_capacity(
        &self,
        hold: CanonicalQuotaVector,
        windows: &QuotaWindowBindings,
    ) -> Result<(), SemanticQuotaError> {
        let request_limit = known_limit(self.limits.request_count)?;
        let concurrency_limit = known_limit(self.limits.concurrency)?;
        let context_limit = known_limit(self.limits.context)?;
        if checked_total(
            self.lifetime_request_count_used,
            self.lifetime_request_count_held,
            hold.request_count,
        )? > request_limit
            || self
                .active_concurrency_held
                .checked_add(hold.concurrency)
                .ok_or(SemanticQuotaError::QuotaExceeded)?
                > concurrency_limit
            || hold.context > context_limit
        {
            return Err(SemanticQuotaError::QuotaExceeded);
        }
        for dimension in [
            QuotaDimension::Rpm,
            QuotaDimension::Tpm,
            QuotaDimension::DayBudget,
        ] {
            let key = windows
                .key_for(dimension)
                .ok_or(SemanticQuotaError::InvalidWindow)?;
            let counter = self.windows.get(key).copied().unwrap_or_default();
            let requested = hold.value(dimension);
            if checked_total(counter.used, counter.held, requested)?
                > limit_for_dimension(self.limits, dimension)?
            {
                return Err(SemanticQuotaError::QuotaExceeded);
            }
        }
        Ok(())
    }

    fn apply_hold(
        &mut self,
        hold: CanonicalQuotaVector,
        windows: &QuotaWindowBindings,
    ) -> Result<(), SemanticQuotaError> {
        self.lifetime_request_count_held = self
            .lifetime_request_count_held
            .checked_add(hold.request_count)
            .ok_or(SemanticQuotaError::CorruptState)?;
        self.active_concurrency_held = self
            .active_concurrency_held
            .checked_add(hold.concurrency)
            .ok_or(SemanticQuotaError::CorruptState)?;
        for dimension in [
            QuotaDimension::Rpm,
            QuotaDimension::Tpm,
            QuotaDimension::DayBudget,
        ] {
            let key = windows
                .key_for(dimension)
                .ok_or(SemanticQuotaError::InvalidWindow)?
                .clone();
            let counter = self.windows.entry(key).or_default();
            counter.held = counter
                .held
                .checked_add(hold.value(dimension))
                .ok_or(SemanticQuotaError::CorruptState)?;
        }
        Ok(())
    }

    fn release_hold(
        &mut self,
        hold: CanonicalQuotaVector,
        windows: &QuotaWindowBindings,
    ) -> Result<(), SemanticQuotaError> {
        self.lifetime_request_count_held = self
            .lifetime_request_count_held
            .checked_sub(hold.request_count)
            .ok_or(SemanticQuotaError::CorruptState)?;
        self.active_concurrency_held = self
            .active_concurrency_held
            .checked_sub(hold.concurrency)
            .ok_or(SemanticQuotaError::CorruptState)?;
        for dimension in [
            QuotaDimension::Rpm,
            QuotaDimension::Tpm,
            QuotaDimension::DayBudget,
        ] {
            let key = windows
                .key_for(dimension)
                .ok_or(SemanticQuotaError::InvalidWindow)?;
            let counter = self
                .windows
                .get_mut(key)
                .ok_or(SemanticQuotaError::CorruptState)?;
            counter.held = counter
                .held
                .checked_sub(hold.value(dimension))
                .ok_or(SemanticQuotaError::CorruptState)?;
        }
        Ok(())
    }

    fn apply_terminal_usage(
        &mut self,
        actual: CanonicalQuotaVector,
        windows: &QuotaWindowBindings,
    ) -> Result<(), SemanticQuotaError> {
        self.lifetime_request_count_used = self
            .lifetime_request_count_used
            .checked_add(actual.request_count)
            .ok_or(SemanticQuotaError::CorruptState)?;
        for dimension in [
            QuotaDimension::Rpm,
            QuotaDimension::Tpm,
            QuotaDimension::DayBudget,
        ] {
            let key = windows
                .key_for(dimension)
                .ok_or(SemanticQuotaError::InvalidWindow)?;
            let counter = self
                .windows
                .get_mut(key)
                .ok_or(SemanticQuotaError::CorruptState)?;
            counter.used = counter
                .used
                .checked_add(actual.value(dimension))
                .ok_or(SemanticQuotaError::CorruptState)?;
        }
        // `actual.context` remains request evidence in the reservation record.
        // It is intentionally absent from aggregate counters. Concurrency is an
        // active hold and is intentionally absent from terminal usage.
        Ok(())
    }

    fn append_transition(
        &mut self,
        from_state: Option<SemanticReservationState>,
        record: &mut SemanticReservationRecord,
    ) -> Result<(), SemanticQuotaError> {
        if !allowed_transition(from_state, record.state) {
            return Err(SemanticQuotaError::InvalidTransition);
        }
        let revision = self
            .revision
            .checked_add(1)
            .ok_or(SemanticQuotaError::CorruptState)?;
        let prior_sha256 = self.head_sha256.clone();
        let placeholder = genesis_digest();
        let mut receipt = SemanticTransitionReceipt {
            reservation_id: record.reservation_id.clone(),
            from_state,
            to_state: record.state,
            revision,
            held: record.held,
            consumed: record.consumed,
            remaining: record.remaining,
            windows_sha256: record.windows.digest(),
            payload_sha256: record.payload_sha256.clone(),
            policy_sha256: record.policy_sha256.clone(),
            prior_sha256: prior_sha256.clone(),
            transition_sha256: placeholder,
        };
        receipt.validate_conservation()?;
        receipt.transition_sha256 = receipt.expected_digest();
        self.revision = revision;
        self.head_sha256 = receipt.transition_sha256.clone();
        record.revision = revision;
        record.prior_state_sha256 = prior_sha256;
        record.state_sha256 = receipt.transition_sha256.clone();
        self.transitions.push(receipt);
        Ok(())
    }

    fn require_revision(&self, expected_revision: u64) -> Result<(), SemanticQuotaError> {
        if expected_revision != self.revision {
            return Err(SemanticQuotaError::StaleRevision);
        }
        Ok(())
    }

    fn transactional<T>(
        &mut self,
        operation: impl FnOnce(&mut Self) -> Result<T, SemanticQuotaError>,
    ) -> Result<T, SemanticQuotaError> {
        let before = self.clone();
        match operation(self) {
            Ok(value) => {
                if let Err(error) = self.verify_invariants() {
                    *self = before;
                    Err(error)
                } else {
                    Ok(value)
                }
            }
            Err(error) => {
                *self = before;
                Err(error)
            }
        }
    }

    /// Recompute all counters, reservation rules, and the transition chain.
    pub fn verify_invariants(&self) -> Result<(), SemanticQuotaError> {
        if self.revision < GENESIS_REVISION || !self.limits.is_fully_known() {
            return Err(SemanticQuotaError::CorruptState);
        }
        let verified_head = verify_transition_chain(&self.transitions)?;
        if verified_head != self.head_sha256
            || self.revision != GENESIS_REVISION + self.transitions.len() as u64
        {
            return Err(SemanticQuotaError::DigestChainMismatch);
        }

        let mut request_used = 0_u64;
        let mut request_held = 0_u64;
        let mut concurrency_held = 0_u64;
        let mut windows: BTreeMap<QuotaWindowKey, WindowCounter> = BTreeMap::new();
        let mut latest: BTreeMap<String, &SemanticTransitionReceipt> = BTreeMap::new();
        for receipt in &self.transitions {
            latest.insert(receipt.reservation_id.clone(), receipt);
        }
        for record in self.reservations.values() {
            record.validate_semantics()?;
            let latest_receipt = latest
                .get(&record.reservation_id)
                .ok_or(SemanticQuotaError::DigestChainMismatch)?;
            if latest_receipt.to_state != record.state
                || latest_receipt.revision != record.revision
                || latest_receipt.transition_sha256 != record.state_sha256
                || latest_receipt.prior_sha256 != record.prior_state_sha256
            {
                return Err(SemanticQuotaError::DigestChainMismatch);
            }
            let idempotency = self
                .idempotency
                .get(&record.idempotency_key)
                .ok_or(SemanticQuotaError::CorruptState)?;
            if idempotency.0 != record.request_sha256 || idempotency.1 != record.reservation_id {
                return Err(SemanticQuotaError::BindingConflict);
            }
            if record.state.is_active() {
                request_held = request_held
                    .checked_add(record.held.request_count)
                    .ok_or(SemanticQuotaError::CorruptState)?;
                concurrency_held = concurrency_held
                    .checked_add(record.held.concurrency)
                    .ok_or(SemanticQuotaError::CorruptState)?;
                accumulate_window_vector(&mut windows, &record.windows, record.held, true)?;
            } else if record.state == SemanticReservationState::Completed {
                request_used = request_used
                    .checked_add(record.consumed.request_count)
                    .ok_or(SemanticQuotaError::CorruptState)?;
                accumulate_window_vector(&mut windows, &record.windows, record.consumed, false)?;
            }
        }
        if request_used != self.lifetime_request_count_used
            || request_held != self.lifetime_request_count_held
            || concurrency_held != self.active_concurrency_held
            || normalize_windows(windows) != normalize_windows(self.windows.clone())
        {
            return Err(SemanticQuotaError::CorruptState);
        }
        if checked_total(request_used, request_held, 0)? > known_limit(self.limits.request_count)?
            || concurrency_held > known_limit(self.limits.concurrency)?
        {
            return Err(SemanticQuotaError::QuotaExceeded);
        }
        for (key, counter) in &self.windows {
            if checked_total(counter.used, counter.held, 0)?
                > limit_for_dimension(self.limits, key.dimension)?
            {
                return Err(SemanticQuotaError::QuotaExceeded);
            }
        }
        Ok(())
    }
}

/// Verify the global append-only transition chain independently of a ledger.
pub fn verify_transition_chain(
    receipts: &[SemanticTransitionReceipt],
) -> Result<Sha256Digest, SemanticQuotaError> {
    let mut head = genesis_digest();
    let mut states: BTreeMap<String, SemanticReservationState> = BTreeMap::new();
    for (index, receipt) in receipts.iter().enumerate() {
        validate_text(&receipt.reservation_id)?;
        let expected_revision = GENESIS_REVISION
            .checked_add(index as u64)
            .and_then(|value| value.checked_add(1))
            .ok_or(SemanticQuotaError::CorruptState)?;
        let expected_from = states.get(&receipt.reservation_id).copied();
        if receipt.revision != expected_revision
            || receipt.prior_sha256 != head
            || receipt.from_state != expected_from
            || !allowed_transition(receipt.from_state, receipt.to_state)
        {
            return Err(SemanticQuotaError::DigestChainMismatch);
        }
        receipt.validate_conservation()?;
        if receipt.expected_digest() != receipt.transition_sha256 {
            return Err(SemanticQuotaError::DigestChainMismatch);
        }
        states.insert(receipt.reservation_id.clone(), receipt.to_state);
        head = receipt.transition_sha256.clone();
    }
    Ok(head)
}

fn allowed_transition(
    from: Option<SemanticReservationState>,
    to: SemanticReservationState,
) -> bool {
    matches!(
        (from, to),
        (None, SemanticReservationState::Held)
            | (
                Some(SemanticReservationState::Held),
                SemanticReservationState::DispatchAttempted
            )
            | (
                Some(SemanticReservationState::Held),
                SemanticReservationState::Released
            )
            | (
                Some(SemanticReservationState::Held),
                SemanticReservationState::ExpiredPreDispatch
            )
            | (
                Some(SemanticReservationState::DispatchAttempted),
                SemanticReservationState::Indeterminate
            )
            | (
                Some(SemanticReservationState::DispatchAttempted),
                SemanticReservationState::Completed
            )
            | (
                Some(SemanticReservationState::DispatchAttempted),
                SemanticReservationState::Released
            )
            | (
                Some(SemanticReservationState::Indeterminate),
                SemanticReservationState::Completed
            )
            | (
                Some(SemanticReservationState::Indeterminate),
                SemanticReservationState::Released
            )
    )
}

fn accumulate_window_vector(
    counters: &mut BTreeMap<QuotaWindowKey, WindowCounter>,
    bindings: &QuotaWindowBindings,
    vector: CanonicalQuotaVector,
    held: bool,
) -> Result<(), SemanticQuotaError> {
    for dimension in [
        QuotaDimension::Rpm,
        QuotaDimension::Tpm,
        QuotaDimension::DayBudget,
    ] {
        let key = bindings
            .key_for(dimension)
            .ok_or(SemanticQuotaError::InvalidWindow)?
            .clone();
        let counter = counters.entry(key).or_default();
        let value = vector.value(dimension);
        if held {
            counter.held = counter
                .held
                .checked_add(value)
                .ok_or(SemanticQuotaError::CorruptState)?;
        } else {
            counter.used = counter
                .used
                .checked_add(value)
                .ok_or(SemanticQuotaError::CorruptState)?;
        }
    }
    Ok(())
}

fn normalize_windows(
    mut counters: BTreeMap<QuotaWindowKey, WindowCounter>,
) -> BTreeMap<QuotaWindowKey, WindowCounter> {
    counters.retain(|_, counter| counter.used != 0 || counter.held != 0);
    counters
}

fn known_limit(limit: Option<u64>) -> Result<u64, SemanticQuotaError> {
    limit.ok_or(SemanticQuotaError::UnknownLimit)
}

fn limit_for_dimension(
    limits: CanonicalQuotaLimits,
    dimension: QuotaDimension,
) -> Result<u64, SemanticQuotaError> {
    match dimension {
        QuotaDimension::RequestCount => known_limit(limits.request_count),
        QuotaDimension::Rpm => known_limit(limits.rpm),
        QuotaDimension::Tpm => known_limit(limits.tpm),
        QuotaDimension::Concurrency => known_limit(limits.concurrency),
        QuotaDimension::DayBudget => known_limit(limits.day_budget),
        QuotaDimension::Context => known_limit(limits.context),
    }
}

fn checked_total(used: u64, held: u64, requested: u64) -> Result<u64, SemanticQuotaError> {
    used.checked_add(held)
        .and_then(|value| value.checked_add(requested))
        .ok_or(SemanticQuotaError::QuotaExceeded)
}

fn validate_text(value: &str) -> Result<(), SemanticQuotaError> {
    if value.is_empty() || value.len() > MAX_TEXT_BYTES || value.chars().any(char::is_control) {
        return Err(SemanticQuotaError::InvalidRequest);
    }
    Ok(())
}

fn genesis_digest() -> Sha256Digest {
    Sha256Digest::for_bytes(b"hepta.authbus.p1.3.semantic-ledger.genesis.v1")
}

fn append_vector(bytes: &mut Vec<u8>, value: CanonicalQuotaVector) {
    for component in value.values() {
        bytes.extend_from_slice(&component.to_be_bytes());
    }
}

fn push_text(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value.as_bytes());
}
