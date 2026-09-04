const RUNTIME_STATUSES = new Set([
  "ready",
  "degraded",
  "quarantined",
  "recovering",
  "unavailable",
]);

const OPERATION_ACTIONS = new Set([
  "request_quarantine",
  "request_reconcile",
  "request_retry",
  "request_rollback",
]);

const STABLE_ID = /^[A-Za-z0-9._:-]{1,128}$/;
const DIGEST = /^[0-9a-f]{64}$/;

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

function requireRevision(value) {
  if (!Number.isSafeInteger(value) || value < 1) {
    throw new TypeError("revision must be a positive safe integer");
  }
  return value;
}

function requireDigest(value) {
  if (typeof value !== "string" || !DIGEST.test(value)) {
    throw new TypeError("digest must contain 64 lowercase hexadecimal characters");
  }
  return value;
}

export function projectRuntime(observation) {
  requireRecord(observation, "observation");
  const moduleId = requireStableId(observation.moduleId, "moduleId");
  if (!RUNTIME_STATUSES.has(observation.status)) {
    throw new TypeError("status is not a registered runtime state");
  }
  const revision = requireRevision(observation.revision);
  const digest = requireDigest(observation.digest);

  return Object.freeze({
    moduleId,
    status: observation.status,
    revision,
    digest,
    ready: observation.status === "ready",
    authorityGranted: false,
    directStoreWrite: false,
  });
}

export function buildOperationIntent(input) {
  requireRecord(input, "input");
  const operationId = requireStableId(input.operationId, "operationId");
  const subjectId = requireStableId(input.subjectId, "subjectId");
  if (!OPERATION_ACTIONS.has(input.action)) {
    throw new TypeError("action is not a registered operator request");
  }
  const expectedRevision = requireRevision(input.expectedRevision);

  return Object.freeze({
    kind: "OperationIntentV1",
    operationId,
    subjectId,
    action: input.action,
    expectedRevision,
    authorityGranted: false,
    directStoreWrite: false,
  });
}
