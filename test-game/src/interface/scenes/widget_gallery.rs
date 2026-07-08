use test_engine::{
    refs::{Weak, manage::DataManager},
    ui::{
        Alert, Button, CheckBox, Container, DropDown, Font, Label, NumberView, ProgressView, Setup, Spinner,
        Switch, TextAlignment, TextField, ViewData, ViewSubviews, WHITE, view,
    },
};

use crate::interface::{
    palette::{ACCENT, BG, BORDER, SURFACE, TEXT, TEXT_DIM},
    scenes::add_back_button,
};

/// A wrapped grid of tiles, one widget per tile with a caption, so every
/// common control can be seen and poked in one place.
#[view]
pub struct WidgetGallery {
    #[init]
    title: Label,
    grid:  Container,
}

impl Setup for WidgetGallery {
    fn setup(self: Weak<Self>) {
        self.set_color(BG);

        self.title
            .set_text("Widget Gallery")
            .set_text_color(TEXT)
            .set_text_size(22)
            .set_font(Font::get("RussoOne-Regular.ttf"))
            .set_alignment(TextAlignment::Center);
        self.title.place().t(20).center_x().w(320).h(34);

        self.grid.place().t(72).lr(20).all(14).all_wrap();

        self.add_widgets();

        add_back_button(self);
    }
}

impl WidgetGallery {
    fn tile(self: Weak<Self>, caption: &str) -> Weak<Container> {
        let tile = self.grid.add_view::<Container>();
        tile.set_color(SURFACE)
            .set_corner_radius(12)
            .set_border_width(1)
            .set_border_color(BORDER);
        tile.place().size(168, 150);

        let cap = tile.add_view::<Label>();
        cap.set_text(caption)
            .set_text_color(TEXT_DIM)
            .set_text_size(12)
            .set_alignment(TextAlignment::Center);
        cap.place().t(8).lr(6).h(16);

        tile
    }

    fn add_widgets(self: Weak<Self>) {
        let btn = self.tile("Button").add_view::<Button>();
        btn.set_text("Tap me")
            .set_color(ACCENT)
            .set_text_color(WHITE)
            .set_corner_radius(8);
        btn.on_tap(|| {
            Alert::show("Hello from the widget gallery");
        });
        btn.place().t(48).center_x().size(120, 40);

        self.tile("CheckBox")
            .add_view::<CheckBox>()
            .place()
            .t(48)
            .center_x()
            .size(46, 46);

        let mut toggle = self.tile("Switch").add_view::<Switch>();
        toggle.set_off_color(BORDER.light);
        toggle.place().t(52).center_x().size(84, 40);

        self.tile("ProgressView")
            .add_view::<ProgressView>()
            .inc_progress(0.65)
            .place()
            .t(64)
            .center_x()
            .size(130, 14);

        let holder = self.tile("Spinner").add_view::<Container>();
        holder.place().t(38).center_x().size(60, 60);
        let mut spinner = holder.add_view::<Spinner>();
        spinner.dot_color = ACCENT.dark;
        spinner.place().back();

        let mut drop = self.tile("DropDown").add_view::<DropDown<&'static str>>();
        drop.set_values(vec!["One", "Two", "Three"]);
        drop.place().t(52).center_x().size(140, 36);

        let stepper = self.tile("NumberView");
        let value = stepper.add_view::<Label>();
        value
            .set_text("1")
            .set_text_color(TEXT)
            .set_text_size(15)
            .set_alignment(TextAlignment::Center);
        value.place().b(10).lr(6).h(18);
        let number = stepper.add_view::<NumberView>();
        number.set_value(1).on_change(move |v| {
            value.set_text(format!("{v:.0}"));
        });
        number.place().t(34).center_x().size(52, 70);

        let field = self.tile("TextField").add_view::<TextField>();
        field.set_placeholder("Type here").set_text_size(15);
        field.place().t(52).center_x().size(150, 36);
    }
}
