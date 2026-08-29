use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::AuthoritySnapshot;
use crate::Digest;
use crate::InferError;
use crate::InferenceRequest;
use crate::RequestId;
use crate::Result;
use crate::TenantId;
use crate::hashing::sha256;

const TOKEN_CHAIN_DOMAIN: &[u8] = b"hepta.inference.token-chain.v1\0";
const TOKEN_EVENT_DOMAIN: &[u8] = b"hepta.inference.token-event.v1\0";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum LifecycleState {
    Queued = 1,
    Running = 2,
    Draining = 3,
    Completed = 100,
    Cancelled = 101,
    FailedClosed = 102,
}

impl LifecycleState {
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Cancelled | Self::FailedClosed)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalReceipt {
    pub request_id: RequestId,
    pub request_generation: u64,
    pub cancel_generation: u64,
    pub backend_generation: u64,
    pub terminal_state: LifecycleState,
    pub last_sequence: u64,
    pub output_tokens: u32,
    pub result_digest: Option<Digest>,
    pub forced_worker_termination: bool,
    pub authority: AuthoritySnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptedEvent {
    pub request_id: RequestId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateEvent {
    pub request_id: RequestId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub sequence: u64,
    pub state: LifecycleState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerSnapshot {
    pub backend_generation: u64,
    pub queued_requests: usize,
    pub running_requests: usize,
    pub terminal_receipts: usize,
    pub registered_tuples: usize,
    pub max_queue: usize,
    pub max_per_tenant: usize,
    pub authority: AuthoritySnapshot,
}

#[derive(Clone, Debug)]
pub struct ControllerConfig {
    pub max_queue: usize,
    pub max_per_tenant: usize,
    pub registered_tuples: HashSet<Digest>,
    pub authority: AuthoritySnapshot,
}

impl ControllerConfig {
    pub fn qualification_only(registered_tuples: impl IntoIterator<Item = Digest>) -> Self {
        Self {
            max_queue: 64,
            max_per_tenant: 8,
            registered_tuples: registered_tuples.into_iter().collect(),
            authority: AuthoritySnapshot::qualification_only_closed(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        self.authority.validate_closed()?;
        if self.max_queue == 0 || self.max_per_tenant == 0 || self.registered_tuples.is_empty() {
            return Err(InferError::InvalidControllerConfig);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Record {
    request: InferenceRequest,
    state: LifecycleState,
    backend_generation: u64,
    cancel_generation: u64,
    last_sequence: u64,
    accepted_token_count: u32,
    accepted_token_bytes: u64,
    token_chain_digest: Digest,
    terminal_receipt: Option<TerminalReceipt>,
}

#[derive(Debug)]
pub struct Controller {
    config: ControllerConfig,
    backend_generation: u64,
    queue: VecDeque<RequestId>,
    queued_per_tenant: HashMap<TenantId, usize>,
    inflight_requests: usize,
    inflight_per_tenant: HashMap<TenantId, usize>,
    running_requests: usize,
    running_per_tenant: HashMap<TenantId, usize>,
    records: HashMap<RequestId, Record>,
}

impl Controller {
    pub fn new(config: ControllerConfig, backend_generation: u64) -> Result<Self> {
        config.validate()?;
        if backend_generation == 0 {
            return Err(InferError::InvalidGeneration);
        }
        Ok(Self {
            config,
            backend_generation,
            queue: VecDeque::new(),
            queued_per_tenant: HashMap::new(),
            inflight_requests: 0,
            inflight_per_tenant: HashMap::new(),
            running_requests: 0,
            running_per_tenant: HashMap::new(),
            records: HashMap::new(),
        })
    }

    pub const fn backend_generation(&self) -> u64 {
        self.backend_generation
    }

    pub const fn inflight_requests(&self) -> usize {
        self.inflight_requests
    }

    pub fn admit(&mut self, request: InferenceRequest, now_unix_ms: u64) -> Result<AcceptedEvent> {
        request.validate_shape()?;
        if request.deadline_unix_ms <= now_unix_ms {
            return Err(InferError::DeadlineExpired);
        }
        if !self
            .config
            .registered_tuples
            .contains(&request.model_tuple_digest)
        {
            return Err(InferError::UnknownModelTuple);
        }
        if self.records.contains_key(&request.identity.request_id) {
            return Err(InferError::DuplicateRequest);
        }
        if self.inflight_requests >= self.config.max_queue {
            return Err(InferError::InflightFull);
        }
        let tenant_inflight = self
            .inflight_per_tenant
            .get(&request.identity.tenant_id)
            .copied()
            .unwrap_or_default();
        if tenant_inflight >= self.config.max_per_tenant {
            return Err(InferError::TenantInflightFull);
        }
        if self.queue.len() >= self.config.max_queue {
            return Err(InferError::QueueFull);
        }
        let tenant_queued = self
            .queued_per_tenant
            .get(&request.identity.tenant_id)
            .copied()
            .unwrap_or_default();
        if tenant_queued >= self.config.max_per_tenant {
            return Err(InferError::TenantQueueFull);
        }

        let token_chain_digest = initial_token_chain(&request, self.backend_generation)?;
        let event = AcceptedEvent {
            request_id: request.identity.request_id.clone(),
            request_generation: request.request_generation,
            backend_generation: self.backend_generation,
            sequence: 1,
        };
        self.queue.push_back(request.identity.request_id.clone());
        increment_count(
            &mut self.queued_per_tenant,
            request.identity.tenant_id.clone(),
        );
        increment_count(
            &mut self.inflight_per_tenant,
            request.identity.tenant_id.clone(),
        );
        self.inflight_requests += 1;
        self.records.insert(
            request.identity.request_id.clone(),
            Record {
                cancel_generation: request.cancel_generation,
                request,
                state: LifecycleState::Queued,
                backend_generation: self.backend_generation,
                last_sequence: event.sequence,
                accepted_token_count: 0,
                accepted_token_bytes: 0,
                token_chain_digest,
                terminal_receipt: None,
            },
        );
        Ok(event)
    }

    pub fn start(
        &mut self,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
    ) -> Result<StateEvent> {
        self.validate_backend_generation(backend_generation)?;
        let (next_sequence, tenant) = {
            let record = self.record(request_id, request_generation)?;
            Self::ensure_nonterminal(record)?;
            if record.state != LifecycleState::Queued {
                return Err(InferError::InvalidTransition);
            }
            let next_sequence = record
                .last_sequence
                .checked_add(1)
                .ok_or(InferError::SequenceOverflow)?;
            (next_sequence, record.request.identity.tenant_id.clone())
        };
        if self.running_requests >= self.config.max_queue {
            return Err(InferError::RunningFull);
        }
        let tenant_running = self
            .running_per_tenant
            .get(&tenant)
            .copied()
            .unwrap_or_default();
        if tenant_running >= self.config.max_per_tenant {
            return Err(InferError::TenantRunningFull);
        }

        self.remove_from_queue(request_id)?;
        self.running_requests += 1;
        increment_count(&mut self.running_per_tenant, tenant);
        let record = self
            .records
            .get_mut(request_id)
            .ok_or(InferError::QueueInvariant)?;
        record.state = LifecycleState::Running;
        record.last_sequence = next_sequence;
        Ok(StateEvent {
            request_id: request_id.clone(),
            request_generation,
            backend_generation,
            sequence: next_sequence,
            state: LifecycleState::Running,
        })
    }

    pub fn publish_token(
        &mut self,
        fence: EventFence<'_>,
        token_digest: &Digest,
        token_byte_length: u64,
    ) -> Result<StateEvent> {
        if token_byte_length == 0 {
            return Err(InferError::EmptyToken);
        }
        self.validate_backend_generation(fence.backend_generation)?;
        let record = self.record_mut(fence.request_id, fence.request_generation)?;
        Self::ensure_nonterminal(record)?;
        if record.state != LifecycleState::Running {
            return Err(InferError::InvalidTransition);
        }
        Self::validate_next_sequence(record, fence.sequence)?;
        let next_count = record
            .accepted_token_count
            .checked_add(1)
            .ok_or(InferError::OutputTokenLimitExceeded)?;
        if next_count > record.request.output_token_limit {
            return Err(InferError::OutputTokenLimitExceeded);
        }
        let next_bytes = record
            .accepted_token_bytes
            .checked_add(token_byte_length)
            .ok_or(InferError::ProtocolBound)?;
        let next_digest = next_token_chain(
            &record.token_chain_digest,
            fence.sequence,
            token_digest,
            token_byte_length,
        )?;
        record.accepted_token_count = next_count;
        record.accepted_token_bytes = next_bytes;
        record.token_chain_digest = next_digest;
        record.last_sequence = fence.sequence;
        Ok(StateEvent {
            request_id: fence.request_id.clone(),
            request_generation: fence.request_generation,
            backend_generation: fence.backend_generation,
            sequence: fence.sequence,
            state: record.state,
        })
    }

    pub fn current_token_chain_digest(
        &self,
        request_id: &RequestId,
        request_generation: u64,
    ) -> Result<&Digest> {
        Ok(&self
            .record(request_id, request_generation)?
            .token_chain_digest)
    }

    pub fn current_token_metrics(
        &self,
        request_id: &RequestId,
        request_generation: u64,
    ) -> Result<(u32, u64, &Digest)> {
        let record = self.record(request_id, request_generation)?;
        Ok((
            record.accepted_token_count,
            record.accepted_token_bytes,
            &record.token_chain_digest,
        ))
    }

    pub fn complete(
        &mut self,
        fence: EventFence<'_>,
        result_digest: Digest,
        output_tokens: u32,
    ) -> Result<TerminalReceipt> {
        self.validate_backend_generation(fence.backend_generation)?;
        let authority = self.config.authority.clone();
        let tenant = {
            let record = self.record(fence.request_id, fence.request_generation)?;
            Self::ensure_nonterminal(record)?;
            if record.state != LifecycleState::Running {
                return Err(InferError::InvalidTransition);
            }
            Self::validate_next_sequence(record, fence.sequence)?;
            if output_tokens != record.accepted_token_count {
                return Err(InferError::OutputTokenCountMismatch);
            }
            if result_digest != record.token_chain_digest {
                return Err(InferError::ResultDigestMismatch);
            }
            self.ensure_active_accounting(&record.request.identity.tenant_id, true)?;
            record.request.identity.tenant_id.clone()
        };

        let receipt = {
            let record = self
                .records
                .get_mut(fence.request_id)
                .ok_or(InferError::QueueInvariant)?;
            record.state = LifecycleState::Completed;
            record.last_sequence = fence.sequence;
            let receipt = TerminalReceipt {
                request_id: fence.request_id.clone(),
                request_generation: fence.request_generation,
                cancel_generation: record.cancel_generation,
                backend_generation: fence.backend_generation,
                terminal_state: LifecycleState::Completed,
                last_sequence: fence.sequence,
                output_tokens: record.accepted_token_count,
                result_digest: Some(record.token_chain_digest.clone()),
                forced_worker_termination: false,
                authority,
            };
            record.terminal_receipt = Some(receipt.clone());
            receipt
        };
        self.release_active_accounting(&tenant, true)?;
        Ok(receipt)
    }

    pub fn cancel(
        &mut self,
        request_id: &RequestId,
        request_generation: u64,
        cancel_generation: u64,
        backend_generation: u64,
    ) -> Result<TerminalReceipt> {
        self.validate_backend_generation(backend_generation)?;
        let (tenant, next_sequence) = {
            let record = self.record(request_id, request_generation)?;
            Self::ensure_nonterminal(record)?;
            if record.state != LifecycleState::Queued {
                return Err(InferError::WorkerCancellationRequired);
            }
            if cancel_generation <= record.cancel_generation {
                return Err(InferError::StaleCancelGeneration);
            }
            self.ensure_active_accounting(&record.request.identity.tenant_id, false)?;
            let next_sequence = record
                .last_sequence
                .checked_add(1)
                .ok_or(InferError::SequenceOverflow)?;
            (record.request.identity.tenant_id.clone(), next_sequence)
        };
        self.remove_from_queue(request_id)?;
        let authority = self.config.authority.clone();
        let receipt = {
            let record = self
                .records
                .get_mut(request_id)
                .ok_or(InferError::QueueInvariant)?;
            record.cancel_generation = cancel_generation;
            record.state = LifecycleState::Cancelled;
            record.last_sequence = next_sequence;
            let receipt = TerminalReceipt {
                request_id: request_id.clone(),
                request_generation,
                cancel_generation,
                backend_generation,
                terminal_state: LifecycleState::Cancelled,
                last_sequence: next_sequence,
                output_tokens: 0,
                result_digest: None,
                forced_worker_termination: false,
                authority,
            };
            record.terminal_receipt = Some(receipt.clone());
            receipt
        };
        self.release_active_accounting(&tenant, false)?;
        Ok(receipt)
    }

    pub fn expire_deadlines(&mut self, now_unix_ms: u64) -> Result<Vec<TerminalReceipt>> {
        let expired: Vec<RequestId> = self
            .records
            .iter()
            .filter(|(_, record)| {
                !record.state.is_terminal() && record.request.deadline_unix_ms <= now_unix_ms
            })
            .map(|(request_id, _)| request_id.clone())
            .collect();
        let mut receipts = Vec::with_capacity(expired.len());
        for request_id in expired {
            let (request_generation, tenant, queued, running, next_sequence, cancel_generation) = {
                let record = self
                    .records
                    .get(&request_id)
                    .ok_or(InferError::QueueInvariant)?;
                let queued = record.state == LifecycleState::Queued;
                let running = matches!(
                    record.state,
                    LifecycleState::Running | LifecycleState::Draining
                );
                self.ensure_active_accounting(&record.request.identity.tenant_id, running)?;
                (
                    record.request.request_generation,
                    record.request.identity.tenant_id.clone(),
                    queued,
                    running,
                    record
                        .last_sequence
                        .checked_add(1)
                        .ok_or(InferError::SequenceOverflow)?,
                    record.cancel_generation,
                )
            };
            if queued {
                self.remove_from_queue(&request_id)?;
            }
            let authority = self.config.authority.clone();
            let receipt = {
                let record = self
                    .records
                    .get_mut(&request_id)
                    .ok_or(InferError::QueueInvariant)?;
                record.state = LifecycleState::FailedClosed;
                record.last_sequence = next_sequence;
                let receipt = TerminalReceipt {
                    request_id: request_id.clone(),
                    request_generation,
                    cancel_generation,
                    backend_generation: record.backend_generation,
                    terminal_state: LifecycleState::FailedClosed,
                    last_sequence: next_sequence,
                    output_tokens: 0,
                    result_digest: None,
                    forced_worker_termination: running,
                    authority,
                };
                record.terminal_receipt = Some(receipt.clone());
                receipt
            };
            self.release_active_accounting(&tenant, running)?;
            receipts.push(receipt);
        }
        receipts.sort_by(|left, right| left.request_id.cmp(&right.request_id));
        Ok(receipts)
    }

    pub fn restart_backend(&mut self, expected_generation: u64) -> Result<Vec<TerminalReceipt>> {
        self.validate_backend_generation(expected_generation)?;
        let next_generation = self
            .backend_generation
            .checked_add(1)
            .ok_or(InferError::GenerationOverflow)?;
        for record in self.records.values() {
            if !record.state.is_terminal() && record.last_sequence.checked_add(1).is_none() {
                return Err(InferError::SequenceOverflow);
            }
        }

        self.backend_generation = next_generation;
        let authority = self.config.authority.clone();
        let mut receipts = Vec::new();
        for record in self.records.values_mut() {
            if record.state.is_terminal() {
                continue;
            }
            let forced_worker_termination = matches!(
                record.state,
                LifecycleState::Running | LifecycleState::Draining
            );
            let next_sequence = record
                .last_sequence
                .checked_add(1)
                .ok_or(InferError::SequenceOverflow)?;
            record.state = LifecycleState::FailedClosed;
            record.last_sequence = next_sequence;
            let receipt = TerminalReceipt {
                request_id: record.request.identity.request_id.clone(),
                request_generation: record.request.request_generation,
                cancel_generation: record.cancel_generation,
                backend_generation: expected_generation,
                terminal_state: LifecycleState::FailedClosed,
                last_sequence: next_sequence,
                output_tokens: 0,
                result_digest: None,
                forced_worker_termination,
                authority: authority.clone(),
            };
            record.terminal_receipt = Some(receipt.clone());
            receipts.push(receipt);
        }
        self.queue.clear();
        self.queued_per_tenant.clear();
        self.inflight_requests = 0;
        self.inflight_per_tenant.clear();
        self.running_requests = 0;
        self.running_per_tenant.clear();
        receipts.sort_by(|left, right| left.request_id.cmp(&right.request_id));
        Ok(receipts)
    }

    pub fn terminal_receipt(&self, request_id: &RequestId) -> Result<&TerminalReceipt> {
        self.records
            .get(request_id)
            .ok_or(InferError::UnknownRequest)?
            .terminal_receipt
            .as_ref()
            .ok_or(InferError::RequestNotTerminal)
    }

    pub fn terminal_receipt_fenced(
        &self,
        request_id: &RequestId,
        request_generation: u64,
        backend_generation: u64,
        minimum_sequence: u64,
    ) -> Result<&TerminalReceipt> {
        let record = self
            .records
            .get(request_id)
            .ok_or(InferError::UnknownRequest)?;
        if record.request.request_generation != request_generation {
            return Err(InferError::StaleRequestGeneration);
        }
        let receipt = record
            .terminal_receipt
            .as_ref()
            .ok_or(InferError::RequestNotTerminal)?;
        if receipt.backend_generation != backend_generation {
            return Err(InferError::StaleBackendGeneration);
        }
        if receipt.last_sequence < minimum_sequence {
            return Err(InferError::ReceiptSequenceNotReached);
        }
        Ok(receipt)
    }

    pub fn forget_terminal(&mut self, request_id: &RequestId) -> Result<TerminalReceipt> {
        let receipt = self.terminal_receipt(request_id)?.clone();
        let removed = self
            .records
            .remove(request_id)
            .ok_or(InferError::UnknownRequest)?;
        if !removed.state.is_terminal() {
            return Err(InferError::RequestNotTerminal);
        }
        Ok(receipt)
    }

    pub fn snapshot(&self) -> ControllerSnapshot {
        let terminal_receipts = self
            .records
            .values()
            .filter(|record| record.terminal_receipt.is_some())
            .count();
        ControllerSnapshot {
            backend_generation: self.backend_generation,
            queued_requests: self.queue.len(),
            running_requests: self.running_requests,
            terminal_receipts,
            registered_tuples: self.config.registered_tuples.len(),
            max_queue: self.config.max_queue,
            max_per_tenant: self.config.max_per_tenant,
            authority: self.config.authority.clone(),
        }
    }

    fn record(&self, request_id: &RequestId, request_generation: u64) -> Result<&Record> {
        let record = self
            .records
            .get(request_id)
            .ok_or(InferError::UnknownRequest)?;
        if record.request.request_generation != request_generation {
            return Err(InferError::StaleRequestGeneration);
        }
        if record.backend_generation != self.backend_generation {
            return Err(InferError::StaleBackendGeneration);
        }
        Ok(record)
    }

    fn record_mut(
        &mut self,
        request_id: &RequestId,
        request_generation: u64,
    ) -> Result<&mut Record> {
        let backend_generation = self.backend_generation;
        let record = self
            .records
            .get_mut(request_id)
            .ok_or(InferError::UnknownRequest)?;
        if record.request.request_generation != request_generation {
            return Err(InferError::StaleRequestGeneration);
        }
        if record.backend_generation != backend_generation {
            return Err(InferError::StaleBackendGeneration);
        }
        Ok(record)
    }

    fn ensure_nonterminal(record: &Record) -> Result<()> {
        if record.state.is_terminal() {
            Err(InferError::TerminalState)
        } else {
            Ok(())
        }
    }

    fn validate_next_sequence(record: &Record, sequence: u64) -> Result<()> {
        let expected = record
            .last_sequence
            .checked_add(1)
            .ok_or(InferError::SequenceOverflow)?;
        if sequence == expected {
            Ok(())
        } else {
            Err(InferError::StaleOrNonMonotonicSequence)
        }
    }

    fn validate_backend_generation(&self, backend_generation: u64) -> Result<()> {
        if backend_generation == self.backend_generation {
            Ok(())
        } else {
            Err(InferError::StaleBackendGeneration)
        }
    }

    fn ensure_active_accounting(&self, tenant: &TenantId, running: bool) -> Result<()> {
        if self.inflight_requests == 0
            || self
                .inflight_per_tenant
                .get(tenant)
                .copied()
                .unwrap_or_default()
                == 0
        {
            return Err(InferError::QueueInvariant);
        }
        if running
            && (self.running_requests == 0
                || self
                    .running_per_tenant
                    .get(tenant)
                    .copied()
                    .unwrap_or_default()
                    == 0)
        {
            return Err(InferError::QueueInvariant);
        }
        Ok(())
    }

    fn release_active_accounting(&mut self, tenant: &TenantId, running: bool) -> Result<()> {
        self.inflight_requests = self
            .inflight_requests
            .checked_sub(1)
            .ok_or(InferError::QueueInvariant)?;
        decrement_count(&mut self.inflight_per_tenant, tenant)?;
        if running {
            self.running_requests = self
                .running_requests
                .checked_sub(1)
                .ok_or(InferError::QueueInvariant)?;
            decrement_count(&mut self.running_per_tenant, tenant)?;
        }
        Ok(())
    }

    fn remove_from_queue(&mut self, request_id: &RequestId) -> Result<()> {
        let index = self
            .queue
            .iter()
            .position(|candidate| candidate == request_id)
            .ok_or(InferError::QueueInvariant)?;
        let removed = self.queue.remove(index).ok_or(InferError::QueueInvariant)?;
        let tenant = self
            .records
            .get(&removed)
            .ok_or(InferError::QueueInvariant)?
            .request
            .identity
            .tenant_id
            .clone();
        decrement_count(&mut self.queued_per_tenant, &tenant)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EventFence<'a> {
    pub request_id: &'a RequestId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub sequence: u64,
}

fn increment_count(map: &mut HashMap<TenantId, usize>, tenant: TenantId) {
    *map.entry(tenant).or_default() += 1;
}

fn decrement_count(map: &mut HashMap<TenantId, usize>, tenant: &TenantId) -> Result<()> {
    let remove = {
        let count = map.get_mut(tenant).ok_or(InferError::QueueInvariant)?;
        *count = count.checked_sub(1).ok_or(InferError::QueueInvariant)?;
        *count == 0
    };
    if remove {
        map.remove(tenant);
    }
    Ok(())
}

fn initial_token_chain(request: &InferenceRequest, backend_generation: u64) -> Result<Digest> {
    let mut preimage = Vec::with_capacity(1024);
    preimage.extend_from_slice(TOKEN_CHAIN_DOMAIN);
    append_text(&mut preimage, request.identity.tenant_id.as_str())?;
    append_text(&mut preimage, request.identity.workspace_id.as_str())?;
    append_text(&mut preimage, request.identity.agent_id.as_str())?;
    append_text(&mut preimage, request.identity.task_id.as_str())?;
    append_text(&mut preimage, request.identity.request_id.as_str())?;
    append_u64(&mut preimage, request.agent_generation);
    append_u64(&mut preimage, request.request_generation);
    append_u64(&mut preimage, request.cancel_generation);
    append_u64(&mut preimage, backend_generation);
    append_u64(&mut preimage, request.deadline_unix_ms);
    append_text(&mut preimage, request.model_tuple_digest.as_str())?;
    append_text(&mut preimage, request.policy_digest.as_str())?;
    append_text(&mut preimage, request.resource_budget_id.as_str())?;
    append_text(&mut preimage, request.prompt_digest.as_str())?;
    append_u64(&mut preimage, request.prompt_byte_length);
    append_u64(&mut preimage, u64::from(request.output_token_limit));
    digest_from_bytes(sha256(&[preimage.as_slice()])?)
}

fn next_token_chain(
    previous: &Digest,
    sequence: u64,
    token_digest: &Digest,
    token_byte_length: u64,
) -> Result<Digest> {
    let mut preimage = Vec::with_capacity(192);
    preimage.extend_from_slice(TOKEN_EVENT_DOMAIN);
    append_text(&mut preimage, previous.as_str())?;
    append_u64(&mut preimage, sequence);
    append_text(&mut preimage, token_digest.as_str())?;
    append_u64(&mut preimage, token_byte_length);
    digest_from_bytes(sha256(&[preimage.as_slice()])?)
}

fn append_text(buffer: &mut Vec<u8>, value: &str) -> Result<()> {
    let length = u64::try_from(value.len()).map_err(|_| InferError::ProtocolBound)?;
    append_u64(buffer, length);
    buffer.extend_from_slice(value.as_bytes());
    Ok(())
}

fn append_u64(buffer: &mut Vec<u8>, value: u64) {
    buffer.extend_from_slice(&value.to_be_bytes());
}

fn digest_from_bytes(bytes: [u8; 32]) -> Result<Digest> {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(71);
    encoded.push_str("sha256:");
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    Digest::parse(&encoded)
}
