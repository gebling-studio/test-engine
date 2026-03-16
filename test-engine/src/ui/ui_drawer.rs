use std::ops::{Deref, DerefMut};

use gm::{
    LossyConvert,
    color::{CLEAR, TURQUOISE},
    flat::Rect,
};
use refs::main_lock::MainLock;
use render::{
    UIGradientPipeline, UIImageRectPipepeline,
    data::{RectView, UIGradientInstance, UIImageInstance, UIRectInstance},
};
use ui::{
    DrawingView, ImageView, Label, TextAlignment, UIManager, View, ViewData, ViewFrame, ViewLayout,
    ViewSubviews,
};
use wgpu::RenderPass;
use wgpu_text::glyph_brush::{BuiltInLineBreaker, HorizontalAlign, Layout, Section, Text, VerticalAlign};
use window::{Font, Window};

use crate::pipelines::Pipelines;

static GRADIENT_DRAWER: MainLock<UIGradientPipeline> = MainLock::new();
static IMAGE_RECT_DRAWER: MainLock<UIImageRectPipepeline> = MainLock::new();
// static UI_PATH_DRAWER: MainLock<UIPathPipeline> = MainLock::new();

struct TextGroup<'a> {
    sections: Vec<Section<'a>>,
    scissor:  Option<[u32; 4]>,
}

impl TextGroup<'_> {
    fn new(scissor: Option<[u32; 4]>) -> Self {
        Self {
            sections: vec![],
            scissor,
        }
    }
}

pub struct UIDrawer;

impl UIDrawer {
    pub(crate) fn update() {
        UIManager::commit_animations();
        Self::update_view(UIManager::root_view().deref_mut());
    }

    pub(crate) fn draw<'a>(pass: &mut RenderPass<'a>) {
        let resolution = UIManager::window_resolution();
        let display_rect = [
            0u32,
            0u32,
            resolution.width.lossy_convert(),
            resolution.height.lossy_convert(),
        ];

        let rect_view = RectView {
            resolution,
            _padding: 0,
        };
        let debug_frames = UIManager::should_draw_debug_frames();
        let scale = UIManager::scale();

        let mut text_groups: Vec<TextGroup<'a>> = vec![TextGroup::new(None)];
        let mut scissor_stack: Vec<[u32; 4]> = vec![];

        Self::draw_view(
            pass,
            UIManager::root_view_static(),
            &mut text_groups,
            debug_frames,
            scale,
            rect_view,
            &mut scissor_stack,
        );

        Self::flush_pipelines(pass, rect_view);
        scissor_checked(
            pass,
            display_rect[0],
            display_rect[1],
            display_rect[2],
            display_rect[3],
        );

        for group in text_groups {
            if group.sections.is_empty() {
                continue;
            }
            let [x, y, w, h] = group.scissor.unwrap_or(display_rect);
            scissor_checked(pass, x, y, w, h);
            Font::default()
                .brush
                .queue(Window::device(), Window::queue(), group.sections)
                .unwrap();
            Font::default().brush.draw(pass);
        }

        scissor_checked(
            pass,
            display_rect[0],
            display_rect[1],
            display_rect[2],
            display_rect[3],
        );
    }

    fn flush_pipelines(pass: &mut RenderPass, rect_view: RectView) {
        Pipelines::rect().draw(pass, rect_view);
        IMAGE_RECT_DRAWER.get_mut().draw(pass, rect_view);
        GRADIENT_DRAWER.get_mut().draw(pass, rect_view);
    }

    fn update_view(view: &mut dyn View) {
        if view.is_hidden() {
            return;
        }
        view.layout();
        view.calculate_absolute_frame();
        view.update();
        view.trigger_events();
        for mut view in view.subviews_weak() {
            Self::update_view(view.deref_mut());
        }
    }

    #[allow(clippy::too_many_lines)]
    fn draw_view<'a>(
        pass: &mut RenderPass<'a>,
        view: &'a dyn View,
        text_groups: &mut Vec<TextGroup<'a>>,
        debug_frames: bool,
        scale: f32,
        rect_view: RectView,
        scissor_stack: &mut Vec<[u32; 4]>,
    ) {
        let frame = *view.absolute_frame();

        if view.is_hidden() || frame.size.has_no_area() {
            return;
        }

        view.before_render(pass);

        let clips = view.clips_to_bounds();

        if clips {
            Self::flush_pipelines(pass, rect_view);

            let pf = frame * scale;
            let vp_w = rect_view.resolution.width.lossy_convert();
            let vp_h = rect_view.resolution.height.lossy_convert();
            let x = pf.x().max(0.0).lossy_convert();
            let y = pf.y().max(0.0).lossy_convert();
            let pf_max_x: u32 = pf.max_x().lossy_convert();
            let pf_max_y: u32 = pf.max_y().lossy_convert();
            let max_x = pf_max_x.min(vp_w);
            let max_y = pf_max_y.min(vp_h);
            let w = max_x.saturating_sub(x);
            let h = max_y.saturating_sub(y);

            if w == 0 || h == 0 {
                return;
            }

            let new_scissor = if let Some(&parent) = scissor_stack.last() {
                Self::intersect_scissor(parent, [x, y, w, h])
            } else {
                [x, y, w, h]
            };

            if new_scissor[2] == 0 || new_scissor[3] == 0 {
                return;
            }

            scissor_stack.push(new_scissor);
            scissor_checked(
                pass,
                new_scissor[0],
                new_scissor[1],
                new_scissor[2],
                new_scissor[3],
            );
            text_groups.push(TextGroup::new(Some(new_scissor)));
        }

        if view.end_gradient_color().a > 0.0 {
            GRADIENT_DRAWER.get_mut().add(UIGradientInstance {
                position: frame.origin,
                size: frame.size,
                start_color: *view.color(),
                end_color: *view.end_gradient_color(),
                corner_radius: view.corner_radius(),
                z_position: view.z_position(),
                scale,
            });
        } else if view.color().a > 0.0 || view.border_color().a > 0.0 {
            Pipelines::rect().add(UIRectInstance::new(
                frame,
                *view.color(),
                *view.border_color(),
                view.border_width(),
                view.corner_radius(),
                view.z_position(),
                scale,
            ));
        }

        if let Some(image_view) = view.as_any().downcast_ref::<ImageView>() {
            if image_view.image().is_ok() {
                let image = image_view.image();

                IMAGE_RECT_DRAWER.get_mut().add_with_image(
                    UIImageInstance::new(
                        image_view.image_frame(),
                        *view.border_color(),
                        view.border_width(),
                        view.corner_radius(),
                        view.z_position(),
                        image_view.flip_x,
                        image_view.flip_y,
                        scale,
                    ),
                    image,
                );
            }
        } else if let Some(label) = view.as_any().downcast_ref::<Label>()
            && !label.text.is_empty()
        {
            let sections = &mut text_groups.last_mut().unwrap().sections;
            Self::draw_label(&frame, label, sections, scale);
        } else if let Some(drawing_view) = view.as_any().downcast_ref::<DrawingView>() {
            for _path in drawing_view.paths().iter().rev() {
                // UI_PATH_DRAWER.get_mut().draw(
                //     pass,
                //     path.buffer(),
                //     path.uniform_bind(),
                //     path.vertex_range(),
                //     drawing_view.z_position() -
                // UIManager::additional_z_offset(), );
            }
        }

        if debug_frames {
            for rect in frame.to_borders(2.0) {
                Pipelines::rect().add(UIRectInstance::new(
                    rect,
                    TURQUOISE,
                    CLEAR,
                    0.0,
                    0.0,
                    view.z_position() - 0.2,
                    scale,
                ));
            }
        }

        let root_frame = UIManager::root_view_static().frame();

        for view in view.subviews() {
            if view.dont_hide() || view.absolute_frame().intersects(root_frame) {
                Self::draw_view(
                    pass,
                    view.deref(),
                    text_groups,
                    debug_frames,
                    scale,
                    rect_view,
                    scissor_stack,
                );
            }
        }

        if clips {
            Self::flush_pipelines(pass, rect_view);
            scissor_stack.pop();

            let vp_w = rect_view.resolution.width.lossy_convert();
            let vp_h = rect_view.resolution.height.lossy_convert();

            if let Some(&parent) = scissor_stack.last() {
                scissor_checked(pass, parent[0], parent[1], parent[2], parent[3]);
                text_groups.push(TextGroup::new(Some(parent)));
            } else {
                scissor_checked(pass, 0, 0, vp_w, vp_h);
                text_groups.push(TextGroup::new(None));
            }
        }
    }

    fn intersect_scissor(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
        let x = a[0].max(b[0]);
        let y = a[1].max(b[1]);
        let max_x = (a[0] + a[2]).min(b[0] + b[2]);
        let max_y = (a[1] + a[3]).min(b[1] + b[3]);
        [x, y, max_x.saturating_sub(x), max_y.saturating_sub(y)]
    }

    fn draw_label<'a>(frame: &Rect, label: &'a Label, sections: &mut Vec<Section<'a>>, scale: f32) {
        let frame = frame * scale;

        let center = frame.center();

        let margin = 16.0;

        let section = Section::default()
            .add_text(
                Text::new(&label.text)
                    .with_scale(label.text_size() * scale)
                    .with_color(label.text_color().as_slice())
                    .with_z(label.z_position() - UIManager::additional_z_offset()),
            )
            .with_bounds((
                frame.width() - if label.alignment.center() { 0.0 } else { margin },
                frame.height(),
            ))
            .with_layout(
                if label.is_multiline() {
                    Layout::default_wrap()
                } else {
                    Layout::default_single_line()
                }
                .v_align(VerticalAlign::Center)
                .h_align(match label.alignment {
                    TextAlignment::Left => HorizontalAlign::Left,
                    TextAlignment::Center => HorizontalAlign::Center,
                    TextAlignment::Right => HorizontalAlign::Right,
                })
                .line_breaker(BuiltInLineBreaker::UnicodeLineBreaker),
            )
            .with_screen_position((
                match label.alignment {
                    TextAlignment::Left => frame.x() + margin,
                    TextAlignment::Center => center.x,
                    TextAlignment::Right => frame.max_x() - margin,
                },
                center.y,
            ));

        sections.push(section);
    }
}

fn scissor_checked(pass: &mut RenderPass, x: u32, y: u32, w: u32, h: u32) {
    pass.set_scissor_rect(x, y, w, h);
}
