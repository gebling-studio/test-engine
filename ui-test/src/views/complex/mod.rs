use crate::views::complex::{
    alert::test_alert, backdrop_blur::test_backdrop_blur, buttons_on_table::test_buttons_on_table_view,
    drop_down::test_drop_down, modal_blur::test_modal_blur, modal_scrim::test_modal_scrim,
    number_view::test_number_view, number_view_design::test_number_view_design,
    point_view::test_point_view, table_view_resize::test_table_view_resize,
};

mod alert;
mod backdrop_blur;
mod buttons_on_table;
mod drop_down;

mod modal_blur;
mod modal_scrim;
mod number_view;
mod number_view_design;
mod point_view;
mod table_view_resize;

pub async fn test_complex_views() -> anyhow::Result<()> {
    test_alert().await?;
    test_modal_scrim().await?;
    test_backdrop_blur().await?;
    test_modal_blur().await?;
    test_table_view_resize().await?;
    test_drop_down().await?;
    test_number_view_design().await?;
    test_number_view().await?;
    test_buttons_on_table_view().await?;
    test_point_view().await?;

    Ok(())
}
