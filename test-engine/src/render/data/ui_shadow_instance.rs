use bytemuck::{Pod, Zeroable};
use crate::gm::{
    color::Color,
    flat::{CornerRadii, Point, Size},
};
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

use crate::render::vertex_layout::VertexLayout;

/// A drop shadow under a rounded rect. `position` and `size` are the
/// casting rect, the quad is expanded by `blur` in the shader.
#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub(crate) struct UIShadowInstance {
    pub position:     Point,
    pub size:         Size,
    pub color:        Color,
    pub corner_radii: CornerRadii,
    pub blur:         f32,
    pub z_position:   f32,
    pub scale:        f32,
}

impl VertexLayout for UIShadowInstance {
    const ATTRIBS: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![2 => Float32x2, 3 => Float32x2, 4 => Float32x4, 5 => Float32x4, 6 => Float32, 7 => Float32, 8 => Float32];
    const VERTEX_LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode:    VertexStepMode::Instance,
        attributes:   Self::ATTRIBS,
    };
}
