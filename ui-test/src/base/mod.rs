use test_engine::Window;

use crate::base::{color_checker::test_color_checker, rest_request::test_rest_request};
use crate::base::{
    // async_calls::test_async_calls,
    colors::test_colors,
    corner_radius::test_corner_radius,
    // dispatch::test_dispatch,
    global_styles::test_global_styles,
    keymap::test_keymap,
    keymap_named_key::test_keymap_named_key,
    layout::test_layout,
    modal_test::test_modal,
    on_tap_add::test_add_on_tap,
    out_bounds_test::test_out_bounds,
    present::test_navigation_view,
    root_view::test_root_view,
    scale::test_scale,
    selection::test_selection,
    styles::test_styles,
    template::test_template,
    text_occlusion::test_text_occlusion,
    touch_order::test_touch_order,
    touch_stack::test_touch_stack,
    transition::test_transition,
    transparency::test_transparency,
    view_order::test_view_order,
};

mod async_calls;
mod color_checker;
mod colors;
mod corner_radius;
mod dispatch;
mod global_styles;
mod keymap;
mod keymap_named_key;
mod layout;
mod modal_test;
mod on_tap_add;
mod out_bounds_test;
mod present;
mod rest_request;
mod root_view;
mod scale;
mod selection;
mod styles;
mod template;
mod text_occlusion;
mod touch_order;
mod touch_stack;
mod transition;
mod transparency;
mod view_order;

pub async fn test_base_ui() -> anyhow::Result<()> {
    use crate::run_test_unit;

    run_test_unit!(test_add_on_tap);
    run_test_unit!(test_text_occlusion);
    run_test_unit!(test_corner_radius);
    run_test_unit!(test_color_checker);

    if !Window::headless() {
        run_test_unit!(test_rest_request);
    }

    run_test_unit!(test_transparency);
    run_test_unit!(test_scale);
    run_test_unit!(test_root_view);
    run_test_unit!(test_view_order);
    run_test_unit!(test_out_bounds);
    run_test_unit!(test_transition);
    run_test_unit!(test_global_styles);
    run_test_unit!(test_styles);
    run_test_unit!(test_colors);
    run_test_unit!(test_modal);
    run_test_unit!(test_touch_order);
    run_test_unit!(test_template);
    run_test_unit!(test_navigation_view);
    run_test_unit!(test_touch_stack);
    run_test_unit!(test_selection);
    run_test_unit!(test_keymap);
    run_test_unit!(test_keymap_named_key);
    run_test_unit!(test_layout);

    Ok(())
}
