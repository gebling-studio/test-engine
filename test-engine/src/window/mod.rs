mod text;
mod window;
mod window_events;

mod app_handler;
mod frame_counter;
pub mod image;
mod render_frame;
mod screen;
mod screenshot;
pub mod state;
mod surface;
mod vertex_buffer;

pub use self::app_handler::AppHandler;
pub use bytemuck::cast_slice;
pub use self::render_frame::RenderFrame;
pub use self::screenshot::*;
pub use self::state::SURFACE_TEXTURE_FORMAT;
pub use self::text::*;
pub use self::vertex_buffer::VertexBuffer;
pub use wgpu::{
    Buffer, BufferUsages, Device, PolygonMode, RenderPass,
    util::{BufferInitDescriptor, DeviceExt},
};
pub use self::window::*;
pub use self::window_events::*;
pub use winit::{
    event::{ElementState, MouseButton},
    keyboard::NamedKey,
    window::Theme,
};
