use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
    mpsc::Receiver,
};

use gm::{
    LossyConvert,
    color::Color,
    flat::{Point, Size},
};
use hreads::on_main;
use log::{info, warn};
use plat::Platform;
use wgpu::{
    Adapter, CompositeAlphaMode, Device, DeviceDescriptor, ExperimentalFeatures, Features, Instance, Limits,
    MemoryHints, PowerPreference, PresentMode, Queue, RequestAdapterOptions, SurfaceConfiguration,
    TextureUsages, Trace,
};
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy};

use crate::{
    Screenshot,
    app_handler::AppHandler,
    screen::Screen,
    state::{SURFACE_TEXTURE_FORMAT, State},
    surface::Surface,
};

static VSYNC: AtomicBool = AtomicBool::new(true);
static MAX_FRAME_LATENCY: AtomicU32 = AtomicU32::new(2);
static RENDER_FRAME: AtomicU64 = AtomicU64::new(0);
/// Mirrors `Screen::Headless` so any thread can check it. Set once at
/// startup, never changes.
static HEADLESS: AtomicBool = AtomicBool::new(false);
/// Doesn't work on some Androids and on Web
pub(crate) const SUPPORT_SCREENSHOT: bool = !Platform::ANDROID && !Platform::WASM;

#[cfg(target_os = "android")]
pub type Events = winit::platform::android::activity::AndroidApp;

#[cfg(not(target_os = "android"))]
pub type Events = ();

pub struct Window {
    pub state: State,

    pub(crate) instance: Instance,
    pub(crate) adapter:  Adapter,
    pub(crate) device:   Device,
    pub(crate) queue:    Queue,

    pub(crate) screen: Screen,

    pub(crate) title_set: bool,

    #[cfg(desktop)]
    pub(crate) is_resizing: bool,
}

impl Window {
    pub fn current() -> &'static mut Self {
        AppHandler::window()
    }

    pub fn device() -> &'static Device {
        &Self::current().device
    }

    pub fn queue() -> &'static Queue {
        &Self::current().queue
    }

    /// Rendering goes to an offscreen texture, there is no window and no
    /// display. Decided at startup. Callable from any thread.
    pub fn headless() -> bool {
        HEADLESS.load(Ordering::Relaxed)
    }

    #[cfg(desktop)]
    pub fn is_resizing() -> bool {
        Self::current().is_resizing
    }

    pub(crate) fn winit_window() -> Option<&'static winit::window::Window> {
        Self::current().screen.winit_window()
    }

    pub fn inner_size() -> Size {
        match &Self::current().screen {
            Screen::Windowed { winit_window, .. } => {
                let size = winit_window.inner_size();
                (size.width, size.height).into()
            }
            Screen::Headless { size } => (size.width, size.height).into(),
        }
    }

    pub fn outer_size() -> Size {
        match &Self::current().screen {
            Screen::Windowed { winit_window, .. } => {
                let size = winit_window.outer_size();
                (size.width, size.height).into()
            }
            Screen::Headless { size } => (size.width, size.height).into(),
        }
    }

    pub fn render_size() -> Size {
        if Platform::IOS {
            Window::outer_size()
        } else {
            Window::inner_size()
        }
    }

    pub fn inner_position() -> Point {
        let Some(window) = Self::winit_window() else {
            return Point::default();
        };
        let pos = window.inner_position().unwrap_or_default();
        (pos.x, pos.y).into()
    }

    pub fn outer_position() -> Point {
        let Some(window) = Self::winit_window() else {
            return Point::default();
        };
        let pos = window.outer_position().unwrap_or_default();
        (pos.x, pos.y).into()
    }

    pub fn screen_scale() -> f32 {
        let Some(window) = Self::winit_window() else {
            return 1.0;
        };
        window.scale_factor().lossy_convert()
    }

    pub fn set_clear_color(color: impl Into<Color>) {
        Self::current().state.clear_color = color.into();
    }

    pub fn close() {
        on_main(AppHandler::close);
    }

    async fn request_device(adapter: &Adapter) -> (Device, Queue) {
        let mut required_limits = Limits::default();

        if Platform::IOS {
            required_limits.max_color_attachments = 4;
        } else if Platform::ANDROID {
            // TODO:
            required_limits.max_compute_invocations_per_workgroup = 0;
            required_limits.max_compute_workgroups_per_dimension = 0;
            required_limits.max_compute_workgroup_storage_size = 0;
            required_limits.max_compute_workgroup_size_x = 0;
            required_limits.max_compute_workgroup_size_y = 0;
            required_limits.max_compute_workgroup_size_z = 0;
            required_limits.max_storage_buffer_binding_size = 0;
            required_limits.max_storage_textures_per_shader_stage = 0;
            required_limits.max_storage_buffers_per_shader_stage = 0;
            required_limits.max_dynamic_storage_buffers_per_pipeline_layout = 0;
            required_limits.max_texture_dimension_3d = 1024;
            required_limits.max_texture_dimension_2d = 4096;
            required_limits.max_texture_dimension_1d = 4096;
        } else if Platform::WASM {
            required_limits = Limits::downlevel_webgl2_defaults();
            required_limits.max_texture_dimension_1d = 8192;
            required_limits.max_texture_dimension_2d = 8192;
        }

        adapter
            .request_device(&DeviceDescriptor {
                required_features: Features::empty(),
                // Doesn't work on some Androids
                // required_features: Features::POLYGON_MODE_LINE, // | Features::POLYGON_MODE_POINT,
                required_limits,
                label: None,
                memory_hints: MemoryHints::Performance,
                trace: Trace::default(),
                experimental_features: ExperimentalFeatures::default(),
            })
            .await
            .expect("Failed to request device")
    }

    pub(crate) async fn start_internal(
        size: PhysicalSize<u32>,
        window: winit::window::Window,
        proxy: EventLoopProxy<Window>,
    ) {
        let winit_window = Arc::new(window);

        let instance = Instance::default();
        let surface = instance.create_surface(winit_window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference:       PowerPreference::HighPerformance,
                force_fallback_adapter: false,

                compatible_surface: Some(&surface),
            })
            .await
            .expect("Could not get an adapter (GPU).");

        let info = adapter.get_info();

        info!("Backend: {}", &info.backend);

        let (device, queue) = Self::request_device(&adapter).await;

        let surface = if size.width != 0 && size.height != 0 {
            Surface::new(
                &instance,
                &adapter,
                &device,
                surface_config_with_size((size.width, size.height)),
                winit_window.clone(),
            )
            .expect("Failed to create surface")
            .into()
        } else {
            None
        };

        let window = Self {
            state: State::default(),
            instance,
            adapter,
            device,
            queue,
            screen: Screen::Windowed { winit_window, surface },
            #[cfg(desktop)]
            is_resizing: false,
            title_set: false,
        };

        if proxy.send_event(window).is_err() {
            panic!("Failed to send window event")
        }
    }

    #[cfg(not_wasm)]
    pub(crate) async fn create_headless(size: Size<u32>) -> Self {
        let instance = Instance::default();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference:       PowerPreference::HighPerformance,
                force_fallback_adapter: false,

                compatible_surface: None,
            })
            .await
            .expect("Could not get an adapter (GPU).");

        let info = adapter.get_info();

        info!("Backend: {} (headless)", &info.backend);

        HEADLESS.store(true, Ordering::Relaxed);

        let (device, queue) = Self::request_device(&adapter).await;

        Self {
            state: State::default(),
            instance,
            adapter,
            device,
            queue,
            screen: Screen::Headless { size },
            #[cfg(desktop)]
            is_resizing: false,
            title_set: false,
        }
    }

    pub fn set_title(title: impl Into<String>) {
        let title = title.into();
        on_main(move || {
            Self::current().title_set = true;
            if let Some(window) = Self::winit_window() {
                if Platform::DESKTOP {
                    window.set_title(&title);
                } else {
                    warn!("set_title is not supported on this platform");
                }
            }
        });
    }

    #[cfg(desktop)]
    pub fn set_size(&mut self, size: impl Into<Size<u32>>) {
        let size = size.into();

        let current_size: Size<u32> = Window::inner_size().lossy_convert();

        if size == current_size {
            return;
        }

        match &mut self.screen {
            Screen::Windowed { winit_window, .. } => {
                self.is_resizing = true;
                let _ = winit_window.request_inner_size(PhysicalSize::new(size.width, size.height));
            }
            Screen::Headless { size: headless_size } => {
                *headless_size = size;
                State::resize();
            }
        }
    }

    pub fn request_screenshot(&self) -> Receiver<Screenshot> {
        self.state.request_read_display()
    }

    pub fn fps(&self) -> f32 {
        self.state.frame_counter.fps
    }

    pub fn frame_time(&self) -> f32 {
        self.state.frame_counter.frame_time
    }

    /// CPU time of the last frame's update and render encoding. Not capped by
    /// vsync or the compositor - use for performance measurements.
    pub fn frame_work_time(&self) -> f32 {
        self.state.frame_work_time
    }

    pub fn frame_drawn(&self) -> u32 {
        self.state.frame_counter.frame_count
    }

    /// Always enabled on mobile platforms. Takes effect on the next frame.
    pub fn set_vsync(enable: bool) {
        on_main(move || {
            VSYNC.store(enable, Ordering::Relaxed);
            Self::reconfigure_surface();
        });
    }

    /// How many frames the GPU is allowed to buffer ahead of the display.
    /// Default is 2 - lowest input latency. 3 renders faster but adds up to
    /// one frame of lag. Backends clamp unsupported values.
    pub fn set_max_frame_latency(latency: u32) {
        on_main(move || {
            MAX_FRAME_LATENCY.store(latency, Ordering::Relaxed);
            Self::reconfigure_surface();
        });
    }

    /// Index of the frame currently being rendered. Bumps once per rendered
    /// frame, before any draw code runs.
    pub fn render_frame() -> u64 {
        RENDER_FRAME.load(Ordering::Relaxed)
    }

    pub(crate) fn next_render_frame() {
        RENDER_FRAME.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn reconfigure_surface() {
        let window = Self::current();

        if let Screen::Windowed {
            surface: Some(surface), ..
        } = &window.screen
        {
            let size: Size<u32> = Self::render_size().lossy_convert();
            surface.presentable.configure(&window.device, &surface_config_with_size(size));
        }
    }

    pub fn display_refresh_rate() -> u32 {
        let Some(window) = Self::winit_window() else {
            return 60;
        };
        window.current_monitor().map_or(60, |monitor| {
            monitor.refresh_rate_millihertz().unwrap_or(60_000) / 1000
        })
    }
}

pub(crate) fn surface_config_with_size(size: impl Into<Size<u32>>) -> SurfaceConfiguration {
    let size: Size<u32> = size.into();

    SurfaceConfiguration {
        usage:        if SUPPORT_SCREENSHOT {
            TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC
        } else {
            TextureUsages::RENDER_ATTACHMENT
        },
        format:       SURFACE_TEXTURE_FORMAT,
        width:        size.width,
        height:       size.height,
        present_mode: if VSYNC.load(Ordering::Relaxed) || Platform::MOBILE {
            PresentMode::AutoVsync
        } else {
            PresentMode::AutoNoVsync
        },
        alpha_mode:   CompositeAlphaMode::Auto,
        view_formats: vec![],

        desired_maximum_frame_latency: MAX_FRAME_LATENCY.load(Ordering::Relaxed),
    }
}
