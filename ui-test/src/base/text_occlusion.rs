use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{LIGHT_GRAY, Label, Setup, ViewData, WHITE, view},
    ui_test::{UITest, helpers::check_colors},
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

pub async fn test_text_occlusion() -> Result<()> {
    let _view = UITest::start::<TextOccclusion>();

    check_colors(
        r"
               4    4 -  89 124 149
             592    4 - 231 231 231
             296   16 -  89 124 149
             448   80 - 231 231 231
             148   92 -  89 124 149
               4  152 -  89 124 149
             296  156 - 255 255 255
             584  160 - 231 231 231
             204  168 - 255 255 255
             340  244 -  10  10  10
             444  244 -  10  10  10
             560  244 -  10  10  10
             108  256 -  13  13  13
             160  264 -  13  13  13
             204  264 -  13  13  13
             240  264 -  13  13  13
             284  308 -  13  13  13
             160  316 -  13  13  13
             204  316 -  13  13  13
             592  316 -   0   0   0
             372  332 -   0   0   0
             488  332 -   0   0   0
               4  376 -  89 124 149
             104  392 - 255 255 255
             192  424 - 255 255 255
             428  456 - 231 231 231
             284  488 - 255 255 255
             104  496 - 255 255 255
             592  500 - 231 231 231
               4  592 -  89 124 149
             320  592 - 231 231 231
             432  592 - 231 231 231
            ",
    )?;

    Ok(())
}
