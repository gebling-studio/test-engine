use refs::main_lock::MainLock;
use render::data::{PathData, RectView, UIRectInstance};
use test_engine::ui::{BLUE, CLEAR, RED};
use window::{RenderPass, Window};

use crate::pipelines::{PATH, UI_RECT};

static PATH_DATA: MainLock<Option<PathData>> = MainLock::new();

pub(crate) fn render_path(pass: &mut RenderPass) {
    let path = PATH_DATA.set(
        PathData::new(
            BLUE,
            Window::render_size(),
            (200, 200).into(),
            0.5,
            &[
                (0, 0).into(),
                (80, 100).into(),
                (20, 200).into(),
                (200, 20).into(),
                (20, 50).into(),
            ],
        )
        .into(),
    );

    let path = path.as_ref().unwrap();

    PATH.draw(pass, path);

    UI_RECT.get_mut().add(UIRectInstance::new(
        (450, 200, 200, 200).into(),
        RED,
        CLEAR,
        0.0,
        0.0,
        0.5,
        1.0,
    ));

    UI_RECT.get_mut().draw(
        pass,
        RectView {
            resolution: Window::inner_size(),
            _padding:   0,
        },
    );
}
