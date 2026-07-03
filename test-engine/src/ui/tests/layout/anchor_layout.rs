use anyhow::Result;
use crate::gm::color::{BLUE, GREEN, RED};
use refs::Weak;
use crate::ui::{
    Anchor::{Bot, Top},
    Container, Setup, ViewData, ViewTest, view_test,
};

use crate::{self as test_engine, ui_test::check_colors};

#[view_test]
pub(crate) struct AnchorLayoutTest {
    #[init]
    top:    Container,
    bot:    Container,
    target: Container,
}

impl Setup for AnchorLayoutTest {
    fn setup(self: Weak<Self>) {
        self.top.set_color(RED).place().tl(20).size(50, 50);
        self.bot.set_color(GREEN).place().bl(20).size(50, 50);
        self.target
            .set_color(BLUE)
            .place()
            .anchor(Top, self.top, 20)
            .l(20)
            .anchor(Bot, self.bot, 20)
            .w(200);
    }
}

impl ViewTest for AnchorLayoutTest {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        check_colors(
            r"
                      38   12 -  89 124 149
                      40   32 - 255   0   0
                      40   58 - 255   0   0
                      40   81 -  89 124 149
                      41  143 -   0   0 231
                      43  238 -   0   0 231
                      43  323 -   0   0 231
                      43  381 -   0   0 231
                      43  463 -   0   0 231
                      39  494 -   0   0 231
                      39  522 -  89 124 149
                      40  550 -   0 255   0
                      40  584 -  89 124 149
                      13  352 -  89 124 149
                      56  352 -   0   0 231
                     153  352 -   0   0 231
                     189  352 -   0   0 231
                     288  352 -  89 124 149
                ",
        )?;

        // record_ui_test();

        Ok(())
    }
}
