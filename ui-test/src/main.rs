use std::{collections::BTreeMap, env::var, hint::black_box, panic::set_hook, process::exit};

use anyhow::Result;
use clap::Parser;
use log::info;
use test_engine::{
    AppRunner, Window,
    dispatch::{from_main, is_main_thread},
    ui::{Label, UIManager},
    ui_test::{
        TestFailure, UITest, UITestEntry, clear_failures, current_test_name, enable_color_recording,
        enable_fps_report, enable_human_mode, push_failure, run_test, spaced_test_name, take_failures,
    },
};

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    test_name: Option<String>,

    /// Print every registered test and the total, then exit without running.
    #[arg(long)]
    list: bool,

    #[command(flatten)]
    run: RunArgs,

    #[command(flatten)]
    display: DisplayArgs,
}

/// How the run reacts to failures and what it reports.
#[derive(clap::Args)]
struct RunArgs {
    #[arg(long)]
    fps_report: bool,

    /// Print ready to paste `check_colors` blocks instead of asserting them.
    #[arg(long)]
    record_colors: bool,
}

/// Where and how the frames are shown.
#[derive(clap::Args)]
struct DisplayArgs {
    #[arg(long)]
    headless: bool,

    /// Watchable run: slows injections, shows touches, holds after
    /// each test until space is pressed.
    #[arg(long)]
    human: bool,
}

/// Names the crates whose tests this runner covers, so a linker keeps them.
///
/// Every test registers through a `ctor` and nothing calls it by name, so a
/// linker drops a whole rlib and takes its tests with it. Nothing reports that,
/// the suite just quietly runs fewer tests. This is the same trap that hid
/// every test on iOS, see `keep_ctor_linked` in
/// `test-engine/src/app_starter.rs`.
fn keep_tests_linked() {
    ui_test_suite::keep_linked();
    black_box(test_game::TestGameApp);
}

/// Every registered test, from the corpus, the app and the engine. They all
/// register into the one engine owned map, so there is nothing to merge.
fn all_tests() -> BTreeMap<String, UITestEntry> {
    keep_tests_linked();
    test_engine::UI_TESTS.lock().clone()
}

fn run(args: Args) -> Result<()> {
    if args.run.fps_report {
        enable_fps_report();
    }

    if args.display.human {
        if args.display.headless {
            anyhow::bail!("--human requires a window, remove --headless");
        }
        enable_human_mode();
    }

    if args.run.record_colors {
        enable_color_recording();
    }

    install_fatal_panic_hook();

    let tests = all_tests();

    // A suite that runs nothing otherwise reports success, which looks exactly
    // like a suite that passes. Registration is a ctor nothing calls by name, so
    // an empty map means the `ui-tests` feature is off or a linker dropped a
    // whole crate, never that there are no tests.
    anyhow::ensure!(
        !tests.is_empty(),
        "No UI tests registered. Either the `test-engine/ui-tests` feature is off, or a linker \
         dropped a test crate whose ctors nothing references, see `keep_tests_linked`.",
    );

    if args.list {
        for name in tests.keys() {
            println!("{name}");
        }
        println!("\n{} UI tests", tests.len());
        return Ok(());
    }

    let test_name = args.test_name;
    let human = args.display.human;
    let total = if test_name.is_some() { 1 } else { tests.len() };

    let actor = async move {
        Label::set_default_text_size(32);
        UIManager::set_display_touches(human);

        from_main(move || {
            UIManager::override_scale(1.0);

            if !human {
                Window::set_vsync(false);
                Window::set_max_frame_latency(3);
            }
        });

        clear_failures();

        if let Some(test_name) = test_name {
            // Also accept the struct ident, so a tool reading `impl ViewTest for
            // ScrollViewTest` off the source can pass what it sees without
            // deriving the spaced name itself. `spaced_test_name` is the one
            // place that rule lives, and drifting from it is what made the old
            // generated `#[test]` pass a name the runner rejected.
            let key = spaced_test_name(&test_name);

            let Some(test) = tests.get(&key) else {
                eprintln!("UI test not found: {test_name}");
                eprintln!("Run `cargo run -p ui-test -- --list` to see every registered test.");
                exit(1);
            };

            run_test(&key, test.run);

            UITest::finish();
            AppRunner::stop();
            return Ok(());
        }

        let cycles: u32 = var("UI_TEST_CYCLES").unwrap_or("2".to_string()).parse().unwrap();

        for i in 1..=cycles {
            for (name, test) in &tests {
                run_test(name, test.run);
            }
            info!("Cycle {i}: OK");
        }

        UITest::finish();
        AppRunner::stop();

        Ok(())
    };

    if args.display.headless {
        AppRunner::start_headless_with_actor(actor)?;
    } else {
        AppRunner::start_with_actor(actor)?;
    }

    let failures = take_failures();

    if failures.is_empty() {
        println!("{total} UI tests passed");
        return Ok(());
    }

    report_failures(total, &failures);
    exit(1);
}

/// A panic inside a `from_main` closure runs on the main thread and kills the
/// frame loop, so `CatchUnwind` on the actor never sees it and any pending
/// `from_main` hangs. Detect that case, report everything gathered so far plus
/// the fatal test, and exit. Actor thread panics are left to `CatchUnwind`.
fn install_fatal_panic_hook() {
    set_hook(Box::new(move |info| {
        if !is_main_thread() {
            return;
        }

        let name = current_test_name();
        let name = if name.is_empty() {
            "unknown".to_string()
        } else {
            name
        };
        push_failure(&name, format!("main thread panic: {info}"));
        report_failures(all_tests().len(), &take_failures());
        exit(1);
    }));
}

/// Print every failed test once, most useful line first, then the full detail.
fn report_failures(total: usize, failures: &[TestFailure]) {
    let mut seen = std::collections::BTreeSet::new();
    let unique: Vec<&TestFailure> = failures.iter().filter(|f| seen.insert(f.name.clone())).collect();

    eprintln!("\n{} of {total} UI test(s) failed:", unique.len());
    for f in &unique {
        eprintln!("  - {}", f.name);
    }

    for f in &unique {
        eprintln!("\n===== {} =====\n{}", f.name, f.detail);
    }
}

fn main() -> Result<()> {
    run(Args::parse())
}
