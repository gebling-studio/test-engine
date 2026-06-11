# UI benchmark

Measures how many views the engine can render before a frame costs more than 16 ms (the
60 fps budget). Deterministic: no randomness, no wall-clock dependence — every run builds
the identical scene in the identical order.

## Run

```bash
make bench                                       # full suite, results saved to bench/
BENCH_RUNS=3 make bench                          # fewer runs per mode
UI_BENCHMARK=1 cargo run -p test-game --release  # single run, prints and exits
TE_HEADLESS=1                                    # add to run without a window
```

Also launchable from the test-game menu ("ui benchmark") — same run, shows an Alert and
keeps the app alive.

## How it works

Every rendered frame adds one `BenchPanel` (~40 views: labels, button, image, switch,
checkbox, slider, progress, circle, number view, drawing, scroll view) placed with every
layout rule family (sides, anchors, same, relative, between, tiling, distribute, custom).
The metric is `Window::frame_work_time` — CPU time of update + render encoding, excluding
the wait for a drawable — so results are not capped by vsync or the compositor. Frames the
window did not actually render (occluded) are skipped. The run stops when the rolling
average over 10 frames crosses 16 ms and reports panels, views and ms.

## Consistency

A run refuses to start (exit code 75) when the system is busy or hot. Thresholds in
`benchmark_view.rs`: cpu usage 15%, load average 0.6 per core, cpu temperature 55 C.
A running browser is rejected outright regardless of load — browsers do bursty
background work even when idle. The suite fails immediately on rejection and names
the heavy processes — close them and rerun.

The strictness is deliberate. No performance change gets merged on vibes: it must show a
clear before/after delta here, ideally with non-overlapping run ranges. Real optimizations
move the result by 3-10%, and a single background process or a throttling chip shifts it
by more than that — numbers collected on a loaded machine are worthless as evidence and
worse than no numbers. Measure both sides in one session, on an idle machine, or don't
bother. Known saboteurs caught by the guard: Docker's VM, Spotlight indexing fresh build
artifacts, the chip still hot from a cargo build.

## History

`make bench` (the `bench` crate) runs all modes — debug, release, headless release —
`BENCH_RUNS` times each (default 5) and writes `bench/<date>-<commit>.json`: every run's
result with the cpu usage, load and temperature it started under, plus a median per mode.
Runs more than 15% below their mode's median are marked `"suspect": true` and excluded
from the median. The JSON files are committed — that is the engine's performance history.
Always benchmark a clean committed state, the file is named after the commit.
