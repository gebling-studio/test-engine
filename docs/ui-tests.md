# UI tests

Real-window tests. They open the app, inject touches and scrolls, then check labels, colors and
state. Live in `ui-test/`, plus some in `test-engine` and `test-game`.

## Run

```bash
cargo run -p ui-test                          # full suite, all tests, 2 cycles
UI_TEST_CYCLES=5 cargo run -p ui-test         # more cycles
cargo run -p ui-test -- --test-name RestRequest   # one test by view struct name
cargo run -p ui-test -- --headless            # offscreen, much faster, for CI and agents
make uui                                      # full suite, headless, release mode
cargo run -p ui-test -- --test-name FontZoo --human            # watch one test, space to advance
cargo run -p ui-test -- --record-colors --headless --test-name FontZoo  # print check_colors blocks
```

By default a failed test leaves the app running so the window can be inspected.
`--stop-on-failure` makes the process print the failure and exit with code 1 instead.
Always pass it when running from a script or agent, together with `--headless`, and always
tee the output to a temp file — with a plain pipe (`| tail`) you lose everything printed
before a hang:

```bash
cargo run -p ui-test -- --stop-on-failure --headless 2>&1 | tee /tmp/ui-test.log | tail -12
```

Don't run the suite after every change. Run it only when the change can affect UI or
rendering behavior, or once before a commit/push. Mechanical changes (renames, comments,
docs) only need `cargo build` and `make lint`.

On failure a report is printed: window resolution and scale, a path to a screenshot of
the actual screen, and the view tree with frames. For `check_colors` failures the failing
pixel also gets a highlight marker, visible in the screenshot. Read the screenshot and
the view tree first — they usually show the problem immediately.

Never edit test expectations (`check_colors` data, asserted values) to make a failing
test pass. The expectations are the spec: the UI must behave exactly like before. If a
test fails after a code change, the code is wrong. Expectations change only when the new
look or behavior is intended and explicitly approved.

Temporary edits that are never committed are allowed — for example breaking one
expectation on purpose to verify the failure machinery. Say what you are doing first,
revert right after the run, and check that `git diff` is clean before committing.

In environments without a display (CI, linux without display) the `#[view_test]`-generated
`cargo test` tests run headless instead of opening a window.

Every test prints `Name: Started` and `Name: OK`. On a hang or failure the broken test is the
one with `Started` and no `OK` — usually the last line of the log.

The test app disables vsync and raises max frame latency at startup (`Window::set_vsync(false)`,
`Window::set_max_frame_latency(3)`) so tests are not capped to the display refresh rate.

`--headless` goes further: the app starts with no window at all — no winit, no surface,
no display. Frames render to an offscreen texture in a plain loop, so the full suite runs
in a few seconds and works on machines without a display server (CI), given a GPU or a
software Vulkan driver. Screenshots and `check_colors` still work. Run headed when you
want to watch the UI. The network test (`RestRequest`) is skipped in headless mode.

For profiling, pass `--fps-report` to print a report at the end of the run: frames, duration
and average fps per test. Per-test fps varies a lot between runs — macOS sometimes paces frames
at display rate anyway — so don't compare single runs.

## Two kinds of tests

1. **Registered** — a view marked `#[view_test]` instead of `#[view]`, with
   `impl ViewTest { fn perform_test(view: Weak<Self>) }`. The macro registers it under the struct
   name, so it works with `--test-name` and also generates a normal `#[test]` that runs it as a
   subprocess.
2. **Manual** — plain async functions called from hardcoded sequences (`test_base_ui()` in
   `ui-test/src/base/mod.rs` and similar). Not individually runnable. Prefer `#[view_test]`
   for new tests.

## Writing a test

```rust
#[view_test]
struct MyTest {
    #[init]
    button: Button,
}

impl Setup for MyTest {
    fn setup(self: Weak<Self>) { /* build UI */ }
}

impl ViewTest for MyTest {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        inject_touches("100 100 b\n100 100 e"); // x y begin/end
        assert_eq!(view.button.text(), "tapped");
        Ok(())
    }
}
```

Test helpers: `inject_touches`, `inject_scroll`, `check_colors` (asserts pixel colors at
coordinates). To read UI state from test code use `from_main` (see [dispatch.md](dispatch.md)).

One `#[view_test]` per file. Deliberate decision to keep files small — do not "fix" the macro
to allow more.

## Human mode

`--human` makes a run watchable: vsync stays on, injected touches are drawn on screen, every
injected event pauses (`UI_TEST_HUMAN_DELAY` ms, default 400, moved touches an eighth of it),
and every screenshot pauses first so the verified state is visible. Every `check_colors`
marks its checked pixels with squares on screen, the window title names the check, and the
run holds until space before asserting. After each test the title shows the result and the
run holds again. Works for one test or the whole suite. Rejected together with `--headless`.

## Recording color probes

`check_colors` expectations are recorded, not written by hand. With `--record-colors` every
`check_colors` call prints a ready to paste block instead of asserting: it takes a
screenshot, picks probe pixels automatically, and prints them labeled with the test name and
check index. Write the test with empty `check_colors("")?` placeholders, run once with the
flag, paste each block over its placeholder, rerun normally to verify.

The picker is deterministic, the same screen always produces the same block. It samples a
4px grid, keeps only pixels whose 3x3 neighborhood is near uniform — skipping antialiased
edges, which differ between renderers — clusters candidates by color so text ink is probed
alongside backgrounds, gives small enclosed features like letter holes their own probes
first, and spreads the rest spatially.

Default is 32 probes per check. A test declares its own density by calling
`set_record_probe_count(n)` at the start of `perform_test`. It is inert outside record runs
and resets when the next test starts.

`--record-colors --human` combined shows the freshly picked probes the same way normal
human runs show existing ones, to review what gets pinned before pasting.

Re-recording an existing block to make a failing test pass is editing expectations — same
rule as above, only with explicit approval.
