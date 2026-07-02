use bit_ops::BitOps;
use bytemuck::{Pod, Zeroable};
use gm::{
    color::Color,
    flat::{CornerRadii, Point, Rect, Size},
};
use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

use crate::vertex_layout::VertexLayout;

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod)]
pub struct UIImageInstance {
    pub position:     Point,
    pub size:         Size,
    pub border_color: Color,
    pub border_width: f32,
    pub corner_radii: CornerRadii,
    pub z_position:   f32,
    pub flags:        u32,
    pub scale:        f32,
}

impl UIImageInstance {
    const FLIP_X_FLAG: u32 = 0;
    const FLIP_Y_FLAG: u32 = 1;

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rect: Rect,
        border_color: Color,
        border_width: f32,
        corner_radii: CornerRadii,
        z_position: f32,
        flip_x: bool,
        flip_y: bool,
        scale: f32,
    ) -> Self {
        let mut result = Self {
            position: rect.origin,
            size: rect.size,
            corner_radii,
            z_position,
            flags: 0,
            scale,
            border_color,
            border_width,
        };

        result.set_flip_x(flip_x);
        result.set_flip_y(flip_y);

        result
    }

    fn set_flag(&mut self, bit: u32, value: bool) {
        if value {
            self.flags = self.flags.set_bit(bit);
        } else {
            self.flags = self.flags.clear_bit(bit);
        }
    }

    fn set_flip_x(&mut self, flip_x: bool) {
        self.set_flag(Self::FLIP_X_FLAG, flip_x);
    }

    fn set_flip_y(&mut self, flip_x: bool) {
        self.set_flag(Self::FLIP_Y_FLAG, flip_x);
    }
}

impl VertexLayout for UIImageInstance {
    const ATTRIBS: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![2 => Float32x2, 3 => Float32x2, 4 => Float32x4, 5 => Float32, 6 => Float32x4, 7 => Float32, 8 => Uint32, 9 => Float32];
    const VERTEX_LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as BufferAddress,
        step_mode:    VertexStepMode::Instance,
        attributes:   Self::ATTRIBS,
    };
}
