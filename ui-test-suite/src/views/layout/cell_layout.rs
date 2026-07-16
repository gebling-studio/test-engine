use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{
        Anchor::{Right, Top},
        Button, Container, GREEN, Label, Setup, TURQUOISE, UIImages, ViewData, ViewSubviews, ViewTest, WHITE,
        YELLOW, view,
    },
    ui_test::check_colors,
};

#[view]
struct CellLayout {
    title: Weak<Container>,
    table: Weak<Container>,

    #[init]
    delete: Button,
    label:  Label,

    container: Container,
}

impl Setup for CellLayout {
    fn setup(mut self: Weak<Self>) {
        self.delete.place().t(100).l(400).size(50, 50);
        self.delete.set_image(UIImages::delete());

        self.label
            .set_color(WHITE)
            .place()
            .l(50)
            .t(100)
            .h(200)
            .anchor(Right, self.delete, 10);

        self.container.set_color(TURQUOISE);
        self.container.place().t(400).l(10).size(200, 160);

        self.title = self.container.add_view();
        self.table = self.container.add_view();

        self.title.place().lrt(10).h(50);
        self.title.set_color(GREEN);

        self.table.place().anchor(Top, self.title, 10).lrb(10);
        self.table.set_color(YELLOW);
    }
}

impl ViewTest for CellLayout {
    fn perform_test(view: Weak<Self>) -> anyhow::Result<()> {
        check_initial_layout()?;
        check_reapplied_placement(view)?;

        Ok(())
    }
}

fn check_initial_layout() -> anyhow::Result<()> {
    check_colors(
        r"
            172    4 -  89 124 149
            424  104 - 255 255 255
            404  108 -  30  96 139
            416  108 -  30  96 139
            436  108 -  30  63 107
            428  112 -  30  63 107
            436  112 -  30  63 107
            444  112 -  30  63 107
            404  120 - 255 255 255
            416  120 -  80 197 255
            432  120 -  72 137 255
            256  124 - 255 255 255
            404  124 -  30  96 139
            444  124 -  30  63 107
            428  128 -  72 137 255
            428  140 -  72 137 255
            444  140 - 255 255 255
            412  144 -  80 197 255
            432  144 -  72 137 255
            432  148 -  72 137 255
            52  228 - 255 255 255
            252  272 - 255 255 255
            24  412 -   0 255   0
            120  412 -   0 255   0
            184  412 -   0 255   0
            196  472 - 255 255   0
            108  484 - 255 255   0
            36  504 - 255 255   0
            116  556 -   0 255 255
            208  556 -   0 255 255
            4  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )?;

    check_colors(
        r"
            172    4 -  89 124 149
            424  104 - 255 255 255
            404  108 -  30  96 139
            416  108 -  30  96 139
            436  108 -  30  63 107
            428  112 -  30  63 107
            436  112 -  30  63 107
            444  112 -  30  63 107
            404  120 - 255 255 255
            416  120 -  80 197 255
            432  120 -  72 137 255
            256  124 - 255 255 255
            404  124 -  30  96 139
            444  124 -  30  63 107
            428  128 -  72 137 255
            428  140 -  72 137 255
            444  140 - 255 255 255
            412  144 -  80 197 255
            432  144 -  72 137 255
            432  148 -  72 137 255
            52  228 - 255 255 255
            252  272 - 255 255 255
            24  412 -   0 255   0
            120  412 -   0 255   0
            184  412 -   0 255   0
            196  472 - 255 255   0
            108  484 - 255 255   0
            36  504 - 255 255   0
            116  556 -   0 255 255
            208  556 -   0 255 255
            4  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )?;

    Ok(())
}

fn check_reapplied_placement(view: Weak<CellLayout>) -> anyhow::Result<()> {
    from_main(move || {
        view.title.place().clear().h(50).lrt(10);

        view.table.place().clear().lrb(10).anchor(Top, view.title, 10);
    });

    check_colors(
        r"
            172    4 -  89 124 149
            424  104 - 255 255 255
            404  108 -  30  96 139
            416  108 -  30  96 139
            436  108 -  30  63 107
            428  112 -  30  63 107
            436  112 -  30  63 107
            444  112 -  30  63 107
            404  120 - 255 255 255
            416  120 -  80 197 255
            432  120 -  72 137 255
            256  124 - 255 255 255
            404  124 -  30  96 139
            444  124 -  30  63 107
            428  128 -  72 137 255
            428  140 -  72 137 255
            444  140 - 255 255 255
            412  144 -  80 197 255
            432  144 -  72 137 255
            432  148 -  72 137 255
            52  228 - 255 255 255
            252  272 - 255 255 255
            24  412 -   0 255   0
            120  412 -   0 255   0
            184  412 -   0 255   0
            196  472 - 255 255   0
            108  484 - 255 255   0
            36  504 - 255 255   0
            116  556 -   0 255 255
            208  556 -   0 255 255
            4  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )?;

    Ok(())
}
