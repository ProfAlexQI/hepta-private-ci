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
        if self.max_queue == 0 || self.max_per_tenant == 0 {
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
    terminal_receipt: Option<TerminalReceipt>,
}

#[derive(Debug)]
pub struct Controller {
    config: ControllerConfig,
    backend_generation: u64,
    queue: VecDeque<RequestId>,
    queued_per_tenant: HashMap<TenantId, usize>,
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
            records: HashMap::new(),
        })
    }

    pub const fn backend_generation(&self) -> u64 {
        self.backend_generation
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

        let event = AcceptedEvent {
            request_id: request.identity.request_id.clone(),
            request_generation: request.request_generation,
            backend_generation: self.backend_generation,
            sequence: 1,
        };
        self.queue.push_back(request.identity.request_id.clone());
        *self
            .queued_per_tenant
            .entry(request.identity.tenant_id.clone())
            .or_default() += 1;
        self.records.insert(
            request.identity.request_id.clone(),
            Record {
                cancel_generation: request.cancel_generation,
                request,
                state: LifecycleState::Queued,
                backend_generation: self.backend_generation,
                last_sequence: event.sequence,
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
        let next_sequence = {
            let record = self.record(request_id, request_generation)?;
            Self::ensure_nonterminal(record)?;
            if record.state != LifecycleState::Queued {
                return Err(InferError::InvalidTransition);
            }
            record
                .last_sequence
                .checked_add(1)
                .ok_or(InferError::SequenceOverflow)?
        };

        // Remove the request from the bounded queue before changing lifecycle state.
        // Every fallible precondition has already been checked, so a queue invariant
        // failure cannot leave a Running record behind in the queued accounting.
        self.remove_from_queue(request_id)?;
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
        let _ = token_digest;
        record.last_sequence = fence.sequence;
        Ok(StateEvent {
            request_id: fence.request_id.clone(),
            request_generation: fence.request_generation,
            backend_generation: fence.backend_generation,
            sequence: fence.sequence,
            state: record.state,
        })
    }

    pub fn complete(
        &mut self,
        fence: EventFence<'_>,
        result_digest: Digest,
        output_tokens: u32,
    ) -> Result<TerminalReceipt> {
        if output_tokens == 0 {
            return Err(InferError::EmptyOutputLimit);
        }
        self.validate_backend_generation(fence.backend_generation)?;
        let authority = self.config.authority.clone();
        let record = self.record_mut(fence.request_id, fence.request_generation)?;
        Self::ensure_nonterminal(record)?;
        if record.state != LifecycleState::Running {
            return Err(InferError::InvalidTransition);
        }
        Self::validate_next_sequence(record, fence.sequence)?;
        record.state = LifecycleState::Completed;
        record.last_sequence = fence.sequence;
        let receipt = TerminalReceipt {
            request_id: fence.request_id.clone(),
            request_generation: fence.request_generation,
            cancel_generation: record.cancel_generation,
            backend_generation: fence.backend_generation,
            terminal_state: LifecycleState::Completed,
            last_sequence: fence.sequence,
            output_tokens,
            result_digest: Some(result_digest),
            forced_worker_termination: false,
            authority,
        };
        record.terminal_receipt = Some(receipt.clone());
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
        let (queued, next_sequence) = {
            let record = self.record(request_id, request_generation)?;
            Self::ensure_nonterminal(record)?;
            if cancel_generation <= record.cancel_generation {
                return Err(InferError::StaleCancelGeneration);
            }
            let next_sequence = record
                .last_sequence
                .checked_add(1)
                .ok_or(InferError::SequenceOverflow)?;
            (record.state == LifecycleState::Queued, next_sequence)
        };
        if queued {
            self.remove_from_queue(request_id)?;
        }
        let authority = self.config.authority.clone();
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
        Ok(receipt)
    }

    pub fn restart_backend(&mut self, expected_generation: u64) -> Result<Vec<TerminalReceipt>> {
        self.validate_backend_generation(expected_generation)?;
        let next_generation = self
            .backend_generation
            .checked_add(1)
            .ok_or(InferError::GenerationOverflow)?;

        // Preflight every sequence increment before mutating either the backend
        // generation or any record. Restart is therefore all-or-nothing.
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
                forced_worker_termination: true,
                authority: authority.clone(),
            };
            record.terminal_receipt = Some(receipt.clone());
            receipts.push(receipt);
        }
        self.queue.clear();
        self.queued_per_tenant.clear();
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

    pub fn snapshot(&self) -> ControllerSnapshot {
        let mut running_requests = 0usize;
        let mut terminal_receipts = 0usize;
        for record in self.records.values() {
            if record.state == LifecycleState::Running {
                running_requests += 1;
            }
            if record.terminal_receipt.is_some() {
                terminal_receipts += 1;
            }
        }
        ControllerSnapshot {
            backend_generation: self.backend_generation,
            queued_requests: self.queue.len(),
            running_requests,
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
        if sequence == record.last_sequence.saturating_add(1) {
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
        let remove_tenant = {
            let count = self
                .queued_per_tenant
                .get_mut(&tenant)
                .ok_or(InferError::QueueInvariant)?;
            *count = count.checked_sub(1).ok_or(InferError::QueueInvariant)?;
            *count == 0
        };
        if remove_tenant {
            self.queued_per_tenant.remove(&tenant);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EventFence<'a> {
    pub request_id: &'a RequestId,
    pub request_generation: u64,
    pub backend_generation: u64,
    pub sequence: u64,
}
