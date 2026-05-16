use codex_utils_absolute_path::AbsolutePathBuf;
use dirs::home_dir;
use std::path::PathBuf;

/// Returns the path to the Hepta configuration directory, which can be
/// specified by the `HEPTA_HOME` environment variable. If `HEPTA_HOME` is not
/// set, legacy `CODEX_HOME` is accepted as a compatibility fallback. If neither
/// is set, defaults to `~/.hepta`.
///
/// - If `HEPTA_HOME` is set, the value must exist and be a directory. The
///   value will be canonicalized and this function will Err otherwise.
/// - If only `CODEX_HOME` is set, it is treated the same way for compatibility.
/// - If neither variable is set, this function does not verify that the default
///   directory exists.
pub fn find_codex_home() -> std::io::Result<AbsolutePathBuf> {
    let hepta_home_env = non_empty_env("HEPTA_HOME");
    let legacy_codex_home_env = non_empty_env("CODEX_HOME");
    find_codex_home_from_env(hepta_home_env.as_deref(), legacy_codex_home_env.as_deref())
}

fn non_empty_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|val| !val.is_empty())
}

fn find_codex_home_from_env(
    hepta_home_env: Option<&str>,
    legacy_codex_home_env: Option<&str>,
) -> std::io::Result<AbsolutePathBuf> {
    match hepta_home_env
        .map(|val| ("HEPTA_HOME", val))
        .or_else(|| legacy_codex_home_env.map(|val| ("CODEX_HOME", val)))
    {
        Some((env_name, val)) => resolve_home_env(env_name, val),
        None => {
            let mut p = home_dir().ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not find home directory",
                )
            })?;
            p.push(".hepta");
            AbsolutePathBuf::from_absolute_path(p)
        }
    }
}

fn resolve_home_env(env_name: &str, val: &str) -> std::io::Result<AbsolutePathBuf> {
    let path = PathBuf::from(val);
    let metadata = std::fs::metadata(&path).map_err(|err| match err.kind() {
        std::io::ErrorKind::NotFound => std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{env_name} points to {val:?}, but that path does not exist"),
        ),
        _ => std::io::Error::new(
            err.kind(),
            format!("failed to read {env_name} {val:?}: {err}"),
        ),
    })?;

    if !metadata.is_dir() {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{env_name} points to {val:?}, but that path is not a directory"),
        ))
    } else {
        let canonical = path.canonicalize().map_err(|err| {
            std::io::Error::new(
                err.kind(),
                format!("failed to canonicalize {env_name} {val:?}: {err}"),
            )
        })?;
        AbsolutePathBuf::from_absolute_path(canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::find_codex_home_from_env;
    use codex_utils_absolute_path::AbsolutePathBuf;
    use dirs::home_dir;
    use pretty_assertions::assert_eq;
    use std::fs;
    use std::io::ErrorKind;
    use tempfile::TempDir;

    #[test]
    fn find_codex_home_env_missing_path_is_fatal() {
        let temp_home = TempDir::new().expect("temp home");
        let missing = temp_home.path().join("missing-hepta-home");
        let missing_str = missing
            .to_str()
            .expect("missing hepta home path should be valid utf-8");

        let err =
            find_codex_home_from_env(Some(missing_str), None).expect_err("missing HEPTA_HOME");
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert!(
            err.to_string().contains("HEPTA_HOME"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn find_codex_home_env_file_path_is_fatal() {
        let temp_home = TempDir::new().expect("temp home");
        let file_path = temp_home.path().join("hepta-home.txt");
        fs::write(&file_path, "not a directory").expect("write temp file");
        let file_str = file_path
            .to_str()
            .expect("file hepta home path should be valid utf-8");

        let err = find_codex_home_from_env(Some(file_str), None).expect_err("file HEPTA_HOME");
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert!(
            err.to_string().contains("not a directory"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn find_codex_home_env_valid_directory_canonicalizes() {
        let temp_home = TempDir::new().expect("temp home");
        let temp_str = temp_home
            .path()
            .to_str()
            .expect("temp hepta home path should be valid utf-8");

        let resolved = find_codex_home_from_env(Some(temp_str), None).expect("valid HEPTA_HOME");
        let expected = temp_home
            .path()
            .canonicalize()
            .expect("canonicalize temp home");
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute home");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn find_codex_home_without_env_uses_default_home_dir() {
        let resolved = find_codex_home_from_env(
            /*hepta_home_env*/ None, /*legacy_codex_home_env*/ None,
        )
        .expect("default HEPTA_HOME");
        let mut expected = home_dir().expect("home dir");
        expected.push(".hepta");
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute home");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn find_codex_home_uses_legacy_codex_home_when_hepta_home_is_absent() {
        let legacy_home = TempDir::new().expect("legacy home");
        let legacy_str = legacy_home
            .path()
            .to_str()
            .expect("legacy home path should be valid utf-8");

        let resolved =
            find_codex_home_from_env(None, Some(legacy_str)).expect("legacy CODEX_HOME fallback");
        let expected = legacy_home.path().canonicalize().expect("canonicalize");
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute home");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn find_codex_home_prefers_hepta_home_over_legacy_codex_home() {
        let hepta_home = TempDir::new().expect("hepta home");
        let legacy_home = TempDir::new().expect("legacy home");
        let hepta_str = hepta_home
            .path()
            .to_str()
            .expect("hepta home path should be valid utf-8");
        let legacy_str = legacy_home
            .path()
            .to_str()
            .expect("legacy home path should be valid utf-8");

        let resolved = find_codex_home_from_env(Some(hepta_str), Some(legacy_str))
            .expect("HEPTA_HOME should win");
        let expected = hepta_home.path().canonicalize().expect("canonicalize");
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute home");
        assert_eq!(resolved, expected);
    }
}
