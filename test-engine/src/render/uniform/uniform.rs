use bytemuck::Pod;
use wgpu::{
    BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, ShaderStages,
};
use crate::window::{BufferUsages, Window};

use crate::render::device_helper::DeviceHelper;

pub(crate) fn make_bind<T: Pod>(data: &T, layout: &BindGroupLayout) -> BindGroup {
    let device = Window::device();
    let buffer = device.buffer(data, BufferUsages::UNIFORM);
    device.bind(&buffer, layout)
}

pub(crate) fn make_uniform_layout(name: &str, shader: ShaderStages) -> BindGroupLayout {
    Window::device().create_bind_group_layout(&BindGroupLayoutDescriptor {
        label:   name.into(),
        entries: &[BindGroupLayoutEntry {
            binding:    0,
            visibility: shader,
            ty:         BindingType::Buffer {
                ty:                 BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size:   None,
            },
            count:      None,
        }],
    })
}
