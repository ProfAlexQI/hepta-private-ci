use super::*;

impl RuntimeKernel {
    /// Returns a bounded authenticated projection for external rollback anchoring.
    pub fn durable_outcome_monotonic_state(&self) -> Result<DurableMonotonicState, HeptaError> {
        self.outcome_sink.monotonic_state().map_err(|error| {
            HeptaError(format!(
                "failed to read durable outcome monotonic state: {error}"
            ))
        })
    }
}
