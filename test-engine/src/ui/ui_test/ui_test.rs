use std::{
    any::type_name,
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

use hreads::{from_main, wait_for_next_frame};
use log::{debug, trace};
use parking_lot::Mutex;
use refs::{Own, Weak};

use crate::{
    gm::{LossyConvert, color::GRAY_BLUE},
    ui::{Setup, UIManager, View, ViewData, ViewTest},
    ui_test::{clear_state, hold_for_human, human_mode, reset_record_probe_count, set_record_canvas},
    window::Window,
};

pub(crate) static TEST_NAME: Mutex<String> = Mutex::new(String::new());

/// Name of the test currently running, for failure attribution from a panic
/// hook where the returned error is not available.
pub fn current_test_name() -> String {
    TEST_NAME.lock().clone()
}

struct FpsRecord {
    name:    String,
    frames:  u32,
    seconds: f32,
}

static FPS_REPORT: AtomicBool = AtomicBool::new(false);
static FPS_RECORDS: Mutex<Vec<FpsRecord>> = Mutex::new(Vec::new());
static FPS_SPAN: Mutex<Option<(String, u32, Instant)>> = Mutex::new(None);

/// Record fps of every test and print a report at the end of the run.
pub fn enable_fps_report() {
    FPS_REPORT.store(true, Ordering::Relaxed);
}

fn record_test_boundary(new_test: Option<String>) {
    if !FPS_REPORT.load(Ordering::Relaxed) {
        return;
    }

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
        println!(
            "{:<width$}  {:>6}  {:>7.2}  {:>5.1}",
            r.name, r.frames, r.seconds, fps
        );
    }
}

pub struct UITest;

impl UITest {
    pub fn start<T: View + ViewTest + Default + 'static>() -> Weak<T> {
        Self::set(T::new(), 600, 600, true, get_test_name::<T>())
    }

    /// A canvas other than the default 600 by 600. It must still fit the
    /// smallest supported screen, which is 640 by 1136 pixels.
    pub fn start_sized<T: View + ViewTest + Default + 'static>(width: u32, height: u32) -> Weak<T> {
        Self::set(T::new(), width, height, true, get_test_name::<T>())
    }

    pub fn reload<T: View + ViewTest + Default + 'static>() -> Weak<T> {
        Self::set(T::new(), 600, 600, false, get_test_name::<T>())
    }

    /// Rebuild the view on a different canvas without starting a new test.
    pub fn reload_sized<T: View + ViewTest + Default + 'static>(width: u32, height: u32) -> Weak<T> {
        Self::set(T::new(), width, height, false, get_test_name::<T>())
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
                hold_for_human();
            }

            debug!("{new_test_name}: Started");

            reset_record_probe_count();

            if human_mode() {
                Window::set_title(new_test_name.clone());
            }

            record_test_boundary(Some(new_test_name.clone()));
        }

        TEST_NAME.lock().clone_from(&new_test_name);

        set_record_canvas(width, height);

        clear_state();

        wait_for_next_frame();

        from_main(move || {
            let weak = view.weak();
            let mut root = UIManager::root_view();
            root.clear_root();
            root.reset_background();
            root.set_test_canvas((width, height).into());
            Window::set_clear_color(GRAY_BLUE);
            root.add_subview_to_root(view).place().back();

            trace!("{width} - {height}");
            weak
        })
    }

    pub fn finish() {
        let test_name = TEST_NAME.lock().clone();

        if !test_name.is_empty() {
            debug!("{test_name}: OK");
            hold_for_human();
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
