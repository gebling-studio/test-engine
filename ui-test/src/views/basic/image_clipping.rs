use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{ImageMode, ImageView, Setup, ViewFrame, ViewTest, view_test},
    ui_test::check_colors,
};

#[view_test]
struct ImageClipping {
    #[init]
    image: ImageView,
}

impl Setup for ImageClipping {
    fn setup(mut self: Weak<Self>) {
        self.image.set_image("cat.png").set_frame((20, 20, 100, 400));
        self.image.mode = ImageMode::AspectFill;
    }
}

impl ViewTest for ImageClipping {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        check_colors(
            r"
                       7  192 -  89 124 149
                      29  198 - 201 175 150
                      58  198 - 209 181 157
                      92  198 -  50  25  18
                     111  206 -  59  52  42
                     129  206 -  89 124 149
                      70  444 -  89 124 149
                      70  429 -  89 124 149
                      70  385 - 204 167 148
                      70  238 - 191 152 121
                      72  137 - 205 179 156
                      73   34 - 223 183 184
                      74   14 -  89 124 149
                ",
        )?;

        from_main(move || {
            view.image.set_size(200, 50);
        });

        check_colors(
            r"
                       9   36 -  89 124 149
                      30   36 - 230 184 187
                     186   49 - 194 149 143
                     243   49 -  89 124 149
                     106   89 -  89 124 149
                     106   59 - 172 126 103
                     105   14 -  89 124 149
                ",
        )?;

        // test_engine::ui_test::record_ui_test();

        Ok(())
    }
}
