#![allow(incomplete_features)]
#![allow(clippy::single_component_path_imports)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(unsized_const_params)]
#![feature(const_type_name)]

mod images;
mod input;
mod layout;
mod modal_view;
mod navigation_view;
mod style;
mod text_field_constraint;
mod to_label;
mod touch_layer;
mod touch_stack;
mod ui_event;
mod ui_manager;
mod view;
mod views;
mod with_header;

pub mod mobile;
pub mod serde;
pub mod ui_dispatch;

pub use images::*;
pub use input::*;
pub use layout::*;
pub use modal_view::*;
pub use navigation_view::*;
pub use style::*;
pub use text_field_constraint::*;
pub use to_label::*;
pub use touch_layer::*;
pub use touch_stack::*;
pub use ui_event::*;
pub use ui_manager::*;
pub use ui_proc::*;
pub use view::*;
pub use views::*;
pub use with_header::*;

extern crate core;
pub extern crate ui_proc;

/// Macro root for views defined inside this crate: `#[view(crate = crate::__macro_root)]`.
#[doc(hidden)]
pub mod __macro_root {
    pub use educe;
    pub use refs;

    pub use crate as ui;
}

pub const ALL_VIEWS: &[&str] = &all_views!();
