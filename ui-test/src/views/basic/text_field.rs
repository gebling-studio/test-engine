use anyhow::Result;
use test_engine::{
    AppRunner,
    dispatch::from_main,
    refs::Weak,
    ui::{Anchor, Setup, ViewData, view},
    ui_test::{UITest, helpers::check_colors, inject_keys, inject_touches},
};

#[view]
struct TextField {
    #[init]
    field:      test_engine::ui::TextField,
    smol_field: test_engine::ui::TextField,
}

impl Setup for TextField {
    fn setup(self: Weak<Self>) {
        self.field.place().size(600, 200).center();
        self.smol_field
            .place()
            .size(200, 50)
            .center_x()
            .anchor(Anchor::Bot, self.field, 40);
    }
}

pub async fn test_text_field() -> Result<()> {
    let view = UITest::start::<TextField>();

    AppRunner::set_window_size((800, 800));

    inject_touches(
        r"
            389  576  b
            389  576  e
            399  292  b
            399  292  e
            427  147  b
            427  147  e
            391  237  b
            391  235  e
    ",
    );

    inject_keys("HELLOY");

    inject_touches(
        r"
            452  442  b
    ",
    );

    inject_keys("text");

    inject_touches(
        r"
            10  10  b
    ",
    );

    check_colors(
        r#"
            4    4 -  89 124 149
            244    4 -  89 124 149
            552    4 -  89 124 149
            792    4 -  89 124 149
            4  196 -  89 124 149
            344  220 - 255 255 255
            352  220 - 255 255 255
            440  220 - 255 255 255
            456  220 - 255 255 255
            372  228 - 255 255 255
            424  228 - 255 255 255
            432  232 - 255 255 255
            376  236 - 255 255 255
            424  236 - 255 255 255
            160  304 - 255 255 255
            640  304 - 255 255 255
            284  332 - 255 255 255
            516  332 - 255 255 255
            392  396 - 255 255 255
            408  400 -   0   0   0
            392  404 - 255 255 255
            184  412 - 255 255 255
            616  416 - 255 255 255
            264  492 - 255 255 255
            104  496 - 255 255 255
            536  496 - 255 255 255
            696  496 - 255 255 255
            200  700 -  89 124 149
            596  704 -  89 124 149
            4  792 -  89 124 149
            400  792 -  89 124 149
            792  792 -  89 124 149
        "#,
    )?;

    from_main(move || {
        view.field.set_text_size(140);
        view.field.clear();
    });

    inject_touches(
        r"
            452  442  b
    ",
    );

    inject_keys("ŽĖЎФЪ");

    check_colors(
        r#"
            4    4 -  89 124 149
            792    4 -  89 124 149
            304  212 - 255 255 255
            400  212 - 255 255 255
            496  212 - 255 255 255
            344  220 - 255 255 255
            440  220 - 255 255 255
            456  220 - 255 255 255
            432  232 - 255 255 255
            376  236 - 255 255 255
            424  236 - 255 255 255
            316  256 - 255 255 255
            476  256 - 255 255 255
            696  304 - 188 188 188
            280  316 -   0   0   0
            184  320 -   1   1   1
            200  320 -   1   1   1
            356  320 -   1   1   1
            392  320 -   1   1   1
            284  324 -   0   0   0
            584  340 -   0   0   0
            492  348 -   1   1   1
            376  420 -   1   1   1
            696  428 - 188 188 188
            212  432 -   0   0   0
            476  436 -   1   1   1
            104  496 - 188 188 188
            312  496 - 188 188 188
            588  496 - 188 188 188
            4  792 -  89 124 149
            376  792 -  89 124 149
            792  792 -  89 124 149
        "#,
    )?;

    Ok(())
}
