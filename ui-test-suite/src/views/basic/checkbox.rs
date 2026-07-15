use test_engine::{
    refs::Weak,
    ui::{CheckBox, Setup, ViewFrame, ui_test, view},
    ui_test::{UITest, inject_touches},
};

#[view]
struct CheckBoxTestView {
    #[init]
    checkbox: CheckBox,
}

impl Setup for CheckBoxTestView {
    fn setup(self: Weak<Self>) {
        self.checkbox.set_frame((50, 50, 50, 50));
    }
}

#[ui_test]
pub fn test_checkbox() {
    let view = UITest::start::<CheckBoxTestView>();

    assert!(!view.checkbox.on());

    inject_touches(
        "
         81   86   b
         81   86   e

     ",
    );

    assert!(view.checkbox.on());

    inject_touches(
        "
         81   86   b
         81   86   e

     ",
    );

    assert!(!view.checkbox.on());
}
