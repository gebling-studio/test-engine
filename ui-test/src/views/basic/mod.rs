use crate::views::basic::{
    background::test_background, button::test_button, checkbox::test_checkbox,
    corner_radii::test_corner_radii, custom_text_field::test_custom_text_field, font_zoo::test_font_zoo,
    gradient::test_gradient, hover::test_hover, inject_touch::test_inject_touch, label::test_label,
    label_fit_text::test_label_fit_text, label_font::test_label_font, label_image::test_label_image,
    label_measure::test_label_measure, multiline_label::test_multiline, nine_segment::test_nine_segment,
    shadow::test_shadow, slider::test_slider, switch::test_switch, text_field::test_text_field,
    theme_switch::test_theme_switch,
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
mod multiline_label;
mod nine_segment;
mod shadow;
mod slider;
mod switch;
mod text_field;
mod theme_switch;

pub async fn test_base_views() -> anyhow::Result<()> {
    test_custom_text_field().await?;
    test_checkbox().await?;
    test_background().await?;
    test_label_image().await?;
    test_label().await?;
    test_label_font().await?;
    test_label_measure().await?;
    test_label_fit_text().await?;
    test_font_zoo().await?;
    test_nine_segment().await?;
    test_gradient().await?;
    test_multiline().await?;
    test_button().await?;
    test_inject_touch().await?;
    test_slider().await?;
    test_switch().await?;
    test_text_field().await?;
    test_theme_switch().await?;
    test_hover().await?;
    test_corner_radii().await?;
    test_shadow().await?;

    Ok(())
}
