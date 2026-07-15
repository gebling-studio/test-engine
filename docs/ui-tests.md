# UI tests

Real-window tests. They open the app, inject touches and scrolls, then check labels, colors and
state. The corpus lives in `ui-test-suite/`, plus some in `test-engine` and `test-game`.
`ui-test/` is only the runner.

## Run

```bash
cargo run -p ui-test -- --list                # every registered test and the total, runs nothing
cargo run -p ui-test                          # full suite, all tests, 2 cycles
UI_TEST_CYCLES=5 cargo run -p ui-test         # more cycles
cargo run -p ui-test -- --test-name "Rest request"  # one test
cargo run -p ui-test -- --headless            # offscreen, much faster, for CI and agents
make uui                                      # full suite, headless, release mode
cargo run -p ui-test -- --test-name "Font zoo" --human            # watch one test, space to advance
cargo run -p ui-test -- --record-colors --headless --test-name "Font zoo"  # print check_colors blocks
```

**A test answers to the name it prints.** `#[view_test]` registers under
`ui_test::get_test_name`, the same call `UITest::start` reports under, so a test that fails as
`Font zoo` is reached by `--test-name "Font zoo"`, not `FontZoo`. A `#[ui_test]` fn answers to
its fn name, `test_modal_blur`. The two used to be derived separately and drifted, so the name
in the log was a name the runner rejected. Deriving one from the other is what keeps them
equal. A name that matches nothing exits 1, so a typo never looks like a pass.

Counting is not done by reading the log. Every test is registered by a `ctor` before `main`,
so `--list` knows the whole suite without running anything.

An app runs the same suite from inside itself, which is how tests run on a device.
`ui_test::run_all_tests` reaches every registered test with no help from the app, and
`te-inspect run-tests` triggers it over the network. `test-game` also has a "Run UI tests"
button in its dev menu. See [inspect.md](inspect.md).

## One registry

Every test, whatever crate it lives in and whichever shape it is written in, registers into a
single map, `test_engine::UI_TESTS`. The count is its length.

That map is a static of the engine, so a `#[view_test]` in `test-engine`, a `#[ui_test]` in
`ui-test-suite` and one in `test-game` all land in the same place. Nothing merges maps, nothing
registers a runner, and the engine can run the whole suite on its own.

Registration is by name, and a duplicate name aborts at startup rather than silently replacing
the other test. A test that quietly stops running looks exactly like a test that passes, which
is the failure this registry exists to prevent.

**A test registers through a `ctor`, and nothing calls it by name.** A linker drops any object
nothing references, so a crate whose only content is tests is dropped whole and its tests
disappear without a word. Every consumer of a test-carrying crate has to name it:
`ui_test_suite::keep_linked()` in `test-game` and in the runner. This is not theoretical, it is
how the device ran 24 tests while the desktop ran 100.

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
want to watch the UI. The network test (`test_rest_request`) returns early in headless mode.
It checks `Window::headless()` itself, since a registry has no place to hang that condition.

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

let view = UITest::start_sized::<TextField>(640, 800);   // a `#[ui_test]` fn
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

## Two shapes, one kind

A test is written either as a view or as a function. Both are UI tests, both are synchronous,
both register into the same map, and both are counted and run the same way.

1. **A view** — marked `#[view_test]` instead of `#[view]`, with
   `impl ViewTest { fn perform_test(view: Weak<Self>) }`. The macro also generates a normal
   `#[test]` that runs it as a subprocess.
2. **A function** — a plain `fn` marked `#[ui_test]`. Return `Result<()>` if it can fail, or
   nothing if it only asserts. The macro wraps the second shape, so a test that cannot fail does
   not carry a `Result` it never uses.

Prefer `#[view_test]` for new tests.

Nothing here is async. A UI test drives the main thread through `from_main` and never awaits,
so `#[ui_test]` rejects an `async fn`. The corpus was async for years and not one test awaited
anything, which cost a second registry, a boxed future type and a hand written call list.

## Platform gating

A test for a feature the platform does not have is gated where the feature is gated, not
skipped at runtime. `Hover::update` is `#[cfg(desktop)]`, so `hover.rs` is too. Typing goes
through the screen keyboard on a phone rather than injected key events, so the text field tests
are desktop only as well.

Desktop runs 100 tests and an iPhone runs 97. The difference is the platform, not the suite.
Gate the module in its `mod.rs`, with a comment saying which feature is missing:

```rust
/// Hover needs a pointer, and there is no such thing on a touch screen.
#[cfg(desktop)]
mod hover;
```

A crate that gates on `#[cfg(desktop)]` needs `plat::platforms()` in its `build.rs`, which is
what defines those cfgs.

**A test that asserts inside `from_main` and fails takes the whole run down**, because the
panic lands on the main thread, unwinds through the dispatch and aborts. On a device that means
the run dies with no report at all. The desktop runner installs a hook for this, the in-app
runner does not yet.

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

Or as a function, when there is no view to hang it on:

```rust
#[ui_test]
fn test_something() {
    let view = UITest::start::<SomeView>();
    inject_touches("100 100 b\n100 100 e");
    assert_eq!(view.button.text(), "tapped");
}
```

Test helpers: `inject_touches`, `inject_scroll`, `check_colors` (asserts pixel colors at
coordinates). To read UI state from test code use `from_main` (see [dispatch.md](dispatch.md)).

## What a run takes from the app

A run is not read only. It pins scale 1, forces 32 point text, paints its own clear color, pins
the root to the test canvas and tears the app's root view down. `run_test_map` snapshots all of
it and hands it back at the end, then asks the app for a new root view, so an app that runs its
own suite lands back on its main screen at its real scale.

Leave any of it behind and the app carries on wrong. On a phone that means half sized UI, since
the harness scale of 1 is not the screen's 2.

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
