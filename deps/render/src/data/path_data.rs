use std::{ops::Range, sync::OnceLock};

use bytemuck::{Pod, Zeroable};
use gm::{
    color::Color,
    flat::{Point, Size},
};
use wgpu::{BindGroup, BindGroupLayout, Buffer, BufferUsages, ShaderStages};
use window::Window;

use crate::{buffer_helper::BufferHelper, device_helper::DeviceHelper, uniform::make_uniform_layout};

#[repr(C)]
#[derive(Debug, Copy, Clone, Zeroable, Pod, PartialEq)]
struct PathView {
    position:   Point,
    resolution: Size,
    color:      Color,
    z_position: f32,
    _padding:   [u32; 3],
}

#[derive(Debug)]
pub struct PathData {
    view:         PathView,
    buffer:       Buffer,
    view_buffer:  Buffer,
    bind:         BindGroup,
    vertex_range: Range<u32>,
}

impl PathData {
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn uniform_bind(&self) -> &BindGroup {
        &self.bind
    }

    pub fn vertex_range(&self) -> Range<u32> {
        self.vertex_range.clone()
    }

    pub fn new(color: Color, resolution: Size, position: Point, z_position: f32, points: &[Point]) -> Self {
        let device = Window::device();

        let buffer = device.buffer(points, BufferUsages::VERTEX);

        let view = PathView {
            position,
            resolution,
            color,
            z_position,
            _padding: [0; 3],
        };

        let view_buffer = device.buffer(&view, BufferUsages::UNIFORM | BufferUsages::COPY_DST);
        let bind = device.bind(&view_buffer, Self::uniform_layout());

        Self {
            view,
            buffer,
            view_buffer,
            bind,
            vertex_range: 0..u32::try_from(points.len()).unwrap(),
        }
    }

    pub fn resize(&mut self, position: Point) {
        self.view.position = position;
        self.view.resolution = Window::render_size();
        self.view_buffer.update(self.view);
    }

    pub fn uniform_layout() -> &'static BindGroupLayout {
        static LAYOUT: OnceLock<BindGroupLayout> = OnceLock::new();
        LAYOUT.get_or_init(|| make_uniform_layout("path_view_layout", ShaderStages::VERTEX_FRAGMENT))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // Web requirements
        assert_eq!(size_of::<PathView>() % 16, 0);
    }
}
