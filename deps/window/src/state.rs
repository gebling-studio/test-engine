use std::{
    cell::RefCell,
    f64,
    sync::mpsc::{Receiver, Sender, channel},
};

use web_time::Instant;

use anyhow::Result;
use gm::{
    LossyConvert,
    color::{Color, GRAY_BLUE},
};
use log::{info, warn};
use plat::Platform;
use refs::manage::DataManager;
use wgpu::{CurrentSurfaceTexture, TextureFormat};

use crate::{
    Font, Screenshot, Window, app_handler::AppHandler, frame_counter::FrameCounter, image::Texture,
    screen::Screen, surface::Surface, window::surface_config_with_size,
};

type ReadDisplayRequest = Sender<Screenshot>;

enum RenderTarget {
    Skip,
    Offscreen,
    Surface(wgpu::SurfaceTexture),
}

/// Start and end timestamp of the render pass.
#[cfg(feature = "bench")]
const GPU_TIMESTAMPS: u32 = 2;
#[cfg(feature = "bench")]
const GPU_TIMESTAMP_BYTES: u64 = GPU_TIMESTAMPS as u64 * 8;

/// GPU-side resources for one render pass timestamp pair. Created lazily on the
/// first benchmarked frame and reused after.
#[cfg(feature = "bench")]
struct GpuTimer {
    query_set: wgpu::QuerySet,
    resolve:   wgpu::Buffer,
    readback:  wgpu::Buffer,
}

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
pub const SURFACE_TEXTURE_FORMAT: TextureFormat = TextureFormat::Bgra8UnormSrgb;
#[cfg(any(target_os = "android", target_arch = "wasm32"))]
pub const SURFACE_TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;

pub struct State {
    pub(crate) clear_color: Color,

    read_display_request:     RefCell<Option<ReadDisplayRequest>>,
    pub(crate) frame_counter: FrameCounter,

    offscreen_texture: Option<wgpu::Texture>,
    depth_texture:     Option<Texture>,

    update_work: f32,

    /// CPU time of the last frame's update and render encoding, excluding the
    /// wait for a drawable and present. Unlike `frame_time` it is not capped
    /// by vsync or the compositor.
    pub(crate) frame_work_time: f32,

    /// GPU execution time of the last frame's render pass, in seconds.
    #[cfg(feature = "bench")]
    pub(crate) frame_gpu_time: f32,

    #[cfg(feature = "bench")]
    gpu_timer: Option<GpuTimer>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            clear_color:          GRAY_BLUE,
            read_display_request: RefCell::default(),
            frame_counter:        FrameCounter::default(),
            offscreen_texture:    None,
            depth_texture:        None,
            update_work:          0.0,
            frame_work_time:      0.0,
            #[cfg(feature = "bench")]
            frame_gpu_time:       0.0,
            #[cfg(feature = "bench")]
            gpu_timer:            None,
        }
    }
}

impl State {
    pub fn resize() {
        let new_size = Window::render_size();

        if new_size.width == 0.0 || new_size.height == 0.0 {
            warn!("Zero size");
            return;
        }

        let window = Window::current();

        if let Screen::Windowed { winit_window, surface } = &mut window.screen {
            if surface.is_none() {
                *surface = Surface::new(
                    &window.instance,
                    &window.adapter,
                    &window.device,
                    surface_config_with_size((
                        new_size.width.lossy_convert(),
                        new_size.height.lossy_convert(),
                    )),
                    winit_window.clone(),
                )
                .expect("Failed to create surface")
                .into();

                info!("surface created");
            }

            surface.as_ref().unwrap().presentable.configure(
                &window.device,
                &surface_config_with_size((
                    new_size.width.lossy_convert(),
                    new_size.height.lossy_convert(),
                )),
            );
        }

        let queue = Window::queue();

        for font in Font::storage_mut().values_mut() {
            font.brush.resize_view(new_size.width, new_size.height, queue);
        }

        AppHandler::current().te_window_events.resize(
            Window::inner_position(),
            Window::outer_position(),
            Window::inner_size(),
            Window::outer_size(),
        );

        #[cfg(desktop)]
        {
            window.is_resizing = false;
        }
    }

    pub fn update(&mut self) {
        #[cfg(desktop)]
        if Window::is_resizing() {
            return;
        }

        let work_started = Instant::now();
        AppHandler::current().te_window_events.update();
        self.update_work = work_started.elapsed().as_secs_f32();

        if Window::current().title_set {
            return;
        }

        if self.frame_counter.update() {
            let a = format!(
                "{:.2}ms frame {:.1} FPS",
                self.frame_counter.frame_time * 1000.0,
                self.frame_counter.fps
            );
            if Platform::DESKTOP
                && let Some(window) = Window::winit_window()
            {
                let size = Window::render_size();
                window.set_title(&format!("{a} {} x {}", size.width, size.height));
            }
        }
    }

    /// Where the next frame goes. `Skip` means no frame can be rendered right
    /// now - the surface is missing, occluded or being recovered.
    fn acquire_render_target() -> RenderTarget {
        match &Window::current().screen {
            Screen::Headless { .. } => RenderTarget::Offscreen,
            Screen::Windowed { surface: None, .. } => RenderTarget::Skip,
            Screen::Windowed {
                surface: Some(surface),
                ..
            } => match surface.presentable.get_current_texture() {
                CurrentSurfaceTexture::Success(tex) => RenderTarget::Surface(tex),
                CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => RenderTarget::Skip,
                CurrentSurfaceTexture::Suboptimal(_) | CurrentSurfaceTexture::Outdated => {
                    warn!("Surface is outdated, reconfiguring");
                    Window::reconfigure_surface();
                    RenderTarget::Skip
                }
                CurrentSurfaceTexture::Lost => {
                    warn!("Surface is lost, recreating");
                    if let Screen::Windowed { surface, .. } = &mut Window::current().screen {
                        *surface = None;
                    }
                    Self::resize();
                    RenderTarget::Skip
                }
                CurrentSurfaceTexture::Validation => {
                    panic!("Validation error in get_current_texture")
                }
            },
        }
    }

    pub fn render(&mut self) -> Result<()> {
        #[cfg(desktop)]
        if Window::is_resizing() {
            return Ok(());
        }

        let surface_texture = match Self::acquire_render_target() {
            RenderTarget::Skip => return Ok(()),
            RenderTarget::Offscreen => None,
            RenderTarget::Surface(tex) => Some(tex),
        };

        Window::next_render_frame();

        let work_started = Instant::now();

        if surface_texture.is_none() {
            self.ensure_offscreen_texture();
        }
        self.ensure_depth_texture();

        #[cfg(feature = "bench")]
        self.ensure_gpu_timer();

        let texture = match &surface_texture {
            Some(surface_texture) => &surface_texture.texture,
            None => self.offscreen_texture.as_ref().unwrap(),
        };

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = Window::device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        #[cfg(feature = "bench")]
        let timestamp_writes = Some(wgpu::RenderPassTimestampWrites {
            query_set:                     &self.gpu_timer.as_ref().expect("gpu timer ensured").query_set,
            beginning_of_pass_write_index: Some(0),
            end_of_pass_write_index:       Some(1),
        });
        #[cfg(not(feature = "bench"))]
        let timestamp_writes = None;

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label:                    Some("Render Pass"),
                color_attachments:        &[Some(wgpu::RenderPassColorAttachment {
                    view:           &view,
                    depth_slice:    None,
                    resolve_target: None,
                    ops:            wgpu::Operations {
                        load:  wgpu::LoadOp::Clear(wgpu::Color {
                            r: f64::from(self.clear_color.r),
                            g: f64::from(self.clear_color.g),
                            b: f64::from(self.clear_color.b),
                            a: f64::from(self.clear_color.a),
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view:        &self.depth_texture.as_ref().unwrap().view,
                    depth_ops:   Some(wgpu::Operations {
                        load:  wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set:      None,
                timestamp_writes,
                multiview_mask:           None,
            });

            AppHandler::current().te_window_events.render(&mut render_pass);
        }

        #[cfg(not_wasm)]
        let buffer = if self.read_display_request.borrow().is_some() {
            Some(Self::read_screen(&mut encoder, texture))
        } else {
            None
        };

        Window::queue().submit(std::iter::once(encoder.finish()));

        self.frame_work_time = self.update_work + work_started.elapsed().as_secs_f32();

        if let Some(surface_texture) = surface_texture {
            surface_texture.present();
        }

        // After the work timer closes and the frame is presented, so resolving
        // and the blocking readback never inflate the CPU measurement.
        #[cfg(feature = "bench")]
        self.read_gpu_time()?;

        #[cfg(not_wasm)]
        if let Some(buffer_sender) = self.read_display_request.take() {
            let (sender, receiver) = channel();

            let Some(buffer) = buffer else {
                return Ok(());
            };

            let buffer_slice = buffer.0.slice(..);

            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                sender.send(result).unwrap();
            });

            hreads::spawn(async move {
                let _ = receiver.recv().unwrap();
                Self::deliver_screenshot(buffer, &buffer_sender);
            });
        }

        Ok(())
    }

    #[cfg(feature = "bench")]
    fn ensure_gpu_timer(&mut self) {
        if self.gpu_timer.is_some() {
            return;
        }

        let device = Window::device();
        self.gpu_timer = Some(GpuTimer {
            query_set: device.create_query_set(&wgpu::QuerySetDescriptor {
                label: Some("bench timestamps"),
                ty:    wgpu::QueryType::Timestamp,
                count: GPU_TIMESTAMPS,
            }),
            resolve:   device.create_buffer(&wgpu::BufferDescriptor {
                label:              Some("bench timestamp resolve"),
                size:               wgpu::QUERY_RESOLVE_BUFFER_ALIGNMENT,
                usage:              wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            }),
            readback:  device.create_buffer(&wgpu::BufferDescriptor {
                label:              Some("bench timestamp readback"),
                size:               GPU_TIMESTAMP_BYTES,
                usage:              wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
        });
    }

    /// Reads back the render pass timestamps resolved during `render` and blocks
    /// until the GPU reports them. The blocking poll runs after the work timer
    /// has closed, so it costs only wall-clock throughput, not `frame_work_time`.
    #[cfg(feature = "bench")]
    fn read_gpu_time(&mut self) -> Result<()> {
        let gpu_seconds = {
            let Some(timer) = self.gpu_timer.as_ref() else {
                return Ok(());
            };

            // Wait for the render submission to finish so Metal commits the
            // end-of-pass timestamp to the query set before we resolve it.
            Window::device().poll(wgpu::PollType::wait_indefinitely())?;

            let mut encoder = Window::device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("bench timestamp resolve"),
            });
            encoder.resolve_query_set(&timer.query_set, 0..GPU_TIMESTAMPS, &timer.resolve, 0);
            encoder.copy_buffer_to_buffer(&timer.resolve, 0, &timer.readback, 0, GPU_TIMESTAMP_BYTES);
            Window::queue().submit(std::iter::once(encoder.finish()));

            let slice = timer.readback.slice(..);
            slice.map_async(wgpu::MapMode::Read, |result| {
                result.expect("bench timestamp readback map failed");
            });
            Window::device().poll(wgpu::PollType::wait_indefinitely())?;

            let period = Window::queue().get_timestamp_period();
            let stamps: [u64; 2] = {
                let data = slice.get_mapped_range();
                bytemuck::pod_read_unaligned(&data[..])
            };
            timer.readback.unmap();

            let ticks: f32 = stamps[1].saturating_sub(stamps[0]).lossy_convert();
            ticks * period / 1.0e9
        };

        self.frame_gpu_time = gpu_seconds;
        Ok(())
    }

    fn ensure_depth_texture(&mut self) {
        let size: gm::flat::Size<u32> = Window::render_size().lossy_convert();

        let up_to_date = self.depth_texture.as_ref().is_some_and(|texture| texture.size == size);

        if up_to_date {
            return;
        }

        self.depth_texture = Some(Texture::create_depth_texture(
            Window::device(),
            size,
            "depth_texture",
        ));
    }

    fn ensure_offscreen_texture(&mut self) {
        let size = Window::render_size();
        let width: u32 = size.width.lossy_convert();
        let height: u32 = size.height.lossy_convert();

        let up_to_date = self
            .offscreen_texture
            .as_ref()
            .is_some_and(|texture| texture.size().width == width && texture.size().height == height);

        if up_to_date {
            return;
        }

        self.offscreen_texture = Some(Window::device().create_texture(&wgpu::TextureDescriptor {
            label:           Some("Offscreen Render Texture"),
            size:            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count:    1,
            dimension:       wgpu::TextureDimension::D2,
            format:          SURFACE_TEXTURE_FORMAT,
            usage:           wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats:    &[],
        }));
    }

    #[cfg(not_wasm)]
    fn deliver_screenshot(buffer: (wgpu::Buffer, gm::flat::Size<u32>), sender: &ReadDisplayRequest) {
        let (buff, size) = buffer;

        if size.width == 0 || size.height == 0 {
            sender.send(Screenshot::new(vec![], size)).unwrap();
            return;
        }

        let width = usize::try_from(size.width).unwrap();
        let height = usize::try_from(size.height).unwrap();
        let real_row_bytes = width * std::mem::size_of::<gm::color::U8Color>();
        let row_bytes =
            real_row_bytes.next_multiple_of(usize::try_from(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT).unwrap());

        let bytes: &[u8] = &buff.slice(..).get_mapped_range();

        let mut data: Vec<gm::color::U8Color> = Vec::with_capacity(width * height);

        for row in bytes.chunks_exact(row_bytes) {
            data.extend(
                bytemuck::cast_slice::<u8, gm::color::U8Color>(&row[..real_row_bytes])
                    .iter()
                    .map(|color| color.bgra_to_rgba()),
            );
        }

        sender.send(Screenshot::new(data, size)).unwrap();
    }

    pub fn request_read_display(&self) -> Receiver<Screenshot> {
        let mut request = self.read_display_request.borrow_mut();

        let (s, r) = channel();
        request.replace(s);
        r
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn read_screen(
        encoder: &mut wgpu::CommandEncoder,
        texture: &wgpu::Texture,
    ) -> (wgpu::Buffer, gm::flat::Size<u32>) {
        if !crate::SUPPORT_SCREENSHOT {
            return (
                Window::device().create_buffer(&wgpu::BufferDescriptor {
                    label:              Some("Empty Buffer"),
                    size:               0,
                    usage:              wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }),
                gm::flat::Size::default(),
            );
        }

        let screen_width_bytes: u64 = u64::from(texture.size().width) * std::mem::size_of::<u32>() as u64;

        let width_bytes = screen_width_bytes.next_multiple_of(u64::from(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT));

        let buffer = Window::device().create_buffer(&wgpu::BufferDescriptor {
            label:              Some("Read Screen Buffer"),
            size:               width_bytes * u64::from(texture.size().height),
            usage:              wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset:         0,
                    bytes_per_row:  u32::try_from(width_bytes).unwrap().into(),
                    rows_per_image: texture.size().height.into(),
                },
            },
            wgpu::Extent3d {
                width:                 texture.size().width,
                height:                texture.size().height,
                depth_or_array_layers: 1,
            },
        );

        let size: gm::flat::Size<u32> = gm::flat::Size::new(texture.size().width, texture.size().height);

        (buffer, size)
    }
}
