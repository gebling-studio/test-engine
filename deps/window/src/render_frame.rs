use gm::color::Color;
use wgpu::{
    CommandEncoder, LoadOp, Operations, RenderPass, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPassTimestampWrites, StoreOp,
    TextureView,
};

/// One frame's render encoding. Owns the encoder and the render pass
/// together, which `forget_lifetime` makes possible, so the frame can
/// later be split into several passes to sample the already drawn
/// scene. wgpu checks the pass and encoder use order at runtime.
pub struct RenderFrame {
    encoder: CommandEncoder,
    pass:    Option<RenderPass<'static>>,
}

impl RenderFrame {
    pub(crate) fn new(
        mut encoder: CommandEncoder,
        color: &TextureView,
        depth: &TextureView,
        clear_color: Color,
        timestamp_writes: Option<RenderPassTimestampWrites>,
    ) -> Self {
        let pass = encoder
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view:           color,
                    depth_slice:    None,
                    resolve_target: None,
                    ops:            Operations {
                        load:  LoadOp::Clear(wgpu::Color {
                            r: f64::from(clear_color.r),
                            g: f64::from(clear_color.g),
                            b: f64::from(clear_color.b),
                            a: f64::from(clear_color.a),
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view:        depth,
                    depth_ops:   Some(Operations {
                        load:  LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes,
                multiview_mask: None,
            })
            .forget_lifetime();

        Self {
            encoder,
            pass: Some(pass),
        }
    }

    pub fn pass(&mut self) -> &mut RenderPass<'static> {
        self.pass.as_mut().expect("render pass is closed")
    }

    pub(crate) fn finish(mut self) -> CommandEncoder {
        self.pass = None;
        self.encoder
    }
}
