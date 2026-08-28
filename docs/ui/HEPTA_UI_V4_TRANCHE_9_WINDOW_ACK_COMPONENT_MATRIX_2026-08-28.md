# Hepta UI v4 — Tranche 9: Exact Window Acknowledgement and Native Component Matrix

## Identity

- Plan lane: U3 / U4 / U8
- Candidate branch: `codex/ui-v4-window-ack-matrix-20260828`
- Base branch: `codex/ui-v4-window-visuals-diagnostics-20260828`
- Exact base commit: `ac36eb8f6ebff88513815286a56b876bb15d57c2`
- Exact base tree: `6b6f2ac46d10d6160f806e7dc545d8d699f84757`
- Source candidate identity: supplied by the Git commit and bound by the workflow receipt; this document does not use a self-referential commit literal.

This tranche is source implementation and qualification preparation. It does not grant network, mutation, effect, live-adapter, production, operator-acceptance, promotion, or release authority.

## Delivered source

### Exact root-window acknowledgement verifier

`hepta_window_visual_ack.rs` creates an acknowledgement request identity only when the preceding Makepad receipt proves that an exact `SetWindowVisuals` operation was queued. The identity binds:

- request sequence;
- platform;
- Makepad WindowId index;
- Makepad WindowId generation;
- normalized requested visuals;
- persistent-chrome intent.

Supported backend identities are currently limited to Windows DWM and macOS AppKit. The verifier rejects stale sequence or window generation, platform/backend mismatch, backend failure, missing or unexpected readback, and visual readback mismatch.

Accepted acknowledgement is intentionally partial. It may prove persistent root chrome or solid fallback handling, with or without exact readback. It cannot prove transient material, complete platform profile, system-material binding, product runtime, or authority.

No backend acknowledgement producer is bound in this tranche.

### Nine-scenario isolated Native component matrix

The matrix producer renders the canonical shared RoomFilterInputBar and measures actual Makepad areas for the filter, input, and clear target. It covers:

- 320, 360, 390, 412, and 600 logical-width scenarios;
- 844 × 390 landscape;
- a disclosed synthetic safe-area layout;
- a disclosed synthetic keyboard-inset layout;
- a disclosed 200% UI-scale layout.

Every scenario binds exact candidate commit/tree, an exact Makepad WindowId, actual viewport/DPI, actual component rectangles, rendered-frame bytes, and screenshot SHA-256.

Synthetic safe-area, keyboard, and scale scenarios are component-layout evidence only. They are not iOS/Android device, Dynamic Type, or IME evidence.

## Qualification boundary

The source gate can emit only:

`PASS_WINDOW_ACK_COMPONENT_MATRIX_SOURCE_ONLY`

It must keep all of the following false:

- Native compile validation;
- backend acknowledgement runtime validation;
- runtime readback;
- component matrix runtime validation;
- screenshot validation;
- Native product runtime validation;
- system material binding;
- device validation;
- production/effect/live-adapter authority;
- operator acceptance;
- promotion and release.

The hosted workflow schedules source validation, Linux/Windows/macOS compilation and focused tests, plus an explicitly dispatched Xvfb nine-scenario producer. Workflow creation or a job with no executable steps is neither PASS nor source failure.

## Remaining work

1. Execute exact-head source, formatting, compile, and focused Rust tests on assigned runners.
2. Implement a Windows DWM acknowledgement producer bound to the exact request identity.
3. Implement a macOS AppKit acknowledgement producer with real readback where supported.
4. Add a separately governed transient host; root-window persistent chrome cannot stand in for Acrylic, popover, or sheet material.
5. Execute and review the nine component screenshots.
6. Extend qualification to product-level Native flows and physical iOS/Android devices.
7. Keep full platform host, production authority, effect authority, operator acceptance, promotion, and release fail-closed until independent receipts exist.
