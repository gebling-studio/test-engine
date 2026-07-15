#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use std::{collections::BTreeMap, env::var, panic::set_hook, process::exit};

use anyhow::Result;
use clap::Parser;
use log::info;
use test_engine::{
    AppRunner, Window,
    dispatch::{from_main, is_main_thread},
    ui::{Label, UIManager},
    ui_test::{
        TestFailure, UITest, clear_failures, current_test_name, enable_color_recording, enable_fps_report,
        enable_human_mode, push_failure, run_test_sync, take_failures,
    },
};

#[cfg(debug_assertions)]
use crate::inspect::test_inspect;
use crate::{
    base::test_base_ui,
    views::{
        basic::test_base_views,
        complex::test_complex_views,
        containers::test_containers,
        helpers::test_helper_views,
        images::test_image_views,
        // input::test_input_views,
        layout::test_layout,
    },
};

mod base;
#[cfg(debug_assertions)]
mod inspect;
mod level;
mod views;

test_engine::export_ui_tests!();

/// Run one async test unit inside an aggregator. Records a failure and keeps
/// going instead of short circuiting, so the whole suite runs in one pass.
#[macro_export]
macro_rules! run_test_unit {
    ($f:ident) => {
        test_engine::ui_test::run_test(stringify!($f), $f()).await
    };
}

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    test_name: Option<String>,

    #[command(flatten)]
    run: RunArgs,

    #[command(flatten)]
    display: DisplayArgs,
}

/// How the run reacts to failures and what it reports.
#[derive(clap::Args)]
struct RunArgs {
    /// Kept for compatibility. The suite always runs every test and reports
    /// every failure at the end, then exits 1 if any failed.
    #[arg(long)]
    stop_on_failure: bool,

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

    let test_name = args.test_name;
    let human = args.display.human;

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

        let my_tests: BTreeMap<_, _> = crate::UI_TESTS.lock().clone();

        let mut te_tests: BTreeMap<_, _> = test_game::UI_TESTS.lock().clone();
        te_tests.append(&mut test_engine::UI_TESTS.lock().clone());

        if let Some(test_name) = test_name {
            if let Some(test) = my_tests.get(&test_name).or_else(|| te_tests.get(&test_name)) {
                run_test_sync(&test_name, test);
                UITest::finish();
                AppRunner::stop();
                return Ok(());
            }

            println!("Test: {test_name} not found");
            exit(1);
        }

        let cycles: u32 = var("UI_TEST_CYCLES").unwrap_or("2".to_string()).parse().unwrap();

        for i in 1..=cycles {
            test().await?;
            info!("Cycle {i}: OK");

            for (name, test) in &te_tests {
                run_test_sync(name, test);
            }
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
    report_failures(&failures);

    if !failures.is_empty() {
        exit(1);
    }

    Ok(())
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
        report_failures(&take_failures());
        exit(1);
    }));
}

/// Print every failed test once, most useful line first, then the full detail.
fn report_failures(failures: &[TestFailure]) {
    if failures.is_empty() {
        info!("All UI tests passed");
        return;
    }

    let mut seen = std::collections::BTreeSet::new();
    let unique: Vec<&TestFailure> = failures.iter().filter(|f| seen.insert(f.name.clone())).collect();

    eprintln!("\n{} UI test(s) failed:", unique.len());
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

async fn test() -> Result<()> {
    test_base_ui().await?;
    test_base_views().await?;
    #[cfg(debug_assertions)]
    test_inspect().await?;
    test_layout().await?;
    test_complex_views().await?;
    test_image_views().await?;
    test_containers().await?;
    // test_input_views().await?;
    test_helper_views().await?;

    Ok(())
}
