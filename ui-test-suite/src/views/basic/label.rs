use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{
        Anchor, BLUE, LIGHTER_GRAY, Label, NumberView, Setup, TextAlignment, ViewData, ViewSubviews,
        ViewTest, WHITE, view,
    },
    ui_test::{helpers::check_colors, inject_touches},
};

#[view]
struct LabelSettings {
    #[init]
    label:          Label,
    text_size_view: NumberView,
}

impl Setup for LabelSettings {
    fn setup(self: Weak<Self>) {
        self.label.set_text("ßšėčыў").set_color(WHITE);
        self.label.place().size(280, 280).tl(80);

        self.text_size_view
            .place()
            .size(50, 50)
            .t(300)
            .anchor(Anchor::Right, self.label, 10);
        self.text_size_view.set_value(32.0).set_step(5.0);

        self.text_size_view.on_change(move |size| {
            self.label.set_text_size(size);
        });
    }
}

impl ViewTest for LabelSettings {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        initial_label_colors()?;
        stepper_changes_text_size()?;
        blue_text_color(view)?;
        left_right_aligned_labels(view)?;

        Ok(())
    }
}

fn initial_label_colors() -> Result<()> {
    check_colors(
        r"
            4    4 -  89 124 149
            592    4 -  89 124 149
            120   84 - 255 255 255
            356   84 - 255 255 255
            240  100 - 255 255 255
            84  176 - 255 255 255
            356  200 - 255 255 255
            176  212 - 255 255 255
            192  216 - 255 255 255
            228  216 - 255 255 255
            232  220 - 255 255 255
            176  224 - 255 255 255
            192  224 - 255 255 255
            244  224 - 255 255 255
            592  260 -  89 124 149
            24  304 -   0 150 230
            68  304 -   0 150 230
            48  312 - 255 255 255
            24  324 -   0 150 230
            64  324 -   0 150 230
            52  328 -   0 150 230
            40  336 - 255 255 255
            68  344 -   0 150 230
            24  348 -   0 150 230
            52  348 -   0 150 230
            220  356 - 255 255 255
            356  356 - 255 255 255
            560  424 -  89 124 149
            424  524 -  89 124 149
            4  588 -  89 124 149
            256  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )
}

fn stepper_changes_text_size() -> Result<()> {
    inject_touches(
        "
            39   305  b
            39   305  e
            41   301  b
            42   301  e
            42   301  b
            42   301  e
            42   301  b
            42   301  e
            42   301  b
            42   301  e
            42   301  b
            42   301  e
            42   301  b
            42   301  e
            42   301  b
            42   301  e
            42   300  b
            42   300  e
            42   300  b
            42   300  e
            42   300  b
            42   300  e
            42   300  b
            42   300  e
            42   300  b
            42   300  e
            42   300  b
            42   300  e
            42   300  b
            42   300  e
            44   325  b
            44   325  e
            44   325  b
            44   325  e
            44   325  b
            44   325  e
            44   325  b
            44   325  e
            43   325  b
            43   325  e
            43   325  b
            43   325  e
            42   306  b
            43   308  e

        ",
    );

    check_colors(
        r"
            592    4 -  89 124 149
            120   84 - 255 255 255
            240   84 - 255 255 255
            356   84 - 255 255 255
            348  184 -   0   0   0
            96  188 -   0   0   0
            196  188 -   0   0   0
            236  200 -   1   1   1
            148  208 - 255 255 255
            196  208 - 255 255 255
            116  212 -   0   0   0
            184  220 -   1   1   1
            328  220 -   1   1   1
            280  224 - 255 255 255
            152  232 - 255 255 255
            208  232 -   0   0   0
            104  240 -   0   0   0
            240  240 -   0   0   0
            324  256 -   1   1   1
            24  304 -   0 150 230
            68  304 -   0 150 230
            48  312 - 255 255 255
            592  316 -  89 124 149
            24  328 -   0 150 230
            68  332 -   0 150 230
            44  340 - 255 255 255
            24  348 -   0 150 230
            224  356 - 255 255 255
            356  356 - 255 255 255
            4  592 -  89 124 149
            260  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )
}

fn blue_text_color(view: Weak<LabelSettings>) -> Result<()> {
    from_main(move || {
        view.label.set_text_color(BLUE);
    });

    check_colors(
        r"
            592    4 -  89 124 149
            120   84 - 255 255 255
            252   84 - 255 255 255
            356   84 - 255 255 255
            232  184 -   0   0 231
            348  184 -   0   0 231
            96  188 -   0   0 231
            196  188 -   0   0 231
            308  204 -   0   0 231
            148  208 - 255 255 255
            116  212 -   0   0 231
            184  220 -   6   6 231
            280  224 - 255 255 255
            344  224 -   6   6 231
            152  232 - 255 255 255
            196  232 - 255 255 255
            104  240 -   0   0 231
            236  240 -   0   0 231
            324  256 -   6   6 231
            24  304 -   0 150 230
            68  304 -   0 150 230
            48  312 - 255 255 255
            592  316 -  89 124 149
            24  328 -   0 150 230
            68  332 -   0 150 230
            44  340 - 255 255 255
            24  348 -   0 150 230
            224  356 - 255 255 255
            356  356 - 255 255 255
            4  592 -  89 124 149
            260  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )
}

fn left_right_aligned_labels(view: Weak<LabelSettings>) -> Result<()> {
    from_main(move || {
        view.label.set_text_size(28);
        view.add_view::<Label>()
            .set_text("Left Left")
            .set_alignment(TextAlignment::Left)
            .set_color(LIGHTER_GRAY)
            .place()
            .tl(60)
            .w(200)
            .h(60);
        view.add_view::<Label>()
            .set_text("Right")
            .set_alignment(TextAlignment::Right)
            .set_color(LIGHTER_GRAY)
            .place()
            .l(60)
            .w(200)
            .t(280)
            .h(60);
    });

    check_colors(
        r"
            592    4 -  89 124 149
            64   64 - 243 243 243
            212   64 - 243 243 243
            356   84 - 255 255 255
            504  132 -  89 124 149
            276  136 - 255 255 255
            84  168 - 255 255 255
            180  212 - 255 255 255
            196  216 - 255 255 255
            232  220 - 255 255 255
            356  228 - 255 255 255
            592  260 -  89 124 149
            296  292 - 255 255 255
            180  300 - 243 243 243
            24  304 -   0 150 230
            68  304 -   0 150 230
            36  308 -   0 150 230
            208  308 - 243 243 243
            48  312 - 255 255 255
            208  320 - 243 243 243
            64  324 -   0 150 230
            32  328 -   0 150 230
            52  328 -   0 150 230
            44  340 - 255 255 255
            64  344 -   0 150 230
            24  348 -   0 150 230
            356  356 - 255 255 255
            520  428 -  89 124 149
            168  476 -  89 124 149
            44  592 -  89 124 149
            292  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )
}
