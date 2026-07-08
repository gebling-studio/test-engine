use netrun::local_ip;
use test_engine::{
    level::LevelManager,
    refs::{Weak, manage::DataManager},
    ui::{
        Container, Font, ImageView, Label, Setup, Shadow, Switch, TextAlignment, Theme, ThemeMode, UIManager,
        ViewData, ViewSubviews, view,
    },
};

use crate::interface::{
    card::Card,
    dev::MenuView,
    noise_view::NoiseView,
    palette::{BG, SURFACE, TEXT, TEXT_DIM},
    render_view::RenderView,
    root_layout_view::RootLayoutView,
    scenes::{EffectsScene, FrostedHud, GameScene, ScrollTables, TextFonts, WidgetGallery},
};

// Title, subtitle and icon per card. The index maps to a scene in `open`.
const CARDS: [(&str, &str, &str); 10] = [
    ("Physics", "gravity and boxes", "crate_box.png"),
    ("Frosted HUD", "blur over the game", "sky.png"),
    ("Widgets", "buttons and inputs", "plus.png"),
    ("Effects", "shadow and blur", "gradient.png"),
    ("Fonts", "the font zoo", "text.png"),
    ("Scrolling", "lists and tables", "file.png"),
    ("Render", "a wgpu pass", "cube_texture.png"),
    ("Noise", "terrain islands", "palm.png"),
    ("Layout", "layout anchors", "square.png"),
    ("Dev", "the raw menu", "cmake.png"),
];

// A small spread of readable fonts, cycled across the card titles to
// show the text pipeline at a glance.
const CARD_FONTS: [&str; 4] = [
    "RussoOne-Regular.ttf",
    "OpenSans.ttf",
    "Neucha.ttf",
    "Pangolin-Regular.ttf",
];

/// The home dashboard. A themed top bar and a responsive grid of cards
/// that open the showcase scenes.
#[view]
pub struct HomeView {
    cards: Vec<Weak<Card>>,

    #[init]
    top_bar: Container,
    logo:    ImageView,
    title:   Label,
    status:  Label,
    theme:   Switch,
    grid:    Container,
}

impl Setup for HomeView {
    fn setup(self: Weak<Self>) {
        LevelManager::stop_level();
        self.set_color(BG);

        self.top_bar.set_color(SURFACE).set_shadow(Shadow::default());
        self.top_bar.place().t(0).lr(0).h(64);

        self.logo.set_image("engine.png");
        self.logo.place().l(20).t(12).size(40, 40);

        self.title
            .set_text("TestEngine")
            .set_text_color(TEXT)
            .set_text_size(26)
            .set_font(Font::get("RussoOne-Regular.ttf"));
        self.title.place().l(70).t(15).w(320).h(34);

        let ip = local_ip().map_or_else(|_| "no ip".to_string(), |ip| ip.to_string());
        self.status
            .set_text(format!("{ip}   {}", UIManager::app_instance_id()))
            .set_text_color(TEXT_DIM)
            .set_text_size(11)
            .set_alignment(TextAlignment::Right);
        self.status.place().t(22).r(94).w(260).h(20);

        self.theme.place().tr(14).size(64, 32);
        self.theme.on_change(move |on| {
            Theme::set_mode(if on { ThemeMode::Dark } else { ThemeMode::Light });
        });

        self.grid.place().t(80).lr(24).all(16).all_wrap();
        self.add_cards();

        // Start light so the toggle sitting in its off state is honest.
        Theme::set_mode(ThemeMode::Light);
    }
}

impl HomeView {
    fn add_cards(mut self: Weak<Self>) {
        for (i, (title, subtitle, icon)) in CARDS.into_iter().enumerate() {
            let card = self.grid.add_view::<Card>();
            card.set_title(title)
                .set_subtitle(subtitle)
                .set_icon(icon)
                .set_title_font(Font::get(CARD_FONTS[i % CARD_FONTS.len()]));
            card.place().size(156, 150);
            card.on_tap(move || Self::open(i));
            self.cards.push(card);
        }
    }

    fn open(index: usize) {
        match index {
            0 => {
                UIManager::set_view(GameScene::new());
            }
            1 => {
                UIManager::set_view(FrostedHud::new());
            }
            2 => {
                UIManager::set_view(WidgetGallery::new());
            }
            3 => {
                UIManager::set_view(EffectsScene::new());
            }
            4 => {
                UIManager::set_view(TextFonts::new());
            }
            5 => {
                UIManager::set_view(ScrollTables::new());
            }
            6 => {
                UIManager::set_view(RenderView::new());
            }
            7 => {
                UIManager::set_view(NoiseView::new());
            }
            8 => {
                UIManager::set_view(RootLayoutView::new());
            }
            9 => {
                UIManager::set_view(MenuView::new());
            }
            _ => {}
        }
    }
}
