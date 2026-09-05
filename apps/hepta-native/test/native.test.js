import assert from "node:assert/strict";
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
