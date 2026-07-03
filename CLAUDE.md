# TestEngine

Cross platform game engine and UI framework in Rust. Rendering on WGPU.
Supports: Windows Linux Mac iOS and Android.

No proof, no merge. A performance claim needs an A/B per [docs/benchmark.md](docs/benchmark.md)
acceptance criteria, a correctness claim needs a reproduced failure. Unproved ideas go to
[docs/guesses.md](docs/guesses.md), not into the code.

Every new UI feature or bugfix must land together with a new UI test that covers it. No exceptions.
See [docs/ui-tests.md](docs/ui-tests.md) for how UI tests work.

## Docs

Do not read these upfront. Read the matching file only when the task touches that area:

- [docs/refs.md](docs/refs.md) — `Own`/`Weak` smart pointers, the memory model. Read before
  working with view lifetimes, pointers, or anything from the `refs` crate.
- [docs/dispatch.md](docs/dispatch.md) — main thread rules, `on_main`/`from_main`, frame loop.
  Read before touching threading, async, or dispatch code.
- [docs/ui-tests.md](docs/ui-tests.md) — how UI tests work and how to run a single one.
  Read before writing or debugging UI tests.
- [docs/inspect.md](docs/inspect.md) — the remote UI inspector, its protocol and debug-only
  gating. Read before touching `deps/inspect`, `test-engine/src/inspect`, the `inspector`
  app, or the `te-inspect` CLI.
- [docs/benchmark.md](docs/benchmark.md) — the UI benchmark, its consistency guard, and the
  results history in `bench/`. Read before touching the benchmark or measuring performance.
- [docs/guesses.md](docs/guesses.md) — parked changes that lacked proof. Read before
  proposing an optimization or a speculative fix; add new unproved ideas there, not to code.
- [docs/text.md](docs/text.md) — the text pipeline: rustybuzz shaping, em sizing, variable
  font instances, letter spacing, line handling. Read before touching label rendering,
  fonts, or `deps/window/src/text`.
- [docs/roadmap.md](docs/roadmap.md) — missing engine features found by porting a real app,
  with current state, design notes, and order. Read before planning or starting a new
  engine capability, and update it when one lands.

Docs should be concise.

## Commands

```bash
cargo run -p ui-test -- --stop-on-failure --headless                         # full UI test suite
cargo run -p ui-test -- --stop-on-failure --headless --test-name <ViewName>  # single test
cargo run -p ui-test -- --test-name <ViewName> --human                       # watchable run, space to advance
cargo run -p ui-test -- --stop-on-failure --headless --test-name <ViewName> --record-colors  # print check_colors blocks
cargo run -p render-test                                                     # render tests
make lint                                                                    # clippy, pedantic, zero warnings
cargo machete                                                                # unused dependencies, zero findings
make bench                                                                   # UI benchmark suite, saves bench/<date>-<commit>.json
UI_BENCHMARK=1 cargo run -p test-game --release --features bench             # single benchmark run, prints and exits
```

`TE_HEADLESS=1` runs any app without a window.

Without `--stop-on-failure` a failed UI test leaves the app window running. Always pass it.
`--headless` runs without a window or a display — tests run many times faster. Always pass it too.

After touching any `Cargo.toml` or removing code, run `cargo machete`. It must report
zero unused dependencies.
