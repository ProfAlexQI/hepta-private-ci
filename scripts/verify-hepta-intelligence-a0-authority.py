"""Fail-closed A0 source and executable-evidence verifier."""
from __future__ import annotations
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import stat
import subprocess
import sys
import time
from typing import Any, NoReturn
from urllib.request import Request, urlopen
import zipfile
ROOT = Path(__file__).resolve().parents[1]
P = ROOT / 'plans/hepta-intelligence'
CURRENT = P / 'HEPTA_INTELLIGENCE_CURRENT_PLAN.json'
INTEGRATION = P / 'HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json'
Q0 = P / 'HEPTA_INTELLIGENCE_Q0_QUALIFICATION_RECEIPT_2026-08-29.json'
TRUTH = ROOT / 'scripts/hepta-intelligence-current-truth.py'
WORKFLOW = ROOT / '.github/workflows/hepta-intelligence-a0-authority.yml'
WORKFLOW_REL = WORKFLOW.relative_to(ROOT).as_posix()
REPO = 'ProfHepta/hepta-private-ci'
BRANCH = 'codex/hepta-intelligence-a0-authority-gap-closure-20260829'
PARENT = 'c768bcbeb4c1168088d2499828c24da521a2a73a'
Q0_TREE = 'ca455a9ef797cd95164c880c7b8faba80b305589'
HEX40 = re.compile('[0-9a-f]{40}')
SHA256 = re.compile('sha256:[0-9a-f]{64}')
SOURCE_JOB = 'A0 canonical authority source gate'
PRODUCER_JOB = 'A0 exact-head executable evidence producer'
SOURCE_STEPS = ['Checkout exact source candidate', 'Verify repository, branch, head, tree and parent', 'Compile A0 Python tools', 'Parse registered machine inputs', 'Verify canonical master plan', 'Verify document authority and compatibility', 'Verify deterministic current truth', 'Run exact-parent A0 source gate', 'Enforce negative authority source receipt', 'Prove clean tracked source', 'Upload exact-head A0 source evidence']
PRODUCER_STEPS = ['Checkout exact source candidate for evidence production', 'Verify evidence producer identity', 'Produce digest-verified executable evidence', 'Prove producer left tracked source clean', 'Upload exact-head A0 executable producer evidence']
GAPS = {'A0-HEAD-TRUTH-001', 'A0-DOC-MM-001', 'A0-TRIGGER-001', 'A0-EVIDENCE-001', 'A0-REVIEW-001', 'A0-SELECT-001', 'A0-MERGE-001', 'B0-BOUNDARY-001', 'B0-SCOPE-001', 'M0-COORDINATOR-001', 'J0-LIFECYCLE-001', 'MM0-ASSET-CONTRACT-001', 'MM0-SEGMENT-CONTRACT-001', 'MM0-DERIVATIVE-CONTRACT-001', 'MM0-MODEL-CONTRACT-001', 'MM0-EMBEDDING-SPACE-001', 'MM0-QUERY-REVALIDATION-001', 'MM0-DELETION-CONTRACT-001', 'MM1-OBJECT-STORE-001', 'MM1-DERIVATION-GRAPH-001', 'MM1-MIGRATION-001', 'MM1-QUOTA-BACKPRESSURE-001', 'MM2-SANDBOX-001', 'MM2-EXTRACTOR-001', 'MM2-PRIVACY-001', 'MM3-LOCAL-MODEL-001', 'MM3-TYPED-PROVIDER-001', 'MM3-INDEX-001', 'MM4-FUSION-001', 'MM4-RERANK-001', 'MM4-REVALIDATE-001', 'MM5-CONSOLIDATION-001', 'MM5-CONTEXT-001', 'MM6-UNLEARNING-001', 'MM6-SECURITY-001', 'MM6-EFFICACY-001', 'MM6-HARDWARE-001', 'MM6-SOAK-001', 'MM6-OPERATOR-001', 'R1-RETRIEVAL-001', 'R1-CORPUS-001', 'C0-LEDGER-001'}
REPO_STATES = {'CLOSED_SOURCE_CONTROLLED', 'OPEN_SOURCE_CONTROLLED', 'BLOCKED_UPSTREAM', 'BLOCKED_EXTERNAL_EVIDENCE', 'STOP_CONDITION'}
PAYLOAD_STATES = {'PR_UPDATE_AFTER_HEAD', 'SOURCE_AWAITING_EVIDENCE', 'PLANNED_BLOCKED', 'EXTERNAL_REQUIRED'}
EXTERNAL = {'A0 executable qualification'}

def fail(message: str) -> NoReturn:
    raise SystemExit(f'FAIL_HEPTA_INTELLIGENCE_A0_AUTHORITY: {message}')

def need(condition: bool, message: str) -> None:
    if not condition:
        fail(message)

def load(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding='utf-8'))
    except Exception as exc:
        fail(f'cannot parse {path.relative_to(ROOT)}: {exc}')
    need(isinstance(value, dict), f'{path.relative_to(ROOT)} must contain an object')
    return value

def canon(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode()

def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()

def all_false(value: Any) -> bool:
    return isinstance(value, dict) and bool(value) and all((v is False for v in value.values()))

def git(*args: str) -> str:
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True).strip()

def identity() -> tuple[str, str, str]:
    head, tree, parent = (git('rev-parse', 'HEAD'), git('rev-parse', 'HEAD^{tree}'), git('rev-parse', 'HEAD^'))
    need(HEX40.fullmatch(head) is not None and HEX40.fullmatch(tree) is not None, 'invalid commit identity')
    need(parent == PARENT, f'exact parent mismatch: {parent}')
    need(git('rev-parse', f'HEAD:{WORKFLOW_REL}') == git('hash-object', WORKFLOW_REL), 'workflow blob mismatch')
    need(os.environ.get('GITHUB_REPOSITORY', REPO) == REPO, 'repository mismatch')
    expected = os.environ.get('EXPECTED_HEAD_SHA') or os.environ.get('GITHUB_SHA')
    need(not expected or expected == head, 'environment head mismatch')
    branch = os.environ.get('GITHUB_HEAD_REF') or os.environ.get('GITHUB_REF_NAME') or BRANCH
    need(branch == BRANCH, f'branch mismatch: {branch}')
    return (head, tree, parent)

def validate_ledger(integration: dict[str, Any]) -> None:
    ledger = integration.get('multimodal_memory_gap_ledger')
    need(isinstance(ledger, dict), 'multimodal gap ledger missing')
    need(ledger.get('schema') == 'hepta_multimodal_memory_gap_ledger_v2', 'gap schema')
    need(ledger.get('version') == '2.0.0' and ledger.get('repository') == REPO, 'gap identity')
    need(ledger.get('as_of_date') == '2026-08-30', 'gap audit date')
    need(set(ledger.get('allowed_repository_statuses', [])) == REPO_STATES, 'gap status vocabulary')
    q0 = ledger.get('audited_q0', {})
    need(q0.get('head') == PARENT and q0.get('tree') == Q0_TREE, 'gap audited Q0')
    need(q0.get('parent') == 'aeb8ac0bfb30d570a16c4914b6e4b31ce035dd62', 'gap audited Q0 parent')
    need(all_false(ledger.get('authority')), 'gap ledger authority')
    claim = ledger.get('claim_boundary', {})
    need(claim.get('multimodal_memory') == 'MM0_SPECIFIED_ONLY', 'multimodal claim phase')
    for key in ('cross_modal_retrieval_qualified', 'full_repository_merge_green', 'multimodal_efficacy_proven', 'native_media_memory_wired', 'production_authority'):
        need(claim.get(key) is False, f'positive multimodal claim: {key}')
    snapshot = ledger.get('source_snapshot', {})
    need(snapshot.get('candidate_branch') == BRANCH, 'gap source branch')
    need(snapshot.get('classification') == 'SOURCE_CONTRACT_NOT_LIVE_EVIDENCE', 'gap source classification')
    need(snapshot.get('candidate_head') == snapshot.get('candidate_tree') == 'BOUND_AT_EXECUTABLE_RECEIPT', 'gap source binding')
    need(snapshot.get('live_status_embedded') is False, 'live gap status embedded')
    contract = ledger.get('entry_contract', {})
    need(contract.get('unknown_gap') == 'FAIL_CLOSED' and contract.get('dependency_field') == 'blocked_by', 'gap contract')
    need(set(contract.get('required_fields', [])) == {'gap_id', 'repository_status', 'payload_status'}, 'gap required fields')
    entries = ledger.get('entries')
    need(isinstance(entries, list) and len(entries) == 42 and all((isinstance(e, dict) for e in entries)), 'gap entries')
    ids = [e.get('gap_id') for e in entries]
    need(len(ids) == len(set(ids)) == 42 and set(ids) == GAPS, 'gap surface drift')
    graph: dict[str, list[str]] = {}
    for entry in entries:
        gap = entry['gap_id']
        rs, ps = (entry.get('repository_status'), entry.get('payload_status'))
        need(rs in REPO_STATES and ps in PAYLOAD_STATES, f'{gap}: invalid status')
        if ps == 'EXTERNAL_REQUIRED':
            need(rs == 'BLOCKED_EXTERNAL_EVIDENCE', f'{gap}: external classification')
        elif ps == 'PLANNED_BLOCKED':
            need(rs == 'BLOCKED_UPSTREAM', f'{gap}: upstream classification')
        else:
            need(rs in {'OPEN_SOURCE_CONTROLLED', 'CLOSED_SOURCE_CONTROLLED'}, f'{gap}: source classification')
        deps = entry.get('blocked_by', [])
        need(isinstance(deps, list) and len(deps) == len(set(deps)), f'{gap}: dependencies')
        graph[gap] = []
        for dep in deps:
            need(isinstance(dep, str) and dep and (dep != gap), f'{gap}: malformed dependency')
            need(dep in GAPS or dep in EXTERNAL, f'{gap}: unknown dependency {dep}')
            if dep in GAPS:
                graph[gap].append(dep)
    visiting: set[str] = set()
    visited: set[str] = set()

    def visit(gap: str) -> None:
        need(gap not in visiting, f'dependency cycle at {gap}')
        if gap in visited:
            return
        visiting.add(gap)
        for dep in graph[gap]:
            visit(dep)
        visiting.remove(gap)
        visited.add(gap)
    for gap in sorted(graph):
        visit(gap)

def validate_workflow() -> None:
    text = WORKFLOW.read_text(encoding='utf-8')
    for marker in ('permissions:\n  actions: read\n  contents: read', 'github.event_name', 'evidence-producer:', 'executable-receipt:', SOURCE_JOB, PRODUCER_JOB, '--produce-executable-evidence', '--attest-executable-evidence', 'PASS_HEPTA_INTELLIGENCE_A0_EXECUTABLE_QUALIFICATION'):
        need(marker in text, f'workflow marker missing: {marker}')
    for marker in ('unzip -q', 'contents: write', 'actions: write', 'git push', 'git commit', 'git update-ref'):
        need(marker not in text, f'workflow forbidden marker: {marker}')

def source_receipt() -> dict[str, Any]:
    for path in (CURRENT, INTEGRATION, Q0, TRUTH, WORKFLOW):
        need(path.is_file(), f'missing {path.relative_to(ROOT)}')
    current, integration, q0 = (load(CURRENT), load(INTEGRATION), load(Q0))
    need(current.get('repository') == REPO and current.get('active_phase', {}).get('id') == 'A0', 'current plan identity')
    truth = current.get('current_truth', {})
    need(truth.get('q0_executable_qualified') is True and truth.get('qualified_candidate') is True, 'Q0/current qualification')
    need(truth.get('wired') is False and truth.get('qualified') is False and (truth.get('full_repository_merge_green') is False), 'current positive claim')
    need(all_false(current.get('authority')), 'current authority')
    observed = q0.get('evidence_observation', {})
    need(observed.get('head') == PARENT and observed.get('tree') == Q0_TREE and (observed.get('run_id') == 33252922404), 'Q0 observation')
    jobs, artifacts = (observed.get('jobs'), observed.get('artifacts'))
    need(isinstance(jobs, list) and len(jobs) == 3 and all((j.get('runner_id', 0) > 0 and j.get('steps_non_empty') is True for j in jobs)), 'Q0 jobs')
    need(isinstance(artifacts, list) and len(artifacts) == 3 and all((SHA256.fullmatch(str(a.get('digest', ''))) for a in artifacts)), 'Q0 artifacts')
    need(q0.get('conclusion', {}).get('q0_executable_qualified') is True and q0.get('conclusion', {}).get('runtime_capability_qualified') is False, 'Q0 conclusion')
    need(all_false(q0.get('authority')), 'Q0 authority')
    bound = dict(q0)
    observed_binding = bound.pop('receipt_binding_sha256', None)
    need(observed_binding == hashlib.sha256(canon(bound)).hexdigest(), 'Q0 receipt binding')
    need(integration.get('repository') == REPO and integration.get('branch') == BRANCH and (integration.get('expected_parent') == PARENT), 'integration identity')
    allowed = integration.get('allowed_changed_paths')
    need(isinstance(allowed, list) and allowed == sorted(set(allowed)) and (len(allowed) == 17), 'integration allowlist')
    need(integration.get('expected_changed_path_count') == 17, 'integration path count')
    need(not any((p.startswith(('codex-rs/', 'migrations/', 'sdk/', 'shell-tool-mcp/')) for p in allowed)), 'runtime path in allowlist')
    freeze = integration.get('source_freeze', {})
    need(freeze.get('rust_runtime_changes_allowed') is False and freeze.get('sql_migrations_allowed') is False and (freeze.get('product_callers_allowed') is False), 'source freeze')
    need(all_false(integration.get('authority')), 'integration authority')
    validate_ledger(integration)
    validate_workflow()
    first = subprocess.check_output([sys.executable, str(TRUTH), '--compact'], cwd=ROOT)
    second = subprocess.check_output([sys.executable, str(TRUTH), '--compact'], cwd=ROOT)
    need(first == second, 'current truth not deterministic')
    truth_doc = json.loads(first)
    need(truth_doc.get('q0', {}).get('qualified_candidate') is True and truth_doc.get('q0', {}).get('runtime_capability_qualified') is False, 'current truth boundary')
    need(all_false(truth_doc.get('authority')), 'current truth authority')
    head = tree = workflow_blob = None
    changed = allowed
    if (ROOT / '.git').exists():
        head, tree, _ = identity()
        changed = sorted((p for p in git('diff', '--name-only', 'HEAD^', 'HEAD').splitlines() if p))
        need(changed == allowed, f'changed-path mismatch: {changed}')
        subprocess.check_call(['git', 'diff', '--check', 'HEAD^', 'HEAD'], cwd=ROOT)
        workflow_blob = git('rev-parse', f'HEAD:{WORKFLOW_REL}')
    return {'schema': 'hepta_intelligence_a0_source_gate_receipt_v2', 'status': 'PASS_HEPTA_INTELLIGENCE_A0_SOURCE_ONLY', 'repository': REPO, 'candidate': {'branch': BRANCH, 'head': head, 'tree': tree, 'parent': PARENT}, 'workflow': {'path': WORKFLOW_REL, 'git_blob_sha': workflow_blob, 'content_sha256': sha(WORKFLOW)}, 'q0_base': {'head': PARENT, 'tree': Q0_TREE, 'run_id': 33252922404, 'qualified_candidate': True}, 'changed_files': changed, 'changed_files_sha256': hashlib.sha256(('\n'.join(changed) + '\n').encode()).hexdigest(), 'current_truth_sha256': hashlib.sha256(first).hexdigest(), 'multimodal_gap_count': 42, 'runtime_source_changed': False, 'sql_migration_changed': False, 'product_caller_changed': False, 'source_writeback': False, 'a0_candidate_qualified': False, 'full_repository_merge_green': False, 'authority': current['authority']}

def api(path_or_url: str) -> bytes:
    token = os.environ.get('GH_TOKEN')
    need(bool(token), 'GH_TOKEN missing')
    url = path_or_url if path_or_url.startswith('https://') else f'https://api.github.com/repos/{REPO}{path_or_url}'
    req = Request(url, headers={'Authorization': f'Bearer {token}', 'Accept': 'application/vnd.github+json', 'X-GitHub-Api-Version': '2022-11-28', 'User-Agent': 'hepta-a0-evidence-v2'})
    with urlopen(req, timeout=60) as response:
        return response.read()

def api_json(path: str) -> dict[str, Any]:
    value = json.loads(api(path))
    need(isinstance(value, dict), f'API object expected: {path}')
    return value

def one(items: Any, field: str, value: Any, label: str) -> dict[str, Any]:
    need(isinstance(items, list), f'{label} collection missing')
    found = [x for x in items if isinstance(x, dict) and x.get(field) == value]
    need(len(found) == 1, f'{label} not unique: {value}')
    return found[0]

def validate_job(job: dict[str, Any], run_id: int, required: list[str]) -> None:
    need(job.get('run_id') == run_id and job.get('status') == 'completed' and (job.get('conclusion') == 'success'), f"job failed: {job.get('name')}")
    need(int(job.get('runner_id') or 0) > 0 and bool(job.get('runner_name')) and bool(job.get('labels')), f"job has no runner: {job.get('name')}")
    steps = job.get('steps')
    need(isinstance(steps, list) and steps, f"job has no steps: {job.get('name')}")
    by_name = {s.get('name'): s for s in steps if isinstance(s, dict)}
    for name in required:
        step = by_name.get(name)
        need(isinstance(step, dict) and step.get('status') == 'completed' and (step.get('conclusion') == 'success'), f'step failed: {name}')

def validate_artifact(artifact: dict[str, Any], run_id: int, head: str) -> None:
    need(artifact.get('expired') is False and isinstance(artifact.get('expires_at'), str), 'artifact expired')
    need(SHA256.fullmatch(str(artifact.get('digest', ''))) is not None, 'artifact digest')
    run = artifact.get('workflow_run', {})
    need(run.get('id') == run_id and run.get('head_sha') == head and (run.get('head_branch') == BRANCH), 'artifact run binding')

def save(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + '\n', encoding='utf-8')

def extract(artifact: dict[str, Any], archive: Path, output: Path, manifest: Path) -> None:
    archive.parent.mkdir(parents=True, exist_ok=True)
    archive.write_bytes(api(str(artifact['archive_download_url'])))
    observed = sha(archive)
    need(artifact['digest'] == f'sha256:{observed}', 'artifact archive digest mismatch')
    output = output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    entries: list[dict[str, Any]] = []
    seen: set[str] = set()
    total = 0
    with zipfile.ZipFile(archive) as bundle:
        infos = bundle.infolist()
        need(0 < len(infos) <= 96, 'artifact entry count')
        for info in infos:
            raw = info.filename
            need(raw and '\x00' not in raw and ('\\' not in raw), 'invalid artifact entry')
            rel = PurePosixPath(raw)
            need(not rel.is_absolute() and rel.parts and all((p not in {'', '.', '..'} for p in rel.parts)), 'artifact path traversal')
            name = rel.as_posix().rstrip('/')
            need(name and name not in seen, 'duplicate artifact entry')
            seen.add(name)
            mode = info.external_attr >> 16 & 65535
            need(not stat.S_ISLNK(mode) and (not info.flag_bits & 1), 'unsafe artifact entry')
            target = output.joinpath(*rel.parts).resolve()
            need(os.path.commonpath([str(output), str(target)]) == str(output), 'artifact target escaped')
            if info.is_dir():
                target.mkdir(parents=True, exist_ok=True)
                continue
            need(info.file_size <= 4 * 1024 * 1024, 'artifact entry too large')
            total += info.file_size
            need(total <= 24 * 1024 * 1024, 'artifact too large')
            data = bundle.read(info)
            need(len(data) == info.file_size and (not target.exists()), 'artifact entry mismatch')
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
            entries.append({'path': name, 'size': len(data), 'sha256': hashlib.sha256(data).hexdigest()})
    save(manifest, {'schema': 'hepta_intelligence_a0_archive_manifest_v2', 'artifact_id': artifact['id'], 'artifact_name': artifact['name'], 'artifact_digest': artifact['digest'], 'artifact_expires_at': artifact['expires_at'], 'archive_sha256': observed, 'entry_count': len(entries), 'total_uncompressed_bytes': total, 'entries': sorted(entries, key=lambda x: x['path'])})

def poll(required_jobs: list[tuple[str, list[str]]], artifact_names: list[str], api_dir: Path) -> tuple[dict[str, Any], dict[str, Any], dict[str, Any]]:
    run_id = int(os.environ['GITHUB_RUN_ID'])
    head = os.environ['EXPECTED_HEAD_SHA']
    last = None
    for _ in range(20):
        run = api_json(f'/actions/runs/{run_id}')
        jobs = api_json(f'/actions/runs/{run_id}/jobs?filter=latest&per_page=100')
        artifacts = api_json(f'/actions/runs/{run_id}/artifacts?per_page=100')
        last = (run, jobs, artifacts)
        try:
            need(run.get('id') == run_id and run.get('run_attempt') == int(os.environ['GITHUB_RUN_ATTEMPT']), 'run identity')
            need(run.get('head_sha') == head and run.get('head_branch') == BRANCH and (run.get('path') == WORKFLOW_REL), 'run binding')
            need(run.get('event') in {'push', 'pull_request', 'workflow_dispatch'}, 'run event')
            for name, steps in required_jobs:
                validate_job(one(jobs.get('jobs'), 'name', name, 'job'), run_id, steps)
            for name in artifact_names:
                validate_artifact(one(artifacts.get('artifacts'), 'name', name, 'artifact'), run_id, head)
            break
        except SystemExit:
            time.sleep(3)
    else:
        fail('predecessor evidence did not become executable and durable')
    run, jobs, artifacts = last
    save(api_dir / 'run.json', run)
    save(api_dir / 'jobs.json', jobs)
    save(api_dir / 'artifacts.json', artifacts)
    return (run, jobs, artifacts)

def unique(root: Path, name: str) -> Path:
    found = list(root.rglob(name))
    need(len(found) == 1, f'{name} missing or ambiguous')
    return found[0]

def verify_binding(receipt: dict[str, Any], label: str) -> None:
    copy = dict(receipt)
    observed = copy.pop('receipt_binding_sha256', None)
    need(observed == hashlib.sha256(canon(copy)).hexdigest(), f'{label} receipt binding')

def produce(root: Path) -> int:
    head, tree, parent = identity()
    api_dir, source_dir, out = (root / 'api', root / 'source', root / 'producer')
    source_name = os.environ['SOURCE_ARTIFACT_NAME']
    run, jobs, artifacts = poll([(SOURCE_JOB, SOURCE_STEPS)], [source_name], api_dir)
    run_id = int(os.environ['GITHUB_RUN_ID'])
    source_job = one(jobs['jobs'], 'name', SOURCE_JOB, 'source job')
    source_artifact = one(artifacts['artifacts'], 'name', source_name, 'source artifact')
    extract(source_artifact, api_dir / 'source-artifact.zip', source_dir, api_dir / 'source-archive-manifest.json')
    source_path, truth_path = (unique(source_dir, 'a0-source-gate-receipt.json'), unique(source_dir, 'current-truth.json'))
    source = load(source_path)
    truth = load(truth_path)
    candidate = {'branch': BRANCH, 'head': head, 'parent': parent, 'tree': tree}
    need(source.get('status') == 'PASS_HEPTA_INTELLIGENCE_A0_SOURCE_ONLY' and source.get('candidate') == candidate, 'source receipt')
    need(source.get('workflow', {}).get('git_blob_sha') == git('rev-parse', f'HEAD:{WORKFLOW_REL}') and source.get('workflow', {}).get('content_sha256') == sha(WORKFLOW), 'source workflow binding')
    need(source.get('a0_candidate_qualified') is False and all_false(source.get('authority')), 'source authority boundary')
    need(truth.get('q0', {}).get('qualified_candidate') is True and truth.get('q0', {}).get('runtime_capability_qualified') is False and all_false(truth.get('authority')), 'source truth boundary')
    manifest = load(api_dir / 'source-archive-manifest.json')
    receipt = {'schema': 'hepta_intelligence_a0_executable_producer_receipt_v2', 'status': 'PASS_HEPTA_INTELLIGENCE_A0_EXECUTABLE_PRODUCER_ONLY', 'repository': REPO, 'candidate': candidate, 'workflow': {'workflow_id': run.get('workflow_id'), 'workflow_name': run.get('name'), 'workflow_path': WORKFLOW_REL, 'workflow_git_blob_sha': git('rev-parse', f'HEAD:{WORKFLOW_REL}'), 'workflow_content_sha256': sha(WORKFLOW), 'run_id': run_id, 'run_attempt': run.get('run_attempt'), 'event': run.get('event'), 'source_job_id': source_job.get('id'), 'source_runner_id': source_job.get('runner_id'), 'source_runner_name': source_job.get('runner_name'), 'source_runner_labels': source_job.get('labels'), 'source_artifact_id': source_artifact.get('id'), 'source_artifact_name': source_name, 'source_artifact_digest': source_artifact.get('digest'), 'source_artifact_expires_at': source_artifact.get('expires_at'), 'source_archive_digest_verified': True}, 'evidence': {'source_receipt_sha256': sha(source_path), 'current_truth_sha256': sha(truth_path), 'source_archive_sha256': manifest['archive_sha256'], 'source_archive_manifest_sha256': sha(api_dir / 'source-archive-manifest.json'), 'run_metadata_sha256': sha(api_dir / 'run.json'), 'jobs_metadata_sha256': sha(api_dir / 'jobs.json'), 'artifacts_metadata_sha256': sha(api_dir / 'artifacts.json')}, 'conclusion': {'q0_base_qualified_candidate': True, 'a0_source_implemented': True, 'a0_candidate_qualified': False, 'terminal_attestation_required': True, 'runtime_wired': False, 'runtime_capability_qualified': False, 'efficacy_proven': False, 'operator_accepted': False, 'promoted': False, 'full_repository_merge_green': False, 'b0_activation_requires_independent_review_and_canonical_selection': True}, 'authority': source['authority']}
    receipt['receipt_binding_sha256'] = hashlib.sha256(canon(receipt)).hexdigest()
    save(out / 'a0-executable-producer-receipt.json', receipt)
    verify_binding(receipt, 'producer')
    print(json.dumps({'status': receipt['status'], 'head': head, 'artifact_digest_verified': True}, sort_keys=True))
    return 0

def attest(root: Path) -> int:
    head, tree, parent = identity()
    api_dir, source_dir, producer_dir, final = (root / 'api', root / 'source', root / 'producer', root / 'final')
    source_name, producer_name = (os.environ['SOURCE_ARTIFACT_NAME'], os.environ['PRODUCER_ARTIFACT_NAME'])
    run, jobs, artifacts = poll([(SOURCE_JOB, SOURCE_STEPS), (PRODUCER_JOB, PRODUCER_STEPS)], [source_name, producer_name], api_dir)
    run_id = int(os.environ['GITHUB_RUN_ID'])
    source_job, producer_job = (one(jobs['jobs'], 'name', SOURCE_JOB, 'source job'), one(jobs['jobs'], 'name', PRODUCER_JOB, 'producer job'))
    source_artifact, producer_artifact = (one(artifacts['artifacts'], 'name', source_name, 'source artifact'), one(artifacts['artifacts'], 'name', producer_name, 'producer artifact'))
    extract(source_artifact, api_dir / 'source-artifact.zip', source_dir, api_dir / 'source-archive-manifest.json')
    extract(producer_artifact, api_dir / 'producer-artifact.zip', producer_dir, api_dir / 'producer-archive-manifest.json')
    source_path, truth_path = (unique(source_dir, 'a0-source-gate-receipt.json'), unique(source_dir, 'current-truth.json'))
    producer_path = unique(producer_dir, 'a0-executable-producer-receipt.json')
    source, truth, producer = (load(source_path), load(truth_path), load(producer_path))
    verify_binding(producer, 'producer')
    candidate = {'branch': BRANCH, 'head': head, 'parent': parent, 'tree': tree}
    need(source.get('status') == 'PASS_HEPTA_INTELLIGENCE_A0_SOURCE_ONLY' and source.get('candidate') == candidate, 'source receipt')
    need(source.get('a0_candidate_qualified') is False and all_false(source.get('authority')), 'source authority')
    need(truth.get('q0', {}).get('qualified_candidate') is True and truth.get('q0', {}).get('runtime_capability_qualified') is False and all_false(truth.get('authority')), 'truth boundary')
    need(producer.get('status') == 'PASS_HEPTA_INTELLIGENCE_A0_EXECUTABLE_PRODUCER_ONLY' and producer.get('candidate') == candidate, 'producer receipt')
    need(producer.get('conclusion', {}).get('a0_candidate_qualified') is False and producer.get('conclusion', {}).get('terminal_attestation_required') is True and all_false(producer.get('authority')), 'producer boundary')
    source_manifest, producer_manifest = (load(api_dir / 'source-archive-manifest.json'), load(api_dir / 'producer-archive-manifest.json'))
    for manifest, artifact, label in ((source_manifest, source_artifact, 'source'), (producer_manifest, producer_artifact, 'producer')):
        need(manifest.get('artifact_id') == artifact.get('id') and manifest.get('artifact_name') == artifact.get('name'), f'{label} manifest identity')
        need(manifest.get('artifact_digest') == artifact.get('digest') and manifest.get('artifact_expires_at') == artifact.get('expires_at'), f'{label} manifest binding')
        need(manifest.get('archive_sha256') == str(artifact.get('digest', '')).removeprefix('sha256:'), f'{label} archive digest')
    receipt = {'schema': 'hepta_intelligence_a0_executable_qualification_receipt_v2', 'status': 'PASS_HEPTA_INTELLIGENCE_A0_EXECUTABLE_QUALIFICATION', 'repository': REPO, 'candidate': candidate, 'workflow': {'workflow_id': run.get('workflow_id'), 'workflow_name': run.get('name'), 'workflow_path': WORKFLOW_REL, 'workflow_git_blob_sha': git('rev-parse', f'HEAD:{WORKFLOW_REL}'), 'workflow_content_sha256': sha(WORKFLOW), 'run_id': run_id, 'run_attempt': run.get('run_attempt'), 'event': run.get('event')}, 'qualified_predecessor_jobs': [{'name': source_job.get('name'), 'job_id': source_job.get('id'), 'runner_id': source_job.get('runner_id'), 'runner_name': source_job.get('runner_name'), 'runner_labels': source_job.get('labels'), 'required_steps': SOURCE_STEPS, 'steps_non_empty': True}, {'name': producer_job.get('name'), 'job_id': producer_job.get('id'), 'runner_id': producer_job.get('runner_id'), 'runner_name': producer_job.get('runner_name'), 'runner_labels': producer_job.get('labels'), 'required_steps': PRODUCER_STEPS, 'steps_non_empty': True}], 'qualified_artifacts': [{'artifact_id': source_artifact.get('id'), 'artifact_name': source_name, 'artifact_digest': source_artifact.get('digest'), 'artifact_expires_at': source_artifact.get('expires_at'), 'archive_sha256': source_manifest['archive_sha256']}, {'artifact_id': producer_artifact.get('id'), 'artifact_name': producer_name, 'artifact_digest': producer_artifact.get('digest'), 'artifact_expires_at': producer_artifact.get('expires_at'), 'archive_sha256': producer_manifest['archive_sha256']}], 'evidence': {'source_receipt_sha256': sha(source_path), 'current_truth_sha256': sha(truth_path), 'producer_receipt_sha256': sha(producer_path), 'source_archive_manifest_sha256': sha(api_dir / 'source-archive-manifest.json'), 'producer_archive_manifest_sha256': sha(api_dir / 'producer-archive-manifest.json'), 'run_metadata_sha256': sha(api_dir / 'run.json'), 'jobs_metadata_sha256': sha(api_dir / 'jobs.json'), 'artifacts_metadata_sha256': sha(api_dir / 'artifacts.json')}, 'conclusion': {'q0_base_qualified_candidate': True, 'a0_source_implemented': True, 'a0_candidate_qualified': True, 'runtime_wired': False, 'runtime_capability_qualified': False, 'efficacy_proven': False, 'operator_accepted': False, 'promoted': False, 'full_repository_merge_green': False, 'b0_activation_requires_independent_review_and_canonical_selection': True, 'qualification_artifact_requires_external_integrity_revalidation': True}, 'authority': source['authority']}
    receipt['receipt_binding_sha256'] = hashlib.sha256(canon(receipt)).hexdigest()
    save(final / 'a0-executable-qualification-receipt.json', receipt)
    verify_binding(receipt, 'qualification')
    print(json.dumps({'status': receipt['status'], 'head': head, 'predecessor_jobs': 2, 'artifact_digests_verified': 2}, sort_keys=True))
    return 0

def main() -> int:
    if len(sys.argv) == 1:
        print(json.dumps(source_receipt(), indent=2, sort_keys=True))
        return 0
    if len(sys.argv) == 3 and sys.argv[1] == '--produce-executable-evidence':
        return produce(Path(sys.argv[2]))
    if len(sys.argv) == 3 and sys.argv[1] == '--attest-executable-evidence':
        return attest(Path(sys.argv[2]))
    fail('usage: verifier.py [--produce-executable-evidence DIR | --attest-executable-evidence DIR]')
if __name__ == '__main__':
    sys.exit(main())
