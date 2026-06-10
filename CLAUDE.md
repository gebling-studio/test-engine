# TestEngine

Cross platform game engine and UI framework in Rust. Rendering on WGPU.

## Docs

Do not read these upfront. Read the matching file only when the task touches that area:

- [docs/refs.md](docs/refs.md) — `Own`/`Weak` smart pointers, the memory model. Read before
  working with view lifetimes, pointers, or anything from the `refs` crate.
- [docs/dispatch.md](docs/dispatch.md) — main thread rules, `on_main`/`from_main`, frame loop.
  Read before touching threading, async, or dispatch code.
- [docs/ui-tests.md](docs/ui-tests.md) — how UI tests work and how to run a single one.
  Read before writing or debugging UI tests.

## Commands

```bash
cargo run -p ui-test -- --stop-on-failure                         # full UI test suite
cargo run -p ui-test -- --stop-on-failure --test-name <ViewName>  # single test
cargo run -p render-test                                          # render tests
make lint                                                         # clippy, pedantic, zero warnings
```

Without `--stop-on-failure` a failed UI test leaves the app window running. Always pass it.
