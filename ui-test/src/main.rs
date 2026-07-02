#![allow(incomplete_features)]
#![allow(clippy::float_cmp)]
#![allow(clippy::too_many_lines)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use std::{
    collections::BTreeMap,
    env::var,
    panic::{catch_unwind, set_hook, take_hook},
    process::exit,
};

use anyhow::{Result, anyhow, bail};
use clap::Parser;
use log::info;
use test_engine::{
    AppRunner, Window,
    dispatch::from_main,
    ui::{Label, UIManager},
    ui_test::{UITest, enable_color_recording, enable_fps_report, enable_human_mode},
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

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    test_name: Option<String>,

    #[arg(long)]
    stop_on_failure: bool,

    #[arg(long)]
    fps_report: bool,

    #[arg(long)]
    headless: bool,

    /// Watchable run: slows injections, shows touches, holds after
    /// each test until space is pressed.
    #[arg(long)]
    human: bool,

    /// Print ready to paste check_colors blocks instead of asserting them.
    #[arg(long)]
    record_colors: bool,
}

fn run(args: Args) -> Result<()> {
    if args.fps_report {
        enable_fps_report();
    }

    if args.human {
        if args.headless {
            bail!("--human requires a window, remove --headless");
        }
        enable_human_mode();
    }

    if args.record_colors {
        enable_color_recording();
    }

    if args.stop_on_failure {
        let default_hook = take_hook();
        set_hook(Box::new(move |info| {
            default_hook(info);
            let report = catch_unwind(test_engine::ui_test::failure_report)
                .unwrap_or_else(|_| Err(anyhow!("report collection panicked")));

            match report {
                Ok(report) => eprintln!("{report}"),
                Err(err) => eprintln!("Failed to collect failure report: {err}"),
            }
            exit(1);
        }));
    }

    let test_name = args.test_name;
    let human = args.human;

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

        let my_tests: BTreeMap<_, _> = crate::UI_TESTS.lock().clone();

        let mut te_tests: BTreeMap<_, _> = test_game::UI_TESTS.lock().clone();
        te_tests.append(&mut test_engine::UI_TESTS.lock().clone());

        if let Some(test_name) = test_name {
            if let Some(test) = my_tests.get(&test_name) {
                test()?;
                UITest::finish();
                AppRunner::stop();
                return Ok(());
            }

            let test = match te_tests.get(&test_name) {
                Some(test) => test,
                None => {
                    println!("Test: {test_name} not found");
                    AppRunner::stop();
                    bail!("Test: {test_name} not found");
                }
            };
            test()?;
            UITest::finish();
            AppRunner::stop();
            return Ok(());
        }

        let cycles: u32 = var("UI_TEST_CYCLES").unwrap_or("2".to_string()).parse().unwrap();

        for i in 1..=cycles {
            test().await?;
            info!("Cycle {i}: OK");

            for (_name, test) in te_tests.iter() {
                test()?;
            }
        }

        UITest::finish();
        AppRunner::stop();

        Ok(())
    };

    if args.headless {
        AppRunner::start_headless_with_actor(actor)?;
    } else {
        AppRunner::start_with_actor(actor)?;
    }

    Ok(())
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
