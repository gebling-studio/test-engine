mod checks;
pub mod helpers;
mod report;
pub mod state;
mod ui_test;

use std::{
    fmt::Display,
    ops::Deref,
    sync::{Arc, mpsc::channel},
};

use anyhow::{Result, bail};
use gm::drop_on_main;
pub use helpers::*;
use hreads::{from_main, is_main_thread, on_main, wait_for_next_frame};
use log::{error, warn};
use parking_lot::Mutex;
use refs::Own;
pub use report::failure_report;
use serde::de::DeserializeOwned;
pub use state::*;
pub use ui_test::*;
use window::Window;

use crate::{
    AppRunner,
    gm::{LossyConvert, ToF32},
    ui::{Input, Touch, U8Color, UIEvents, UIManager},
};

pub fn test_combinations<const A: usize, Val>(comb: [(&'static str, Val); A]) -> Result<()>
where Val: Display + PartialEq + DeserializeOwned + Default + Send + 'static {
    for comb in comb {
        clear_state();

        let touches = Touch::vec_from_str(comb.0);

        for touch in touches {
            from_main(move || {
                inject_touch(touch);
            });
        }

        if get_state::<Val>() != comb.1 {
            error!(
                "Failed state for: {} Expected: {} got: {}",
                comb.0,
                comb.1,
                get_state::<Val>()
            );
            bail!("UI test failed")
        }
    }
    Ok(())
}

fn inject_touch(touch: impl Into<Touch> + Send + Copy + 'static) {
    Input::process_touch_event(touch.into());
}

#[allow(dead_code)]
pub fn inject_scroll(scroll: impl ToF32) {
    from_main(move || {
        Input::on_scroll((0, scroll).into());
    });
}

pub fn inject_touches(data: impl ToString + Send + 'static) {
    let scale = UIManager::scale();
    from_main(move || {
        for mut touch in Touch::vec_from_str(&data.to_string()) {
            touch.position *= scale;
            inject_touch(touch);
        }
    });
}

pub fn inject_touches_delayed(data: &str) {
    for touch in Touch::vec_from_str(data) {
        wait_for_next_frame();
        from_main(move || {
            inject_touch(touch);
        });
        wait_for_next_frame();
    }
}

pub fn inject_keys(s: impl ToString) {
    let s = s.to_string();
    for ch in s.chars() {
        inject_key(ch);
    }
}

pub fn inject_key(key: char) {
    from_main(move || Input::on_char(key));
}

#[allow(dead_code)]
pub fn record_touches() {
    record_touches_internal(true);
}

#[allow(dead_code)]
pub fn record_moved_touches() {
    record_touches_internal(false);
}

fn record_touches_internal(skip_moved: bool) {
    let touches_own: Own<_> = Vec::<Touch>::new().into();
    let mut touches = touches_own.weak();

    let (s, r) = channel::<()>();

    let moved_record_skip = 10;

    let moved_counter = Arc::new(Mutex::new(0));

    on_main(move || {
        UIEvents::on_touch().val(move |touch| {
            if touch.is_moved() {
                let mut counter = moved_counter.lock();
                *counter += 1;
                if *counter == moved_record_skip {
                    *counter = 0;
                } else {
                    return;
                }
            }

            if skip_moved && touch.is_moved() {
                return;
            }

            touches.push(touch);
        });

        UIManager::keymap().add(UIManager::root_view(), 'a', move || {
            _ = s.send(());
        });
    });

    if r.recv().is_err() {
        warn!("Failed to receive record_touches result");
    }

    from_main(|| {
        UIEvents::on_touch().remove_subscribers();
    });

    println!(
        r#"
        inject_touches(
        "
{}
        ",
    );
    "#,
        Touch::str_from_vec(touches.to_vec()),
    );

    drop_on_main(touches_own);
}

#[allow(dead_code)]
pub fn record_ui_test() {
    if is_main_thread() {
        panic!("record_ui_test is blocking function. It shouldn't be called on main thread.");
    }

    loop {
        Window::set_title("Recording touches");
        record_touches();
        Window::set_title("Recording colors");
        record_colors().unwrap();
    }
}

#[allow(dead_code)]
pub fn record_colors() -> Result<()> {
    let touch_lock = Touch::lock();

    let screenshot = AppRunner::take_screenshot()?;

    let touches_own: Own<_> = Vec::<(Touch, U8Color)>::new().into();
    let mut touches = touches_own.weak();

    let (s, r) = channel::<()>();

    on_main(move || {
        UIEvents::on_debug_touch().val(move |touch| {
            if !touch.is_began() {
                return;
            }

            touches.push((touch, screenshot.get_pixel(touch.position)));
        });

        UIManager::keymap().add(UIManager::root_view(), 'a', move || {
            _ = s.send(());
        });
    });

    if r.recv().is_err() {
        warn!("Failed to receive record_touches_with_colors result");
    }

    on_main(|| {
        UIEvents::on_touch().remove_subscribers();
        UIEvents::on_debug_touch().remove_subscribers();
    });

    println!("check_colors( r\"");

    for (touch, color) in touches.deref() {
        let x: u32 = touch.position.x.lossy_convert();
        let y: u32 = touch.position.y.lossy_convert();
        println!(
            "            {:>4} {:>4} - {:>3} {:>3} {:>3}",
            x, y, color.r, color.g, color.b
        );
    }

    println!("        \"");
    println!(")?;");

    drop(touch_lock);

    drop_on_main(touches_own);

    Ok(())
}
