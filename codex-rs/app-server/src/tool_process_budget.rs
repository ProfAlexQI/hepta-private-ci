use std::num::NonZeroUsize;
use std::sync::Arc;

use tokio::sync::OwnedSemaphorePermit;
use tokio::sync::Semaphore;

#[derive(Clone, Debug)]
pub(crate) struct ToolProcessBudget {
    limit: Option<NonZeroUsize>,
    capacity: Option<Arc<Semaphore>>,
}

impl ToolProcessBudget {
    pub(crate) fn new(limit: Option<NonZeroUsize>) -> Self {
        Self {
            limit,
            capacity: limit.map(|limit| Arc::new(Semaphore::new(limit.get()))),
        }
    }

    pub(crate) fn limit(&self) -> Option<NonZeroUsize> {
        self.limit
    }

    pub(crate) fn try_reserve(&self) -> Result<Option<OwnedSemaphorePermit>, usize> {
        let Some(limit) = self.limit else {
            return Ok(None);
        };
        let Some(capacity) = self.capacity.as_ref() else {
            return Err(limit.get());
        };
        capacity
            .clone()
            .try_acquire_owned()
            .map(Some)
            .map_err(|_| limit.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cloned_budgets_share_one_process_capacity() {
        let budget = ToolProcessBudget::new(Some(NonZeroUsize::MIN));
        let clone = budget.clone();
        let permit = match budget.try_reserve() {
            Ok(Some(permit)) => permit,
            other => panic!("first process must reserve capacity: {other:?}"),
        };
        assert!(matches!(clone.try_reserve(), Err(1)));
        drop(permit);
        assert!(matches!(clone.try_reserve(), Ok(Some(_))));
    }

    #[test]
    fn unbounded_budget_requires_no_permit() {
        let budget = ToolProcessBudget::new(None);
        assert_eq!(budget.limit(), None);
        assert!(matches!(budget.try_reserve(), Ok(None)));
    }
}
