use std::{fmt::Display, sync::atomic::Ordering};

use atomic_float::AtomicF32;
use gm::{
    ToF32,
    color::{BLACK, Color},
    flat::Size,
};
use refs::{Weak, weak_from_ref};
use ui_proc::view;
use window::{Font, image::ToImage};

use crate::{
    ImageView, Setup, Style, ToLabel, UIManager, View, ViewFrame,
    view::{ViewData, ViewSubviews},
};

static DEFAULT_TEXT_SIZE: AtomicF32 = AtomicF32::new(16.0);

#[derive(Debug, Default)]
pub enum TextAlignment {
    Left,
    #[default]
    Center,
    Right,
}

impl TextAlignment {
    pub fn center(&self) -> bool {
        matches!(self, Self::Center)
    }
}

#[view(crate = crate::__macro_root)]
pub struct Label {
    pub alignment: TextAlignment,

    pub text: String,

    multiline: bool,

    #[educe(Default = BLACK)]
    text_color: Color,

    #[educe(Default = DEFAULT_TEXT_SIZE.load(Ordering::Relaxed))]
    text_size: f32,

    font: Weak<Font>,
}

impl Label {
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&self, text: impl ToLabel) -> &Self {
        weak_from_ref(self).text = text.to_label();
        self
    }

    pub fn text_color(&self) -> &Color {
        &self.text_color
    }

    pub fn set_text_color(&self, color: impl Into<Color>) -> &Self {
        weak_from_ref(self).text_color = color.into();
        self
    }

    pub fn text_size(&self) -> f32 {
        self.text_size
    }

    pub fn set_text_size(&self, size: impl ToF32) -> &Self {
        weak_from_ref(self).text_size = size.to_f32();
        self
    }

    pub fn font(&self) -> Weak<Font> {
        if self.font.is_ok() {
            self.font
        } else {
            Font::default()
        }
    }

    pub fn set_font(&self, font: Weak<Font>) -> &Self {
        weak_from_ref(self).font = font;
        self
    }

    /// Size the label's frame needs to show the current text. Multiline
    /// wraps at the current frame width.
    pub fn content_size(&self) -> Size {
        self.size_for_width(self.width())
    }

    /// Size the label's frame needs at the given frame width. For multiline
    /// this is how auto-height panels measure before layout.
    pub fn size_for_width(&self, width: f32) -> Size {
        let margin = self.alignment_margin();
        let bound = self.multiline.then_some(width - margin);
        let measured = self.font().measure(&self.text, self.text_size, bound);

        if measured.has_no_area() {
            return measured;
        }

        Size::new(measured.width + margin, measured.height)
    }

    // The drawer indents left and right aligned text by 16 physical pixels,
    // so the fitted frame must include it or the text clips.
    fn alignment_margin(&self) -> f32 {
        if self.alignment.center() {
            0.0
        } else {
            16.0 / UIManager::scale()
        }
    }
}

impl Label {
    pub fn set_alignment(&self, alignment: TextAlignment) -> &Self {
        weak_from_ref(self).alignment = alignment;
        self
    }

    pub fn is_multiline(&self) -> bool {
        self.multiline
    }

    pub fn set_multiline(&self, multiline: bool) -> &Self {
        weak_from_ref(self).multiline = multiline;
        self
    }

    pub fn set_image(&self, image: impl ToImage) -> &Self {
        self.remove_all_subviews();
        let image_view = self.add_view::<ImageView>();
        image_view.place().back();
        image_view.set_image(image);
        image_view.__base_view().z_position = self.z_position();

        self
    }

    pub fn set_resizing_image(&mut self, name: impl Display) -> &mut Self {
        self.remove_all_subviews();
        let mut image_view = self.add_view::<ImageView>();
        image_view.place().back();
        image_view.set_resizing_image(name);
        image_view.__base_view().z_position = self.z_position();
        image_view.subviews_weak().iter_mut().for_each(|v| {
            v.__base_view().z_position = self.z_position();
            v.subviews_weak().iter_mut().for_each(|v| {
                v.__base_view().z_position = self.z_position();
            });
        });

        self
    }
}

impl Label {
    pub fn set_default_text_size(size: impl ToF32) {
        DEFAULT_TEXT_SIZE.store(size.to_f32(), Ordering::Relaxed);
    }
}

impl Setup for Label {
    fn setup(self: Weak<Self>) {
        Style::apply_global(self);
    }
}

pub trait AddLabel {
    fn add_label(&self, text: impl ToLabel) -> &Self;
}

impl<T: ?Sized + View> AddLabel for T {
    fn add_label(&self, text: impl ToLabel) -> &Self {
        let mut label = self.add_view::<Label>();
        label.place().center().h(20).lr(0);
        label.text = text.to_label();
        self
    }
}
