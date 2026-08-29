use std::path::Path;
use std::path::PathBuf;

use codex_hepta_contracts::AgentId;

use crate::FleetRegistry;
use crate::FleetRegistryError;
use crate::ReleaseId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllowedRuntimeRelease {
    pub release_id: ReleaseId,
    pub program: PathBuf,
}

/// Resolves an executable to at most one allowed fleet release without
/// interpreting the presence or absence of runtime provenance.
pub fn allowed_runtime_release_for_program(
    registry: &FleetRegistry,
    agent_id: &AgentId,
    program: &Path,
) -> Result<Option<AllowedRuntimeRelease>, FleetRegistryError> {
    let actual = program.canonicalize().map_err(|error| {
        FleetRegistryError::Invalid(format!(
            "runtime executable cannot be canonicalized: {error}"
        ))
    })?;
    let mut resolved = None;
    for release_id in registry.allowed_releases(agent_id)? {
        let release = registry.resolve_release(agent_id, &release_id)?;
        let candidate = release.program.canonicalize()?;
        if candidate != actual {
            continue;
        }
        if resolved.is_some() {
            return Err(FleetRegistryError::Corrupt(
                "runtime executable resolves to multiple allowed releases".to_string(),
            ));
        }
        resolved = Some(AllowedRuntimeRelease {
            release_id,
            program: candidate,
        });
    }
    Ok(resolved)
}

#[cfg(test)]
#[path = "runtime_release_lookup_tests.rs"]
mod tests;
