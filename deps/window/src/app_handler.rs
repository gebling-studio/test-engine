use std::sync::atomic::{AtomicBool, Ordering};

use gm::flat::Point;
use log::{debug, error};
use plat::Platform;
use refs::main_lock::MainLock;
use winit::{
    application::ApplicationHandler,
    event::{MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

use crate::{Window, WindowEvents, state::State};

static APP_HANDLER: MainLock<Option<AppHandler>> = MainLock::new();

/// Rough pixel height of one mouse wheel line. Wheel deltas arrive in lines,
/// trackpads send pixels, this converts lines to pixels. The result is
/// scaled again by `SCROLL_SPEED` in the engine.
const LINE_SCROLL_PIXELS: f32 = 28.0;

#[allow(clippy::large_enum_variant)]
enum AppHandlerState {
    Ready(Window),
    Init(Option<EventLoopProxy<Window>>),
}

impl AppHandlerState {
    fn ready(&self) -> bool {
        !self.not_ready()
    }

    fn not_ready(&self) -> bool {
        matches!(self, Self::Init(_))
    }
}

pub struct AppHandler {
    state:                       AppHandlerState,
    pub(crate) te_window_events: Box<dyn WindowEvents>,
    pub(crate) close:            AtomicBool,
}

impl AppHandler {
    pub fn new(app: impl WindowEvents + 'static, event_loop: &EventLoop<Window>) -> &'static mut Self {
        let handler = APP_HANDLER.get_mut();

        *handler = Some(Self {
            state:            AppHandlerState::Init(Some(event_loop.create_proxy())),
            te_window_events: Box::new(app),
            close:            AtomicBool::new(false),
        });

        handler.as_mut().expect("Failed to get handler")
    }
}

impl AppHandler {
    /// Run without a window, an event loop or a display. Frames render to an
    /// offscreen texture in a plain loop until [`Window::close`] is called.
    #[cfg(not_wasm)]
    pub fn run_headless(app: impl WindowEvents + 'static, size: gm::flat::Size<u32>) {
        let handler = APP_HANDLER.get_mut();

        *handler = Some(Self {
            state:            AppHandlerState::Init(None),
            te_window_events: Box::new(app),
            close:            AtomicBool::new(false),
        });

        let handler = handler.as_mut().expect("Failed to get handler");

        let window = hreads::unasync(Window::create_headless(size));

        handler.state = AppHandlerState::Ready(window);
        handler.te_window_events.window_ready();

        while !handler.close.load(Ordering::Relaxed) {
            let window = Self::window();

            window.state.update();

            if let Err(e) = window.state.render() {
                error!("Render error: {e:?}");
            }
        }
    }

    pub(crate) fn close() {
        Self::current().close.store(true, Ordering::Relaxed);
    }

    pub(crate) fn current() -> &'static mut Self {
        APP_HANDLER
            .get_mut()
            .as_mut()
            .expect("AppHandler has not been initialized yet.")
    }

    pub(crate) fn window() -> &'static mut Window {
        let AppHandlerState::Ready(ref mut window) = Self::current().state else {
            panic!("Window is not initialized yet")
        };
        window
    }
}

impl ApplicationHandler<Window> for AppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let AppHandlerState::Init(proxy) = &mut self.state
            && let Some(proxy) = proxy.take()
        {
            let mut win_attr = winit::window::Window::default_attributes();

            #[cfg(not_wasm)]
            {
                win_attr = win_attr.with_title("test-engine");
            }

            #[cfg(wasm)]
            {
                use winit::platform::web::WindowAttributesExtWebSys;
                win_attr = win_attr.with_append(true);
            }

            let window = event_loop.create_window(win_attr).expect("create window err.");

            let render_size = if Platform::IOS {
                window.outer_size()
            } else {
                window.inner_size()
            };

            hreads::block_on(Window::start_internal(render_size, window, proxy));
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, window: Window) {
        self.state = AppHandlerState::Ready(window);
        self.te_window_events.window_ready();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::CursorMoved { position, .. } => {
                self.te_window_events.mouse_moved((position.x, position.y).into());
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.te_window_events.mouse_event(state, button);
            }
            WindowEvent::CursorLeft { .. } => {
                self.te_window_events.cursor_left();
            }
            WindowEvent::Touch(touch) => {
                self.te_window_events.touch_event(touch);
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    let point: Point = (x, y).into();
                    self.te_window_events.mouse_scroll(point * LINE_SCROLL_PIXELS);
                }
                MouseScrollDelta::PixelDelta(delta) => {
                    self.te_window_events.mouse_scroll((delta.x, delta.y).into());
                }
            },
            WindowEvent::KeyboardInput { event, .. } => {
                if Window::quit_on_escape() && event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    event_loop.exit();
                }
                self.te_window_events.key_event(event);
            }
            WindowEvent::DroppedFile(path) => {
                self.te_window_events.dropped_file(path);
            }
            WindowEvent::ThemeChanged(theme) => {
                self.te_window_events.theme_changed(theme);
            }
            WindowEvent::Resized(_physical_size) => {
                if self.state.not_ready() {
                    return;
                }
                State::resize();
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer: _,
            } => {
                debug!("Scale factor: {scale_factor}");
            }
            WindowEvent::RedrawRequested => {
                if self.state.not_ready() {
                    return;
                }

                if self.close.load(Ordering::Relaxed) {
                    event_loop.exit();
                }

                Self::window().state.update();

                match Self::window().state.render() {
                    Ok(()) => {}
                    Err(e) => error!("Render error: {e:?}"),
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.state.ready()
            && let Some(window) = Window::winit_window()
        {
            window.request_redraw();
        }
    }
}
