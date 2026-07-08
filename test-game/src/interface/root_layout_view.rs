use test_engine::{
    refs::Weak,
    ui::{
        Container, LIGHT_BLUE, NoImage, NumberView, Setup, Style, UIManager, ViewData, ViewSubviews,
        ViewTest, view_test,
    },
    ui_test::check_colors,
};

use crate::interface::HAS_BACK_BUTTON;

const CORNER_STYLE: Style = Style::new(|v| {
    v.set_color(LIGHT_BLUE).place().size(80, 80);
});

#[view_test]
pub struct RootLayoutView {
    #[init]
    scale: NumberView,
}

impl Setup for RootLayoutView {
    fn setup(self: Weak<Self>) {
        UIManager::enable_debug_frames();
        UIManager::root_view().set_image("square.png");

        self.apply_style(HAS_BACK_BUTTON);

        self.add_view::<Container>().apply_style(CORNER_STYLE).place().tl(0);
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().tr(0);
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().br(0);
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().bl(0);

        self.add_view::<Container>().apply_style(CORNER_STYLE).place().t(0).center_x();
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().l(0).center_y();
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().r(0).center_y();
        self.add_view::<Container>().apply_style(CORNER_STYLE).place().b(0).center_x();

        self.scale
            .set_min(0.2)
            .set_step(0.1)
            .set_value(1)
            .place()
            .center()
            .size(100, 200);
        self.scale.on_change(|scale| {
            UIManager::set_scale(scale);
        });
    }
}

impl Drop for RootLayoutView {
    fn drop(&mut self) {
        UIManager::disable_debug_frames();
        UIManager::root_view().set_image(NoImage);
    }
}

impl ViewTest for RootLayoutView {
    fn perform_test(_view: Weak<Self>) -> anyhow::Result<()> {
        check_colors(
            r"
                188    4 -  94 103 222
                304    4 -   0 218 255
                436    4 - 109  90 224
                592    8 -   0 218 255
                4   32 -   0 218 255
                536   88 - 121  92 231
                108   96 -  98 113 224
                312  116 - 126 108 241
                456  144 - 120  94 233
                592  168 - 132  71 226
                336  204 -   0 150 230
                260  212 -   0 150 230
                32  216 - 255 255 255
                76  224 - 255 255 255
                36  228 - 255 255 255
                52  228 - 255 255 255
                284  268 - 255 255 255
                344  280 -   0 150 230
                492  288 - 116  59 222
                204  304 - 134  93 238
                244  304 - 134  93 238
                320  332 - 255 255 255
                244  344 - 134  93 238
                300  372 - 255 255 255
                120  396 -  89  60 202
                592  396 - 123  31 215
                4  444 - 130 102 242
                376  504 - 120  26 213
                584  516 - 103   0 199
                92  592 - 110  48 209
                264  592 -   0 218 255
                488  592 - 128  16 218
            ",
        )?;

        // test_engine::ui_test::record_ui_test();

        Ok(())
    }
}
