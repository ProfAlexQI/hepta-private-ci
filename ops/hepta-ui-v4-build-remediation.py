#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import re
from pathlib import Path

ROOT = Path(os.environ.get('HEPTA_CANDIDATE_ROOT', '.')).resolve()
LIFECYCLE = 'apps/hepta-native/src/shared/hepta_material_app_lifecycle.rs'


def replace_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise RuntimeError(f'{label}: expected one exact match, found {count}')
    return text.replace(old, new, 1)


def write(path: str, text: str) -> None:
    target = ROOT / path
    target.write_text(text, encoding='utf-8', newline='\n')


def ensure_pr_path(text: str, path: str, *, anchor: str) -> str:
    quoted = f'      - "{path}"\n'
    if quoted in text:
        return text
    anchor_line = f'      - "{anchor}"\n'
    return replace_once(text, anchor_line, quoted + anchor_line, f'path trigger {path}')


def ensure_candidate_env(text: str, label: str) -> str:
    declaration = '  HEPTA_CANDIDATE_SHA: ${{ github.event.pull_request.head.sha || github.sha }}\n'
    if declaration in text:
        return text
    marker = 'permissions:\n  contents: read\n'
    if marker not in text:
        raise RuntimeError(f'{label}: permissions marker missing')
    return text.replace(marker, marker + '\nenv:\n' + declaration, 1)


def update_concurrency(text: str, label: str) -> str:
    pattern = re.compile(r'(?m)^(  group:) .+$')
    replacement = r'\1 ${{ github.workflow }}-${{ github.event_name }}-${{ github.event.pull_request.head.sha || github.sha }}'
    text, count = pattern.subn(replacement, text, count=1)
    if count != 1:
        raise RuntimeError(f'{label}: concurrency group missing')
    return text


def windows_checkout_step(indent: str) -> str:
    return (
        f"{indent}- name: Configure deterministic Windows checkout\n"
        f"{indent}  if: runner.os == 'Windows'\n"
        f"{indent}  shell: pwsh\n"
        f"{indent}  run: |\n"
        f"{indent}    git config --global core.longpaths true\n"
        f"{indent}    git config --global core.autocrlf false\n"
        f"{indent}    git config --global core.eol lf\n\n"
    )


def checkout_step(indent: str) -> str:
    return (
        windows_checkout_step(indent)
        + f"{indent}- name: Checkout exact candidate\n"
        + f"{indent}  uses: actions/checkout@v4\n"
        + f"{indent}  with:\n"
        + f"{indent}    ref: ${{{{ env.HEPTA_CANDIDATE_SHA }}}}\n"
        + f"{indent}    fetch-depth: 1\n"
        + f"{indent}    persist-credentials: false\n"
    )


def verify_step(indent: str) -> str:
    return (
        f"{indent}- name: Verify exact checkout identity\n"
        f"{indent}  shell: bash\n"
        f"{indent}  run: |\n"
        f"{indent}    set -euo pipefail\n"
        f'{indent}    test "$(git rev-parse HEAD)" = "$HEPTA_CANDIDATE_SHA"\n'
    )


def harden_checkouts(text: str, label: str) -> str:
    lines = text.splitlines(keepends=True)
    output: list[str] = []
    i = 0
    checkout_count = 0
    while i < len(lines):
        line = lines[i]
        raw_line = line.rstrip('\r\n')
        named = re.match(r'^(\s*)- name:\s*(?:Checkout exact candidate|Checkout candidate)\s*$', raw_line)
        anonymous = re.match(r'^(\s*)- uses:\s*actions/checkout@v4\s*$', raw_line)
        indent: str | None = None
        if named and i + 1 < len(lines) and re.match(
            rf'^{re.escape(named.group(1))}  uses:\s*actions/checkout@v4\s*$',
            lines[i + 1].rstrip('\r\n'),
        ):
            indent = named.group(1)
            j = i + 2
        elif anonymous:
            indent = anonymous.group(1)
            j = i + 1
        else:
            output.append(line)
            i += 1
            continue

        checkout_count += 1
        base_indent = len(indent)
        while j < len(lines):
            raw = lines[j].rstrip('\r\n')
            if raw and len(raw) - len(raw.lstrip(' ')) <= base_indent:
                break
            j += 1

        stale_start = len(output)
        for back in range(len(output) - 1, max(-1, len(output) - 10), -1):
            if re.match(rf'^{re.escape(indent)}- name: (?:Enable Windows long paths before checkout|Configure deterministic Windows checkout)\s*$', output[back].rstrip('\r\n')):
                stale_start = back
                break
            if output[back].strip().startswith('- name:') or output[back].strip().startswith('- uses:'):
                break
        if stale_start < len(output):
            del output[stale_start:]

        output.append(checkout_step(indent))
        i = j

        next_named = None
        if i < len(lines):
            next_named = re.match(
                rf'^{re.escape(indent)}- name:\s*Verify exact checkout(?: identity)?\s*$',
                lines[i].rstrip('\r\n'),
            )
        if not next_named:
            output.append('\n')
            output.append(verify_step(indent))
            output.append('\n')

    if checkout_count == 0:
        raise RuntimeError(f'{label}: no checkout steps found')
    return ''.join(output)


def harden_native_workflow(path: str, anchor: str) -> None:
    text = (ROOT / path).read_text(encoding='utf-8')
    text = ensure_pr_path(text, LIFECYCLE, anchor=anchor)
    text = ensure_candidate_env(text, path)
    text = update_concurrency(text, path)
    text = harden_checkouts(text, path)
    text = text.replace('toolchain: 1.88.0', 'toolchain: 1.95.0')
    text = text.replace('Install Rust 1.88', 'Install Rust 1.95')
    text = text.replace('${{ github.sha }}', '${{ env.HEPTA_CANDIDATE_SHA }}')
    text = text.replace('$env:GITHUB_SHA', '$env:HEPTA_CANDIDATE_SHA')
    text = text.replace('$GITHUB_SHA', '$HEPTA_CANDIDATE_SHA')
    text = text.replace('ENV.fetch("GITHUB_SHA")', 'ENV.fetch("HEPTA_CANDIDATE_SHA")')
    text = text.replace("ENV.fetch('GITHUB_SHA')", "ENV.fetch('HEPTA_CANDIDATE_SHA')")
    write(path, text)


def patch_gitattributes() -> None:
    path = '.gitattributes'
    text = (ROOT / path).read_text(encoding='utf-8')
    additions = (
        '\n# Canonical Rust-served Control UI text assets are byte-stable on every runner.\n'
        'apps/hepta-control-ui/*.html text eol=lf\n'
        'apps/hepta-control-ui/*.css text eol=lf\n'
        'apps/hepta-control-ui/*.js text eol=lf\n'
    )
    if 'apps/hepta-control-ui/*.js text eol=lf' in text:
        raise RuntimeError('Control UI LF contract already exists unexpectedly')
    write(path, text.rstrip('\n') + '\n' + additions)


def patch_build_rs() -> None:
    path = 'codex-rs/hepta-core/build.rs'
    text = (ROOT / path).read_text(encoding='utf-8')
    text = replace_once(
        text,
        '    validate_sha256_implementation()?;\n\n'
        '    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);\n',
        '    validate_sha256_implementation()?;\n'
        '    validate_control_ui_line_ending_canonicalization()?;\n\n'
        '    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);\n',
        'build.rs canonicalization self-test call',
    )
    text = replace_once(
        text,
        '    let base_js = fs::read(base_js_path)?;\n'
        '    let runtime_js = fs::read(runtime_js_path)?;\n',
        '    let base_js = canonicalize_control_ui_line_endings(&fs::read(base_js_path)?)?;\n'
        '    let runtime_js = canonicalize_control_ui_line_endings(&fs::read(runtime_js_path)?)?;\n',
        'build.rs canonical inputs',
    )
    implementation = r'''fn validate_control_ui_line_ending_canonicalization() -> Result<(), Box<dyn Error>> {
    let canonical = canonicalize_control_ui_line_endings(b"alpha\r\nbeta\ngamma\r\n")?;
    if canonical != b"alpha\nbeta\ngamma\n" {
        return Err("Control UI line-ending canonicalization failed its known-answer test".into());
    }
    if canonicalize_control_ui_line_endings(b"lone\rcarriage-return").is_ok() {
        return Err("Control UI canonicalization accepted a lone carriage return".into());
    }
    Ok(())
}

fn canonicalize_control_ui_line_endings(input: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut output = Vec::with_capacity(input.len());
    let mut index = 0;
    while index < input.len() {
        if input[index] != b'\r' {
            output.push(input[index]);
            index += 1;
            continue;
        }
        if input.get(index + 1) != Some(&b'\n') {
            return Err("Control UI source contains a lone carriage return".into());
        }
        output.push(b'\n');
        index += 2;
    }
    Ok(output)
}

'''
    text = replace_once(
        text,
        'fn sha256_hex(input: &[u8]) -> String {\n',
        implementation + 'fn sha256_hex(input: &[u8]) -> String {\n',
        'build.rs canonicalization implementation',
    )
    write(path, text)


def patch_browser_workflow() -> None:
    path = '.github/workflows/hepta-ui-v4-rust-served-cross-browser-final.yml'
    text = (ROOT / path).read_text(encoding='utf-8')
    text = ensure_pr_path(text, '.gitattributes', anchor='apps/hepta-control-ui/**')
    old_config = (
        "      - name: Enable Windows long paths before checkout\n"
        "        if: runner.os == 'Windows'\n"
        "        shell: pwsh\n"
        "        run: git config --global core.longpaths true\n"
    )
    new_config = (
        "      - name: Configure deterministic Windows checkout\n"
        "        if: runner.os == 'Windows'\n"
        "        shell: pwsh\n"
        "        run: |\n"
        "          git config --global core.longpaths true\n"
        "          git config --global core.autocrlf false\n"
        "          git config --global core.eol lf\n"
    )
    text = replace_once(text, old_config, new_config, 'browser Windows checkout')
    old_verify = (
        "      - name: Verify exact candidate and bind tree\n"
        "        shell: bash\n"
        "        run: |\n"
        "          set -euo pipefail\n"
        '          test "$(git rev-parse HEAD)" = "$HEPTA_CANDIDATE_SHA"\n'
        '          echo "HEPTA_CANDIDATE_TREE=$(git rev-parse \'HEAD^{tree}\')" >> "$GITHUB_ENV"\n'
    )
    new_verify = (
        "      - name: Verify exact candidate, LF contract, and bind tree\n"
        "        shell: bash\n"
        "        run: |\n"
        "          set -euo pipefail\n"
        '          test "$(git rev-parse HEAD)" = "$HEPTA_CANDIDATE_SHA"\n'
        "          for path in \\\n"
        "            apps/hepta-control-ui/index.html \\\n"
        "            apps/hepta-control-ui/styles.css \\\n"
        "            apps/hepta-control-ui/control-ui.js \\\n"
        "            apps/hepta-control-ui/control-ui-v4-runtime.js; do\n"
        '            git check-attr eol -- "$path" | grep -Fqx "$path: eol: lf"\n'
        "          done\n"
        '          echo "HEPTA_CANDIDATE_TREE=$(git rev-parse \'HEAD^{tree}\')" >> "$GITHUB_ENV"\n'
    )
    text = replace_once(text, old_verify, new_verify, 'browser LF verification')
    write(path, text)


def patch_lifecycle() -> None:
    path = LIFECYCLE
    text = (ROOT / path).read_text(encoding='utf-8')
    text = replace_once(
        text,
        '#[derive(Script, Widget)]',
        '#[derive(Script, ScriptHook, Widget)]',
        'Makepad ScriptHook derive',
    )
    write(path, text)


def patch_exact_gate() -> None:
    path = 'scripts/hepta-ui-v4-exact-candidate-materialization-gate'
    text = (ROOT / path).read_text(encoding='utf-8')
    text = replace_once(
        text,
        'checks["workflow.pipefail"] = workflow.scan("set -o pipefail").length >= 3',
        'checks["workflow.pipefail"] = workflow.scan(/set\\s+-[A-Za-z]*o\\s+pipefail/).length >= 3',
        'exact materialization pipefail semantics',
    )
    write(path, text)


def patch_dwm() -> None:
    path = '.github/workflows/hepta-ui-v4-windows-dwm-ack-producer.yml'
    text = (ROOT / path).read_text(encoding='utf-8')
    text = ensure_pr_path(text, LIFECYCLE, anchor='apps/hepta-native/src/shared/hepta_window_visual_ack.rs')
    text = update_concurrency(text, path)
    old = (
        "      - name: Enable Windows long paths before checkout\n"
        "        if: runner.os == 'Windows'\n"
        "        shell: pwsh\n"
        "        run: git config --global core.longpaths true\n"
    )
    new = (
        "      - name: Configure deterministic Windows checkout\n"
        "        if: runner.os == 'Windows'\n"
        "        shell: pwsh\n"
        "        run: |\n"
        "          git config --global core.longpaths true\n"
        "          git config --global core.autocrlf false\n"
        "          git config --global core.eol lf\n"
    )
    text = replace_once(text, old, new, 'DWM Windows checkout')
    write(path, text)


def main() -> None:
    patch_gitattributes()
    patch_build_rs()
    patch_browser_workflow()
    patch_lifecycle()
    patch_exact_gate()
    harden_native_workflow(
        '.github/workflows/hepta-ui-v4-windows-material-profile-runtime.yml',
        'apps/hepta-native/src/bin/hepta-ui-v4-windows-material-profile-probe.rs',
    )
    harden_native_workflow(
        '.github/workflows/hepta-ui-v4-windows-material-profile-aggregate.yml',
        'apps/hepta-native/src/shared/hepta_windows_material_profile_aggregate.rs',
    )
    harden_native_workflow(
        '.github/workflows/hepta-ui-v4-windows-material-profile-exact.yml',
        'apps/hepta-native/src/shared/hepta_window_visual_ack.rs',
    )
    patch_dwm()

    modified = [
        '.gitattributes',
        'codex-rs/hepta-core/build.rs',
        '.github/workflows/hepta-ui-v4-rust-served-cross-browser-final.yml',
        LIFECYCLE,
        'scripts/hepta-ui-v4-exact-candidate-materialization-gate',
        '.github/workflows/hepta-ui-v4-windows-material-profile-runtime.yml',
        '.github/workflows/hepta-ui-v4-windows-material-profile-aggregate.yml',
        '.github/workflows/hepta-ui-v4-windows-material-profile-exact.yml',
        '.github/workflows/hepta-ui-v4-windows-dwm-ack-producer.yml',
    ]
    output = Path(os.environ.get('HEPTA_MODIFIED_PATHS_OUT', str(ROOT / 'hepta-ui-v4-modified-paths.json')))
    output.write_text(json.dumps(modified, indent=2) + '\n', encoding='utf-8')
    print('\n'.join(modified))


if __name__ == '__main__':
    main()
