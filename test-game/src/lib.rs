#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

mod api;
mod app;
mod interface;
mod levels;
mod no_physics;

// The library build is what iOS and ui-test link. Exposing the app entry
// keeps its whole chain reachable, so it is not dead code off the binary.
pub use app::TestGameApp;
#[cfg(not(ios))]
pub use test_engine;

#[cfg(ios)]
test_engine::register_app!(TestGameApp);
test_engine::export_ui_tests!();
