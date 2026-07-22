use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(not_wasm)]
use hreads::is_main_thread;
#[cfg(not_wasm)]
use parking_lot::Mutex;
#[cfg(not_wasm)]
use winit::event_loop::EventLoopProxy;

#[cfg(not_wasm)]
use crate::window::app_handler::UserEvent;

/// A frame is pending. Set by any change that has to reach the screen, cleared
/// once per rendered frame. Starts true so the very first frame always draws.
static NEEDS_REDRAW: AtomicBool = AtomicBool::new(true);

/// Wakes the winit loop when it sleeps in `ControlFlow::Wait`. Sending a user
/// event is the winit blessed way to wake the loop from any thread. It stays
/// `None` in headless, which renders every iteration and ignores the flag.
///
/// Wasm never has one. It is single threaded and browser driven, the loop polls
/// every iteration and there is no other thread to wake it from. The proxy type
/// is also not `Sync` there, so a static holding it would not even compile.
#[cfg(not_wasm)]
static WAKE_PROXY: Mutex<Option<EventLoopProxy<UserEvent>>> = Mutex::new(None);

#[cfg(not_wasm)]
pub(crate) fn set_wake_proxy(proxy: EventLoopProxy<UserEvent>) {
    *WAKE_PROXY.lock() = Some(proxy);
}

/// Ask for one more rendered frame. Safe to call from any thread. Continuous
/// work like animations and levels calls this every frame to keep drawing, so
/// a screen with neither goes idle and stops burning CPU.
pub(crate) fn request_frame() {
    NEEDS_REDRAW.store(true, Ordering::Relaxed);

    #[cfg(not_wasm)]
    {
        // On the main thread the loop is already awake and `about_to_wait` will
        // pick up the flag this iteration, so a wake event would be redundant.
        if is_main_thread() {
            return;
        }

        if let Some(proxy) = WAKE_PROXY.lock().as_ref()
            && proxy.send_event(UserEvent::Wake).is_err()
        {
            // The event loop already closed, the app is shutting down.
            log::trace!("wake requested after the event loop closed");
        }
    }
}

/// Consumes the pending flag. The winit loop calls this once per iteration and
/// draws only when it returns true.
pub(crate) fn take_needs_render() -> bool {
    NEEDS_REDRAW.swap(false, Ordering::Relaxed)
}
