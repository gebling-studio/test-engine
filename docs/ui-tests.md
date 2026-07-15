# UI tests

Real-window tests. They open the app, inject touches and scrolls, then check labels, colors and
state. Live in `ui-test/`, plus some in `test-engine` and `test-game`.

## Run

```bash
cargo run -p ui-test                          # full suite, all tests, 2 cycles
UI_TEST_CYCLES=5 cargo run -p ui-test         # more cycles
cargo run -p ui-test -- --test-name RestRequest   # one test
cargo run -p ui-test -- --headless            # offscreen, much faster, for CI and agents
make uui                                      # full suite, headless, release mode
cargo run -p ui-test -- --test-name FontZoo --human            # watch one test, space to advance
cargo run -p ui-test -- --record-colors --headless --test-name FontZoo  # print check_colors blocks
```

`--test-name` reaches every test. A registered one answers to its view name, `FontZoo` or
`Text field`. An aggregated one answers to its fn name, `test_text_field`. A name that
matches nothing exits 1, so a typo never looks like a pass.

An app can also run its own suite from inside itself, which is how tests run on a device.
`test-game` registers a runner with `ui_test::register_test_runner`, and `te-inspect
run-tests` triggers it over the network and prints every failure. See
[inspect.md](inspect.md).

A failing test does not stop the run. Every test executes, each failure is collected, and
the whole report prints at the end, then the process exits 1 if anything failed. One run
therefore shows every broken test rather than only the first.

Always pass `--headless` when running from a script or agent, and always tee the output to
a temp file — with a plain pipe (`| tail`) you lose everything printed before a hang:

```bash
cargo run -p ui-test -- --headless 2>&1 | tee /tmp/ui-test.log | tail -12
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

Never change existing UI tests while implementing a new feature unless the user
explicitly allows it. Design the feature so old tests stay green: make new behavior
opt-in instead of changing defaults. If a new mechanism genuinely invalidates an old
assertion, stop and ask before touching it.

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

## The canvas

Probes index screen pixels, so a test needs a fixed rectangle to draw in. Tests never
resize the window. Instead the harness pins the root view to a canvas at the frame origin,
600 by 600 by default, and the rest of the screen shows the clear color. A phone screen
cannot be resized, so this is what lets the same test and the same probes run on desktop
and on a device.

The canvas is counted in screen pixels, not points, and the harness divides the scale back
out. A scale change resizes the root, so the canvas keeps the same pixels either way.

Because the root itself is the canvas, anything laid out against the root lands inside it,
including modals, alerts and drop downs. Touch dispatch starts at the root, so injected
touches outside the canvas go nowhere.

Declare a different canvas when the default is too small. Keep it within the smallest
supported screen, 640 by 1136 on an iPhone 5S, or the test cannot run on device.

```rust
impl ViewTest for LongTableTest {
    fn canvas() -> (u32, u32) { (640, 1000) }
    ...
}

let view = UITest::start_sized::<TextField>(640, 800);   // aggregated test
```

`AppRunner::set_window_size` stays an app API. No test may call it, a window smaller than
the canvas clips it and every later test probes the clipped frame.

A game or a level fills the root rather than the window, see `UIManager::render_area`.
Anything else that renders from the whole frame, such as a blur, samples the clear color
around the canvas, so probes within a blur radius of a canvas edge pick that up. That is
consistent on every screen, since the canvas is always smaller than the frame.

Global state is reset per test: the root background, the clear color and the string state.
A test that fails part way never reaches its own cleanup, and without the reset every
later test would probe the leftovers.

## Two kinds of tests

1. **Registered** — a view marked `#[view_test]` instead of `#[view]`, with
   `impl ViewTest { fn perform_test(view: Weak<Self>) }`. The macro registers it under the struct
   name, so it works with `--test-name` and also generates a normal `#[test]` that runs it as a
   subprocess.
2. **Aggregated** — plain async functions called from hardcoded sequences (`test_base_ui()` in
   `ui-test/src/base/mod.rs` and similar) through `run_test_unit!`. Runnable by fn name, the
   aggregator runs with every other unit filtered out. Prefer `#[view_test]` for new tests.

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

A new UI test is not finished when it passes. Always show it to the user for approval:
run it with `--human`, let them watch it, and wait for their verdict before treating
the work as done.

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

The picker is deterministic, the same screen always produces the same block. It is bounded
to the canvas, the frame around it is not part of the test and does not exist on a device.
It samples a 4px grid, keeps only pixels whose 3x3 neighborhood is near uniform — skipping
antialiased edges, which differ between renderers — clusters candidates by color so text
ink is probed alongside backgrounds, gives small enclosed features like letter holes their
own probes first, and spreads the rest spatially.

Default is 32 probes per check. A test declares its own density with
`set_record_probe_count(n)`, called after `UITest::start`, since test start resets it. It
is inert outside record runs. Keeping it in the test source means it survives the next
re-record.

`--record-colors --human` combined shows the freshly picked probes the same way normal
human runs show existing ones, to review what gets pinned before pasting.

Re-recording an existing block to make a failing test pass is editing expectations — same
rule as above, only with explicit approval. Approval to record is not approval of the
result: paste the block, show the render with `--test-name <name> --human --record-colors`,
and wait. A passing rerun proves nothing, the expectation came from that same render.

A recorded block is large. Keep it in a `const` next to the test rather than inline, so
the function stays readable and within the line limit.
