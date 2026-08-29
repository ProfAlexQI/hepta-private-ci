#!/usr/bin/env bash
set -euo pipefail
repo=ProfHepta/hepta-private-ci
branch=codex/hepta-intelligence-a0-2-evidence-gap-closure-20260829
parent=a7e407fd3e4b4f092f771e0428f275b7eb5a9870
b64_sha=1eeb103d1b1264086ee8424ba670f20969e8fc521da7c60c39aeb3d5904bb1a5
archive_sha=97f62d4004f930071be19386458bf1b36a2813e837cdba51a21c9486af375694
payload=.hepta-a02-payload
work="$RUNNER_TEMP/hepta-a02-publisher"
test "$GITHUB_REPOSITORY" = "$repo"
test "$GITHUB_REF_NAME" = "$branch"
test "$(git rev-parse HEAD)" = "$GITHUB_SHA"
test "$(git rev-parse HEAD^)" = "$parent"
test "$(git ls-remote origin refs/heads/$branch | awk '{print $1}')" = "$GITHUB_SHA"
mkdir -p "$work/extract"
cat "$payload"/part-* > "$work/payload.b64"
printf '%s  %s\n' "$b64_sha" "$work/payload.b64" | sha256sum -c --strict
base64 --decode "$work/payload.b64" > "$work/payload.tar.gz"
printf '%s  %s\n' "$archive_sha" "$work/payload.tar.gz" | sha256sum -c --strict
python3 - "$work/payload.tar.gz" "$payload/allowed.txt" <<'PY'
from pathlib import Path, PurePosixPath
import sys, tarfile
allowed=set(Path(sys.argv[2]).read_text().splitlines())
with tarfile.open(sys.argv[1],"r:gz") as t:
    seen=set()
    for m in t.getmembers():
        n=m.name[2:] if m.name.startswith("./") else m.name
        if m.isdir(): continue
        p=PurePosixPath(n)
        assert m.isfile() and not p.is_absolute() and ".." not in p.parts, n
        seen.add(n)
    assert seen==allowed,(sorted(seen-allowed),sorted(allowed-seen))
PY
tar -xzf "$work/payload.tar.gz" -C "$work/extract"
git fetch --no-tags --depth=1 origin "$parent"
git worktree add --detach "$work/final" "$parent"
cp -a "$work/extract/." "$work/final/"
cd "$work/final"
git add --all
sort "$GITHUB_WORKSPACE/$payload/allowed.txt" > "$work/allowed.sorted"
git diff --cached --name-only | sort > "$work/observed.sorted"
diff -u "$work/allowed.sorted" "$work/observed.sorted"
git diff --cached --check
tree=$(git write-tree)
export GIT_AUTHOR_NAME='Hepta Intelligence A0 Publisher'
export GIT_AUTHOR_EMAIL='102159240+ProfHepta@users.noreply.github.com'
export GIT_COMMITTER_NAME="$GIT_AUTHOR_NAME" GIT_COMMITTER_EMAIL="$GIT_AUTHOR_EMAIL"
commit=$(printf '%s\n' 'docs(intelligence): close A0.2 evidence and gap authority defects' | git commit-tree "$tree" -p "$parent")
test "$(git rev-parse "$commit^")" = "$parent"
git push --force-with-lease="refs/heads/$branch:$GITHUB_SHA" origin "$commit:refs/heads/$branch"
printf 'final_commit=%s\nfinal_tree=%s\n' "$commit" "$tree" >> "$GITHUB_STEP_SUMMARY"
