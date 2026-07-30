use std::env;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;

pub(crate) use hepta_paths::HEPTA_STATE_ROOT_ENV;
pub(crate) use hepta_paths::HeptaStateRoot;

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
            std::path::Path::new("/tmp/hepta-state/runtime-v2/outcomes.sqlite3")
        );
        assert!(HeptaStateRoot::parse(".hepta").is_err());
        assert!(HeptaStateRoot::parse("/").is_err());
        assert!(root.join("../escape").is_err());
        assert!(root.join("/absolute").is_err());
    }
}
