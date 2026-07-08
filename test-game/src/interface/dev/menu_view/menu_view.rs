use std::{
    fs::{OpenOptions, create_dir_all},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use netrun::System;
use test_engine::{
    Platform,
    audio::Sound,
    dispatch::{after, spawn},
    filesystem::Paths,
    level::LevelManager,
    refs::{Weak, manage::DataManager},
    ui::{
        ALL_VIEWS, AfterSetup, Alert, Anchor, Button, Container, Font, InfiniteScrollTest, Label, Point,
        Setup, Shadow, Spinner, TextAlignment, UIManager, ViewData, ViewSubviews, all_view_tests, all_views,
        view,
    },
};

#[cfg(feature = "bench")]
use crate::interface::dev::UIBenchmarkView;
use crate::{
    api::TEST_REST_REQUEST,
    interface::{
        game_view::GameView,
        home_view::HomeView,
        noise_view::NoiseView,
        palette::{BG, BORDER, SURFACE, TEXT, TEXT_DIM},
        polygon_view::PolygonView,
        render_view::RenderView,
        root_layout_view::RootLayoutView,
        scenes::{GameScene, add_back_button},
    },
    levels::BenchmarkLevel,
    no_physics::NoPhysicsView,
};

/// A flat dev launcher. A themed top bar over a set of labelled sections,
/// each a wrapped row of buttons that fire one dev action.
#[view]
pub struct MenuView {
    #[init]
    top_bar: Container,
}

impl Setup for MenuView {
    fn setup(self: Weak<Self>) {
        self.set_color(BG);

        self.top_bar.set_color(SURFACE).set_shadow(Shadow::default());
        self.top_bar.place().t(0).lr(0).h(64);

        let title = self.top_bar.add_view::<Label>();
        title
            .set_text("Dev")
            .set_text_color(TEXT)
            .set_text_size(24)
            .set_font(Font::get("RussoOne-Regular.ttf"))
            .set_alignment(TextAlignment::Center);
        title.place().center().size(140, 34);

        add_back_button(self);

        let scenes = self.scenes();
        let ui = self.ui(scenes);
        let level = self.level(ui);
        self.system(level);
    }
}

impl MenuView {
    fn scenes(self: Weak<Self>) -> Weak<Container> {
        let scenes = self.section(self.top_bar, "SCENES");
        Self::btn(scenes, "Main level", || UIManager::set_view(GameScene::new()));
        Self::btn(scenes, "Polygon", || UIManager::set_view(PolygonView::new()));
        Self::btn(scenes, "Noise", || {
            LevelManager::stop_level();
            UIManager::set_view(NoiseView::new().on_back(|| {
                UIManager::set_view(Self::new());
            }));
        });
        Self::btn(scenes, "Render", || {
            LevelManager::stop_level();
            UIManager::set_view(RenderView::new());
        });
        Self::btn(scenes, "No physics", || UIManager::set_view(NoPhysicsView::new()));
        Self::btn(scenes, "Root layout", || {
            LevelManager::stop_level();
            UIManager::set_view(RootLayoutView::new());
        });
        Self::btn(scenes, "Empty game", || {
            LevelManager::stop_level();
            UIManager::set_view(GameView::new());
        });
        scenes
    }

    fn ui(self: Weak<Self>, anchor: Weak<Container>) -> Weak<Container> {
        let ui = self.section(anchor, "UI");
        #[cfg(feature = "bench")]
        Self::btn(ui, "UI bench", || {
            LevelManager::stop_level();
            UIManager::set_view(UIBenchmarkView::new());
        });
        Self::btn(ui, "Alert", || {
            Alert::show("Hello!");
        });
        Self::btn(ui, "Sound", || Sound::get("retro.wav").play());
        Self::btn(ui, "Spinner", || {
            let spin = Spinner::lock();
            after(2.0, move || {
                spin.animated_stop();
            });
        });
        Self::btn(ui, "Pick folder", || {
            spawn(async {
                Alert::show(format!("{:?}", Paths::pick_folder().await));
            });
        });
        Self::btn(ui, "Scroll test", || {
            let view = InfiniteScrollTest::new().after_setup(|mut v| {
                v.add_view::<Button>()
                    .set_text("Back")
                    .on_tap(|| UIManager::set_view(HomeView::new()))
                    .place()
                    .size(100, 20);
                v.table.place().clear().back();
                v.table.set_columns(4);
            });
            LevelManager::stop_level();
            UIManager::set_view(view);
        });
        Self::btn(ui, "UI 1x", || UIManager::set_scale(1.0));
        Self::btn(ui, "UI 2x", || UIManager::set_scale(2.0));
        ui
    }

    fn level(self: Weak<Self>, anchor: Weak<Container>) -> Weak<Container> {
        let level = self.section(anchor, "LEVEL");
        Self::btn(level, "Benchmark", || {
            *LevelManager::camera_pos() = Point::default();
            LevelManager::set_level(BenchmarkLevel::default());
        });
        Self::btn(level, "Level 1x", || LevelManager::set_scale(1.0));
        Self::btn(level, "Level 2x", || LevelManager::set_scale(2.0));
        level
    }

    fn system(self: Weak<Self>, anchor: Weak<Container>) {
        let system = self.section(anchor, "SYSTEM");
        Self::btn(system, "System info", || {
            Alert::with_label(|l| {
                l.set_text_size(15);
            })
            .show(System::get_info().dump());
        });
        if Platform::IOS {
            Self::btn(system, "Cloud", write_cloud_data);
        }
        Self::btn(system, "REST request", move || {
            spawn(async move {
                self.rest_pressed().await.unwrap();
            });
        });
        Self::btn(system, "All views", || {
            dbg!(all_views!());
            dbg!(ALL_VIEWS);
            dbg!(all_view_tests!());
        });
        Self::btn(system, "Panic", || panic!("test panic"));
    }

    /// Adds a section header under `anchor` and returns the wrapped button
    /// row below it, to feed the next section as its anchor.
    fn section(self: Weak<Self>, anchor: Weak<Container>, title: &str) -> Weak<Container> {
        let label = self.add_view::<Label>();
        label
            .set_text(title)
            .set_text_color(TEXT_DIM)
            .set_text_size(13)
            .set_font(Font::get("RussoOne-Regular.ttf"))
            .set_alignment(TextAlignment::Left);
        label.place().anchor(Anchor::Top, anchor, 16).lr(24).h(20);

        let grid = self.add_view::<Container>();
        // Anchor only the top to the header. below() would also copy the
        // header width, so lr could no longer inset the row.
        grid.place().anchor(Anchor::Top, label, 6).lr(18).all(6).all_wrap();
        grid
    }

    fn btn<Ret>(grid: Weak<Container>, title: &str, mut action: impl FnMut() -> Ret + Send + 'static) {
        let button = grid.add_view::<Button>();
        button
            .set_text(title)
            .set_color(SURFACE)
            .set_text_color(TEXT)
            .set_corner_radius(10)
            .set_border_width(1)
            .set_border_color(BORDER);
        button.on_tap(move || {
            action();
        });
        button.place().size(132, 38);
    }

    async fn rest_pressed(self: Weak<Self>) -> Result<()> {
        let spin = Spinner::lock();

        let users = TEST_REST_REQUEST.await?;

        spin.stop();

        Alert::show(format!(
            "Got {} users. First name: {}",
            users.len(),
            users.first().unwrap().name
        ));

        Ok(())
    }
}

fn write_cloud_data() {
    let Some(path) = UIManager::cloud_storage_dir() else {
        Alert::show("No path!");
        return;
    };

    let path = path.to_string_lossy();
    let path = path.trim_start_matches("file://");

    let mut path = PathBuf::from(path);

    // iCloud only syncs files that live inside this Documents subfolder.
    path.push("Documents");

    if !path.exists() {
        create_dir_all(&path).unwrap();
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.join("data.txt"))
        .unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut number: i32 = content.parse().unwrap_or_default();
    number += 1;

    file.write_all(number.to_string().as_bytes()).unwrap();

    Alert::show(format!("{}", path.display()));
}
