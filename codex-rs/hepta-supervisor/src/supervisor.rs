use std::collections::BTreeMap;
use std::time::Instant;

use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::AgentLifecycle;
use codex_hepta_fleet::AgentRecord;
use codex_hepta_fleet::FleetRegistry;

use crate::AgentCommand;
use crate::AgentFault;
use crate::AgentRelease;
use crate::AgentSupervisorSnapshot;
use crate::ProcessDriver;
use crate::SupervisorConfig;
use crate::SupervisorError;
use crate::SupervisorEventKind;
use crate::TickReport;
use crate::runtime::AgentSlot;
use crate::runtime::bounded_message;

/// Lifecycle-only controller with one process handle and bounded buffers per agent.
pub struct Supervisor<D: ProcessDriver> {
    pub(crate) registry: FleetRegistry,
    pub(crate) driver: D,
    pub(crate) config: SupervisorConfig,
    slots: BTreeMap<AgentId, AgentSlot<D::Process>>,
}

#[cfg(test)]
#[path = "supervisor_tests.rs"]
mod tests;

impl<D: ProcessDriver> Supervisor<D> {
    pub fn recover(
        registry: FleetRegistry,
        driver: D,
        config: SupervisorConfig,
        now: Instant,
    ) -> Result<(Self, TickReport), SupervisorError> {
        config.validate()?;
        let snapshot = registry.load()?;
        let slots = snapshot
            .agents
            .keys()
            .cloned()
            .map(|agent_id| (agent_id, AgentSlot::new(&config)))
            .collect();
        let mut supervisor = Self {
            registry,
            driver,
            config,
            slots,
        };
        let mut report = TickReport::default();
        for (agent_id, record) in snapshot.agents {
            let result = supervisor.with_slot(&agent_id, |supervisor, slot| {
                supervisor.recover_slot(&agent_id, slot, &record, now)
            });
            if let Err(error) = result {
                supervisor.record_fault(&agent_id, &error, &mut report);
            }
        }
        Ok((supervisor, report))
    }

    pub fn snapshot(&self, agent_id: &AgentId) -> Option<AgentSupervisorSnapshot> {
        self.slots
            .get(agent_id)
            .map(|slot| AgentSupervisorSnapshot {
                active: slot.runtime.is_some(),
                runtime_generation: slot.runtime.as_ref().map(|runtime| runtime.generation),
                spawn_generation: slot
                    .runtime
                    .as_ref()
                    .map(|runtime| runtime.spawn_generation),
                process_system_id: slot
                    .runtime
                    .as_ref()
                    .map(|runtime| runtime.identity.system_id()),
                active_release: slot
                    .active_release
                    .as_ref()
                    .map(|release| release.identity().to_string()),
                previous_release: slot
                    .previous_release
                    .as_ref()
                    .map(|release| release.identity().to_string()),
                release_change_pending: slot.release_change.is_some(),
                events: slot.events.items.iter().cloned().collect(),
                logs: slot.logs.items.iter().cloned().collect(),
            })
    }

    pub fn start(
        &mut self,
        agent_id: &AgentId,
        command: AgentCommand,
        now: Instant,
    ) -> Result<(), SupervisorError> {
        self.with_slot(agent_id, |supervisor, slot| {
            supervisor.start_slot(agent_id, slot, command, now)
        })
    }

    pub fn start_release(
        &mut self,
        agent_id: &AgentId,
        release: AgentRelease,
        now: Instant,
    ) -> Result<(), SupervisorError> {
        self.with_slot(agent_id, |supervisor, slot| {
            supervisor.start_release_slot(agent_id, slot, release, now)
        })
    }

    pub fn drain(&mut self, agent_id: &AgentId, now: Instant) -> Result<(), SupervisorError> {
        self.with_slot(agent_id, |supervisor, slot| {
            supervisor.drain_slot(agent_id, slot, now)
        })
    }

    pub fn stop(&mut self, agent_id: &AgentId, now: Instant) -> Result<(), SupervisorError> {
        self.with_slot(agent_id, |supervisor, slot| {
            supervisor.stop_slot(agent_id, slot, now)
        })
    }

    pub fn kill(&mut self, agent_id: &AgentId) -> Result<(), SupervisorError> {
        self.with_slot(agent_id, |supervisor, slot| {
            supervisor.kill_slot(agent_id, slot)
        })
    }

    pub fn restart(&mut self, agent_id: &AgentId, now: Instant) -> Result<(), SupervisorError> {
        self.with_slot(agent_id, |supervisor, slot| {
            supervisor.restart_slot(agent_id, slot, now)
        })
    }

    pub fn upgrade(
        &mut self,
        agent_id: &AgentId,
        target: AgentRelease,
        now: Instant,
    ) -> Result<(), SupervisorError> {
        self.with_slot(agent_id, |supervisor, slot| {
            supervisor.upgrade_slot(agent_id, slot, target, now, false)
        })
    }

    pub fn rollback(&mut self, agent_id: &AgentId, now: Instant) -> Result<(), SupervisorError> {
        self.with_slot(agent_id, |supervisor, slot| {
            let target = slot
                .previous_release
                .clone()
                .ok_or_else(|| SupervisorError::NoPreviousRelease(agent_id.clone()))?;
            supervisor.upgrade_slot(agent_id, slot, target, now, true)
        })
    }

    pub fn tick(&mut self, now: Instant) -> TickReport {
        let mut report = TickReport::default();
        let agent_ids: Vec<_> = self.slots.keys().cloned().collect();
        for agent_id in agent_ids {
            let result = self.with_slot(&agent_id, |supervisor, slot| {
                supervisor.tick_slot(&agent_id, slot, now)
            });
            if let Err(error) = result {
                self.record_fault(&agent_id, &error, &mut report);
            }
        }
        report
    }

    fn with_slot<R>(
        &mut self,
        agent_id: &AgentId,
        operation: impl FnOnce(&mut Self, &mut AgentSlot<D::Process>) -> Result<R, SupervisorError>,
    ) -> Result<R, SupervisorError> {
        let mut slot = self
            .slots
            .remove(agent_id)
            .ok_or_else(|| SupervisorError::UnknownAgent(agent_id.clone()))?;
        let result = operation(self, &mut slot);
        self.slots.insert(agent_id.clone(), slot);
        result
    }

    pub(crate) fn transition_without_runtime(
        &self,
        agent_id: &AgentId,
        slot: &mut AgentSlot<D::Process>,
        expected: u64,
        lifecycle: AgentLifecycle,
    ) -> Result<u64, SupervisorError> {
        let next = self
            .registry
            .compare_and_transition(agent_id, expected, lifecycle)?;
        slot.event(next.generation, SupervisorEventKind::Lifecycle(lifecycle));
        Ok(next.generation)
    }

    pub(crate) fn record(&self, agent_id: &AgentId) -> Result<AgentRecord, SupervisorError> {
        self.registry
            .load()?
            .agent(agent_id)
            .cloned()
            .ok_or_else(|| SupervisorError::UnknownAgent(agent_id.clone()))
    }

    fn record_fault(
        &mut self,
        agent_id: &AgentId,
        error: &SupervisorError,
        report: &mut TickReport,
    ) {
        let message = bounded_message(error.to_string());
        if let Some(slot) = self.slots.get_mut(agent_id) {
            let generation = slot
                .runtime
                .as_ref()
                .map(|runtime| runtime.generation)
                .unwrap_or(0);
            slot.event(
                generation,
                SupervisorEventKind::DriverFault(message.clone()),
            );
        }
        report.faults.push(AgentFault {
            agent_id: agent_id.clone(),
            message,
        });
    }
}
