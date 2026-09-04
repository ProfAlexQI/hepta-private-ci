import assert from "node:assert/strict";
import test from "node:test";

import { buildOperationIntent, projectRuntime } from "../src/control.js";

const digest = "ab".repeat(32);

test("runtime projection exposes only registered safe fields", () => {
  const projection = projectRuntime({
    moduleId: "runtime.agentd",
    status: "ready",
    revision: 7,
    digest,
    secret: "must-not-leak",
    providerPayload: { token: "must-not-leak" },
  });

  assert.deepEqual(projection, {
    moduleId: "runtime.agentd",
    status: "ready",
    revision: 7,
    digest,
    ready: true,
    authorityGranted: false,
    directStoreWrite: false,
  });
  assert.equal(Object.isFrozen(projection), true);
  assert.equal("secret" in projection, false);
  assert.equal("providerPayload" in projection, false);
});

test("operation intent cannot issue authority or write a store", () => {
  const intent = buildOperationIntent({
    operationId: "operation:1",
    subjectId: "module:1",
    action: "request_quarantine",
    expectedRevision: 3,
  });

  assert.equal(intent.authorityGranted, false);
  assert.equal(intent.directStoreWrite, false);
  assert.equal(intent.kind, "OperationIntentV1");
});

test("unknown actions fail closed", () => {
  assert.throws(
    () =>
      buildOperationIntent({
        operationId: "operation:1",
        subjectId: "module:1",
        action: "merge_and_release",
        expectedRevision: 3,
      }),
    /registered operator request/,
  );
});

test("invalid digest is rejected", () => {
  assert.throws(
    () =>
      projectRuntime({
        moduleId: "runtime.agentd",
        status: "ready",
        revision: 7,
        digest: "not-a-digest",
      }),
    /64 lowercase hexadecimal/,
  );
});
