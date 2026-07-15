use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{LIGHT_GRAY, Label, Setup, ViewData, ViewTest, WHITE, view},
    ui_test::helpers::check_colors,
};

#[view]
pub struct TextOccclusion {
    #[init]
    label_below: Label,
    label_above: Label,
}

impl Setup for TextOccclusion {
    fn setup(self: Weak<Self>) {
        self.label_below
            .set_color(WHITE)
            .set_text_size(100)
            .set_text("OOOOOOOO")
            .place()
            .size(400, 400)
            .center();

        self.label_above
            .set_text_size(140)
            .set_text("A A A A A")
            .set_color(LIGHT_GRAY)
            .place()
            .right_half();
    }
}

impl ViewTest for TextOccclusion {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        check_colors(
            r"
            4    4 -  89 124 149
            236    4 -  89 124 149
            396    4 - 231 231 231
            592   48 - 231 231 231
            156  104 - 255 255 255
            476  108 - 231 231 231
            296  116 - 255 255 255
            4  188 -  89 124 149
            112  200 - 255 255 255
            456  240 -   1   1   1
            576  248 -   1   1   1
            344  252 -   0   0   0
            204  264 -   1   1   1
            240  264 -   1   1   1
            4  300 -  89 124 149
            128  308 -   0   0   0
            308  312 -   0   0   0
            160  316 -   1   1   1
            204  316 -   1   1   1
            116  320 -   1   1   1
            412  332 -   0   0   0
            528  332 -   0   0   0
            296  380 - 255 255 255
            4  412 -  89 124 149
            564  420 - 231 231 231
            440  460 - 231 231 231
            132  484 - 255 255 255
            296  496 - 255 255 255
            4  592 -  89 124 149
            208  592 -  89 124 149
            428  592 - 231 231 231
            592  592 - 231 231 231
        ",
        )?;

        Ok(())
    }
}
