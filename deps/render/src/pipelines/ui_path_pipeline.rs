use std::ops::Range;

use gm::flat::Point;
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, PipelineLayoutDescriptor, PolygonMode, PrimitiveTopology, RenderPass,
    RenderPipeline, ShaderStages, include_wgsl,
};
use window::Window;

use crate::{
    data::PathData,
    device_helper::DeviceHelper,
    uniform::{cached_float_bind, make_uniform_layout},
    vertex_layout::VertexLayout,
};

#[derive(Debug)]
pub struct UIPathPipeline {
    pipeline: RenderPipeline,

    z_pos_layout: BindGroupLayout,
}

impl Default for UIPathPipeline {
    fn default() -> Self {
        let device = Window::device();

        let shader = device.create_shader_module(include_wgsl!("shaders/ui_path.wgsl"));

        let z_pos_layout = make_uniform_layout("path_z_pos_layout", ShaderStages::VERTEX);

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:              Some("Path Pipeline Layout"),
            bind_group_layouts: &[Some(&z_pos_layout), Some(&PathData::uniform_layout())],
            immediate_size:     0,
        });

        let pipeline = device.pipeline(
            "Path Fill Render Pipeline",
            &pipeline_layout,
            &shader,
            PolygonMode::Fill,
            PrimitiveTopology::TriangleStrip,
            &[Point::VERTEX_LAYOUT],
        );

        Self {
            pipeline,
            z_pos_layout,
        }
    }
}

impl UIPathPipeline {
    pub fn draw<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        buffer: &'a Buffer,
        bind_group: &'a BindGroup,
        vertex_range: Range<u32>,
        z_position: f32,
    ) {
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(0, cached_float_bind(z_position, &self.z_pos_layout), &[]);
        render_pass.set_bind_group(1, bind_group, &[]);
        render_pass.set_vertex_buffer(0, buffer.slice(..));
        render_pass.draw(vertex_range, 0..1);
    }
}
