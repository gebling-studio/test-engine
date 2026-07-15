use log::debug;
use test_engine::{
    refs::Weak,
    ui::{KeyboardView, Setup, ViewData, ui_test, view},
    ui_test::UITest,
};

#[view]
struct KeyboardViewTest {
    #[init]
    keyboard: KeyboardView,
}

impl Setup for KeyboardViewTest {
    fn setup(self: Weak<Self>) {
        self.keyboard.place().back();
    }
}

#[ui_test]
pub fn test_keyboard_view() {
    let _view = UITest::start::<KeyboardViewTest>();

    //  record_ui_test().await;

    debug!("Keyboard view: OK");
}
