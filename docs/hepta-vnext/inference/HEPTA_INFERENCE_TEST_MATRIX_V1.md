# Hepta Local Inference Test Matrix V1

Every row is required unless its evidence class is explicitly removed from supported
scope. `Source` means compiled/tested source, not real execution.

| ID | Area | Scenario | Expected result | Class |
|---|---|---|---|---|
| T0-01 | Truth | canonical pointer references all active files | pass, no missing path | E0 |
| T0-02 | Truth | required job skipped/empty/runner_id=0 | aggregate fails | E0 |
| T0-03 | Truth | descendant replays historical INF-0C receipt | ancestry/digest check, no schema rewrite | E0 |
| P-01 | Protocol | canonical CBOR round trip | byte-stable | E0 |
| P-02 | Protocol | oversized, truncated, trailing, indefinite frame | fail closed | E0/E1 |
| A-01 | Authority | public client sends Start/Token/Complete/Restart | encode impossible or role rejection | E0/E1 |
| A-02 | Authority | request ID stolen without capability | rejected | E1 |
| A-03 | Authority | capability used by wrong session | rejected | E1 |
| A-04 | Authority | stale worker nonce after restart | rejected | E1 |
| A-05 | Authority | any Memory/KG/remote/production authority | absent and rejected | E0/E1 |
| R-01 | Resource | fill global queue | next admit rejected | E0/E1 |
| R-02 | Resource | admit/start loop exceeds inflight/running | bounded rejection | E0/E1 |
| R-03 | Resource | tenant or model monopolizes capacity | bounded/fair rejection | E0/E1 |
| R-04 | Resource | slow partial frame connections | timed out and semaphore recovered | E1 |
| R-05 | Resource | receipt disk budget reached | deterministic eviction/refusal | E1 |
| D-01 | Deadline | queued deadline expires | terminal receipt and accounting release | E1 |
| D-02 | Deadline | running deadline expires, worker ACKs | acknowledged cancellation receipt | E1 |
| D-03 | Deadline | running deadline expires, worker hangs | observed kill, generation rollover | E1 |
| I-01 | Integrity | token sequence gap/replay | rejected | E0/E1 |
| I-02 | Integrity | token count/bytes exceed request limits | rejected/fail closed | E0/E1 |
| I-03 | Integrity | final digest differs from rolling digest | rejected | E0/E1 |
| I-04 | Integrity | wrong model/runtime/device digest | rejected before load/result | E1/E3 |
| J-01 | Journal | crash before file sync | no false durable receipt | E1 |
| J-02 | Journal | restart and query terminal receipt | recovered | E1 |
| J-03 | Journal | conflicting duplicate/replay | fail closed | E1 |
| J-04 | Journal | TTL expiry and compaction | deterministic, bounded, indexed | E1 |
| W-01 | Worker | authenticated fixture start/heartbeat | ready | E1 |
| W-02 | Worker | child crash/protocol violation | affected requests fail closed | E1 |
| W-03 | Worker | child stdout/stderr flood | bounded/discarded | E1 |
| W-04 | Worker | drain/unload/restart | no stale event accepted | E1 |
| C-01 | Cancel | queued cancel | immediate, no worker dispatch | E1 |
| C-02 | Cancel | cancel before first token | ACK or observed kill | E1/E2/E3 |
| C-03 | Cancel | cancel during stream | no post-terminal token accepted | E1/E2/E3 |
| C-04 | Cancel | colocated requests after forced kill | all fail closed, generation rolls | E1/E3 |
| O-01 | Ollama | exact model readiness/semantic output | verified, no pull | E2 |
| O-02 | Ollama | redirect/proxy/non-loopback | rejected | E2 |
| O-03 | Ollama | disconnect vs cancel ACK | not conflated | E2 |
| L-01 | LM Studio | exact model readiness/semantic output | verified, no install | E2 |
| L-02 | LM Studio | malformed JSON/SSE/media type | rejected | E2 |
| L-03 | LM Studio | helper replacement/mode/env secret | rejected/no leak | E2 |
| N-01 | Native | exact llama.cpp + GGUF load/warm | passes exact digests | E3 |
| N-02 | Native | generate real UTF-8 tokens | receipt reconciles | E3 |
| N-03 | Native | wrong GGUF/tokenizer/template | rejected | E3 |
| N-04 | Native | unload and memory reclamation | within device profile budget | E3/E4 |
| S-01 | Scheduler | fairness and tenant starvation | deterministic fair progress | E1 |
| S-02 | Lease | cancel/restart revokes input/KV/prefix lease | no stale access | E1 |
| S-03 | Cache | key mismatch across model/template/policy | miss, never cross-use | E1 |
| S-04 | Memory | pressure eviction | budget never exceeded | E1/E4 |
| B-01 | Product | opt-in shadow success | authoritative output unchanged | E5 |
| B-02 | Product | shadow failure/cancel | isolated and kill-switchable | E5 |
| B-03 | Product | Memory/KG writer probe | no authority | E5 |
| E-01 | Device | cold/warm/TTFT/tok-s/RSS/VRAM | profile thresholds pass | E4 |
| E-02 | Device | 30-minute soak | bounded memory/disk, no stale state | E4 |
| E-03 | Device | crash/cancel recovery latency | profile thresholds pass | E4 |
| OPR-01 | Operator | evidence completeness and provenance | independent acceptance | E6 |
| OPR-02 | Operator | rollback/upgrade/downgrade/kill switch | pass | E6 |
| REL-01 | Release | activation attempted without E6/E7 | denied | E7 |

## Required workflow structure

The final aggregate has explicit jobs for E0 through E6. Jobs whose prerequisite is not
available must fail with a precise prerequisite code or report `BLOCKED_PREREQUISITE`;
they must not be silently skipped and must not be counted as passing.

## Minimum commands for every Rust inference crate

```bash
cargo fmt -p <package> -- --check
cargo check --locked --all-targets -p <package>
cargo test --locked --all-targets -p <package>
cargo clippy --locked --all-targets --no-deps -p <package> -- -D warnings
```

Use repository `just` commands where required by `AGENTS.md`, and update Bazel lock/data
when dependencies or compile-time files change.
