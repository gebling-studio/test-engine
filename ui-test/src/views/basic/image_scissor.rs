use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{ImageMode, ImageView, Setup, ViewFrame, ViewTest, view_test},
    ui_test::check_colors,
};

#[view_test]
struct ImageScissor {
    #[init]
    image: ImageView,
}

impl Setup for ImageScissor {
    fn setup(mut self: Weak<Self>) {
        self.image.set_image("cat.png").set_frame((20, 20, 100, 400));
        self.image.mode = ImageMode::AspectFill;
    }
}

impl ViewTest for ImageScissor {
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

        from_main(move || {
            view.image.set_position((-20, -40));
        });

        check_colors(
            r"
                      14   74 -  89 124 149
                      23   32 - 232 202 191
                      28    9 - 229 199 189
                     122   12 - 197 161 137
                     160   16 - 175 142 127
                     192   20 -  89 124 149
                     224   85 -  89 124 149
                ",
        )?;

        from_main(move || {
            view.image.set_size(200, 1000);
        });

        check_colors(
            r"
                     217   73 -  89 124 149
                     229  301 -  89 124 149
                     240  493 -  89 124 149
                     113  528 - 178 141 112
                      30  436 - 173 141 116
                     174  421 -  14   4   3
                      16  368 -  58  55  34
                      42  206 - 207 177 155
                      58   79 - 223 181 182
                     126   79 - 227 178 181
                ",
        )?;

        from_main(move || {
            view.image.set_frame((20, 500, 200, 200));
        });

        check_colors(
            r"
                      10  546 -  89 124 149
                      36  512 - 231 191 192
                      56  488 -  89 124 149
                     105  488 -  89 124 149
                     130  510 - 225 179 181
                     181  538 - 186 134 113
                     234  566 -  89 124 149
                     146  589 -  14   4   3
                ",
        )?;

        // test_engine::ui_test::record_ui_test();

        Ok(())
    }
}
