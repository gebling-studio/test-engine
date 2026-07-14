use anyhow::Result;
use refs::Weak;

use crate::{
    self as test_engine,
    gm::color::{BLUE, GREEN, RED},
    ui::{
        Anchor::{Bot, Top},
        Container, Setup, ViewData, ViewTest, view_test,
    },
    ui_test::check_colors,
};

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
                380    4 -  89 124 149
                592    4 -  89 124 149
                40   24 - 255   0   0
                68   24 - 255   0   0
                24   28 - 255   0   0
                24   44 - 255   0   0
                52   44 - 255   0   0
                68   52 - 255   0   0
                36   68 - 255   0   0
                64   68 - 255   0   0
                24   92 -   0   0 231
                216  136 -   0   0 231
                448  180 -  89 124 149
                100  204 -   0   0 231
                24  252 -   0   0 231
                176  300 -   0   0 231
                300  300 -  89 124 149
                592  300 -  89 124 149
                60  336 -   0   0 231
                448  420 -  89 124 149
                108  432 -   0   0 231
                216  448 -   0   0 231
                24  532 -   0 255   0
                48  532 -   0 255   0
                68  532 -   0 255   0
                48  552 -   0 255   0
                28  556 -   0 255   0
                44  572 -   0 255   0
                24  576 -   0 255   0
                68  576 -   0 255   0
                376  592 -  89 124 149
                592  592 -  89 124 149
            ",
        )?;

        // record_ui_test();

        Ok(())
    }
}
