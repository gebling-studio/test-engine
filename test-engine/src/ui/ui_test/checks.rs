use anyhow::{Result, bail};
use crate::gm::{color::U8Color, flat::Point};
use hreads::from_main;
use crate::ui::{HighlightView, Setup, UIManager, ViewFrame, ViewSubviews};
use crate::window::Screenshot;

use crate::{
    AppRunner,
    ui_test::{TEST_NAME, failure_report},
};

pub(super) fn check_pixel_color(screenshot: &Screenshot, pos: Point, color: U8Color) -> Result<()> {
    let pixel: U8Color = screenshot.get_pixel(pos);

    let diff = pixel.diff_u8(color);

    let max_diff = 45;

    if diff > max_diff {
        from_main(move || {
            let mut high = HighlightView::new();
            high.set_z_position(0.1);

            UIManager::root_view()
                .add_subview_to_root(high)
                .downcast_view::<HighlightView>()
                .unwrap()
                .set(pos, color.into(), pixel.into());
        });

        let test_name = TEST_NAME.lock().clone();

        bail!(
            r"
        Test: {test_name} has failed.
        Color diff is too big: {diff}. Max: {max_diff}. Position: {pos:?}.
        Expected: {color}, got: {pixel}.
        {:>4} {:>4} - {:>3} {:>3} {:>3} -> {:>3} {:>3} {:>3}
        {}",
            pos.x,
            pos.y,
            color.r,
            color.g,
            color.b,
            pixel.r,
            pixel.g,
            pixel.b,
            failure_report()?,
        )
    }

    Ok(())
}

pub(super) fn check_colors_structured(data: &[(Point, U8Color)]) -> Result<()> {
    let screenshot = AppRunner::take_screenshot()?;

    for (pos, color) in data {
        check_pixel_color(&screenshot, *pos, *color)?;
    }

    Ok(())
}
