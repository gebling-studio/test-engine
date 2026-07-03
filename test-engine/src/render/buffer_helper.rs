use wgpu::Buffer;
use crate::window::Window;

use crate::render::to_bytes::ToBytes;

pub(crate) trait BufferHelper {
    fn update<T: ToBytes>(&self, data: T);
}

impl BufferHelper for Buffer {
    fn update<T: ToBytes>(&self, data: T) {
        Window::queue().write_buffer(self, 0, data.to_bytes());
    }
}
