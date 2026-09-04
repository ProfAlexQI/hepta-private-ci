import assert from "node:assert/strict";
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
