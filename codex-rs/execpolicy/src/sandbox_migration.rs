use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;

const MIGRATION_MARKER_FILENAME: &str = ".sandbox_migration";
const MIGRATION_MARKER_CONTENTS: &[u8] = b"v1\n";

/// Removes exact legacy allow rules that newer Codex versions no longer offer.
///
/// The migration is intentionally one-shot. Once its versioned marker exists,
/// rules created by a newer version are preserved on subsequent startups.
/// This function performs blocking I/O and should be called from
/// `tokio::task::spawn_blocking` in async contexts.
pub fn prefix_rule_migration(
    codex_home: &Path,
    policy_path: &Path,
    banned_prefixes: &[&[&str]],
) -> io::Result<()> {
    let marker_path = codex_home.join(MIGRATION_MARKER_FILENAME);
    if migration_marker_is_valid(&marker_path)? {
        return Ok(());
    }

    clean_rules_file(policy_path, banned_prefixes)?;
    write_migration_marker(codex_home, &marker_path)
}

fn migration_marker_is_valid(marker_path: &Path) -> io::Result<bool> {
    let metadata = match fs::symlink_metadata(marker_path) {
        Ok(metadata) => metadata,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(err),
    };
    if !metadata.file_type().is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "exec policy migration marker is not a regular file: {}",
                marker_path.display()
            ),
        ));
    }

    let contents = fs::read(marker_path)?;
    if contents != MIGRATION_MARKER_CONTENTS {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "unsupported exec policy migration marker contents at {}",
                marker_path.display()
            ),
        ));
    }
    Ok(true)
}

fn write_migration_marker(codex_home: &Path, marker_path: &Path) -> io::Result<()> {
    fs::create_dir_all(codex_home)?;
    let mut marker = tempfile::NamedTempFile::new_in(codex_home)?;
    marker.write_all(MIGRATION_MARKER_CONTENTS)?;
    marker.flush()?;
    marker.as_file().sync_all()?;
    match marker.persist_noclobber(marker_path) {
        Ok(_) => Ok(()),
        Err(err) if err.error.kind() == io::ErrorKind::AlreadyExists => {
            // A concurrent startup may have completed the same migration. Only
            // accept its marker when it is the exact version we understand.
            migration_marker_is_valid(marker_path).and_then(|valid| {
                if valid {
                    Ok(())
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "exec policy migration marker disappeared",
                    ))
                }
            })
        }
        Err(err) => Err(err.error),
    }
}

fn clean_rules_file(policy_path: &Path, banned_prefixes: &[&[&str]]) -> io::Result<()> {
    let Some(write_path) = resolve_policy_write_path(policy_path)? else {
        return Ok(());
    };
    let contents = fs::read_to_string(&write_path)?;
    let retained = strip_banned_allow_rules(&contents, banned_prefixes);
    if retained == contents {
        return Ok(());
    }

    write_policy_atomically(&write_path, retained.as_bytes())
}

fn resolve_policy_write_path(policy_path: &Path) -> io::Result<Option<PathBuf>> {
    let metadata = match fs::symlink_metadata(policy_path) {
        Ok(metadata) => metadata,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err),
    };
    if metadata.file_type().is_symlink() {
        return fs::canonicalize(policy_path).map(Some);
    }
    if !metadata.file_type().is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "exec policy path is not a regular file: {}",
                policy_path.display()
            ),
        ));
    }
    Ok(Some(policy_path.to_path_buf()))
}

fn write_policy_atomically(policy_path: &Path, contents: &[u8]) -> io::Result<()> {
    let parent = policy_path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("exec policy path has no parent: {}", policy_path.display()),
        )
    })?;
    let permissions = fs::metadata(policy_path)?.permissions();
    let mut replacement = tempfile::NamedTempFile::new_in(parent)?;
    replacement.as_file().set_permissions(permissions)?;
    replacement.write_all(contents)?;
    replacement.flush()?;
    replacement.as_file().sync_all()?;
    replacement
        .persist(policy_path)
        .map(|_| ())
        .map_err(|err| err.error)
}

fn strip_banned_allow_rules(contents: &str, banned_prefixes: &[&[&str]]) -> String {
    let banned_prefixes = banned_prefixes
        .iter()
        .map(|prefix| {
            prefix
                .iter()
                .map(|token| token.to_ascii_lowercase())
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();
    contents
        .split_inclusive('\n')
        .filter(|line| !should_remove_rule(line, &banned_prefixes))
        .collect()
}

fn should_remove_rule(line: &str, banned_prefixes: &HashSet<Vec<String>>) -> bool {
    let line = line.strip_suffix('\n').unwrap_or(line);
    let line = line.strip_suffix('\r').unwrap_or(line);
    let Some(pattern) = line
        .strip_prefix("prefix_rule(pattern=")
        .and_then(|line| line.strip_suffix(r#", decision="allow")"#))
    else {
        return false;
    };
    let Ok(prefix) = serde_json::from_str::<Vec<String>>(pattern) else {
        return false;
    };
    let prefix = prefix
        .iter()
        .map(|token| token.to_ascii_lowercase())
        .collect::<Vec<_>>();
    banned_prefixes.contains(&prefix)
}

#[cfg(test)]
#[path = "sandbox_migration_tests.rs"]
mod tests;
