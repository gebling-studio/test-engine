use std::ops::{Deref, DerefMut};

use gm::{
    LossyConvert,
    color::{CLEAR, TURQUOISE},
    flat::{Rect, Size},
};
use refs::main_lock::MainLock;
use render::{
    UIGradientPipeline, UIImageRectPipeline,
    data::{RectView, UIGradientInstance, UIImageInstance, UIRectInstance},
};
use ui::{
    ImageView, Label, TextAlignment, UIManager, View, ViewData, ViewFrame, ViewLayout, ViewSubviews,
};
use wgpu::RenderPass;
use wgpu_text::glyph_brush::{BuiltInLineBreaker, HorizontalAlign, Layout, Section, Text, VerticalAlign};
use window::{Font, Window};

use crate::pipelines::Pipelines;

static GRADIENT_DRAWER: MainLock<UIGradientPipeline> = MainLock::new();
static IMAGE_RECT_DRAWER: MainLock<UIImageRectPipeline> = MainLock::new();

pub struct UIDrawer;

impl UIDrawer {
    pub(crate) fn update() {
        UIManager::commit_animations();
        Self::update_view(UIManager::root_view().deref_mut());
    }

    pub(crate) fn draw(pass: &mut RenderPass) {
        let resolution = UIManager::window_resolution();
        let display_rect: Rect<u32> = Size::<u32>::new(
            resolution.width.lossy_convert(),
            resolution.height.lossy_convert(),
        )
        .into();

        let debug_frames = UIManager::should_draw_debug_frames();
        let scale = UIManager::scale();

        let mut text_sections: Vec<Section> = vec![];

        Self::draw_view(
            pass,
            UIManager::root_view_static(),
            &mut text_sections,
            debug_frames,
            scale,
            resolution,
        );

        Self::flush_pipelines(pass, resolution);
        scissor(pass, display_rect);

        let mut font = Font::default();
        font.brush.queue(Window::device(), Window::queue(), text_sections).unwrap();
        font.brush.draw(pass);
    }

    fn flush_pipelines(pass: &mut RenderPass, resolution: Size) {
        let rect_view = RectView {
            resolution,
            _padding: 0,
        };

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
        text_sections: &mut Vec<Section<'a>>,
        debug_frames: bool,
        scale: f32,
        resolution: Size,
    ) {
        let frame = *view.absolute_frame();

        if view.is_hidden() || frame.size.has_no_area() {
            return;
        }

        view.before_render(pass);

        let clips = view.clips_to_bounds();

        if clips {
            Self::flush_pipelines(pass, resolution);
            let mut frame = frame * scale;
            frame.origin.clip_positive();

            if frame.max_x() > resolution.width {
                frame.size.width -= frame.max_x() - resolution.width;
            }

            if frame.max_y() > resolution.height {
                frame.size.height -= frame.max_y() - resolution.height;
            }

            scissor(pass, frame.lossy_convert());
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
            Self::draw_label(&frame, label, text_sections, scale);
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
                Self::draw_view(pass, view.deref(), text_sections, debug_frames, scale, resolution);
            }
        }

        if clips {
            Self::flush_pipelines(pass, resolution);
            scissor(
                pass,
                Size::<u32>::new(
                    resolution.width.lossy_convert(),
                    resolution.height.lossy_convert(),
                )
                .into(),
            );
        }
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
