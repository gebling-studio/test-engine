mod effects;
mod frosted_hud;
mod game_scene;
mod scroll_tables;
mod text_fonts;
mod widget_gallery;

pub use effects::EffectsScene;
pub use frosted_hud::FrostedHud;
pub use game_scene::GameScene;
pub use scroll_tables::ScrollTables;
use test_engine::{
    refs::Weak,
    ui::{Button, Setup, UIManager, View, ViewData, ViewSubviews, WHITE},
};
pub use text_fonts::TextFonts;
pub use widget_gallery::WidgetGallery;

use crate::interface::{HomeView, palette::ACCENT};

/// A themed "Back" button pinned top-left that returns to the home
/// dashboard. Every showcase scene adds one.
pub fn add_back_button<T: View>(view: Weak<T>) {
    let btn = view.add_view::<Button>();
    btn.set_color(ACCENT)
        .set_text_color(WHITE)
        .set_corner_radius(10)
        .set_text("Back");
    btn.on_tap(|| {
        UIManager::set_view(HomeView::new());
    });
    btn.place().tl(20).size(90, 40);
}
