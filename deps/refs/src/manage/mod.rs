use std::collections::BTreeMap;

mod data_manager;
mod exists_managed;
mod managed;
mod resource_loader;
mod tests;

pub use data_manager::{DataManager, InFlightDownloads};
pub use exists_managed::ExistsManaged;
pub use resource_loader::ResourceLoader;

pub type DataStorage<T> = BTreeMap<String, crate::Own<T>>;

pub trait Managed: 'static + ResourceLoader + DataManager<Self> {}
