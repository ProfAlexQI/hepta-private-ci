use std::env;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;

pub(crate) const HEPTA_STATE_ROOT_ENV: &str = "HEPTA_STATE_ROOT";
pub(crate) const HEPTA_STATE_ROOT_SCHEMA: &str = "hepta_runtime_state_root_v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HeptaStateRoot(PathBuf);

impl HeptaStateRoot {
    pub(crate) fn parse(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        if !root.is_absolute()
            || root == Path::new("/")
            || root
                .components()
                .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
        {
            anyhow::bail!(
                "{HEPTA_STATE_ROOT_ENV} ({HEPTA_STATE_ROOT_SCHEMA}) must be a normalized absolute non-root path"
            );
        }
        if root
            .symlink_metadata()
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
        {
            anyhow::bail!("{HEPTA_STATE_ROOT_ENV} must not be a symlink");
        }
        Ok(Self(root))
    }

    pub(crate) fn join(&self, relative: &str) -> Result<PathBuf> {
        let relative = Path::new(relative);
        if relative.is_absolute()
            || relative
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
            anyhow::bail!("state-root child must be a normalized relative path");
        }
        Ok(self.0.join(relative))
    }
}

pub(crate) fn validate_state_root_env() -> Result<()> {
    let Some(raw) = env::var_os(HEPTA_STATE_ROOT_ENV) else {
        return Ok(());
    };
    let root = HeptaStateRoot::parse(PathBuf::from(raw))?;
    root.join("runtime-v2")
        .context("validate typed Hepta runtime state root")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_root_is_absolute_normalized_and_child_bounded() {
        let root = HeptaStateRoot::parse("/tmp/hepta-state").expect("absolute state root");
        assert_eq!(
            root.join("runtime-v2/outcomes.sqlite3")
                .expect("bounded child"),
            Path::new("/tmp/hepta-state/runtime-v2/outcomes.sqlite3")
        );
        assert!(HeptaStateRoot::parse(".hepta").is_err());
        assert!(HeptaStateRoot::parse("/").is_err());
        assert!(root.join("../escape").is_err());
        assert!(root.join("/absolute").is_err());
    }
}
