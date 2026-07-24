use std::collections::BTreeMap;

use hepta_core::HeptaError;

use super::ExactOutcomeRecord;
use super::OutcomeReceiptSinkError;
use super::PendingOutcomeKind;

#[derive(Debug, Clone)]
struct PendingOutcome {
    exact: ExactOutcomeRecord,
    failure: OutcomeReceiptSinkError,
    kind: PendingOutcomeKind,
    reconciliation_in_flight: bool,
}

/// Structured fail-closed state for fatal, safe-retry, and ambiguous causes.
#[derive(Debug, Default)]
pub(crate) struct OutcomeBreakerState {
    fatal_reason: Option<String>,
    pending: BTreeMap<String, PendingOutcome>,
}

impl OutcomeBreakerState {
    pub(crate) fn reason(&self) -> Option<String> {
        if let Some(reason) = &self.fatal_reason {
            return Some(reason.clone());
        }
        self.pending.iter().next().map(|(attempt_id, pending)| {
            format!(
                "durable outcome attempt {attempt_id} requires exact {}: {}",
                match pending.kind {
                    PendingOutcomeKind::SafeRetry => "retry",
                    PendingOutcomeKind::CommitAmbiguous => "reconciliation",
                },
                pending.failure,
            )
        })
    }

    pub(crate) fn trip_fatal(&mut self, reason: impl Into<String>) {
        if self.fatal_reason.is_none() {
            self.fatal_reason = Some(reason.into());
        }
    }

    pub(super) fn retain_pending(
        &mut self,
        exact: ExactOutcomeRecord,
        failure: OutcomeReceiptSinkError,
    ) -> Result<(), String> {
        let Some(kind) = failure.pending_kind() else {
            return Err("only a typed durable retryable failure may retain exact material".into());
        };
        match self.pending.get_mut(exact.attempt_id()) {
            Some(pending) if pending.exact == exact => {
                pending.failure = failure;
                pending.kind = pending.kind.merge(kind);
                pending.reconciliation_in_flight = false;
                Ok(())
            }
            Some(_) => Err(format!(
                "pending outcome attempt {} changed exact material",
                exact.attempt_id()
            )),
            None => {
                self.pending.insert(
                    exact.attempt_id.clone(),
                    PendingOutcome {
                        exact,
                        failure,
                        kind,
                        reconciliation_in_flight: false,
                    },
                );
                Ok(())
            }
        }
    }

    pub(super) fn begin_reconciliation(
        &mut self,
        attempt_id: &str,
    ) -> Result<Option<(ExactOutcomeRecord, PendingOutcomeKind)>, HeptaError> {
        let Some(pending) = self.pending.get_mut(attempt_id) else {
            return Ok(None);
        };
        if pending.reconciliation_in_flight {
            return Err(HeptaError(format!(
                "outcome reconciliation is already in progress for attempt {attempt_id}"
            )));
        }
        pending.reconciliation_in_flight = true;
        Ok(Some((pending.exact.clone(), pending.kind)))
    }

    pub(super) fn resolve(&mut self, exact: &ExactOutcomeRecord) -> bool {
        if self
            .pending
            .get(exact.attempt_id())
            .is_some_and(|pending| pending.exact == *exact)
        {
            self.pending.remove(exact.attempt_id());
            true
        } else {
            false
        }
    }

    pub(super) fn finish_nonretryable_failure(&mut self, exact: &ExactOutcomeRecord) {
        if let Some(pending) = self.pending.get_mut(exact.attempt_id())
            && pending.exact == *exact
        {
            pending.reconciliation_in_flight = false;
        }
    }

    #[cfg(test)]
    pub(crate) fn ambiguity_count(&self) -> usize {
        self.pending.len()
    }
}
