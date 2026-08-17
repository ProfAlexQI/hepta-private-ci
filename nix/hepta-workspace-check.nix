{
  cargo-nextest,
  discoveryOnly ? false,
  expectedInventory ? null,
  lib,
  product,
  python3,
}:
let
  suitePackages = [
    "codex-hepta-contracts"
    "codex-hepta-evidence"
    "codex-hepta-governance"
    "codex-hepta-memory"
    "codex-hepta-memory-extension"
    "codex-hepta-mnl-replay-v1"
    "codex-hepta-mnl-trust-v1"
    "codex-hepta-native-gateway"
    "codex-hepta-nix-mnl-v1"
    "codex-hepta-paths"
    "codex-hepta-runtime"
  ];
  packageFlags = lib.concatMapStringsSep " " (package: "-p ${lib.escapeShellArg package}") suitePackages;
  parser = ./hepta-check-suite-v1.py;
  nextestConfig = ./hepta-nextest.toml;
in
assert lib.assertMsg (
  lib.getVersion cargo-nextest == "0.9.124"
) "workspace check requires exact cargo-nextest 0.9.124";
assert lib.assertMsg (
  discoveryOnly || expectedInventory != null
) "full workspace check requires a separately reviewed frozen expected inventory";
product.overrideAttrs (previous: {
  pname = "hepta-nix-mnl-exact-check-suite";
  doCheck = false;

  # The product derivation rewrites the workspace's placeholder version for
  # packaging. This check intentionally tests the exact committed Cargo files,
  # so preserve Cargo.lock compatibility instead of inheriting that mutation.
  postPatch = "";

  nativeBuildInputs = (previous.nativeBuildInputs or [ ]) ++ [
    cargo-nextest
    python3
  ];

  buildPhase = ''
    runHook preBuild

    export CARGO_BUILD_JOBS=1
    export CARGO_INCREMENTAL=0
    export CARGO_NET_OFFLINE=true
    export CARGO_TARGET_DIR="$TMPDIR/hepta-nextest-target"
    export CARGO_TERM_COLOR=never
    export NO_COLOR=1
    export RUST_BACKTRACE=0

    work="$TMPDIR/hepta-check-suite-v1"
    mkdir -p "$work"

    cargo --version >"$work/cargo-version.txt"
    rustc --version >"$work/rustc-version.txt"
    cargo nextest --version >"$work/runner-version.txt"

    ${python3}/bin/python3 ${parser} preflight-tool-versions \
      --cargo-version-file "$work/cargo-version.txt" \
      --rustc-version-file "$work/rustc-version.txt" \
      --runner-version-file "$work/runner-version.txt"

    cargo metadata \
      --locked \
      --offline \
      --no-deps \
      --format-version 1 \
      --manifest-path Cargo.toml \
      >"$work/cargo-metadata.json"

    ${python3}/bin/python3 ${parser} preflight-config \
      --nextest-config ${nextestConfig}

    ${lib.optionalString (!discoveryOnly) ''
      ${python3}/bin/python3 ${parser} preflight-metadata \
        --cargo-metadata-json "$work/cargo-metadata.json" \
        --expected-inventory ${expectedInventory}
    ''}

    cargo nextest \
      --user-config-file none \
      --config-file ${nextestConfig} \
      --profile default \
      list \
      --ignore-default-filter \
      --locked \
      --offline \
      ${packageFlags} \
      --list-type full \
      --message-format json \
      >"$work/list.json" \
      2>"$work/list.stderr"

    ${python3}/bin/python3 ${parser} discover \
      --cargo-metadata-json "$work/cargo-metadata.json" \
      --list-json "$work/list.json" \
      --output "$work/discovered-inventory.json"

    ${lib.optionalString (!discoveryOnly) ''
      cmp --silent \
        "$work/discovered-inventory.json" \
        ${expectedInventory}

      NEXTEST_EXPERIMENTAL_LIBTEST_JSON=1 \
        cargo nextest \
          --user-config-file none \
          --config-file ${nextestConfig} \
          --profile default \
          run \
          --ignore-default-filter \
          --locked \
          --offline \
          ${packageFlags} \
          --no-fail-fast \
          --no-tests fail \
          --retries 0 \
          --test-threads 1 \
          --message-format libtest-json-plus \
          --message-format-version 0.1 \
          >"$work/events.jsonl" \
          2>"$work/run.stderr"

      ${python3}/bin/python3 ${parser} verify \
        --cargo-metadata-json "$work/cargo-metadata.json" \
        --list-json "$work/list.json" \
        --events-jsonl "$work/events.jsonl" \
        --discovered-inventory "$work/discovered-inventory.json" \
        --expected-inventory ${expectedInventory} \
        --cargo-lock Cargo.lock \
        --nextest-config ${nextestConfig} \
        --cargo-version-file "$work/cargo-version.txt" \
        --rustc-version-file "$work/rustc-version.txt" \
        --runner-version-file "$work/runner-version.txt" \
        --output "$work/check-suite-v1.json"
    ''}

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    mkdir -p "$out/share/hepta/check-suite-v1"

    ${if discoveryOnly then ''
      cp "$TMPDIR/hepta-check-suite-v1/discovered-inventory.json" \
        "$out/share/hepta/check-suite-v1/discovered-inventory.json"
      printf '%s' \
        '{"authorizes_pass":false,"schema":"hepta_nix_mnl_check_suite_discovery_only_v1","schema_version":1}' \
        >"$out/share/hepta/check-suite-v1.json"
    '' else ''
      cp ${expectedInventory} \
        "$out/share/hepta/check-suite-v1/discovered-inventory.json"
      cp "$TMPDIR/hepta-check-suite-v1/check-suite-v1.json" \
        "$out/share/hepta/check-suite-v1.json"
    ''}

    runHook postInstall
  '';
})
