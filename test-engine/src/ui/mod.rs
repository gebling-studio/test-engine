mod input;
mod tests;
mod ui_drawer;
pub mod ui_test;
pub(crate) mod views;

pub use ::ui::*;
pub use gm::{
    color::*,
    flat::{CornerRadii, Point, PointsPath, Rect, Size},
};
pub use input::*;
pub use ui_drawer::UIDrawer;
pub use ui_proc::view;
pub use views::*;
pub use window::{
    Font, PolygonMode, Screenshot,
    image::{Image, NoImage, Tinted},
};

pub use crate::ui::views::SpriteView;
