use anyhow::Result;
use parking_lot::Mutex;
use test_engine::{
    refs::Weak,
    ui::{BLUE, Button, Setup, ViewData, ViewTransition, view},
    ui_test::{UITest, check_colors, inject_touches},
};

static ACTIONS: Mutex<Vec<&str>> = Mutex::new(vec![]);

#[view]
struct Transition {
    #[init]
    to_blue: Button,
}

impl Setup for Transition {
    fn setup(self: Weak<Self>) {
        self.to_blue.set_text("To Blue");
        self.to_blue.place().tl(20).size(200, 100);
        self.to_blue.add_transition::<Self, BlueView>();
    }
}

impl ViewTransition<BlueView> for Transition {
    fn transition_to(self: Weak<Self>, _target: &mut BlueView) {
        ACTIONS.lock().push("Transition callback");
    }
}

#[view]
struct BlueView {}

impl Setup for BlueView {
    fn setup(self: Weak<Self>) {
        self.set_color(BLUE);
        ACTIONS.lock().push("Blue setup");
    }
}

pub async fn test_transition() -> Result<()> {
    UITest::start::<Transition>();

    check_colors(
        r"
            452    4 -  89 124 149
            24   24 - 255 255 255
            152   24 - 255 255 255
            216   24 - 255 255 255
            76   60 -   0   0   0
            116   60 - 255 255 255
            120   60 - 255 255 255
            76   64 -   0   0   0
            176   64 - 255 255 255
            76   68 -   0   0   0
            92   68 - 255 255 255
            28   72 - 255 255 255
            76   72 -   0   0   0
            92   72 - 255 255 255
            116   72 - 255 255 255
            120   72 - 255 255 255
            76   76 -   0   0   0
            216   76 - 255 255 255
            24  116 - 255 255 255
            100  116 - 255 255 255
            148  116 - 255 255 255
            204  116 - 255 255 255
            592  196 -  89 124 149
            380  236 -  89 124 149
            220  284 -  89 124 149
            68  356 -  89 124 149
            516  396 -  89 124 149
            300  436 -  89 124 149
            4  592 -  89 124 149
            192  592 -  89 124 149
            404  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )?;

    inject_touches(
        "
            142  88   b
            142  87   e

        ",
    );

    check_colors(
        r"
            4    4 -   0   0 231
            444    4 -   0   0 231
            592    4 -   0   0 231
            296    8 -   0   0 231
            148   12 -   0   0 231
            228   84 -   0   0 231
            12  148 -   0   0 231
            444  152 -   0   0 231
            592  152 -   0   0 231
            156  156 -   0   0 231
            300  156 -   0   0 231
            84  228 -   0   0 231
            228  228 -   0   0 231
            372  228 -   0   0 231
            8  296 -   0   0 231
            448  296 -   0   0 231
            156  300 -   0   0 231
            300  300 -   0   0 231
            592  300 -   0   0 231
            228  372 -   0   0 231
            372  372 -   0   0 231
            516  372 -   0   0 231
            4  444 -   0   0 231
            152  444 -   0   0 231
            444  444 -   0   0 231
            296  448 -   0   0 231
            588  448 -   0   0 231
            448  588 -   0   0 231
            4  592 -   0   0 231
            152  592 -   0   0 231
            300  592 -   0   0 231
            592  592 -   0   0 231
        ",
    )?;

    assert_eq!(ACTIONS.lock().as_slice(), &["Transition callback", "Blue setup"]);

    ACTIONS.lock().clear();

    Ok(())
}
