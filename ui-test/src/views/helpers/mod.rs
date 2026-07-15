use crate::views::helpers::{
    highlight::test_highlight, keyboard::test_keyboard_view, position_view::test_position_view,
};

mod highlight;
mod keyboard;
mod position_view;

pub async fn test_helper_views() -> anyhow::Result<()> {
    use crate::run_test_unit;

    run_test_unit!(test_position_view);
    run_test_unit!(test_keyboard_view);
    run_test_unit!(test_highlight);
    Ok(())
}
