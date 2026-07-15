use anyhow::Result;
use test_engine::{
    gm::Apply,
    refs::Weak,
    ui::{Anchor::Top, Button, ORANGE, Setup, Style, ViewData, ViewSubviews, ui_test, view},
    ui_test::{UITest, check_colors},
};

const MENU_BUTTON: Style = Style::new(|view| {
    view.set_color((75, 129, 244));
    view.set_corner_radius(20);
    view.place().size(280, 100).l(50);

    if let Some(view) = view.downcast_view::<Button>() {
        view.set_text_color(ORANGE);
        view.set_text_size(64);
    }
});

#[view]
struct Styles {
    #[init]
    button_1: Button,
    button_2: Button,
    button_3: Button,
}

impl Setup for Styles {
    fn setup(self: Weak<Self>) {
        [self.button_1, self.button_2, self.button_3].apply(|button| {
            button.apply_style(MENU_BUTTON);
        });

        self.button_1.set_text("Button 1").place().t(50);

        self.button_2.set_text("Button 2");
        self.button_2.place().anchor(Top, self.button_1, 40);

        self.button_3.set_text("Button 3");
        self.button_3.place().anchor(Top, self.button_2, 40);
    }
}

#[ui_test]
pub fn test_styles() -> Result<()> {
    UITest::start::<Styles>();

    check_colors(
        r"
            592    4 -  89 124 149
            92   80 -  75 129 244
            248   92 - 255 203   0
            92  104 -  75 129 244
            188  104 - 255 203   0
            292  112 - 255 203   0
            128  116 - 254 203  19
            184  192 -  75 129 244
            324  200 -  75 129 244
            92  220 -  75 129 244
            156  224 - 255 203   0
            256  236 -  75 129 244
            296  236 - 254 203  19
            92  244 -  75 129 244
            128  256 - 254 203  19
            200  256 - 254 203  19
            324  280 -  75 129 244
            588  300 -  89 124 149
            156  356 - 255 203   0
            92  360 -  75 129 244
            300  360 - 254 203  19
            88  368 -  75 129 244
            248  372 - 255 203   0
            92  384 -  75 129 244
            128  396 - 254 203  19
            204  396 - 254 203  19
            52  412 -  75 129 244
            288  428 -  75 129 244
            468  444 -  89 124 149
            148  592 -  89 124 149
            352  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )?;

    Ok(())
}
