use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{BLACK, BLUE, Button, Container, GREEN, RED, Setup, ViewData, ViewSubviews, view},
    ui_test::{UITest, check_colors},
};

#[view]
struct TilingLayout {
    #[init]
    menu: Container,
}

impl Setup for TilingLayout {
    fn setup(self: Weak<Self>) {
        self.menu.set_color(BLACK).place().tl(20).size(280, 280).all_ver();

        self.menu.add_view::<Container>().set_color(RED);
        self.menu.add_view::<Container>().set_color(GREEN);
        self.menu.add_view::<Container>().set_color(BLUE);
    }
}

pub async fn test_tiling_layout() -> anyhow::Result<()> {
    let view = UITest::start::<TilingLayout>();

    check_colors(
        r"
         472    4 -  89 124 149
          44   24 - 255   0   0
         140   24 - 255   0   0
         236   24 - 255   0   0
         288   24 - 255   0   0
          92   44 - 255   0   0
         216   72 - 255   0   0
         104   96 - 255   0   0
         164  108 - 255   0   0
          24  116 -   0 255   0
         280  116 -   0 255   0
         592  144 -  89 124 149
         248  156 -   0 255   0
         196  160 -   0 255   0
          48  168 -   0 255   0
         104  176 -   0 255   0
         296  192 -   0 255   0
         236  204 -   0 255   0
          24  224 -   0   0 231
         140  236 -   0   0 231
          80  240 -   0   0 231
         264  256 -   0   0 231
         204  264 -   0   0 231
          28  296 -   0   0 231
         104  296 -   0   0 231
         160  296 -   0   0 231
         304  300 -  89 124 149
         528  368 -  89 124 149
         152  476 -  89 124 149
           4  592 -  89 124 149
         300  592 -  89 124 149
         592  592 -  89 124 149
        ",
    )?;

    from_main(move || {
        view.menu.remove_all_subviews();
    });

    check_colors(
        r"
         592    4 -  89 124 149
         444    8 -  89 124 149
          24   24 -   0   0   0
         228   24 -   0   0   0
         296   28 -   0   0   0
         160   32 -   0   0   0
          92   36 -   0   0   0
          32   92 -   0   0   0
         228   96 -   0   0   0
         100  104 -   0   0   0
         480  152 -  89 124 149
          28  160 -   0   0   0
         296  160 -   0   0   0
         168  168 -   0   0   0
          36  228 -   0   0   0
         236  228 -   0   0   0
         104  236 -   0   0   0
          32  296 -   0   0   0
         176  296 -   0   0   0
         592  296 -  89 124 149
         304  300 -  89 124 149
         448  300 -  89 124 149
         592  440 -  89 124 149
           4  444 -  89 124 149
         296  444 -  89 124 149
         452  444 -  89 124 149
         152  456 -  89 124 149
         444  588 -  89 124 149
           4  592 -  89 124 149
         160  592 -  89 124 149
         300  592 -  89 124 149
         592  592 -  89 124 149
        ",
    )?;

    from_main(move || {
        view.menu
            .add_view::<Button>()
            .add_transition::<TilingLayout, TilingLayout>()
            .set_text("Classic")
            .set_text_size(80);

        view.menu
            .add_view::<Button>()
            .add_transition::<TilingLayout, TilingLayout>()
            .set_text("Custom Game")
            .set_text_size(80);

        view.menu
            .add_view::<Button>()
            .add_transition::<TilingLayout, TilingLayout>()
            .set_text("Settings")
            .set_text_size(80);
    });

    check_colors(
        r"
            592    4 -  89 124 149
            24   24 - 255 255 255
            240   24 - 255 255 255
            72   36 -   1   1   1
            116   56 -   0   0   0
            212   56 - 255 255 255
            172   76 - 255 255 255
            280   80 -   1   1   1
            60   92 - 255 255 255
            116  120 - 255 255 255
            220  128 -   0   0   0
            80  144 -   0   0   0
            24  148 - 255 255 255
            284  148 - 255 255 255
            160  164 - 255 255 255
            216  176 -   0   0   0
            92  180 -   0   0   0
            260  208 - 255 255 255
            144  224 -   0   0   0
            200  236 -   0   0   0
            88  240 - 255 255 255
            32  244 -   0   0   0
            268  256 - 255 255 255
            296  264 -   1   1   1
            168  268 -   0   0   0
            124  272 -   1   1   1
            240  280 - 255 255 255
            588  300 -  89 124 149
            428  416 -  89 124 149
            300  568 -  89 124 149
            4  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )?;

    Ok(())
}
