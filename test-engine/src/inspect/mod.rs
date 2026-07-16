#![cfg(all(not_wasm, feature = "inspect"))]

pub mod protocol;

mod edit_log;
mod inspect_service;
mod view_conversion;

pub mod views;

pub use self::{
    protocol::{AppCommand, InspectorCommand, ui::ViewRepr},
    view_conversion::ViewToInspect,
};
pub use crate::inspect::inspect_service::InspectService;
