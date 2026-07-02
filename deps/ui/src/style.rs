use std::{any::type_name, collections::HashMap, ops::DerefMut};

use refs::{Weak, main_lock::MainLock};

use crate::{Button, Label, NumberView, View};

static GLOBAL_STYLES: MainLock<HashMap<&'static str, Vec<Style>>> = MainLock::new();

static ALLOWED_TYPES: &[&str] = &[
    type_name::<Button>(),
    type_name::<Label>(),
    type_name::<NumberView>(),
];

#[allow(unpredictable_function_pointer_comparisons)] // TODO:
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    action: fn(&mut dyn View),
}

impl Style {
    pub const fn new(action: fn(&mut dyn View)) -> Self {
        Self { action }
    }

    pub fn apply(&self, view: &mut dyn View) {
        (self.action)(view);
    }

    pub(crate) fn apply_global<T: View>(view: Weak<T>) {
        Self::check_allowed::<T>();

        if view.__base_view().ignore_global_style {
            return;
        }

        for style in Self::get_global_for::<T>() {
            style.apply(view.weak_view().deref_mut());
        }
    }

    fn get_global_for<T: View>() -> &'static [Style] {
        if let Some(styles) = GLOBAL_STYLES.get(type_name::<T>()) {
            styles
        } else {
            &[]
        }
    }

    pub fn apply_globally<T: View>(&self) {
        Self::check_allowed::<T>();
        let styles = GLOBAL_STYLES.get_mut().entry(type_name::<T>()).or_default();

        assert!(
            !styles.contains(self),
            "{} already has this global style",
            type_name::<T>()
        );

        styles.push(*self);
    }

    pub fn reset_global<T: View>(&self) {
        Self::check_allowed::<T>();
        let styles = GLOBAL_STYLES.get_mut().entry(type_name::<T>()).or_default();
        styles.clear();
    }

    fn check_allowed<T: View>() {
        assert!(
            ALLOWED_TYPES.contains(&type_name::<T>()),
            "Global style for {} is not allowed. Allowed types: {ALLOWED_TYPES:?}",
            type_name::<T>()
        );
    }
}

#[cfg(test)]
mod test {
    use std::hint::black_box;

    use hreads::set_current_thread_as_main;

    use crate::{Button, Label, Style, Switch};

    // Styles compare by function pointer. Empty identical bodies can be
    // merged by the compiler into one function, which would make these
    // styles equal and trip the duplicate check in apply_globally.
    // black_box with distinct strings keeps the bodies distinct.
    const STYLE: Style = Style::new(|_v| {});
    const STYLE2: Style = Style::new(|_v| {
        black_box("a");
    });
    const STYLE3: Style = Style::new(|_v| {
        black_box("b");
    });

    #[test]
    fn valid_global_style_type() {
        set_current_thread_as_main();
        STYLE.apply_globally::<Button>();
        STYLE2.apply_globally::<Button>();
        STYLE3.apply_globally::<Label>();

        assert_eq!(Style::get_global_for::<Button>(), &[STYLE, STYLE2]);
        assert_eq!(Style::get_global_for::<Label>(), &[STYLE3]);
    }

    #[test]
    #[should_panic]
    fn invalid_global_style_type() {
        STYLE.apply_globally::<Switch>();
    }
}
