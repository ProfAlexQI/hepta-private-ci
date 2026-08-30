use std::collections::BTreeMap;

use crate::Digest;
use crate::InferError;
use crate::RequestId;
use crate::Result;
use crate::TenantId;

const VIRTUAL_SERVICE_SCALE: u128 = 1_000_000;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchedulerConfig {
    pub max_queued: usize,
    pub aging_quantum_ms: u64,
    pub max_cost_units: u64,
    pub max_tenant_weight: u32,
}

impl SchedulerConfig {
    pub fn validate(&self) -> Result<()> {
        if self.max_queued == 0
            || self.aging_quantum_ms == 0
            || self.max_cost_units == 0
            || self.max_tenant_weight == 0
        {
            return Err(InferError::SchedulerConfigInvalid);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduledRequest {
    pub request_id: RequestId,
    pub tenant_id: TenantId,
    pub tuple_digest: Digest,
    pub deadline_unix_ms: u64,
    pub enqueued_unix_ms: u64,
    pub cost_units: u64,
    pub tenant_weight: u32,
}

impl ScheduledRequest {
    pub fn validate(&self, config: &SchedulerConfig) -> Result<()> {
        if self.deadline_unix_ms <= self.enqueued_unix_ms
            || self.cost_units == 0
            || self.cost_units > config.max_cost_units
            || self.tenant_weight == 0
            || self.tenant_weight > config.max_tenant_weight
        {
            return Err(InferError::SchedulerRequestInvalid);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct DeterministicScheduler {
    config: SchedulerConfig,
    queued: BTreeMap<RequestId, ScheduledRequest>,
    tenant_service: BTreeMap<TenantId, u128>,
}

impl DeterministicScheduler {
    pub fn new(config: SchedulerConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            config,
            queued: BTreeMap::new(),
            tenant_service: BTreeMap::new(),
        })
    }

    pub fn enqueue(&mut self, request: ScheduledRequest) -> Result<()> {
        request.validate(&self.config)?;
        if self.queued.contains_key(&request.request_id) {
            return Err(InferError::SchedulerDuplicateRequest);
        }
        if self.queued.len() >= self.config.max_queued {
            return Err(InferError::SchedulerQueueFull);
        }
        self.queued.insert(request.request_id.clone(), request);
        Ok(())
    }

    pub fn remove(&mut self, request_id: &RequestId) -> Result<ScheduledRequest> {
        self.queued
            .remove(request_id)
            .ok_or(InferError::SchedulerUnknownRequest)
    }

    pub fn pop_next(&mut self, now_unix_ms: u64) -> Result<Option<ScheduledRequest>> {
        let selected = self
            .queued
            .values()
            .min_by_key(|request| self.score(request, now_unix_ms))
            .map(|request| request.request_id.clone());
        let Some(request_id) = selected else {
            return Ok(None);
        };
        let request = self
            .queued
            .remove(&request_id)
            .ok_or(InferError::SchedulerUnknownRequest)?;
        let finish = self.virtual_finish(&request)?;
        self.tenant_service
            .insert(request.tenant_id.clone(), finish);
        Ok(Some(request))
    }

    pub fn len(&self) -> usize {
        self.queued.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queued.is_empty()
    }

    fn score(
        &self,
        request: &ScheduledRequest,
        now_unix_ms: u64,
    ) -> (u64, u128, u64, RequestId) {
        let waited = now_unix_ms.saturating_sub(request.enqueued_unix_ms);
        let age_quanta = waited / self.config.aging_quantum_ms;
        let age_credit = age_quanta.saturating_mul(self.config.aging_quantum_ms);
        let effective_deadline = request.deadline_unix_ms.saturating_sub(age_credit);
        let virtual_finish = self.virtual_finish(request).unwrap_or(u128::MAX);
        (
            effective_deadline,
            virtual_finish,
            request.enqueued_unix_ms,
            request.request_id.clone(),
        )
    }

    fn virtual_finish(&self, request: &ScheduledRequest) -> Result<u128> {
        let current = self
            .tenant_service
            .get(&request.tenant_id)
            .copied()
            .unwrap_or_default();
        let scaled_cost = u128::from(request.cost_units)
            .checked_mul(VIRTUAL_SERVICE_SCALE)
            .ok_or(InferError::SchedulerArithmeticOverflow)?;
        let weight = u128::from(request.tenant_weight);
        let service = scaled_cost
            .checked_add(weight.saturating_sub(1))
            .ok_or(InferError::SchedulerArithmeticOverflow)?
            / weight;
        current
            .checked_add(service)
            .ok_or(InferError::SchedulerArithmeticOverflow)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReservationLimits {
    pub max_inflight_global: usize,
    pub max_inflight_per_tenant: usize,
    pub max_running_global: usize,
    pub max_running_per_tenant: usize,
    pub max_running_per_tuple: usize,
    pub max_reserved_prompt_bytes: u64,
    pub max_reserved_output_tokens: u64,
    pub max_reserved_output_bytes: u64,
}

impl ReservationLimits {
    pub fn validate(&self) -> Result<()> {
        if self.max_inflight_global == 0
            || self.max_inflight_per_tenant == 0
            || self.max_running_global == 0
            || self.max_running_per_tenant == 0
            || self.max_running_per_tuple == 0
            || self.max_reserved_prompt_bytes == 0
            || self.max_reserved_output_tokens == 0
            || self.max_reserved_output_bytes == 0
            || self.max_running_global > self.max_inflight_global
            || self.max_running_per_tenant > self.max_inflight_per_tenant
        {
            return Err(InferError::ReservationConfigInvalid);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ReservationKey {
    pub request_id: RequestId,
    pub request_generation: u64,
    pub backend_generation: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReservationRequest {
    pub key: ReservationKey,
    pub tenant_id: TenantId,
    pub tuple_digest: Digest,
    pub prompt_bytes: u64,
    pub output_tokens: u32,
    pub output_bytes: u64,
}

impl ReservationRequest {
    fn validate(&self) -> Result<()> {
        if self.key.request_generation == 0
            || self.key.backend_generation == 0
            || self.prompt_bytes == 0
            || self.output_tokens == 0
            || self.output_bytes == 0
        {
            return Err(InferError::ReservationRequestInvalid);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReservationPhase {
    Inflight,
    Running,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReservationRecord {
    request: ReservationRequest,
    phase: ReservationPhase,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReservationSnapshot {
    pub inflight_global: usize,
    pub running_global: usize,
    pub reserved_prompt_bytes: u64,
    pub reserved_output_tokens: u64,
    pub reserved_output_bytes: u64,
    pub entries: usize,
}

#[derive(Debug)]
pub struct ReservationLedger {
    limits: ReservationLimits,
    records: BTreeMap<ReservationKey, ReservationRecord>,
    inflight_per_tenant: BTreeMap<TenantId, usize>,
    running_per_tenant: BTreeMap<TenantId, usize>,
    running_per_tuple: BTreeMap<Digest, usize>,
    inflight_global: usize,
    running_global: usize,
    reserved_prompt_bytes: u64,
    reserved_output_tokens: u64,
    reserved_output_bytes: u64,
}

impl ReservationLedger {
    pub fn new(limits: ReservationLimits) -> Result<Self> {
        limits.validate()?;
        Ok(Self {
            limits,
            records: BTreeMap::new(),
            inflight_per_tenant: BTreeMap::new(),
            running_per_tenant: BTreeMap::new(),
            running_per_tuple: BTreeMap::new(),
            inflight_global: 0,
            running_global: 0,
            reserved_prompt_bytes: 0,
            reserved_output_tokens: 0,
            reserved_output_bytes: 0,
        })
    }

    pub fn reserve(&mut self, request: ReservationRequest) -> Result<()> {
        request.validate()?;
        if self.records.contains_key(&request.key) {
            return Err(InferError::DuplicateReservation);
        }
        if self.inflight_global >= self.limits.max_inflight_global {
            return Err(InferError::ReservationGlobalFull);
        }
        let tenant_inflight = count(&self.inflight_per_tenant, &request.tenant_id);
        if tenant_inflight >= self.limits.max_inflight_per_tenant {
            return Err(InferError::ReservationTenantFull);
        }
        let next_prompt = checked_add_u64(self.reserved_prompt_bytes, request.prompt_bytes)?;
        let next_tokens = checked_add_u64(
            self.reserved_output_tokens,
            u64::from(request.output_tokens),
        )?;
        let next_output = checked_add_u64(self.reserved_output_bytes, request.output_bytes)?;
        if next_prompt > self.limits.max_reserved_prompt_bytes
            || next_tokens > self.limits.max_reserved_output_tokens
            || next_output > self.limits.max_reserved_output_bytes
        {
            return Err(InferError::ReservationBudgetFull);
        }

        self.inflight_global = self
            .inflight_global
            .checked_add(1)
            .ok_or(InferError::ReservationArithmeticOverflow)?;
        increment(&mut self.inflight_per_tenant, request.tenant_id.clone())?;
        self.reserved_prompt_bytes = next_prompt;
        self.reserved_output_tokens = next_tokens;
        self.reserved_output_bytes = next_output;
        self.records.insert(
            request.key.clone(),
            ReservationRecord {
                request,
                phase: ReservationPhase::Inflight,
            },
        );
        Ok(())
    }

    pub fn promote_running(&mut self, key: &ReservationKey) -> Result<()> {
        let record = self
            .records
            .get(key)
            .ok_or(InferError::UnknownReservation)?;
        if record.phase != ReservationPhase::Inflight {
            return Err(InferError::ReservationPhaseInvalid);
        }
        if self.running_global >= self.limits.max_running_global {
            return Err(InferError::ReservationRunningGlobalFull);
        }
        if count(&self.running_per_tenant, &record.request.tenant_id)
            >= self.limits.max_running_per_tenant
        {
            return Err(InferError::ReservationRunningTenantFull);
        }
        if count(&self.running_per_tuple, &record.request.tuple_digest)
            >= self.limits.max_running_per_tuple
        {
            return Err(InferError::ReservationRunningTupleFull);
        }

        let tenant = record.request.tenant_id.clone();
        let tuple = record.request.tuple_digest.clone();
        self.running_global = self
            .running_global
            .checked_add(1)
            .ok_or(InferError::ReservationArithmeticOverflow)?;
        increment(&mut self.running_per_tenant, tenant)?;
        increment(&mut self.running_per_tuple, tuple)?;
        self.records
            .get_mut(key)
            .ok_or(InferError::UnknownReservation)?
            .phase = ReservationPhase::Running;
        Ok(())
    }

    pub fn release(&mut self, key: &ReservationKey) -> Result<ReservationRequest> {
        let record = self
            .records
            .remove(key)
            .ok_or(InferError::UnknownReservation)?;
        self.inflight_global = self
            .inflight_global
            .checked_sub(1)
            .ok_or(InferError::ReservationInvariant)?;
        decrement(&mut self.inflight_per_tenant, &record.request.tenant_id)?;
        self.reserved_prompt_bytes = checked_sub_u64(
            self.reserved_prompt_bytes,
            record.request.prompt_bytes,
        )?;
        self.reserved_output_tokens = checked_sub_u64(
            self.reserved_output_tokens,
            u64::from(record.request.output_tokens),
        )?;
        self.reserved_output_bytes = checked_sub_u64(
            self.reserved_output_bytes,
            record.request.output_bytes,
        )?;
        if record.phase == ReservationPhase::Running {
            self.running_global = self
                .running_global
                .checked_sub(1)
                .ok_or(InferError::ReservationInvariant)?;
            decrement(
                &mut self.running_per_tenant,
                &record.request.tenant_id,
            )?;
            decrement(
                &mut self.running_per_tuple,
                &record.request.tuple_digest,
            )?;
        }
        Ok(record.request)
    }

    pub fn release_backend_generation(
        &mut self,
        backend_generation: u64,
    ) -> Result<Vec<ReservationRequest>> {
        let keys: Vec<ReservationKey> = self
            .records
            .keys()
            .filter(|key| key.backend_generation == backend_generation)
            .cloned()
            .collect();
        let mut released = Vec::with_capacity(keys.len());
        for key in keys {
            released.push(self.release(&key)?);
        }
        Ok(released)
    }

    pub fn phase(&self, key: &ReservationKey) -> Result<ReservationPhase> {
        self.records
            .get(key)
            .map(|record| record.phase)
            .ok_or(InferError::UnknownReservation)
    }

    pub fn snapshot(&self) -> ReservationSnapshot {
        ReservationSnapshot {
            inflight_global: self.inflight_global,
            running_global: self.running_global,
            reserved_prompt_bytes: self.reserved_prompt_bytes,
            reserved_output_tokens: self.reserved_output_tokens,
            reserved_output_bytes: self.reserved_output_bytes,
            entries: self.records.len(),
        }
    }
}

fn count<K: Ord>(map: &BTreeMap<K, usize>, key: &K) -> usize {
    map.get(key).copied().unwrap_or_default()
}

fn increment<K: Ord>(map: &mut BTreeMap<K, usize>, key: K) -> Result<()> {
    let count = map.entry(key).or_default();
    *count = count
        .checked_add(1)
        .ok_or(InferError::ReservationArithmeticOverflow)?;
    Ok(())
}

fn decrement<K: Ord>(map: &mut BTreeMap<K, usize>, key: &K) -> Result<()> {
    let remove = {
        let count = map.get_mut(key).ok_or(InferError::ReservationInvariant)?;
        *count = count
            .checked_sub(1)
            .ok_or(InferError::ReservationInvariant)?;
        *count == 0
    };
    if remove {
        map.remove(key);
    }
    Ok(())
}

fn checked_add_u64(left: u64, right: u64) -> Result<u64> {
    left.checked_add(right)
        .ok_or(InferError::ReservationArithmeticOverflow)
}

fn checked_sub_u64(left: u64, right: u64) -> Result<u64> {
    left.checked_sub(right)
        .ok_or(InferError::ReservationInvariant)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn must<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> T {
        match result {
            Ok(value) => value,
            Err(error) => panic!("unexpected error: {error}"),
        }
    }

    fn digest(fill: char) -> Digest {
        must(Digest::parse(&format!(
            "sha256:{}",
            fill.to_string().repeat(64)
        )))
    }

    fn scheduled(
        request_id: &str,
        tenant: &str,
        deadline: u64,
        enqueued: u64,
        cost: u64,
        weight: u32,
    ) -> ScheduledRequest {
        ScheduledRequest {
            request_id: must(RequestId::parse(request_id)),
            tenant_id: must(TenantId::parse(tenant)),
            tuple_digest: digest('a'),
            deadline_unix_ms: deadline,
            enqueued_unix_ms: enqueued,
            cost_units: cost,
            tenant_weight: weight,
        }
    }

    fn scheduler() -> DeterministicScheduler {
        must(DeterministicScheduler::new(SchedulerConfig {
            max_queued: 16,
            aging_quantum_ms: 10,
            max_cost_units: 1_000,
            max_tenant_weight: 16,
        }))
    }

    #[test]
    fn edf_is_deterministic_with_request_id_tie_break() {
        let first = scheduled("request-a", "tenant-a", 100, 1, 1, 1);
        let second = scheduled("request-b", "tenant-b", 100, 1, 1, 1);
        let mut left = scheduler();
        must(left.enqueue(second.clone()));
        must(left.enqueue(first.clone()));
        let mut right = scheduler();
        must(right.enqueue(first));
        must(right.enqueue(second));
        assert_eq!(
            must(left.pop_next(1)).map(|request| request.request_id),
            must(right.pop_next(1)).map(|request| request.request_id)
        );
    }

    #[test]
    fn aging_eventually_beats_newer_deadlines() {
        let mut scheduler = scheduler();
        must(scheduler.enqueue(scheduled(
            "request-old",
            "tenant-a",
            1_000,
            1,
            1,
            1,
        )));
        must(scheduler.enqueue(scheduled(
            "request-new",
            "tenant-b",
            500,
            490,
            1,
            1,
        )));
        let selected = must(scheduler.pop_next(1_001)).expect("one request must be selected");
        assert_eq!(selected.request_id.as_str(), "request-old");
    }

    #[test]
    fn weighted_service_prefers_under_served_tenant() {
        let mut scheduler = scheduler();
        must(scheduler.enqueue(scheduled(
            "request-heavy",
            "tenant-a",
            100,
            1,
            10,
            1,
        )));
        must(scheduler.enqueue(scheduled(
            "request-weighted",
            "tenant-b",
            100,
            1,
            10,
            10,
        )));
        let selected = must(scheduler.pop_next(1)).expect("one request must be selected");
        assert_eq!(selected.request_id.as_str(), "request-weighted");
    }

    fn limits() -> ReservationLimits {
        ReservationLimits {
            max_inflight_global: 2,
            max_inflight_per_tenant: 1,
            max_running_global: 2,
            max_running_per_tenant: 1,
            max_running_per_tuple: 1,
            max_reserved_prompt_bytes: 64,
            max_reserved_output_tokens: 16,
            max_reserved_output_bytes: 128,
        }
    }

    fn reservation(request_id: &str, tenant: &str, tuple: char) -> ReservationRequest {
        ReservationRequest {
            key: ReservationKey {
                request_id: must(RequestId::parse(request_id)),
                request_generation: 1,
                backend_generation: 7,
            },
            tenant_id: must(TenantId::parse(tenant)),
            tuple_digest: digest(tuple),
            prompt_bytes: 8,
            output_tokens: 4,
            output_bytes: 32,
        }
    }

    #[test]
    fn reservation_limits_are_independent_and_release_exactly_once() {
        let mut ledger = must(ReservationLedger::new(limits()));
        let first = reservation("request-a", "tenant-a", 'a');
        let first_key = first.key.clone();
        must(ledger.reserve(first));
        must(ledger.promote_running(&first_key));
        assert_eq!(ledger.phase(&first_key), Ok(ReservationPhase::Running));

        assert_eq!(
            ledger.reserve(reservation("request-b", "tenant-a", 'b')),
            Err(InferError::ReservationTenantFull)
        );
        let second = reservation("request-b", "tenant-b", 'a');
        let second_key = second.key.clone();
        must(ledger.reserve(second));
        assert_eq!(
            ledger.promote_running(&second_key),
            Err(InferError::ReservationRunningTupleFull)
        );

        must(ledger.release(&first_key));
        must(ledger.promote_running(&second_key));
        must(ledger.release(&second_key));
        assert_eq!(
            ledger.release(&second_key),
            Err(InferError::UnknownReservation)
        );
        assert_eq!(
            ledger.snapshot(),
            ReservationSnapshot {
                inflight_global: 0,
                running_global: 0,
                reserved_prompt_bytes: 0,
                reserved_output_tokens: 0,
                reserved_output_bytes: 0,
                entries: 0,
            }
        );
    }

    #[test]
    fn backend_generation_release_is_complete() {
        let mut ledger = must(ReservationLedger::new(limits()));
        must(ledger.reserve(reservation("request-a", "tenant-a", 'a')));
        must(ledger.reserve(reservation("request-b", "tenant-b", 'b')));
        assert_eq!(must(ledger.release_backend_generation(7)).len(), 2);
        assert_eq!(ledger.snapshot().entries, 0);
    }
}
