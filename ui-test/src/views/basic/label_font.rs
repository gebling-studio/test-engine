use anyhow::Result;
use test_engine::{
    AppRunner,
    dispatch::from_main,
    refs::{Weak, manage::DataManager},
    ui::{Font, Label, Screenshot, Setup, U8Color, ViewFrame, ViewTest, view_test},
    ui_test::check_colors,
};

const TEXT: &str = "Grumpy wizards 123";

const DEFAULT_FRAME: (u32, u32, u32, u32) = (20, 20, 400, 80);
const CUSTOM_FRAME: (u32, u32, u32, u32) = (20, 120, 400, 80);

#[view_test]
struct LabelFont {
    #[init]
    default_label: Label,
    custom_label:  Label,
}

impl Setup for LabelFont {
    fn setup(self: Weak<Self>) {
        self.default_label.set_frame(DEFAULT_FRAME);
        self.default_label.set_text(TEXT).set_text_size(50);

        self.custom_label.set_frame(CUSTOM_FRAME);
        self.custom_label.set_text(TEXT).set_text_size(50);
    }
}

impl ViewTest for LabelFont {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        check_colors(
            r"
             380   44 -   2   2   4
              80   52 -   0   0   0
             416   52 -   2   2   4
             232   56 -  89 124 149
             128   60 -  89 124 149
             264   60 -   2   2   4
             316   60 -  89 124 149
              52   68 -   0   0   0
             288   68 -   0   0   0
             340   68 -  89 124 149
             156   72 -   0   0   0
             592  100 -  89 124 149
             340  148 -   2   2   4
             416  148 -   2   2   4
              80  152 -   0   0   0
             224  152 -   0   0   0
             268  152 -  89 124 149
             132  160 -  89 124 149
             252  160 -  89 124 149
              52  168 -   0   0   0
             340  168 -  89 124 149
             380  168 -   2   2   4
             156  172 -   0   0   0
             264  172 -   2   2   4
             312  172 -   2   2   4
             592  336 -  89 124 149
             412  344 -  89 124 149
              96  388 -  89 124 149
             300  484 -  89 124 149
               4  592 -  89 124 149
             172  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        let original = AppRunner::take_screenshot()?;

        from_main(move || {
            view.custom_label.set_font(Font::get("OpenSans.ttf"));
        });

        check_colors(
            r"
             224   40 -   0   0   0
             380   44 -   2   2   4
              80   52 -   0   0   0
             416   52 -   2   2   4
             128   60 -  89 124 149
             252   60 -  89 124 149
             316   60 -  89 124 149
              52   68 -   0   0   0
             340   68 -  89 124 149
             156   72 -   0   0   0
             592  132 -  89 124 149
             100  156 -   2   2   4
             220  156 -   0   0   0
             300  156 -   2   2   4
              68  160 -   2   2   4
             128  164 -  89 124 149
             244  164 -  89 124 149
             332  164 -  89 124 149
             360  172 -  89 124 149
             404  172 -   0   0   0
              40  176 -   0   0   0
             156  176 -   2   2   4
             192  176 -   0   0   0
             276  176 -   2   2   4
             116  188 -   0   0   0
             592  324 -  89 124 149
             192  372 -  89 124 149
               4  412 -  89 124 149
             376  436 -  89 124 149
             160  592 -  89 124 149
             324  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        let custom_font = AppRunner::take_screenshot()?;

        assert!(
            region(&custom_font, CUSTOM_FRAME) != region(&original, CUSTOM_FRAME),
            "set_font did not change the label rendering"
        );
        assert!(
            region(&custom_font, DEFAULT_FRAME) == region(&original, DEFAULT_FRAME),
            "set_font on one label changed another label"
        );

        from_main(|| {
            Font::set_default(Font::get("DroidSansMono.ttf"));
        });

        check_colors(
            r"
             416   44 -   0   0   0
             288   52 -   2   2   4
             360   52 -   0   0   0
              80   60 -  89 124 149
             120   64 -   2   2   4
             196   64 -  89 124 149
             244   64 -  89 124 149
             328   64 -  89 124 149
             268   68 -  89 124 149
              44   72 -   0   0   0
             592  124 -  89 124 149
             340  144 -   0   0   0
              24  156 -   2   2   4
             164  156 -   0   0   0
             300  156 -   2   2   4
             244  160 -  89 124 149
             124  164 -  89 124 149
             208  168 -   0   0   0
             328  168 -  89 124 149
              84  172 -   2   2   4
             404  172 -   0   0   0
              44  176 -   0   0   0
             276  176 -   2   2   4
             360  176 -   2   2   4
             144  188 -   2   2   4
             592  340 -  89 124 149
             416  352 -  89 124 149
             188  356 -  89 124 149
              12  376 -  89 124 149
             300  488 -  89 124 149
               4  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        let custom_default = AppRunner::take_screenshot()?;

        assert!(
            region(&custom_default, DEFAULT_FRAME) != region(&custom_font, DEFAULT_FRAME),
            "set_default did not change the default label rendering"
        );
        assert!(
            region(&custom_default, CUSTOM_FRAME) == region(&custom_font, CUSTOM_FRAME),
            "set_default changed a label with its own font"
        );

        from_main(Font::reset_default);

        check_colors(
            r"
             224   40 -   0   0   0
             380   44 -   2   2   4
              80   52 -   0   0   0
             416   52 -   2   2   4
             128   60 -  89 124 149
             252   60 -  89 124 149
             316   60 -  89 124 149
              52   68 -   0   0   0
             340   68 -  89 124 149
             156   72 -   0   0   0
             592  132 -  89 124 149
             100  156 -   2   2   4
             220  156 -   0   0   0
             300  156 -   2   2   4
              68  160 -   2   2   4
             128  164 -  89 124 149
             244  164 -  89 124 149
             332  164 -  89 124 149
             360  172 -  89 124 149
             404  172 -   0   0   0
              40  176 -   0   0   0
             156  176 -   2   2   4
             192  176 -   0   0   0
             276  176 -   2   2   4
             116  188 -   0   0   0
             592  324 -  89 124 149
             192  372 -  89 124 149
               4  412 -  89 124 149
             376  436 -  89 124 149
             160  592 -  89 124 149
             324  592 -  89 124 149
             592  592 -  89 124 149
            ",
        )?;

        let restored = AppRunner::take_screenshot()?;

        assert!(
            region(&restored, DEFAULT_FRAME) == region(&original, DEFAULT_FRAME),
            "reset_default did not restore the original rendering"
        );

        Ok(())
    }
}

fn region(shot: &Screenshot, frame: (u32, u32, u32, u32)) -> Vec<U8Color> {
    let (x, y, width, height) = frame;
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for py in y..y + height {
        for px in x..x + width {
            pixels.push(shot.get_pixel((px, py)));
        }
    }

    pixels
}

pub async fn test_label_font() -> Result<()> {
    run_ui_test()
}
