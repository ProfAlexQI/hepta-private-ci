use std::fmt;

use codex_hepta_contracts::AuthorityGrant;
use codex_hepta_contracts::RuntimeAuthorityContext;
use codex_hepta_contracts::Sha256Digest;

use crate::AgentLifecycle;
use crate::AgentRecord;
use crate::ReleaseId;

pub const RUNTIME_LAUNCH_BINDING_SCHEMA_VERSION: u32 = 1;

/// Exact launch-time identity reconstructed independently by the Supervisor
/// and Agentd from fleet-owned state plus the closed local authority grant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeLaunchBinding {
    schema_version: u32,
    release_id: ReleaseId,
    runtime_authority: RuntimeAuthorityContext,
    binding_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeLaunchBindingError {
    LifecycleNotStarting,
    AgentMismatch,
    GenerationMismatch,
    ReleaseEpochOverflow,
    Authority(String),
}

impl fmt::Display for RuntimeLaunchBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LifecycleNotStarting => {
                formatter.write_str("runtime launch requires the Starting lifecycle")
            }
            Self::AgentMismatch => {
                formatter.write_str("runtime launch Agent identity does not match")
            }
            Self::GenerationMismatch => {
                formatter.write_str("runtime launch generation does not match")
            }
            Self::ReleaseEpochOverflow => {
                formatter.write_str("runtime launch release authority epoch overflowed")
            }
            Self::Authority(reason) => {
                write!(formatter, "runtime launch authority failed: {reason}")
            }
        }
    }
}

impl std::error::Error for RuntimeLaunchBindingError {}

impl RuntimeLaunchBinding {
    pub fn for_starting(
        record: &AgentRecord,
        release_id: ReleaseId,
        authority: &AuthorityGrant,
    ) -> Result<Self, RuntimeLaunchBindingError> {
        if record.lifecycle.lifecycle != AgentLifecycle::Starting {
            return Err(RuntimeLaunchBindingError::LifecycleNotStarting);
        }
        if record.lifecycle.agent_id != record.manifest.agent_id
            || authority.subject_agent_id() != &record.manifest.agent_id
        {
            return Err(RuntimeLaunchBindingError::AgentMismatch);
        }
        if authority.generation() != record.lifecycle.generation || record.lifecycle.generation == 0
        {
            return Err(RuntimeLaunchBindingError::GenerationMismatch);
        }
        authority
            .validate_binding(&record.manifest.agent_id, record.lifecycle.generation)
            .map_err(|error| RuntimeLaunchBindingError::Authority(error.to_string()))?;

        let authority_epoch = record
            .release_state
            .generation
            .checked_add(1)
            .ok_or(RuntimeLaunchBindingError::ReleaseEpochOverflow)?;
        let owner_epoch = record.lifecycle.generation;
        let fencing_token_sha256 = runtime_fencing_token(record, &release_id, authority);
        let runtime_authority = RuntimeAuthorityContext::new(
            record.manifest.agent_id.clone(),
            authority_epoch,
            owner_epoch,
            record.lifecycle.generation,
            fencing_token_sha256,
            authority.digest(),
        )
        .and_then(|context| {
            context.validate_grant(authority)?;
            Ok(context)
        })
        .map_err(|error| RuntimeLaunchBindingError::Authority(error.to_string()))?;

        let mut bytes = Vec::new();
        frame(&mut bytes, b"hepta:runtime-launch-binding:v1");
        frame(
            &mut bytes,
            &RUNTIME_LAUNCH_BINDING_SCHEMA_VERSION.to_be_bytes(),
        );
        frame(&mut bytes, release_id.as_str().as_bytes());
        frame(&mut bytes, runtime_authority.digest().as_str().as_bytes());
        let binding_sha256 = Sha256Digest::for_bytes(&bytes);

        Ok(Self {
            schema_version: RUNTIME_LAUNCH_BINDING_SCHEMA_VERSION,
            release_id,
            runtime_authority,
            binding_sha256,
        })
    }

    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn release_id(&self) -> &ReleaseId {
        &self.release_id
    }

    pub fn runtime_authority(&self) -> &RuntimeAuthorityContext {
        &self.runtime_authority
    }

    pub fn digest(&self) -> &Sha256Digest {
        &self.binding_sha256
    }
}

fn runtime_fencing_token(
    record: &AgentRecord,
    release_id: &ReleaseId,
    authority: &AuthorityGrant,
) -> Sha256Digest {
    let mut bytes = Vec::new();
    frame(&mut bytes, b"hepta:agent-runtime-fence:v2");
    frame(&mut bytes, record.manifest.agent_id.as_str().as_bytes());
    frame(&mut bytes, release_id.as_str().as_bytes());
    frame(&mut bytes, &record.lifecycle.schema_version.to_be_bytes());
    frame(&mut bytes, &record.lifecycle.generation.to_be_bytes());
    frame(
        &mut bytes,
        lifecycle_name(record.lifecycle.lifecycle).as_bytes(),
    );
    frame(
        &mut bytes,
        &record.release_state.schema_version.to_be_bytes(),
    );
    frame(&mut bytes, &record.release_state.generation.to_be_bytes());
    frame(
        &mut bytes,
        record
            .release_state
            .current
            .as_ref()
            .map(ReleaseId::as_str)
            .unwrap_or("")
            .as_bytes(),
    );
    frame(
        &mut bytes,
        record
            .release_state
            .previous
            .as_ref()
            .map(ReleaseId::as_str)
            .unwrap_or("")
            .as_bytes(),
    );
    frame(&mut bytes, authority.digest().as_str().as_bytes());
    frame(
        &mut bytes,
        &record.manifest.resources.max_concurrent_turns.to_be_bytes(),
    );
    frame(
        &mut bytes,
        &record.manifest.resources.memory_limit_mib.to_be_bytes(),
    );
    frame(
        &mut bytes,
        &record.manifest.resources.max_tool_processes.to_be_bytes(),
    );
    frame(
        &mut bytes,
        &record.manifest.resources.turn_queue_capacity.to_be_bytes(),
    );
    Sha256Digest::for_bytes(&bytes)
}

const fn lifecycle_name(lifecycle: AgentLifecycle) -> &'static str {
    match lifecycle {
        AgentLifecycle::Stopped => "stopped",
        AgentLifecycle::Starting => "starting",
        AgentLifecycle::Running => "running",
        AgentLifecycle::Draining => "draining",
        AgentLifecycle::Failed => "failed",
    }
}

fn frame(target: &mut Vec<u8>, part: &[u8]) {
    target.extend_from_slice(&(part.len() as u64).to_be_bytes());
    target.extend_from_slice(part);
}

#[cfg(test)]
#[path = "runtime_launch_tests.rs"]
mod tests;
