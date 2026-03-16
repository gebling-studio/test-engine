use std::ops::{DerefMut, Neg};

use gm::{
    ToF32,
    flat::{Point, Size},
};
use refs::{Own, Weak, weak_from_ref};
use ui::{
    NO_TOUCH_ID, Scrollable, Setup, Touch, TouchStack, UIAnimation, UIEvent, UIManager, View, ViewData,
    ViewFrame, ViewSubviews, view,
};
use vents::Event;

use crate::{self as test_engine, ui::views::containers::scrolling::ScrollContent};

#[view]
pub struct ScrollView {
    inertia:            f32,
    previous_touch:     Point,
    pub on_scroll:      Event<f32>,
    pub bottom_reached: UIEvent,

    #[init]
    pub(crate) content: ScrollContent,
}

impl ScrollView {
    pub fn remove_all_subviews(&mut self) {
        self.content.remove_all_subviews();
    }

    // Content offset must be negative
    fn max_offset(&self) -> f32 {
        (self.content.content_size.height - self.height()).neg().min(0.0)
    }

    pub fn set_content_offset(&mut self, offset: impl ToF32) -> &mut Self {
        self.content.__base_view().__content_offset = offset.to_f32();

        if self.content.__base_view().__content_offset < self.max_offset() {
            self.content.__base_view().__content_offset = self.max_offset();
        }

        self
    }

    pub fn set_content_size(&mut self, size: impl Into<Size>) -> &mut Self {
        self.content.content_size = size.into();
        self
    }

    pub fn set_content_width(&mut self, width: impl ToF32) -> &mut Self {
        self.content.content_size.width = width.to_f32();
        self
    }

    pub fn set_content_height(&mut self, height: impl ToF32) -> &mut Self {
        self.content.content_size.height = height.to_f32();

        if self.content.__base_view().__content_offset < self.max_offset() {
            self.content.__base_view().__content_offset = self.max_offset();
        }

        self
    }

    pub fn get_scroll_content_offset(&self) -> f32 {
        self.content.content_offset()
    }
}

impl Setup for ScrollView {
    fn setup(mut self: Weak<Self>) {
        self.content.__base_view().dont_hide_off_screen = true;
        self.content.place().back();

        UIManager::on_scroll(self, move |scroll| {
            self.on_scroll(scroll.y);
        });

        self.size_changed().sub(move || {
            self.on_scroll(0.0);
        });

        TouchStack::enable_scroll(self);
    }
}

impl ViewSubviews for ScrollView {
    fn remove_all_subviews(&self) {
        self.content.remove_all_subviews();
    }

    fn add_subview<V: ?Sized + View + 'static>(&self, view: Own<V>) -> Weak<V> {
        self.content.add_subview(view)
    }
}

impl Scrollable for ScrollView {
    fn __process_scroll_touch(&mut self, touch: Touch) -> bool {
        if touch.is_ended() {
            if touch.id == self.__view_base.__touch_id {
                self.add_inertia_animation();
            }

            self.__view_base.__touch_id = NO_TOUCH_ID;
            return false;
        }

        let mut target_frame = self.content.__base_view().__absolute_frame;
        target_frame.origin.y -= self.content.__base_view().__content_offset;

        if touch.is_began() && target_frame.contains(touch.position) {
            self.__view_base.__touch_id = touch.id;
            self.previous_touch = touch.position;
            return true;
        }

        if touch.is_moved() && self.__view_base.__touch_id == touch.id {
            let delta = -(self.previous_touch.y - touch.position.y);
            self.previous_touch = touch.position;

            if delta == 0.0 {
                return true;
            }

            self.inertia = delta;
            self.on_scroll(delta);
            return true;
        }

        false
    }
}

impl ScrollView {
    fn add_inertia_animation(&self) {
        if self.inertia == 0.0 {
            return;
        }

        let mut scroll = weak_from_ref(self);

        let anim = UIAnimation::new(move |_, _| {
            let inertia = scroll.inertia;
            scroll.on_scroll(inertia);
            scroll.inertia *= 0.97;
        })
        .finish_condition(move || scroll.inertia.abs() <= 0.2);

        self.add_animation(anim);
    }

    fn on_scroll(&mut self, scroll: f32) {
        let height = self.content.height();
        let content = self.content.deref_mut();

        if height >= content.content_size.height {
            return;
        }

        *content.content_offset_mut() += scroll;
        let range = content.content_size.height - height;

        if *content.content_offset_mut() <= -range {
            self.bottom_reached.trigger(());
        }

        *content.content_offset_mut() = content.content_offset_mut().clamp(-range, 0.0);

        self.on_scroll.trigger(*content.content_offset_mut());
    }
}
