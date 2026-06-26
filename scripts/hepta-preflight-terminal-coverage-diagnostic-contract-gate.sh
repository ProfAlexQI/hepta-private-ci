#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT"

source "$ROOT/scripts/lib/hepta-json-report-capture.sh"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

required_markers=(
  "metadata"
  "fmt"
  "cargo check"
  "adapter behavior-equivalence gate"
  "adapter shadow-replay gate"
  "name/repository closure gate"
  "active service dependency isolation gate"
  "legacy preflight entrypoint migration gate"
  "legacy watchdog entrypoint migration gate"
  "legacy live gates entrypoint migration gate"
  "legacy release/readiness entrypoint migration gate"
  "legacy inventory entrypoint migration gate"
  "memory-rem status closure gate"
  "memory-tools catalog closure gate"
  "native residual runtime status closure gate"
  "plugin migration plan closure gate"
  "skill workshop plan closure gate"
  "memory/intelligence closure gate"
  "KG prompt-preview preflight gate"
  "KG prompt-preview terminal next-action activation denial summary gate"
  "KG prompt-preview memory/intelligence full enablement activation readiness gate"
  "memory/intelligence full enablement memory live mutation staging fixture gate"
  "memory/intelligence full enablement KG external adapter staging receipt gate"
  "memory/intelligence full enablement bounded prompt-preview context handoff activation packet gate"
  "memory/intelligence full enablement runtime provider-router context attachment staging gate"
  "memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution gate"
  "memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution readiness route gate"
  "memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled route gate"
  "memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt no-persistence gate"
  "memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt authority-denial gate"
  "memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt trusted operator packet separation gate"
  "memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt trusted operator packet intake precondition gate"
  "memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt trusted operator packet partial precondition denial matrix gate"
  "memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt trusted operator packet complete precondition authority-denial gate"
  "memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt trusted operator packet complete precondition operator approval lane separation gate"
  "memory/intelligence full enablement operator-approved memory live mutation durable lane gate"
  "memory/intelligence full enablement operator-approved Hepta Intelligence context attachment lane gate"
  "memory/intelligence full enablement operator-approved KG prompt-preview read-only adapter lane gate"
  "memory/intelligence full enablement operator-approved KG prompt payload materialization lane gate"
  "memory/intelligence full enablement operator-approved KG prompt payload acceptance receipt lane gate"
  "memory/intelligence full enablement operator-approved KG prompt payload readback audit receipt lane gate"
  "memory/intelligence full enablement operator-approved context handoff acceptance lane gate"
  "memory/intelligence full enablement operator-approved context handoff receipt audit lane gate"
  "memory/intelligence full enablement operator-approved bounded provider-router injection precondition lane gate"
  "memory/intelligence full enablement operator-approved bounded provider-router injection dry-run envelope lane gate"
  "memory/intelligence full enablement operator-approved bounded provider-router injection dry-run envelope readback audit receipt lane gate"
  "memory/intelligence full enablement operator-approved bounded provider-router injection dry-run envelope readback audit receipt acknowledgement no-op handoff lane gate"
  "memory/intelligence full enablement positive activation packet dry-run scaffold gate"
  "memory/intelligence full enablement positive activation packet validator scoreboard gate"
  "memory/intelligence full enablement canary live harness scaffold gate"
  "memory/intelligence full enablement explicit operator-approved canary packet record scaffold gate"
  "memory/intelligence full enablement operator canary packet value fixture scoreboard gate"
  "memory/intelligence full enablement operator canary arm plan dry-run gate"
  "memory/intelligence full enablement operator canary arm readiness scoreboard gate"
  "memory/intelligence full enablement operator canary dispatch envelope preview gate"
  "memory/intelligence full enablement operator canary controlled request payload preview no-write sink gate"
  "memory/intelligence full enablement operator canary controlled request payload readback audit receipt preview gate"
  "memory/intelligence full enablement operator canary controlled request payload readback audit receipt acceptance packet dry-run scaffold gate"
  "memory/intelligence full enablement operator canary controlled request payload readback audit receipt acceptance packet value scoreboard gate"
  "memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record scaffold gate"
  "memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record intake validator gate"
  "memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record negative fixture matrix gate"
  "memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record positive precondition scoreboard gate"
  "memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record template gate"
  "memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record readiness lock gate"
  "memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record controlled request dispatch envelope lock validator gate"
  "memory/intelligence full enablement operator canary controlled request harness no-dispatch readback audit scoreboard gate"
  "memory/intelligence full enablement operator canary controlled request harness redacted payload preview no-materialization gate"
  "memory/intelligence full enablement operator canary controlled request harness readback/audit receipt hash preview acceptance skeleton gate"
  "memory/intelligence full enablement operator canary controlled request harness single-budget dispatch dry-run no-op receipt gate"
  "memory/intelligence full enablement operator canary controlled request harness single-budget dispatch dry-run no-op receipt route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review/readback index no-persistence gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review/readback index no-persistence route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement non-acceptance gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement non-acceptance route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation request denial matrix gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation request denial matrix route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command no-op handoff gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command no-op handoff route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt no-persistence gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt no-persistence route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt replay idempotency denial gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt replay idempotency denial route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt ordering monotonicity denial gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt ordering monotonicity denial route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt cancellation supersession denial gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt cancellation supersession denial route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt audit trail immutable evidence denial gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt audit trail immutable evidence denial route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt retention expiry garbage collection denial gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt retention expiry garbage collection denial route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt export query observability denial gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt export query observability denial route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt operator-facing summary briefing non-persistence denial gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt operator-facing summary briefing non-persistence denial route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt final operator acknowledgement non-acceptance denial gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt final operator acknowledgement non-acceptance denial route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt terminal operator decision public-claim non-promotion denial gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt terminal operator decision public-claim non-promotion denial route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt release artifact publication denial gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt release artifact publication denial route gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt release artifact publication result receipt no-persistence gate"
  "memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt release artifact publication result receipt no-persistence route gate"
  "memory/intelligence/KG full live activation readiness index gate"
  "memory/intelligence/KG full live activation readiness index replay/idempotency denial gate"
  "memory/intelligence/KG full live activation readiness index replay/idempotency denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template gate"
  "memory/intelligence/KG full live activation operator readiness packet template route gate"
  "memory/intelligence/KG full live activation operator readiness packet template non-acceptance authority replay denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template non-acceptance authority replay denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template field validation denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template field validation denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template section completion non-acceptance gate"
  "memory/intelligence/KG full live activation operator readiness packet template section completion non-acceptance route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet assembly non-acceptance gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet assembly non-acceptance route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt non-persistence gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt non-persistence route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt replay/idempotency denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt replay/idempotency denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt ordering/monotonicity denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt ordering/monotonicity denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt cancellation/supersession denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt cancellation/supersession denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt audit-trail/immutable-evidence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt audit-trail/immutable-evidence denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt retention/expiry/garbage-collection denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt retention/expiry/garbage-collection denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt export/query/observability denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt export/query/observability denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt redaction/privacy/payload-exposure denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt redaction/privacy/payload-exposure denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt operator briefing non-persistence gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt operator briefing non-persistence route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt final acknowledgement non-acceptance gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt final acknowledgement non-acceptance route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt terminal decision/status promotion denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt terminal decision/status promotion denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt no-persistence gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt no-persistence route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt replay/idempotency denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt replay/idempotency denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt ordering/monotonicity denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt ordering/monotonicity denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt cancellation/supersession denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt cancellation/supersession denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt audit-trail/immutable-evidence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt audit-trail/immutable-evidence denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt retention/expiry/garbage-collection denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt retention/expiry/garbage-collection denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt export/query/observability denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt export/query/observability denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt operator-facing summary/briefing non-persistence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt operator-facing summary/briefing non-persistence denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt final operator acknowledgement non-acceptance denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt final operator acknowledgement non-acceptance denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal decision/status promotion denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal decision/status promotion denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal public claim/status exposure denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal public claim/status exposure denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution queue/artifact availability status denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution queue/artifact availability status denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt/external delivery non-persistence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt/external delivery non-persistence denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt query/export/observability denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt query/export/observability denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt privacy/redaction/payload-exposure denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt operator briefing non-persistence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt final operator acknowledgement non-acceptance denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt terminal decision/status promotion denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt terminal public claim/status exposure denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt package/release channel status exposure denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt distribution artifact/manifest status denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact distribution signing/notarization surface denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt no-persistence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt no-persistence denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt replay/idempotency denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt replay/idempotency denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt ordering/monotonicity denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt ordering/monotonicity denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt cancellation/supersession denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt cancellation/supersession denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt audit-trail/immutable-evidence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt audit-trail/immutable-evidence denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt retention/expiry/garbage-collection denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt retention/expiry/garbage-collection denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt export/query/observability denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt export/query/observability denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator-facing summary/briefing non-persistence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator-facing summary/briefing non-persistence denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt final operator acknowledgement non-acceptance denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt final operator acknowledgement non-acceptance denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt terminal decision/status promotion denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt terminal decision/status promotion denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator intent/consent reconfirmation denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator intent/consent reconfirmation denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session binding denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session binding denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session replay/cross-binding denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session replay/cross-binding denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement ordering/monotonicity denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement ordering/monotonicity denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement cancellation/supersession denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement cancellation/supersession denial route gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement audit/evidence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement retention/expiry/garbage-collection denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement export/query/observability denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator-facing summary/briefing non-persistence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement final operator acknowledgement non-acceptance denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement terminal decision/status promotion denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent reconfirmation denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence persistence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence export/query/observability denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence summary/briefing non-persistence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence final operator acknowledgement non-acceptance denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence terminal decision/status promotion denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence terminal public claim/status exposure denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence package/release channel status exposure denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence distribution artifact/manifest status denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization surface denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization result receipt no-persistence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt replay/idempotency denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt ordering/monotonicity denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt cancellation/supersession denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt audit/evidence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt retention/expiry/garbage-collection denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt export/query/observability denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt operator-facing summary/briefing non-persistence denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt final operator acknowledgement non-acceptance denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt terminal decision/status promotion denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt terminal public claim/status exposure denial gate"
  "memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt terminal public claim delivery/readback denial gate"
  "memory/intelligence full enablement runtime provider-router context attachment negative fixture matrix gate"
  "memory/intelligence full enablement runtime provider-router readback receipt skeleton gate"
  "memory/intelligence full enablement runtime provider-router receipt observability denial gate"
  "memory/intelligence full enablement runtime provider-router operator-facing summary non-persistence gate"
  "memory/intelligence full enablement runtime provider-router operator acknowledgement non-acceptance gate"
  "memory/intelligence full enablement runtime provider-router activation request denial matrix gate"
  "memory/intelligence full enablement runtime provider-router activation command no-op handoff gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt no-persistence gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt replay idempotency denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt ordering monotonicity denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt cancellation supersession denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt audit trail immutable evidence denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt retention expiry garbage collection denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt export query observability denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt operator-facing summary briefing non-persistence denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt final operator acknowledgement non-acceptance denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt terminal operator decision public-claim non-promotion denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt release artifact publication denial gate"
  "readiness denial review acceptance closure summary gate"
  "upstream Codex promotion closure gate"
  "terminal release-governance final audit index gate"
  "operator-security attention-budget diagnostic gate"
  "terminal watchdog/soak regression gate"
  "core activation evidence receipt terminal closure decision gate"
  "core activation terminal closure gap evidence index gate"
  "core activation terminal closure operator packet template gate"
  "core activation terminal closure operator packet dry-run validator gate"
  "core activation terminal closure operator packet authority replay matrix gate"
  "core activation terminal closure operator packet trusted-record acceptance skeleton gate"
  "core activation terminal closure operator packet trusted-record acceptance negative-fixture matrix gate"
  "core activation terminal closure operator packet trusted-record acceptance precondition scoreboard gate"
  "core activation terminal closure operator packet trusted-record positive packet dry-run scaffold gate"
  "core activation terminal closure operator packet trusted-record positive packet authority replay denial matrix gate"
  "core activation terminal closure operator packet trusted-record positive packet authority replay denial summary gate"
  "core activation terminal closure operator packet trusted-record positive packet authority replay denial summary index gate"
  "core activation terminal closure operator packet trusted-record positive packet authority replay denial summary index manifest gate"
  "core activation terminal closure operator packet trusted-record positive packet authority replay denial summary index manifest JSON-capture boundary gate"
  "core activation terminal closure operator packet trusted-record positive packet operator approval gap ledger gate"
  "core activation operator approval gap ledger summary briefing non-persistence gate"
  "core activation operator approval gap ledger summary briefing acknowledgement non-acceptance gate"
  "core activation operator approval gap ledger summary briefing acknowledgement activation request denial matrix gate"
  "core activation operator approval gap ledger summary briefing acknowledgement activation command no-op handoff gate"
  "core activation operator approval gap ledger summary briefing acknowledgement activation command result receipt no-persistence gate"
  "core activation operator approval gap ledger summary briefing acknowledgement activation command result receipt replay idempotency denial gate"
  "core activation operator approval gap ledger summary briefing acknowledgement activation command result receipt ordering monotonicity denial gate"
  "core activation operator approval gap ledger summary briefing acknowledgement activation command result receipt cancellation supersession denial gate"
  "JSON report capture diagnostic contract gate"
  "JSON report capture migration inventory gate"
  "preflight terminal coverage inventory gate"
  "preflight terminal coverage diagnostic contract gate"
  "upstream Codex latest active-safety regression gate"
  "upstream Codex latest release-governance non-activation gate"
  "upstream Codex latest operator briefing non-persistence gate"
  "hepta-gateway tests"
  "codex-cli native tests"
  "control-ui smoke"
  "native app metadata/check/tests"
  "release build compatibility codex-cli"
  "release build active hepta-cli"
  "whitespace/status"
)

emit_phase_family_budget_markers() {
  local mode="${1:-good}"
  local i
  local live_marker_count=55

  if [[ "$mode" == "missing-phase-family-budget" ]]; then
    live_marker_count=49
  fi

  if [[ "$mode" != "missing-phase-family-anchor" ]]; then
    printf '%s\n' 'echo "[hepta-preflight] KG prompt-preview operator approval checklist schema gate"'
  fi
  printf '%s\n' \
    'echo "[hepta-preflight] live mutation governance gate"' \
    'echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt final operator acknowledgement non-acceptance denial gate"' \
    'echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review acceptance readiness denial review acceptance closure gate"' \
    'echo "[hepta-preflight] upstream Codex snapshot gate"' \
    'echo "[hepta-preflight] upstream Codex activation evidence receipt filesystem persistence execution denial matrix gate"' \
    'echo "[hepta-preflight] upstream Codex sync lane gate"' \
    'echo "[hepta-preflight] terminal denial index gate"' \
    'echo "[hepta-preflight] terminal public distribution non-publication lock gate"' \
    'echo "[hepta-preflight] core activation long-soak observation non-acceptance gate"' \
    'echo "[hepta-preflight] core activation fresh long-soak evidence ledger receipt gate"'

  for ((i = 1; i <= 7; i++)); do
    printf 'echo "[hepta-preflight] KG prompt-preview synthetic family budget marker %02d"\n' "$i"
  done

  for ((i = 1; i <= live_marker_count; i++)); do
    printf 'echo "[hepta-preflight] live mutation synthetic family budget marker %02d"\n' "$i"
  done

  for ((i = 1; i <= 43; i++)); do
    printf 'echo "[hepta-preflight] upstream Codex synthetic family budget marker %02d"\n' "$i"
  done

  for ((i = 1; i <= 8; i++)); do
    printf 'echo "[hepta-preflight] terminal synthetic family budget marker %02d"\n' "$i"
  done

  for ((i = 1; i <= 10; i++)); do
    printf 'echo "[hepta-preflight] core activation synthetic family budget marker %02d"\n' "$i"
  done
}

emit_fixture_preflight() {
  local mode="${1:-good}"
  local marker

  printf '%s\n' \
    '#!/usr/bin/env bash' \
    'set -euo pipefail' \
    'RUN_NATIVE="${HEPTA_PREFLIGHT_NATIVE:-0}"' \
    'RUN_RELEASE="${HEPTA_PREFLIGHT_RELEASE:-0}"'

  for marker in "${required_markers[@]}"; do
    if [[ "$mode" == "missing-required-marker" \
      && "$marker" == "upstream Codex latest operator briefing non-persistence gate" ]]; then
      continue
    fi

    if [[ "$mode" == "missing-spine-marker" \
      && "$marker" == "active service dependency isolation gate" ]]; then
      continue
    fi

    if [[ "$mode" == "out-of-order-required-marker" \
      && "$marker" == "JSON report capture diagnostic contract gate" ]]; then
      printf '%s\n' \
        'echo "[hepta-preflight] JSON report capture migration inventory gate"' \
        'echo "[hepta-preflight] JSON report capture diagnostic contract gate"'
      continue
    fi

    if [[ "$mode" == "out-of-order-required-marker" \
      && "$marker" == "JSON report capture migration inventory gate" ]]; then
      continue
    fi

    printf 'echo "[hepta-preflight] %s"\n' "$marker"

    if [[ "$mode" == "duplicate-required-marker" \
      && "$marker" == "preflight terminal coverage inventory gate" ]]; then
      printf 'echo "[hepta-preflight] %s"\n' "$marker"
    fi
  done

  emit_phase_family_budget_markers "$mode"

  if [[ "$mode" != "missing-native-release-skip-branches" ]]; then
    printf '%s\n' \
      'if [[ "$RUN_NATIVE" != "1" ]]; then' \
      '  echo "[hepta-preflight] native app gates skipped (HEPTA_PREFLIGHT_NATIVE=$RUN_NATIVE)"' \
      'fi' \
      'if [[ "$RUN_RELEASE" != "1" ]]; then' \
      '  echo "[hepta-preflight] release build skipped (set HEPTA_PREFLIGHT_RELEASE=1)"' \
      'fi'
  fi

  if [[ "$mode" == "out-of-order-final-status-checks" ]]; then
    printf '%s\n' \
      'git diff --cached --check' \
      'git diff --check' \
      'git status -sb'
  else
    if [[ "$mode" != "missing-workspace-diff-check" ]]; then
      printf '%s\n' 'git diff --check'
    fi
    if [[ "$mode" != "missing-cached-diff-check" ]]; then
      printf '%s\n' 'git diff --cached --check'
    fi
    if [[ "$mode" != "missing-git-status-check" ]]; then
      printf '%s\n' 'git status -sb'
    fi
  fi

  if [[ "$mode" != "missing-terminal-pass-marker" ]]; then
    printf '%s\n' 'echo "Hepta preflight passed"'
  fi
}

fixture_report=""
fixture_rc=0

capture_fixture_report() {
  local fixture_text="$1"
  local min_marker_count="$2"
  local output
  local report
  local rc=0

  set +e
  output="$(
    HEPTA_PREFLIGHT_TERMINAL_COVERAGE_PREFLIGHT_TEXT="$fixture_text" \
    HEPTA_PREFLIGHT_TERMINAL_COVERAGE_MIN_MARKER_COUNT="$min_marker_count" \
      scripts/hepta-preflight-terminal-coverage-inventory-gate.sh 2>&1
  )"
  rc=$?
  set -e

  report="$(printf '%s\n' "$output" | extract_first_json_object)"
  if ! jq -e . >/dev/null <<<"$report"; then
    echo "fixture inventory did not emit parseable JSON" >&2
    printf '%s\n' "$output" >&2
    fixture_report='{}'
    fixture_rc=99
    return
  fi

  fixture_report="$report"
  fixture_rc="$rc"
}

good_fixture="$(emit_fixture_preflight good)"
missing_marker_fixture="$(emit_fixture_preflight missing-required-marker)"
missing_spine_marker_fixture="$(emit_fixture_preflight missing-spine-marker)"
duplicate_marker_fixture="$(emit_fixture_preflight duplicate-required-marker)"
out_of_order_fixture="$(emit_fixture_preflight out-of-order-required-marker)"
missing_pass_fixture="$(emit_fixture_preflight missing-terminal-pass-marker)"
missing_skip_fixture="$(emit_fixture_preflight missing-native-release-skip-branches)"
missing_workspace_diff_fixture="$(emit_fixture_preflight missing-workspace-diff-check)"
missing_cached_diff_fixture="$(emit_fixture_preflight missing-cached-diff-check)"
missing_git_status_fixture="$(emit_fixture_preflight missing-git-status-check)"
out_of_order_final_status_fixture="$(emit_fixture_preflight out-of-order-final-status-checks)"
missing_phase_family_budget_fixture="$(emit_fixture_preflight missing-phase-family-budget)"
missing_phase_family_anchor_fixture="$(emit_fixture_preflight missing-phase-family-anchor)"

capture_fixture_report "$good_fixture" 0
good_report="$fixture_report"
good_rc="$fixture_rc"

capture_fixture_report "$missing_marker_fixture" 0
missing_marker_report="$fixture_report"
missing_marker_rc="$fixture_rc"

capture_fixture_report "$missing_spine_marker_fixture" 0
missing_spine_marker_report="$fixture_report"
missing_spine_marker_rc="$fixture_rc"

capture_fixture_report "$duplicate_marker_fixture" 0
duplicate_marker_report="$fixture_report"
duplicate_marker_rc="$fixture_rc"

capture_fixture_report "$out_of_order_fixture" 0
out_of_order_report="$fixture_report"
out_of_order_rc="$fixture_rc"

capture_fixture_report "$good_fixture" 999
marker_count_budget_report="$fixture_report"
marker_count_budget_rc="$fixture_rc"

capture_fixture_report "$missing_pass_fixture" 0
missing_pass_report="$fixture_report"
missing_pass_rc="$fixture_rc"

capture_fixture_report "$missing_skip_fixture" 0
missing_skip_report="$fixture_report"
missing_skip_rc="$fixture_rc"

capture_fixture_report "$missing_workspace_diff_fixture" 0
missing_workspace_diff_report="$fixture_report"
missing_workspace_diff_rc="$fixture_rc"

capture_fixture_report "$missing_cached_diff_fixture" 0
missing_cached_diff_report="$fixture_report"
missing_cached_diff_rc="$fixture_rc"

capture_fixture_report "$missing_git_status_fixture" 0
missing_git_status_report="$fixture_report"
missing_git_status_rc="$fixture_rc"

capture_fixture_report "$out_of_order_final_status_fixture" 0
out_of_order_final_status_report="$fixture_report"
out_of_order_final_status_rc="$fixture_rc"

capture_fixture_report "$missing_phase_family_budget_fixture" 0
missing_phase_family_budget_report="$fixture_report"
missing_phase_family_budget_rc="$fixture_rc"

capture_fixture_report "$missing_phase_family_anchor_fixture" 0
missing_phase_family_anchor_report="$fixture_report"
missing_phase_family_anchor_rc="$fixture_rc"

good_fixture_ok=false
missing_required_marker_fixture_ok=false
missing_spine_marker_fixture_ok=false
duplicate_required_marker_fixture_ok=false
out_of_order_required_marker_fixture_ok=false
marker_count_budget_fixture_ok=false
missing_terminal_pass_marker_fixture_ok=false
missing_native_release_skip_branches_fixture_ok=false
missing_workspace_diff_check_fixture_ok=false
missing_cached_diff_check_fixture_ok=false
missing_git_status_check_fixture_ok=false
out_of_order_final_status_checks_fixture_ok=false
missing_phase_family_budget_fixture_ok=false
missing_phase_family_anchor_fixture_ok=false
phase_family_anchor_family_evidence_ok=false

if [[ "$good_rc" -eq 0 ]] \
  && jq -e '
    .status == "ready"
    and .preflight_terminal_coverage_inventory_ready == true
    and .inline_fixture_mode == true
    and .required_marker_count >= 300
    and .present_required_marker_count == .required_marker_count
    and .missing_required_marker_count == 0
    and .duplicate_required_marker_count == 0
    and .out_of_order_required_marker_count == 0
    and .required_markers_ordered == true
    and .phase_family_count == 10
    and .phase_family_ready_count == 10
    and .phase_family_budget_failure_count == 0
    and .phase_family_budget_ready == true
    and .phase_family_anchor_count == 51
    and .phase_family_anchor_ready_count == 51
    and .phase_family_anchor_failure_count == 0
    and .phase_family_anchor_ready == true
    and .phase_family_anchor_family_count == 10
    and .phase_family_anchor_family_ready_count == 10
    and .phase_family_anchor_family_failure_count == 0
    and .phase_family_anchor_family_ready == true
    and .terminal_pass_marker_present == true
    and .native_release_skip_branches_present == true
    and .final_workspace_diff_check_present == true
    and .final_cached_diff_check_present == true
    and .final_git_status_present == true
    and .final_status_checks_present == true
    and .final_status_checks_ordered == true
    and .final_status_checks_ready == true
  ' >/dev/null <<<"$good_report"; then
  good_fixture_ok=true
fi

if [[ "$missing_marker_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .missing_required_marker_count == 1
    and (.missing_required_markers | index("upstream Codex latest operator briefing non-persistence gate") != null)
  ' >/dev/null <<<"$missing_marker_report"; then
  missing_required_marker_fixture_ok=true
fi

if [[ "$missing_spine_marker_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .missing_required_marker_count == 1
    and (.missing_required_markers | index("active service dependency isolation gate") != null)
  ' >/dev/null <<<"$missing_spine_marker_report"; then
  missing_spine_marker_fixture_ok=true
fi

if [[ "$duplicate_marker_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .duplicate_required_marker_count == 1
    and (.duplicate_required_markers | index("preflight terminal coverage inventory gate") != null)
  ' >/dev/null <<<"$duplicate_marker_report"; then
  duplicate_required_marker_fixture_ok=true
fi

if [[ "$out_of_order_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .out_of_order_required_marker_count == 1
    and .required_markers_ordered == false
    and (.out_of_order_required_markers | index("JSON report capture migration inventory gate") != null)
  ' >/dev/null <<<"$out_of_order_report"; then
  out_of_order_required_marker_fixture_ok=true
fi

if [[ "$marker_count_budget_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .marker_count_budget_ok == false
    and .minimum_required_preflight_marker_count == 999
  ' >/dev/null <<<"$marker_count_budget_report"; then
  marker_count_budget_fixture_ok=true
fi

if [[ "$missing_pass_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .terminal_pass_marker_present == false
  ' >/dev/null <<<"$missing_pass_report"; then
  missing_terminal_pass_marker_fixture_ok=true
fi

if [[ "$missing_skip_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .native_release_skip_branches_present == false
  ' >/dev/null <<<"$missing_skip_report"; then
  missing_native_release_skip_branches_fixture_ok=true
fi

if [[ "$missing_workspace_diff_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .final_workspace_diff_check_present == false
    and .final_cached_diff_check_present == true
    and .final_git_status_present == true
    and .final_status_checks_present == false
    and .final_status_checks_ready == false
  ' >/dev/null <<<"$missing_workspace_diff_report"; then
  missing_workspace_diff_check_fixture_ok=true
fi

if [[ "$missing_cached_diff_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .final_workspace_diff_check_present == true
    and .final_cached_diff_check_present == false
    and .final_git_status_present == true
    and .final_status_checks_present == false
    and .final_status_checks_ready == false
  ' >/dev/null <<<"$missing_cached_diff_report"; then
  missing_cached_diff_check_fixture_ok=true
fi

if [[ "$missing_git_status_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .final_workspace_diff_check_present == true
    and .final_cached_diff_check_present == true
    and .final_git_status_present == false
    and .final_status_checks_present == false
    and .final_status_checks_ready == false
  ' >/dev/null <<<"$missing_git_status_report"; then
  missing_git_status_check_fixture_ok=true
fi

if [[ "$out_of_order_final_status_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .final_workspace_diff_check_present == true
    and .final_cached_diff_check_present == true
    and .final_git_status_present == true
    and .final_status_checks_present == true
    and .final_status_checks_ordered == false
    and .final_status_checks_ready == false
  ' >/dev/null <<<"$out_of_order_final_status_report"; then
  out_of_order_final_status_checks_fixture_ok=true
fi

if [[ "$missing_phase_family_budget_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .missing_required_marker_count == 0
    and .duplicate_required_marker_count == 0
    and .out_of_order_required_marker_count == 0
    and .phase_family_budget_ready == false
    and .phase_family_budget_failure_count == 1
    and (.phase_family_budget_failures[] | select(
      .id == "live-mutation-denial"
      and .current_count == 54
      and .minimum_count == 55
    ))
  ' >/dev/null <<<"$missing_phase_family_budget_report"; then
  missing_phase_family_budget_fixture_ok=true
fi

if [[ "$missing_phase_family_anchor_rc" -eq 1 ]] \
  && jq -e '
    .status == "attention"
    and .preflight_terminal_coverage_inventory_ready == false
    and .missing_required_marker_count == 0
    and .duplicate_required_marker_count == 0
    and .out_of_order_required_marker_count == 0
    and .phase_family_budget_ready == true
    and .phase_family_anchor_ready == false
    and .phase_family_anchor_failure_count == 1
    and (.phase_family_anchor_failures[] | select(
      .family_id == "kg-prompt-preview-readiness"
      and .marker == "KG prompt-preview operator approval checklist schema gate"
    ))
  ' >/dev/null <<<"$missing_phase_family_anchor_report"; then
  missing_phase_family_anchor_fixture_ok=true
fi

if jq -e '
    .phase_family_anchor_family_count == 10
    and .phase_family_anchor_family_ready_count == 10
    and .phase_family_anchor_family_failure_count == 0
    and .phase_family_anchor_family_ready == true
    and (.phase_family_anchor_family_coverage[] | select(
      .id == "kg-prompt-preview-readiness"
      and .required_anchor_count == 3
      and .ready_anchor_count == 3
      and .missing_anchor_count == 0
      and .ready == true
    ))
  ' >/dev/null <<<"$good_report" \
  && jq -e '
    .phase_family_anchor_family_count == 10
    and .phase_family_anchor_family_ready_count == 9
    and .phase_family_anchor_family_failure_count == 1
    and .phase_family_anchor_family_ready == false
    and (.phase_family_anchor_family_coverage[] | select(
      .id == "kg-prompt-preview-readiness"
      and .required_anchor_count == 3
      and .ready_anchor_count == 2
      and .missing_anchor_count == 1
      and .ready == false
      and (.missing_anchors | index("KG prompt-preview operator approval checklist schema gate") != null)
    ))
  ' >/dev/null <<<"$missing_phase_family_anchor_report"; then
  phase_family_anchor_family_evidence_ok=true
fi

contract_hash_sha256="$(
  sha256_text "hepta-preflight-terminal-coverage-diagnostic:$good_fixture_ok:$missing_required_marker_fixture_ok:$missing_spine_marker_fixture_ok:$duplicate_required_marker_fixture_ok:$out_of_order_required_marker_fixture_ok:$marker_count_budget_fixture_ok:$missing_terminal_pass_marker_fixture_ok:$missing_native_release_skip_branches_fixture_ok:$missing_workspace_diff_check_fixture_ok:$missing_cached_diff_check_fixture_ok:$missing_git_status_check_fixture_ok:$out_of_order_final_status_checks_fixture_ok:$missing_phase_family_budget_fixture_ok:$missing_phase_family_anchor_fixture_ok:$phase_family_anchor_family_evidence_ok"
)"
policy_hash_sha256="$(sha256_text "hepta-preflight-terminal-coverage-diagnostic:synthetic-fixtures:no-child-gate-execution:no-workspace-write:no-release-build:no-native-gate")"
side_effect_hash_sha256="$(sha256_text "preflight_fixture_text_only=true;workspace_written=false;release_build=false;native_gate=false;service_restart=false")"

jq -n -e \
  --arg contract_hash_sha256 "$contract_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson good_fixture_ok "$good_fixture_ok" \
  --argjson missing_required_marker_fixture_ok "$missing_required_marker_fixture_ok" \
  --argjson missing_spine_marker_fixture_ok "$missing_spine_marker_fixture_ok" \
  --argjson duplicate_required_marker_fixture_ok "$duplicate_required_marker_fixture_ok" \
  --argjson out_of_order_required_marker_fixture_ok "$out_of_order_required_marker_fixture_ok" \
  --argjson marker_count_budget_fixture_ok "$marker_count_budget_fixture_ok" \
  --argjson missing_terminal_pass_marker_fixture_ok "$missing_terminal_pass_marker_fixture_ok" \
  --argjson missing_native_release_skip_branches_fixture_ok "$missing_native_release_skip_branches_fixture_ok" \
  --argjson missing_workspace_diff_check_fixture_ok "$missing_workspace_diff_check_fixture_ok" \
  --argjson missing_cached_diff_check_fixture_ok "$missing_cached_diff_check_fixture_ok" \
  --argjson missing_git_status_check_fixture_ok "$missing_git_status_check_fixture_ok" \
  --argjson out_of_order_final_status_checks_fixture_ok "$out_of_order_final_status_checks_fixture_ok" \
  --argjson missing_phase_family_budget_fixture_ok "$missing_phase_family_budget_fixture_ok" \
  --argjson missing_phase_family_anchor_fixture_ok "$missing_phase_family_anchor_fixture_ok" \
  --argjson phase_family_anchor_family_evidence_ok "$phase_family_anchor_family_evidence_ok" \
  '
    if (
      $good_fixture_ok == true
      and $missing_required_marker_fixture_ok == true
      and $missing_spine_marker_fixture_ok == true
      and $duplicate_required_marker_fixture_ok == true
      and $out_of_order_required_marker_fixture_ok == true
      and $marker_count_budget_fixture_ok == true
      and $missing_terminal_pass_marker_fixture_ok == true
      and $missing_native_release_skip_branches_fixture_ok == true
      and $missing_workspace_diff_check_fixture_ok == true
      and $missing_cached_diff_check_fixture_ok == true
      and $missing_git_status_check_fixture_ok == true
      and $out_of_order_final_status_checks_fixture_ok == true
      and $missing_phase_family_budget_fixture_ok == true
      and $missing_phase_family_anchor_fixture_ok == true
      and $phase_family_anchor_family_evidence_ok == true
    ) then {
      product: "Hepta",
      runtime: "hepta",
      status: "ready",
      gate: "hepta_preflight_terminal_coverage_diagnostic_contract_gate",
      preflight_terminal_coverage_diagnostic_contract_schema_version: "hepta_preflight_terminal_coverage_diagnostic_contract_v1",
      preflight_terminal_coverage_diagnostic_contract_ready: true,
      diagnostic_mode: "synthetic_inline_preflight_fixture_inventory_no_child_gate_execution",
      diagnostic_decision: "preflight_terminal_coverage_inventory_passes_good_fixture_and_fails_closed_for_missing_spine_phase_family_terminal_duplicate_reordered_and_shrunken_coverage",
      inventory_gate_path: "scripts/hepta-preflight-terminal-coverage-inventory-gate.sh",
      diagnostic_fixture_count: 14,
      good_fixture_ok: $good_fixture_ok,
      missing_required_marker_fixture_ok: $missing_required_marker_fixture_ok,
      missing_spine_marker_fixture_ok: $missing_spine_marker_fixture_ok,
      duplicate_required_marker_fixture_ok: $duplicate_required_marker_fixture_ok,
      out_of_order_required_marker_fixture_ok: $out_of_order_required_marker_fixture_ok,
      marker_count_budget_fixture_ok: $marker_count_budget_fixture_ok,
      missing_terminal_pass_marker_fixture_ok: $missing_terminal_pass_marker_fixture_ok,
      missing_native_release_skip_branches_fixture_ok: $missing_native_release_skip_branches_fixture_ok,
      missing_workspace_diff_check_fixture_ok: $missing_workspace_diff_check_fixture_ok,
      missing_cached_diff_check_fixture_ok: $missing_cached_diff_check_fixture_ok,
      missing_git_status_check_fixture_ok: $missing_git_status_check_fixture_ok,
      out_of_order_final_status_checks_fixture_ok: $out_of_order_final_status_checks_fixture_ok,
      missing_phase_family_budget_fixture_ok: $missing_phase_family_budget_fixture_ok,
      missing_phase_family_anchor_fixture_ok: $missing_phase_family_anchor_fixture_ok,
      phase_family_anchor_family_evidence_ok: $phase_family_anchor_family_evidence_ok,
      good_fixture_ready_preserved: true,
      missing_required_marker_attention_exposed: true,
      missing_spine_marker_attention_exposed: true,
      duplicate_required_marker_attention_exposed: true,
      out_of_order_required_marker_attention_exposed: true,
      marker_count_budget_attention_exposed: true,
      terminal_pass_marker_attention_exposed: true,
      native_release_skip_branch_attention_exposed: true,
      final_workspace_diff_check_attention_exposed: true,
      final_cached_diff_check_attention_exposed: true,
      final_git_status_check_attention_exposed: true,
      final_status_check_order_attention_exposed: true,
      phase_family_budget_attention_exposed: true,
      phase_family_anchor_attention_exposed: true,
      phase_family_anchor_family_evidence_exposed: true,
      contract_hash_sha256: $contract_hash_sha256,
      policy_hash_sha256: $policy_hash_sha256,
      side_effect_hash_sha256: $side_effect_hash_sha256,
      denied_by_preflight_terminal_coverage_diagnostic_contract: [
        "preflight_terminal_coverage_diagnostic_child_gate_execution_denied",
        "preflight_terminal_coverage_diagnostic_release_build_denied",
        "preflight_terminal_coverage_diagnostic_native_gate_execution_denied",
        "preflight_terminal_coverage_diagnostic_workspace_write_denied",
        "preflight_terminal_coverage_diagnostic_service_restart_denied",
        "preflight_terminal_coverage_diagnostic_external_send_denied",
        "preflight_terminal_coverage_diagnostic_secret_read_denied"
      ],
      side_effects: {
        child_gate_execution_performed: false,
        synthetic_fixture_workspace_written: false,
        workspace_written: false,
        filesystem_written: false,
        release_build_executed: false,
        native_app_gate_executed: false,
        runtime_mutation_performed: false,
        active_binary_mutated: false,
        service_restarted: false,
        launchd_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        provider_invoked: false,
        model_invoked: false,
        external_send_performed: false,
        credential_read: false,
        secret_file_read: false
      }
    } else false end
  '

echo "Hepta preflight terminal coverage diagnostic contract gate passed"
