use test_engine::{
    gm::LossyConvert,
    refs::{Weak, manage::DataManager},
    ui::{Font, Label, Setup, TextAlignment, ViewData, ViewSubviews, view},
};

use crate::interface::{
    palette::{BG, TEXT, TEXT_DIM},
    scenes::add_back_button,
};

// Font file and a short sample. Each renders in its own typeface to show
// the shaping pipeline across very different scripts.
const FONTS: [(&str, &str); 5] = [
    ("RussoOne-Regular.ttf", "Russo One heading"),
    ("OpenSans.ttf", "Open Sans stays clean"),
    ("Monoton-Regular.ttf", "MONOTON NEON 88"),
    ("Neucha.ttf", "Neucha soft handwriting"),
    ("SpecialElite-Regular.ttf", "Special Elite typewriter"),
];

/// A stack of labels, each in a different font, plus a letter spacing
/// row and a multiline row.
#[view]
pub struct TextFonts {}

impl Setup for TextFonts {
    fn setup(self: Weak<Self>) {
        self.set_color(BG);

        let title = self.add_view::<Label>();
        title
            .set_text("Text and Fonts")
            .set_text_color(TEXT)
            .set_text_size(24)
            .set_font(Font::get("RussoOne-Regular.ttf"))
            .set_alignment(TextAlignment::Center);
        title.place().t(18).center_x().w(360).h(36);

        for (i, (font, sample)) in FONTS.into_iter().enumerate() {
            let y = 66.0 + 42.0 * i.lossy_convert();
            let label = self.add_view::<Label>();
            label
                .set_text(sample)
                .set_text_color(TEXT)
                .set_text_size(28)
                .set_font(Font::get(font));
            label.place().t(y).l(24).r(24).h(38);
        }

        let spacing = self.add_view::<Label>();
        spacing
            .set_text("LETTER SPACING")
            .set_text_color(TEXT_DIM)
            .set_text_size(24)
            .set_letter_spacing(10);
        spacing.place().t(410).l(24).r(24).h(34);

        let multiline = self.add_view::<Label>();
        multiline
            .set_text(
                "This is a multiline label. It wraps across several lines when the text is longer than its width, so paragraphs stay readable.",
            )
            .set_text_color(TEXT)
            .set_text_size(18)
            .set_multiline(true);
        multiline.place().t(452).l(24).r(24).h(110);

        add_back_button(self);
    }
}
