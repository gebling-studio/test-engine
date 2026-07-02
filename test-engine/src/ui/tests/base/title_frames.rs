use anyhow::{Result, ensure};
use hreads::{from_main, wait_for_next_frame};
use refs::Weak;
use ui::{Setup, ViewTest, view_test};
use window::Window;

use crate as test_engine;

// Regression test. Setting a custom title used to stop the frame counter,
// freezing fps, frame_time and frame_drawn forever.
#[view_test]
struct TitleFrames {}

impl Setup for TitleFrames {
    fn setup(self: Weak<Self>) {}
}

impl ViewTest for TitleFrames {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        Window::set_title("Custom title");

        wait_for_next_frame();
        wait_for_next_frame();

        let before = from_main(|| Window::current().frame_drawn());

        for _ in 0..5 {
            wait_for_next_frame();
        }

        let after = from_main(|| Window::current().frame_drawn());

        ensure!(
            after > before,
            "frame counter froze after set_title: {before} -> {after}"
        );

        Ok(())
    }
}
