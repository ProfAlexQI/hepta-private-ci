# Hepta Servo patch queue

This directory governs Hepta-specific changes applied to the exact Servo source
identified by `docs/hepta-vnext/browser/SERVO_UPSTREAM_PIN.json`.

It does **not** contain Servo source, a Servo binary, a browser runtime, or any
production authority. The current queue is intentionally empty.

Every future patch must be an ordered `git format-patch` file and must have one
entry in `PATCH_INVENTORY.json` with its exact SHA-256, affected files, reason,
security boundary, compatibility tests, upstream issue or pull request, and a
clear deletion condition. A patch that lacks any of those fields is not an
implementation input.

The first planned patch is a private inherited-channel worker entrypoint. It
must not call Servo's WebDriver HTTP `start_server`, bind TCP, expose raw
WebDriver, enable external network access, or alter Hepta authority flags.
