use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;

use codex_hepta_linux_v8_native::InstallExecutionFailureV2;
use codex_hepta_linux_v8_native::MAX_INSTALL_AUTHORITY_BYTES_V8;
use codex_hepta_linux_v8_native::MAX_INSTALL_PLAN_BYTES_V8;
use codex_hepta_linux_v8_native::NativeErrorV8;
use codex_hepta_linux_v8_native::build_install_plan_v8;
use codex_hepta_linux_v8_native::canonical_install_plan_bytes_v8;
use codex_hepta_linux_v8_native::canonical_install_result_bytes_v8;
use codex_hepta_linux_v8_native::current_unix_seconds_v8;
use codex_hepta_linux_v8_native::execute_install_plan_v8;
use codex_hepta_linux_v8_native::install_plan_sha256_v8;
use codex_hepta_linux_v8_native::parse_fresh_install_plan_v8;
use codex_hepta_linux_v8_native::parse_signed_install_authority_bytes_v8;
use codex_hepta_linux_v8_native::read_bounded_regular_absolute_v8;
use codex_hepta_linux_v8_native::verify_install_authority_for_plan_v8;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), InstallExecutionFailureV2> {
    let arguments = std::env::args_os()
        .skip(1)
        .map(|argument| {
            argument.into_string().map_err(|_| {
                NativeErrorV8::Invalid("command argument is not valid UTF-8".to_string())
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    match arguments.as_slice() {
        [command] if command == "install-plan" => {
            let current = std::env::current_exe()?;
            if current.file_name().and_then(|name| name.to_str()) != Some("hepta-linux-v8ctl") {
                return Err(NativeErrorV8::Invalid(
                    "install-plan must run from the current hepta-linux-v8ctl ELF".to_string(),
                )
                .into());
            }
            let directory = current.parent().ok_or_else(|| {
                NativeErrorV8::Invalid("current executable has no artifact directory".to_string())
            })?;
            emit_install_plan(directory)?;
            Ok(())
        }
        [
            command,
            execute,
            plan_option,
            plan_path,
            digest_option,
            digest,
            authority_option,
            authority_path,
        ]
            if command == "install"
                && execute == "--execute"
                && plan_option == "--plan"
                && digest_option == "--plan-sha256"
                && authority_option == "--authority" =>
        {
            execute_install(Path::new(plan_path), digest, Path::new(authority_path))
        }
        _ => Err(NativeErrorV8::Invalid(
            "usage: hepta-linux-v8ctl install-plan | install --execute --plan FILE --plan-sha256 HEX --authority SIGNED_JSON"
                .to_string(),
        )
        .into()),
    }
}

fn emit_install_plan(directory: &Path) -> Result<(), NativeErrorV8> {
    let now = current_unix_seconds_v8()?;
    let plan = build_install_plan_v8(directory, now)?;
    let bytes = canonical_install_plan_bytes_v8(&plan)?;
    let digest = install_plan_sha256_v8(&plan)?;
    std::io::stdout().lock().write_all(&bytes)?;
    eprintln!("plan_sha256={digest}");
    Ok(())
}

fn execute_install(
    plan_path: &Path,
    expected_digest: &str,
    authority_path: &Path,
) -> Result<(), InstallExecutionFailureV2> {
    let plan_path = absolute_input_path(plan_path)?;
    let authority_path = absolute_input_path(authority_path)?;
    let bytes = read_bounded_regular_absolute_v8(&plan_path, MAX_INSTALL_PLAN_BYTES_V8)?;
    let now = current_unix_seconds_v8()?;
    let plan = parse_fresh_install_plan_v8(&bytes, expected_digest, now)?;
    let authority_bytes =
        read_bounded_regular_absolute_v8(&authority_path, MAX_INSTALL_AUTHORITY_BYTES_V8)?;
    let signed_authority = parse_signed_install_authority_bytes_v8(&authority_bytes)?;
    let authority = verify_install_authority_for_plan_v8(&signed_authority, &plan)?;
    let result = execute_install_plan_v8(&plan, &authority)?;
    let result_bytes = canonical_install_result_bytes_v8(&result)?;
    std::io::stdout().lock().write_all(&result_bytes)?;
    Ok(())
}

fn absolute_input_path(path: &Path) -> Result<PathBuf, NativeErrorV8> {
    if !path.is_absolute() {
        return Err(NativeErrorV8::Invalid(
            "install plan and authority paths must be absolute".to_string(),
        ));
    }
    Ok(path.to_path_buf())
}
