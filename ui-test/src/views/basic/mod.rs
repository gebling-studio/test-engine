use crate::views::basic::{
    background::test_background, button::test_button, checkbox::test_checkbox,
    custom_text_field::test_custom_text_field, gradient::test_gradient, inject_touch::test_inject_touch,
    label::test_label, label_image::test_label_image, multiline_label::test_multiline,
    nine_segment::test_nine_segment, slider::test_slider, switch::test_switch, text_field::test_text_field,
};

mod background;
mod button;
mod checkbox;
mod custom_text_field;
mod gradient;
mod inject_touch;
mod label;
mod image_scissor;
mod label_image;
mod multiline_label;
mod nine_segment;
mod slider;
mod switch;
mod text_field;

pub async fn test_base_views() -> anyhow::Result<()> {
    test_custom_text_field().await?;
    test_checkbox().await?;
    test_background().await?;
    test_label_image().await?;
    test_label().await?;
    test_nine_segment().await?;
    test_gradient().await?;
    test_multiline().await?;
    test_button().await?;
    test_inject_touch().await?;
    test_slider().await?;
    test_switch().await?;
    test_text_field().await?;

    Ok(())
}
