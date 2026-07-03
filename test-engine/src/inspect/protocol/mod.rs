mod app_command;
mod inspector_command;
mod transport;
pub mod ui;

pub use self::{app_command::*, inspector_command::*, transport::*};

pub const SERVICE_TYPE: &str = "_te-inspect._tcp.local.";
