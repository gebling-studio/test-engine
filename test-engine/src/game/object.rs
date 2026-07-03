use refs::Weak;
use crate::window::image::Image;

use crate::game::{Point, Rotation, Shape};

pub struct Object {
    pub position: Point,
    pub velocity: Point,

    pub shape:    Shape,
    pub rotation: Rotation,

    pub texture: Weak<Image>,
}

impl Object {
    pub(crate) fn update(&mut self) {
        self.position += self.velocity;
    }
}
