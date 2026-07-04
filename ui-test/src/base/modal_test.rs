use anyhow::Result;
use test_engine::{
    OnceEvent,
    refs::Weak,
    ui::{
        Color, Container, Label, ModalView, Setup, Size, ViewData, ViewFrame, ViewSubviews, WHITE, WeakView,
        ui_test::helpers::check_colors, view,
    },
    ui_test::UITest,
};

#[view]
struct ShowModally {}

impl Setup for ShowModally {
    fn setup(self: Weak<Self>) {
        let mut view = WeakView::default();

        for _ in 0..200 {
            if view.is_ok() {
                view = view.add_view::<Container>();
                view.set_color(Color::random()).place().all_sides(1);
            } else {
                view = self.add_view::<Container>();
                view.set_color(Color::random()).place().tl(1).size(400, 400);
                assert_eq!(view.z_position(), 0.49_996_987);
            }
        }

        assert_eq!(view.z_position(), 0.49_797_717);
    }
}

#[view]
struct Modal {
    event: OnceEvent,

    #[init]
    label: Label,
}

impl Setup for Modal {
    fn setup(self: Weak<Self>) {
        self.label.place().back();
        self.label.set_text_size(100);
        self.label.set_text("Hello");
        self.label.set_color(WHITE);
    }
}

impl ModalView for Modal {
    fn modal_event(&self) -> &OnceEvent<()> {
        &self.event
    }

    fn modal_size() -> Size {
        (400, 400).into()
    }
}

pub async fn test_modal() -> Result<()> {
    UITest::start::<ShowModally>();

    Modal::show_modally_with_input((), |()| {});

    check_colors(
        r#"
            592    4 -  89 124 149
            104  104 - 255 255 255
            432  104 - 255 255 255
            268  108 - 255 255 255
            484  204 - 255 255 255
            592  232 -  89 124 149
            200  260 -   0   0   0
            344  260 -   0   0   0
            272  284 -   0   0   0
            284  284 - 255 255 255
            288  284 - 255 255 255
            300  284 -   0   0   0
            372  284 -   0   0   0
            400  284 -   1   1   1
            236  288 -   0   0   0
            200  292 -   0   0   0
            404  312 -   1   1   1
            300  316 -   0   0   0
            272  320 -   0   0   0
            296  320 -   0   0   0
            372  320 -   1   1   1
            196  324 -   0   0   0
            324  324 -   0   0   0
            588  380 -  89 124 149
            4  404 -  89 124 149
            240  432 - 255 255 255
            348  472 - 255 255 255
            132  480 - 255 255 255
            496  496 - 255 255 255
            4  592 -  89 124 149
            260  592 -  89 124 149
            592  592 -  89 124 149
        "#,
    )?;

    Ok(())
}
