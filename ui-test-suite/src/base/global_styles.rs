use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{
        Anchor::{Top, X},
        BLUE, Button, Setup, Style, ViewData, ViewSubviews, ViewTest, view,
    },
    ui_test::{UITest, check_colors},
};

const GLOBAL_STYLE: Style = Style::new(|view| {
    view.set_color((175, 129, 115));
    view.set_corner_radius(20);

    if let Some(view) = view.downcast_view::<Button>() {
        view.set_text_color(BLUE);
        view.set_text_size(55);
    }
});

#[view]
struct GlobalStyles {
    #[init]
    button_1: Button,
    button_2: Button,
    button_3: Button,
}

impl Setup for GlobalStyles {
    fn setup(self: Weak<Self>) {
        self.button_1.set_text("Button 1").place().t(50).l(50).size(200, 50);

        self.button_2.set_text("Button 2");
        self.button_2
            .place()
            .anchor(Top, self.button_1, 40)
            .same_size(self.button_1)
            .same([X], self.button_1);

        self.button_3.set_text("Button 3");
        self.button_3
            .place()
            .anchor(Top, self.button_2, 40)
            .same_size(self.button_1)
            .same([X], self.button_1);
    }
}

impl ViewTest for GlobalStyles {
    fn before_start() {
        from_main(|| {
            GLOBAL_STYLE.apply_globally::<Button>();
        });
    }

    fn perform_test(_view: Weak<Self>) -> Result<()> {
        check_colors(
            r"
            452    4 -  89 124 149
            164   52 - 175 129 115
            212   52 - 175 129 115
            76   56 -   0   0 231
            128   56 - 175 129 115
            248   80 - 175 129 115
            80   88 - 175 129 115
            160   88 -   0   0 231
            120   92 - 175 129 115
            196   96 - 175 129 115
            132  148 - 175 129 115
            228  148 -   2   2 231
            168  156 -   2   2 231
            64  164 - 175 129 115
            248  164 - 175 129 115
            592  168 -  89 124 149
            108  176 -   2   2 231
            200  176 -   0   0 231
            148  180 - 175 129 115
            56  232 -   0   0 231
            136  236 -   0   0 231
            244  236 -   0   0 231
            108  244 -   0   0 231
            200  248 -   0   0 231
            96  268 -   2   2 231
            140  268 -   0   0 231
            172  268 - 175 129 115
            220  276 - 175 129 115
            480  380 -  89 124 149
            300  536 -  89 124 149
            4  592 -  89 124 149
            592  592 -  89 124 149
        ",
        )?;

        from_main(|| {
            GLOBAL_STYLE.reset_global::<Button>();
        });

        UITest::reload::<GlobalStyles>();

        check_colors(
            r"
            592    4 -  89 124 149
            52   52 - 255 255 255
            196   52 - 255 255 255
            248   52 - 255 255 255
            100   68 - 255 255 255
            136   72 - 255 255 255
            156   76 - 255 255 255
            96   96 - 255 255 255
            212   96 - 255 255 255
            436  116 -  89 124 149
            52  144 - 255 255 255
            100  156 - 255 255 255
            248  156 - 255 255 255
            136  164 - 255 255 255
            156  164 - 255 255 255
            592  164 -  89 124 149
            100  168 - 255 255 255
            204  188 - 255 255 255
            100  248 - 255 255 255
            200  248 - 255 255 255
            156  252 - 255 255 255
            136  256 - 255 255 255
            52  276 - 255 255 255
            248  276 - 255 255 255
            504  300 -  89 124 149
            368  360 -  89 124 149
            128  448 -  89 124 149
            464  472 -  89 124 149
            300  532 -  89 124 149
            4  592 -  89 124 149
            164  592 -  89 124 149
            592  592 -  89 124 149
        ",
        )?;

        Ok(())
    }
}
