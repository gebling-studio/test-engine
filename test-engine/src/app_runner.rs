use std::{path::PathBuf, sync::Once};

use anyhow::Result;
use gm::{
    LossyConvert,
    flat::{Point, Size},
};
use hreads::{from_main, invoke_dispatched, is_main_thread, wait_for_next_frame};
use level::LevelManager;
use log::debug;
use refs::{Own, main_lock::MainLock};
use ui::{Touch, TouchEvent, UIEvents, UIManager, View, ViewData, ViewSubviews, WeakView};
use wgpu::RenderPass;
use window::{ElementState, MouseButton, Screenshot, Window};
use winit::{
    event::{KeyEvent, TouchPhase},
    keyboard::Key,
};

use crate::{
    App,
    level_drawer::LevelDrawer,
    pipelines::Pipelines,
    ui::{Input, UIDrawer},
};

#[cfg(not_wasm)]
static WINDOW_READY: parking_lot::Mutex<vents::OnceEvent> =
    parking_lot::Mutex::new(vents::OnceEvent::const_default());
static CURSOR_POSITION: MainLock<Point> = MainLock::new();

/// Scroll sensitivity. Mouse wheel line deltas are already converted to
/// pixels by `LINE_SCROLL_PIXELS` in the window crate, then scaled by this.
const SCROLL_SPEED: f32 = 0.25;

pub struct AppRunner {
    pub(crate) app:        Box<dyn App>,
    pub(crate) first_view: Option<Own<dyn View>>,
    pub cursor_position:   Point,
}

impl AppRunner {
    pub fn stop() {
        Window::close();
    }

    pub fn cursor_position() -> Point {
        *CURSOR_POSITION
    }

    #[cfg(not_wasm)]
    pub(crate) fn setup_log() {
        use fern::Dispatch;
        use log::{Level, LevelFilter};

        Dispatch::new()
            .level(LevelFilter::Warn)
            .level_for("test_engine", LevelFilter::Debug)
            .level_for("inspector", LevelFilter::Debug)
            .level_for("shopping", LevelFilter::Debug)
            .level_for("netrun", LevelFilter::Debug)
            .format(|out, message, record| {
                let level_icon = match record.level() {
                    Level::Error => "🔴",
                    Level::Warn => "🟡",
                    Level::Info => "🟢",
                    Level::Debug => "🔵",
                    Level::Trace => "⚪",
                };

                let location = false;
                let module = false;

                let mut log = format!("{level_icon} {message}");

                if location {
                    log = format!(
                        "[{}::{}] {}",
                        record.file().unwrap_or_default(),
                        record.line().unwrap_or_default(),
                        log
                    );
                }

                if module {
                    log = format!("{} {}", record.module_path().unwrap_or_default(), log);
                }

                out.finish(format_args!("{log}"));
            })
            .chain(std::io::stdout())
            .apply()
            .expect("Failed to initialize logging");

        debug!("logs setup");
    }

    #[cfg(not_wasm)]
    pub async fn setup_sentry(app: &dyn App) -> Option<sentry::ClientInitGuard> {
        let sentry_url = crate::config::Config::sentry_url(app).await?;

        let client = sentry::init((
            sentry_url,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                // Capture user IPs and potentially sensitive headers when using HTTP server integrations
                // see https://docs.sentry.io/platforms/rust/data-management/data-collected for more info
                send_default_pii: true,
                ..Default::default()
            },
        ));

        debug!("sentry ready");

        Some(client)
    }

    #[cfg(wasm)]
    pub fn setup_sentry(_app: &dyn App) -> Option<()> {
        None
    }

    pub fn new(app: Box<dyn App>) -> Self {
        #[cfg(desktop)]
        crate::assets::Assets::init(filesystem::Paths::git_root().expect("git_root()"));
        #[cfg(mobile)]
        crate::assets::Assets::init(std::path::PathBuf::default());

        let first_view = Some(app.make_root_view());

        Self {
            app,
            cursor_position: Point::default(),
            first_view,
        }
    }

    #[cfg(target_os = "android")]
    pub(crate) async fn start(first_view: Own<dyn View>, app: crate::AndroidApp) -> Result<()> {
        std::panic::set_hook(Box::new(|pan| {
            let backtrace = std::backtrace::Backtrace::force_capture();
            eprintln!("{pan}");
            eprintln!("Backtrace: {backtrace}");
        }));

        use winit::platform::android::EventLoopBuilderExtAndroid;

        // android_logger::try_init(android_logger::Config::default().
        // with_max_level(LevelFilter::Trace));

        // try_init();

        android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Warn));

        let event_loop: crate::EventLoop =
            crate::EventLoop::with_user_event().with_android_app(app).build().unwrap();

        Window::start(Self::new(first_view), event_loop).await
    }

    #[cfg(not_wasm)]
    pub fn start_with_actor(
        actions: impl std::future::Future<Output = Result<()>> + Send + 'static,
    ) -> Result<()> {
        Self::start_with_actor_impl(actions, false);
        Ok(())
    }

    /// Run without a window or a display. Frames render to an offscreen
    /// texture. Screenshots and `check_colors` still work.
    #[cfg(not_wasm)]
    pub fn start_headless_with_actor(
        actions: impl std::future::Future<Output = Result<()>> + Send + 'static,
    ) -> Result<()> {
        Self::start_with_actor_impl(actions, true);
        Ok(())
    }

    #[cfg(not_wasm)]
    fn start_with_actor_impl(
        actions: impl std::future::Future<Output = Result<()>> + Send + 'static,
        headless: bool,
    ) {
        use ui::Setup;

        #[derive(Default)]
        struct ActorApp;

        impl App for ActorApp {
            fn make_root_view(&self) -> Own<dyn View> {
                ui::Container::new()
            }
        }

        WINDOW_READY.lock().sub(|| {
            hreads::unasync(actions).unwrap();
        });

        if headless {
            crate::app_starter::test_engine_start_with_app_headless(Box::new(ActorApp));
        } else {
            crate::app_starter::test_engine_start_with_app(Box::new(ActorApp));
        }
    }

    pub fn set_window_title(title: impl Into<String>) {
        Window::set_title(title);
    }

    #[cfg(desktop)]
    pub fn set_window_size(size: impl Into<Size<u32>> + Send + 'static) {
        let size = size.into();

        from_main(move || {
            Window::current().set_size(size);
        });

        if is_main_thread() {
            return;
        }

        // In windowed mode the OS applies the resize later. A touch injected
        // before it lands is processed against the old layout and misses
        // every view. Wait until the new size is real.
        for _ in 0..100 {
            let current: Size<u32> = from_main(Window::inner_size).lossy_convert();
            if current == size {
                return;
            }
            wait_for_next_frame();
        }

        panic!("Window did not resize to {size:?}");
    }

    pub fn take_screenshot() -> Result<Screenshot> {
        let recv = from_main(|| Window::current().request_screenshot());
        let screenshot = recv.recv()?;
        Ok(screenshot)
    }

    pub fn fps() -> f32 {
        Window::current().fps()
    }
}

impl window::WindowEvents for AppRunner {
    fn window_ready(&mut self) {
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            Pipelines::initialize();

            let mut root = UIManager::root_view();
            let view = root.add_subview_to_root(self.first_view.take().unwrap());
            view.place().back();

            UIManager::on_scale_changed(root, move |scale| {
                root.rescale_root(scale);
            });

            self.update();
            *LevelManager::update_interval() = 1.0 / Window::display_refresh_rate().lossy_convert();

            window::state::State::resize();

            self.resize(
                Window::inner_position(),
                Window::outer_position(),
                Window::inner_size(),
                Window::outer_size(),
            );

            debug!("UI initialized");

            #[cfg(not_wasm)]
            {
                #[cfg(desktop)]
                {
                    Window::current().set_size(self.app.initial_size().lossy_convert());
                }
                if self.app.enable_inspection() {
                    crate::inspect::InspectService::start_listening();
                }
            }

            UIManager::keymap().add(UIManager::root_view(), 'i', || {
                fn call_inspect(mut view: WeakView) {
                    view.__internal_inspect();
                    for sub in view.subviews() {
                        call_inspect(sub.weak());
                    }
                }

                call_inspect(UIManager::root_view());
            });

            self.app.after_launch();

            #[cfg(not_wasm)]
            hreads::spawn(async {
                debug!("window ready");
                WINDOW_READY.lock().trigger(());
            });
        });
    }

    fn update(&mut self) {
        UIManager::free_deleted_views();
        invoke_dispatched();
        LevelDrawer::update();
        UIDrawer::update();
    }

    fn render<'a>(&'a mut self, pass: &mut RenderPass<'a>) {
        if UIManager::window_resolution().has_no_area() {
            return;
        }

        LevelDrawer::draw(pass);
        UIDrawer::draw(pass);
    }

    fn resize(&mut self, inner_pos: Point, outer_pos: Point, inner_size: Size, outer_size: Size) {
        UIManager::set_scale(UIManager::display_scale());
        LevelManager::set_scale(UIManager::display_scale());

        UIManager::root_view().resize_root(inner_pos, outer_pos, inner_size, outer_size, UIManager::scale());
        UIEvents::size_changed().trigger(());
        self.update();
    }

    fn mouse_moved(&mut self, position: Point) -> bool {
        self.cursor_position = position;
        *CURSOR_POSITION.get_mut() = position;
        Input::process_touch_event(Touch {
            id: 1,
            position,
            event: TouchEvent::Moved,
            button: MouseButton::Left,
        })
    }

    fn mouse_event(&mut self, state: ElementState, button: MouseButton) -> bool {
        Input::process_touch_event(Touch {
            id: 1,
            position: self.cursor_position,
            event: state.into(),
            button,
        })
    }

    fn mouse_scroll(&mut self, delta: Point) {
        Input::on_scroll(delta * SCROLL_SPEED);
    }

    fn touch_event(&mut self, touch: winit::event::Touch) -> bool {
        Input::process_touch_event(Touch {
            id:       1,
            position: (touch.location.x, touch.location.y).into(),
            event:    match touch.phase {
                TouchPhase::Started => TouchEvent::Began,
                TouchPhase::Moved => TouchEvent::Moved,
                TouchPhase::Ended | TouchPhase::Cancelled => TouchEvent::Ended,
            },
            button:   MouseButton::Left,
        })
    }

    fn key_event(&mut self, event: KeyEvent) {
        if !event.state.is_pressed() {
            return;
        }

        if let Key::Named(key) = event.logical_key {
            Input::on_key(key);
        }

        if let Some(ch) = event.logical_key.to_text() {
            Input::on_char(ch.chars().last().unwrap());
        }
    }

    fn dropped_file(&mut self, path: PathBuf) {
        UIManager::trigger_drop_file(path);
    }
}
