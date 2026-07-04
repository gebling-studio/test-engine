use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{BLUE, Container, GREEN, Setup, ViewData, ViewFrame, ViewSubviews, view},
    ui_test::{UITest, check_colors},
};

#[view]
struct RelativeLayout {
    view: Weak<Container>,

    #[init]
    parent: Container,
}

impl Setup for RelativeLayout {
    fn setup(mut self: Weak<Self>) {
        self.parent.set_color(BLUE);
        self.parent.set_frame((50, 50, 200, 200));

        self.view = self.parent.add_view();

        self.view.set_color(GREEN);
        self.view
            .place()
            .relative_size(self.parent, 0.4)
            .relative_x(0.2)
            .relative_y(0.5);
    }
}

pub async fn test_relative_layout() -> Result<()> {
    let view = UITest::start::<RelativeLayout>();

    check_colors(
        r#"
            392    4 -  89 124 149
            592    4 -  89 124 149
            52   52 -   0   0 231
            152   52 -   0   0 231
            248   72 -   0   0 231
            192   96 -   0   0 231
            104  100 -   0   0 231
            52  104 -   0   0 231
            236  140 -   0   0 231
            92  152 -   0 255   0
            168  152 -   0 255   0
            448  152 -  89 124 149
            52  156 -   0   0 231
            140  160 -   0 255   0
            112  168 -   0 255   0
            160  180 -   0 255   0
            136  192 -   0 255   0
            92  204 -   0 255   0
            168  204 -   0 255   0
            248  212 -   0   0 231
            124  228 -   0 255   0
            152  228 -   0 255   0
            52  248 -   0   0 231
            204  248 -   0   0 231
            300  300 -  89 124 149
            592  300 -  89 124 149
            144  436 -  89 124 149
            444  444 -  89 124 149
            152  584 -  89 124 149
            4  592 -  89 124 149
            300  592 -  89 124 149
            592  592 -  89 124 149
        "#,
    )?;

    from_main(move || {
        view.parent.set_size(280, 400);
    });

    check_colors(
        r#"
            432    4 -  89 124 149
            592    4 -  89 124 149
            52   52 -   0   0 231
            132   52 -   0   0 231
            228   52 -   0   0 231
            304   52 -   0   0 231
            180  112 -   0   0 231
            96  120 -   0   0 231
            468  136 -  89 124 149
            288  180 -   0   0 231
            208  184 -   0   0 231
            52  188 -   0   0 231
            168  252 -   0 255   0
            108  260 -   0 255   0
            592  268 -  89 124 149
            216  292 -   0 255   0
            332  300 -  89 124 149
            112  320 -   0 255   0
            168  320 -   0 255   0
            208  356 -   0 255   0
            140  364 -   0 255   0
            328  388 -   0   0 231
            216  400 -   0 255   0
            592  404 -  89 124 149
            108  408 -   0 255   0
            172  408 -   0 255   0
            464  444 -  89 124 149
            268  448 -   0   0 231
            4  592 -  89 124 149
            176  592 -  89 124 149
            368  592 -  89 124 149
            592  592 -  89 124 149
        "#,
    )?;

    Ok(())
}
