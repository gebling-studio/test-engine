# UI tests

Real-window tests. They open the app, inject touches and scrolls, then check labels, colors and
state. Live in `ui-test/`, plus some in `test-engine` and `test-game`.

## Run

```bash
cargo run -p ui-test                          # full suite, all tests, 2 cycles
UI_TEST_CYCLES=5 cargo run -p ui-test         # more cycles
cargo run -p ui-test -- --test-name RestRequest   # one test by view struct name
```

Always tee the output to a temp file. The suite can get stuck, and with a plain
pipe (`| tail`) you lose everything printed before the hang:

```bash
cargo run -p ui-test 2>&1 | tee /tmp/ui-test.log | tail -12
```

The suite stops on the first failure. Headless environments (CI, linux without display) skip
UI tests automatically.

Every test prints `Name: Started` and `Name: OK`. On a hang or failure the broken test is the
one with `Started` and no `OK` — usually the last line of the log.

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
