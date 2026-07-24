//! Atomic execution/mutation coordination for frozen safety context.
//!
//! The capability catalog is immutable after [`RuntimeKernel`] construction,
//! and the current preference stamp is explicitly unattached. This registry
//! therefore coordinates the mutable global policy plus the per-session model,
//! execution profile, filesystem scope, write scope, and path capability gates.
//!
//! Registry locks are held only while epochs and active markers are updated.
//! Neither [`ExecutionLease`] nor [`MutationMarker`] retains a `MutexGuard`.

use crate::HeptaError;
use crate::PreparedReadCapability;
use crate::PreparedWriteReservationSet;
use crate::PreparedWriteTransaction;
use crate::RuntimeKernel;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;

/// Stable private failures for execution/mutation coordination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExecutionLeaseError {
    RegistryPoisoned,
    EpochExhausted,
    GlobalMutationActive,
    SessionMutationActive { session_id: String },
    ExecutionInFlight { session_id: String },
    FrozenContextChanged { session_id: String },
}

impl ExecutionLeaseError {
    pub(crate) const fn code(&self) -> &'static str {
        match self {
            Self::RegistryPoisoned => "execution_lease.registry_poisoned",
            Self::EpochExhausted => "execution_lease.epoch_exhausted",
            Self::GlobalMutationActive => "execution_lease.global_mutation_active",
            Self::SessionMutationActive { .. } => "execution_lease.session_mutation_active",
            Self::ExecutionInFlight { .. } => "execution_lease.execution_in_flight",
            Self::FrozenContextChanged { .. } => "execution_lease.frozen_context_changed",
        }
    }
}

impl fmt::Display for ExecutionLeaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())?;
        match self {
            Self::SessionMutationActive { session_id }
            | Self::ExecutionInFlight { session_id }
            | Self::FrozenContextChanged { session_id } => {
                write!(formatter, ": session={session_id}")
            }
            Self::RegistryPoisoned | Self::EpochExhausted | Self::GlobalMutationActive => Ok(()),
        }
    }
}

impl Error for ExecutionLeaseError {}

#[derive(Debug, Default)]
pub(crate) struct ExecutionLeaseRegistry {
    global_epoch: u64,
    session_epochs: BTreeMap<String, u64>,
    global_mutation_active: bool,
    mutating_sessions: BTreeSet<String>,
    in_flight_sessions: BTreeSet<String>,
    epoch_exhausted: bool,
}

/// Epochs captured immediately before final policy evaluation and freezing.
#[derive(Debug)]
pub(crate) struct ExecutionEpoch {
    session_id: String,
    global_epoch: u64,
    session_epoch: u64,
}

/// Non-cloneable proof that frozen-context mutation is excluded for a session.
#[derive(Debug)]
pub(crate) struct ExecutionLease {
    registry: Arc<Mutex<ExecutionLeaseRegistry>>,
    session_id: String,
    active: bool,
}

/// Non-cloneable execution lease after all tool-specific resources are held.
///
/// `AuthorizedToolExecution` can only be constructed with this type. The
/// prepared set owns every filesystem identity reservation from before-state
/// capture through terminal receipt finalization.
#[derive(Debug)]
pub(crate) struct ResourceBoundExecutionLease {
    lease: ExecutionLease,
    prepared_read: Option<PreparedReadCapability>,
    prepared_writes: PreparedWriteReservationSet,
}

/// Non-cloneable marker that excludes execution while a mutation is applied.
#[derive(Debug)]
pub(crate) struct MutationMarker {
    registry: Arc<Mutex<ExecutionLeaseRegistry>>,
    scope: MutationScope,
    active: bool,
}

#[derive(Debug)]
enum MutationScope {
    Global,
    Sessions(Vec<String>),
}

impl RuntimeKernel {
    pub(crate) fn capture_execution_epoch(
        &self,
        session_id: &str,
    ) -> Result<ExecutionEpoch, HeptaError> {
        let registry = self.lock_execution_lease_registry()?;
        ensure_registry_open(&registry)?;
        if registry.global_mutation_active {
            return Err(lease_error(ExecutionLeaseError::GlobalMutationActive));
        }
        if registry.mutating_sessions.contains(session_id) {
            return Err(lease_error(ExecutionLeaseError::SessionMutationActive {
                session_id: session_id.to_string(),
            }));
        }
        if registry.in_flight_sessions.contains(session_id) {
            return Err(lease_error(ExecutionLeaseError::ExecutionInFlight {
                session_id: session_id.to_string(),
            }));
        }
        Ok(ExecutionEpoch {
            session_id: session_id.to_string(),
            global_epoch: registry.global_epoch,
            session_epoch: session_epoch(&registry, session_id),
        })
    }

    pub(crate) fn begin_execution_lease(
        &self,
        expected: ExecutionEpoch,
    ) -> Result<ExecutionLease, HeptaError> {
        let mut registry = self.lock_execution_lease_registry()?;
        ensure_registry_open(&registry)?;
        if registry.global_mutation_active {
            return Err(lease_error(ExecutionLeaseError::GlobalMutationActive));
        }
        if registry.mutating_sessions.contains(&expected.session_id) {
            return Err(lease_error(ExecutionLeaseError::SessionMutationActive {
                session_id: expected.session_id,
            }));
        }
        if registry.in_flight_sessions.contains(&expected.session_id) {
            return Err(lease_error(ExecutionLeaseError::ExecutionInFlight {
                session_id: expected.session_id,
            }));
        }
        if registry.global_epoch != expected.global_epoch
            || session_epoch(&registry, &expected.session_id) != expected.session_epoch
        {
            return Err(lease_error(ExecutionLeaseError::FrozenContextChanged {
                session_id: expected.session_id,
            }));
        }
        registry
            .in_flight_sessions
            .insert(expected.session_id.clone());
        drop(registry);
        Ok(ExecutionLease {
            registry: Arc::clone(&self.execution_lease_registry),
            session_id: expected.session_id,
            active: true,
        })
    }

    pub(crate) fn begin_global_context_mutation(&self) -> Result<MutationMarker, HeptaError> {
        let mut registry = self.lock_execution_lease_registry()?;
        ensure_registry_open(&registry)?;
        if registry.global_mutation_active || !registry.mutating_sessions.is_empty() {
            return Err(lease_error(ExecutionLeaseError::GlobalMutationActive));
        }
        if let Some(session_id) = registry.in_flight_sessions.iter().next() {
            return Err(lease_error(ExecutionLeaseError::ExecutionInFlight {
                session_id: session_id.clone(),
            }));
        }
        registry.global_mutation_active = true;
        drop(registry);
        Ok(MutationMarker {
            registry: Arc::clone(&self.execution_lease_registry),
            scope: MutationScope::Global,
            active: true,
        })
    }

    pub(crate) fn begin_session_context_mutation(
        &self,
        session_id: &str,
    ) -> Result<MutationMarker, HeptaError> {
        self.begin_sessions_context_mutation([session_id])
    }

    pub(crate) fn begin_sessions_context_mutation<'a>(
        &self,
        session_ids: impl IntoIterator<Item = &'a str>,
    ) -> Result<MutationMarker, HeptaError> {
        let sessions = session_ids
            .into_iter()
            .map(str::trim)
            .filter(|session_id| !session_id.is_empty())
            .map(ToOwned::to_owned)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let mut registry = self.lock_execution_lease_registry()?;
        ensure_registry_open(&registry)?;
        if registry.global_mutation_active {
            return Err(lease_error(ExecutionLeaseError::GlobalMutationActive));
        }
        if let Some(session_id) = sessions
            .iter()
            .find(|session_id| registry.mutating_sessions.contains(*session_id))
        {
            return Err(lease_error(ExecutionLeaseError::SessionMutationActive {
                session_id: session_id.clone(),
            }));
        }
        if let Some(session_id) = sessions
            .iter()
            .find(|session_id| registry.in_flight_sessions.contains(*session_id))
        {
            return Err(lease_error(ExecutionLeaseError::ExecutionInFlight {
                session_id: session_id.clone(),
            }));
        }
        registry.mutating_sessions.extend(sessions.iter().cloned());
        drop(registry);
        Ok(MutationMarker {
            registry: Arc::clone(&self.execution_lease_registry),
            scope: MutationScope::Sessions(sessions),
            active: true,
        })
    }

    fn lock_execution_lease_registry(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, ExecutionLeaseRegistry>, HeptaError> {
        self.execution_lease_registry
            .lock()
            .map_err(|_| lease_error(ExecutionLeaseError::RegistryPoisoned))
    }
}

impl ExecutionLease {
    pub(crate) fn bind_tool_resources(
        self,
        runtime: &RuntimeKernel,
        session_id: &str,
        tool_name: &str,
        canonical_arguments: &str,
    ) -> Result<ResourceBoundExecutionLease, HeptaError> {
        if self.session_id != session_id {
            return Err(HeptaError(format!(
                "execution lease session mismatch: lease={} execution={session_id}",
                self.session_id
            )));
        }
        let prepared_read =
            runtime.prepare_read_capability(session_id, tool_name, canonical_arguments)?;
        let prepared_writes = runtime.prepare_write_transactions_with_lock_check(
            session_id,
            tool_name,
            canonical_arguments,
        )?;
        Ok(ResourceBoundExecutionLease {
            lease: self,
            prepared_read,
            prepared_writes,
        })
    }

    fn release_context_exclusion(&mut self) {
        if !self.active {
            return;
        }
        let mut registry = match self.registry.lock() {
            Ok(registry) => registry,
            Err(poisoned) => poisoned.into_inner(),
        };
        registry.in_flight_sessions.remove(&self.session_id);
        self.active = false;
    }
}

impl ResourceBoundExecutionLease {
    pub(super) fn prepared_read_capability(&self) -> Option<&PreparedReadCapability> {
        self.prepared_read.as_ref()
    }

    pub(super) fn prepared_write_transactions(&self) -> &[PreparedWriteTransaction] {
        &self.prepared_writes.transactions
    }

    pub(super) fn release_context_exclusion(&mut self) {
        self.lease.release_context_exclusion();
    }

    pub(super) fn holds_write_target_reservation(&self) -> bool {
        !self.prepared_writes.transactions.is_empty()
    }

    #[cfg(test)]
    pub(super) fn holds_read_capability(&self) -> bool {
        self.prepared_read.is_some()
    }
}

impl Drop for ExecutionLease {
    fn drop(&mut self) {
        self.release_context_exclusion();
    }
}

impl Drop for MutationMarker {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let mut registry = match self.registry.lock() {
            Ok(registry) => registry,
            Err(poisoned) => poisoned.into_inner(),
        };
        match &self.scope {
            MutationScope::Global => {
                registry.global_mutation_active = false;
                registry.global_epoch =
                    next_epoch(registry.global_epoch, &mut registry.epoch_exhausted);
            }
            MutationScope::Sessions(session_ids) => {
                for session_id in session_ids {
                    registry.mutating_sessions.remove(session_id);
                    let current = session_epoch(&registry, session_id);
                    let next = next_epoch(current, &mut registry.epoch_exhausted);
                    registry.session_epochs.insert(session_id.clone(), next);
                }
            }
        }
        self.active = false;
    }
}

fn ensure_registry_open(registry: &ExecutionLeaseRegistry) -> Result<(), HeptaError> {
    if registry.epoch_exhausted {
        Err(lease_error(ExecutionLeaseError::EpochExhausted))
    } else {
        Ok(())
    }
}

fn session_epoch(registry: &ExecutionLeaseRegistry, session_id: &str) -> u64 {
    registry
        .session_epochs
        .get(session_id)
        .copied()
        .unwrap_or_default()
}

fn next_epoch(current: u64, exhausted: &mut bool) -> u64 {
    match current.checked_add(1) {
        Some(next) => next,
        None => {
            *exhausted = true;
            current
        }
    }
}

fn lease_error(error: ExecutionLeaseError) -> HeptaError {
    HeptaError(error.to_string())
}
