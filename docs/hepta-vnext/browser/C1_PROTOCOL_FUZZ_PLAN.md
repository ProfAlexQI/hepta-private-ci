# WEB-C1.3 parser and state-machine fuzz plan

Status: `PLANNED / REQUIRED_BEFORE_REAL_SERVO_WORKER`

## Targets

1. `decode_message` with arbitrary byte slices from 0 to 65,537 bytes;
2. length-prefixed `read_message` with fragmented and short reads;
3. every valid message encoded then mutated at every byte position;
4. handshake state order with duplicate, omitted, reordered, or replayed frames;
5. string boundaries at 0, 1, maximum, and maximum+1 bytes;
6. integer boundaries for request ID, generation, epoch, revision, lease, and observe limit;
7. UTF-8 valid/invalid prefixes and overlong/truncated sequences;
8. source-pin lowercase/uppercase/nonhex/short/long forms;
9. authority bit combinations across all 16 bits;
10. established-channel stale identity and post-handshake handshake-message injection.

## Properties

- no panic, abort, stack overflow, or process exit;
- no allocation larger than the declared frame/string bounds;
- decoder either consumes exactly one frame or returns a typed error;
- `decode(encode(x)) == x` for every valid message;
- invalid input is never normalized into a different valid identity;
- unknown authority bits never become a negative/ignored extension;
- no secret-bearing type appears in error or Debug output;
- no rejected frame changes the established binding;
- no parser error authorizes retry, network, credential, or effect behavior.

## Corpus

Seed corpus must include one canonical frame per message/command/outcome kind, minimum and maximum
field sizes, wrong magic/version, truncated prefix/body, trailing byte, unknown kind, zero identity,
stale generation, wrong capability, wrong host nonce, and the exact pinned Servo commit/tree.

## Evidence

The eventual qualification receipt records:

- fuzzer/tool version and artifact digest;
- source commit/tree and protocol version;
- seed corpus digest;
- duration and total executions;
- peak RSS and maximum observed allocation;
- crash/hang count;
- minimized findings and disposition;
- all authority flags false.

Fuzzing is additional evidence. It does not replace deterministic protocol, integration, sandbox,
platform, or source/artifact qualification.
