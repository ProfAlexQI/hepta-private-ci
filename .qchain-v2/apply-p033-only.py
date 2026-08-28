#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

BUNDLE = Path('/tmp/hepta-qchain-v2/hepta_kg_p0_3_3_qualification_v2')
sys.path.insert(0, str(BUNDLE))
from apply_qualification_chain_v2 import (  # noqa: E402
    P033_BRANCH,
    copy_payload,
    git,
    update_pre_restack_status,
    validate_static,
    verify_blob,
)
from patch_logic import patch_verifier  # noqa: E402

EXPECTED_HEAD = '30e9202f5b37ca2fca1f32866a98579cd5ae1057'
WORKFLOW = '.github/workflows/hepta-intelligence-evidence-resolver-v4.yml'
VERIFIER = 'scripts/verify-hepta-intelligence-evidence-resolver-v4.py'
SCOPED_CLIPPY = 'scripts/check-hepta-intelligence-p0-3-3-clippy.py'
RUNNER = 'scripts/run-hepta-intelligence-evidence-resolver-v5.py'
STATUS = 'plans/hepta-intelligence/HEPTA_INTELLIGENCE_KG_EXECUTION_STATUS_V3_2.json'


def align_v5_workflow_gate(text: str) -> str:
    start_marker = '    checks["workflow.repository_toolchain_and_scoped_fmt"] = ('
    end_marker = '    checks["resolver.contract"] = contains_all('
    start = text.find(start_marker)
    end = text.find(end_marker, start)
    if start < 0 or end < 0:
        raise RuntimeError('P0.3.3 workflow source-gate block drift')
    replacement = '''    checks["workflow.repository_toolchain_and_scoped_fmt"] = (
        contains_all(
            workflow,
            [
                "name: hepta-intelligence-evidence-resolver-v5",
                "toolchain: 1.95.0",
                "scripts/run-hepta-intelligence-evidence-resolver-v5.py",
                "scripts/check-hepta-intelligence-p0-3-3-clippy.py",
                "github.event_name == 'pull_request'",
                "P0.3.3 exact-head host evidence resolver qualification v5",
            ],
        )
        and "cargo fmt --all -- --check" not in workflow
        and "toolchain: 1.88.0" not in workflow
    )
'''
    return text[:start] + replacement + text[end:]


def main() -> int:
    repo = Path(sys.argv[1]).resolve()
    if git(repo, 'status', '--porcelain'):
        raise RuntimeError('root checkout must be clean')
    git(
        repo,
        'fetch',
        'origin',
        f'+refs/heads/{P033_BRANCH}:refs/remotes/origin/{P033_BRANCH}',
        capture=False,
    )
    observed = git(repo, 'rev-parse', f'origin/{P033_BRANCH}')
    if observed != EXPECTED_HEAD:
        raise RuntimeError(
            f'P0.3.3 exact-head drift: expected {EXPECTED_HEAD}, observed {observed}'
        )

    temp = Path(tempfile.mkdtemp(prefix='hepta-p033-qchain-v2-'))
    worktree = temp / 'p033'
    try:
        git(repo, 'worktree', 'add', '--detach', str(worktree), f'origin/{P033_BRANCH}', capture=False)
        verify_blob(worktree, P033_BRANCH, WORKFLOW)
        verify_blob(worktree, P033_BRANCH, VERIFIER)
        verify_blob(worktree, P033_BRANCH, STATUS)

        copy_payload(BUNDLE, worktree, WORKFLOW)
        copy_payload(BUNDLE, worktree, SCOPED_CLIPPY)
        copy_payload(BUNDLE, worktree, RUNNER)
        verifier = worktree / VERIFIER
        patched = patch_verifier(verifier.read_text(encoding='utf-8'))
        verifier.write_text(align_v5_workflow_gate(patched), encoding='utf-8')
        update_pre_restack_status(worktree / STATUS)

        # Validate the complete final tree, including the v5 workflow, before
        # publishing any part of it.
        validate_static(worktree, True)
        git(worktree, 'diff', '--check', capture=False)
        workflow_sha256 = hashlib.sha256((worktree / WORKFLOW).read_bytes()).hexdigest()

        # The Actions GITHUB_TOKEN cannot update workflow files. Publish only
        # the already-validated non-workflow source/status changes here; the
        # connector publishes the exact validated workflow payload next.
        git(worktree, 'add', VERIFIER, SCOPED_CLIPPY, RUNNER, STATUS, capture=False)
        git(worktree, 'config', 'user.name', 'Qian QI', capture=False)
        git(worktree, 'config', 'user.email', '102159240+ProfAlexQI@users.noreply.github.com', capture=False)
        git(
            worktree,
            'commit',
            '--no-gpg-sign',
            '-m',
            'ci(memory): make P0.3.3 qualification dependency-aware',
            capture=False,
        )
        new_head = git(worktree, 'rev-parse', 'HEAD')
        git(
            repo,
            'push',
            f'--force-with-lease=refs/heads/{P033_BRANCH}:{EXPECTED_HEAD}',
            'origin',
            f'{new_head}:refs/heads/{P033_BRANCH}',
            capture=False,
        )
        print(json.dumps({
            'schema': 'hepta_p033_qchain_v2_source_publish_result',
            'old_head': EXPECTED_HEAD,
            'new_head': new_head,
            'workflow_sha256': workflow_sha256,
            'workflow_publication_pending': True,
            'pushed': True,
            'qualified': False,
            'production_authority': False,
        }, indent=2, sort_keys=True))
    finally:
        subprocess.run(['git', 'worktree', 'remove', '--force', str(worktree)], cwd=repo)
        shutil.rmtree(temp, ignore_errors=True)
    return 0


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except Exception as error:
        print(f'FAIL_CLOSED: {error}', file=sys.stderr)
        raise SystemExit(1)
