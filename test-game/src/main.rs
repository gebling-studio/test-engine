#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use crate::app::TestGameApp;

mod api;
mod app;
mod interface;
mod levels;
mod no_physics;

test_engine::register_app!(TestGameApp);

fn main() {
    test_engine::test_engine_start_app();
}
