use crate::gm::color::{BLUE, Color, GREEN};
use refs::{Weak, weak_from_ref};
use ui_proc::view;
use vents::Event;

use crate::ui::{
    Anchor, Container, Setup,
    view::{ViewData, ViewTouch},
};
#[view]
pub struct Switch {
    on: bool,

    off_color: Color,

    pub selected: Event<bool>,

    #[init]
    center: Container,
}

impl Switch {
    pub fn on(&self) -> bool {
        self.on
    }

    pub fn on_change<Ret>(
        self: Weak<Self>,
        mut callback: impl FnMut(bool) -> Ret + Send + 'static,
    ) -> Weak<Self> {
        self.selected.val(move |val| {
            callback(val);
        });
        self
    }

    pub(crate) fn set_on(&mut self, on: bool) {
        const MARGIN: f32 = 5.0;
        self.on = on;
        self.center
            .place()
            .clear()
            .relative(Anchor::Width, weak_from_ref(self), 0.4)
            .tb(MARGIN);
        if on {
            self.center.place().r(MARGIN);
            self.set_color(GREEN);
        } else {
            self.center.place().l(MARGIN);
            self.set_color(self.off_color);
        }
    }

    pub fn set_off_color(&mut self, color: Color) -> &mut Self {
        self.off_color = color;
        self
    }
}

impl Setup for Switch {
    fn setup(mut self: Weak<Self>) {
        self.enable_touch();
        self.center.set_color(BLUE);
        self.set_on(false);
        self.touch().began.sub(move || {
            let on = !self.on;
            self.set_on(on);
            self.selected.trigger(on);
        });
    }
}
