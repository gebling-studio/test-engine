# Inspect

Remote UI inspector. Debug builds only.

Every app in a debug build starts an inspect server at launch (`test-engine/src/inspect/`):
a TCP listener on an OS-assigned port, advertised over mDNS as `_te-inspect._tcp.local.`
with the app instance id in the TXT record. No config, no fixed ports, any number of apps
per machine.

The `inspector` crate is the client GUI. It browses mDNS continuously, lists running apps
by app id in a dropdown, filters out its own advertisement, and talks to the selected app
over TCP.

## Protocol

Lives in `deps/inspect`. Length-prefixed JSON frames over TCP (`transport.rs`), request in,
response out:

- `GetUI` — returns scale and the whole view tree as `ViewRepr`.
- `SetScale(f32)` — applies the scale on the main thread.
- `EditRule { view_id, rule_index, offset, enabled }` — finds the live view by id and edits
  its placer rule. This is what the layout rule rows in the inspector send.
- `PlaySound` — plays a sound in the app, for finding which instance is which.

`SetScale` and `EditRule` reply with a fresh tree snapshotted one frame later, after layout
ran, so the client never sees stale frames. All UI access happens on the main thread via
`from_main`. Responses hold `Own` pointers, so the transport hands them to the main thread
for dropping (see [refs.md](refs.md)).

## Release builds

The whole module is `cfg(debug_assertions)` and does not exist in release builds.
`test-engine/build.rs` fails any release-profile build that enables debug-assertions, so
the server can never ship. Consequence: the `inspector` app itself builds only in debug.

## Local hook

Unrelated to the remote inspector: pressing `i` in any app calls the per-view
`Setup::inspect()` hook recursively. Default is empty, override it for ad hoc debugging.
