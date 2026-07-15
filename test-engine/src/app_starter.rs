use std::hint::black_box;

use winit::event_loop::{ControlFlow, EventLoop};

use crate::{
    App, AppRunner,
    app::test_engine_create_app,
    window::{AppHandler, Window},
};

/// Names a symbol in the `ctor` crate so a linker keeps that crate's object.
///
/// `#[view]` registers a test through a `ctor`, which only writes an entry
/// into a linker section. A single initializer inside the `ctor` crate walks
/// that section and calls the entries, and it returns without a word when its
/// guard is missing. iOS links an app against `libtest_game.a`, and a linker
/// loads an archive member only to resolve an undefined symbol. Nothing named
/// the guard, which is reachable only through section boundary symbols, so the
/// member stayed out of the link. The initializer never made it into the app
/// and every test registration was dropped in silence. This reference is that
/// undefined symbol.
fn keep_ctor_linked() {
    black_box(&crate::__internal_macro_deps::ctor::collect::GUARD_ATOMIC);
}

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
    event_loop.run_app(app).expect("Event loop failed");
}

#[cfg(not(target_os = "android"))]
#[unsafe(no_mangle)]
pub extern "C" fn test_engine_start_app() -> std::ffi::c_int {
    #[allow(unused_unsafe)]
    test_engine_start_with_app(unsafe { test_engine_create_app() })
}

pub(crate) fn test_engine_start_with_app(app: Box<dyn App>) -> std::ffi::c_int {
    start_with_app(app, false)
}

/// Run without a window or a display. Frames render to an offscreen texture.
#[cfg(not_wasm)]
pub(crate) fn test_engine_start_with_app_headless(app: Box<dyn App>) -> std::ffi::c_int {
    start_with_app(app, true)
}

fn start_with_app(app: Box<dyn App>, headless: bool) -> std::ffi::c_int {
    fn start(app: Box<dyn App>, headless: bool) {
        keep_ctor_linked();
        hreads::set_current_thread_as_main();
        app.before_launch();

        #[cfg(not_wasm)]
        if headless {
            use crate::gm::LossyConvert;

            let size = app.initial_size().lossy_convert();
            AppHandler::run_headless(AppRunner::new(app), size);
            return;
        }

        #[cfg(wasm)]
        let _ = headless;

        let event_loop = EventLoop::<Window>::with_user_event().build().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let app = AppHandler::new(AppRunner::new(app), &event_loop);
        run_app(event_loop, app);
    }

    let headless = headless || std::env::var("TE_HEADLESS").is_ok();

    #[cfg(not_wasm)]
    AppRunner::setup_log();

    #[cfg(target_os = "ios")]
    {
        crate::ios_log::set_panic_hook();
        crate::ios_log::set_exception_logger();
    }

    #[cfg(wasm)]
    {
        // Sets up panics to go to the console.error in browser environments
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize logger");

        log::info!("Hello from wasm");
    }

    #[cfg(wasm)]
    {
        start(app, headless);
    }

    #[cfg(not_wasm)]
    {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let sentry_guard = AppRunner::setup_sentry(std::ops::Deref::deref(&app)).await;

            start(app, headless);

            drop(sentry_guard);
        });
    }

    0
}
