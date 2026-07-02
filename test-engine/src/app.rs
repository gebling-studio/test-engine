use std::pin::Pin;

use anyhow::bail;
use gm::flat::Size;
use refs::Own;
use ui::View;

use crate::app_starter::test_engine_start_with_app;

pub type PinnedFuture<T> = Pin<Box<dyn Future<Output = anyhow::Result<T>>>>;

pub trait App {
    fn before_launch(&self) {}
    fn after_launch(&self) {}
    fn make_root_view(&self) -> Own<dyn View>;
    fn initial_size(&self) -> Size {
        (1200, 1000).into()
    }

    fn start()
    where Self: Default + Sized + 'static {
        test_engine_start_with_app(Box::new(Self::default()));
    }

    fn sentry_url(&self) -> PinnedFuture<String> {
        Box::pin(async { bail!("Not implemented") })
    }
}

#[cfg(ios)]
unsafe extern "C" {
    #[allow(improper_ctypes_definitions)]
    #[allow(improper_ctypes)]
    pub fn test_engine_create_app() -> Box<dyn App>;
}

#[cfg(not(ios))]
#[unsafe(no_mangle)]
#[linkage = "weak"]
#[allow(improper_ctypes_definitions)]
#[allow(improper_ctypes)]
pub extern "C" fn test_engine_create_app() -> Box<dyn App> {
    panic!("you need to use test_engine::register_app!(YourApp) macro")
}

#[macro_export]
macro_rules! register_app {
    ($app:ty) => {
        pub use test_engine;

        #[unsafe(no_mangle)]
        #[allow(improper_ctypes_definitions)]
        pub extern "C" fn test_engine_create_app() -> Box<dyn test_engine::App> {
            use test_engine::App;

            fn check_trait<T: test_engine::App>() {}
            check_trait::<$app>();

            Box::new(<$app>::default())
        }
    };
}

#[macro_export]
macro_rules! export_ui_tests {
    () => {
        #[allow(clippy::type_complexity)]
        pub static UI_TESTS: test_engine::__internal_macro_deps::Mutex<
            std::collections::BTreeMap<String, fn() -> anyhow::Result<()>>,
        > = test_engine::__internal_macro_deps::Mutex::new(std::collections::BTreeMap::new());
    };
}
