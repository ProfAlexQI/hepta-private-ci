use std::sync::Arc;

use codex_hepta_automation::AutomationError;
use codex_hepta_automation::AutomationOperationContext;
use codex_hepta_automation::AutomationStore;
use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::Authorized;
use codex_hepta_contracts::AutomationMutationCapability;
use tokio_util::sync::CancellationToken;

use crate::AgentdError;
use crate::AgentdIdentity;
use crate::AgentdState;
use crate::automation::run_automation_scheduler;

pub(crate) struct AgentAutomationService {
    store: Option<AutomationStore>,
    operation_context: AutomationOperationContext,
    _mutation: Authorized<AutomationMutationCapability>,
}

impl AgentAutomationService {
    pub(crate) async fn open(
        state: &AgentdState,
        identity: &AgentdIdentity,
        authority: &AuthorityGrant,
    ) -> Result<Self, AgentdError> {
        authority
            .validate_binding(&identity.agent_id, identity.spawn_generation)
            .map_err(|error| {
                AgentdError::Protocol(format!("validate Automation authority binding: {error}"))
            })?;
        let mutation = authority
            .authorize::<AutomationMutationCapability>()
            .map_err(|error| {
                AgentdError::Protocol(format!("authorize Automation service: {error}"))
            })?;
        let operation_context = AutomationOperationContext::new(
            u64::from(authority.schema_version()),
            identity.spawn_generation,
            identity.spawn_generation,
            authority.digest(),
        )
        .map_err(|error| {
            AgentdError::Protocol(format!("bind Automation operation fence: {error}"))
        })?;

        state.refresh_generation()?;
        let opened = AutomationStore::open(&identity.layout).await;
        state.refresh_generation()?;
        let store = match opened {
            Ok(store) => Some(store),
            Err(AutomationError::Unavailable | AutomationError::Corrupt) => None,
            Err(error) => return Err(error.into()),
        };
        if let Some(store) = store.as_ref() {
            state.attach_automation_store(store.clone())?;
        }

        Ok(Self {
            store,
            operation_context,
            _mutation: mutation,
        })
    }

    #[cfg(test)]
    pub(crate) fn is_available(&self) -> bool {
        self.store.is_some()
    }

    pub(crate) async fn run(
        self,
        state: Arc<AgentdState>,
        identity: AgentdIdentity,
        cancellation: CancellationToken,
    ) -> Result<(), AgentdError> {
        match self.store {
            Some(store) => {
                run_automation_scheduler(
                    store,
                    self.operation_context,
                    state,
                    identity,
                    cancellation,
                )
                .await
            }
            None => {
                cancellation.cancelled().await;
                Ok(())
            }
        }
    }
}
