use std::{
    cell::RefCell,
    f64,
    sync::mpsc::{Receiver, Sender, channel},
};

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
    surface::Surface, window::surface_config_with_size,
};

type ReadDisplayRequest = Sender<Screenshot>;

#[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
pub const SURFACE_TEXTURE_FORMAT: TextureFormat = TextureFormat::Bgra8UnormSrgb;
#[cfg(any(target_os = "android", target_arch = "wasm32"))]
pub const SURFACE_TEXTURE_FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;

pub struct State {
    pub(crate) clear_color: Color,

    read_display_request:     RefCell<Option<ReadDisplayRequest>>,
    pub(crate) frame_counter: FrameCounter,

    offscreen_texture: Option<wgpu::Texture>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            clear_color:          GRAY_BLUE,
            read_display_request: RefCell::default(),
            frame_counter:        FrameCounter::default(),
            offscreen_texture:    None,
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

        if window.surface.is_none() {
            window.surface = Surface::new(
                &window.instance,
                &window.adapter,
                &window.device,
                surface_config_with_size((new_size.width.lossy_convert(), new_size.height.lossy_convert())),
                window.winit_window.clone(),
            )
            .expect("Failed to create surface")
            .into();

            info!("surface created");
        }

        let surface = window.surface.as_mut().unwrap();

        surface.depth_texture = Texture::create_depth_texture(
            &window.device,
            (new_size.width.lossy_convert(), new_size.height.lossy_convert()).into(),
            "depth_texture",
        );
        surface.presentable.configure(
            &window.device,
            &surface_config_with_size((new_size.width.lossy_convert(), new_size.height.lossy_convert())),
        );

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

        AppHandler::current().te_window_events.update();

        if Window::current().title_set {
            return;
        }

        if self.frame_counter.update() {
            let a = format!(
                "{:.2}ms frame {:.1} FPS",
                self.frame_counter.frame_time * 1000.0,
                self.frame_counter.fps
            );
            if Platform::DESKTOP {
                let size = Window::render_size();
                Window::winit_window().set_title(&format!("{a} {} x {}", size.width, size.height));
            }
        }
    }

    pub fn render(&mut self) -> Result<()> {
        let Some(ref surface) = Window::current().surface else {
            return Ok(());
        };

        #[cfg(desktop)]
        if Window::is_resizing() {
            return Ok(());
        }

        Window::next_render_frame();

        let surface_texture = if Window::headless() {
            self.ensure_offscreen_texture();
            None
        } else {
            match surface.presentable.get_current_texture() {
                CurrentSurfaceTexture::Success(tex) => Some(tex),
                CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return Ok(()),
                CurrentSurfaceTexture::Suboptimal(_) | CurrentSurfaceTexture::Outdated => {
                    warn!("Surface is outdated, reconfiguring");
                    Window::reconfigure_surface();
                    return Ok(());
                }
                CurrentSurfaceTexture::Lost => {
                    warn!("Surface is lost, recreating");
                    Window::current().surface = None;
                    Self::resize();
                    return Ok(());
                }
                CurrentSurfaceTexture::Validation => {
                    panic!("Validation error in get_current_texture")
                }
            }
        };

        let texture = match &surface_texture {
            Some(surface_texture) => &surface_texture.texture,
            None => self.offscreen_texture.as_ref().unwrap(),
        };

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = Window::device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

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
                    view:        &surface.depth_texture.view,
                    depth_ops:   Some(wgpu::Operations {
                        load:  wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set:      None,
                timestamp_writes:         None,
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

        if let Some(surface_texture) = surface_texture {
            surface_texture.present();
        }

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
