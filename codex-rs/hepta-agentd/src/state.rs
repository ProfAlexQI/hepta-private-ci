use std::sync::Mutex;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use codex_hepta_automation::AutomationError;
use codex_hepta_automation::AutomationStore;
use codex_hepta_fleet::AgentLifecycle;
use codex_hepta_fleet::FleetRegistry;

use crate::AgentdError;
use crate::AgentdEventKind;
use crate::AgentdIdentity;
use crate::AgentdPayload;
use crate::AgentdResponse;
use crate::EventBuffer;
use crate::HealthSnapshot;
use crate::LifecycleSnapshot;
use crate::SessionIngress;
use crate::SessionTransport;

const AUTOMATION_UNAVAILABLE_CODE: &str = "automation_unavailable";
const AUTOMATION_UNAVAILABLE_MESSAGE: &str =
    "this Agent's private automation storage is unavailable";

pub(crate) struct AgentdState {
    identity: AgentdIdentity,
    registry: FleetRegistry,
    runtime: Mutex<RuntimeState>,
    events: Mutex<EventBuffer>,
    automation: Mutex<Option<AutomationStore>>,
}

struct RuntimeState {
    current_generation: u64,
    lifecycle: AgentLifecycle,
    app_server_ready: bool,
    fenced: bool,
}

impl AgentdState {
    pub(crate) fn new(
        identity: AgentdIdentity,
        registry: FleetRegistry,
        event_capacity: usize,
    ) -> Result<Self, AgentdError> {
        let mut events = EventBuffer::new(event_capacity)?;
        events.push(AgentdEventKind::Bootstrapped);
        events.push(AgentdEventKind::Lifecycle {
            lifecycle: AgentLifecycle::Starting,
            generation: identity.spawn_generation,
        });
        Ok(Self {
            runtime: Mutex::new(RuntimeState {
                current_generation: identity.spawn_generation,
                lifecycle: AgentLifecycle::Starting,
                app_server_ready: false,
                fenced: false,
            }),
            identity,
            registry,
            events: Mutex::new(events),
            automation: Mutex::new(None),
        })
    }

    pub(crate) fn attach_automation_store(
        &self,
        store: AutomationStore,
    ) -> Result<(), AgentdError> {
        if store.owner_agent_id() != &self.identity.agent_id {
            return Err(AgentdError::GenerationFenced(
                "automation store owner does not match agentd identity".to_string(),
            ));
        }
        let mut automation = self.automation.lock().map_err(poisoned_state)?;
        if automation.replace(store).is_some() {
            return Err(AgentdError::Protocol(
                "automation store was attached more than once".to_string(),
            ));
        }
        Ok(())
    }

    pub(crate) fn mark_automation_unavailable(&self) -> Result<(), AgentdError> {
        self.automation.lock().map_err(poisoned_state)?.take();
        Ok(())
    }

    pub(crate) fn automation_is_available(&self) -> Result<bool, AgentdError> {
        Ok(self.automation.lock().map_err(poisoned_state)?.is_some())
    }

    pub(crate) fn identity(&self) -> &AgentdIdentity {
        &self.identity
    }

    pub(crate) fn refresh_generation(&self) -> Result<(), AgentdError> {
        let record = self
            .registry
            .load()?
            .agent(&self.identity.agent_id)
            .cloned()
            .ok_or_else(|| {
                AgentdError::GenerationFenced(format!(
                    "agent {} disappeared from fleet registry",
                    self.identity.agent_id
                ))
            })?;
        if record.layout.home_root() != self.identity.home_root
            || record.layout.run_root() != self.identity.run_root
            || record.layout.cognitive_root() != self.identity.layout.cognitive_root()
            || record.layout.automation_root() != self.identity.layout.automation_root()
            || record.manifest.workspace.as_path() != self.identity.workspace
        {
            return Err(AgentdError::GenerationFenced(
                "registered agent roots changed while agentd was running".to_string(),
            ));
        }
        let distance = record
            .lifecycle
            .generation
            .checked_sub(self.identity.spawn_generation);
        let accepted = matches!(
            (record.lifecycle.lifecycle, distance),
            (AgentLifecycle::Starting, Some(0))
                | (AgentLifecycle::Running, Some(1))
                | (AgentLifecycle::Draining, Some(2))
        );
        if !accepted {
            return Err(AgentdError::GenerationFenced(format!(
                "agent {} spawn generation {} cannot serve {:?} generation {}",
                self.identity.agent_id,
                self.identity.spawn_generation,
                record.lifecycle.lifecycle,
                record.lifecycle.generation
            )));
        }

        let mut runtime = self.runtime.lock().map_err(poisoned_state)?;
        if runtime.current_generation != record.lifecycle.generation
            || runtime.lifecycle != record.lifecycle.lifecycle
        {
            runtime.current_generation = record.lifecycle.generation;
            runtime.lifecycle = record.lifecycle.lifecycle;
            if runtime.lifecycle != AgentLifecycle::Running {
                runtime.app_server_ready = false;
            }
            self.events
                .lock()
                .map_err(poisoned_state)?
                .push(AgentdEventKind::Lifecycle {
                    lifecycle: record.lifecycle.lifecycle,
                    generation: record.lifecycle.generation,
                });
        }
        Ok(())
    }

    pub(crate) fn mark_app_server_ready(&self) -> Result<(), AgentdError> {
        let mut runtime = self.runtime.lock().map_err(poisoned_state)?;
        if !runtime.app_server_ready {
            runtime.app_server_ready = true;
            self.events
                .lock()
                .map_err(poisoned_state)?
                .push(AgentdEventKind::AppServerReady);
        }
        Ok(())
    }

    pub(crate) fn mark_draining(&self) -> Result<(), AgentdError> {
        let mut runtime = self.runtime.lock().map_err(poisoned_state)?;
        runtime.app_server_ready = false;
        self.events
            .lock()
            .map_err(poisoned_state)?
            .push(AgentdEventKind::Draining);
        Ok(())
    }

    pub(crate) fn mark_fenced(&self) {
        if let Ok(mut runtime) = self.runtime.lock() {
            runtime.app_server_ready = false;
            runtime.fenced = true;
        }
        if let Ok(mut events) = self.events.lock() {
            events.push(AgentdEventKind::GenerationFenced);
        }
    }

    pub(crate) fn is_fenced(&self) -> Result<bool, AgentdError> {
        Ok(self.runtime.lock().map_err(poisoned_state)?.fenced)
    }

    pub(crate) fn automation_admission_ready(&self) -> Result<bool, AgentdError> {
        self.refresh_generation()?;
        let runtime = self.runtime.lock().map_err(poisoned_state)?;
        Ok(runtime.lifecycle == AgentLifecycle::Running
            && runtime.app_server_ready
            && !runtime.fenced)
    }

    pub(crate) async fn response(
        &self,
        request_id: u64,
        spawn_generation: u64,
        method: crate::AgentdMethod,
    ) -> Result<AgentdResponse, AgentdError> {
        if spawn_generation != self.identity.spawn_generation {
            return Err(AgentdError::GenerationFenced(format!(
                "request spawn generation {spawn_generation} does not match {}",
                self.identity.spawn_generation
            )));
        }
        self.refresh_generation()?;
        let (current_generation, lifecycle, app_server_ready, fenced) = {
            let runtime = self.runtime.lock().map_err(poisoned_state)?;
            (
                runtime.current_generation,
                runtime.lifecycle,
                runtime.app_server_ready,
                runtime.fenced,
            )
        };
        let automation = self.automation.lock().map_err(poisoned_state)?.clone();
        let payload = match method {
            crate::AgentdMethod::Health => AgentdPayload::Health(HealthSnapshot {
                promotion_ready: matches!(
                    lifecycle,
                    AgentLifecycle::Starting | AgentLifecycle::Running
                ) && app_server_ready
                    && !fenced,
                ready: lifecycle == AgentLifecycle::Running && app_server_ready && !fenced,
                fenced,
                process_id: std::process::id(),
                workspace: self.identity.workspace.clone(),
                home_root: self.identity.home_root.clone(),
                run_root: self.identity.run_root.clone(),
            }),
            crate::AgentdMethod::Lifecycle => AgentdPayload::Lifecycle(LifecycleSnapshot {
                lifecycle,
                app_server_ready,
                fenced,
            }),
            crate::AgentdMethod::SessionIngress => {
                if lifecycle != AgentLifecycle::Running || !app_server_ready || fenced {
                    AgentdPayload::Error {
                        code: "not_ready".to_string(),
                        message: "session ingress is unavailable until this generation is ready"
                            .to_string(),
                    }
                } else {
                    AgentdPayload::SessionIngress(SessionIngress {
                        socket_path: self.identity.app_server_socket.clone(),
                        transport: SessionTransport::CodexAppServerWebsocketOverUds,
                    })
                }
            }
            crate::AgentdMethod::Events {
                after_cursor,
                limit,
            } => {
                if !(1..=crate::MAX_EVENT_BATCH).contains(&limit) {
                    AgentdPayload::Error {
                        code: "invalid_limit".to_string(),
                        message: "event limit must be between 1 and 256".to_string(),
                    }
                } else {
                    let batch = self
                        .events
                        .lock()
                        .map_err(poisoned_state)?
                        .batch(after_cursor, usize::from(limit));
                    AgentdPayload::Events(batch)
                }
            }
            crate::AgentdMethod::AutomationCreate { draft } => {
                require_automation_ready(lifecycle, app_server_ready, fenced)?;
                match automation {
                    Some(store) => self.automation_result(
                        store.create_task(&draft).await,
                        AgentdPayload::AutomationTask,
                    )?,
                    None => automation_unavailable(),
                }
            }
            crate::AgentdMethod::AutomationList { limit } => {
                require_automation_ready(lifecycle, app_server_ready, fenced)?;
                if !(1..=256).contains(&limit) {
                    return Err(AgentdError::Invalid(
                        "automation list limit must be between 1 and 256".to_string(),
                    ));
                }
                match automation {
                    Some(store) => self
                        .automation_result(store.list_tasks(usize::from(limit)).await, |tasks| {
                            AgentdPayload::AutomationTasks { tasks }
                        })?,
                    None => automation_unavailable(),
                }
            }
            crate::AgentdMethod::AutomationCancel { task_id } => {
                require_automation_ready(lifecycle, app_server_ready, fenced)?;
                match automation {
                    Some(store) => self.automation_result(
                        store.cancel_task(task_id, now_ms()?).await,
                        AgentdPayload::AutomationTask,
                    )?,
                    None => automation_unavailable(),
                }
            }
            crate::AgentdMethod::AutomationSetEnabled {
                task_id,
                enabled,
                resume_at_ms,
            } => {
                require_automation_ready(lifecycle, app_server_ready, fenced)?;
                match automation {
                    Some(store) => self.automation_result(
                        store
                            .set_enabled(task_id, enabled, resume_at_ms, now_ms()?)
                            .await,
                        AgentdPayload::AutomationTask,
                    )?,
                    None => automation_unavailable(),
                }
            }
        };
        Ok(AgentdResponse {
            schema_version: crate::AGENTD_CONTROL_SCHEMA_VERSION,
            request_id,
            agent_id: self.identity.agent_id.clone(),
            spawn_generation: self.identity.spawn_generation,
            current_generation,
            payload,
        })
    }

    fn automation_result<T>(
        &self,
        result: Result<T, AutomationError>,
        success: impl FnOnce(T) -> AgentdPayload,
    ) -> Result<AgentdPayload, AgentdError> {
        match result {
            Ok(value) => Ok(success(value)),
            Err(AutomationError::Unavailable | AutomationError::Corrupt) => {
                self.mark_automation_unavailable()?;
                Ok(automation_unavailable())
            }
            Err(AutomationError::AccessDenied) => {
                self.mark_fenced();
                Err(AgentdError::GenerationFenced(
                    "automation store owner or generation boundary was violated".to_string(),
                ))
            }
            Err(error) => Err(error.into()),
        }
    }
}

fn automation_unavailable() -> AgentdPayload {
    AgentdPayload::Error {
        code: AUTOMATION_UNAVAILABLE_CODE.to_string(),
        message: AUTOMATION_UNAVAILABLE_MESSAGE.to_string(),
    }
}

fn require_automation_ready(
    lifecycle: AgentLifecycle,
    app_server_ready: bool,
    fenced: bool,
) -> Result<(), AgentdError> {
    if lifecycle == AgentLifecycle::Running && app_server_ready && !fenced {
        Ok(())
    } else {
        Err(AgentdError::Protocol(
            "automation control is unavailable until this Agent generation is ready".to_string(),
        ))
    }
}

fn now_ms() -> Result<u64, AgentdError> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AgentdError::Protocol("system clock precedes Unix epoch".to_string()))?
        .as_millis();
    u64::try_from(millis)
        .map_err(|_| AgentdError::Protocol("system clock exceeds u64 milliseconds".to_string()))
}

fn poisoned_state<T>(_error: std::sync::PoisonError<T>) -> AgentdError {
    AgentdError::Protocol("agentd control state mutex is poisoned".to_string())
}
