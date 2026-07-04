use anyhow::Result;
use test_engine::{
    refs::Weak,
    ui::{BLUE, Button, RED, Setup, ViewData, ViewTest, view_test},
    ui_test::check_colors,
};

#[view_test]
struct JustView {
    #[init]
    red:  Button,
    blue: Button,
}

impl Setup for JustView {
    fn setup(self: Weak<Self>) {
        self.red.set_color(RED).place().left_half();
        self.blue.set_color(BLUE).place().right_half();
    }
}

impl ViewTest for JustView {
    fn perform_test(_: Weak<Self>) -> Result<()> {
        check_colors(
            r"
                4    4 - 255   0   0
                152    4 - 255   0   0
                440    4 -   0   0 231
                592    4 -   0   0 231
                296    8 - 255   0   0
                516   72 -   0   0 231
                364   84 -   0   0 231
                444  148 -   0   0 231
                148  152 - 255   0   0
                592  152 -   0   0 231
                4  156 - 255   0   0
                292  156 - 255   0   0
                368  224 -   0   0 231
                512  224 -   0   0 231
                152  296 - 255   0   0
                440  296 -   0   0 231
                588  296 -   0   0 231
                4  300 - 255   0   0
                296  300 - 255   0   0
                84  372 - 255   0   0
                228  376 - 255   0   0
                444  440 -   0   0 231
                8  444 - 255   0   0
                304  444 -   0   0 231
                592  444 -   0   0 231
                152  448 - 255   0   0
                228  516 - 255   0   0
                448  584 -   0   0 231
                304  588 -   0   0 231
                4  592 - 255   0   0
                160  592 - 255   0   0
                592  592 -   0   0 231
            ",
        )?;

        Ok(())
    }
}
