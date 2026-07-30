include!("preference_cas/fixtures.rs");
include!("preference_cas/legacy.rs");
include!("preference_cas/document.rs");

mod durable;
mod durable_concurrency;
mod durable_opening;
mod durable_opening_security;

fn private_tempdir() -> Result<tempfile::TempDir, Box<dyn std::error::Error>> {
    let directory = tempfile::tempdir()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700))?;
    }
    Ok(directory)
}
