#![allow(incomplete_features)]
#![allow(clippy::float_cmp)]
#![allow(clippy::too_many_lines)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use std::{collections::BTreeMap, env::var};

use anyhow::{Result, bail};
use clap::Parser;
use log::info;
use test_engine::{
    AppRunner,
    dispatch::from_main,
    ui::{Label, UIManager},
};

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
mod inspect;
mod level;
mod views;

test_engine::export_ui_tests!();

#[derive(Parser)]
struct Args {
    #[arg(long, short)]
    test_name: Option<String>,
}

fn run(test_name: Option<String>) -> Result<()> {
    AppRunner::start_with_actor(async {
        Label::set_default_text_size(32);
        UIManager::set_display_touches(false);

        from_main(|| {
            UIManager::override_scale(1.0);
        });

        let mut my_tests: BTreeMap<_, _> = crate::UI_TESTS.lock().clone();
        my_tests.append(&mut crate::UI_TESTS.lock().clone());

        let mut te_tests: BTreeMap<_, _> = test_game::UI_TESTS.lock().clone();
        te_tests.append(&mut test_engine::UI_TESTS.lock().clone());

        if let Some(test_name) = test_name {
            if let Some(test) = my_tests.get(&test_name) {
                test()?;
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

        AppRunner::stop();

        Ok(())
    })?;

    Ok(())
}

fn main() -> Result<()> {
    run(Args::parse().test_name)
    // run(Some("TableView2Test".to_string()))
}

async fn test() -> Result<()> {
    test_base_ui().await?;
    test_base_views().await?;
    test_inspect().await?;
    test_layout().await?;
    test_complex_views().await?;
    test_image_views().await?;
    test_containers().await?;
    // test_input_views().await?;
    test_helper_views().await?;

    Ok(())
}
