use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{LIGHT_GRAY, Label, ScrollView, Setup, ViewData, ViewSubviews, ViewTest, WHITE, view_test},
};

#[view_test]
struct LabelScissor {
    #[init]
    label:  Label,
    scroll: ScrollView,
}

impl Setup for LabelScissor {
    fn setup(mut self: Weak<Self>) {
        self.label.set_text("ßšėčыў").set_color(WHITE);
        self.label.place().size(150, 50).tl(20);

        self.scroll.place().below(self.label, 20).w(200).h(400);
        self.scroll.set_content_size((200, 1000));

        self.scroll
            .add_view::<Label>()
            .set_text("Scrolling")
            .set_color(LIGHT_GRAY)
            .place()
            .center()
            .size(150, 50);
    }
}

impl ViewTest for LabelScissor {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        test_engine::ui_test::record_ui_test();

        Ok(())
    }
}
