use crate::gm::color::Color;
use log::error;
use refs::{Weak, manage::DataManager};

use crate::window::image::{DEFAULT_IMAGE_DATA, Image, ToImage};

/// Image with tint color
/// Works only with SVG files
/// Implementation is very naive it replaces all #000000 strings (black hex) to
/// the hex of the tint. So all tinted elements in the SVG should be black
pub struct Tinted {
    pub tint: Color,
    pub name: String,
}

impl ToImage for Tinted {
    fn to_image(&self) -> Weak<Image> {
        let path = Image::full_path(&self.name);

        let data = std::fs::read(&path);

        let data = data
            .as_ref()
            .map(Vec::as_slice)
            .inspect_err(|err| {
                error!(
                    "Failed to read image file: {}. Error: {err} Returning default image",
                    path.display()
                );
            })
            .unwrap_or(DEFAULT_IMAGE_DATA);

        let new_data = std::str::from_utf8(data)
            .expect("Data for tinted image is not a valid string. Most likely is not an SVG")
            .replace("#000000", &self.tint.hex());

        Image::from_file_data(new_data.as_bytes(), &format!("{}:{}", self.name, self.tint.hex()))
    }
}
