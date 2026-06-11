use gm::{
    color::Color,
    flat::{Point, Size},
};
use plat::Platform;
use refs::{Own, Weak};
use ui_proc::view;
use window::image::ToImage;

use crate::{
    Container, ImageMode, ImageView, View, ViewData, ViewFrame, ViewSubviews, WeakView, view::Setup,
};

#[view(crate = crate::__macro_root)]
pub struct RootView {
    inner_pos: Point,
    outer_pos: Point,

    inner_size: Size,
    outer_size: Size,

    background: Weak<ImageView>,
    screen:     Weak<Container>,
}

impl RootView {
    pub fn add_subview_to_root(&mut self, view: Own<dyn View>) -> WeakView {
        self.screen.add_subview(view)
    }

    pub fn setup_root(&mut self) {
        let image = ImageView::new();
        self.background = self.__add_subview_internal(image, true).downcast_view::<ImageView>().unwrap();
        self.background.place().back();

        let screen = Container::new();
        self.screen = self.__add_subview_internal(screen, true).downcast_view::<Container>().unwrap();
    }

    pub fn clear_root(&mut self) {
        self.screen.remove_all_subviews();
    }

    pub fn set_color(self: Weak<Self>, color: impl Into<Color>) -> Weak<Self> {
        self.background.set_color(color.into());
        self
    }

    pub fn set_image(mut self: Weak<Self>, image: impl ToImage) -> Weak<Self> {
        self.background.mode = ImageMode::AspectFill;
        self.background.set_image(image);
        self
    }

    pub fn resize_root(
        mut self: Weak<Self>,
        inner_pos: Point,
        outer_pos: Point,
        inner_size: Size,
        outer_size: Size,
        scale: f32,
    ) {
        self.inner_pos = inner_pos;
        self.outer_pos = outer_pos;
        self.inner_size = inner_size;
        self.outer_size = outer_size;

        let render_size = if Platform::DESKTOP {
            self.inner_size
        } else {
            self.outer_size
        };

        self.set_size(
            render_size.width * (1.0 / scale),
            render_size.height * (1.0 / scale),
        );

        self.screen.set_size(
            inner_size.width * (1.0 / scale),
            inner_size.height * (1.0 / scale),
        );

        if Platform::IOS {
            self.screen.set_position(inner_pos * (1.0 / scale));
        } else {
            self.screen.set_position((0, 0));
        }
    }

    pub fn rescale_root(self: Weak<Self>, scale: f32) {
        self.resize_root(
            self.inner_pos,
            self.outer_pos,
            self.inner_size,
            self.outer_size,
            scale,
        );
    }
}
