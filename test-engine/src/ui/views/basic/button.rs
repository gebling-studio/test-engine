use std::{fmt::Display, ops::DerefMut};

use refs::{Weak, weak_from_ref};
use ui_proc::view;

use crate::{
    gm::{
        ToF32,
        color::{CLEAR, Color, WHITE},
    },
    ui::{
        ImageView, Label, Setup, Style, ToLabel, UIColor, UIEvent, UIManager, View, ViewSubviews,
        ViewTransition,
        view::{ViewData, ViewTouch},
    },
    window::{Font, image::ToImage},
};

#[view]
pub struct Button {
    on_tap: UIEvent,

    label: Weak<Label>,

    #[init]
    image: ImageView,
}

impl Button {
    pub fn text(&self) -> &str {
        self.label.text()
    }

    pub fn set_text(&self, text: impl ToLabel) -> &Self {
        self.label.set_hidden(false);
        self.label.set_text(text);
        self
    }

    pub fn text_color(&self) -> &Color {
        self.label.text_color()
    }

    pub fn set_text_color(&self, color: impl Into<UIColor>) -> &Self {
        self.label.set_text_color(color);
        self
    }

    pub fn text_size(&self) -> f32 {
        self.label.text_size()
    }

    pub fn set_text_size(&self, size: impl ToF32) -> &Self {
        weak_from_ref(self).label.set_text_size(size);
        self
    }

    pub fn set_letter_spacing(&self, spacing: impl ToF32) -> &Self {
        weak_from_ref(self).label.set_letter_spacing(spacing);
        self
    }

    pub fn set_font(&self, font: Weak<Font>) -> &Self {
        weak_from_ref(self).label.set_font(font);
        self
    }
}

impl Button {
    pub fn on_tap<R>(&self, mut action: impl FnMut() -> R + Send + 'static) -> &Self {
        self.enable_touch();
        self.on_tap.sub(self.weak(), move || {
            action();
        });
        self
    }

    pub fn set_image(&self, image: impl ToImage) -> &Self {
        self.image.set_hidden(false);
        self.image.set_image(image);
        self
    }

    pub fn set_resizing_image(&mut self, name: impl Display) -> &mut Self {
        self.set_color(CLEAR);
        self.image.set_hidden(false);
        self.image.set_resizing_image(name);
        self
    }
}

impl Button {
    pub fn add_transition<From: View + 'static, To: View + Default + 'static>(
        self: Weak<Self>,
    ) -> Weak<Self> {
        self.on_tap(move || {
            let from = self.find_superview::<From>();
            let mut to = To::new();
            from.transition_to(to.deref_mut());
            UIManager::set_view(to);
        });
        self
    }
}

impl Setup for Button {
    fn setup(mut self: Weak<Self>) {
        self.set_color(WHITE);

        let label = Label::new();

        label.__base_view().is_system = true;
        label.__base_view().ignore_global_style = true;

        self.label = self.add_subview(label).downcast_view().unwrap();

        self.label.place().back();
        self.label.set_color(CLEAR);
        self.label.set_hidden(true);

        self.image.place().back();
        self.image.set_hidden(true);
        self.image.__base_view().is_system = true;

        self.touch().up_inside.sub(self, move || self.on_tap.trigger(()));

        Style::apply_global(self);
    }
}

#[macro_export]
macro_rules! link_button {
    ($self:ident, $($button:ident).+, $method:ident) => {{
        use test_engine::ui::AlertErr;
        $self.$($button).+.on_tap(move || { $self.$method().alert_err(); });
    }}
}

#[macro_export]
macro_rules! async_call {
    ($self:ident, $method:ident) => {
        tokio::spawn(async move {
            $self.$method().await.alert_err();
        });
    };
}
