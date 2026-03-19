use std::fs::read;

use anyhow::Result;
use gm::LossyConvert;
use log::error;
use refs::{
    Weak,
    manage::{DataManager, ResourceLoader},
    managed,
};
use wgpu::{CompareFunction, DepthBiasState, DepthStencilState, StencilState, TextureFormat};
use wgpu_text::{BrushBuilder, TextBrush, glyph_brush::ab_glyph::FontArc};

use crate::{RGBA_TEXTURE_FORMAT, window::Window};

pub struct Font {
    pub name:  String,
    pub brush: TextBrush,
}

impl Font {
    fn new(name: impl ToString, data: &[u8]) -> Result<Self> {
        let window = Window::current();

        let render_size = Window::render_size();

        let font = FontArc::try_from_vec(data.to_vec())?;

        let brush = BrushBuilder::using_font(font).with_depth_stencil( DepthStencilState {
            format:              TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare:       Some(CompareFunction::Less),
            stencil:             StencilState::default(),
            bias:                DepthBiasState::default(),
        }.into())
            /* .initial_cache_size((16_384, 16_384))) */ // use this to avoid resizing cache texture
            .build(&window.device, render_size.width.lossy_convert(), render_size.height.lossy_convert(), RGBA_TEXTURE_FORMAT);
        Ok(Self {
            name: name.to_string(),
            brush,
        })
    }
}

impl Font {
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Weak<Font> {
        Self::store_with_name("default", || {
            Self::new("Helvetica.ttf", include_bytes!("fonts/Helvetica.ttf"))
        })
        .expect("Failed to load default font")
    }
}

managed!(Font);

static DEFAULT_FONT_DATA: &[u8] = include_bytes!("fonts/Helvetica.ttf");

impl ResourceLoader for Font {
    fn load_path(path: &std::path::Path) -> Self {
        let data = read(path);

        let data = data
            .as_ref()
            .map(Vec::as_slice)
            .inspect_err(|err| {
                error!(
                    "Failed to read font file: {}. Error: {err} Returning default font",
                    path.display()
                );
            })
            .unwrap_or(DEFAULT_FONT_DATA);

        Self::load_data(data, path.display())
    }

    fn load_data(data: &[u8], name: impl ToString) -> Self {
        Font::new(name, data).expect("Failed to load font")
    }
}
