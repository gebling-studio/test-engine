use std::{
    env::var,
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, channel},
    },
    thread::sleep,
    time::Duration,
};

use gm::{
    LossyConvert,
    color::{BLACK, CLEAR, WHITE},
};
use hreads::from_main;
use log::warn;
use parking_lot::Mutex;
use ui::{Container, Setup, UIManager, ViewData, ViewFrame, ViewSubviews, WeakView};
use window::Window;

use crate::ui_test::TEST_NAME;

static HUMAN_MODE: AtomicBool = AtomicBool::new(false);
static ADVANCE: OnceLock<Mutex<Receiver<()>>> = OnceLock::new();

/// Slows down injections and holds after each test until space is pressed,
/// so a human can watch the tests run. Enabled by `--human` in ui-test.
pub fn enable_human_mode() {
    HUMAN_MODE.store(true, Ordering::Relaxed);
}

pub fn human_mode() -> bool {
    HUMAN_MODE.load(Ordering::Relaxed)
}

fn delay() -> Duration {
    let ms = var("UI_TEST_HUMAN_DELAY").ok().and_then(|ms| ms.parse().ok()).unwrap_or(400);
    Duration::from_millis(ms)
}

pub(crate) fn human_pause() {
    if human_mode() {
        sleep(delay());
    }
}

/// Shorter pause for moved touches, a full delay per move would
/// stretch a recorded drag into minutes.
pub(crate) fn human_pause_quick() {
    if human_mode() {
        sleep(delay() / 8);
    }
}

pub(crate) fn hold_for_human() {
    if !human_mode() {
        return;
    }

    let test_name = TEST_NAME.lock().clone();
    Window::set_title(format!("{test_name}: OK - space to continue"));

    wait_for_space();
}

/// Marks every checked pixel with a square around it, the pixel in the
/// center, and holds until space. Black square in a white one, so
/// markers stay visible on any background.
pub(crate) fn show_probes(positions: &[(u32, u32)], test_name: &str, index: usize) {
    let positions = positions.to_vec();

    let markers = from_main(move || {
        let mut markers: Vec<WeakView> = vec![];

        for (x, y) in positions {
            let x: f32 = x.lossy_convert();
            let y: f32 = y.lossy_convert();

            for (size, color) in [(12.0, WHITE), (10.0, BLACK)] {
                let mut view = Container::new();
                view.set_z_position(0.1);
                view.set_color(CLEAR).set_border_color(color).set_border_width(1).set_frame((
                    x - size / 2.0,
                    y - size / 2.0,
                    size,
                    size,
                ));
                markers.push(UIManager::root_view().add_subview_to_root(view));
            }
        }

        markers
    });

    Window::set_title(format!("{test_name} check {index}: space to continue"));
    wait_for_space();

    from_main(move || {
        for mut marker in markers {
            marker.remove_from_superview();
        }
    });
}

pub(crate) fn wait_for_space() {
    let receiver = ADVANCE.get_or_init(|| {
        let (sender, receiver) = channel();

        from_main(move || {
            UIManager::keymap().add(UIManager::root_view(), ' ', move || {
                if sender.send(()).is_err() {
                    warn!("Failed to send human continue signal");
                }
            });
        });

        Mutex::new(receiver)
    });

    let receiver = receiver.lock();

    while receiver.try_recv().is_ok() {}

    if receiver.recv().is_err() {
        warn!("Failed to receive human continue signal");
    }
}
