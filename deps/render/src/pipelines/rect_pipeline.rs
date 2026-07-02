use bytemuck::Pod;
use gm::flat::{Point, Vertex2D};
use indexmap::IndexMap;
use refs::Weak;
use wgpu::{
    Buffer, PipelineLayoutDescriptor, PrimitiveTopology, RenderPass, RenderPipeline, ShaderModuleDescriptor,
    ShaderSource, ShaderStages,
};
use window::{PolygonMode, Window, image::Image};

use crate::{
    device_helper::DeviceHelper,
    pipelines::pipeline_type::PipelineType,
    uniform::{UniformBind, make_uniform_layout},
    vec_buffer::VecBuffer,
    vertex_layout::VertexLayout,
};

pub struct RectPipeline<
    const TYPE: PipelineType,
    const NAME: &'static str,
    const SHADER_CODE: &'static str,
    View,
    Instance,
> {
    pipeline: RenderPipeline,

    vertex_buffer: Buffer,

    view: UniformBind<View>,

    // Entries are never removed. Managed images live for the whole process,
    // see docs/refs.md, so a key cannot die.
    instances: IndexMap<Weak<Image>, VecBuffer<Instance>>,
}

impl<
    const TYPE: PipelineType,
    const NAME: &'static str,
    const SHADER_CODE: &'static str,
    View: Default + Pod,
    Instance: VertexLayout,
> Default for RectPipeline<TYPE, NAME, SHADER_CODE, View, Instance>
{
    fn default() -> Self {
        let device = Window::device();

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label:  Some(&format!("{NAME}.wgsl")),
            source: ShaderSource::Wgsl(SHADER_CODE.into()),
        });

        let sprite_view_layout =
            make_uniform_layout(&format!("{NAME}_uniform_layout"), ShaderStages::VERTEX_FRAGMENT);

        let mut bind_group_layouts = vec![Some(&sprite_view_layout)];

        if TYPE.image() {
            bind_group_layouts.push(Some(Image::uniform_layout()));
        }

        let uniform_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label:              Some(&format!("{NAME}_pipeline_layout")),
            bind_group_layouts: &bind_group_layouts,
            immediate_size:     0,
        });

        let pipeline = if TYPE.image() {
            device.pipeline(
                &format!("{NAME}_pipeline"),
                &uniform_layout,
                &shader,
                PolygonMode::Fill,
                PrimitiveTopology::TriangleStrip,
                &[Vertex2D::VERTEX_LAYOUT, Instance::VERTEX_LAYOUT],
            )
        } else {
            device.pipeline(
                &format!("{NAME}_pipeline"),
                &uniform_layout,
                &shader,
                PolygonMode::Fill,
                PrimitiveTopology::TriangleStrip,
                &[Point::VERTEX_LAYOUT, Instance::VERTEX_LAYOUT],
            )
        };

        Self {
            pipeline,
            vertex_buffer: TYPE.vertex_buffer(device),
            view: sprite_view_layout.into(),
            instances: IndexMap::default(),
        }
    }
}

impl<
    const TYPE: PipelineType,
    const NAME: &'static str,
    const SHADER_CODE: &'static str,
    View: Pod + PartialEq,
    Instance: Pod,
> RectPipeline<TYPE, NAME, SHADER_CODE, View, Instance>
{
    pub fn add(&mut self, instance: Instance) {
        assert!(TYPE.color());
        self.instances.entry(Weak::default()).or_default().push(instance);
    }

    pub fn add_with_image(&mut self, instance: Instance, image: Weak<Image>) {
        assert!(TYPE.image());
        self.instances.entry(image).or_default().push(instance);
    }

    pub fn draw<'a>(&'a mut self, render_pass: &mut RenderPass<'a>, view: View) {
        if self.instances.is_empty() {
            return;
        }

        render_pass.set_pipeline(&self.pipeline);

        self.view.update(view);

        for (image, instances) in &mut self.instances {
            if instances.is_empty() {
                continue;
            }

            instances.load();

            render_pass.set_bind_group(0, self.view.bind(), &[]);

            if TYPE.image() {
                render_pass.set_bind_group(1, image.bind(), &[]);
            }

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instances.slice());

            render_pass.draw(TYPE.vertex_range(), 0..instances.len());
        }
    }
}
