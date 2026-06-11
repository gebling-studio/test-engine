use ui_proc::view;

#[view(crate = crate::__macro_root)]
pub struct NewLabel<T: 'static> {
    value: T,
}

impl<T> NewLabel<T> {
    pub fn value(&self) -> &T {
        &self.value
    }
}
