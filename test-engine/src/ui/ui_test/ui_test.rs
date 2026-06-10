use std::any::type_name;

use hreads::{from_main, wait_for_next_frame};
use log::{debug, trace};
use parking_lot::Mutex;
use refs::{Own, Weak};
use ui::{Setup, UIManager, View, ViewData, ViewTest};

use crate::ui_test::clear_state;

pub static TEST_NAME: Mutex<String> = Mutex::new(String::new());

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
