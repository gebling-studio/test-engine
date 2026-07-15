use crate::views::images::{
    image_flip::test_image_flip, image_on_view::test_image_on_view, image_view::test_image_view,
    image_view_svg::test_image_view_svg,
};

mod image_flip;
mod image_on_view;
mod image_view;
mod image_view_svg;

pub async fn test_image_views() -> anyhow::Result<()> {
    use crate::run_test_unit;

    run_test_unit!(test_image_view_svg);
    run_test_unit!(test_image_view);
    run_test_unit!(test_image_flip);
    run_test_unit!(test_image_on_view);

    Ok(())
}
