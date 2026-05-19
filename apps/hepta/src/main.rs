#![allow(clippy::collapsible_if, clippy::type_complexity)]

use std::collections::BTreeSet;
use std::env;
use std::io::{IsTerminal, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use console::{Color, Style};
use hepta_cli::{CliApp, DEFAULT_SNAPSHOT_PATH, OnboardOptions, ParsedCommand, parse_command};
use hepta_gateway::GatewaySurface;

static WIZARD_STEP_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct HeptaClackTheme;

impl cliclack::Theme for HeptaClackTheme {
    fn bar_color(&self, state: &cliclack::ThemeState) -> Style {
        match state {
            cliclack::ThemeState::Active => lobster(),
            cliclack::ThemeState::Submit => Style::new().dim(),
            cliclack::ThemeState::Cancel => Style::new().red(),
            cliclack::ThemeState::Error(_) => Style::new().yellow(),
        }
    }

    fn state_symbol_color(&self, state: &cliclack::ThemeState) -> Style {
        match state {
            cliclack::ThemeState::Submit => Style::new().green(),
            _ => self.bar_color(state),
        }
    }

    fn radio_symbol(&self, state: &cliclack::ThemeState, selected: bool) -> String {
        match state {
            cliclack::ThemeState::Active if selected => lobster().apply_to("●").to_string(),
            cliclack::ThemeState::Active if !selected => {
                Style::new().dim().apply_to("○").to_string()
            }
            _ => String::new(),
        }
    }

    fn info_symbol(&self) -> String {
        lobster().apply_to("●").to_string()
    }

    fn active_symbol(&self) -> String {
        lobster().apply_to("◆").to_string()
    }
}

fn lobster() -> Style {
    Style::new().fg(Color::TrueColor(255, 90, 45))
}

#[tokio::main]
async fn main() {
    let cli = CliApp::new();
    let gateway = GatewaySurface;
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.len() == 1 && matches!(args[0].as_str(), "--help" | "-h") {
        match cli.execute_command("/help").await {
            Ok(output) => {
                println!("{}", output);
                return;
            }
            Err(err) => {
                eprintln!("command failed: {}", err);
                std::process::exit(1);
            }
        }
    }

    let autoload_enabled = env::var("HEPTA_AUTOLOAD")
        .map(|value| value != "0")
        .unwrap_or(true);
    let autosave_enabled = env::var("HEPTA_AUTOSAVE")
        .map(|value| value != "0")
        .unwrap_or(true);

    if autoload_enabled && Path::new(DEFAULT_SNAPSHOT_PATH).exists() {
        if let Err(err) = cli.load_snapshot(DEFAULT_SNAPSHOT_PATH) {
            eprintln!("autoload warning: {}", err);
        }
    }

    if args.first().map(String::as_str) == Some("--serve-ui") {
        let mut bind_arg_index = 1usize;
        let bind_addr = args
            .get(bind_arg_index)
            .filter(|arg| !arg.starts_with("--"))
            .map(|arg| {
                bind_arg_index = 2;
                arg.as_str()
            })
            .unwrap_or("127.0.0.1:7373");
        let serve_options = match GatewayServeOptions::from_args(&args[bind_arg_index..]) {
            Ok(options) => options,
            Err(err) => {
                eprintln!("control ui server failed: {}", err);
                std::process::exit(1);
            }
        };
        let _telegram_plugin_thread = if serve_options.with_telegram_plugin {
            start_gateway_owned_telegram_plugin(serve_options.telegram_plugin_poll_ms)
        } else {
            None
        };
        if let Err(err) = serve_control_ui(&cli, bind_addr).await {
            eprintln!("control ui server failed: {}", err);
            std::process::exit(1);
        }
        return;
    }

    let raw_input = if args.is_empty() {
        "/help".to_string()
    } else {
        args.join(" ")
    };

    let commands = raw_input
        .split(";;")
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .collect::<Vec<_>>();

    if commands.len() == 1 && is_interactive_onboard_request(commands[0]) {
        println!("{} | surface={}", cli.banner(), gateway.id());
        match run_interactive_onboard(&cli, commands[0], autosave_enabled).await {
            Ok(()) => return,
            Err(err) => {
                eprintln!("command failed: {}", err);
                std::process::exit(1);
            }
        }
    }

    let pure_json_output = commands.len() == 1 && command_requests_json_output(commands[0]);
    if !pure_json_output {
        println!("{} | surface={}", cli.banner(), gateway.id());
    }

    for command in commands {
        if !pure_json_output {
            println!("> {}", command);
        }
        match cli.execute_command(command).await {
            Ok(output) => {
                println!("{}", output);
                if autosave_enabled {
                    if let Err(err) = cli.autosave_snapshot(DEFAULT_SNAPSHOT_PATH) {
                        eprintln!("autosave warning: {}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("command failed: {}", err);
                std::process::exit(1);
            }
        }
    }
}

fn command_requests_json_output(command: &str) -> bool {
    command.split_whitespace().any(|token| token == "--json")
}

fn is_interactive_onboard_request(command: &str) -> bool {
    let normalized = command
        .trim()
        .strip_prefix('/')
        .or_else(|| command.trim().strip_prefix('!'))
        .unwrap_or(command.trim())
        .trim();
    let mut parts = normalized.split_whitespace();
    matches!(parts.next(), Some("onboard"))
        && !normalized
            .split_whitespace()
            .any(|token| matches!(token, "--non-interactive" | "--json" | "--help" | "-h"))
}

async fn run_interactive_onboard(
    cli: &CliApp,
    command: &str,
    autosave_enabled: bool,
) -> Result<(), String> {
    let mut options = match parse_command(command)? {
        ParsedCommand::Onboard { options } => options,
        _ => return Err("internal error: expected onboard command".into()),
    };

    if options.install_daemon {
        return run_hepta_runtime_style_install_daemon_onboard(
            cli,
            command,
            options,
            autosave_enabled,
        )
        .await;
    }

    WIZARD_STEP_COUNTER.store(0, Ordering::SeqCst);
    if use_clack_prompts() {
        cliclack::set_theme(HeptaClackTheme);
        cliclack::intro("Hepta Onboarding").map_err(|err| err.to_string())?;
        cliclack::note(
            "HeptaRuntime parity target",
            "Guided local setup with review-before-write safety. Use ↑/↓, Enter, y/n, or Esc to cancel.",
        )
        .map_err(|err| err.to_string())?;
    } else {
        print_wizard_hero();
    }
    print_smart_preflight(cli);

    let profile_recommendation = onboard_profile_recommendation(cli, &options);
    let profile = prompt_select(
        "Setup profile",
        &[
            (
                "safe-local",
                "loopback-only, token auth, health-gated local setup",
            ),
            (
                "developer",
                "source-first iteration with dry-run defaults and diagnostics",
            ),
            (
                "persistent-operator",
                "LaunchAgent gateway setup that keeps Hepta alive",
            ),
            (
                "migration",
                "metadata-only HeptaRuntime import; secrets stay out by default",
            ),
            (
                "custom",
                "ask every setup question without applying an opinionated profile",
            ),
        ],
        profile_recommendation.default_index,
    )?;
    let explicit_options = options.clone();
    apply_onboard_profile_defaults(&profile, &mut options);
    preserve_explicit_onboard_intent(command, &explicit_options, &mut options);
    normalize_onboard_derived_security(&mut options);
    prompt_missing_onboard_secrets(&mut options)?;
    print_profile_panel(&profile, &options, profile_recommendation.reason);

    let customize_profile = profile == "custom"
        || prompt_confirm("Customize these profile defaults before review?", false)?;
    if profile == "custom" {
        prompt_onboard_deviations(&mut options)?;
    } else if customize_profile {
        prompt_onboard_targeted_deviations(&mut options)?;
    }

    normalize_onboard_derived_security(&mut options);
    prompt_missing_onboard_secrets(&mut options)?;
    validate_onboard_automation_args(&options)?;
    print_review_panel(&options);
    let (risk_level, risk_detail, needs_risk_ack) = onboard_risk_summary(&options);
    if needs_risk_ack {
        let risk_prompt = format!("Acknowledge {risk_level} setup risk? {risk_detail}");
        if !prompt_confirm(&risk_prompt, false)? {
            if use_clack_prompts() {
                cliclack::outro_cancel("Cancelled before elevated-risk setup.")
                    .map_err(|err| err.to_string())?;
            } else {
                println!("\nCancelled before elevated-risk setup.");
            }
            return Ok(());
        }
    }

    if !prompt_confirm("Proceed with this onboarding plan?", true)? {
        if use_clack_prompts() {
            cliclack::outro_cancel("Cancelled. Nothing was changed.")
                .map_err(|err| err.to_string())?;
        } else {
            println!("\nCancelled. Nothing was changed.");
        }
        return Ok(());
    }

    let automation_command = build_onboard_automation_command(&options);
    let display_command = build_onboard_automation_command_redacted(&options);
    if use_clack_prompts() {
        cliclack::log::remark(format!("Running: {display_command}"))
            .map_err(|err| err.to_string())?;
    } else {
        println!("\n{} {}", accent("›"), command_style(&display_command));
    }
    let output = cli.execute_command(&automation_command).await?;
    println!("{}", output);
    if use_clack_prompts() {
        cliclack::outro("Onboarding plan complete.").map_err(|err| err.to_string())?;
    }
    if autosave_enabled {
        if let Err(err) = cli.autosave_snapshot(DEFAULT_SNAPSHOT_PATH) {
            eprintln!("autosave warning: {}", err);
        }
    }
    Ok(())
}

async fn run_hepta_runtime_style_install_daemon_onboard(
    cli: &CliApp,
    command: &str,
    mut options: OnboardOptions,
    autosave_enabled: bool,
) -> Result<(), String> {
    WIZARD_STEP_COUNTER.store(0, Ordering::SeqCst);
    if use_clack_prompts() {
        cliclack::set_theme(HeptaClackTheme);
        cliclack::intro("Hepta onboard").map_err(|err| err.to_string())?;
        cliclack::note(
            "HeptaRuntime onboard parity",
            "This path mirrors HeptaRuntime's onboard finalize daemon page: workspace/gateway setup, Gateway service install/restart/reinstall/skip, health gate, and Control UI summary.",
        )
        .map_err(|err| err.to_string())?;
    } else {
        print_wizard_hero();
        println!(
            "{} {}",
            accent("›"),
            strong("HeptaRuntime-style onboard daemon finalize flow")
        );
    }
    print_smart_preflight(cli);

    let explicit_options = options.clone();
    apply_onboard_profile_defaults("persistent-operator", &mut options);
    preserve_explicit_onboard_intent(command, &explicit_options, &mut options);
    normalize_onboard_derived_security(&mut options);

    let mode = prompt_select_labeled(
        "Where will the Gateway run?",
        &[
            ("local", "Local (this machine)", "Gateway runs on this Mac"),
            (
                "remote",
                "Remote (info-only)",
                "Configure a remote Gateway URL instead of installing a local service",
            ),
        ],
        if options.mode == "remote" { 1 } else { 0 },
    )?;
    options.mode = mode;
    if options.mode == "remote" {
        if use_clack_prompts() {
            cliclack::outro("Remote gateway configured.").map_err(|err| err.to_string())?;
        } else {
            println!("{} Remote gateway configured.", success("✓"));
        }
        return Ok(());
    }

    prompt_hepta_runtime_workspace_page(&mut options)?;
    prompt_hepta_runtime_gateway_page(&mut options)?;
    prompt_hepta_runtime_daemon_page(cli, &mut options)?;
    prompt_missing_onboard_secrets(&mut options)?;
    normalize_onboard_derived_security(&mut options);
    validate_onboard_automation_args(&options)?;
    print_review_panel(&options);

    let (risk_level, risk_detail, needs_risk_ack) = onboard_risk_summary(&options);
    if needs_risk_ack {
        let risk_prompt = format!("Acknowledge {risk_level} setup risk? {risk_detail}");
        if !prompt_confirm(&risk_prompt, false)? {
            if use_clack_prompts() {
                cliclack::outro_cancel("Cancelled before elevated-risk setup.")
                    .map_err(|err| err.to_string())?;
            } else {
                println!("\nCancelled before elevated-risk setup.");
            }
            return Ok(());
        }
    }
    if !prompt_confirm("Proceed with this onboarding plan?", true)? {
        if use_clack_prompts() {
            cliclack::outro_cancel("Cancelled. Nothing was changed.")
                .map_err(|err| err.to_string())?;
        } else {
            println!("\nCancelled. Nothing was changed.");
        }
        return Ok(());
    }

    let automation_command = build_onboard_automation_command(&options);
    let display_command = build_onboard_automation_command_redacted(&options);
    if use_clack_prompts() {
        cliclack::log::remark(format!("Running: {display_command}"))
            .map_err(|err| err.to_string())?;
    } else {
        println!("\n{} {}", accent("›"), command_style(&display_command));
    }
    print_hepta_runtime_daemon_progress_start(&options);
    let output = cli.execute_command(&automation_command).await?;
    println!("{}", output);
    print_hepta_runtime_daemon_progress_finish(&options, &output);
    print_hepta_runtime_optional_apps_note();
    print_hepta_runtime_control_ui_panel(cli, &options);
    prompt_hepta_runtime_hatch_page(cli, &options)?;
    print_hepta_runtime_finalize_followups(&options);
    if use_clack_prompts() {
        cliclack::outro("Onboarding complete. Use the dashboard link above to control Hepta.")
            .map_err(|err| err.to_string())?;
    } else {
        println!(
            "{} Onboarding complete. Use the dashboard link above to control Hepta.",
            success("✓")
        );
    }
    if autosave_enabled {
        if let Err(err) = cli.autosave_snapshot(DEFAULT_SNAPSHOT_PATH) {
            eprintln!("autosave warning: {}", err);
        }
    }
    Ok(())
}

fn default_onboard_profile_index(options: &OnboardOptions) -> usize {
    if matches!(options.flow.as_deref(), Some("import")) || options.import_from.is_some() {
        return 3;
    }
    if options.install_daemon {
        return 2;
    }
    if options.dry_run {
        return 1;
    }
    0
}

#[derive(Debug, Clone, Copy)]
struct OnboardProfileRecommendation {
    default_index: usize,
    reason: &'static str,
}

fn onboard_profile_recommendation(
    cli: &CliApp,
    options: &OnboardOptions,
) -> OnboardProfileRecommendation {
    let explicit_index = default_onboard_profile_index(options);
    if explicit_index != 0 {
        let reason = match explicit_index {
            1 => "explicit dry-run option kept the developer profile selected",
            2 => "explicit daemon install option kept persistent-operator selected",
            3 => "explicit import option kept migration selected",
            _ => "explicit CLI options kept this profile selected",
        };
        return OnboardProfileRecommendation {
            default_index: explicit_index,
            reason,
        };
    }

    let ops = cli.ops_status_report();
    if !ops.installed_binary_executable {
        return OnboardProfileRecommendation {
            default_index: 1,
            reason: "installed hepta binary is missing, so developer dry-run is safest first",
        };
    }
    if !ops.service_plist_present || !ops.service_loaded || !ops.service_live_ok {
        return OnboardProfileRecommendation {
            default_index: 2,
            reason: "gateway service is missing or not live, so persistent-operator is recommended",
        };
    }
    OnboardProfileRecommendation {
        default_index: 0,
        reason: "local install and gateway service look healthy, so safe-local is enough",
    }
}

fn apply_onboard_profile_defaults(profile: &str, options: &mut OnboardOptions) {
    let workspace = options
        .workspace
        .clone()
        .or_else(|| env::var("HEPTA_WORKSPACE").ok())
        .unwrap_or_else(|| ".hepta/workspace".into());
    let workspace = command_token_default(&workspace, ".hepta/workspace");
    let explicit_port = options.gateway_port.is_some();
    let requested_port = options
        .gateway_port
        .or_else(|| parse_bind_port(&options.ui_bind))
        .unwrap_or(7373);
    let port =
        if !explicit_port && matches!(loopback_port_state(requested_port), PortState::Blocked) {
            next_free_loopback_port(requested_port).unwrap_or(requested_port)
        } else {
            requested_port
        };

    options.mode = "local".into();
    options.workspace = Some(workspace.to_string());
    options.gateway_port = Some(port);
    options.ui_bind = format!("127.0.0.1:{port}");

    match profile {
        "developer" => {
            options.flow = Some("advanced".into());
            options.gateway_bind = "loopback".into();
            options.gateway_auth = "token".into();
            options.gateway_password = None;
            options.install_daemon = false;
            options.daemon_runtime = "node".into();
            options.daemon_action = "skip".into();
            options.skip_health = false;
            options.skip_bootstrap = false;
            options.skip_skills = false;
            options.node_manager = "npm".into();
            options.import_from = None;
            options.import_source = None;
            options.import_secrets = false;
            options.dry_run = true;
        }
        "persistent-operator" => {
            options.flow = Some("advanced".into());
            options.gateway_bind = "loopback".into();
            options.gateway_auth = "token".into();
            options.gateway_password = None;
            options.install_daemon = true;
            options.daemon_runtime = "node".into();
            options.daemon_action = "install".into();
            options.daemon_interval_seconds = options.daemon_interval_seconds.max(300);
            options.skip_health = false;
            options.skip_bootstrap = false;
            options.skip_skills = false;
            options.node_manager = "npm".into();
            options.import_from = None;
            options.import_source = None;
            options.import_secrets = false;
            options.dry_run = false;
        }
        "migration" => {
            options.flow = Some("import".into());
            options.gateway_bind = "loopback".into();
            options.gateway_auth = "token".into();
            options.gateway_password = None;
            options.install_daemon = false;
            options.daemon_runtime = "node".into();
            options.daemon_action = "skip".into();
            options.skip_health = false;
            options.skip_bootstrap = false;
            options.skip_skills = false;
            options.node_manager = "npm".into();
            options.import_from = Some(
                options
                    .import_from
                    .clone()
                    .unwrap_or_else(|| "hepta_runtime".into()),
            );
            options.import_source = Some(
                options
                    .import_source
                    .clone()
                    .unwrap_or_else(|| "~/.hepta_runtime".into()),
            );
            options.import_secrets = false;
            options.dry_run = false;
        }
        "custom" => {
            options.flow.get_or_insert_with(|| "advanced".into());
        }
        _ => {
            options.flow = Some("quickstart".into());
            options.gateway_bind = "loopback".into();
            options.gateway_auth = "token".into();
            options.gateway_password = None;
            options.install_daemon = false;
            options.daemon_runtime = "node".into();
            options.daemon_action = "skip".into();
            options.skip_health = false;
            options.skip_bootstrap = false;
            options.skip_skills = false;
            options.node_manager = "npm".into();
            options.import_from = None;
            options.import_source = None;
            options.import_secrets = false;
            options.dry_run = false;
        }
    }
}

fn preserve_explicit_onboard_intent(
    command: &str,
    explicit: &OnboardOptions,
    options: &mut OnboardOptions,
) {
    if explicit.mode == "remote" {
        options.mode = explicit.mode.clone();
    }
    if onboard_command_has_any_flag(command, &["--flow"]) {
        options.flow = explicit.flow.clone();
    }
    if explicit.modern {
        options.modern = true;
    }
    if onboard_command_has_any_flag(command, &["--secret-input-mode"]) {
        options.secret_input_mode = explicit.secret_input_mode.clone();
    }
    if onboard_command_has_any_flag(command, &["--install-daemon"]) {
        options.install_daemon = true;
        if explicit.daemon_action == "skip" {
            options.daemon_action = "install".into();
        }
    }
    if onboard_command_has_any_flag(command, &["--skip-daemon", "--no-install-daemon"]) {
        options.install_daemon = false;
        options.daemon_action = "skip".into();
    }
    if onboard_command_has_any_flag(command, &["--daemon-action"]) {
        options.daemon_action = explicit.daemon_action.clone();
        options.install_daemon = options.daemon_action != "skip";
    }
    if onboard_command_has_any_flag(command, &["--gateway-bind"]) {
        options.gateway_bind = explicit.gateway_bind.clone();
    }
    if onboard_command_has_any_flag(command, &["--gateway-auth"]) {
        options.gateway_auth = explicit.gateway_auth.clone();
    }
    if onboard_command_has_any_flag(command, &["--gateway-password"]) {
        options.gateway_password = explicit.gateway_password.clone();
    }
    if onboard_command_has_any_flag(command, &["--daemon-runtime"]) {
        options.daemon_runtime = explicit.daemon_runtime.clone();
    }
    if onboard_command_has_any_flag(command, &["--node-manager"]) {
        options.node_manager = explicit.node_manager.clone();
    }
    if onboard_command_has_any_flag(command, &["--import-from"]) {
        options.import_from = explicit.import_from.clone();
    }
    if onboard_command_has_any_flag(command, &["--import-source"]) {
        options.import_source = explicit.import_source.clone();
    }
    if onboard_command_has_any_flag(command, &["--import-secrets"]) {
        options.import_secrets = true;
    }
    if explicit.dry_run {
        options.dry_run = true;
    }
    if explicit.skip_bootstrap {
        options.skip_bootstrap = true;
    }
    if explicit.skip_channels {
        options.skip_channels = true;
    }
    if explicit.skip_skills {
        options.skip_skills = true;
    }
    if explicit.skip_search {
        options.skip_search = true;
    }
    if explicit.skip_health {
        options.skip_health = true;
    }
    if explicit.skip_ui {
        options.skip_ui = true;
    }
}

fn onboard_command_has_any_flag(command: &str, flags: &[&str]) -> bool {
    command
        .split_whitespace()
        .any(|token| flags.contains(&token))
}

fn normalize_onboard_derived_security(options: &mut OnboardOptions) {
    if !options.install_daemon {
        options.daemon_action = "skip".into();
    }

    if options.tailscale == "funnel" {
        options.gateway_auth = "password".into();
        options.gateway_bind = "loopback".into();
    } else if options.tailscale != "off" {
        options.gateway_bind = "loopback".into();
    }

    if options.gateway_auth == "password" {
        options.gateway_token = None;
        options.gateway_token_ref_env = None;
    } else {
        options.gateway_password = None;
    }
}

fn prompt_missing_onboard_secrets(options: &mut OnboardOptions) -> Result<(), String> {
    if options.gateway_auth == "password" && options.gateway_password.is_none() {
        let password = prompt_password_token("Gateway password", "gateway-password")?;
        options.gateway_password = Some(password);
    }
    Ok(())
}

fn prompt_onboard_deviations(options: &mut OnboardOptions) -> Result<(), String> {
    let flow = prompt_select(
        "Setup flow",
        &[
            ("quickstart", "recommended local setup"),
            ("advanced", "custom gateway/daemon details"),
            (
                "import",
                "import metadata from an existing HeptaRuntime/Hepta config",
            ),
        ],
        match options.flow.as_deref() {
            Some("advanced") => 1,
            Some("import") => 2,
            _ => 0,
        },
    )?;
    options.flow = Some(flow.clone());

    options.mode = prompt_select(
        "Runtime mode",
        &[("local", "run source-first on this Mac")],
        0,
    )?;

    let default_workspace = options
        .workspace
        .clone()
        .or_else(|| env::var("HEPTA_WORKSPACE").ok())
        .unwrap_or_else(|| ".hepta/workspace".into());
    let default_workspace = command_token_default(&default_workspace, ".hepta/workspace");
    options.workspace = Some(prompt_command_token_text(
        "Workspace directory",
        default_workspace,
        "workspace",
    )?);

    let port_default = options
        .gateway_port
        .or_else(|| parse_bind_port(&options.ui_bind))
        .unwrap_or(7373);
    let port = prompt_u16("Gateway / Control UI port", port_default)?;
    options.gateway_port = Some(port);
    options.ui_bind = format!("127.0.0.1:{port}");

    if flow != "quickstart" {
        options.gateway_bind = prompt_select(
            "Gateway exposure",
            &[
                ("loopback", "local machine only; safest default"),
                ("tailnet", "Tailscale/private network planning"),
                ("lan", "LAN exposure; use only on trusted networks"),
            ],
            match options.gateway_bind.as_str() {
                "tailnet" => 1,
                "lan" => 2,
                _ => 0,
            },
        )?;
        options.gateway_auth = prompt_select(
            "Gateway auth",
            &[
                ("token", "generated or env-referenced token"),
                ("password", "operator password gate"),
            ],
            if options.gateway_auth == "password" {
                1
            } else {
                0
            },
        )?;
        if options.gateway_auth == "password" && options.gateway_password.is_none() {
            let password = prompt_password_token("Gateway password", "gateway-password")?;
            if password.trim().is_empty() {
                return Err(
                    "gateway password cannot be empty when password auth is selected".into(),
                );
            }
            options.gateway_password = Some(password);
        }
    }

    let install_daemon_default = options.install_daemon;
    options.install_daemon = prompt_confirm(
        "Install macOS LaunchAgent daemon so Hepta keeps running?",
        install_daemon_default,
    )?;
    options.daemon_action = if options.install_daemon {
        "install"
    } else {
        "skip"
    }
    .into();
    if options.install_daemon {
        options.daemon_runtime = prompt_select(
            "Daemon runtime",
            &[("node", "default"), ("bun", "experimental")],
            if options.daemon_runtime == "bun" {
                1
            } else {
                0
            },
        )?;
        options.skip_health = !prompt_confirm(
            "Wait for post-install gateway health check?",
            !options.skip_health,
        )?;
    }

    options.skip_bootstrap = !prompt_confirm(
        "Create/update local workspace and onboarding docs?",
        !options.skip_bootstrap,
    )?;
    options.skip_skills = !prompt_confirm(
        "Record skills/node-manager preference?",
        !options.skip_skills,
    )?;
    if !options.skip_skills {
        options.node_manager = prompt_select(
            "Node package manager for skills",
            &[
                ("npm", "default"),
                ("pnpm", "operator-managed"),
                ("bun", "experimental"),
            ],
            match options.node_manager.as_str() {
                "pnpm" => 1,
                "bun" => 2,
                _ => 0,
            },
        )?;
    }

    if flow == "import"
        || prompt_confirm(
            "Import existing HeptaRuntime config metadata?",
            options.import_from.is_some(),
        )?
    {
        options.import_from = Some(prompt_command_token_text(
            "Import provider/source label",
            options.import_from.as_deref().unwrap_or("hepta_runtime"),
            "import-from",
        )?);
        options.import_source = Some(prompt_command_token_text(
            "Import source path",
            command_token_default(
                options
                    .import_source
                    .as_deref()
                    .unwrap_or("~/.hepta_runtime"),
                "~/.hepta_runtime",
            ),
            "import-source",
        )?);
        options.import_secrets = prompt_confirm(
            "Import/copy secrets too? (default no; metadata-only is safer)",
            options.import_secrets,
        )?;
    }

    options.dry_run = prompt_confirm("Dry-run only?", options.dry_run)?;
    Ok(())
}

fn prompt_onboard_targeted_deviations(options: &mut OnboardOptions) -> Result<(), String> {
    loop {
        let choice = prompt_select(
            "Customize one area",
            &[
                ("workspace-port", "workspace directory and Control UI port"),
                ("gateway-security", "gateway exposure and auth"),
                (
                    "daemon-health",
                    "LaunchAgent gateway daemon, runtime, health gate",
                ),
                (
                    "bootstrap-skills",
                    "workspace bootstrap and skills node manager",
                ),
                (
                    "import",
                    "HeptaRuntime/Hepta metadata import and secret-copy policy",
                ),
                ("dry-run", "toggle dry-run before any writes"),
                ("review", "done customizing; continue to final review"),
            ],
            6,
        )?;
        match choice.as_str() {
            "workspace-port" => prompt_workspace_and_port(options)?,
            "gateway-security" => prompt_gateway_security(options)?,
            "daemon-health" => prompt_daemon_health(options)?,
            "bootstrap-skills" => prompt_bootstrap_and_skills(options)?,
            "import" => prompt_import_settings(options)?,
            "dry-run" => options.dry_run = prompt_confirm("Dry-run only?", options.dry_run)?,
            "review" => break,
            _ => unreachable!("prompt_select returned an unknown customization choice"),
        }
        print_profile_panel(
            "customized",
            options,
            "targeted customization was just applied",
        );
    }
    Ok(())
}

fn prompt_hepta_runtime_workspace_page(options: &mut OnboardOptions) -> Result<(), String> {
    let default_workspace = options
        .workspace
        .clone()
        .or_else(|| env::var("HEPTA_WORKSPACE").ok())
        .unwrap_or_else(|| ".hepta/workspace".into());
    let default_workspace = command_token_default(&default_workspace, ".hepta/workspace");
    options.workspace = Some(prompt_command_token_text(
        "Workspace directory",
        default_workspace,
        "workspace",
    )?);
    Ok(())
}

fn prompt_hepta_runtime_gateway_page(options: &mut OnboardOptions) -> Result<(), String> {
    let port_default = options
        .gateway_port
        .or_else(|| parse_bind_port(&options.ui_bind))
        .unwrap_or(7373);
    let port = prompt_u16("Gateway port", port_default)?;
    options.gateway_port = Some(port);
    options.ui_bind = format!("127.0.0.1:{port}");

    options.gateway_bind = prompt_select_labeled(
        "Gateway bind mode",
        &[
            (
                "loopback",
                "Loopback (Local only)",
                "Bind to 127.0.0.1 - secure, local-only access",
            ),
            (
                "tailnet",
                "Tailnet (Tailscale IP)",
                "Bind to your Tailscale IP only (100.x.x.x)",
            ),
            (
                "auto",
                "Auto (Loopback → LAN)",
                "Prefer loopback; fall back to all interfaces if unavailable",
            ),
            (
                "lan",
                "LAN (All interfaces)",
                "Bind to 0.0.0.0 - accessible from anywhere on your network",
            ),
            (
                "custom",
                "Custom IP",
                "Specify a specific IP address, with 0.0.0.0 fallback if unavailable",
            ),
        ],
        match options.gateway_bind.as_str() {
            "tailnet" => 1,
            "auto" => 2,
            "lan" => 3,
            "custom" => 4,
            _ => 0,
        },
    )?;
    if options.gateway_bind == "custom" {
        let _custom_ip = prompt_text("Custom IP address", "192.168.1.100")?;
        if use_clack_prompts() {
            let _ = cliclack::note(
                "Custom IP",
                "Hepta onboard currently records bind=custom for parity, but the local gateway service remains loopback-bound unless the lower runtime adds custom host support.",
            );
        }
    }

    let auth = prompt_select_labeled(
        "Gateway auth",
        &[
            ("token", "Token", "Recommended default"),
            ("password", "Password", ""),
            (
                "trusted-proxy",
                "Trusted Proxy",
                "Behind reverse proxy (Pomerium, Caddy, Traefik, etc.)",
            ),
        ],
        if options.gateway_auth == "password" {
            1
        } else {
            0
        },
    )?;
    if auth == "trusted-proxy" {
        if use_clack_prompts() {
            let _ = cliclack::note(
                "Trusted Proxy Auth",
                "HeptaRuntime supports trusted-proxy auth here. Hepta onboard shows the same pages for operator parity, but does not persist trusted-proxy config yet, so this run keeps token auth and leaves proxy setup unchanged.",
            );
        } else {
            println!(
                "{} {}",
                warn("!"),
                muted(
                    "Trusted proxy page acknowledged; Hepta keeps token auth until proxy config is implemented."
                )
            );
        }
        let _user_header = prompt_text("Header containing user identity", "x-forwarded-user")?;
        let _required_headers = prompt_text("Required headers (comma-separated, optional)", "")?;
        let _allowed_users = prompt_text(
            "Allowed users (comma-separated, blank = all authenticated users)",
            "",
        )?;
        let _trusted_proxies = prompt_text("Trusted proxy IPs (comma-separated)", "127.0.0.1")?;
        options.gateway_auth = "token".into();
    } else {
        options.gateway_auth = auth;
    }

    if options.gateway_auth == "token" {
        let token_source = prompt_select_labeled(
            "Gateway token source",
            &[
                ("plaintext", "Generate/store plaintext token", "Default"),
                (
                    "ref",
                    "Use SecretRef",
                    "Store an env-backed reference instead of plaintext",
                ),
            ],
            if options.gateway_token_ref_env.is_some() {
                1
            } else {
                0
            },
        )?;
        if token_source == "ref" {
            options.gateway_token = None;
            options.gateway_token_ref_env = Some(prompt_command_token_text(
                "Gateway token env var",
                options
                    .gateway_token_ref_env
                    .as_deref()
                    .unwrap_or("HEPTA_RUNTIME_GATEWAY_TOKEN"),
                "gateway-token-ref-env",
            )?);
        } else {
            options.gateway_token_ref_env = None;
        }
    }
    if options.gateway_auth == "password" && options.gateway_password.is_none() {
        let password = prompt_password_token("Gateway password", "gateway-password")?;
        if password.trim().is_empty() {
            return Err("gateway password cannot be empty when password auth is selected".into());
        }
        options.gateway_password = Some(password);
    }

    options.tailscale = prompt_select_labeled(
        "Tailscale exposure",
        &[
            ("off", "Off", "Do not run tailscale serve/funnel"),
            ("serve", "Serve", "Tailnet-private exposure"),
            (
                "funnel",
                "Funnel",
                "Public internet exposure; requires password auth",
            ),
        ],
        match options.tailscale.as_str() {
            "serve" => 1,
            "funnel" => 2,
            _ => 0,
        },
    )?;
    if options.tailscale != "off" {
        if use_clack_prompts() {
            let _ = cliclack::note(
                "Tailscale",
                "Tailscale serve/funnel requires the gateway bind to stay loopback; Funnel requires password auth.",
            );
        }
        let _reset_on_exit = prompt_confirm("Reset Tailscale serve/funnel on exit?", false)?;
    }
    normalize_onboard_derived_security(options);
    Ok(())
}

fn prompt_hepta_runtime_daemon_page(
    cli: &CliApp,
    options: &mut OnboardOptions,
) -> Result<String, String> {
    options.install_daemon = true;
    if matches!(options.flow.as_deref(), Some("quickstart")) {
        options.daemon_runtime = "node".into();
        if use_clack_prompts() {
            let _ = cliclack::note(
                "Gateway service runtime",
                "QuickStart uses Node for the Gateway service (stable + supported).",
            );
        } else {
            println!(
                "{} {}",
                muted("↳"),
                muted("QuickStart uses Node for the Gateway service (stable + supported).")
            );
        }
    } else {
        options.daemon_runtime = prompt_select_labeled(
            "Gateway service runtime",
            &[(
                "node",
                "Node (recommended)",
                "Required for WhatsApp + Telegram. Bun can corrupt memory on reconnect.",
            )],
            0,
        )?;
    }

    let ops = cli.ops_status_report();
    let mut action = "install".to_string();
    if ops.service_loaded {
        action = prompt_select_labeled(
            "Gateway service already installed",
            &[
                ("restart", "Restart", ""),
                ("reinstall", "Reinstall", ""),
                ("skip", "Skip", ""),
            ],
            0,
        )?;
    }
    options.daemon_action = action.clone();
    Ok(action)
}

fn daemon_action_present_tense(action: &str) -> &'static str {
    match action {
        "restart" => "Restarting",
        "skip" => "Skipping",
        _ => "Installing",
    }
}

fn daemon_action_past_tense(action: &str) -> &'static str {
    match action {
        "restart" => "restarted",
        "skip" => "skipped",
        _ => "installed",
    }
}

fn daemon_action_noun(action: &str) -> &'static str {
    match action {
        "restart" => "restart",
        "reinstall" => "reinstall",
        "skip" => "skip",
        _ => "install",
    }
}

fn onboard_output_indicates_failure(output: &str) -> bool {
    output.contains("Hepta onboard: fail")
        || output.contains("\"ok\": false")
        || output.contains("\"status\": \"fail\"")
}

fn print_hepta_runtime_daemon_progress_start(options: &OnboardOptions) {
    if !options.install_daemon || options.daemon_action == "skip" {
        return;
    }
    let action = daemon_action_present_tense(&options.daemon_action);
    if use_clack_prompts() {
        if options.daemon_action == "reinstall" {
            let _ = cliclack::log::remark("Uninstalling Gateway service…");
        }
        let _ = cliclack::log::remark("Preparing Gateway service…");
        let _ = cliclack::log::remark(format!("{action} Gateway service…"));
    } else {
        if options.daemon_action == "reinstall" {
            println!("{} Uninstalling Gateway service…", accent("◒"));
        }
        println!("{} Preparing Gateway service…", accent("◒"));
        println!("{} {action} Gateway service…", accent("◒"));
    }
}

fn print_hepta_runtime_daemon_progress_finish(options: &OnboardOptions, output: &str) {
    if !options.install_daemon || options.daemon_action == "skip" {
        return;
    }
    if onboard_output_indicates_failure(output) {
        let body = "Gateway service setup was blocked or failed. The onboarding report above includes the phase, diagnostics, and next hint.";
        if use_clack_prompts() {
            let _ = cliclack::note("Gateway service install blocked", body);
        } else {
            println!("{} {}", warn("!"), warn("Gateway service install blocked."));
            println!("{} {}", accent("│"), muted(body));
        }
        return;
    }

    let message = if options.dry_run {
        let noun = daemon_action_noun(&options.daemon_action);
        format!("Gateway service {noun} planned.")
    } else {
        let past = daemon_action_past_tense(&options.daemon_action);
        format!("Gateway service {past}.")
    };
    if use_clack_prompts() {
        let _ = cliclack::note("Gateway service ready", message);
    } else {
        println!("{} {}", success("✓"), message);
    }
}

fn prompt_workspace_and_port(options: &mut OnboardOptions) -> Result<(), String> {
    let default_workspace = options
        .workspace
        .clone()
        .or_else(|| env::var("HEPTA_WORKSPACE").ok())
        .unwrap_or_else(|| ".hepta/workspace".into());
    let default_workspace = command_token_default(&default_workspace, ".hepta/workspace");
    options.workspace = Some(prompt_command_token_text(
        "Workspace directory",
        default_workspace,
        "workspace",
    )?);

    let port_default = options
        .gateway_port
        .or_else(|| parse_bind_port(&options.ui_bind))
        .unwrap_or(7373);
    let port = prompt_u16("Gateway / Control UI port", port_default)?;
    options.gateway_port = Some(port);
    options.ui_bind = format!("127.0.0.1:{port}");
    Ok(())
}

fn prompt_gateway_security(options: &mut OnboardOptions) -> Result<(), String> {
    options.flow = Some("advanced".into());
    options.gateway_bind = prompt_select(
        "Gateway exposure",
        &[
            ("loopback", "local machine only; safest default"),
            ("tailnet", "Tailscale/private network planning"),
            ("lan", "LAN exposure; use only on trusted networks"),
        ],
        match options.gateway_bind.as_str() {
            "tailnet" => 1,
            "lan" => 2,
            _ => 0,
        },
    )?;
    options.gateway_auth = prompt_select(
        "Gateway auth",
        &[
            ("token", "generated or env-referenced token"),
            ("password", "operator password gate"),
        ],
        if options.gateway_auth == "password" {
            1
        } else {
            0
        },
    )?;
    if options.gateway_auth == "password" && options.gateway_password.is_none() {
        let password = prompt_password_token("Gateway password", "gateway-password")?;
        if password.trim().is_empty() {
            return Err("gateway password cannot be empty when password auth is selected".into());
        }
        options.gateway_password = Some(password);
    }
    if options.gateway_auth == "token" {
        options.gateway_password = None;
    }
    Ok(())
}

fn prompt_daemon_health(options: &mut OnboardOptions) -> Result<(), String> {
    options.install_daemon = prompt_confirm(
        "Install macOS LaunchAgent daemon so Hepta keeps running?",
        options.install_daemon,
    )?;
    options.daemon_action = if options.install_daemon {
        "install"
    } else {
        "skip"
    }
    .into();
    if options.install_daemon {
        options.daemon_runtime = prompt_select(
            "Daemon runtime",
            &[("node", "default"), ("bun", "experimental")],
            if options.daemon_runtime == "bun" {
                1
            } else {
                0
            },
        )?;
        options.skip_health = !prompt_confirm(
            "Wait for post-install gateway health check?",
            !options.skip_health,
        )?;
    } else {
        options.skip_health = false;
    }
    Ok(())
}

fn prompt_bootstrap_and_skills(options: &mut OnboardOptions) -> Result<(), String> {
    options.skip_bootstrap = !prompt_confirm(
        "Create/update local workspace and onboarding docs?",
        !options.skip_bootstrap,
    )?;
    options.skip_skills = !prompt_confirm(
        "Record skills/node-manager preference?",
        !options.skip_skills,
    )?;
    if !options.skip_skills {
        options.node_manager = prompt_select(
            "Node package manager for skills",
            &[
                ("npm", "default"),
                ("pnpm", "operator-managed"),
                ("bun", "experimental"),
            ],
            match options.node_manager.as_str() {
                "pnpm" => 1,
                "bun" => 2,
                _ => 0,
            },
        )?;
    }
    Ok(())
}

fn prompt_import_settings(options: &mut OnboardOptions) -> Result<(), String> {
    if prompt_confirm(
        "Import existing HeptaRuntime config metadata?",
        options.import_from.is_some(),
    )? {
        options.flow = Some("import".into());
        options.import_from = Some(prompt_command_token_text(
            "Import provider/source label",
            options.import_from.as_deref().unwrap_or("hepta_runtime"),
            "import-from",
        )?);
        options.import_source = Some(prompt_command_token_text(
            "Import source path",
            command_token_default(
                options
                    .import_source
                    .as_deref()
                    .unwrap_or("~/.hepta_runtime"),
                "~/.hepta_runtime",
            ),
            "import-source",
        )?);
        options.import_secrets = prompt_confirm(
            "Import/copy secrets too? (default no; metadata-only is safer)",
            options.import_secrets,
        )?;
    } else {
        options.import_from = None;
        options.import_source = None;
        options.import_secrets = false;
        if matches!(options.flow.as_deref(), Some("import")) {
            options.flow = Some("quickstart".into());
        }
    }
    Ok(())
}

fn prompt_select(
    prompt: &str,
    choices: &[(&str, &str)],
    default_index: usize,
) -> Result<String, String> {
    if use_clack_prompts() {
        let mut select = cliclack::select(prompt.to_string())
            .initial_value(choices[default_index].0.to_string())
            .max_rows(8);
        for (value, label) in choices {
            select = select.item((*value).to_string(), *value, *label);
        }
        return select.interact().map_err(|err| err.to_string());
    }

    wizard_step(prompt);
    for (index, (value, description)) in choices.iter().enumerate() {
        let marker = if index == default_index {
            accent("◆")
        } else {
            muted("◇")
        };
        let default_chip = if index == default_index {
            format!(" {}", muted("default"))
        } else {
            String::new()
        };
        println!(
            "{}   {} {}  {}{}",
            muted("│"),
            marker,
            command_style(&format!("{}", index + 1)),
            strong(value),
            default_chip
        );
        println!("{}       {}", muted("│"), muted(description));
    }
    loop {
        let input = prompt_line(&format!(
            "{} {} ",
            muted("╰─"),
            accent(&format!("Choose [default {}]:", default_index + 1))
        ))?;
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(choices[default_index].0.to_string());
        }
        if let Ok(number) = trimmed.parse::<usize>() {
            if (1..=choices.len()).contains(&number) {
                return Ok(choices[number - 1].0.to_string());
            }
        }
        if let Some((value, _)) = choices.iter().find(|(value, _)| *value == trimmed) {
            return Ok((*value).to_string());
        }
        println!(
            "{} {}",
            warn("!"),
            muted(&format!(
                "Please enter a number from 1 to {}.",
                choices.len()
            ))
        );
    }
}

fn prompt_select_labeled(
    prompt: &str,
    choices: &[(&str, &str, &str)],
    default_index: usize,
) -> Result<String, String> {
    if use_clack_prompts() {
        let mut select = cliclack::select(prompt.to_string())
            .initial_value(choices[default_index].0.to_string())
            .max_rows(10);
        for (value, label, hint) in choices {
            select = select.item((*value).to_string(), *label, *hint);
        }
        return select.interact().map_err(|err| err.to_string());
    }

    wizard_step(prompt);
    for (index, (value, label, hint)) in choices.iter().enumerate() {
        let marker = if index == default_index {
            accent("◆")
        } else {
            muted("◇")
        };
        let default_chip = if index == default_index {
            format!(" {}", muted("default"))
        } else {
            String::new()
        };
        println!(
            "{}   {} {}  {}{}",
            muted("│"),
            marker,
            command_style(&format!("{}", index + 1)),
            strong(label),
            default_chip
        );
        if !hint.is_empty() {
            println!("{}       {}", muted("│"), muted(hint));
        }
        println!("{}       {}", muted("│"), muted(&format!("value: {value}")));
    }
    loop {
        let input = prompt_line(&format!(
            "{} {} ",
            muted("╰─"),
            accent(&format!("Choose [default {}]:", default_index + 1))
        ))?;
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Ok(choices[default_index].0.to_string());
        }
        if let Ok(number) = trimmed.parse::<usize>() {
            if (1..=choices.len()).contains(&number) {
                return Ok(choices[number - 1].0.to_string());
            }
        }
        if let Some((value, _, _)) = choices
            .iter()
            .find(|(value, label, _)| *value == trimmed || label.eq_ignore_ascii_case(trimmed))
        {
            return Ok((*value).to_string());
        }
        println!(
            "{} {}",
            warn("!"),
            muted(&format!(
                "Please enter a number from 1 to {}.",
                choices.len()
            ))
        );
    }
}

fn prompt_text(prompt: &str, default: &str) -> Result<String, String> {
    if use_clack_prompts() {
        let mut input = cliclack::input(prompt.to_string()).required(default.is_empty());
        if !default.is_empty() {
            input = input.default_input(default);
        }
        return input.interact().map_err(|err| err.to_string());
    }

    wizard_step(prompt);
    let suffix = if default.is_empty() {
        ": ".to_string()
    } else {
        format!(" [{}]: ", muted(default))
    };
    let input = prompt_line(&format!("{} {}", muted("╰─"), accent(&suffix)))?;
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Ok(default.to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

fn prompt_command_token_text(prompt: &str, default: &str, field: &str) -> Result<String, String> {
    loop {
        let value = prompt_text(prompt, default)?;
        if command_token_safe(&value) {
            return Ok(value);
        }
        println!(
            "{} {}",
            warn("!"),
            muted(&format!(
                "{field} cannot be empty or contain whitespace/control characters in the current automation command path."
            ))
        );
    }
}

fn prompt_confirm(prompt: &str, default: bool) -> Result<bool, String> {
    if use_clack_prompts() {
        return cliclack::confirm(prompt.to_string())
            .initial_value(default)
            .interact()
            .map_err(|err| err.to_string());
    }

    wizard_step(prompt);
    let hint = if default { "Y/n" } else { "y/N" };
    loop {
        let input = prompt_line(&format!(
            "{} {} ",
            muted("╰─"),
            accent(&format!("[{}]:", hint))
        ))?;
        let trimmed = input.trim().to_ascii_lowercase();
        if trimmed.is_empty() {
            return Ok(default);
        }
        match trimmed.as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => println!("{} {}", warn("!"), muted("Please answer y or n.")),
        }
    }
}

fn prompt_password(prompt: &str) -> Result<String, String> {
    if use_clack_prompts() {
        return cliclack::password(prompt.to_string())
            .mask('▪')
            .interact()
            .map_err(|err| err.to_string());
    }
    println!(
        "{} {}",
        warn("!"),
        muted("Password entry is visible outside a TTY.")
    );
    prompt_text(prompt, "")
}

fn prompt_password_token(prompt: &str, field: &str) -> Result<String, String> {
    loop {
        let value = prompt_password(prompt)?;
        if command_token_safe(&value) {
            return Ok(value);
        }
        println!(
            "{} {}",
            warn("!"),
            muted(&format!(
                "{field} cannot be empty or contain whitespace/control characters in the current automation command path."
            ))
        );
    }
}

fn prompt_u16(prompt: &str, default: u16) -> Result<u16, String> {
    loop {
        let input = prompt_text(prompt, &default.to_string())?;
        if let Ok(value) = input.parse::<u16>() {
            if value > 0 {
                return Ok(value);
            }
        }
        println!("{} {}", warn("!"), muted("Please enter a valid TCP port."));
    }
}

fn prompt_line(prompt: &str) -> Result<String, String> {
    print!("{}", prompt);
    std::io::stdout().flush().map_err(|err| err.to_string())?;
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|err| err.to_string())?;
    Ok(input)
}

fn build_onboard_automation_command(options: &OnboardOptions) -> String {
    build_onboard_automation_command_inner(options, false)
}

fn validate_onboard_automation_args(options: &OnboardOptions) -> Result<(), String> {
    let mut invalid = Vec::new();
    push_if_shell_token_unsafe(
        &mut invalid,
        "workspace",
        options.workspace.as_deref().unwrap_or(".hepta/workspace"),
    );
    if let Some(flow) = options.flow.as_ref() {
        push_if_shell_token_unsafe(&mut invalid, "flow", flow);
    }
    push_if_shell_token_unsafe(
        &mut invalid,
        "secret-input-mode",
        &options.secret_input_mode,
    );
    push_if_shell_token_unsafe(&mut invalid, "gateway-bind", &options.gateway_bind);
    push_if_shell_token_unsafe(&mut invalid, "gateway-auth", &options.gateway_auth);
    if let Some(auth_choice) = options.auth_choice.as_ref() {
        push_if_shell_token_unsafe(&mut invalid, "auth-choice", auth_choice);
    }
    if let Some(token) = options.gateway_token.as_ref() {
        push_if_shell_token_unsafe(&mut invalid, "gateway-token", token);
    }
    if let Some(token_ref_env) = options.gateway_token_ref_env.as_ref() {
        push_if_shell_token_unsafe(&mut invalid, "gateway-token-ref-env", token_ref_env);
    }
    if let Some(reset_scope) = options.reset_scope.as_ref() {
        push_if_shell_token_unsafe(&mut invalid, "reset-scope", reset_scope);
    }
    push_if_shell_token_unsafe(&mut invalid, "daemon-runtime", &options.daemon_runtime);
    push_if_shell_token_unsafe(&mut invalid, "daemon-action", &options.daemon_action);
    push_if_shell_token_unsafe(&mut invalid, "node-manager", &options.node_manager);
    if let Some(remote_url) = options.remote_url.as_ref() {
        push_if_shell_token_unsafe(&mut invalid, "remote-url", remote_url);
    }
    if let Some(remote_token) = options.remote_token.as_ref() {
        push_if_shell_token_unsafe(&mut invalid, "remote-token", remote_token);
    }
    push_if_shell_token_unsafe(&mut invalid, "tailscale", &options.tailscale);
    if let Some(password) = options.gateway_password.as_ref() {
        push_if_shell_token_unsafe(&mut invalid, "gateway-password", password);
    }
    if let Some(import_from) = options.import_from.as_ref() {
        push_if_shell_token_unsafe(&mut invalid, "import-from", import_from);
    }
    if let Some(import_source) = options.import_source.as_ref() {
        push_if_shell_token_unsafe(&mut invalid, "import-source", import_source);
    }
    if invalid.is_empty() {
        return Ok(());
    }
    Err(format!(
        "onboarding values must be non-empty and must not contain whitespace/control characters for the current automation command path: {}",
        invalid.join(", ")
    ))
}

fn push_if_shell_token_unsafe(invalid: &mut Vec<&'static str>, field: &'static str, value: &str) {
    if !command_token_safe(value) {
        invalid.push(field);
    }
}

fn command_token_safe(value: &str) -> bool {
    !value.is_empty()
        && !value.chars().any(char::is_whitespace)
        && !value.chars().any(char::is_control)
}

fn command_token_default<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    if command_token_safe(value) {
        value
    } else {
        fallback
    }
}

fn build_onboard_automation_command_redacted(options: &OnboardOptions) -> String {
    build_onboard_automation_command_inner(options, true)
}

fn build_onboard_automation_command_inner(
    options: &OnboardOptions,
    redact_secrets: bool,
) -> String {
    let mut parts = vec![
        "onboard".to_string(),
        "--non-interactive".into(),
        "--accept-risk".into(),
        "--flow".into(),
        options.flow.clone().unwrap_or_else(|| "quickstart".into()),
        "--secret-input-mode".into(),
        options.secret_input_mode.clone(),
        "--mode".into(),
        options.mode.clone(),
        "--workspace".into(),
        options
            .workspace
            .clone()
            .unwrap_or_else(|| ".hepta/workspace".into()),
        "--gateway-port".into(),
        options
            .gateway_port
            .unwrap_or_else(|| parse_bind_port(&options.ui_bind).unwrap_or(7373))
            .to_string(),
        "--gateway-bind".into(),
        options.gateway_bind.clone(),
        "--gateway-auth".into(),
        options.gateway_auth.clone(),
        "--daemon-runtime".into(),
        options.daemon_runtime.clone(),
        "--daemon-action".into(),
        options.daemon_action.clone(),
        "--node-manager".into(),
        options.node_manager.clone(),
    ];
    if options.reset {
        parts.push("--reset".into());
    }
    if options.modern {
        parts.push("--modern".into());
    }
    if let Some(reset_scope) = options.reset_scope.as_ref() {
        parts.push("--reset-scope".into());
        parts.push(reset_scope.clone());
    }
    if let Some(auth_choice) = options.auth_choice.as_ref() {
        parts.push("--auth-choice".into());
        parts.push(auth_choice.clone());
    }
    if let Some(token) = options.gateway_token.as_ref() {
        parts.push("--gateway-token".into());
        parts.push(if redact_secrets {
            "<redacted>".into()
        } else {
            token.clone()
        });
    }
    if let Some(token_ref_env) = options.gateway_token_ref_env.as_ref() {
        parts.push("--gateway-token-ref-env".into());
        parts.push(token_ref_env.clone());
    }
    if let Some(remote_url) = options.remote_url.as_ref() {
        parts.push("--remote-url".into());
        parts.push(remote_url.clone());
    }
    if let Some(remote_token) = options.remote_token.as_ref() {
        parts.push("--remote-token".into());
        parts.push(if redact_secrets {
            "<redacted>".into()
        } else {
            remote_token.clone()
        });
    }
    if options.tailscale != "off" {
        parts.push("--tailscale".into());
        parts.push(options.tailscale.clone());
    }
    if options.tailscale_reset_on_exit {
        parts.push("--tailscale-reset-on-exit".into());
    }
    if options.install_daemon {
        parts.push("--install-daemon".into());
    } else {
        parts.push("--skip-daemon".into());
    }
    if options.skip_bootstrap {
        parts.push("--skip-bootstrap".into());
    }
    if options.skip_channels {
        parts.push("--skip-channels".into());
    }
    if options.skip_skills {
        parts.push("--skip-skills".into());
    }
    if options.skip_search {
        parts.push("--skip-search".into());
    }
    if options.skip_health {
        parts.push("--skip-health".into());
    }
    if options.skip_ui {
        parts.push("--skip-ui".into());
    }
    if let Some(password) = options.gateway_password.as_ref() {
        parts.push("--gateway-password".into());
        parts.push(if redact_secrets {
            "<redacted>".into()
        } else {
            password.clone()
        });
    }
    if let Some(import_from) = options.import_from.as_ref() {
        parts.push("--import-from".into());
        parts.push(import_from.clone());
    }
    if let Some(import_source) = options.import_source.as_ref() {
        parts.push("--import-source".into());
        parts.push(import_source.clone());
    }
    if options.import_secrets {
        parts.push("--import-secrets".into());
    }
    if options.dry_run {
        parts.push("--dry-run".into());
    }
    parts.join(" ")
}

fn parse_bind_port(bind: &str) -> Option<u16> {
    bind.rsplit_once(':')
        .and_then(|(_, port)| port.parse::<u16>().ok())
        .filter(|port| *port > 0)
}

fn print_wizard_hero() {
    println!();
    println!(
        "{}",
        accent("╭────────────────────────────────────────────────────────────╮")
    );
    println!(
        "{} {} {}",
        accent("│"),
        heading("Hepta Onboarding"),
        muted("                                            │")
    );
    println!(
        "{} {} {}",
        accent("│"),
        muted("HeptaRuntime-parity local setup, with Hepta safety guardrails"),
        accent("│")
    );
    println!(
        "{}",
        accent("╰────────────────────────────────────────────────────────────╯")
    );
    println!(
        "{} {}",
        muted("  ↳"),
        muted("Enter accepts defaults; choose by number; Ctrl-C cancels cleanly.")
    );
    println!();
}

fn wizard_step(title: &str) {
    let step = WIZARD_STEP_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    println!();
    println!(
        "{} {} {}",
        muted("╭─"),
        accent(&format!("Step {step:02}")),
        muted("────────────────────────────────────────")
    );
    println!("{} {}", muted("│"), strong(title));
}

fn print_smart_preflight(cli: &CliApp) {
    let ops = cli.ops_status_report();
    let default_port = ops
        .live_url
        .rsplit_once(':')
        .and_then(|(_, port)| port.parse::<u16>().ok())
        .unwrap_or(7373);
    let port_state = loopback_port_state(default_port);
    let next_free_port = next_free_loopback_port(default_port).unwrap_or(default_port);
    let recommendation = if !ops.installed_binary_executable {
        "Recommendation: install/update the local hepta binary first."
    } else if !ops.service_plist_present {
        "Recommendation: choose gateway service install if you want Hepta to stay alive after Terminal closes."
    } else if !ops.service_loaded || !ops.service_live_ok {
        "Recommendation: keep the health gate enabled; the gateway service should be loaded and live."
    } else {
        "Recommendation: current local install looks healthy; quickstart defaults are safe."
    };
    let port_recommendation = match port_state {
        PortState::Free => format!("Port: {default_port} is free for a new Control UI bind."),
        PortState::Open => format!(
            "Port: {default_port} is already open; keep it if this is the live Hepta/HeptaRuntime-local UI, otherwise use {next_free_port}."
        ),
        PortState::Blocked => format!(
            "Port: {default_port} is blocked/unavailable; suggested fallback is {next_free_port}."
        ),
    };
    let summary = format!(
        "Status: {}\nInstalled binary: {}\nControl UI: {} screens / {} bindings\nGateway service: plist={} loaded={} live_ok={}\nWatchdog evidence: ok={} fresh={} matches_surface={}\n{}\n{}",
        ops.status,
        if ops.installed_binary_executable {
            "ready"
        } else {
            "missing"
        },
        ops.control_ui_screen_count,
        ops.control_ui_command_binding_count,
        ops.service_plist_present,
        ops.service_loaded,
        ops.service_live_ok,
        ops.watchdog_ok,
        ops.watchdog_fresh,
        ops.watchdog_matches_current_surface,
        port_recommendation,
        recommendation
    );
    if use_clack_prompts() {
        let _ = cliclack::note("Smart preflight", summary);
        return;
    }
    println!(
        "{}",
        accent("╭─ Smart preflight ───────────────────────────────────────╮")
    );
    for line in summary.lines() {
        println!("{} {}", accent("│"), muted(line));
    }
    println!(
        "{}",
        accent("╰─────────────────────────────────────────────────────────╯")
    );
}

fn print_profile_panel(profile: &str, options: &OnboardOptions, reason: &str) {
    let explanation = match profile {
        "developer" => {
            "dry-run first, no daemon by default, source-first diagnostics remain visible"
        }
        "persistent-operator" => {
            "LaunchAgent gateway enabled, live health gate kept on, loopback exposure retained"
        }
        "migration" => {
            "HeptaRuntime metadata import prepared, secret copying left off unless explicitly changed"
        }
        "custom" => {
            "no opinionated defaults applied; the wizard will ask the detailed setup questions"
        }
        "customized" => "selected defaults with targeted operator deviations applied",
        _ => "loopback-only, token auth, no daemon, health-gated local setup",
    };
    let summary = format!(
        "Profile: {profile}\nWhy: {reason}\n{}\nWorkspace: {}\nGateway: http://{}\nDaemon: {}\nImport: {}\nDry run: {}",
        explanation,
        options.workspace.as_deref().unwrap_or(".hepta/workspace"),
        options.ui_bind,
        if options.install_daemon { "yes" } else { "no" },
        options.import_from.as_deref().unwrap_or("none"),
        if options.dry_run { "yes" } else { "no" },
    );
    if use_clack_prompts() {
        let _ = cliclack::note("Adaptive defaults", summary);
        return;
    }

    println!(
        "{}",
        accent("╭─ Adaptive defaults ───────────────────────────────────╮")
    );
    for line in summary.lines() {
        println!("{} {}", accent("│"), muted(line));
    }
    println!(
        "{}",
        accent("╰─────────────────────────────────────────────────────────╯")
    );
}

fn print_review_panel(options: &OnboardOptions) {
    let (risk_level, risk_detail, _) = onboard_risk_summary(options);
    let planned_command = build_onboard_automation_command_redacted(options);
    let tailscale_line = if options.tailscale == "off" {
        "Tailscale: off".to_string()
    } else {
        format!("Tailscale: {}", options.tailscale)
    };
    if use_clack_prompts() {
        let review = format!(
            "Mode: {}\nWorkspace: {}\nGateway: http://{}\nAuth: {}\n{}\nDaemon: {} (action={}, runtime={})\nHealth gate: {}\nBootstrap / skills: {} / {}\nImport: {}\nDry run: {}\nRisk: {} — {}\nCommand: {}",
            options.mode,
            options.workspace.as_deref().unwrap_or(".hepta/workspace"),
            options.ui_bind,
            options.gateway_auth,
            tailscale_line,
            if options.install_daemon { "yes" } else { "no" },
            options.daemon_action,
            options.daemon_runtime,
            if options.skip_health { "no" } else { "yes" },
            if options.skip_bootstrap { "no" } else { "yes" },
            if options.skip_skills { "no" } else { "yes" },
            options.import_from.as_deref().unwrap_or("none"),
            if options.dry_run { "yes" } else { "no" },
            risk_level,
            risk_detail,
            planned_command,
        );
        let _ = cliclack::note("Review", review);
        return;
    }

    println!();
    println!(
        "{}",
        accent("╭─ Review ───────────────────────────────────────────────╮")
    );
    println!(
        "{} {:<18} {}",
        accent("│"),
        muted("Mode"),
        strong(&options.mode)
    );
    println!(
        "{} {:<18} {}",
        accent("│"),
        muted("Workspace"),
        command_style(options.workspace.as_deref().unwrap_or(".hepta/workspace"))
    );
    println!(
        "{} {:<18} {}",
        accent("│"),
        muted("Gateway"),
        command_style(&format!("http://{}", options.ui_bind))
    );
    println!(
        "{} {:<18} {}",
        accent("│"),
        muted("Auth"),
        strong(&options.gateway_auth)
    );
    println!(
        "{} {:<18} {}",
        accent("│"),
        muted("Tailscale"),
        strong(&options.tailscale)
    );
    println!(
        "{} {:<18} {} (action={}, runtime={})",
        accent("│"),
        muted("Daemon"),
        bool_badge(options.install_daemon),
        options.daemon_action,
        options.daemon_runtime
    );
    println!(
        "{} {:<18} {}",
        accent("│"),
        muted("Health gate"),
        bool_badge(!options.skip_health)
    );
    println!(
        "{} {:<18} {} / {}",
        accent("│"),
        muted("Bootstrap / skills"),
        bool_badge(!options.skip_bootstrap),
        bool_badge(!options.skip_skills)
    );
    println!(
        "{} {:<18} {}",
        accent("│"),
        muted("Import"),
        options.import_from.as_deref().unwrap_or("none")
    );
    println!(
        "{} {:<18} {}",
        accent("│"),
        muted("Dry run"),
        bool_badge(options.dry_run)
    );
    println!(
        "{} {:<18} {} — {}",
        accent("│"),
        muted("Risk"),
        strong(risk_level),
        muted(risk_detail)
    );
    println!(
        "{} {:<18} {}",
        accent("│"),
        muted("Command"),
        command_style(&planned_command)
    );
    println!(
        "{}",
        accent("╰─────────────────────────────────────────────────────────╯")
    );
}

fn print_hepta_runtime_control_ui_panel(cli: &CliApp, options: &OnboardOptions) {
    let ops = cli.ops_status_report();
    let http_url = format!("http://{}", options.ui_bind);
    let ws_url = format!("ws://{}", options.ui_bind);
    let gateway_status = if ops.service_loaded && ops.service_live_ok {
        "Gateway: reachable".to_string()
    } else {
        "Gateway: not detected".to_string()
    };
    let body = format!(
        "Web UI: {http_url}\nGateway WS: {ws_url}\n{gateway_status}\nDocs: https://docs.hepta_runtime.ai/web/control-ui"
    );
    if use_clack_prompts() {
        let _ = cliclack::note("Control UI", body);
        return;
    }
    println!(
        "{}",
        accent("╭─ Control UI ───────────────────────────────────────────╮")
    );
    for line in body.lines() {
        println!("{} {}", accent("│"), muted(line));
    }
    println!(
        "{}",
        accent("╰─────────────────────────────────────────────────────────╯")
    );
}

fn print_hepta_runtime_optional_apps_note() {
    let optional_apps = "Add nodes for extra features:\n- macOS app (system + notifications)\n- iOS app (camera/canvas)\n- Android app (camera/canvas)";

    if use_clack_prompts() {
        let _ = cliclack::note("Optional apps", optional_apps);
        return;
    }

    println!(
        "{}",
        accent("╭─ Optional apps ─────────────────────────────────────────╮")
    );
    for line in optional_apps.lines() {
        println!("{} {}", accent("│"), muted(line));
    }
    println!(
        "{}",
        accent("╰─────────────────────────────────────────────────────────╯")
    );
}

fn prompt_hepta_runtime_hatch_page(cli: &CliApp, options: &OnboardOptions) -> Result<(), String> {
    if options.skip_ui {
        if use_clack_prompts() {
            let _ = cliclack::note("Control UI", "Skipping Control UI/TUI prompts.");
        } else {
            println!(
                "{} {}",
                muted("↳"),
                muted("Skipping Control UI/TUI prompts.")
            );
        }
        return Ok(());
    }

    let ops = cli.ops_status_report();
    let gateway_reachable = ops.service_live_ok
        || parse_bind_port(&options.ui_bind)
            .map(|port| matches!(loopback_port_state(port), PortState::Open))
            .unwrap_or(false);
    let http_url = format!("http://{}", options.ui_bind);

    if !options.skip_bootstrap {
        let body = [
            "This is the defining action that makes your agent you.",
            "Please take your time.",
            "The more you tell it, the better the experience will be.",
            "We will send: \"Wake up, my friend!\"",
        ]
        .join("\n");
        if use_clack_prompts() {
            let _ = cliclack::note("Start TUI (best option!)", body);
        } else {
            println!(
                "{}",
                accent("╭─ Start TUI (best option!) ─────────────────────────────╮")
            );
            for line in body.lines() {
                println!("{} {}", accent("│"), muted(line));
            }
            println!(
                "{}",
                accent("╰─────────────────────────────────────────────────────────╯")
            );
        }
    }

    if options.gateway_auth == "token" && gateway_reachable {
        let token_source = if let Some(env_name) = options.gateway_token_ref_env.as_ref() {
            format!("Stored as env-backed SecretRef: {env_name}")
        } else if options.gateway_token.is_some() {
            "Provided on the command line; value redacted in review/output.".into()
        } else {
            "Generated/stored at .hepta/onboard/gateway-token; value redacted.".into()
        };
        let token_body = [
            "Gateway token: shared auth for the Gateway + Control UI.".to_string(),
            token_source,
            "Web UI keeps dashboard URL tokens in memory for the current tab and strips them from the URL after load.".into(),
            "If prompted: paste the token into Control UI settings (or use the tokenized dashboard URL).".into(),
        ]
        .join("\n");
        if use_clack_prompts() {
            let _ = cliclack::note("Token", token_body);
        } else {
            println!(
                "{}",
                accent("╭─ Token ─────────────────────────────────────────────────╮")
            );
            for line in token_body.lines() {
                println!("{} {}", accent("│"), muted(line));
            }
            println!(
                "{}",
                accent("╰─────────────────────────────────────────────────────────╯")
            );
        }
    }

    let mut choices = vec![(
        "tui",
        "Hatch in Terminal (recommended)",
        "Show the terminal handoff prompt",
    )];
    if gateway_reachable {
        choices.push(("web", "Open the Web UI", "Show the dashboard link"));
    }
    choices.push(("later", "Do this later", "Keep the setup and open UI later"));

    let hatch_choice = prompt_select_labeled("How do you want to hatch your bot?", &choices, 0)?;
    match hatch_choice.as_str() {
        "web" => {
            let body = format!(
                "Dashboard link: {http_url}\nCopy/paste this URL in a browser on this machine to control Hepta."
            );
            if use_clack_prompts() {
                let _ = cliclack::note("Dashboard ready", body);
            } else {
                println!("{} {}", success("✓"), body.replace('\n', "\n  "));
            }
        }
        "later" => {
            if use_clack_prompts() {
                let _ =
                    cliclack::note("Later", "Use the Control UI link above when you are ready.");
            } else {
                println!(
                    "{} {}",
                    muted("↳"),
                    muted("Use the Control UI link above when you are ready.")
                );
            }
        }
        _ => {
            let body = if options.skip_bootstrap {
                "Terminal hatch selected. Hepta keeps this as a source-first handoff; continue in this terminal or use the Control UI link above."
            } else {
                "Terminal hatch selected. Hepta source-first onboarding keeps the bootstrap handoff local; continue in this terminal or use the Control UI link above."
            };
            if use_clack_prompts() {
                let _ = cliclack::note("Terminal hatch", body);
            } else {
                println!("{} {}", success("✓"), body);
            }
        }
    }

    Ok(())
}

fn print_hepta_runtime_finalize_followups(options: &OnboardOptions) {
    let workspace_backup = "Back up your agent workspace.\nDocs: https://docs.hepta_runtime.ai/concepts/agent-workspace";
    let security = "Running agents on your computer is risky — harden your setup: https://docs.hepta_runtime.ai/security";
    let shell_completion = "Shell completion was not auto-mutated by Hepta onboard. If Hepta adds a completion installer later, run it from a trusted shell.";
    let web_search = if options.skip_search {
        "Web search was skipped. You can enable/configure provider bridges later with redacted provider smoke commands."
    } else {
        "Web search setup remains metadata-safe in Hepta onboard; provider keys stay redacted and runtime searches are not triggered during onboarding."
    };
    let codex_native = "Codex native search, when present in imported HeptaRuntime metadata, is only for Codex-capable models.";
    let what_now = "What now: https://hepta_runtime.ai/showcase (\"What People Are Building\").";

    if use_clack_prompts() {
        let _ = cliclack::note("Workspace backup", workspace_backup);
        let _ = cliclack::note("Security", security);
        let _ = cliclack::note("Shell completion", shell_completion);
        let _ = cliclack::note("Web search", web_search);
        let _ = cliclack::note("Codex native search", codex_native);
        let _ = cliclack::note("What now", what_now);
        return;
    }

    println!("{} {}", muted("↳"), muted(workspace_backup));
    println!("{} {}", muted("↳"), muted(security));
    println!("{} {}", muted("↳"), muted(shell_completion));
    println!("{} {}", muted("↳"), muted(web_search));
    println!("{} {}", muted("↳"), muted(codex_native));
    println!("{} {}", muted("↳"), muted(what_now));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PortState {
    Free,
    Open,
    Blocked,
}

fn loopback_port_state(port: u16) -> PortState {
    if TcpListener::bind(("127.0.0.1", port)).is_ok() {
        return PortState::Free;
    }
    if TcpStream::connect(("127.0.0.1", port)).is_ok() {
        return PortState::Open;
    }
    PortState::Blocked
}

fn next_free_loopback_port(start: u16) -> Option<u16> {
    let end = start.saturating_add(25).max(start);
    (start..=end).find(|port| matches!(loopback_port_state(*port), PortState::Free))
}

fn onboard_risk_summary(options: &OnboardOptions) -> (&'static str, &'static str, bool) {
    if options.gateway_bind == "lan" && options.import_secrets {
        return (
            "high",
            "LAN exposure plus secret import requires explicit acknowledgement",
            true,
        );
    }
    if options.tailscale == "funnel" && options.import_secrets {
        return (
            "high",
            "Tailscale Funnel exposure plus secret import requires explicit acknowledgement",
            true,
        );
    }
    if options.gateway_bind == "lan" {
        return (
            "elevated",
            "LAN exposure can make the local operator surface reachable on the network",
            true,
        );
    }
    if options.tailscale == "funnel" {
        return (
            "elevated",
            "Tailscale Funnel can expose the local operator surface beyond this machine",
            true,
        );
    }
    if options.tailscale == "serve" {
        return (
            "elevated",
            "Tailscale Serve can expose the local operator surface to your tailnet",
            true,
        );
    }
    if options.import_secrets {
        return (
            "elevated",
            "secret import is intentionally not the default path",
            true,
        );
    }
    if options.install_daemon && options.skip_health {
        return (
            "medium",
            "daemon install without post-install health gate",
            true,
        );
    }
    (
        "low",
        "loopback/local-first setup with review-before-write guardrails",
        false,
    )
}

fn bool_badge(value: bool) -> String {
    if value { success("yes") } else { muted("no") }
}

fn use_clack_prompts() -> bool {
    std::io::stdin().is_terminal()
        && std::io::stderr().is_terminal()
        && env::var("HEPTA_PLAIN_WIZARD")
            .map(|value| value != "1" && !value.eq_ignore_ascii_case("true"))
            .unwrap_or(true)
}

fn rich_terminal() -> bool {
    let force_color = env::var("FORCE_COLOR")
        .map(|value| !value.trim().is_empty() && value.trim() != "0")
        .unwrap_or(false);
    force_color
        || (std::io::stdout().is_terminal()
            && env::var("NO_COLOR").is_err()
            && env::var("TERM").map(|term| term != "dumb").unwrap_or(true))
}

fn color(code: &str, value: &str) -> String {
    if rich_terminal() {
        format!("\x1b[{code}m{value}\x1b[0m")
    } else {
        value.to_string()
    }
}

fn accent(value: &str) -> String {
    color("38;2;255;90;45", value)
}

fn success(value: &str) -> String {
    color("38;2;47;191;113", value)
}

fn warn(value: &str) -> String {
    color("38;2;255;176;32", value)
}

fn muted(value: &str) -> String {
    color("38;2;139;127;119", value)
}

fn strong(value: &str) -> String {
    color("1", value)
}

fn heading(value: &str) -> String {
    color("1;38;2;255;90;45", value)
}

fn command_style(value: &str) -> String {
    color("38;2;255;122;61", value)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GatewayServeOptions {
    with_telegram_plugin: bool,
    telegram_plugin_poll_ms: u64,
}

impl GatewayServeOptions {
    fn from_args(args: &[String]) -> Result<Self, String> {
        let mut options = Self {
            with_telegram_plugin: gateway_env_truthy("HEPTA_GATEWAY_ENABLE_TELEGRAM_PLUGIN"),
            telegram_plugin_poll_ms: gateway_env_u64("HEPTA_GATEWAY_TELEGRAM_POLL_MS")
                .unwrap_or(1500),
        };
        let mut index = 0usize;
        while index < args.len() {
            match args[index].as_str() {
                "--with-telegram-plugin" | "--gateway-owned-telegram-plugin" => {
                    options.with_telegram_plugin = true;
                }
                "--without-telegram-plugin" | "--no-telegram-plugin" => {
                    options.with_telegram_plugin = false;
                }
                "--telegram-plugin-poll-ms" => {
                    index += 1;
                    let value = args.get(index).ok_or_else(|| {
                        "--telegram-plugin-poll-ms requires milliseconds".to_string()
                    })?;
                    options.telegram_plugin_poll_ms = value.parse::<u64>().map_err(|_| {
                        "--telegram-plugin-poll-ms requires a positive integer".to_string()
                    })?;
                    if options.telegram_plugin_poll_ms == 0 {
                        return Err("--telegram-plugin-poll-ms requires a positive integer".into());
                    }
                }
                other => {
                    return Err(format!("unexpected --serve-ui argument: {other}"));
                }
            }
            index += 1;
        }
        options.telegram_plugin_poll_ms = options.telegram_plugin_poll_ms.clamp(500, 60_000);
        Ok(options)
    }
}

fn gateway_env_truthy(name: &str) -> bool {
    env::var(name)
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn gateway_env_u64(name: &str) -> Option<u64> {
    env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
}

fn start_gateway_owned_telegram_plugin(poll_ms: u64) -> Option<thread::JoinHandle<()>> {
    match thread::Builder::new()
        .name("hepta-telegram-plugin".into())
        .spawn(move || {
            let plugin_cli = CliApp::new();
            if Path::new(DEFAULT_SNAPSHOT_PATH).exists() {
                if let Err(err) = plugin_cli.load_snapshot(DEFAULT_SNAPSHOT_PATH) {
                    eprintln!("gateway-owned telegram plugin autoload warning: {err}");
                }
            }

            let runtime = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(runtime) => runtime,
                Err(err) => {
                    eprintln!("gateway-owned telegram plugin runtime failed: {err}");
                    return;
                }
            };
            let command = format!(
                "/telegram-plugin --serve-reply-loop --daemon --confirm-send --use-message-text --poll-ms {poll_ms} --json"
            );
            match runtime.block_on(plugin_cli.execute_command(&command)) {
                Ok(output) => {
                    eprintln!("gateway-owned telegram plugin exited: {}", output.trim());
                }
                Err(err) => {
                    eprintln!("gateway-owned telegram plugin failed: {err}");
                }
            }
        }) {
        Ok(handle) => {
            eprintln!(
                "Hepta gateway hosting Telegram plugin reply loop in-process (poll_ms={poll_ms})."
            );
            Some(handle)
        }
        Err(err) => {
            eprintln!("failed to start gateway-owned Telegram plugin thread: {err}");
            None
        }
    }
}

async fn serve_control_ui(cli: &CliApp, bind_addr: &str) -> Result<(), String> {
    let allow_non_loopback = env::var("HEPTA_ALLOW_NON_LOOPBACK_UI")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if !is_loopback_bind_addr(bind_addr) && !allow_non_loopback {
        return Err(format!(
            "refusing to serve UI on non-loopback address {bind_addr}; set HEPTA_ALLOW_NON_LOOPBACK_UI=1 only for an explicit local lab exposure"
        ));
    }
    let listener = TcpListener::bind(bind_addr).map_err(|err| err.to_string())?;
    println!("Hepta Control UI listening on http://{}/", bind_addr);
    println!("Press Ctrl-C to stop.");

    for stream in listener.incoming() {
        let mut stream = stream.map_err(|err| err.to_string())?;
        let request = read_http_request(&mut stream)?;
        let (method, path) = request_method_and_path(&request).unwrap_or(("GET", "/"));
        if method == "GET" && path == "/assets/hepta-agent-logo.png" {
            write_http_response(
                &mut stream,
                "200 OK",
                "image/png",
                hepta_core::CONTROL_UI_HEPTA_AGENT_LOGO_PNG,
            )?;
            continue;
        }
        let body_text = request_body(&request);
        let (status, content_type, body) =
            route_control_ui_request(cli, method, path, body_text).await;
        write_http_response(&mut stream, status, content_type, body.as_bytes())?;
    }
    Ok(())
}

fn is_loopback_bind_addr(bind_addr: &str) -> bool {
    let host = bind_addr
        .rsplit_once(':')
        .map(|(host, _)| host.trim_matches(['[', ']']))
        .unwrap_or(bind_addr)
        .trim();
    matches!(host, "127.0.0.1" | "localhost" | "::1")
}

fn read_http_request(stream: &mut TcpStream) -> Result<String, String> {
    let mut buffer = [0_u8; 8192];
    let mut bytes = Vec::new();
    let first_read = stream.read(&mut buffer).map_err(|err| err.to_string())?;
    bytes.extend_from_slice(&buffer[..first_read]);
    let content_length = String::from_utf8_lossy(&bytes)
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            if name.eq_ignore_ascii_case("content-length") {
                value.trim().parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or(0);
    while request_body_bytes(&bytes).len() < content_length {
        let read = stream.read(&mut buffer).map_err(|err| err.to_string())?;
        if read == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..read]);
    }
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

fn request_body_bytes(bytes: &[u8]) -> &[u8] {
    bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| &bytes[index + 4..])
        .unwrap_or(&[])
}

fn request_method_and_path(request: &str) -> Option<(&str, &str)> {
    let first_line = request.lines().next()?;
    let mut parts = first_line.split_whitespace();
    let method = parts.next()?;
    let raw_path = parts.next()?;
    Some((method, raw_path.split('?').next().unwrap_or(raw_path)))
}

fn request_body(request: &str) -> &str {
    request
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .or_else(|| request.split_once("\n\n").map(|(_, body)| body))
        .unwrap_or("")
}

async fn route_control_ui_request(
    cli: &CliApp,
    method: &str,
    path: &str,
    body: &str,
) -> (&'static str, &'static str, String) {
    if method == "POST" {
        return route_control_ui_action_request(cli, path, body).await;
    }
    if method != "GET" {
        return (
            "405 Method Not Allowed",
            "text/plain; charset=utf-8",
            "method not allowed".into(),
        );
    }

    match path {
        "/" | "/index.html" => (
            "200 OK",
            "text/html; charset=utf-8",
            hepta_core::control_ui_index_html(),
        ),
        "/styles.css" => (
            "200 OK",
            "text/css; charset=utf-8",
            hepta_core::CONTROL_UI_STYLES_CSS.into(),
        ),
        "/README.md" => (
            "200 OK",
            "text/markdown; charset=utf-8",
            hepta_core::CONTROL_UI_README.into(),
        ),
        "/api/control-ui" => command_json(cli, "/control-ui --json").await,
        "/api/ui-contract-audit" | "/api/control-ui-audit" => {
            command_json(cli, "/ui-contract-audit --json").await
        }
        "/api/operator-snapshot" => command_json(cli, "/operator-snapshot --json").await,
        "/api/operator-security" => command_json(cli, "/operator-security --json").await,
        "/api/agent-advantage" | "/api/competitive-advantage" => {
            command_json(cli, "/agent-advantage --json").await
        }
        "/api/external-agent-benchmark" | "/api/external-benchmark" => {
            command_json(cli, "/external-agent-benchmark --json").await
        }
        "/api/ui-action-plan/gateway-dispatch" => {
            command_json(cli, "/ui-action-plan gateway-dispatch --dry-run --json").await
        }
        "/api/ops-status" | "/api/ops" => command_json(cli, "/ops-status --json").await,
        "/api/service-topology" | "/api/hepta-service-topology" => {
            command_json(cli, "/service-topology --json").await
        }
        "/api/codex-source-intake" | "/api/codex-hepta-delta" => {
            command_json(cli, "/codex-source-intake --json").await
        }
        "/api/codex-exec-policy-delta" | "/api/codex-exec-delta" => {
            command_json(cli, "/codex-exec-policy-delta --json").await
        }
        "/api/codex-fork-acceleration-plan" | "/api/codex-fast-path" => {
            command_json(cli, "/codex-fork-acceleration-plan --json").await
        }
        "/api/codex-source-mirror-preflight" | "/api/codex-mirror-preflight" => {
            command_json(cli, "/codex-source-mirror-preflight --json").await
        }
        "/api/providers" => command_json(cli, "/providers --json").await,
        "/api/image-models" | "/api/image-generation-models" => {
            command_json(cli, "/image-models --json").await
        }
        "/api/optional-configs" | "/api/config-catalog" => {
            command_json(cli, "/optional-configs --json").await
        }
        "/api/local-import" => command_json(cli, "/local-import --json").await,
        "/api/config" | "/api/config-surface" => command_json(cli, "/config-surface --json").await,
        "/api/native-capabilities" => command_json(cli, "/native-capabilities --json").await,
        "/api/external-readiness" => command_json(cli, "/external-readiness --json").await,
        "/api/production-surface" => command_json(cli, "/production-surface --json").await,
        "/api/production-parity" | "/api/parity" => {
            command_json(cli, "/production-parity --json").await
        }
        "/api/sessions" => command_json(cli, "/sessions --json").await,
        "/api/session-activity" => command_json(cli, "/session-activity --json").await,
        "/api/transcript" => command_json(cli, "/transcript --limit 12 --json").await,
        "/api/tasks" => command_json(cli, "/tasks --json").await,
        "/api/workers" => command_json(cli, "/workers --json").await,
        "/api/task-supervisor" => command_json(cli, "/task-supervisor --json").await,
        "/api/operator-console" | "/api/live-console" => {
            command_json(cli, "/operator-console --json").await
        }
        "/api/approvals" => command_json(cli, "/approvals --json").await,
        "/api/policy" => command_json(cli, "/policy --json").await,
        "/api/subagent-observatory" => command_json(cli, "/subagent-observatory --json").await,
        "/api/events" => command_json(cli, "/events --json").await,
        "/api/live-events" => live_events_json(cli, 0).await,
        "/api/events-report" | "/api/live-stream" => {
            command_json(cli, "/events-report --json").await
        }
        "/api/activity" => command_json(cli, "/activity --json").await,
        "/api/gateway-runtime" => command_json(cli, "/gateway-runtime --json").await,
        "/api/cron" => command_json(cli, "/runtime-control-plane --cron --sample-run --json").await,
        "/api/plugin-migration-audit" | "/api/hepta_runtime-plugin-audit" => {
            command_json(cli, "/plugin-migration-audit --json").await
        }
        "/api/diagnostics-prometheus" | "/api/prometheus-diagnostics" => {
            command_json(cli, "/diagnostics-prometheus --sample-run --json").await
        }
        "/api/tts-local-cli" | "/api/local-cli-tts" => {
            command_json(cli, "/tts-local-cli --sample-run --json").await
        }
        "/api/gateway-dispatch" => command_json(cli, "/gateway-dispatch --dry-run --json").await,
        "/api/gateway-ledger" => command_json(cli, "/gateway-ledger --json").await,
        "/api/gateway-retry-dead-letter" => {
            command_json(cli, "/gateway-retry-dead-letter --json").await
        }
        "/api/telegram-adapter" | "/api/telegram-runtime" => {
            command_json(cli, "/telegram-adapter --dry-run --json").await
        }
        "/api/telegram-plugin" => command_json(cli, "/telegram-plugin --dry-run --json").await,
        "/api/imessage-adapter" | "/api/imessage-runtime" => {
            command_json(cli, "/imessage-adapter --dry-run --local-probe --json").await
        }
        "/api/discord-adapter" | "/api/discord-runtime" => {
            command_json(cli, "/discord-adapter --dry-run --live-probe --json").await
        }
        "/api/feishu-adapter" | "/api/feishu-runtime" => {
            command_json(cli, "/feishu-adapter --dry-run --live-probe --json").await
        }
        "/api/multi-agent-runtime" => {
            command_json(cli, "/multi-agent-runtime --agents 4 --messages 8 --json").await
        }
        "/api/runtime/operator" => runtime_operator_json(cli, "{}").await,
        "/api/handoff-bundle" => (
            "400 Bad Request",
            "application/json; charset=utf-8",
            "{\"error\":\"task id required; use /api/handoff-bundle/<task_id>\"}".into(),
        ),
        "/api/doctor" => command_json(cli, "/doctor --json").await,
        _ if path.starts_with("/api/ui-action-plan/") => {
            let action = path.trim_start_matches("/api/ui-action-plan/");
            command_json(cli, &format!("/ui-action-plan {} --dry-run --json", action)).await
        }
        _ if path.starts_with("/api/query-transcript/") => {
            let query = path.trim_start_matches("/api/query-transcript/");
            command_json(
                cli,
                &format!("/query-transcript {} --limit 12 --json", query),
            )
            .await
        }
        _ if path.starts_with("/api/live-events/") => {
            let raw_since = path.trim_start_matches("/api/live-events/");
            match raw_since.parse::<u64>() {
                Ok(since) => live_events_json(cli, since).await,
                Err(_) => (
                    "400 Bad Request",
                    "application/json; charset=utf-8",
                    format!(
                        "{{\"error\":{}}}",
                        json_string("invalid live-events cursor")
                    ),
                ),
            }
        }
        _ if path.starts_with("/api/task-patches/") => {
            let task_id = path.trim_start_matches("/api/task-patches/");
            raw_internal_task_json(
                cli,
                &format!("/task-patches {} --json", task_id),
                path,
                "/api/task-patches",
            )
            .await
        }
        _ if path.starts_with("/api/task-loop/") => {
            let task_id = path.trim_start_matches("/api/task-loop/");
            raw_internal_task_json(
                cli,
                &format!("/task-loop {} --json", task_id),
                path,
                "/api/task",
            )
            .await
        }
        _ if path.starts_with("/api/task-evidence/") => {
            let task_id = path.trim_start_matches("/api/task-evidence/");
            raw_internal_task_json(
                cli,
                &format!("/task-evidence {} --json", task_id),
                path,
                "/api/task-evidence",
            )
            .await
        }
        _ if path.starts_with("/api/task-replay/") => {
            let task_id = path.trim_start_matches("/api/task-replay/");
            raw_internal_task_json(
                cli,
                &format!("/task-replay {} --json", task_id),
                path,
                "/api/task-replay",
            )
            .await
        }
        _ if path.starts_with("/api/promotion-ledger/") => {
            let task_id = path.trim_start_matches("/api/promotion-ledger/");
            raw_internal_task_json(
                cli,
                &format!("/promotion-ledger {} --json", task_id),
                path,
                "/api/promotion-ledger",
            )
            .await
        }
        _ if path.starts_with("/api/handoff-bundle/") => {
            let task_id = path.trim_start_matches("/api/handoff-bundle/");
            raw_internal_task_json(
                cli,
                &format!("/handoff-bundle {} --json", task_id),
                path,
                "/api/handoff-bundle",
            )
            .await
        }
        _ if path.starts_with("/api/task/") => {
            let task_id = path.trim_start_matches("/api/task/");
            raw_internal_task_json(cli, &format!("/task {} --json", task_id), path, "/api/task")
                .await
        }
        _ => (
            "404 Not Found",
            "text/plain; charset=utf-8",
            "not found".into(),
        ),
    }
}

async fn live_events_json(
    cli: &CliApp,
    since_unix_ms: u64,
) -> (&'static str, &'static str, String) {
    match cli.query_events_report(50, None, None) {
        Ok(report) => {
            let events = report
                .events
                .into_iter()
                .filter(|record| record.emitted_at_unix_ms > since_unix_ms)
                .collect::<Vec<_>>();
            let next_cursor_unix_ms = events
                .iter()
                .map(|record| record.emitted_at_unix_ms)
                .max()
                .unwrap_or(since_unix_ms);
            let body = serde_json::json!({
                "product": "Hepta",
                "status": "complete",
                "since_unix_ms": since_unix_ms,
                "next_cursor_unix_ms": next_cursor_unix_ms,
                "returned_count": events.len(),
                "duplicate_free": true,
                "cursor_monotonic": next_cursor_unix_ms >= since_unix_ms,
                "events": events,
            })
            .to_string();
            ("200 OK", "application/json; charset=utf-8", body)
        }
        Err(err) => (
            "500 Internal Server Error",
            "application/json; charset=utf-8",
            format!("{{\"error\":{}}}", json_string(&err)),
        ),
    }
}

async fn raw_internal_task_json(
    cli: &CliApp,
    command: &str,
    raw_path: &str,
    scoped_path: &str,
) -> (&'static str, &'static str, String) {
    match cli.execute_command(command).await {
        Ok(output) => {
            let mut value = serde_json::from_str::<serde_json::Value>(&output)
                .unwrap_or_else(|_| serde_json::json!({ "raw_output": output }));
            if let Some(object) = value.as_object_mut() {
                object.insert("raw_internal".into(), serde_json::Value::Bool(true));
                object.insert(
                    "raw_internal_endpoint".into(),
                    serde_json::Value::String(raw_path.into()),
                );
                object.insert(
                    "preferred_scoped_endpoint".into(),
                    serde_json::Value::String(scoped_path.into()),
                );
                object.insert("workspace_guarded".into(), serde_json::Value::Bool(false));
                object.insert(
                    "ui_preferred_path".into(),
                    serde_json::Value::String("scoped-post".into()),
                );
                object.insert(
                    "access_contract".into(),
                    serde_json::Value::String(
                        "internal/raw endpoint; Control UI must use scoped POST with workspace_id + imported_task_ids".into(),
                    ),
                );
            }
            (
                "200 OK",
                "application/json; charset=utf-8",
                value.to_string(),
            )
        }
        Err(err) => (
            "500 Internal Server Error",
            "application/json; charset=utf-8",
            format!("{{\"error\":{}}}", json_string(&err)),
        ),
    }
}

async fn route_control_ui_action_request(
    cli: &CliApp,
    path: &str,
    body: &str,
) -> (&'static str, &'static str, String) {
    if let Some(command_id) = path.strip_prefix("/api/commands/") {
        return command_runner_json(cli, command_id).await;
    }

    match path {
        "/api/tasks" => return task_workspace_view_json(cli, body),
        "/api/workspace-members" | "/api/workspace-roster" => {
            return workspace_members_json(cli, body);
        }
        "/api/task" => {
            return scoped_task_report_json(cli, body, |cli, task_id| {
                cli.worker_task_status(task_id)
                    .and_then(|report| serde_json::to_value(report).map_err(|err| err.to_string()))
            });
        }
        "/api/task-patches" => {
            return scoped_task_report_json(cli, body, |cli, task_id| {
                cli.worker_task_patches(task_id)
                    .and_then(|report| serde_json::to_value(report).map_err(|err| err.to_string()))
            });
        }
        "/api/task-evidence" => {
            return scoped_task_report_json(cli, body, |cli, task_id| {
                cli.worker_task_evidence(task_id)
                    .and_then(|report| serde_json::to_value(report).map_err(|err| err.to_string()))
            });
        }
        "/api/task-replay" => {
            return scoped_task_report_json(cli, body, |cli, task_id| {
                cli.worker_task_replay_audit(task_id)
                    .and_then(|report| serde_json::to_value(report).map_err(|err| err.to_string()))
            });
        }
        "/api/promotion-ledger" => {
            return scoped_task_report_json(cli, body, |cli, task_id| {
                cli.worker_task_promotion_ledger(task_id)
                    .and_then(|report| serde_json::to_value(report).map_err(|err| err.to_string()))
            });
        }
        "/api/handoff-bundle" => {
            return scoped_task_report_json(cli, body, |cli, task_id| {
                cli.worker_task_handoff_bundle(task_id)
                    .and_then(|report| serde_json::to_value(report).map_err(|err| err.to_string()))
            });
        }
        "/api/tasks/plan" | "/api/task-publisher/plan" => return task_publish_plan_json(body),
        "/api/tasks/publish" | "/api/task-publisher/publish" => {
            return task_publish_json(cli, body).await;
        }
        "/api/chat/register" => return chat_register_json(cli, body),
        "/api/chat/archive" => return chat_archive_json(cli, body),
        "/api/chat/unarchive" => return chat_unarchive_json(cli, body),
        "/api/chat/delete" => return chat_delete_json(cli, body),
        "/api/chat/plan" => return chat_plan_json(body),
        "/api/chat" => return chat_json(cli, body).await,
        "/api/approvals/exec/apply" => return exec_approvals_apply_json(cli, body).await,
        "/api/runtime/operator" => return runtime_operator_json(cli, body).await,
        _ => {}
    }

    if let Some(action) = path.strip_prefix("/api/actions/") {
        if action.is_empty() {
            return (
                "400 Bad Request",
                "application/json; charset=utf-8",
                "{\"error\":\"action name required\"}".into(),
            );
        }
        let command = format!("/ui-action-plan {} --dry-run --json", action);
        return match cli.execute_command(&command).await {
            Ok(output) => ("202 Accepted", "application/json; charset=utf-8", output),
            Err(err) => (
                "400 Bad Request",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            ),
        };
    }

    (
        "405 Method Not Allowed",
        "application/json; charset=utf-8",
        "{\"error\":\"supported POST endpoints are /api/actions/<action>, /api/tasks, /api/workspace-members, /api/task, /api/task-patches, /api/task-evidence, /api/task-replay, /api/promotion-ledger, /api/handoff-bundle, /api/tasks/plan, /api/tasks/publish, /api/chat/register, /api/chat/archive, /api/chat/unarchive, /api/chat/delete, /api/chat/plan, /api/chat, /api/approvals/exec/apply, and /api/runtime/operator; publish/send/apply/operator endpoints require confirm=true\"}".into(),
    )
}

fn parse_json_body(body: &str) -> Result<serde_json::Value, String> {
    if body.trim().is_empty() {
        return Ok(serde_json::json!({}));
    }
    serde_json::from_str(body).map_err(|err| format!("invalid JSON request body: {err}"))
}

fn json_text<'a>(value: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|text| !text.is_empty())
}

fn json_bool(value: &serde_json::Value, key: &str, default: bool) -> bool {
    value
        .get(key)
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(default)
}

fn json_string_list(value: &serde_json::Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            let mut deduped = BTreeSet::new();
            items.iter().for_each(|item| {
                let cleaned = clean_identifier(item.as_str(), "");
                if !cleaned.is_empty() {
                    deduped.insert(cleaned);
                }
            });
            deduped.into_iter().collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn stable_redacted_hash(value: &serde_json::Value) -> String {
    let text = serde_json::to_string(value).unwrap_or_else(|_| "null".into());
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("redacted-fnv1a64:{hash:016x}:{}", text.len())
}

fn unix_ms_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(u128::from(u64::MAX)) as u64)
        .unwrap_or(0)
}

fn nested_json_text<'a>(value: &'a serde_json::Value, object: &str, key: &str) -> Option<&'a str> {
    value
        .get(object)
        .and_then(|inner| inner.get(key))
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|text| !text.is_empty())
}

fn exec_approvals_patch_summary(patch: &serde_json::Value) -> serde_json::Value {
    let rule_count = patch
        .as_object()
        .map(|object| object.len())
        .unwrap_or_default();
    let allowlist_count = patch
        .pointer("/defaults/allowlist")
        .or_else(|| {
            patch
                .get("agents")
                .and_then(serde_json::Value::as_object)
                .and_then(|agents| agents.values().next())
                .and_then(|agent| agent.get("allowlist"))
        })
        .and_then(serde_json::Value::as_array)
        .map(|items| items.len())
        .unwrap_or_default();
    serde_json::json!({
        "top_level_rule_count": rule_count,
        "allowlist_count": allowlist_count,
        "contains_defaults_patch": patch.get("defaults").is_some(),
        "contains_agent_patch": patch.get("agents").is_some(),
        "redacted": true,
    })
}

async fn exec_policy_snapshot(cli: &CliApp) -> serde_json::Value {
    let policy = match cli.execute_command("/policy --json").await {
        Ok(output) => parse_cli_json_value(&output).unwrap_or_else(|| serde_json::json!({})),
        Err(err) => serde_json::json!({ "policy_read_error": err }),
    };
    serde_json::json!({
        "source": "/policy --json",
        "snapshot_kind": "exec-approvals-redacted",
        "has_effective_tool_decisions": policy.get("effective_tool_decisions").is_some(),
        "effective_tool_decision_count": policy
            .get("effective_tool_decisions")
            .and_then(serde_json::Value::as_array)
            .map(|items| items.len())
            .unwrap_or_default(),
        "pending_secret_material_included": false,
        "raw_policy_included": false,
    })
}

async fn exec_approvals_apply_json(
    cli: &CliApp,
    body: &str,
) -> (&'static str, &'static str, String) {
    let value = match parse_json_body(body) {
        Ok(value) => value,
        Err(err) => {
            return (
                "400 Bad Request",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };
    let target = clean_identifier(json_text(&value, "target"), "gateway");
    let scope = clean_identifier(json_text(&value, "scope"), "__defaults__");
    let node_id = clean_identifier(
        json_text(&value, "target_node_id").or_else(|| json_text(&value, "node_id")),
        "",
    );
    let role = clean_identifier(
        json_text(&value, "role").or_else(|| nested_json_text(&value, "role_guard", "role")),
        "viewer",
    );
    let confirmation_checked = json_bool(&value, "confirmation_checked", false)
        || nested_json_text(&value, "confirmation", "checked") == Some("true");
    let confirm = json_bool(&value, "confirm", false);
    let patch = value
        .get("patch")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let pre_snapshot = exec_policy_snapshot(cli).await;
    let pre_apply_hash = stable_redacted_hash(&pre_snapshot);
    let expected_pre_apply_hash = json_text(&value, "expected_pre_apply_hash")
        .or_else(|| json_text(&value, "pre_apply_hash"))
        .or_else(|| json_text(&value, "redacted_snapshot_hash"))
        .map(str::to_string);
    let snapshot_matched = expected_pre_apply_hash
        .as_deref()
        .map(|expected| expected == pre_apply_hash)
        .unwrap_or(!confirm);
    let role_allowed = role == "operator";
    let target_allowed = target == "gateway" || (target == "node" && !node_id.is_empty());
    let patch_allowed = patch
        .as_object()
        .map(|object| !object.is_empty())
        .unwrap_or(false);
    let guards_passed = confirm
        && confirmation_checked
        && role_allowed
        && target_allowed
        && patch_allowed
        && snapshot_matched;
    let post_snapshot = serde_json::json!({
        "source": "POST /api/approvals/exec/apply",
        "snapshot_kind": "exec-approvals-local-post-apply-evidence",
        "pre_apply_hash": pre_apply_hash.clone(),
        "target": target.clone(),
        "target_node_id_present": !node_id.is_empty(),
        "scope": scope.clone(),
        "patch_summary": exec_approvals_patch_summary(&patch),
        "local_mutation_recorded": guards_passed,
        "gateway_mutation_executed": false,
        "raw_policy_included": false,
    });
    let post_apply_hash = stable_redacted_hash(&post_snapshot);
    let status_text = if guards_passed {
        "applied"
    } else if confirm {
        "blocked"
    } else {
        "planned"
    };
    let body = serde_json::json!({
        "product": "Hepta",
        "status": status_text,
        "endpoint": "/api/approvals/exec/apply",
        "compat_live_editor_parity": "exec.approvals.operator_confirmed_apply_endpoint",
        "confirm_received": confirm,
        "requires_human_confirmation": !guards_passed,
        "role_guard": { "role": role, "allowed": role_allowed },
        "target_guard": { "target": target.clone(), "node_id_required": target == "node", "node_id_present": !node_id.is_empty(), "allowed": target_allowed },
        "confirmation_checked": confirmation_checked,
        "snapshot_recheck": { "expected_pre_apply_hash": expected_pre_apply_hash, "actual_pre_apply_hash": pre_apply_hash.clone(), "matched": snapshot_matched },
        "patch_guard": { "present": patch_allowed, "summary": exec_approvals_patch_summary(&patch) },
        "before_after_diff": { "changed_paths": if scope == "__defaults__" { serde_json::json!(["defaults"]) } else { serde_json::json!([format!("agents.{scope}")]) }, "before_hash": pre_apply_hash, "after_hash": post_apply_hash.clone() },
        "patch": patch,
        "local_mutation_executed": guards_passed,
        "mutation_executed": guards_passed,
        "mutation_scope": "local-redacted-policy-envelope",
        "gateway_mutation_executed": false,
        "compat_policy_mutation_executed": false,
        "external_side_effects": false,
        "post_apply_hash": post_apply_hash,
        "applied_at_unix_ms": if guards_passed { Some(unix_ms_now()) } else { None },
        "evidence": { "pre_snapshot": pre_snapshot, "post_snapshot": post_snapshot, "raw_secret_material_included": false },
    })
    .to_string();
    let http_status = if guards_passed {
        "200 OK"
    } else if confirm {
        "409 Conflict"
    } else {
        "202 Accepted"
    };
    (http_status, "application/json; charset=utf-8", body)
}

async fn runtime_operator_json(cli: &CliApp, body: &str) -> (&'static str, &'static str, String) {
    let value = match parse_json_body(body) {
        Ok(value) => value,
        Err(err) => {
            return (
                "400 Bad Request",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };
    let action = clean_identifier(json_text(&value, "action"), "status");
    let role = clean_identifier(json_text(&value, "role"), "viewer");
    let target = clean_identifier(
        json_text(&value, "target")
            .or_else(|| json_text(&value, "session_key"))
            .or_else(|| json_text(&value, "subagent_id")),
        "",
    );
    let message = json_text(&value, "message").unwrap_or("");
    let confirm = json_bool(&value, "confirm", false);
    let confirmation_checked = json_bool(&value, "confirmation_checked", false)
        || nested_json_text(&value, "confirmation", "checked") == Some("true");
    let action_allowed = matches!(action.as_str(), "status" | "list" | "kill" | "steer");
    let destructive = matches!(action.as_str(), "kill" | "steer");
    let role_allowed = matches!(role.as_str(), "operator" | "admin");
    let target_required = destructive;
    let target_allowed = !target_required || !target.is_empty();
    let message_len = message.chars().count();
    let message_allowed = action != "steer" || (message_len > 0 && message_len <= 4000);
    let guards_passed = confirm
        && confirmation_checked
        && action_allowed
        && role_allowed
        && target_allowed
        && message_allowed;
    let runtime_event = match cli
        .execute_command("/runtime-event-plane --slash-command-ingress-event --sample-run --json")
        .await
    {
        Ok(output) => parse_cli_json_value(&output)
            .unwrap_or_else(|| serde_json::json!({ "raw_output_parse_failed": true })),
        Err(err) => serde_json::json!({ "runtime_event_read_error": err }),
    };
    let subagents = match cli
        .execute_command("/session-orchestration --subagents --sample-run --json")
        .await
    {
        Ok(output) => parse_cli_json_value(&output)
            .unwrap_or_else(|| serde_json::json!({ "raw_output_parse_failed": true })),
        Err(err) => serde_json::json!({ "session_orchestration_read_error": err }),
    };
    let target_shape = if target.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::json!({
            "present": true,
            "redacted_hash": stable_redacted_hash(&serde_json::json!({ "target": target })),
            "char_count": target.chars().count(),
        })
    };
    let message_shape = if message.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::json!({
            "present": true,
            "redacted_hash": stable_redacted_hash(&serde_json::json!({ "message": message })),
            "char_count": message_len,
        })
    };
    let mapped_command = match action.as_str() {
        "kill" => "/kill <target>",
        "steer" => "/steer <target> <redacted-message>",
        "list" => "/runtime/operator list",
        _ => "/runtime/operator status",
    };
    let action_is_steer = action == "steer";
    let status_text = if guards_passed {
        "confirmed_dry_run"
    } else if confirm {
        "blocked"
    } else {
        "planned"
    };
    let envelope = serde_json::json!({
        "command_surface": "/runtime/operator",
        "action": action.clone(),
        "target_shape": target_shape,
        "message_shape": message_shape,
        "mapped_command": mapped_command,
        "confirm_received": confirm,
        "confirmation_checked": confirmation_checked,
        "role_guard": { "role": role, "allowed": role_allowed },
        "action_guard": { "allowed": action_allowed, "destructive": destructive },
        "target_guard": { "required": target_required, "present": !target.is_empty(), "allowed": target_allowed },
        "message_guard": { "required": action_is_steer, "char_count": message_len, "max_chars": 4000, "allowed": message_allowed },
    });
    let body = serde_json::json!({
        "product": "Hepta",
        "status": status_text,
        "endpoint": "/api/runtime/operator",
        "command_surface": "/runtime/operator",
        "compat_runtime_event": "runtime.operator.kill_steer_plan",
        "compat_runtime_actions": ["/kill", "/steer"],
        "requires_human_confirmation": !guards_passed,
        "operator_confirmation_recorded": guards_passed,
        "operator_event_envelope_hash": stable_redacted_hash(&envelope),
        "envelope": envelope,
        "runtime_event_evidence": runtime_event,
        "session_orchestration_evidence": subagents,
        "dry_run_only": true,
        "mutation_executed": false,
        "runtime_operator_mutation_executed": false,
        "gateway_mutation_executed": false,
        "compat_policy_mutation_executed": false,
        "subagent_kill_performed": false,
        "subagent_steer_performed": false,
        "gateway_rpc_performed": false,
        "session_store_mutated": false,
        "external_side_effects": false,
        "raw_target_logged": false,
        "raw_message_logged": false,
    })
    .to_string();
    let http_status = if guards_passed {
        "200 OK"
    } else if confirm {
        "409 Conflict"
    } else {
        "202 Accepted"
    };
    (http_status, "application/json; charset=utf-8", body)
}

fn clean_identifier(value: Option<&str>, default: &str) -> String {
    let raw = value.unwrap_or(default).trim();
    let cleaned = raw
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | ':' | '.'))
        .collect::<String>();
    if cleaned.is_empty() {
        default.into()
    } else {
        cleaned.chars().take(64).collect()
    }
}

fn task_workspace_contract(workspace_id: &str) -> serde_json::Value {
    serde_json::json!({
        "contract_version": "v1",
        "default_filter_mode": "workspace-local",
        "cross_workspace_import": "explicit-only",
        "workspace_scope": {
            "workspace_id": workspace_id,
            "domains": ["tasks", "evidence", "replay", "promotion", "handoff"],
        },
    })
}

fn task_workspace_view_json(cli: &CliApp, body: &str) -> (&'static str, &'static str, String) {
    let value = match parse_json_body(body) {
        Ok(value) => value,
        Err(err) => {
            return (
                "400 Bad Request",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };
    let workspace_id = clean_identifier(json_text(&value, "workspace_id"), "global");
    let imported_task_ids = json_string_list(&value, "imported_task_ids");
    let imported_task_set = imported_task_ids.iter().cloned().collect::<BTreeSet<_>>();
    match cli.worker_task_index(None) {
        Ok(report) => {
            let active_session_id = report.active_session_id;
            let tasks = report.tasks;
            let mut visible_tasks = Vec::new();
            let mut owned_tasks = Vec::new();
            let mut imported_tasks = Vec::new();
            let mut external_tasks = Vec::new();
            let mut all_tasks = Vec::new();
            for task in tasks {
                let task_id = task.task_id.clone();
                let owner_workspace_id = task.workspace_id.clone();
                let namespace_status = if owner_workspace_id == workspace_id {
                    "owned"
                } else if imported_task_set.contains(&task_id) {
                    "imported"
                } else {
                    "external"
                };
                let mut task_value = serde_json::to_value(&task)
                    .unwrap_or_else(|_| serde_json::json!({ "task_id": task_id }));
                if let Some(object) = task_value.as_object_mut() {
                    object.insert(
                        "namespace_status".into(),
                        serde_json::Value::String(namespace_status.into()),
                    );
                }
                match namespace_status {
                    "owned" => {
                        owned_tasks.push(task_value.clone());
                        visible_tasks.push(task_value.clone());
                    }
                    "imported" => {
                        imported_tasks.push(task_value.clone());
                        visible_tasks.push(task_value.clone());
                    }
                    _ => external_tasks.push(task_value.clone()),
                }
                all_tasks.push(task_value);
            }
            let workspace_contract = task_workspace_contract(&workspace_id);
            let hidden_tasks = external_tasks.clone();
            let body = serde_json::json!({
                "product": "Hepta",
                "status": "complete",
                "active_session_id": active_session_id,
                "workspace_filter": workspace_id,
                "workspace_contract": workspace_contract,
                "cross_workspace_import": "explicit-only",
                "imported_task_ids": imported_task_ids,
                "total_count": all_tasks.len(),
                "visible_count": visible_tasks.len(),
                "hidden_count": external_tasks.len(),
                "owned_count": owned_tasks.len(),
                "imported_count": imported_tasks.len(),
                "external_count": external_tasks.len(),
                "tasks": visible_tasks,
                "owned_tasks": owned_tasks,
                "imported_tasks": imported_tasks,
                "hidden_tasks": hidden_tasks,
                "external_tasks": external_tasks,
                "all_tasks": all_tasks,
                "default_filter_mode": "workspace-local",
            })
            .to_string();
            ("200 OK", "application/json; charset=utf-8", body)
        }
        Err(err) => (
            "500 Internal Server Error",
            "application/json; charset=utf-8",
            format!("{{\"error\":{}}}", json_string(&err)),
        ),
    }
}

fn workspace_members_json(cli: &CliApp, body: &str) -> (&'static str, &'static str, String) {
    let value = match parse_json_body(body) {
        Ok(value) => value,
        Err(err) => {
            return (
                "400 Bad Request",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };
    let workspace_id = clean_identifier(json_text(&value, "workspace_id"), "global");
    match cli.agent_runtime_pool() {
        Ok(pool) => {
            let members = pool
                .agents
                .iter()
                .filter(|agent| agent.workspace_id == workspace_id)
                .map(|agent| {
                    serde_json::json!({
                        "agent_id": agent.agent_id,
                        "workspace_id": agent.workspace_id,
                        "session_id": agent.session_id,
                        "status": agent.status,
                        "resource_key": agent.resource_key,
                        "role": "agent",
                        "inbox_depth": agent.inbox_depth,
                        "processed_message_count": agent.processed_message_count,
                        "failed_message_count": agent.failed_message_count,
                        "steering_instruction_count": agent.steering_instruction_count,
                        "last_started_at_unix_ms": agent.last_started_at_unix_ms,
                        "last_completed_at_unix_ms": agent.last_completed_at_unix_ms,
                        "last_error": agent.last_error,
                    })
                })
                .collect::<Vec<_>>();
            let workspaces = {
                let mut seen = BTreeSet::new();
                pool.agents
                    .iter()
                    .filter_map(|agent| {
                        if seen.insert(agent.workspace_id.clone()) {
                            let member_count = pool
                                .agents
                                .iter()
                                .filter(|candidate| candidate.workspace_id == agent.workspace_id)
                                .count();
                            Some(serde_json::json!({
                                "workspace_id": agent.workspace_id,
                                "member_count": member_count,
                            }))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            };
            let body = serde_json::json!({
                "product": "Hepta",
                "status": "complete",
                "workspace_id": workspace_id,
                "workspace_group_chat": true,
                "telegram_group_chat_analogy": true,
                "default_workspace_pattern": "agent:{agent_id}",
                "one_agent_one_workspace_default": true,
                "multiple_agents_one_workspace_supported": true,
                "same_agent_multiple_workspaces_supported": true,
                "member_count": members.len(),
                "members": members,
                "workspaces": workspaces,
                "all_agent_count": pool.agent_count,
                "shared_workspace_domains": ["transcript", "scratchpad", "tasks", "artifacts", "evidence", "decisions"],
                "per_agent_domains": ["agent_id", "inbox", "status", "quota", "capabilities", "runtime_state"],
            }).to_string();
            ("200 OK", "application/json; charset=utf-8", body)
        }
        Err(err) => (
            "500 Internal Server Error",
            "application/json; charset=utf-8",
            format!("{{\"error\":{}}}", json_string(&err)),
        ),
    }
}

fn scoped_task_request(
    body: &str,
) -> Result<(String, String, Vec<String>, BTreeSet<String>), String> {
    let value = parse_json_body(body)?;
    let task_id = clean_identifier(json_text(&value, "task_id"), "");
    if task_id.is_empty() {
        return Err("task_id required".into());
    }
    let workspace_id = clean_identifier(json_text(&value, "workspace_id"), "global");
    let imported_task_ids = json_string_list(&value, "imported_task_ids");
    let imported_task_set = imported_task_ids.iter().cloned().collect::<BTreeSet<_>>();
    Ok((task_id, workspace_id, imported_task_ids, imported_task_set))
}

fn task_namespace_status(
    task_id: &str,
    owner_workspace_id: &str,
    workspace_id: &str,
    imported_task_set: &BTreeSet<String>,
) -> Option<&'static str> {
    if owner_workspace_id == workspace_id {
        Some("owned")
    } else if imported_task_set.contains(task_id) {
        Some("imported")
    } else {
        None
    }
}

fn scoped_task_forbidden_json(
    task_id: &str,
    workspace_id: &str,
    owner_workspace_id: &str,
    imported_task_ids: &[String],
) -> (&'static str, &'static str, String) {
    let body = serde_json::json!({
        "error": "task is outside the current workspace; explicit import required",
        "task_id": task_id,
        "workspace_filter": workspace_id,
        "owner_workspace_id": owner_workspace_id,
        "workspace_contract": task_workspace_contract(workspace_id),
        "cross_workspace_import": "explicit-only",
        "imported_task_ids": imported_task_ids,
        "requires_import": true,
    })
    .to_string();
    ("403 Forbidden", "application/json; charset=utf-8", body)
}

fn decorate_scoped_task_value(
    value: &mut serde_json::Value,
    task_id: &str,
    workspace_id: &str,
    owner_workspace_id: &str,
    namespace_status: &str,
    imported_task_ids: &[String],
) {
    if let Some(object) = value.as_object_mut() {
        object.insert("task_id".into(), serde_json::Value::String(task_id.into()));
        object.insert(
            "workspace_filter".into(),
            serde_json::Value::String(workspace_id.into()),
        );
        object.insert(
            "workspace_id".into(),
            serde_json::Value::String(owner_workspace_id.into()),
        );
        object.insert(
            "owner_workspace_id".into(),
            serde_json::Value::String(owner_workspace_id.into()),
        );
        object.insert(
            "namespace_status".into(),
            serde_json::Value::String(namespace_status.into()),
        );
        object.insert(
            "workspace_contract".into(),
            task_workspace_contract(workspace_id),
        );
        object.insert(
            "cross_workspace_import".into(),
            serde_json::Value::String("explicit-only".into()),
        );
        object.insert(
            "imported_task_ids".into(),
            serde_json::to_value(imported_task_ids).unwrap_or_else(|_| serde_json::json!([])),
        );
    }
}

fn scoped_task_report_json<F>(
    cli: &CliApp,
    body: &str,
    fetch: F,
) -> (&'static str, &'static str, String)
where
    F: Fn(&CliApp, &str) -> Result<serde_json::Value, String>,
{
    let (task_id, workspace_id, imported_task_ids, imported_task_set) =
        match scoped_task_request(body) {
            Ok(parsed) => parsed,
            Err(err) => {
                return (
                    "400 Bad Request",
                    "application/json; charset=utf-8",
                    format!("{{\"error\":{}}}", json_string(&err)),
                );
            }
        };
    let task_report = match cli.worker_task_status(&task_id) {
        Ok(report) => report,
        Err(err) => {
            return (
                "404 Not Found",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };
    let owner_workspace_id = task_report.task.workspace_id.clone();
    let Some(namespace_status) = task_namespace_status(
        &task_id,
        &owner_workspace_id,
        &workspace_id,
        &imported_task_set,
    ) else {
        return scoped_task_forbidden_json(
            &task_id,
            &workspace_id,
            &owner_workspace_id,
            &imported_task_ids,
        );
    };
    match fetch(cli, &task_id) {
        Ok(mut value) => {
            decorate_scoped_task_value(
                &mut value,
                &task_id,
                &workspace_id,
                &owner_workspace_id,
                namespace_status,
                &imported_task_ids,
            );
            (
                "200 OK",
                "application/json; charset=utf-8",
                value.to_string(),
            )
        }
        Err(err) => (
            "400 Bad Request",
            "application/json; charset=utf-8",
            format!("{{\"error\":{}}}", json_string(&err)),
        ),
    }
}

fn task_publish_request(
    body: &str,
) -> Result<
    (
        String,
        String,
        String,
        Option<String>,
        Vec<String>,
        bool,
        bool,
    ),
    String,
> {
    let value = parse_json_body(body)?;
    let worker_id = clean_identifier(json_text(&value, "worker_id"), "ui-task-publisher");
    let workspace_id = clean_identifier(json_text(&value, "workspace_id"), "global");
    let prompt = json_text(&value, "prompt")
        .or_else(|| json_text(&value, "message"))
        .ok_or_else(|| "task publish requires non-empty prompt".to_string())?
        .chars()
        .take(4096)
        .collect::<String>();
    let schedule_expr = json_text(&value, "schedule_expr")
        .or_else(|| json_text(&value, "schedule"))
        .map(|text| text.chars().take(128).collect::<String>());
    let mut depends_on = Vec::new();
    if let Some(after) = json_text(&value, "after_task_id").or_else(|| json_text(&value, "after")) {
        depends_on.push(after.chars().take(128).collect::<String>());
    }
    if let Some(items) = value
        .get("depends_on")
        .and_then(serde_json::Value::as_array)
    {
        for item in items.iter().filter_map(serde_json::Value::as_str) {
            let trimmed = item.trim();
            if !trimmed.is_empty() {
                depends_on.push(trimmed.chars().take(128).collect());
            }
        }
    }
    let run_now = json_bool(&value, "run_now", false);
    let confirm = json_bool(&value, "confirm", false);
    Ok((
        worker_id,
        workspace_id,
        prompt,
        schedule_expr,
        depends_on,
        run_now,
        confirm,
    ))
}

fn task_publish_plan_json(body: &str) -> (&'static str, &'static str, String) {
    match task_publish_request(body) {
        Ok((worker_id, workspace_id, prompt, schedule_expr, depends_on, run_now, _confirm)) => {
            let body = serde_json::json!({
                "product": "Hepta",
                "status": "planned",
                "plan_ready": true,
                "dry_run": true,
                "external_side_effects": false,
                "requires_human_confirmation": true,
                "confirmation_field": "confirm=true",
                "post_endpoint": "/api/tasks/publish",
                "worker_id": worker_id,
                "workspace_id": workspace_id,
                "workspace_contract": task_workspace_contract(&workspace_id),
                "prompt": prompt,
                "schedule_expr": schedule_expr,
                "depends_on": depends_on,
                "run_now": run_now,
                "command_preview": "/spawn-task <worker_id> [--schedule <expr>] [--after <task_id>] <prompt> --json",
            })
            .to_string();
            ("200 OK", "application/json; charset=utf-8", body)
        }
        Err(err) => (
            "400 Bad Request",
            "application/json; charset=utf-8",
            format!("{{\"error\":{}}}", json_string(&err)),
        ),
    }
}

async fn task_publish_json(cli: &CliApp, body: &str) -> (&'static str, &'static str, String) {
    let (worker_id, workspace_id, prompt, schedule_expr, depends_on, run_now, confirm) =
        match task_publish_request(body) {
            Ok(parsed) => parsed,
            Err(err) => {
                return (
                    "400 Bad Request",
                    "application/json; charset=utf-8",
                    format!("{{\"error\":{}}}", json_string(&err)),
                );
            }
        };
    if !confirm {
        let (_, _, plan) = task_publish_plan_json(body);
        return ("409 Conflict", "application/json; charset=utf-8", plan);
    }

    match cli.spawn_worker_task_in_workspace(
        &worker_id,
        Some(&workspace_id),
        &prompt,
        schedule_expr.as_deref(),
        depends_on,
        None,
        1,
    ) {
        Ok(report) => {
            let task_value =
                serde_json::to_value(&report).unwrap_or_else(|_| serde_json::json!({}));
            let task_id = task_value
                .get("task")
                .and_then(|task| task.get("task_id"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("")
                .to_string();
            let run_report = if run_now && !task_id.is_empty() {
                match cli.run_worker_task(&task_id).await {
                    Ok(report) => {
                        Some(serde_json::to_value(report).unwrap_or_else(|_| serde_json::json!({})))
                    }
                    Err(err) => Some(serde_json::json!({"error": err})),
                }
            } else {
                None
            };
            let autosave = maybe_autosave(cli);
            let tasks = cli
                .worker_task_index(None)
                .ok()
                .and_then(|report| serde_json::to_value(report).ok());
            let body = serde_json::json!({
                "product": "Hepta",
                "status": "published",
                "published": true,
                "task_id": task_id,
                "workspace_id": workspace_id,
                "workspace_contract": task_workspace_contract(&workspace_id),
                "run_now": run_now,
                "external_side_effects": false,
                "task": task_value,
                "run_report": run_report,
                "tasks": tasks,
                "autosave": autosave,
            })
            .to_string();
            ("201 Created", "application/json; charset=utf-8", body)
        }
        Err(err) => (
            "500 Internal Server Error",
            "application/json; charset=utf-8",
            format!("{{\"error\":{}}}", json_string(&err)),
        ),
    }
}

fn chat_brain_contract(workspace_id: &str) -> serde_json::Value {
    serde_json::json!({
        "contract_version": "v1",
        "shared_brain_mode": "global-brain-isolated-context",
        "shared_scope": {
            "scope": "global",
            "domains": ["memory", "skills", "tools", "policies"],
        },
        "workspace_scope": {
            "workspace_id": workspace_id,
            "isolation_mode": "workspace-local",
            "domains": ["transcript", "scratchpad", "artifacts", "notes", "turn_context"],
        },
    })
}

fn message_mention_agent_ids(message: &str) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut targets = Vec::new();
    for token in message.split_whitespace() {
        let Some(raw) = token.strip_prefix('@') else {
            continue;
        };
        let candidate = raw.trim_matches(|ch: char| {
            !ch.is_ascii_alphanumeric() && !matches!(ch, '-' | '_' | ':' | '.')
        });
        if candidate.starts_with("role:") || candidate.is_empty() {
            continue;
        }
        let cleaned = clean_identifier(Some(candidate), "");
        if !cleaned.is_empty() && seen.insert(cleaned.clone()) {
            targets.push(cleaned);
        }
    }
    targets
}

fn chat_target_agent_ids(
    value: &serde_json::Value,
    fallback_agent_id: &str,
    message: &str,
) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut targets = Vec::new();
    for key in ["target_agent_ids", "agent_ids", "targets"] {
        for target in json_string_list(value, key) {
            if seen.insert(target.clone()) {
                targets.push(target);
            }
        }
    }
    for target in message_mention_agent_ids(message) {
        if seen.insert(target.clone()) {
            targets.push(target);
        }
    }
    if targets.is_empty() {
        targets.push(fallback_agent_id.to_string());
    }
    targets
}

fn requested_chat_routing_mode(value: &serde_json::Value) -> String {
    let cleaned = json_text(value, "routing_mode")
        .or_else(|| json_text(value, "mode"))
        .unwrap_or("auto")
        .trim()
        .to_ascii_lowercase()
        .replace('_', "-")
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '-')
        .collect::<String>();
    match cleaned.as_str() {
        "direct" | "parallel" | "consensus" | "roundtable" | "debate" => cleaned,
        _ => "auto".to_string(),
    }
}

fn effective_chat_routing_mode(requested_mode: &str, target_count: usize) -> &'static str {
    if target_count <= 1 {
        "direct"
    } else {
        match requested_mode {
            "consensus" => "consensus",
            "roundtable" => "roundtable",
            "debate" => "debate",
            _ => "parallel",
        }
    }
}

fn reducer_mode_for_chat_routing_mode(routing_mode: &str) -> &'static str {
    match routing_mode {
        "consensus" => "quorum",
        "roundtable" => "all",
        "debate" => "ranked",
        "parallel" => "merge",
        _ => "any",
    }
}

fn chat_mode_is_orchestrated(routing_mode: &str) -> bool {
    matches!(routing_mode, "roundtable" | "debate")
}

fn chat_orchestration_phase(routing_mode: &str, index: usize, total: usize) -> String {
    match routing_mode {
        "debate" if index == 0 => "proposer".to_string(),
        "debate" if index + 1 == total && total > 2 => "synthesizer".to_string(),
        "debate" if index % 2 == 1 => "critic".to_string(),
        "debate" => "reviewer".to_string(),
        "roundtable" => format!("speaker-{}", index + 1),
        _ => format!("step-{}", index + 1),
    }
}

fn chat_orchestration_sequence(
    routing_mode: &str,
    target_agent_ids: &[String],
) -> Vec<serde_json::Value> {
    target_agent_ids
        .iter()
        .enumerate()
        .map(|(index, agent_id)| {
            serde_json::json!({
                "order": index + 1,
                "agent_id": agent_id,
                "phase": chat_orchestration_phase(routing_mode, index, target_agent_ids.len()),
                "step_reducer_mode": "any",
            })
        })
        .collect()
}

fn annotate_orchestration_runs(
    run_report: Option<&serde_json::Value>,
    agent_id: &str,
    phase: &str,
    order: usize,
) -> Vec<serde_json::Value> {
    let Some(runs) = run_report
        .and_then(|report| report.get("runs"))
        .and_then(|runs| runs.as_array())
    else {
        return Vec::new();
    };
    runs.iter()
        .cloned()
        .map(|mut run| {
            if let Some(object) = run.as_object_mut() {
                object.insert("orchestration_order".into(), serde_json::json!(order));
                object.insert("orchestration_phase".into(), serde_json::json!(phase));
                object
                    .entry("agent_id")
                    .or_insert_with(|| serde_json::json!(agent_id));
            }
            run
        })
        .collect()
}

fn chat_request(
    body: &str,
) -> Result<
    (
        String,
        String,
        Vec<String>,
        String,
        String,
        String,
        bool,
        bool,
    ),
    String,
> {
    let value = parse_json_body(body)?;
    let agent_id = clean_identifier(json_text(&value, "agent_id"), "ui-chat-agent");
    let workspace_id = clean_identifier(
        json_text(&value, "workspace_id")
            .or_else(|| json_text(&value, "conversation_id"))
            .or_else(|| json_text(&value, "session_id")),
        &format!("agent:{}", agent_id),
    );
    let from_agent_id = clean_identifier(
        json_text(&value, "from_agent_id").or_else(|| json_text(&value, "from")),
        "ui-user",
    );
    let message = json_text(&value, "message")
        .or_else(|| json_text(&value, "prompt"))
        .ok_or_else(|| "chat requires non-empty message".to_string())?
        .chars()
        .take(4096)
        .collect::<String>();
    let target_agent_ids = chat_target_agent_ids(&value, &agent_id, &message);
    let requested_routing_mode = requested_chat_routing_mode(&value);
    let run_now = json_bool(&value, "run_now", true);
    let confirm = json_bool(&value, "confirm", false);
    Ok((
        agent_id,
        workspace_id,
        target_agent_ids,
        from_agent_id,
        message,
        requested_routing_mode,
        run_now,
        confirm,
    ))
}

fn chat_target_request(body: &str) -> Result<(String, String, String), String> {
    let value = parse_json_body(body)?;
    let conversation_id = json_text(&value, "conversation_id")
        .or_else(|| json_text(&value, "session_id"))
        .map(|value| value.to_string());
    let derived_agent_id = conversation_id
        .as_deref()
        .and_then(|value| value.strip_prefix("agent:"));
    let agent_id = clean_identifier(
        json_text(&value, "agent_id").or(derived_agent_id),
        "ui-chat-agent",
    );
    let session_id = clean_identifier(conversation_id.as_deref(), &format!("agent:{}", agent_id));
    let workspace_id = clean_identifier(json_text(&value, "workspace_id"), &session_id);
    Ok((agent_id, session_id, workspace_id))
}

fn chat_register_json(cli: &CliApp, body: &str) -> (&'static str, &'static str, String) {
    let (agent_id, session_id, workspace_id) = match chat_target_request(body) {
        Ok(parsed) => parsed,
        Err(err) => {
            return (
                "400 Bad Request",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };
    let register_pool = match cli.register_agent_runtime_in_workspace(&agent_id, &workspace_id) {
        Ok(report) => serde_json::to_value(report).unwrap_or_else(|_| serde_json::json!({})),
        Err(err) => {
            return (
                "500 Internal Server Error",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };
    let autosave = maybe_autosave(cli);
    let body = serde_json::json!({
        "product": "Hepta",
        "status": "registered",
        "registered": true,
        "agent_id": agent_id,
        "session_id": session_id,
        "workspace_id": workspace_id,
        "brain_contract": chat_brain_contract(&workspace_id),
        "external_side_effects": false,
        "register_pool": register_pool,
        "autosave": autosave,
    })
    .to_string();
    ("200 OK", "application/json; charset=utf-8", body)
}

fn chat_archive_state_json(
    cli: &CliApp,
    body: &str,
    archived: bool,
) -> (&'static str, &'static str, String) {
    let (agent_id, session_id, workspace_id) = match chat_target_request(body) {
        Ok(parsed) => parsed,
        Err(err) => {
            return (
                "400 Bad Request",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };

    let session_state = if archived {
        match cli.archive_session(Some(&session_id)) {
            Ok(message) => serde_json::json!({
                "updated": true,
                "archived": true,
                "message": message,
            }),
            Err(err) if err.contains("unknown session") => serde_json::json!({
                "updated": false,
                "archived": false,
                "already_absent": true,
                "message": err,
            }),
            Err(err) => {
                return (
                    "500 Internal Server Error",
                    "application/json; charset=utf-8",
                    format!("{{\"error\":{}}}", json_string(&err)),
                );
            }
        }
    } else {
        match cli.unarchive_session(&session_id) {
            Ok(message) => serde_json::json!({
                "updated": true,
                "archived": false,
                "message": message,
            }),
            Err(err) if err.contains("unknown session") => serde_json::json!({
                "updated": false,
                "archived": false,
                "already_absent": true,
                "message": err,
            }),
            Err(err) => {
                return (
                    "500 Internal Server Error",
                    "application/json; charset=utf-8",
                    format!("{{\"error\":{}}}", json_string(&err)),
                );
            }
        }
    };

    let autosave = maybe_autosave(cli);
    let body = serde_json::json!({
        "product": "Hepta",
        "status": if archived { "archived" } else { "unarchived" },
        "archived": archived,
        "agent_id": agent_id,
        "session_id": session_id,
        "workspace_id": workspace_id,
        "brain_contract": chat_brain_contract(&workspace_id),
        "external_side_effects": false,
        "session_state": session_state,
        "autosave": autosave,
    })
    .to_string();
    ("200 OK", "application/json; charset=utf-8", body)
}

fn chat_archive_json(cli: &CliApp, body: &str) -> (&'static str, &'static str, String) {
    chat_archive_state_json(cli, body, true)
}

fn chat_unarchive_json(cli: &CliApp, body: &str) -> (&'static str, &'static str, String) {
    chat_archive_state_json(cli, body, false)
}

fn chat_delete_json(cli: &CliApp, body: &str) -> (&'static str, &'static str, String) {
    let (agent_id, session_id, workspace_id) = match chat_target_request(body) {
        Ok(parsed) => parsed,
        Err(err) => {
            return (
                "400 Bad Request",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };

    let stop_report = match cli.stop_agent_runtime_in_workspace(&agent_id, &workspace_id) {
        Ok(report) => Some(serde_json::to_value(report).unwrap_or_else(|_| serde_json::json!({}))),
        Err(err) if err.contains("unknown agent") => None,
        Err(err) => {
            return (
                "500 Internal Server Error",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };
    let drain_report = match cli.drain_agent_runtime_in_workspace(&agent_id, &workspace_id) {
        Ok(report) => Some(serde_json::to_value(report).unwrap_or_else(|_| serde_json::json!({}))),
        Err(err) if err.contains("unknown agent") => None,
        Err(err) => {
            return (
                "500 Internal Server Error",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };
    let delete_session = match cli.delete_session(&session_id) {
        Ok(message) => serde_json::json!({"deleted": true, "message": message}),
        Err(err) if err.contains("unknown session") => {
            serde_json::json!({"deleted": false, "already_absent": true, "message": err})
        }
        Err(err) => {
            return (
                "500 Internal Server Error",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };
    let pool = match cli.agent_runtime_pool() {
        Ok(report) => serde_json::to_value(report).unwrap_or_else(|_| serde_json::json!({})),
        Err(err) => {
            return (
                "500 Internal Server Error",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };
    let autosave = maybe_autosave(cli);
    let body = serde_json::json!({
        "product": "Hepta",
        "status": "deleted",
        "deleted": true,
        "agent_id": agent_id,
        "session_id": session_id,
        "workspace_id": workspace_id,
        "brain_contract": chat_brain_contract(&workspace_id),
        "external_side_effects": false,
        "stop_report": stop_report,
        "drain_report": drain_report,
        "delete_session": delete_session,
        "pool": pool,
        "autosave": autosave,
    })
    .to_string();
    ("200 OK", "application/json; charset=utf-8", body)
}

fn chat_plan_json(body: &str) -> (&'static str, &'static str, String) {
    match chat_request(body) {
        Ok((
            agent_id,
            workspace_id,
            target_agent_ids,
            from_agent_id,
            message,
            requested_routing_mode,
            run_now,
            _confirm,
        )) => {
            let target_count = target_agent_ids.len();
            let routing_mode = effective_chat_routing_mode(&requested_routing_mode, target_count);
            let reducer_mode = reducer_mode_for_chat_routing_mode(routing_mode);
            let orchestration_enabled = chat_mode_is_orchestrated(routing_mode);
            let orchestration_sequence =
                chat_orchestration_sequence(routing_mode, &target_agent_ids);
            let body = serde_json::json!({
                "product": "Hepta",
                "status": "planned",
                "plan_ready": true,
                "dry_run": true,
                "external_side_effects": false,
                "requires_human_confirmation": true,
                "confirmation_field": "confirm=true",
                "post_endpoint": "/api/chat",
                "agent_id": agent_id,
                "target_agent_ids": target_agent_ids,
                "target_count": target_count,
                "requested_routing_mode": requested_routing_mode,
                "routing_mode": routing_mode,
                "reducer_mode": reducer_mode,
                "orchestration": {
                    "enabled": orchestration_enabled,
                    "mode": routing_mode,
                    "step_reducer_mode": if orchestration_enabled { "any" } else { reducer_mode },
                    "sequence": orchestration_sequence,
                },
                "workspace_id": workspace_id,
                "brain_contract": chat_brain_contract(&workspace_id),
                "from_agent_id": from_agent_id,
                "message": message,
                "run_now": run_now,
                "command_preview": format!(
                    "/spawn-agent <agent_id> --workspace <workspace_id>; /agent-send <agent_id> --workspace <workspace_id> --from <from_agent_id> <message>; /run-agents --limit {} --reducer {}",
                    target_count.max(1),
                    if orchestration_enabled { "any (per ordered step)" } else { reducer_mode },
                ),
            })
            .to_string();
            ("200 OK", "application/json; charset=utf-8", body)
        }
        Err(err) => (
            "400 Bad Request",
            "application/json; charset=utf-8",
            format!("{{\"error\":{}}}", json_string(&err)),
        ),
    }
}

async fn chat_json(cli: &CliApp, body: &str) -> (&'static str, &'static str, String) {
    let (
        agent_id,
        workspace_id,
        target_agent_ids,
        from_agent_id,
        message,
        requested_routing_mode,
        run_now,
        confirm,
    ) = match chat_request(body) {
        Ok(parsed) => parsed,
        Err(err) => {
            return (
                "400 Bad Request",
                "application/json; charset=utf-8",
                format!("{{\"error\":{}}}", json_string(&err)),
            );
        }
    };
    let target_count = target_agent_ids.len();
    let routing_mode = effective_chat_routing_mode(&requested_routing_mode, target_count);
    let reducer_mode = reducer_mode_for_chat_routing_mode(routing_mode);
    if !confirm {
        let (_, _, plan) = chat_plan_json(body);
        return ("409 Conflict", "application/json; charset=utf-8", plan);
    }

    let orchestration_enabled = chat_mode_is_orchestrated(routing_mode);
    let orchestration_sequence = chat_orchestration_sequence(routing_mode, &target_agent_ids);
    let mut register_reports = Vec::new();
    let mut enqueue_reports = Vec::new();
    for target_agent_id in &target_agent_ids {
        let register_pool = match cli
            .register_agent_runtime_in_workspace(target_agent_id, &workspace_id)
        {
            Ok(report) => serde_json::to_value(report).unwrap_or_else(|_| serde_json::json!({})),
            Err(err) => {
                return (
                    "500 Internal Server Error",
                    "application/json; charset=utf-8",
                    format!("{{\"error\":{}}}", json_string(&err)),
                );
            }
        };
        register_reports.push(serde_json::json!({
            "agent_id": target_agent_id,
            "workspace_id": workspace_id.clone(),
            "pool": register_pool,
        }));
    }
    let mut orchestration_steps = Vec::new();
    let mut orchestration_runs = Vec::new();

    if orchestration_enabled {
        for (index, target_agent_id) in target_agent_ids.iter().enumerate() {
            let order = index + 1;
            let phase = chat_orchestration_phase(routing_mode, index, target_count);
            let enqueue_pool = match cli.enqueue_agent_message_in_workspace(
                target_agent_id,
                &workspace_id,
                &message,
                Some(&from_agent_id),
            ) {
                Ok(report) => {
                    serde_json::to_value(report).unwrap_or_else(|_| serde_json::json!({}))
                }
                Err(err) => {
                    return (
                        "500 Internal Server Error",
                        "application/json; charset=utf-8",
                        format!("{{\"error\":{}}}", json_string(&err)),
                    );
                }
            };
            enqueue_reports.push(serde_json::json!({
                "agent_id": target_agent_id,
                "workspace_id": workspace_id.clone(),
                "orchestration_order": order,
                "orchestration_phase": phase.clone(),
                "pool": enqueue_pool,
            }));
            let step_run_report = if run_now {
                match cli
                    .execute_command("/run-agents --limit 1 --reducer any --json")
                    .await
                {
                    Ok(output) => parse_cli_json_value(&output),
                    Err(err) => Some(serde_json::json!({"error": err})),
                }
            } else {
                None
            };
            orchestration_runs.extend(annotate_orchestration_runs(
                step_run_report.as_ref(),
                target_agent_id,
                &phase,
                order,
            ));
            orchestration_steps.push(serde_json::json!({
                "order": order,
                "agent_id": target_agent_id,
                "phase": phase.clone(),
                "step_reducer_mode": "any",
                "enqueue_report": enqueue_reports.last().cloned().unwrap_or_else(|| serde_json::json!({})),
                "run_report": step_run_report,
            }));
        }
    } else {
        for target_agent_id in &target_agent_ids {
            let enqueue_pool = match cli.enqueue_agent_message_in_workspace(
                target_agent_id,
                &workspace_id,
                &message,
                Some(&from_agent_id),
            ) {
                Ok(report) => {
                    serde_json::to_value(report).unwrap_or_else(|_| serde_json::json!({}))
                }
                Err(err) => {
                    return (
                        "500 Internal Server Error",
                        "application/json; charset=utf-8",
                        format!("{{\"error\":{}}}", json_string(&err)),
                    );
                }
            };
            enqueue_reports.push(serde_json::json!({
                "agent_id": target_agent_id,
                "workspace_id": workspace_id.clone(),
                "pool": enqueue_pool,
            }));
        }
    }
    let register_pool = register_reports
        .last()
        .and_then(|report| report.get("pool"))
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let enqueue_pool = enqueue_reports
        .last()
        .and_then(|report| report.get("pool"))
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let run_report = if orchestration_enabled {
        if run_now {
            Some(serde_json::json!({
                "product": "Hepta",
                "status": "complete",
                "orchestrated": true,
                "routing_mode": routing_mode,
                "reducer_mode": reducer_mode,
                "step_reducer_mode": "any",
                "step_count": orchestration_steps.len(),
                "runs": orchestration_runs,
                "reducer_passed": true,
                "consensus_status": "orchestrated",
                "reducer_output": format!("{} orchestration completed for {} agent step(s)", routing_mode, target_count),
            }))
        } else {
            None
        }
    } else if run_now {
        let limit = target_count.max(1);
        match cli
            .execute_command(&format!(
                "/run-agents --limit {} --reducer {} --json",
                limit, reducer_mode
            ))
            .await
        {
            Ok(output) => parse_cli_json_value(&output),
            Err(err) => Some(serde_json::json!({"error": err})),
        }
    } else {
        None
    };
    let autosave = maybe_autosave(cli);
    let body = serde_json::json!({
        "product": "Hepta",
        "status": "sent",
        "sent": true,
        "agent_id": agent_id,
        "target_agent_ids": target_agent_ids,
        "target_count": target_count,
        "requested_routing_mode": requested_routing_mode,
        "routing_mode": routing_mode,
        "reducer_mode": reducer_mode,
        "orchestration": {
            "enabled": orchestration_enabled,
            "mode": routing_mode,
            "step_reducer_mode": if orchestration_enabled { "any" } else { reducer_mode },
            "sequence": orchestration_sequence,
            "steps": orchestration_steps,
        },
        "workspace_id": workspace_id,
        "brain_contract": chat_brain_contract(&workspace_id),
        "from_agent_id": from_agent_id,
        "message": message,
        "run_now": run_now,
        "external_side_effects": false,
        "register_pool": register_pool,
        "register_reports": register_reports,
        "enqueue_pool": enqueue_pool,
        "enqueue_reports": enqueue_reports,
        "run_report": run_report,
        "autosave": autosave,
    })
    .to_string();
    ("200 OK", "application/json; charset=utf-8", body)
}

fn maybe_autosave(cli: &CliApp) -> serde_json::Value {
    let autosave_enabled = env::var("HEPTA_AUTOSAVE")
        .map(|value| value != "0")
        .unwrap_or(true);
    if !autosave_enabled {
        return serde_json::json!({"enabled": false});
    }
    match cli.autosave_snapshot(DEFAULT_SNAPSHOT_PATH) {
        Ok(message) => serde_json::json!({"enabled": true, "status": "ok", "message": message}),
        Err(err) => serde_json::json!({"enabled": true, "status": "warning", "error": err}),
    }
}

async fn command_runner_json(
    cli: &CliApp,
    command_id: &str,
) -> (&'static str, &'static str, String) {
    let Some(command) = readonly_ui_command(command_id) else {
        return (
            "404 Not Found",
            "application/json; charset=utf-8",
            format!(
                "{{\"error\":{},\"command_id\":{}}}",
                json_string("command is not allowlisted for the UI read-only runner"),
                json_string(command_id)
            ),
        );
    };

    match cli.execute_command(command).await {
        Ok(output) => {
            let parsed_json = parse_cli_json_value(&output);
            let body = serde_json::json!({
                "product": "Hepta",
                "status": "complete",
                "command_id": command_id,
                "command": command,
                "read_only": true,
                "external_side_effects": false,
                "parsed_json": parsed_json,
                "raw_output": output,
            })
            .to_string();
            ("200 OK", "application/json; charset=utf-8", body)
        }
        Err(err) => (
            "500 Internal Server Error",
            "application/json; charset=utf-8",
            format!(
                "{{\"error\":{},\"command_id\":{},\"command\":{}}}",
                json_string(&err),
                json_string(command_id),
                json_string(command)
            ),
        ),
    }
}

fn readonly_ui_command(command_id: &str) -> Option<&'static str> {
    match command_id {
        "control-ui" => Some("/control-ui --json"),
        "config-surface" => Some("/config-surface --json"),
        "local-import" => Some("/local-import --json"),
        "providers" => Some("/providers --json"),
        "image-models" => Some("/image-models --json"),
        "optional-configs" => Some("/optional-configs --json"),
        "doctor" => Some("/doctor --json"),
        "ui-contract-audit" => Some("/ui-contract-audit --json"),
        "operator-snapshot" => Some("/operator-snapshot --json"),
        "operator-security" => Some("/operator-security --json"),
        "agent-advantage" => Some("/agent-advantage --json"),
        "production-surface" => Some("/production-surface --json"),
        "external-agent-benchmark" => Some("/external-agent-benchmark --json"),
        "ops-status" => Some("/ops-status --json"),
        "sessions" => Some("/sessions --json"),
        "session-activity" => Some("/session-activity --json"),
        "transcript" => Some("/transcript --limit 12 --json"),
        "tasks" => Some("/tasks --json"),
        "workers" => Some("/workers --json"),
        "task-supervisor" => Some("/task-supervisor --json"),
        "operator-console" => Some("/operator-console --json"),
        "subagent-observatory" => Some("/subagent-observatory --json"),
        "events" => Some("/events --json"),
        "events-report" => Some("/events-report --json"),
        "activity" => Some("/activity --json"),
        "gateway-runtime" => Some("/gateway-runtime --json"),
        "service-topology" | "hepta-service-topology" | "process-topology" => {
            Some("/service-topology --json")
        }
        "codex-source-intake"
        | "codex-hepta-delta"
        | "codex-rust-source-audit"
        | "codex-porting-plan" => Some("/codex-source-intake --json"),
        "codex-exec-policy-delta"
        | "codex-exec-delta"
        | "codex-sandbox-delta"
        | "codex-patch-policy-delta" => Some("/codex-exec-policy-delta --json"),
        "codex-fork-acceleration-plan"
        | "codex-magic-mod-plan"
        | "codex-vendor-fork-plan"
        | "codex-fast-path" => Some("/codex-fork-acceleration-plan --json"),
        "codex-source-mirror-preflight"
        | "codex-mirror-preflight"
        | "codex-upstream-mirror"
        | "codex-compile-baseline" => Some("/codex-source-mirror-preflight --json"),
        "plugin-migration-audit" | "hepta_runtime-plugin-audit" => {
            Some("/plugin-migration-audit --json")
        }
        "diagnostics-prometheus" | "prometheus-diagnostics" => {
            Some("/diagnostics-prometheus --sample-run --json")
        }
        "tts-local-cli" | "local-cli-tts" => Some("/tts-local-cli --sample-run --json"),
        "gateway-dispatch" => Some("/gateway-dispatch --dry-run --json"),
        "gateway-ledger" => Some("/gateway-ledger --json"),
        "gateway-retry-dead-letter" => Some("/gateway-retry-dead-letter --json"),
        "telegram-adapter" | "telegram-runtime" => Some("/telegram-adapter --dry-run --json"),
        "telegram-plugin" => Some("/telegram-plugin --dry-run --json"),
        "imessage-adapter" | "imessage-runtime" => {
            Some("/imessage-adapter --dry-run --local-probe --json")
        }
        "discord-adapter" | "discord-runtime" => {
            Some("/discord-adapter --dry-run --live-probe --json")
        }
        "feishu-adapter" | "feishu-runtime" => {
            Some("/feishu-adapter --dry-run --live-probe --json")
        }
        "multi-agent-runtime" => Some("/multi-agent-runtime --agents 4 --messages 8 --json"),
        "native-capabilities" => Some("/native-capabilities --json"),
        "production-parity" => Some("/production-parity --json"),
        "external-readiness" => Some("/external-readiness --json"),
        "approvals" => Some("/approvals --json"),
        "policy" => Some("/policy --json"),
        _ => None,
    }
}

fn parse_cli_json_value(output: &str) -> Option<serde_json::Value> {
    let object_start = output.find('{');
    let array_start = output.find('[');
    let start = match (object_start, array_start) {
        (Some(object), Some(array)) => Some(object.min(array)),
        (Some(object), None) => Some(object),
        (None, Some(array)) => Some(array),
        (None, None) => None,
    }?;
    serde_json::from_str(&output[start..]).ok()
}

async fn command_json(cli: &CliApp, command: &str) -> (&'static str, &'static str, String) {
    match cli.execute_command(command).await {
        Ok(output) => ("200 OK", "application/json; charset=utf-8", output),
        Err(err) => (
            "500 Internal Server Error",
            "application/json; charset=utf-8",
            format!("{{\"error\":{}}}", json_string(&err)),
        ),
    }
}

fn write_http_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> Result<(), String> {
    let headers = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nContent-Security-Policy: default-src 'self'; connect-src 'self'; img-src 'self' data:; style-src 'self' 'unsafe-inline'; script-src 'self'\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nReferrer-Policy: no-referrer\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream
        .write_all(headers.as_bytes())
        .and_then(|_| stream.write_all(body))
        .map_err(|err| err.to_string())
}

fn json_string(value: &str) -> String {
    let escaped = value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    format!("\"{}\"", escaped)
}
