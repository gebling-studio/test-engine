use std::fmt::Display;

use refs::{Weak, weak_from_ref};
use ui_proc::view;

use crate::{
    gm::flat::Rect,
    ui::{NineSegmentImageView, Setup, ViewData, ViewFrame, ViewSubviews},
    window::image::{Image, ToImage},
};

#[derive(Default)]
pub enum ImageMode {
    #[default]
    Fill,
    AspectFit,
    AspectFill,
}

#[view]
pub struct ImageView {
    image: Weak<Image>,

    nine_segment: Weak<NineSegmentImageView>,

    pub mode: ImageMode,

    pub flip_x: bool,
    pub flip_y: bool,
}

impl ImageView {
    pub(crate) fn image(&self) -> Weak<Image> {
        self.image
    }

    pub fn set_image(&self, image: impl ToImage) -> &Self {
        weak_from_ref(self).image = image.to_image();
        self
    }

    pub fn set_resizing_image(&mut self, name: impl Display) -> &mut Self {
        if !self.nine_segment.was_initialized() {
            self.nine_segment = self.add_view();
            self.nine_segment.place().back();
            self.nine_segment
                .subviews_weak()
                .iter_mut()
                .for_each(|v| v.__base_view().z_position = self.z_position());
        }

        self.nine_segment.set_image(name);

        self
    }

    pub(crate) fn image_frame(&self) -> Rect {
        match self.mode {
            ImageMode::Fill => *self.absolute_frame(),
            ImageMode::AspectFit => self.absolute_frame().fit_aspect_ratio(self.image.size.into()),
            ImageMode::AspectFill => self.absolute_frame().fill_aspect_ratio(self.image.size.into()),
        }
    }
}

impl Setup for ImageView {
    fn clips_to_bounds(&self) -> bool {
        matches!(self.mode, ImageMode::AspectFill)
    }
}
