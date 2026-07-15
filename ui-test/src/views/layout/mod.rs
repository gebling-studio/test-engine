use center_field::test_center_field;
use tiling_layout::test_tiling_layout;

use crate::views::layout::{
    cell_layout::test_cell_layout, flow_wrap::test_flow_wrap, flow_wrap_text::test_flow_wrap_text,
    min_width::test_min_width, near_layout::test_near_layout, relative_layout::test_relative_layout,
};

mod cell_layout;
mod center_field;
mod flow_wrap;
mod flow_wrap_text;
mod min_width;
mod near_layout;
mod relative_layout;
mod tiling_layout;

pub async fn test_layout() -> anyhow::Result<()> {
    use crate::run_test_unit;

    run_test_unit!(test_near_layout);
    run_test_unit!(test_relative_layout);
    run_test_unit!(test_cell_layout);
    run_test_unit!(test_min_width);
    run_test_unit!(test_center_field);
    run_test_unit!(test_tiling_layout);
    run_test_unit!(test_flow_wrap);
    run_test_unit!(test_flow_wrap_text);
    Ok(())
}
