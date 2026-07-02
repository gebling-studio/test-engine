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
              52   40 -   0   0   0
             388   48 -   2   2   4
             116   52 -   0   0   0
             144   52 -   2   2   4
              84   56 -   0   0   0
             236   56 -   0   0   0
             176   60 -  89 124 149
             284   60 -  89 124 149
             300   60 -  89 124 149
             320   64 -  89 124 149
             356   64 -  89 124 149
              60   72 -   2   2   4
             592  104 -  89 124 149
             152  148 -   0   0   0
             116  152 -   0   0   0
             316  152 -  89 124 149
             236  156 -   0   0   0
              44  160 -   2   2   4
             300  160 -  89 124 149
             364  160 -  89 124 149
             396  164 -   0   0   0
              84  168 -   0   0   0
             272  168 -   0   0   0
             180  172 -   0   0   0
             312  172 -   0   0   0
             592  316 -  89 124 149
             412  340 -  89 124 149
              96  384 -  89 124 149
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
             592    4 -  89 124 149
              52   40 -   0   0   0
             388   48 -   2   2   4
             116   52 -   0   0   0
             144   52 -   2   2   4
             236   56 -   0   0   0
             176   60 -  89 124 149
             284   60 -  89 124 149
             300   60 -  89 124 149
             320   64 -  89 124 149
              84   68 -   0   0   0
             204   72 -   0   0   0
             360   72 -   2   2   4
              40  156 -   0   0   0
             228  156 -   2   2   4
             260  160 -  89 124 149
             364  160 -  89 124 149
              96  164 -   0   0   0
             152  164 -  89 124 149
             320  168 -  89 124 149
             400  168 -   2   2   4
              68  172 -   0   0   0
             120  172 -   2   2   4
             200  172 -   2   2   4
             300  172 -   2   2   4
             592  336 -  89 124 149
             416  348 -  89 124 149
              96  388 -  89 124 149
             300  484 -  89 124 149
               4  592 -  89 124 149
             172  592 -  89 124 149
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
             592    4 -  89 124 149
             120   56 -   0   0   0
              48   60 -  89 124 149
             252   60 -  89 124 149
             360   60 -  89 124 149
             380   64 -  89 124 149
              96   68 -   0   0   0
             148   68 -   2   2   4
             308   68 -  89 124 149
             308  148 -   0   0   0
             128  156 -   0   0   0
             228  156 -   2   2   4
             176  160 -   2   2   4
             268  160 -   2   2   4
             364  160 -  89 124 149
              40  164 -   0   0   0
              96  164 -   0   0   0
             152  164 -  89 124 149
             248  168 -  89 124 149
             328  168 -   2   2   4
             400  168 -   2   2   4
              68  172 -   0   0   0
             200  172 -   2   2   4
             300  172 -   2   2   4
             144  180 -   2   2   4
             592  336 -  89 124 149
             416  348 -  89 124 149
              92  392 -  89 124 149
             300  484 -  89 124 149
               4  592 -  89 124 149
             172  592 -  89 124 149
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
             592    4 -  89 124 149
              52   40 -   0   0   0
             388   48 -   2   2   4
             116   52 -   0   0   0
             144   52 -   2   2   4
             236   56 -   0   0   0
             176   60 -  89 124 149
             284   60 -  89 124 149
             300   60 -  89 124 149
             320   64 -  89 124 149
              84   68 -   0   0   0
             204   72 -   0   0   0
             360   72 -   2   2   4
              40  156 -   0   0   0
             228  156 -   2   2   4
             260  160 -  89 124 149
             364  160 -  89 124 149
              96  164 -   0   0   0
             152  164 -  89 124 149
             320  168 -  89 124 149
             400  168 -   2   2   4
              68  172 -   0   0   0
             120  172 -   2   2   4
             200  172 -   2   2   4
             300  172 -   2   2   4
             592  336 -  89 124 149
             416  348 -  89 124 149
              96  388 -  89 124 149
             300  484 -  89 124 149
               4  592 -  89 124 149
             172  592 -  89 124 149
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
