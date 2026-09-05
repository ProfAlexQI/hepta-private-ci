const STABLE_ID = /^[A-Za-z0-9._:-]{1,128}$/;
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
