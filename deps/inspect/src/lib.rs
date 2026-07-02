mod app_command;
mod inspector_command;
mod transport;
pub mod ui;

pub use app_command::*;
pub use inspector_command::*;
pub use transport::*;

pub const SERVICE_TYPE: &str = "_te-inspect._tcp.local.";
