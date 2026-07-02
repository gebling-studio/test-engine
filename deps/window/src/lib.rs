mod text;
mod window;
mod window_events;

mod app_handler;
mod frame_counter;
pub mod image;
mod screen;
mod screenshot;
pub mod state;
mod surface;
mod vertex_buffer;

pub use app_handler::AppHandler;
pub use bytemuck::cast_slice;
pub use screenshot::*;
pub use state::SURFACE_TEXTURE_FORMAT;
pub use text::*;
pub use vertex_buffer::VertexBuffer;
pub use wgpu::{
    Buffer, BufferUsages, Device, PolygonMode, RenderPass,
    util::{BufferInitDescriptor, DeviceExt},
};
pub use window::*;
pub use window_events::*;
pub use winit::{
    event::{ElementState, MouseButton},
    keyboard::NamedKey,
    window::Theme,
};
