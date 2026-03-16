use std::ops::{Deref, DerefMut};

use gm::{
    LossyConvert,
    color::{CLEAR, TURQUOISE},
    flat::{Rect, Size},
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
    scissor:  Option<Rect<u32>>,
}

impl TextGroup<'_> {
    fn new(scissor: Option<Rect<u32>>) -> Self {
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
        let display_rect: Rect<u32> = Size::<u32>::new(
            resolution.width.lossy_convert(),
            resolution.height.lossy_convert(),
        )
        .into();

        let rect_view = RectView {
            resolution,
            _padding: 0,
        };
        let debug_frames = UIManager::should_draw_debug_frames();
        let scale = UIManager::scale();

        let mut text_groups: Vec<TextGroup<'a>> = vec![TextGroup::new(None)];
        let mut scissor_stack: Vec<Rect<u32>> = vec![];

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
        scissor(pass, display_rect);

        for group in text_groups {
            if group.sections.is_empty() {
                continue;
            }
            let rect = group.scissor.unwrap_or(display_rect);
            scissor(pass, rect);
            let mut font = Font::default();
            font.brush.queue(Window::device(), Window::queue(), group.sections).unwrap();
            font.brush.draw(pass);
        }

        scissor(pass, display_rect);
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
        scissor_stack: &mut Vec<Rect<u32>>,
    ) {
        let frame = *view.absolute_frame();

        if view.is_hidden() || frame.size.has_no_area() {
            return;
        }

        view.before_render(pass);

        let clips = view.clips_to_bounds();

        if clips {
            Self::flush_pipelines(pass, rect_view);

            let frame = frame * scale;
            let resolution_width = rect_view.resolution.width.lossy_convert();
            let resolution_height = rect_view.resolution.height.lossy_convert();
            let x = frame.x().max(0.0).lossy_convert();
            let y = frame.y().max(0.0).lossy_convert();
            let max_x: u32 = frame.max_x().lossy_convert();
            let max_y: u32 = frame.max_y().lossy_convert();
            let max_x = max_x.min(resolution_width);
            let max_y = max_y.min(resolution_height);
            let w = max_x.saturating_sub(x);
            let h = max_y.saturating_sub(y);

            if w == 0 || h == 0 {
                return;
            }

            let new_scissor = if let Some(&parent) = scissor_stack.last() {
                Self::intersect_scissor(parent, Rect::new(x, y, w, h))
            } else {
                Rect::new(x, y, w, h)
            };

            if new_scissor.width() == 0 || new_scissor.height() == 0 {
                return;
            }

            scissor_stack.push(new_scissor);
            scissor(pass, new_scissor);
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
                scissor(pass, parent);
                text_groups.push(TextGroup::new(Some(parent)));
            } else {
                scissor(pass, Rect::new(0, 0, vp_w, vp_h));
                text_groups.push(TextGroup::new(None));
            }
        }
    }

    fn intersect_scissor(a: Rect<u32>, b: Rect<u32>) -> Rect<u32> {
        let x = a.x().max(b.x());
        let y = a.y().max(b.y());
        let max_x = (a.x() + a.width()).min(b.x() + b.width());
        let max_y = (a.y() + a.height()).min(b.y() + b.height());
        Rect::new(x, y, max_x.saturating_sub(x), max_y.saturating_sub(y))
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

fn scissor(pass: &mut RenderPass, rect: Rect<u32>) {
    pass.set_scissor_rect(rect.x(), rect.y(), rect.width(), rect.height());
}
