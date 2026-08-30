from __future__ import annotations

import os
import pathlib
import subprocess
import tomllib

ROOT = pathlib.Path.cwd()
CARGO_ROOT = ROOT / "codex-rs"
TARGET_BRANCH = os.environ["TARGET_BRANCH"]
EXPECTED_HEAD = os.environ["EXPECTED_HEAD"]

SPECS: dict[str, tuple[str, str]] = {
    "ext/executor-skills": ("executor-skills", "codex_executor_skills_extension"),
    "hepta-authority": ("hepta-authority", "hepta_authority"),
    "hepta-cli": ("hepta-cli", "hepta_cli"),
    "hepta-contracts": ("hepta-contracts", "hepta_contracts"),
    "hepta-core": ("hepta-core", "hepta_core"),
    "hepta-durable-store": ("hepta-durable-store", "hepta_durable_store"),
    "hepta-egress": ("hepta-egress", "hepta_egress"),
    "hepta-gateway": ("hepta-gateway", "hepta_gateway"),
    "hepta-intelligence": ("hepta-intelligence", "hepta_intelligence"),
    "hepta-kernel": ("hepta-kernel", "hepta_kernel"),
    "hepta-kg": ("hepta-kg", "hepta_kg"),
    "hepta-mcp-pagination": ("hepta-mcp-pagination", "hepta_mcp_pagination"),
    "hepta-memory": ("hepta-memory", "hepta_memory"),
    "hepta-native-gateway": ("hepta-native-gateway", "hepta_native_gateway"),
    "hepta-paths": ("hepta-paths", "hepta_paths"),
    "hepta-plugins": ("hepta-plugins", "hepta_plugins"),
    "hepta-runtime": ("hepta-runtime", "hepta_runtime"),
}

EXPECTED_PATHS = sorted(
    [
        "codex-rs/cli/BUILD.bazel",
        "codex-rs/core-plugins/src/manager_tests.rs",
        "codex-rs/ext/executor-skills/BUILD.bazel",
        "codex-rs/hepta-authority/BUILD.bazel",
        "codex-rs/hepta-cli/BUILD.bazel",
        "codex-rs/hepta-contracts/BUILD.bazel",
        "codex-rs/hepta-core/BUILD.bazel",
        "codex-rs/hepta-durable-store/BUILD.bazel",
        "codex-rs/hepta-egress/BUILD.bazel",
        "codex-rs/hepta-gateway/BUILD.bazel",
        "codex-rs/hepta-intelligence/BUILD.bazel",
        "codex-rs/hepta-kernel/BUILD.bazel",
        "codex-rs/hepta-kg/BUILD.bazel",
        "codex-rs/hepta-mcp-pagination/BUILD.bazel",
        "codex-rs/hepta-memory/BUILD.bazel",
        "codex-rs/hepta-native-gateway/BUILD.bazel",
        "codex-rs/hepta-paths/BUILD.bazel",
        "codex-rs/hepta-plugins/BUILD.bazel",
        "codex-rs/hepta-runtime/BUILD.bazel",
        "codex-rs/http-client/src/outbound_proxy_tests.rs",
        "codex-rs/rmcp-client/BUILD.bazel",
    ]
)


def run(*args: str, capture: bool = False) -> str:
    completed = subprocess.run(
        list(args),
        check=True,
        text=True,
        stdout=subprocess.PIPE if capture else None,
    )
    return completed.stdout.strip() if capture else ""


def joined(lines: list[str]) -> str:
    return "\n".join(lines) + "\n"


def replace_once(path: pathlib.Path, old: str, new: str, label: str) -> None:
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one anchor, found {count}")
    updated = text.replace(old, new, 1)
    if updated == text:
        raise SystemExit(f"{label}: replacement produced no change")
    path.write_text(updated, encoding="utf-8", newline="\n")


def patch_windows_socket_fixture() -> None:
    path = CARGO_ROOT / "http-client/src/outbound_proxy_tests.rs"
    old = joined(
        [
            "            };",
            "            stream",
            "                .set_read_timeout(Some(Duration::from_secs(10)))",
            "                .expect(\"HTTP stream should get a read timeout\");",
        ]
    )
    new = joined(
        [
            "            };",
            "            stream",
            "                .set_nonblocking(false)",
            "                .expect(\"accepted HTTP stream should become blocking\");",
            "            stream",
            "                .set_read_timeout(Some(Duration::from_secs(10)))",
            "                .expect(\"HTTP stream should get a read timeout\");",
        ]
    )
    replace_once(path, old, new, "accepted Windows socket blocking transition")


def patch_remote_plugin_fixture() -> None:
    path = CARGO_ROOT / "core-plugins/src/manager_tests.rs"
    text = path.read_text(encoding="utf-8")
    old_name = (
        "async fn remote_installed_cache_adds_plugin_skill_roots_without_remote_plugin_flag() {"
    )
    new_name = (
        "async fn remote_installed_cache_adds_plugin_skill_roots_in_chatgpt_remote_mode() {"
    )
    next_name = "async fn plugins_for_config_routes_curated_plugins_by_auth_mode() {"
    if text.count(old_name) != 1 or text.count(next_name) != 1:
        raise SystemExit("remote plugin test boundary drift")
    start = text.index(old_name)
    end = text.index(next_name, start)
    block = text[start:end]
    if block.count(old_name) != 1:
        raise SystemExit("remote plugin test name anchor drift")
    block = block.replace(old_name, new_name, 1)

    old_config = joined(
        [
            "        r#\"[features]",
            "plugins = true",
            "\"#;",
        ]
    )
    new_config = joined(
        [
            "        r#\"[features]",
            "plugins = true",
            "remote_plugin = true",
            "\"#;",
        ]
    )
    if block.count(old_config) != 1:
        raise SystemExit(
            "remote plugin feature fixture: "
            f"expected one anchor, found {block.count(old_config)}"
        )
    block = block.replace(old_config, new_config, 1)

    old_manager = joined(
        [
            "    let manager = PluginsManager::new_with_restriction_product(",
            "        codex_home.path().to_path_buf(),",
            "        /*restriction_product*/ None,",
            "    );",
            "    manager.write_remote_installed_plugins_cache(vec![RemoteInstalledPlugin {",
        ]
    )
    new_manager = joined(
        [
            "    let manager = PluginsManager::new_with_restriction_product(",
            "        codex_home.path().to_path_buf(),",
            "        /*restriction_product*/ None,",
            "    );",
            "    assert!(manager.set_auth_mode(Some(AuthMode::Chatgpt)));",
            "    manager.write_remote_installed_plugins_cache(vec![RemoteInstalledPlugin {",
        ]
    )
    if block.count(old_manager) != 1:
        raise SystemExit(
            "remote plugin auth fixture: "
            f"expected one anchor, found {block.count(old_manager)}"
        )
    block = block.replace(old_manager, new_manager, 1)
    path.write_text(text[:start] + block + text[end:], encoding="utf-8", newline="\n")


def patch_binary_labels() -> None:
    cli_build = CARGO_ROOT / "cli/BUILD.bazel"
    replace_once(
        cli_build,
        joined(["multiplatform_binaries(", "    name = \"codex\",", ")"]),
        joined(
            [
                "multiplatform_binaries(",
                "    name = \"hepta-codex-compat\",",
                ")",
            ]
        ),
        "CLI binary target rename",
    )

    rmcp_build = CARGO_ROOT / "rmcp-client/BUILD.bazel"
    replace_once(
        rmcp_build,
        "        \"//codex-rs/cli:codex\",",
        "        \"//codex-rs/cli:hepta-codex-compat\",",
        "RMCP external binary label",
    )


def standard_build(target_name: str, crate_name: str) -> str:
    return joined(
        [
            "load(\"//:defs.bzl\", \"codex_rust_crate\")",
            "",
            "codex_rust_crate(",
            f"    name = \"{target_name}\",",
            f"    crate_name = \"{crate_name}\",",
            ")",
        ]
    )


def add_missing_bazel_packages() -> None:
    workspace = tomllib.loads((CARGO_ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    members = workspace["workspace"]["members"]
    if len(members) != len(set(members)):
        raise SystemExit("duplicate Cargo workspace members")
    missing_before = {
        member
        for member in members
        if not (CARGO_ROOT / member / "BUILD.bazel").is_file()
        and not (CARGO_ROOT / member / "BUILD").is_file()
    }
    if missing_before != set(SPECS):
        raise SystemExit(
            "workspace/Bazel package set drift: "
            f"expected={sorted(SPECS)} actual={sorted(missing_before)}"
        )

    for member, (target_name, crate_name) in SPECS.items():
        member_dir = CARGO_ROOT / member
        cargo_path = member_dir / "Cargo.toml"
        if not cargo_path.is_file():
            raise SystemExit(f"workspace Cargo manifest missing: {member}")
        cargo = tomllib.loads(cargo_path.read_text(encoding="utf-8"))
        package_name = cargo.get("package", {}).get("name")
        if not isinstance(package_name, str) or not package_name:
            raise SystemExit(f"workspace package identity missing: {member}")
        if target_name != pathlib.PurePosixPath(member).name:
            raise SystemExit(f"non-canonical Bazel target name for {member}: {target_name}")

        has_lib = (member_dir / "src/lib.rs").is_file()
        has_main = (member_dir / "src/main.rs").is_file()
        if not has_lib and not has_main:
            raise SystemExit(f"Rust crate root missing: {member}")
        if has_lib:
            cargo_lib = cargo.get("lib", {})
            expected_crate_name = cargo_lib.get("name", package_name.replace("-", "_"))
            if crate_name != expected_crate_name:
                raise SystemExit(
                    f"crate name drift for {member}: "
                    f"expected={expected_crate_name} actual={crate_name}"
                )

        build_path = member_dir / "BUILD.bazel"
        if build_path.exists():
            raise SystemExit(f"refusing to overwrite existing BUILD.bazel: {member}")

        if member == "hepta-cli":
            bins = cargo.get("bin", [])
            if not isinstance(bins, list) or [entry.get("name") for entry in bins] != ["hepta"]:
                raise SystemExit("hepta-cli binary identity drift")
            content = joined(
                [
                    "load(\"//:defs.bzl\", \"codex_rust_crate\", \"multiplatform_binaries\")",
                    "",
                    "codex_rust_crate(",
                    "    name = \"hepta-cli\",",
                    "    crate_name = \"hepta_cli\",",
                    ")",
                    "",
                    "multiplatform_binaries(",
                    "    name = \"hepta\",",
                    ")",
                ]
            )
        elif member == "hepta-runtime":
            required = [
                member_dir / "build.rs",
                member_dir / "codegen/workgraph-v3/modules.bundle.gz",
                member_dir / "src/work_graph_control_plane_tests.rs",
            ]
            missing_inputs = [str(path) for path in required if not path.is_file()]
            if missing_inputs:
                raise SystemExit(f"hepta-runtime build inputs missing: {missing_inputs}")
            content = joined(
                [
                    "load(\"//:defs.bzl\", \"codex_rust_crate\")",
                    "",
                    "codex_rust_crate(",
                    "    name = \"hepta-runtime\",",
                    "    crate_name = \"hepta_runtime\",",
                    "    build_script_data = glob([",
                    "        \"codegen/workgraph-v3/**\",",
                    "        \"src/work_graph_control_plane_tests.rs\",",
                    "    ]),",
                    ")",
                ]
            )
        else:
            content = standard_build(target_name, crate_name)

        if not content.startswith("load("):
            raise SystemExit(f"generated BUILD does not start at column zero: {member}")
        if any(line.startswith("          ") for line in content.splitlines()):
            raise SystemExit(f"generated BUILD retained publisher indentation: {member}")
        build_path.write_text(content, encoding="utf-8", newline="\n")

    missing_after = [
        member
        for member in members
        if not (CARGO_ROOT / member / "BUILD.bazel").is_file()
        and not (CARGO_ROOT / member / "BUILD").is_file()
    ]
    if missing_after:
        raise SystemExit(f"workspace members still missing Bazel packages: {missing_after}")


def validate_identity_and_surface() -> None:
    cli_cargo = tomllib.loads((CARGO_ROOT / "cli/Cargo.toml").read_text(encoding="utf-8"))
    cli_bins = [entry.get("name") for entry in cli_cargo.get("bin", [])]
    if cli_bins != ["hepta-codex-compat"]:
        raise SystemExit(f"CLI Cargo binary identity drift: {cli_bins}")
    cli_build = (CARGO_ROOT / "cli/BUILD.bazel").read_text(encoding="utf-8")
    rmcp_build = (CARGO_ROOT / "rmcp-client/BUILD.bazel").read_text(encoding="utf-8")
    if 'name = "hepta-codex-compat"' not in cli_build:
        raise SystemExit("CLI Bazel binary identity was not repaired")
    if "//codex-rs/cli:hepta-codex-compat" not in rmcp_build:
        raise SystemExit("RMCP Bazel binary identity was not repaired")
    if "//codex-rs/cli:codex" in rmcp_build:
        raise SystemExit("stale RMCP binary identity remains")

    run("git", "diff", "--check")
    actual = sorted(run("git", "diff", "--name-only", capture=True).splitlines())
    if actual != EXPECTED_PATHS:
        raise SystemExit(f"exact change surface drift: expected={EXPECTED_PATHS} actual={actual}")
    for relative in actual:
        data = pathlib.Path(relative).read_bytes()
        if b"\r\n" in data:
            raise SystemExit(f"CRLF introduced: {relative}")
        if not data.endswith(b"\n"):
            raise SystemExit(f"missing final LF: {relative}")


def publish() -> None:
    current = run("git", "rev-parse", "HEAD", capture=True)
    if current != EXPECTED_HEAD:
        raise SystemExit(f"unexpected checkout: expected={EXPECTED_HEAD} actual={current}")
    if run("git", "status", "--porcelain", capture=True):
        raise SystemExit("working tree is not clean before patching")

    patch_windows_socket_fixture()
    patch_remote_plugin_fixture()
    patch_binary_labels()
    add_missing_bazel_packages()
    validate_identity_and_surface()

    run("git", "config", "user.name", "Hepta UI governed source publisher")
    run(
        "git",
        "config",
        "user.email",
        "hepta-ui-source-publisher@users.noreply.github.com",
    )
    run("git", "add", "-A")
    staged = sorted(run("git", "diff", "--cached", "--name-only", capture=True).splitlines())
    if staged != EXPECTED_PATHS:
        raise SystemExit(f"staged change surface drift: expected={EXPECTED_PATHS} actual={staged}")
    run("git", "commit", "-m", "fix(ui): close exact-head Windows Bazel source blockers")
    commit = run("git", "rev-parse", "HEAD", capture=True)
    tree = run("git", "rev-parse", "HEAD^{tree}", capture=True)
    parent = run("git", "rev-parse", "HEAD^", capture=True)
    if parent != EXPECTED_HEAD:
        raise SystemExit(f"source repair parent drift: expected={EXPECTED_HEAD} actual={parent}")

    live_head = run(
        "git",
        "ls-remote",
        "--exit-code",
        "origin",
        f"refs/heads/{TARGET_BRANCH}",
        capture=True,
    ).split()[0]
    if live_head != EXPECTED_HEAD:
        raise SystemExit(
            f"target branch moved before publication: expected={EXPECTED_HEAD} actual={live_head}"
        )
    run("git", "push", "origin", f"HEAD:refs/heads/{TARGET_BRANCH}")
    print(f"published_commit={commit}")
    print(f"published_tree={tree}")
    print(f"published_parent={parent}")


if __name__ == "__main__":
    publish()
