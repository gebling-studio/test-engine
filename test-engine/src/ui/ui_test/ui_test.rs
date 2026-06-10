use std::{any::type_name, time::Instant};

use hreads::{from_main, wait_for_next_frame};
use log::{debug, trace};
use parking_lot::Mutex;
use refs::{Own, Weak};
use ui::{Setup, UIManager, View, ViewData, ViewTest};
use window::Window;

use crate::{gm::LossyConvert, ui_test::clear_state};

pub static TEST_NAME: Mutex<String> = Mutex::new(String::new());

struct FpsRecord {
    name:    String,
    frames:  u32,
    seconds: f32,
}

static FPS_RECORDS: Mutex<Vec<FpsRecord>> = Mutex::new(Vec::new());
static FPS_SPAN: Mutex<Option<(String, u32, Instant)>> = Mutex::new(None);

fn record_test_boundary(new_test: Option<String>) {
    let frames = from_main(|| Window::current().frame_drawn());
    let now = Instant::now();

    let mut span = FPS_SPAN.lock();

    if let Some((name, start_frames, start_time)) = span.take() {
        FPS_RECORDS.lock().push(FpsRecord {
            name,
            frames: frames - start_frames,
            seconds: (now - start_time).as_secs_f32(),
        });
    }

    if let Some(name) = new_test {
        *span = Some((name, frames, now));
    }
}

fn print_fps_report() {
    let records = FPS_RECORDS.lock();

    if records.is_empty() {
        return;
    }

    let width = records.iter().map(|r| r.name.len()).max().unwrap_or(0).max(4);

    println!();
    println!("FPS report:");
    println!("{:<width$}  frames     secs    fps", "test");

    for r in records.iter() {
        let frames: f32 = r.frames.lossy_convert();
        let fps = if r.seconds > 0.0 { frames / r.seconds } else { 0.0 };
        println!("{:<width$}  {:>6}  {:>7.2}  {:>5.1}", r.name, r.frames, r.seconds, fps);
    }
}

pub struct UITest;

impl UITest {
    pub fn start<T: View + ViewTest + Default + 'static>() -> Weak<T> {
        Self::set(T::new(), 600, 600, true, get_test_name::<T>())
    }

    pub fn reload<T: View + ViewTest + Default + 'static>() -> Weak<T> {
        Self::set(T::new(), 600, 600, false, get_test_name::<T>())
    }

    pub fn set<T: View + 'static>(
        view: Own<T>,
        width: u32,
        height: u32,
        test_start: bool,
        new_test_name: String,
    ) -> Weak<T> {
        if test_start {
            let test_name = TEST_NAME.lock().clone();

            if !test_name.is_empty() {
                debug!("{test_name}: OK");
            }

            debug!("{new_test_name}: Started");

            record_test_boundary(Some(new_test_name.clone()));
        }

        TEST_NAME.lock().clone_from(&new_test_name);

        clear_state();

        #[cfg(desktop)]
        {
            crate::AppRunner::set_window_size((width, height));
        }
        wait_for_next_frame();
        let view = from_main(move || {
            let weak = view.weak();
            let mut root = UIManager::root_view();
            root.clear_root();
            let view = root.add_subview_to_root(view);
            view.place().back();
            trace!("{width} - {height}");
            weak
        });
        wait_for_next_frame();

        view
    }

    pub fn finish() {
        let test_name = TEST_NAME.lock().clone();

        if !test_name.is_empty() {
            debug!("{test_name}: OK");
        }

        TEST_NAME.lock().clear();

        record_test_boundary(None);
        print_fps_report();
    }
}

fn get_test_name<T>() -> String {
    let input = type_name::<T>();

    let last_part = input.split("::").last().unwrap();

    last_part
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i > 0 && c.is_uppercase() {
                format!(" {}", c.to_ascii_lowercase())
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
}
