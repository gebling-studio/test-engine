# TestEngine

Cross platform game engine and UI framework in Rust. Rendering on WGPU.
Supports: Windows Linux Mac iOS and Android.

## Docs

Do not read these upfront. Read the matching file only when the task touches that area:

- [docs/refs.md](docs/refs.md) — `Own`/`Weak` smart pointers, the memory model. Read before
  working with view lifetimes, pointers, or anything from the `refs` crate.
- [docs/dispatch.md](docs/dispatch.md) — main thread rules, `on_main`/`from_main`, frame loop.
  Read before touching threading, async, or dispatch code.
- [docs/ui-tests.md](docs/ui-tests.md) — how UI tests work and how to run a single one.
  Read before writing or debugging UI tests.

Docs should be concise.

## Commands

```bash
cargo run -p ui-test -- --stop-on-failure --headless                         # full UI test suite
cargo run -p ui-test -- --stop-on-failure --headless --test-name <ViewName>  # single test
cargo run -p render-test                                                     # render tests
make lint                                                                    # clippy, pedantic, zero warnings
make bench                                                                   # UI benchmark suite, saves bench/<date>-<commit>.json
UI_BENCHMARK=1 cargo run -p test-game --release                              # single benchmark run, prints and exits
```

`TE_HEADLESS=1` runs any app without a window. The benchmark adds views until frame work
time (`Window::frame_work_time`, not capped by vsync) hits 16 ms, then reports views count.
Results in `bench/` are committed for performance history. `BENCH_RUNS=N` overrides 10.

Without `--stop-on-failure` a failed UI test leaves the app window running. Always pass it.
`--headless` runs without a window or a display — tests run many times faster. Always pass it too.
