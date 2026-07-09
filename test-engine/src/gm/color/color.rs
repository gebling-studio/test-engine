use std::hash::{Hash, Hasher};

use crate::gm::{
    color::{Color, U8Color, helpers::srgb_to_linear},
    num::lossy_convert::LossyConvert,
};

/// sRGB color representation
impl Color<f32> {
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }
}

impl From<Color> for U8Color {
    fn from(value: Color) -> Self {
        U8Color::rgba(
            (255.0 * srgb_to_linear(value.r)).lossy_convert(),
            (255.0 * srgb_to_linear(value.g)).lossy_convert(),
            (255.0 * srgb_to_linear(value.b)).lossy_convert(),
            (255.0 * srgb_to_linear(value.a)).lossy_convert(),
        )
    }
}

impl From<&str> for Color<f32> {
    fn from(value: &str) -> Self {
        U8Color::hex(value).into()
    }
}

pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
pub const BLUE: Color = Color::rgb(0.0, 0.0, 0.8);
pub const LIGHT_BLUE: Color = Color::rgb(0.0, 0.7, 1.0);
pub const GRAY_BLUE: Color = Color::rgb(0.1, 0.2, 0.3);
pub const YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
pub const ORANGE: Color = Color::rgb(1.0, 0.6, 0.0);
pub const PURPLE: Color = Color::rgb(1.0, 0.0, 1.0);
pub const TURQUOISE: Color = Color::rgb(0.0, 1.0, 1.0);
pub const GRAY: Color = Color::rgb(0.5, 0.5, 0.5);
pub const BROWN: Color = Color::rgb(0.7, 0.4, 0.2);
pub const LIGHT_GRAY: Color = Color::rgb(0.8, 0.8, 0.8);
pub const LIGHTER_GRAY: Color = Color::rgb(0.9, 0.9, 0.9);
pub const CLEAR: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

impl Color {
    pub(crate) const ALL: [Color; 13] = [
        BLACK,
        WHITE,
        RED,
        GREEN,
        BLUE,
        LIGHT_BLUE,
        YELLOW,
        ORANGE,
        PURPLE,
        TURQUOISE,
        BROWN,
        LIGHT_GRAY,
        LIGHTER_GRAY,
    ];

    pub fn random() -> Self {
        *Self::ALL.get(fastrand::usize(..Self::ALL.len())).unwrap()
    }

    pub(crate) fn hex(&self) -> String {
        U8Color::from(*self).as_hex()
    }
}

impl Hash for Color<f32> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.r.to_bits());
        state.write_u32(self.g.to_bits());
        state.write_u32(self.b.to_bits());
        state.write_u32(self.a.to_bits());
    }
}

#[cfg(test)]
mod tests {
    use crate::gm::color::{CLEAR, Color, U8Color, WHITE};

    #[test]
    fn color_diff() {
        assert!((WHITE.diff(CLEAR) - 4.0).abs() < f32::EPSILON);
        assert!(WHITE.diff(WHITE).abs() < f32::EPSILON);
        assert!((WHITE.diff(WHITE.with_alpha(0.9)) - 0.1).abs() < 0.000_001);
    }

    #[test]
    fn color_to_u8() {
        let color: U8Color = Color::rgba(0.5, 1.0, 0.1, 0.0).into();
        assert_eq!(color, U8Color::rgba(54, 255, 2, 0));
    }
}
