use std::ops::DerefMut;

use test_engine::{
    RenderPass,
    game::{Game, GameDrawer, Object, Shape},
    refs::{Own, Weak, manage::DataManager},
    ui::{Image, Point, Setup, ViewCallbacks, ViewData, ViewTest, view},
    ui_test::check_colors,
};

use crate::interface::HAS_BACK_BUTTON;

#[view]
pub struct GameView {
    game: Own<Game>,
}

impl Setup for GameView {
    fn setup(mut self: Weak<Self>) {
        self.apply_style(HAS_BACK_BUTTON);

        self.game.skybox = Image::get("sky.png");

        self.game.objects.push(Own::new(Object {
            position: Point::default(),
            rotation: 0.0,
            texture:  Image::get("cat.png"),
            velocity: (0.1, 0.1).into(),
            shape:    Shape::Rect((5, 10).into()),
        }));
    }
}

impl ViewCallbacks for GameView {
    fn before_render(&self, pass: &mut RenderPass) {
        GameDrawer::draw(pass, self.game.weak().deref_mut());
    }
}

impl ViewTest for GameView {
    fn perform_test(_view: Weak<Self>) -> anyhow::Result<()> {
        check_colors(
            r"
                136    4 -  82 177  85
                380    4 -  82 177  85
                592    4 -  82 177  85
                284   84 -  55 159  92
                380  116 -  54 159  92
                480  124 - 103 183  91
                256  200 - 234 197 204
                108  204 - 255 255 255
                348  212 - 216 170 172
                36  216 - 255 255 255
                76  224 - 255 255 255
                32  228 - 255 255 255
                52  228 - 255 255 255
                300  236 - 227 181 181
                12  248 - 255 255 255
                108  248 - 255 255 255
                256  248 - 234 194 195
                560  260 -  56 159  92
                340  276 - 203 153 152
                260  292 - 229 183 185
                400  292 -  54 159  92
                348  308 - 200 148 150
                284  332 - 214 179 159
                340  340 - 165 133 112
                340  364 - 159 129 105
                348  380 - 180 152 130
                260  396 - 221 161 161
                328  396 - 159 127 102
                196  580 - 101 186 227
                4  592 - 101 186 227
                388  592 - 101 186 227
                592  592 - 101 186 227
            ",
        )?;

        // test_engine::ui_test::record_ui_test();

        Ok(())
    }
}
