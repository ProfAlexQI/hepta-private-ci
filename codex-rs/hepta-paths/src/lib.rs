use std::env;
use std::io;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

pub const HEPTA_STATE_ROOT_ENV: &str = "HEPTA_STATE_ROOT";
pub const HEPTA_STATE_ROOT_SCHEMA: &str = "hepta_runtime_state_root_v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaStateRoot(PathBuf);

impl HeptaStateRoot {
    pub fn parse(root: impl Into<PathBuf>) -> io::Result<Self> {
        let root = root.into();
        if !root.is_absolute()
            || root.parent().is_none()
            || root
                .components()
                .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "{HEPTA_STATE_ROOT_ENV} ({HEPTA_STATE_ROOT_SCHEMA}) must be a normalized absolute non-root path"
                ),
            ));
        }
        if root
            .symlink_metadata()
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{HEPTA_STATE_ROOT_ENV} must not be a symlink"),
            ));
        }
        Ok(Self(root))
    }

    pub fn discover() -> io::Result<Self> {
        if let Some(root) = env::var_os(HEPTA_STATE_ROOT_ENV) {
            return Self::parse(root);
        }
        Self::parse(env::current_dir()?.join(".hepta"))
    }

    pub fn join(&self, relative: impl AsRef<Path>) -> io::Result<PathBuf> {
        let relative = relative.as_ref();
        if relative.is_absolute()
            || relative
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "state-root child must be a normalized relative path",
            ));
        }
        Ok(self.0.join(relative))
    }

    pub fn resolve_legacy_default(&self, legacy: &str) -> io::Result<PathBuf> {
        let relative = Path::new(legacy).strip_prefix(".hepta").map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "legacy state path must be rooted at .hepta",
            )
        })?;
        self.join(relative)
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_root_bounds_legacy_and_typed_children() {
        let root_path = env::current_dir().unwrap().join("hepta-state");
        let root = HeptaStateRoot::parse(&root_path).unwrap();
        assert_eq!(
            root.resolve_legacy_default(".hepta/runtime-v2/outcomes.sqlite3")
                .unwrap(),
            root_path.join("runtime-v2/outcomes.sqlite3")
        );
        assert!(HeptaStateRoot::parse(".hepta").is_err());
        assert!(HeptaStateRoot::parse(root_path.ancestors().last().unwrap()).is_err());
        assert!(root.join("../escape").is_err());
        assert!(root.resolve_legacy_default("other/file").is_err());
    }
}
