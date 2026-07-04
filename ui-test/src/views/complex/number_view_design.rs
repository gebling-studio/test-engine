use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{BLUE, LIGHT_GRAY, NumberView, Setup, Style, ViewData, ViewSubviews, view},
    ui_test::{UITest, check_colors},
};

const STYLE: Style = Style::new(|view| {
    view.apply_if::<NumberView>(|mut num| {
        num.set_labels("+", "–")
            .set_text_color(LIGHT_GRAY)
            .set_text_size(80)
            .set_gradient(BLUE, (0, 150, 150))
            .set_corner_radius(20);
    })
});

#[view]
struct NumberViewDesign {
    #[init]
    view: NumberView,
}

impl Setup for NumberViewDesign {
    fn setup(self: Weak<Self>) {
        self.view.place().tl(200).size(100, 200);
    }
}

pub async fn test_number_view_design() -> Result<()> {
    from_main(|| {
        STYLE.apply_globally::<NumberView>();
    });

    let _view = UITest::start::<NumberViewDesign>();

    check_colors(
        r#"
            4    4 -  89 124 149
            592    4 -  89 124 149
            240  204 -   0  20 230
            208  208 -   0  30 228
            292  208 -   0  30 228
            236  228 -   0  59 222
            264  228 -   0  59 222
            204  240 -   0  70 218
            296  248 -   0  77 215
            232  252 - 231 231 231
            248  252 - 231 231 231
            260  260 -   0  86 211
            204  272 -   0  93 206
            236  284 -   0 101 202
            592  296 -  89 124 149
            304  300 -  89 124 149
            208  304 -   0 111 194
            260  304 -   0 111 194
            236  320 -   0 119 188
            204  336 -   0 126 181
            296  340 -   0 128 179
            232  348 - 231 231 231
            256  348 - 231 231 231
            268  348 - 231 231 231
            204  368 -   0 139 166
            240  372 -   0 140 164
            268  376 -   0 142 162
            292  392 -   0 147 154
            216  396 -   0 149 152
            244  396 -   0 149 152
            4  592 -  89 124 149
            592  592 -  89 124 149
        "#,
    )?;

    from_main(|| {
        STYLE.reset_global::<NumberView>();
    });

    Ok(())
}
