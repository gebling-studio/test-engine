use std::ops::Range;

use refs::main_lock::MainLock;

use bytemuck::{Pod, Zeroable};
use crate::gm::{
    color::Color,
    flat::{Point, Size},
};
use wgpu::{BindGroup, BindGroupLayout, Buffer, BufferUsages, ShaderStages};
use crate::window::Window;

use crate::render::{buffer_helper::BufferHelper, device_helper::DeviceHelper, uniform::make_uniform_layout};

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
    pub(crate) fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub(crate) fn uniform_bind(&self) -> &BindGroup {
        &self.bind
    }

    pub(crate) fn vertex_range(&self) -> Range<u32> {
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

    pub(crate) fn resize(&mut self, position: Point) {
        self.view.position = position;
        self.view.resolution = Window::render_size();
        self.view_buffer.update(self.view);
    }

    pub(crate) fn uniform_layout() -> &'static BindGroupLayout {
        static LAYOUT: MainLock<BindGroupLayout> = MainLock::new();
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
