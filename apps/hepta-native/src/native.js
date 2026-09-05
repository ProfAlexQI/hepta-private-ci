const STABLE_ID = /^[A-Za-z0-9._:-]{1,128}$/;
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
