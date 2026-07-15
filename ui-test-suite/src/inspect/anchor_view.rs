use test_engine::{
    dispatch::from_main,
    inspect::views::AnchorView,
    refs::Weak,
    ui::{Anchor, Setup, ViewFrame, ui_test, view},
    ui_test::UITest,
};

#[view]
struct AnchorViewTest {
    #[init]
    anchor_view: AnchorView,
}

impl Setup for AnchorViewTest {
    fn setup(self: Weak<Self>) {
        self.anchor_view.set_frame((50, 50, 200, 200));
        self.anchor_view.set_anchor(Anchor::Top);
    }
}

#[ui_test]
pub(crate) fn test_anchor_view() {
    let view = UITest::start::<AnchorViewTest>();

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::Bot);
    });

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::Left);
    });

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::Right);
    });

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::Width);
    });

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::Height);
    });

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::MaxWidth);
    });

    from_main(move || {
        view.anchor_view.set_anchor(Anchor::MaxHeight);
    });
}
