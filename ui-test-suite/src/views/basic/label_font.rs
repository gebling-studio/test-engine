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
        default_font_colors()?;

        let original = AppRunner::take_screenshot()?;

        let custom_font = set_font_changes_one_label(view, &original)?;
        set_default_changes_default_label(&custom_font)?;
        reset_default_restores_original(&original)?;

        Ok(())
    }
}

fn default_font_colors() -> Result<()> {
    check_colors(
        r"
                592    4 -  89 124 149
                380   44 -   0   0   0
                88   48 -   0   0   0
                260   52 -   0   0   0
                416   52 -   0   0   0
                128   60 -  89 124 149
                196   60 -   0   0   0
                232   60 -  89 124 149
                316   64 -  89 124 149
                52   68 -   0   0   0
                288   68 -   0   0   0
                340   68 -  89 124 149
                380   68 -   0   0   0
                380  144 -   0   0   0
                88  148 -   0   0   0
                416  148 -   0   0   0
                224  152 -   0   0   0
                132  160 -  89 124 149
                188  160 -   0   0   0
                252  160 -  89 124 149
                268  164 -  89 124 149
                52  168 -   0   0   0
                340  168 -  89 124 149
                380  168 -   0   0   0
                312  172 -   0   0   0
                592  336 -  89 124 149
                412  344 -  89 124 149
                96  388 -  89 124 149
                300  484 -  89 124 149
                4  592 -  89 124 149
                172  592 -  89 124 149
                592  592 -  89 124 149
            ",
    )
}

fn set_font_changes_one_label(view: Weak<LabelFont>, original: &Screenshot) -> Result<Screenshot> {
    from_main(move || {
        view.custom_label.set_font(Font::get("OpenSans.ttf"));
    });

    check_colors(
        r"
                224   40 -   0   0   0
                380   44 -   0   0   0
                88   48 -   0   0   0
                288   52 -   0   0   0
                416   52 -   0   0   0
                128   60 -  89 124 149
                188   60 -   0   0   0
                252   60 -  89 124 149
                316   64 -  89 124 149
                52   68 -   0   0   0
                340   68 -  89 124 149
                156   72 -   0   0   0
                592  132 -  89 124 149
                80  156 -   0   0   0
                300  156 -   0   0   0
                216  160 -   0   0   0
                124  164 -  89 124 149
                260  164 -  89 124 149
                332  164 -  89 124 149
                156  172 -   0   0   0
                360  172 -  89 124 149
                404  172 -   0   0   0
                40  176 -   0   0   0
                244  176 -   0   0   0
                276  176 -   0   0   0
                592  324 -  89 124 149
                192  364 -  89 124 149
                4  412 -  89 124 149
                376  436 -  89 124 149
                160  592 -  89 124 149
                324  592 -  89 124 149
                592  592 -  89 124 149
            ",
    )?;

    let custom_font = AppRunner::take_screenshot()?;

    assert!(
        region(&custom_font, CUSTOM_FRAME) != region(original, CUSTOM_FRAME),
        "set_font did not change the label rendering"
    );
    assert!(
        region(&custom_font, DEFAULT_FRAME) == region(original, DEFAULT_FRAME),
        "set_font on one label changed another label"
    );

    Ok(custom_font)
}

fn set_default_changes_default_label(custom_font: &Screenshot) -> Result<()> {
    from_main(|| {
        Font::set_default(Font::get("DroidSansMono.ttf"));
    });

    check_colors(
        r"
                416   44 -   0   0   0
                124   52 -   0   0   0
                204   52 -   0   0   0
                288   52 -   0   0   0
                360   52 -   0   0   0
                80   60 -  89 124 149
                164   64 -   0   0   0
                244   64 -  89 124 149
                328   64 -  89 124 149
                268   68 -  89 124 149
                44   72 -   0   0   0
                592  124 -  89 124 149
                340  144 -   0   0   0
                24  156 -   0   0   0
                164  156 -   0   0   0
                244  160 -  89 124 149
                124  164 -  89 124 149
                328  168 -  89 124 149
                196  172 -   0   0   0
                360  172 -  89 124 149
                404  172 -   0   0   0
                44  176 -   0   0   0
                84  176 -   0   0   0
                276  176 -   0   0   0
                148  188 -   0   0   0
                592  340 -  89 124 149
                416  352 -  89 124 149
                180  360 -  89 124 149
                4  376 -  89 124 149
                300  488 -  89 124 149
                4  592 -  89 124 149
                592  592 -  89 124 149
            ",
    )?;

    let custom_default = AppRunner::take_screenshot()?;

    assert!(
        region(&custom_default, DEFAULT_FRAME) != region(custom_font, DEFAULT_FRAME),
        "set_default did not change the default label rendering"
    );
    assert!(
        region(&custom_default, CUSTOM_FRAME) == region(custom_font, CUSTOM_FRAME),
        "set_default changed a label with its own font"
    );

    Ok(())
}

fn reset_default_restores_original(original: &Screenshot) -> Result<()> {
    from_main(Font::reset_default);

    check_colors(
        r"
                224   40 -   0   0   0
                380   44 -   0   0   0
                88   48 -   0   0   0
                288   52 -   0   0   0
                416   52 -   0   0   0
                128   60 -  89 124 149
                188   60 -   0   0   0
                252   60 -  89 124 149
                316   64 -  89 124 149
                52   68 -   0   0   0
                340   68 -  89 124 149
                156   72 -   0   0   0
                592  132 -  89 124 149
                80  156 -   0   0   0
                300  156 -   0   0   0
                216  160 -   0   0   0
                124  164 -  89 124 149
                260  164 -  89 124 149
                332  164 -  89 124 149
                156  172 -   0   0   0
                360  172 -  89 124 149
                404  172 -   0   0   0
                40  176 -   0   0   0
                244  176 -   0   0   0
                276  176 -   0   0   0
                592  324 -  89 124 149
                192  364 -  89 124 149
                4  412 -  89 124 149
                376  436 -  89 124 149
                160  592 -  89 124 149
                324  592 -  89 124 149
                592  592 -  89 124 149
            ",
    )?;

    let restored = AppRunner::take_screenshot()?;

    assert!(
        region(&restored, DEFAULT_FRAME) == region(original, DEFAULT_FRAME),
        "reset_default did not restore the original rendering"
    );

    Ok(())
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
