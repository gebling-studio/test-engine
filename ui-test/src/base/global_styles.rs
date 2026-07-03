use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{
        Anchor::{Top, X},
        BLUE, Button, Setup, Style, ViewData, ViewSubviews, view,
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

pub async fn test_global_styles() -> Result<()> {
    from_main(|| {
        GLOBAL_STYLE.apply_globally::<Button>();
    });

    UITest::start::<GlobalStyles>();

    check_colors(
        r"
             432    4 -  89 124 149
             168   52 - 175 129 115
              76   56 -   0   0 231
             136   56 -   0   0 231
             236   60 -   0   0 231
             108   64 -   0   0 231
             136   84 -   0   0 231
             168   84 -   6   3 231
             200   84 -   0   0 231
              80   88 - 175 129 115
             592  132 -  89 124 149
             128  144 - 175 129 115
              64  148 - 175 129 115
             192  152 -   0   0 231
             232  156 - 175 129 115
             160  164 - 175 129 115
             136  172 -   0   0 231
              92  176 -   0   0 231
             208  188 - 175 129 115
             192  232 - 175 129 115
              76  236 -   0   0 231
             128  236 - 175 129 115
             248  248 - 175 129 115
             108  264 -   0   0 231
             148  268 - 175 129 115
             232  268 -   6   3 231
              56  272 - 175 129 115
             208  276 - 175 129 115
             476  364 -  89 124 149
               4  524 -  89 124 149
             252  592 -  89 124 149
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
             136   72 - 255 255 255
             100   76 - 255 255 255
             156   76 - 255 255 255
             212   96 - 255 255 255
             436  124 -  89 124 149
             204  148 - 255 255 255
             100  156 - 255 255 255
             592  160 -  89 124 149
             136  164 - 255 255 255
             104  168 - 255 255 255
             156  168 - 255 255 255
             248  176 - 255 255 255
              52  188 - 255 255 255
             208  232 - 255 255 255
             100  248 - 255 255 255
             136  252 - 255 255 255
             104  256 - 255 255 255
             156  256 - 255 255 255
              52  276 - 255 255 255
             248  276 - 255 255 255
             524  300 -  89 124 149
             380  348 -  89 124 149
             128  448 -  89 124 149
             468  468 -  89 124 149
             300  520 -  89 124 149
               4  592 -  89 124 149
             168  592 -  89 124 149
             592  592 -  89 124 149
            ",
    )?;

    Ok(())
}
