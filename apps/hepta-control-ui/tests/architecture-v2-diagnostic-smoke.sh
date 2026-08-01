#!/usr/bin/env zsh
set -euo pipefail

test_dir=${0:A:h}
app_dir=${test_dir:h}
index_html="$app_dir/index.html"
styles_css=(
  "$app_dir/light-glass-tokens.generated.css"
  "$app_dir/styles.legacy.css"
  "$app_dir/styles.foundation.css"
  "$app_dir/styles.components.css"
  "$app_dir/styles.responsive.css"
  "$app_dir/styles.accessibility.css"
)
snapshot="$app_dir/snapshots/architecture-v2-diagnostic.html"
actual=$(mktemp "${TMPDIR:-/tmp}/hepta-v2-diagnostic.XXXXXX")
trap 'rm -f "$actual"' EXIT

sed -n '/<!-- architecture-v2-diagnostic:start -->/,/<!-- architecture-v2-diagnostic:end -->/p' \
  "$index_html" >"$actual"
diff -u "$snapshot" "$actual"

required_html=(
  'data-architecture-v2-diagnostic="read-only"'
  'data-architecture-v2-evidence-state="absent"'
  'data-architecture-v2-authority-state="unverified"'
  'data-architecture-v2-live-state="unverified"'
  'data-architecture-v2-production-state="not-claimed"'
  'readback not attached'
  'their status does not prove admission or authority'
  'Runtime</strong><small>freeze revisions'
  'Kernel</strong><small>admit exact request'
  'Intelligence</strong><small>select admitted option'
  'Kernel</strong><small>reauthorize at commit'
  'Runtime</strong><small>execute and receipt'
  'Memory</strong><small>preference CAS'
  'href="/api/approvals"'
  'href="/api/policy"'
  'href="/api/events-report"'
  'No live or production-readiness state is inferred.'
  'V2 authority, live state, and production readiness remain unverified.'
)
for marker in "${required_html[@]}"; do
  grep -Fq "$marker" "$actual" || {
    print -u2 "missing Architecture V2 diagnostic marker: $marker"
    exit 1
  }
done

grep -Fq 'data-thread-signature="rust-no-js-static:4"' "$index_html" || {
  print -u2 'Architecture V2 diagnostic did not advance the static thread signature'
  exit 1
}

required_css=(
  '/* architecture-v2-readback-diagnostic */'
  '.tg-message--v2-diagnostic'
  '.v2-control-flow'
  '.v2-readbacks'
  '@media(max-width:700px)'
)
for marker in "${required_css[@]}"; do
  grep -Fq "$marker" "${styles_css[@]}" || {
    print -u2 "missing Architecture V2 diagnostic style: $marker"
    exit 1
  }
done

if grep -Eq 'data-architecture-v2-(evidence-state="(ready|verified)"|authority-state="(authorized|ready)"|live-state="(enabled|live)"|production-state="ready")' "$actual"; then
  print -u2 'Architecture V2 diagnostic contains a positive state claim'
  exit 1
fi
if grep -Fq '<details open>' "$actual"; then
  print -u2 'Architecture V2 diagnostic must stay compact until the operator expands it'
  exit 1
fi
if grep -Fq '/api/approvals/exec/apply' "$actual"; then
  print -u2 'Architecture V2 diagnostic linked a mutating approval surface'
  exit 1
fi
readback_count=$(grep -o 'data-v2-readback=' "$actual" | wc -l | tr -d ' ')
if (( readback_count != 3 )); then
  print -u2 "Architecture V2 diagnostic expected 3 read-only sources, found $readback_count"
  exit 1
fi
if grep -Fqi '<script' "$index_html"; then
  print -u2 'Architecture V2 diagnostic must preserve the no-JS frontend'
  exit 1
fi

styles_bytes=$(wc -c "${styles_css[@]}" | awk 'END { print $1 }')
if (( styles_bytes >= 300000 )); then
  print -u2 "Control UI stylesheet exceeds the Rust bundle budget: $styles_bytes"
  exit 1
fi

print 'Architecture V2 diagnostic smoke passed'
