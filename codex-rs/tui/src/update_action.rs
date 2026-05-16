#[cfg(any(not(debug_assertions), test))]
use codex_install_context::InstallContext;
#[cfg(test)]
use codex_install_context::StandalonePlatform;

const DISABLED_UPDATE_SH_ARGS: &[&str] = &[
    "-c",
    "printf '%s\n' 'Hepta self-update is not configured for this source fork.' >&2; exit 1",
];
const DISABLED_UPDATE_POWERSHELL_ARGS: &[&str] = &[
    "-c",
    "Write-Error 'Hepta self-update is not configured for this source fork.'; exit 1",
];

/// Update action the CLI should perform after the TUI exits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateAction {
    /// Upstream npm self-update is disabled for the Hepta source fork.
    NpmGlobalLatest,
    /// Upstream bun self-update is disabled for the Hepta source fork.
    BunGlobalLatest,
    /// Upstream Homebrew self-update is disabled for the Hepta source fork.
    BrewUpgrade,
    /// Upstream standalone Unix self-update is disabled for the Hepta source fork.
    StandaloneUnix,
    /// Upstream standalone Windows self-update is disabled for the Hepta source fork.
    StandaloneWindows,
}

impl UpdateAction {
    #[cfg(any(not(debug_assertions), test))]
    pub(crate) fn from_install_context(context: &InstallContext) -> Option<Self> {
        let _ = context;
        None
    }

    /// Returns the list of command-line arguments for invoking the update.
    pub fn command_args(self) -> (&'static str, &'static [&'static str]) {
        match self {
            UpdateAction::NpmGlobalLatest
            | UpdateAction::BunGlobalLatest
            | UpdateAction::BrewUpgrade
            | UpdateAction::StandaloneUnix => ("sh", DISABLED_UPDATE_SH_ARGS),
            UpdateAction::StandaloneWindows => ("powershell", DISABLED_UPDATE_POWERSHELL_ARGS),
        }
    }

    /// Returns string representation of the command-line arguments for invoking the update.
    pub fn command_str(self) -> String {
        let (command, args) = self.command_args();
        shlex::try_join(std::iter::once(command).chain(args.iter().copied()))
            .unwrap_or_else(|_| format!("{command} {}", args.join(" ")))
    }
}

#[cfg(not(debug_assertions))]
pub fn get_update_action() -> Option<UpdateAction> {
    UpdateAction::from_install_context(InstallContext::current())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    #[test]
    fn maps_install_context_to_update_action() {
        let native_release_dir = PathBuf::from("/tmp/native-release");

        assert_eq!(
            UpdateAction::from_install_context(&InstallContext::Other),
            None
        );
        assert_eq!(
            UpdateAction::from_install_context(&InstallContext::Npm),
            None
        );
        assert_eq!(
            UpdateAction::from_install_context(&InstallContext::Bun),
            None
        );
        assert_eq!(
            UpdateAction::from_install_context(&InstallContext::Brew),
            None
        );
        assert_eq!(
            UpdateAction::from_install_context(&InstallContext::Standalone {
                platform: StandalonePlatform::Unix,
                release_dir: native_release_dir.clone(),
                resources_dir: Some(native_release_dir.join("codex-resources")),
            }),
            None
        );
        assert_eq!(
            UpdateAction::from_install_context(&InstallContext::Standalone {
                platform: StandalonePlatform::Windows,
                release_dir: native_release_dir.clone(),
                resources_dir: Some(native_release_dir.join("codex-resources")),
            }),
            None
        );
    }

    #[test]
    fn update_commands_are_disabled_for_hepta_fork() {
        assert_eq!(
            UpdateAction::StandaloneUnix.command_args(),
            ("sh", DISABLED_UPDATE_SH_ARGS)
        );
        assert_eq!(
            UpdateAction::StandaloneWindows.command_args(),
            ("powershell", DISABLED_UPDATE_POWERSHELL_ARGS)
        );
        assert!(
            UpdateAction::NpmGlobalLatest
                .command_str()
                .contains("Hepta self-update is not configured for this source fork.")
        );
    }
}
