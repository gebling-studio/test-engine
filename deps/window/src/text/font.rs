use std::fs::read;

use anyhow::Result;
use gm::LossyConvert;
use log::error;
use refs::{
    Weak,
    main_lock::MainLock,
    manage::{DataManager, ResourceLoader},
    managed,
};
use wgpu::{CompareFunction, DepthBiasState, DepthStencilState, StencilState, TextureFormat};
use wgpu_text::{BrushBuilder, TextBrush, glyph_brush::ab_glyph::FontArc};

use crate::{SURFACE_TEXTURE_FORMAT, window::Window};

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
            .build(&window.device, render_size.width.lossy_convert(), render_size.height.lossy_convert(), SURFACE_TEXTURE_FORMAT);
        Ok(Self {
            name: name.to_string(),
            brush,
        })
    }
}

static DEFAULT_FONT: MainLock<Option<Weak<Font>>> = MainLock::new();

impl Font {
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Weak<Font> {
        if let Some(font) = *DEFAULT_FONT
            && font.is_ok()
        {
            return font;
        }
        Self::helvetica()
    }

    pub fn set_default(font: Weak<Font>) {
        *DEFAULT_FONT.get_mut() = Some(font);
    }

    pub fn reset_default() {
        *DEFAULT_FONT.get_mut() = None;
    }

    pub fn helvetica() -> Weak<Font> {
        Self::store_with_name("Helvetica.ttf", || {
            Self::new("Helvetica.ttf", include_bytes!("fonts/Helvetica.ttf"))
        })
        .expect("Failed to load Helvetica font")
    }

    pub fn san_francisco() -> Weak<Font> {
        Self::store_with_name("SF.otf", || Self::new("SF.otf", include_bytes!("fonts/SF.otf")))
            .expect("Failed to load San Francisco font")
    }

    pub fn roboto() -> Weak<Font> {
        Self::store_with_name("Roboto-Regular.ttf", || {
            Self::new("Roboto-Regular.ttf", include_bytes!("fonts/Roboto-Regular.ttf"))
        })
        .expect("Failed to load Roboto font")
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
