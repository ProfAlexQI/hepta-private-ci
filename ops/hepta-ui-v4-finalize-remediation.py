#!/usr/bin/env python3
from __future__ import annotations

import base64
import json
import os
import subprocess
import urllib.request
from pathlib import Path

REPO = os.environ['GITHUB_REPOSITORY']
TOKEN = os.environ['GH_TOKEN']
TARGET_SHA = os.environ['TARGET_SHA']
TARGET_TREE = os.environ['TARGET_TREE']
ROOT = Path(os.environ.get('HEPTA_CANDIDATE_ROOT', 'candidate'))
PATHS_FILE = Path(os.environ.get('HEPTA_MODIFIED_PATHS_OUT', 'artifacts/modified-paths.json'))
OUTPUT = Path(os.environ.get('HEPTA_COMMIT_OBJECT_OUT', 'artifacts/commit-object.json'))
MESSAGE = os.environ.get(
    'HEPTA_COMMIT_MESSAGE',
    'ci(ui): close exact-head Native, Windows checkout, and browser determinism gaps',
)


def request(method: str, endpoint: str, payload: dict | None = None) -> dict:
    data = None if payload is None else json.dumps(payload).encode()
    req = urllib.request.Request(
        f'https://api.github.com/repos/{REPO}{endpoint}',
        data=data,
        method=method,
        headers={
            'Authorization': f'Bearer {TOKEN}',
            'Accept': 'application/vnd.github+json',
            'X-GitHub-Api-Version': '2022-11-28',
            'Content-Type': 'application/json',
            'User-Agent': 'hepta-ui-v4-remediation-finalizer',
        },
    )
    with urllib.request.urlopen(req, timeout=60) as response:
        return json.load(response)


def main() -> None:
    paths = json.loads(PATHS_FILE.read_text())
    entries = []
    blobs = {}
    for path in paths:
        source = ROOT / path
        content = base64.b64encode(source.read_bytes()).decode()
        blob = request('POST', '/git/blobs', {'content': content, 'encoding': 'base64'})
        mode = subprocess.check_output(
            ['git', '-C', str(ROOT), 'ls-tree', TARGET_SHA, '--', path], text=True
        ).split()[0]
        entries.append({'path': path, 'mode': mode, 'type': 'blob', 'sha': blob['sha']})
        blobs[path] = blob['sha']

    tree = request('POST', '/git/trees', {'base_tree': TARGET_TREE, 'tree': entries})
    commit = request('POST', '/git/commits', {
        'message': MESSAGE,
        'tree': tree['sha'],
        'parents': [TARGET_SHA],
    })
    resolved = request('GET', f"/git/commits/{commit['sha']}")
    if resolved['tree']['sha'] != tree['sha']:
        raise SystemExit('remote commit tree mismatch')
    parents = [parent['sha'] for parent in resolved['parents']]
    if parents != [TARGET_SHA]:
        raise SystemExit(f'remote commit parent mismatch: {parents}')

    payload = {
        'schema': 'hepta.ui.v4.remediation-commit-object.v3',
        'parent': TARGET_SHA,
        'baseTree': TARGET_TREE,
        'tree': tree['sha'],
        'commit': commit['sha'],
        'blobs': blobs,
        'authority': {
            'refUpdated': False,
            'targetBranchUpdated': False,
            'production': False,
            'operatorAcceptance': False,
            'promotion': False,
            'release': False,
        },
    }
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(json.dumps(payload, indent=2) + '\n')
    print(json.dumps(payload, indent=2))


if __name__ == '__main__':
    main()
