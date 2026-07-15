use crate::views::basic::{
    background::test_background, button::test_button, checkbox::test_checkbox,
    corner_radii::test_corner_radii, custom_text_field::test_custom_text_field, font_zoo::test_font_zoo,
    gradient::test_gradient, hover::test_hover, inject_touch::test_inject_touch, label::test_label,
    label_fit_text::test_label_fit_text, label_font::test_label_font, label_image::test_label_image,
    label_measure::test_label_measure, letter_spacing::test_letter_spacing, multiline_label::test_multiline,
    nine_segment::test_nine_segment, shadow::test_shadow, slider::test_slider, switch::test_switch,
    text_field::test_text_field, theme_switch::test_theme_switch,
};

mod background;
mod button;
mod checkbox;
mod corner_radii;
mod custom_text_field;
mod font_zoo;
mod gradient;
mod hover;
mod image_scissor;
mod inject_touch;
mod label;
mod label_fit_text;
mod label_font;
mod label_image;
mod label_measure;
mod letter_spacing;
mod multiline_label;
mod nine_segment;
mod shadow;
mod slider;
mod switch;
mod text_field;
mod theme_switch;

pub async fn test_base_views() -> anyhow::Result<()> {
    use crate::run_test_unit;

    run_test_unit!(test_custom_text_field);
    run_test_unit!(test_checkbox);
    run_test_unit!(test_background);
    run_test_unit!(test_label_image);
    run_test_unit!(test_label);
    run_test_unit!(test_label_font);
    run_test_unit!(test_label_measure);
    run_test_unit!(test_label_fit_text);
    run_test_unit!(test_letter_spacing);
    run_test_unit!(test_font_zoo);
    run_test_unit!(test_nine_segment);
    run_test_unit!(test_gradient);
    run_test_unit!(test_multiline);
    run_test_unit!(test_button);
    run_test_unit!(test_inject_touch);
    run_test_unit!(test_slider);
    run_test_unit!(test_switch);
    run_test_unit!(test_text_field);
    run_test_unit!(test_theme_switch);
    run_test_unit!(test_hover);
    run_test_unit!(test_corner_radii);
    run_test_unit!(test_shadow);

    Ok(())
}
