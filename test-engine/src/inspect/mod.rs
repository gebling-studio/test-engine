#![cfg(not_wasm)]

// The protocol stays available in release builds for the te-inspect CLI.
// The server exists only in debug builds and can never ship, see build.rs.
pub mod protocol;

#[cfg(debug_assertions)]
mod edit_log;
#[cfg(debug_assertions)]
mod inspect_service;
#[cfg(debug_assertions)]
mod view_conversion;

// The views are plain UI on top of the protocol, the inspector GUI needs
// them in release too. Only the server below is debug-only.
pub mod views;

pub use self::protocol::{AppCommand, InspectorCommand, ui::ViewRepr};
#[cfg(debug_assertions)]
pub use self::view_conversion::ViewToInspect;

#[cfg(debug_assertions)]
pub use crate::inspect::inspect_service::InspectService;
