use anyhow::Result;
use refs::Weak;

use crate::ui::View;

pub trait ViewTest {
    fn perform_test(view: Weak<Self>) -> Result<()>;

    /// Screen pixels the test draws in. Override when the default is too small
    /// for the view, but keep it within the smallest supported screen, which is
    /// 640 by 1136 on an iPhone 5S.
    fn canvas() -> (u32, u32);
}

impl<T: ?Sized + View> ViewTest for T {
    default fn perform_test(_view: Weak<Self>) -> Result<()> {
        Ok(())
    }

    default fn canvas() -> (u32, u32) {
        (600, 600)
    }
}
