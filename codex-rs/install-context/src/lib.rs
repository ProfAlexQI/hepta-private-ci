use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;

const RELEASES_DIRNAME: &str = "releases";
const RESOURCES_DIRNAME: &str = "hepta-resources";
const STANDALONE_PACKAGES_DIRNAME: &str = "standalone";
static INSTALL_CONTEXT: OnceLock<InstallContext> = OnceLock::new();

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StandalonePlatform {
    Unix,
    Windows,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InstallContext {
    Standalone {
        /// The managed standalone release directory, for example
        /// `~/.hepta/packages/standalone/releases/0.111.0-x86_64-unknown-linux-musl`.
        release_dir: PathBuf,
        /// The bundled resource directory that sits next to the executable when
        /// this install ships managed dependencies.
        resources_dir: Option<PathBuf>,
        /// The platform of the standalone release, either `Unix` or `Windows`.
        platform: StandalonePlatform,
    },
    /// A Hepta binary launched through the npm-managed `hepta.js` shim.
    Npm,
    /// A Hepta binary launched through the bun-managed `hepta.js` shim.
    Bun,
    /// A Hepta binary that appears to come from a Homebrew install prefix.
    Brew,
    /// Any other execution environment.
    ///
    /// This commonly covers `cargo run`, app-bundled Hepta binaries, custom
    /// internal launchers, and tests that execute Hepta from an arbitrary path.
    Other,
}

impl InstallContext {
    pub fn from_exe(
        is_macos: bool,
        current_exe: Option<&Path>,
        managed_by_npm: bool,
        managed_by_bun: bool,
    ) -> Self {
        let hepta_home = codex_utils_home_dir::find_codex_home().ok();
        Self::from_exe_with_hepta_home(
            is_macos,
            current_exe,
            managed_by_npm,
            managed_by_bun,
            hepta_home.as_deref(),
        )
    }

    fn from_exe_with_hepta_home(
        is_macos: bool,
        current_exe: Option<&Path>,
        managed_by_npm: bool,
        managed_by_bun: bool,
        hepta_home: Option<&Path>,
    ) -> Self {
        if managed_by_npm {
            return Self::Npm;
        }

        if managed_by_bun {
            return Self::Bun;
        }

        if let Some(exe_path) = current_exe
            && let Some(standalone_context) = standalone_install_context(exe_path, hepta_home)
        {
            return standalone_context;
        }

        if is_macos
            && let Some(exe_path) = current_exe
            && (exe_path.starts_with("/opt/homebrew") || exe_path.starts_with("/usr/local"))
        {
            return Self::Brew;
        }

        Self::Other
    }

    pub fn current() -> &'static Self {
        INSTALL_CONTEXT.get_or_init(|| {
            let current_exe = std::env::current_exe().ok();
            let managed_by_npm =
                managed_env_present("HEPTA_MANAGED_BY_NPM", "CODEX_MANAGED_BY_NPM");
            let managed_by_bun =
                managed_env_present("HEPTA_MANAGED_BY_BUN", "CODEX_MANAGED_BY_BUN");
            Self::from_exe(
                cfg!(target_os = "macos"),
                current_exe.as_deref(),
                managed_by_npm,
                managed_by_bun,
            )
        })
    }

    pub fn rg_command(&self) -> PathBuf {
        match self {
            Self::Standalone {
                resources_dir: Some(resources_dir),
                ..
            } => {
                let bundled_rg = resources_dir.join(default_rg_command());
                if bundled_rg.exists() {
                    bundled_rg
                } else {
                    default_rg_command()
                }
            }
            Self::Standalone {
                resources_dir: None,
                ..
            }
            | Self::Npm
            | Self::Bun
            | Self::Brew
            | Self::Other => default_rg_command(),
        }
    }
}

fn standalone_install_context(
    exe_path: &Path,
    hepta_home: Option<&Path>,
) -> Option<InstallContext> {
    let canonical_exe = std::fs::canonicalize(exe_path).ok()?;
    let canonical_hepta_home = std::fs::canonicalize(hepta_home?).ok()?;
    let release_dir = canonical_exe.parent()?.to_path_buf();
    let releases_root = canonical_hepta_home
        .join("packages")
        .join(STANDALONE_PACKAGES_DIRNAME)
        .join(RELEASES_DIRNAME);
    if !release_dir.starts_with(releases_root) {
        return None;
    }

    let resources_dir = release_dir.join(RESOURCES_DIRNAME);
    Some(InstallContext::Standalone {
        release_dir,
        resources_dir: resources_dir.is_dir().then_some(resources_dir),
        platform: standalone_platform(),
    })
}

fn standalone_platform() -> StandalonePlatform {
    if cfg!(windows) {
        StandalonePlatform::Windows
    } else {
        StandalonePlatform::Unix
    }
}

fn default_rg_command() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from("rg.exe")
    } else {
        PathBuf::from("rg")
    }
}

fn managed_env_present(primary: &str, legacy: &str) -> bool {
    std::env::var_os(primary).is_some() || std::env::var_os(legacy).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;

    #[test]
    fn detects_standalone_install_from_release_layout() -> std::io::Result<()> {
        let hepta_home = tempfile::tempdir()?;
        let release_dir = hepta_home
            .path()
            .join("packages/standalone/releases/1.2.3-x86_64-unknown-linux-musl");
        let resources_dir = release_dir.join(RESOURCES_DIRNAME);
        fs::create_dir_all(&resources_dir)?;
        let exe_path = release_dir.join(if cfg!(windows) { "hepta.exe" } else { "hepta" });
        fs::write(&exe_path, "")?;
        fs::write(resources_dir.join(default_rg_command()), "")?;
        let canonical_release_dir = release_dir.canonicalize()?;
        let canonical_resources_dir = resources_dir.canonicalize()?;

        let context = InstallContext::from_exe_with_hepta_home(
            /*is_macos*/ false,
            /*current_exe*/ Some(&exe_path),
            /*managed_by_npm*/ false,
            /*managed_by_bun*/ false,
            /*hepta_home*/ Some(hepta_home.path()),
        );
        assert_eq!(
            context,
            InstallContext::Standalone {
                release_dir: canonical_release_dir,
                resources_dir: Some(canonical_resources_dir),
                platform: standalone_platform(),
            }
        );
        Ok(())
    }

    #[test]
    fn standalone_rg_falls_back_when_resources_are_missing() -> std::io::Result<()> {
        let hepta_home = tempfile::tempdir()?;
        let release_dir = hepta_home
            .path()
            .join("packages/standalone/releases/1.2.3-x86_64-unknown-linux-musl");
        fs::create_dir_all(&release_dir)?;
        let exe_path = release_dir.join(if cfg!(windows) { "hepta.exe" } else { "hepta" });
        fs::write(&exe_path, "")?;

        let context = InstallContext::from_exe_with_hepta_home(
            /*is_macos*/ false,
            /*current_exe*/ Some(&exe_path),
            /*managed_by_npm*/ false,
            /*managed_by_bun*/ false,
            /*hepta_home*/ Some(hepta_home.path()),
        );
        assert_eq!(context.rg_command(), default_rg_command());
        Ok(())
    }

    #[test]
    fn npm_and_bun_take_precedence() {
        let npm_context = InstallContext::from_exe_with_hepta_home(
            /*is_macos*/ false,
            /*current_exe*/ Some(Path::new("/tmp/hepta")),
            /*managed_by_npm*/ true,
            /*managed_by_bun*/ false,
            /*hepta_home*/ None,
        );
        assert_eq!(npm_context, InstallContext::Npm);

        let bun_context = InstallContext::from_exe_with_hepta_home(
            /*is_macos*/ false,
            /*current_exe*/ Some(Path::new("/tmp/hepta")),
            /*managed_by_npm*/ false,
            /*managed_by_bun*/ true,
            /*hepta_home*/ None,
        );
        assert_eq!(bun_context, InstallContext::Bun);
    }

    #[test]
    fn brew_is_detected_on_macos_prefixes() {
        let context = InstallContext::from_exe_with_hepta_home(
            /*is_macos*/ true,
            /*current_exe*/ Some(Path::new("/opt/homebrew/bin/hepta")),
            /*managed_by_npm*/ false,
            /*managed_by_bun*/ false,
            /*hepta_home*/ None,
        );
        assert_eq!(context, InstallContext::Brew);
    }
}
