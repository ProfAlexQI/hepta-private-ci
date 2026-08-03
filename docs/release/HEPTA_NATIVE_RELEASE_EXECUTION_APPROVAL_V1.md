# Hepta Native release execution approval v1

Status: implemented, fail-closed, no production approval currently installed.

This contract authorizes one narrowly described local macOS release operation:
Developer ID signing, Apple notarization submission, ticket stapling, and the
local write of one exact DMG. It does **not** authorize or perform an upload,
release publication, channel delivery, or Public GA claim.

## Trust boundary

`scripts/hepta-ui-release-execution-approval-verifier-v1` is a read-only
consumer. It never signs an approval, reads a private key, calls Apple, opens a
network connection, uploads an artifact, or changes a release output. The
approval JSON and detached signature must be produced by independent release
authority outside the packaging process.

The packaging caller supplies three independent authority files:

- the signed approval JSON;
- its detached RSA PKCS#1 SHA-256 signature;
- the trusted RSA public key (3072 bits or stronger, public material only).

The signer id and public-key hash are pinned in the fixed, source-controlled
`apps/hepta-native/packaging/release-execution-approval-trust-v1.json` policy;
they are not selected by the release command. Values copied from an approval
or supplied alongside it are not an independent trust decision. The committed
policy is intentionally `not_configured` until real release authority provides
its signer id and public-key SHA-256. That state cannot authorize execution.

## Exact signed payload

The approval uses kind `hepta-ui-release-execution-approval-v1`, schema version
1, strict field sets, and duplicate-key rejection. Its signature covers the
exact JSON bytes. It binds:

- approval UUID, signer id, public-key hash, and `rsa-pkcs1-sha256`;
- UTC issue and expiry times, with a maximum one-hour window;
- clean exact source head, head tree, and source fingerprint;
- absolute unsigned app and current-package receipt paths;
- unsigned receipt, app bundle fingerprint, and executable SHA-256 values;
- packaging script, approval verifier, and fixed trust-policy SHA-256 values;
- product version, architecture, output, release receipt, and evidence paths;
- exact Developer ID Application identity, its installed 40-hex certificate
  identity hash, and 10-character Team ID;
- exact entitlements file SHA-256;
- a SHA-256 of the selected notarytool keychain-profile name;
- the action `sign_notarize_staple_local_dmg`;
- signing, notarization submission, stapling, and local artifact write set to
  `true`;
- `public_distribution_authorized=false`;
- `public_upload_authorized=false` and `public_upload_performed=false`.

The notary profile hash is computed over the profile name with no newline:

```sh
printf '%s' "$HEPTA_NOTARY_PROFILE" | shasum -a 256
```

When release governance intentionally provisions the fixed policy, its pinned
public-key hash is computed over the exact PEM file bytes:

```sh
shasum -a 256 /absolute/path/release-approval-public.pem
```

An independent authority signs the final, reviewed JSON bytes with its private
key. The private key must never be passed to Hepta packaging. A typical offline
RSA signing command is shown only to define the detached signature format:

```sh
openssl dgst -sha256 \
  -sign /offline/authority/private-key.pem \
  -out /transfer/release-approval.sig \
  /transfer/release-approval.json
```

## Production wiring

Actual execution requires a prebuilt formal unsigned app and its exact
current-package receipt. This prevents the release command from building a new
input after authority approved a different tuple.

```sh
HEPTA_SIGNING_IDENTITY='Developer ID Application: … (TEAMID1234)' \
HEPTA_EXPECTED_TEAM_ID='TEAMID1234' \
HEPTA_NOTARY_PROFILE='hepta-notary-profile' \
apps/hepta-native/packaging/build-macos-dmg.sh \
  --app-path /absolute/path/Hepta.app \
  --app-receipt /absolute/path/current-package.json \
  --output /absolute/path/Hepta.dmg \
  --receipt /absolute/path/Hepta.dmg.receipt.json \
  --release-approval /absolute/path/release-approval.json \
  --release-approval-signature /absolute/path/release-approval.sig \
  --release-approval-public-key /absolute/path/release-approval-public.pem
```

Missing approval inputs stop before tool discovery or output creation with exit
77. Partial input sets are usage errors. An unconfigured trust policy and any
invalid, expired, untrusted, changed, or tuple-mismatched approvals stop with
exit 77 after the unsigned input check and before the first `codesign` call.
The normalized verification receipt is included in release evidence and the
final release receipt.

Even a successful local run records `public_upload_performed=false` and keeps
public-distribution, release, and live-product claim readiness false. A later
publication workflow needs separate authority and evidence.

The success receipt may still report
`public_distribution_artifact_written=true`: that is an observed capability of
the signed, notarized DMG written locally, not authority to publish it. The
separate `public_distribution_authorized=false`,
`public_upload_performed=false`, and claim-boundary fields remain controlling.

## Tests and present blocker

Run:

```sh
scripts/hepta-ui-release-execution-approval-verifier-self-test.sh
scripts/hepta-native-macos-release-chain-self-test.sh
```

The first test creates an ephemeral test-only RSA key and synthetic approval in
a private temporary directory, exercises the verifier, then deletes all test
material. Its fixture is never accepted or installed as production evidence.
The negative matrix covers forged signatures, wrong trust anchors, private and
weak key input, duplicate JSON keys, symlink/hardlink paths, stale/future
approval, source/input/action mismatch, upload authorization, and upload-state
forgery.

This wiring does not manufacture missing external conditions. The checked-in
trust policy is currently unconfigured. With no provisioned independent trust
root, valid Developer ID identity, notarytool profile, independently signed
approval, and Apple notarization response, release execution and Public GA
remain blocked.
