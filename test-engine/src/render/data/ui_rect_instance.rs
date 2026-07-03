use bytemuck::{Pod, Zeroable};
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

use crate::{
    gm::{
        color::Color,
        flat::{CornerRadii, Point, Rect, Size},
    },
    render::vertex_layout::VertexLayout,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct UIRectInstance {
    pub position:     Point,
    pub size:         Size,
    pub color:        Color,
    pub border_color: Color,
    pub border_width: f32,
    pub corner_radii: CornerRadii,
    pub z_position:   f32,
    pub scale:        f32,
}

impl UIRectInstance {
    pub fn new(
        rect: Rect,
        color: Color,
        border_color: Color,
        border_width: f32,
        corner_radii: CornerRadii,
        z_position: f32,
        scale: f32,
    ) -> Self {
        Self {
            position: rect.origin,
            size: rect.size,
            color,
            border_color,
            border_width,
            corner_radii,
            z_position,
            scale,
        }
    }
}

impl VertexLayout for UIRectInstance {
    const ATTRIBS: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![2 => Float32x2, 3 => Float32x2, 4 => Float32x4, 5 => Float32x4, 6 => Float32, 7 => Float32x4, 8 => Float32, 9 => Float32];
    const VERTEX_LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode:    VertexStepMode::Instance,
        attributes:   Self::ATTRIBS,
    };
}
