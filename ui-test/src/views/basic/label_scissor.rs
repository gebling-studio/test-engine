use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{Label, Setup, ViewData, ViewTest, WHITE, view_test},
};

#[view_test]
struct LabelScissor {
    #[init]
    label: Label,
}

impl Setup for LabelScissor {
    fn setup(self: Weak<Self>) {
        self.label.set_text("ßšėčыў").set_color(WHITE);
        self.label.place().size(280, 280).tl(80);
    }
}

impl ViewTest for LabelScissor {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        test_engine::ui_test::record_ui_test();

        Ok(())
    }
}
