# Inspect

Remote UI inspector. Debug builds only.

Every app in a debug build starts an inspect server at launch (`test-engine/src/inspect/`):
a TCP listener on an OS-assigned port, advertised over mDNS as `_te-inspect._tcp.local.`
with the app instance id in the TXT record. No config, no fixed ports, any number of apps
per machine.

Two clients exist:

- `inspector` — the GUI. Browses mDNS continuously, lists running apps in a dropdown,
  filters out its own advertisement.
- `te-inspect` — the CLI, also the interface for AI agents. Install once with
  `cargo install --path te-inspect`, reinstall after protocol changes. A serde error like
  `unknown field 'fit_text'` from any command means the installed CLI is older than the
  app's protocol, reinstall and retry. Commands: `apps`,
  `tree`, `view`, `ui`, `screenshot`, `edit-rule`, `set-text`, `set-color`, `set-scale`,
  `edits`, `play-sound`. The last discovery is cached in the temp dir, so repeat calls
  connect instantly and fall back to a fresh mDNS browse when the cached address is dead.
  The agent workflow is documented in
  [.claude/skills/test-engine/SKILL.md](../.claude/skills/test-engine/SKILL.md).

## Protocol

Lives in `test-engine/src/inspect/protocol/`. Length-prefixed JSON frames over TCP
(`transport.rs`), request in, response out:

- `GetUI` — returns scale and the whole view tree as `ViewRepr`: labels, ids, frames,
  colors, texts and placer rules.
- `SetScale(f32)` — applies the scale on the main thread.
- `EditRule { view_id, rule_index, offset, enabled }` — edits a placer rule of the live
  view. Offset applies to Side and Anchor rules and edits the ratio of Relative rules.
- `SetText { view_id, text }` — sets the text of a live `Label`, `Button` or `TextField`.
- `SetColor { view_id, color }` — sets the background color of a live view.
- `Screenshot` — returns the current frame as base64 PNG. Works headless too.
- `ListEdits` — returns every edit applied in this session.
- `PlaySound` — plays a sound in the app, for finding which instance is which.

Edits reply with a fresh tree snapshotted one frame later, after layout ran, so the client
never sees stale frames. Failures (unknown view id, bad rule index, view without text)
reply with `Error(String)` instead of being silently ignored. All UI access happens on the
main thread via `from_main`. Responses hold `Own` pointers, so the transport hands them to
the main thread for dropping (see [refs.md](refs.md)).

## Edit log

Every applied edit (`edit_log.rs`) is kept in memory for `ListEdits` and appended as a JSON
line to `target/inspect-edits.jsonl` under the app's git root: timestamp, view label and
id, what changed, old and new values. The file survives app restarts. Outside a git repo,
on a device for example, only the in-memory list works.

## Release builds

The server parts — `inspect_service`, `edit_log`, `view_conversion` and the `serve`
transport — are `cfg(debug_assertions)` and do not exist in release builds.
`test-engine/build.rs` fails any release-profile build that enables debug-assertions, so
the server can never ship. The protocol and the `inspect::views` widgets stay available
in release, so the host-side tools `inspector` and `te-inspect` build in release like any
other crate. `te-inspect` is excluded from default workspace members.

## Local hook

Unrelated to the remote inspector: pressing `i` in any app calls the per-view
`Setup::inspect()` hook recursively. Default is empty, override it for ad hoc debugging.
