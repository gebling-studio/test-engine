use refs::{Own, Weak};
use crate::window::image::Image;

use crate::game::object::Object;

#[derive(Default)]
pub struct Game {
    pub objects: Vec<Own<Object>>,
    pub skybox:  Weak<Image>,
}

impl Game {
    pub(crate) fn update(&mut self) {
        for obj in &mut self.objects {
            obj.update();
        }
    }
}
