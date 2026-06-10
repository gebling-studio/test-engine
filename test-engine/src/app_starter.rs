use window::{AppHandler, Window};
use winit::event_loop::{ControlFlow, EventLoop};

use crate::{App, AppRunner, app::test_engine_create_app};

#[cfg(target_arch = "wasm32")]
fn run_app(event_loop: EventLoop<Window>, app: &'static mut AppHandler) {
    // Runs the app async via the browsers event loop
    use winit::platform::web::EventLoopExtWebSys;
    hreads::spawn(async move {
        event_loop.spawn_app(app);
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn run_app(event_loop: EventLoop<Window>, app: &mut AppHandler) {
    let _ = event_loop.run_app(app);
}

#[cfg(not(target_os = "android"))]
#[unsafe(no_mangle)]
pub extern "C" fn test_engine_start_app() -> std::ffi::c_int {
    #[allow(unused_unsafe)]
    test_engine_start_with_app(unsafe { test_engine_create_app() })
}

pub(crate) fn test_engine_start_with_app(app: Box<dyn App>) -> std::ffi::c_int {
    fn start(app: Box<dyn App>) {
        hreads::set_current_thread_as_main();

        let event_loop = EventLoop::<Window>::with_user_event().build().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        app.before_launch();
        let app = AppHandler::new(AppRunner::new(app), &event_loop);
        run_app(event_loop, app);
    }

    #[cfg(not_wasm)]
    AppRunner::setup_log();

    #[cfg(wasm)]
    {
        // Sets up panics to go to the console.error in browser environments
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");

        log::info!("Hello from wasm");
    }

    #[cfg(wasm)]
    {
        start(app);
    }

    #[cfg(not_wasm)]
    {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let sentry_guard = AppRunner::setup_sentry(std::ops::Deref::deref(&app)).await;

            start(app);

            drop(sentry_guard);
        });
    }

    0
}
