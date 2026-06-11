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

fn main() {}
