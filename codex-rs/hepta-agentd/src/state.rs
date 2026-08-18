use std::sync::Mutex;

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

pub(crate) struct AgentdState {
    identity: AgentdIdentity,
    registry: FleetRegistry,
    runtime: Mutex<RuntimeState>,
    events: Mutex<EventBuffer>,
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
        })
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

    pub(crate) fn response(
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
        let runtime = self.runtime.lock().map_err(poisoned_state)?;
        let current_generation = runtime.current_generation;
        let payload = match method {
            crate::AgentdMethod::Health => AgentdPayload::Health(HealthSnapshot {
                promotion_ready: matches!(
                    runtime.lifecycle,
                    AgentLifecycle::Starting | AgentLifecycle::Running
                ) && runtime.app_server_ready
                    && !runtime.fenced,
                ready: runtime.lifecycle == AgentLifecycle::Running
                    && runtime.app_server_ready
                    && !runtime.fenced,
                fenced: runtime.fenced,
                process_id: std::process::id(),
                workspace: self.identity.workspace.clone(),
                home_root: self.identity.home_root.clone(),
                run_root: self.identity.run_root.clone(),
            }),
            crate::AgentdMethod::Lifecycle => AgentdPayload::Lifecycle(LifecycleSnapshot {
                lifecycle: runtime.lifecycle,
                app_server_ready: runtime.app_server_ready,
                fenced: runtime.fenced,
            }),
            crate::AgentdMethod::SessionIngress => {
                if runtime.lifecycle != AgentLifecycle::Running
                    || !runtime.app_server_ready
                    || runtime.fenced
                {
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
}

fn poisoned_state<T>(_error: std::sync::PoisonError<T>) -> AgentdError {
    AgentdError::Protocol("agentd control state mutex is poisoned".to_string())
}
