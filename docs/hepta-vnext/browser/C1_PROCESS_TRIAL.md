# WEB-C1.3B — inherited anonymous-pipe parent/worker process trial

Status: `IMPLEMENTED_QUALIFICATION_ONLY / SERVO_NOT_LINKED / EVIDENCE_PENDING`  
Authority: none.  
Relationship: runtime exercise of the C1.3 protocol; it does not complete WEB-C1.

## Topology

The qualification binary starts a second copy of itself with exactly one nonsecret `--worker`
argument. The parent creates anonymous stdin/stdout pipes before spawn. No TCP, UDP, HTTP,
WebSocket, WebDriver, CDP, filesystem socket, discovery record, or external network endpoint exists.

```text
parent trial process
   | child stdin: bootstrap + HostAck + Command
   | child stdout: WorkerHello + WorkerConfirm + Outcome
   v
worker trial process
```

The parent reads 64 bytes from the Unix private-random source:

```text
32-byte startup capability
32-byte host nonce
```

Those bytes are written into the already inherited child-input pipe before protocol framing begins.
They are never placed in argv, environment variables, JSON output, receipts, or filesystem files.
The worker reads exactly 64 bytes, constructs the expected binding, clears its temporary bootstrap
buffers, and starts `WorkerHello`.

## Exact trial

1. spawn the child with piped stdin/stdout and inherited stderr;
2. generate nonzero bootstrap capability and nonce;
3. write and flush the 64-byte private bootstrap;
4. complete WorkerHello → HostAck → WorkerConfirm;
5. require the exact qualification-only authority posture;
6. send request 1 `Ping` at page revision 1;
7. require exact `Completed / pong` outcome;
8. send request 2 `Shutdown`;
9. require exact `Completed / shutdown_complete` outcome;
10. close the framed channel;
11. wait for successful child exit;
12. print one compact nonsecret result line.

Any mismatch, EOF, malformed frame, stale identity, wrong capability/nonce, unexpected message,
noncompleted result, or unsuccessful child exit fails the parent process.

## Nonclaims

The process trial does not:

- link, import, build, or execute Servo;
- create a WebView, page, renderer, profile, or browser sandbox;
- prove executable/source correspondence;
- prove descendant cleanup beyond the single qualification child;
- implement timeouts or forced termination;
- implement Windows named pipes;
- prove there are no ambient inherited descriptors beyond stdin/stdout/stderr;
- enable navigation, egress, credentials, production/effect authority, operator acceptance,
  promotion, or release.

## Qualification requirements

- integration test launches the binary as a real child process;
- stdout must equal the exact compact negative-authority result;
- stderr must be empty;
- unknown arguments must fail with no stdout;
- direct workflow execution must produce the same result;
- the qualification receipt records `servo_linked=false`, `real_webview=false`,
  `external_network=false`, and all authority flags false;
- the binary must remain in the zero-third-party-dependency standalone crate.

## Next process-boundary work

1. add bounded monotonic handshake/read/write/exit deadlines;
2. implement forced termination and deterministic reap evidence;
3. inventory inherited descriptors/handles;
4. replace trial child with the first source/artifact-bound Servo worker;
5. implement Windows one-client named-pipe/SID-ACL equivalent;
6. bind process executable SHA-256 to C1-004B artifact receipt;
7. prove no listener and no external egress at OS level;
8. create one local-fixture WebView and connect it to BrowserActor fences.
