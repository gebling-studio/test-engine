use gm::{Apply, ToF32, flat::Point};
use refs::Weak;
use ui_proc::view;
use vents::Event;

use crate::{NumberView, Setup, view::ViewData};

#[view(crate = crate::__macro_root)]
pub struct PointView {
    mul: f32,

    pub changed: Event<Point>,

    #[init]
    x: NumberView,
    y: NumberView,
}

impl PointView {
    pub fn point(&self) -> Point {
        (self.x.value(), self.y.value()).into()
    }

    pub fn set_multiplier(&mut self, mul: impl ToF32) -> &mut Self {
        self.mul = mul.to_f32();
        self
    }
}

impl Setup for PointView {
    fn setup(mut self: Weak<Self>) {
        self.mul = 1.0;
        self.place().all_hor().all(10);

        [self.x, self.y].apply(move |v| {
            v.on_change(move |_| self.changed.trigger(self.point() * self.mul));
        });
    }
}
