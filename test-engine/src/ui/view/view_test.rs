use anyhow::Result;
use refs::Weak;

use crate::ui::View;

pub trait ViewTest {
    fn perform_test(view: Weak<Self>) -> Result<()>;
}

impl<T: ?Sized + View> ViewTest for T {
    default fn perform_test(_view: Weak<Self>) -> Result<()> {
        Ok(())
    }
}
