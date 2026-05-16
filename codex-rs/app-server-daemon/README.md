# hepta-app-server-daemon

> `hepta-app-server-daemon` is experimental and its lifecycle contract may
> change while the remote-management flow is still being developed.

`hepta-app-server-daemon` backs the machine-readable `hepta app-server`
lifecycle commands used by remote clients such as the desktop and mobile apps.
It is intended for Hepta instances launched over SSH, including fresh developer
machines that should expose app-server with `remote_control` enabled.

## Platform support

The current daemon implementation is Unix-only. It uses pidfile-backed
daemonization plus Unix process and file-locking primitives, and does not yet
support Windows lifecycle management.

## Commands

```sh
hepta app-server daemon start
hepta app-server daemon restart
hepta app-server daemon enable-remote-control
hepta app-server daemon disable-remote-control
hepta app-server daemon stop
hepta app-server daemon version
hepta app-server daemon bootstrap --remote-control
```

On success, every command writes exactly one JSON object to stdout. Consumers
should parse that JSON rather than relying on human-readable text. Lifecycle
responses report the resolved backend, socket path, local CLI version, and
running app-server version when applicable.

## Bootstrap flow

For a new remote machine, stage a managed Hepta build first:

```sh
$HOME/.hepta/packages/standalone/current/hepta app-server daemon bootstrap --remote-control
```

`bootstrap` requires the standalone managed install. It records the daemon
settings under `HEPTA_HOME/app-server-daemon/`, starts app-server as a
pidfile-backed detached process, and launches a detached updater loop. The
source fork currently leaves the standalone updater disabled until a Hepta
release feed exists.

## Installation and update cases

The daemon assumes Hepta has a staged standalone managed binary under
`HEPTA_HOME`.

| Situation | What starts | Does this daemon fetch new binaries? | Does a running app-server eventually move to a newer binary on its own? |
| --- | --- | --- | --- |
| Managed Hepta binary is staged, but only `start` is used | `start` uses `HEPTA_HOME/packages/standalone/current/hepta` | No | No. The managed path is used when starting or restarting, but no updater is installed. |
| Managed Hepta binary is staged, then `bootstrap` is used | The pidfile backend uses `HEPTA_HOME/packages/standalone/current/hepta` | No. The Hepta source fork does not fetch upstream Codex installers. | No. The updater loop exits until a Hepta release feed is configured. |
| Some other tool updates the managed binary path | The next fresh start or restart uses the updated file at that path | No. | Without an enabled Hepta updater, no. |

### Standalone installs

For staged Hepta standalone installs:

- lifecycle commands always use the standalone managed binary path
- `bootstrap` is supported
- `bootstrap` starts a detached pid-backed updater loop, but the current fork
  intentionally fails closed instead of fetching upstream Codex installers
- the updater loop is not reboot-persistent; it must be started again by
  rerunning `bootstrap` after a reboot

### Out-of-band updates

This daemon does not watch arbitrary executable files for replacement. If some
other tool updates the managed binary path:

- without `bootstrap`, a currently running app-server remains on the old
  executable image until an explicit `restart`
- with `bootstrap`, no automatic refresh happens until a Hepta updater source
  is configured

## Lifecycle semantics

`start` is idempotent and returns after app-server is ready to answer the normal
JSON-RPC initialize handshake on the Unix control socket.

`restart` stops any managed daemon and starts it again.

`enable-remote-control` and `disable-remote-control` persist the launch setting
for future starts. If a managed app-server is already running, they restart it
so the new setting takes effect immediately.

Top-level `hepta remote-control` bootstraps with `--remote-control` when the
updater loop is not running. Otherwise it enables remote control and starts the
daemon normally.

`stop` sends a graceful termination request first, then sends a second
termination signal after the grace window if the process is still alive.

All mutating lifecycle commands are serialized per `HEPTA_HOME`, so a concurrent
`start`, `restart`, `enable-remote-control`, `disable-remote-control`, `stop`,
or `bootstrap` does not race another in-flight lifecycle operation.

## State

The daemon stores its local state under `HEPTA_HOME/app-server-daemon/`:

- `settings.json` for persisted launch settings
- `app-server.pid` for the app-server process record
- `app-server-updater.pid` for the pid-backed standalone updater loop
- `daemon.lock` for daemon-wide lifecycle serialization
