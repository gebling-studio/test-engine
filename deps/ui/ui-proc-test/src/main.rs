#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use ui::Button;
use ui_proc::view;

#[view(crate = ui::__macro_root)]
struct _ProcView {
    _button:      Button,
    #[init]
    _weak_button: Button,
}

// Pins where-clause support: the macro must carry it into generated impls.
#[view(crate = ui::__macro_root)]
struct _GenericView<T>
where T: Default + 'static
{
    _value: T,
}

fn main() {}
