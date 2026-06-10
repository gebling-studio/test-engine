use std::sync::Arc;

use gm::flat::Size;

use crate::surface::Surface;

/// Where rendered frames go. `Windowed` presents to a real window,
/// `Headless` renders to an offscreen texture and never touches a display —
/// no winit window, no surface, no compositor.
pub(crate) enum Screen {
    Windowed {
        winit_window: Arc<winit::window::Window>,
        surface:      Option<Surface>,
    },
    Headless {
        size: Size<u32>,
    },
}

impl Screen {
    pub(crate) fn winit_window(&self) -> Option<&winit::window::Window> {
        match self {
            Self::Windowed { winit_window, .. } => Some(winit_window),
            Self::Headless { .. } => None,
        }
    }
}
