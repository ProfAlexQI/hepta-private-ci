#!/usr/bin/env python3
"""Source definitions for browser, native shell and external bindings."""

from __future__ import annotations

SOURCE_ROOTS = {
    "browser.servo": (
        "apps/hepta-browser",
        "third_party/servo-patches",
    ),
    "ui.native": ("apps/hepta-native",),
}

FILES = {
    "apps/hepta-browser/package.json": """{
  "name": "@hepta/browser",
  "private": true,
  "type": "module",
  "scripts": {
    "test": "node --test"
  }
}
""",
    "apps/hepta-browser/src/browser.js": r'''const STABLE_ID = /^[A-Za-z0-9._:-]{1,128}$/;
const DIGEST = /^[0-9a-f]{64}$/;
const PAGE_STATES = new Set(["loading", "ready", "failed", "quarantined"]);

function requireRecord(value, name) {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(`${name} must be an object`);
  }
}

function requireStableId(value, name) {
  if (typeof value !== "string" || !STABLE_ID.test(value)) {
    throw new TypeError(`${name} must be a bounded stable identifier`);
  }
  return value;
}

function requireDigest(value, name) {
  if (typeof value !== "string" || !DIGEST.test(value)) {
    throw new TypeError(`${name} must be a lowercase SHA-256 digest`);
  }
  return value;
}

function requireWebUrl(value) {
  const url = new URL(value);
  if (url.protocol !== "https:" && url.protocol !== "http:") {
    throw new TypeError("navigation URL must use HTTP or HTTPS");
  }
  url.username = "";
  url.password = "";
  url.hash = "";
  return url.toString();
}

export function buildNavigationIntent(input) {
  requireRecord(input, "input");
  const navigationId = requireStableId(input.navigationId, "navigationId");
  const tabId = requireStableId(input.tabId, "tabId");
  const url = requireWebUrl(input.url);
  const policyDigest = requireDigest(input.policyDigest, "policyDigest");
  const expectedRevision = input.expectedRevision;
  if (!Number.isSafeInteger(expectedRevision) || expectedRevision < 1) {
    throw new TypeError("expectedRevision must be a positive safe integer");
  }
  return Object.freeze({
    kind: "BrowserNavigationIntentV1",
    navigationId,
    tabId,
    url,
    policyDigest,
    expectedRevision,
    networkAuthority: false,
    effectAuthority: false,
    directStoreWrite: false,
  });
}

export function projectPageState(observation) {
  requireRecord(observation, "observation");
  const tabId = requireStableId(observation.tabId, "tabId");
  if (!PAGE_STATES.has(observation.state)) {
    throw new TypeError("state is not registered");
  }
  const documentDigest = requireDigest(
    observation.documentDigest,
    "documentDigest",
  );
  const sourceRevision = observation.sourceRevision;
  if (!Number.isSafeInteger(sourceRevision) || sourceRevision < 1) {
    throw new TypeError("sourceRevision must be a positive safe integer");
  }
  return Object.freeze({
    tabId,
    state: observation.state,
    documentDigest,
    sourceRevision,
    interactive: observation.state === "ready",
    networkAuthority: false,
    effectAuthority: false,
  });
}
''',
    "apps/hepta-browser/test/browser.test.js": r'''import assert from "node:assert/strict";
import test from "node:test";

import { buildNavigationIntent, projectPageState } from "../src/browser.js";

const digest = "ab".repeat(32);

test("navigation intent is normalized and authority free", () => {
  const intent = buildNavigationIntent({
    navigationId: "navigation:1",
    tabId: "tab:1",
    url: "https://user:password@example.test/path#fragment",
    policyDigest: digest,
    expectedRevision: 7,
  });
  assert.equal(intent.url, "https://example.test/path");
  assert.equal(intent.networkAuthority, false);
  assert.equal(intent.effectAuthority, false);
  assert.equal(intent.directStoreWrite, false);
});

test("non-web schemes fail closed", () => {
  assert.throws(
    () =>
      buildNavigationIntent({
        navigationId: "navigation:1",
        tabId: "tab:1",
        url: "file:///etc/passwd",
        policyDigest: digest,
        expectedRevision: 7,
      }),
    /HTTP or HTTPS/,
  );
});

test("page projection excludes untrusted payloads", () => {
  const state = projectPageState({
    tabId: "tab:1",
    state: "ready",
    documentDigest: digest,
    sourceRevision: 2,
    documentHtml: "<script>unsafe()</script>",
  });
  assert.deepEqual(state, {
    tabId: "tab:1",
    state: "ready",
    documentDigest: digest,
    sourceRevision: 2,
    interactive: true,
    networkAuthority: false,
    effectAuthority: false,
  });
  assert.equal("documentHtml" in state, false);
});
''',
    "apps/hepta-browser/README.md": """# Hepta browser

This root contains an authority-free browser presentation and navigation-intent
boundary. It normalizes HTTP(S) targets, binds policy and source revisions, and
never grants network or effect authority. Servo integration is pinned separately
under `third_party/servo-patches`.
""",
    "third_party/servo-patches/MANIFEST.json": """{
  "schema_version": 1,
  "upstream_repository": "servo/servo",
  "upstream_branch": "main",
  "upstream_commit": "84bcc9ac701874fa9819e5cdee06356b961d736c",
  "patches": [],
  "integration_mode": "pinned_upstream_with_hepta_adapter_boundary",
  "network_authority": false,
  "effect_authority": false,
  "automatic_update": false,
  "interpretation": "pinning an upstream source identity is not runtime activation or independent acceptance"
}
""",
    "third_party/servo-patches/README.md": """# Servo source pin

The browser boundary is evaluated against Servo commit
`84bcc9ac701874fa9819e5cdee06356b961d736c`. This directory intentionally
contains no automatically applied patch and grants no network, deployment,
promotion or release authority. Any future patch must be digest-bound and pass
the same browser qualification gate.
""",
    "apps/hepta-native/package.json": """{
  "name": "@hepta/native",
  "private": true,
  "type": "module",
  "scripts": {
    "test": "node --test"
  }
}
""",
    "apps/hepta-native/src/native.js": r'''const STABLE_ID = /^[A-Za-z0-9._:-]{1,128}$/;
const DIGEST = /^[0-9a-f]{64}$/;
const ACTIONS = new Set([
  "request_open_path",
  "request_reveal_path",
  "request_copy_text",
  "request_notify",
]);

function requireRecord(value, name) {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(`${name} must be an object`);
  }
}

function requireStableId(value, name) {
  if (typeof value !== "string" || !STABLE_ID.test(value)) {
    throw new TypeError(`${name} must be a bounded stable identifier`);
  }
  return value;
}

function requireDigest(value, name) {
  if (typeof value !== "string" || !DIGEST.test(value)) {
    throw new TypeError(`${name} must be a lowercase SHA-256 digest`);
  }
  return value;
}

export function buildNativeIntent(input) {
  requireRecord(input, "input");
  const operationId = requireStableId(input.operationId, "operationId");
  const subjectId = requireStableId(input.subjectId, "subjectId");
  if (!ACTIONS.has(input.action)) {
    throw new TypeError("action is not registered");
  }
  const payloadDigest = requireDigest(input.payloadDigest, "payloadDigest");
  const leasePayloadDigest = requireDigest(
    input.leasePayloadDigest,
    "leasePayloadDigest",
  );
  if (payloadDigest !== leasePayloadDigest) {
    throw new TypeError("lease payload does not match the final payload");
  }
  return Object.freeze({
    kind: "NativeOperationIntentV1",
    operationId,
    subjectId,
    action: input.action,
    payloadDigest,
    effectAuthority: false,
    filesystemAuthority: false,
    notificationAuthority: false,
  });
}

export function observeNativeOutcome(input) {
  requireRecord(input, "input");
  const operationId = requireStableId(input.operationId, "operationId");
  if (input.terminalObserved !== true) {
    return Object.freeze({
      operationId,
      status: "indeterminate",
      outcomeDigest: null,
      effectAuthority: false,
    });
  }
  const outcomeDigest = requireDigest(input.outcomeDigest, "outcomeDigest");
  return Object.freeze({
    operationId,
    status: "succeeded",
    outcomeDigest,
    effectAuthority: false,
  });
}
''',
    "apps/hepta-native/test/native.test.js": r'''import assert from "node:assert/strict";
import test from "node:test";

import { buildNativeIntent, observeNativeOutcome } from "../src/native.js";

const digest = "cd".repeat(32);

test("native intent requires exact payload binding", () => {
  const intent = buildNativeIntent({
    operationId: "operation:1",
    subjectId: "path:1",
    action: "request_open_path",
    payloadDigest: digest,
    leasePayloadDigest: digest,
  });
  assert.equal(intent.effectAuthority, false);
  assert.equal(intent.filesystemAuthority, false);
  assert.equal(intent.notificationAuthority, false);
});

test("payload drift fails closed", () => {
  assert.throws(
    () =>
      buildNativeIntent({
        operationId: "operation:1",
        subjectId: "path:1",
        action: "request_open_path",
        payloadDigest: digest,
        leasePayloadDigest: "ab".repeat(32),
      }),
    /does not match/,
  );
});

test("unobserved outcome remains indeterminate", () => {
  assert.deepEqual(
    observeNativeOutcome({
      operationId: "operation:1",
      terminalObserved: false,
      outcomeDigest: digest,
    }),
    {
      operationId: "operation:1",
      status: "indeterminate",
      outcomeDigest: null,
      effectAuthority: false,
    },
  );
});
''',
    "apps/hepta-native/README.md": """# Hepta native shell

This root defines bounded native-operation intents and terminal observations.
It does not call platform APIs directly and grants no filesystem, notification,
physical-effect, promotion or release authority.
""",
    "codex-rs/codex-app-server/BINDING.json": """{
  "schema_version": 1,
  "module": "runtime.codex",
  "declared_root": "codex-rs/codex-app-server",
  "implementation_root": "codex-rs/app-server",
  "binding_mode": "canonical_alias",
  "duplicate_cargo_package_created": false,
  "model_authority": false,
  "provider_authority": false,
  "interpretation": "the alias binds the plan root to the existing app-server package without creating a second implementation"
}
""",
    "codex-rs/codex-app-server/README.md": """# Codex app-server binding

The canonical plan name `codex-rs/codex-app-server` is bound to the existing
workspace package at `codex-rs/app-server`. No duplicate Cargo package or second
runtime implementation is introduced. Hepta-specific authority checks live in
`codex-rs/hepta-codex-adapter`.
""",
    "external/HeptaBao/EXTERNAL_SOURCE.json": """{
  "schema_version": 1,
  "module": "secrets.heptabao",
  "repository": "TrillionniumFoundation/HeptaBao",
  "branch": "main",
  "commit": "1c69131d7251bc02ebeab726689ecd53bce89968",
  "integration_mode": "opaque_reference_adapter",
  "contains_secret_material": false,
  "automatic_update": false,
  "runtime_authority": false,
  "release_authority": false,
  "interpretation": "this record pins external source identity and does not vendor secrets or grant runtime access"
}
""",
    "external/HeptaBao/README.md": """# HeptaBao external source binding

The secret-boundary adapter is evaluated against
`TrillionniumFoundation/HeptaBao@1c69131d7251bc02ebeab726689ecd53bce89968`.
Only opaque identity, version and digest references cross the boundary. This
directory contains no credential or secret payload.
""",
}
