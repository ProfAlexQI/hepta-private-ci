# Hepta Inference Performance Contract v1 (planning)

## Status

`PLANNING_ONLY` / `qualification_only`. This document defines admission and
measurement semantics; it does not claim that any model has passed a hardware
benchmark.

## Canonical admission tuple

An inference route is valid only when all fields below match the signed registry
entry and the active `RunStartSnapshot`:

```text
model_digest
tokenizer_digest
backend_name
backend_commit
backend_abi
compiler_id/version/build_flags
driver_id/version
device_profile_digest
thermal_mode/power_mode
model_format/quantization
context_length
batch_or_continuous_batch_config
kv_cache/prefix_cache_config
compiled_artifact_digest
sbom_digest/license_digest
```

Any missing, stale, unsigned or unbenchmarked field causes `NotAdmitted`.
Admission code must not choose a different CPU, remote endpoint, quantization or
backend to make a request appear successful.

## Benchmark protocol

For each tuple, run the same fixture through direct native invocation and the
Hepta sidecar, with identical model files, tokenizer, context, batch, thread
count, thermal/power mode and output limits. Report cold and warm runs separately;
do not mix model load or queue wait into decode throughput.

Required metrics include:

- p50/p95 queue wait, model load, TTFT, end-to-end latency;
- prefill/decode tokens per second and generated token count;
- ASR real-time factor and audio duration;
- image seconds/step, resolution, sampler and step count;
- RSS/VRAM, CPU/GPU/NPU utilization, power, temperature and throttling;
- cache hit/miss, cancellation, worker restart and error/abstention rates.

## Relative gates

Until INF-0 produces device-specific baselines, these are provisional review
gates:

- warm sidecar p95 overhead ≤ `max(10 ms, 10%)` versus direct native;
- warm streaming throughput ≥ 90% of direct native;
- ASR RTF and image latency ≤ 1.1× direct native;
- cold-start, thermal and queue metrics are separate gates, never hidden.

The performance reviewer may tighten a gate per device/task, but may not loosen it
without a signed change receipt. A failed tuple is retained as `KNOWN_GAP` and
removed from the default router manifest.

## Reproducibility and supply chain

Every receipt must include source head/tree/dirty, backend commit, compiler/driver,
model/tokenizer/compiled artifact/SBOM/license digests, benchmark fixture digest,
thread/CPU affinity and environment variables that affect execution. Model files
must be signed and license-checked before entering the registry.
