use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use netrun::Function;
use test_engine::{
    Platform,
    audio::Sound,
    dispatch::{after, from_back, spawn},
    filesystem::Paths,
    gm::Toggle,
    level::LevelManager,
    refs::{Weak, manage::DataManager},
    ui::{
        ALL_VIEWS, AfterSetup, Alert, AlertErr, Button, CellRegistry, Image, InfiniteScrollTest, Point,
        Setup, Spinner, TableData, TableView, UIManager, View, ViewData, ViewFrame, ViewSubviews,
        all_view_tests, all_views, view,
    },
};

use crate::{
    api::TEST_REST_REQUEST,
    interface::{
        game_view::GameView,
        noise_view::NoiseView,
        polygon_view::PolygonView,
        render_view::RenderView,
        root_layout_view::RootLayoutView,
        test_game_view::{MenuEntry, Node, NodeCell, ScaleCell, TestGameView},
    },
    levels::{BenchmarkLevel, TestLevel},
    no_physics::NoPhysicsView,
};
#[cfg(feature = "bench")]
use crate::interface::test_game_view::UIBenchmarkView;

#[view]
pub struct MenuView {
    root: Node<MenuEntry>,

    #[init]
    table: TableView,
}

impl Setup for MenuView {
    #[allow(clippy::too_many_lines)]
    fn setup(mut self: Weak<Self>) {
        self.table.set_data_source(self).place().back();
        self.table
            .register_cell::<NodeCell>()
            .register_cell::<ScaleCell>()
            .register_cell_id("ui scale", || {
                ScaleCell::make(
                    Function::new(|()| UIManager::scale()),
                    Function::new(UIManager::set_scale),
                )
            })
            .register_cell_id("lvl scale", || {
                ScaleCell::make(
                    Function::new(|()| LevelManager::scale()),
                    Function::new(LevelManager::set_scale),
                )
            });

        self.root = Node::new(
            MenuEntry::new("Root"),
            vec![
                Node::new(
                    MenuEntry::new("Scenes"),
                    vec![
                        MenuEntry::new("main")
                            .action(|| {
                                *LevelManager::camera_pos() = Point::default();
                                LevelManager::set_level(TestLevel::default());
                            })
                            .into(),
                        MenuEntry::new("polygon")
                            .action(|| UIManager::set_view(PolygonView::new()))
                            .into(),
                        MenuEntry::new("noise")
                            .action(|| {
                                LevelManager::stop_level();
                                UIManager::set_view(NoiseView::new().on_back(|| {
                                    UIManager::set_view(Self::new());
                                }));
                            })
                            .into(),
                        MenuEntry::new("render")
                            .action(|| {
                                LevelManager::stop_level();
                                UIManager::set_view(RenderView::new());
                            })
                            .into(),
                        MenuEntry::new("no physics")
                            .action(|| UIManager::set_view(NoPhysicsView::new()))
                            .into(),
                        MenuEntry::new("root view")
                            .action(|| {
                                LevelManager::stop_level();
                                UIManager::set_view(RootLayoutView::new());
                            })
                            .into(),
                        MenuEntry::new("empty game")
                            .action(|| {
                                LevelManager::stop_level();
                                UIManager::set_view(GameView::new());
                            })
                            .into(),
                    ],
                ),
                Node::new(
                    MenuEntry::new("UI"),
                    vec![
                        #[cfg(feature = "bench")]
                        MenuEntry::new("ui bench")
                            .action(|| {
                                LevelManager::stop_level();
                                UIManager::set_view(UIBenchmarkView::new());
                            })
                            .into(),
                        MenuEntry::new("ui scale").into(),
                        MenuEntry::new("sound").action(|| Sound::get("retro.wav").play()).into(),
                        MenuEntry::new("alert")
                            .action(|| {
                                Alert::show("Hello!");
                            })
                            .into(),
                        MenuEntry::new("spinner")
                            .action(|| {
                                let spin = Spinner::lock();
                                after(2.0, || {
                                    spin.animated_stop();
                                });
                            })
                            .into(),
                        MenuEntry::new("pick folder")
                            .action(|| {
                                test_engine::dispatch::spawn(async {
                                    Alert::show(format!("{:?}", Paths::pick_folder().await));
                                });
                            })
                            .into(),
                        MenuEntry::new("scroll")
                            .action(|| {
                                let view = InfiniteScrollTest::new();
                                let view = view.after_setup(|mut v| {
                                    v.add_view::<Button>()
                                        .set_text("Back")
                                        .on_tap(|| {
                                            UIManager::set_view(TestGameView::new());
                                        })
                                        .place()
                                        .size(100, 20);
                                    v.table.place().clear().back();
                                    v.table.set_columns(4);
                                });

                                LevelManager::stop_level();
                                UIManager::set_view(view);
                            })
                            .into(),
                    ],
                ),
                Node::new(
                    MenuEntry::new("Level"),
                    vec![
                        MenuEntry::new("benchmark")
                            .action(|| {
                                *LevelManager::camera_pos() = Point::default();
                                LevelManager::set_level(BenchmarkLevel::default());
                            })
                            .into(),
                        MenuEntry::new("lvl scale").into(),
                    ],
                ),
                Node::new(
                    MenuEntry::new("System"),
                    vec![
                        MenuEntry::new("system info")
                            .action(|| {
                                Alert::with_label(|l| {
                                    l.set_text_size(15);
                                })
                                .show(netrun::System::get_info().dump());
                            })
                            .into(),
                        MenuEntry::new("cloud")
                            .action(|| {
                                let Some(path) = UIManager::cloud_storage_dir() else {
                                    Alert::show("No path!");
                                    return;
                                };

                                let path = path.to_string_lossy();
                                let path = path.trim_start_matches("file://");

                                let mut path = PathBuf::from(path);

                                dbg!(&path);

                                // 2. IMPORTANT: Append "Documents"
                                // iCloud only syncs/shows files inside this specific subfolder
                                path.push("Documents");

                                dbg!(&path);

                                // 3. Create the Documents directory if it's missing
                                if !path.exists() {
                                    std::fs::create_dir_all(&path).unwrap();
                                }

                                let mut file = OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .create(true)
                                    .truncate(true)
                                    .open(path.join("data.txt"))
                                    .unwrap();

                                // 2. Read existing content
                                let mut content = String::new();
                                dbg!(&content);
                                file.read_to_string(&mut content).unwrap();

                                let mut number: i32 = content.parse().unwrap_or_default();

                                println!("Existing content: '{number}'");

                                number += 1;

                                // 3. Write new content
                                // Note: After reading, the cursor is at the end of the file.
                                // If you want to overwrite or append, manage the cursor or use .append(true)
                                file.write_all(number.to_string().as_bytes()).unwrap();

                                Alert::show(format!("{}", path.display()));
                            })
                            .enabled(Platform::IOS)
                            .into(),
                        MenuEntry::new("add box")
                            .action(|| {
                                let mut level = LevelManager::downcast_level::<TestLevel>();
                                level.add_random_box((-20, 40));
                            })
                            .into(),
                        MenuEntry::new("load assets")
                            .action(|| {
                                from_back(load_assets_test, |result| {
                                    let Some(image) = result.alert_err() else {
                                        return;
                                    };

                                    LevelManager::level_weak().background = image;
                                });
                            })
                            .into(),
                        MenuEntry::new("request")
                            .action(move || {
                                spawn(async move {
                                    self.rest_pressed().await.unwrap();
                                });
                            })
                            .into(),
                        MenuEntry::new("all views")
                            .action(|| {
                                dbg!(all_views!());
                                dbg!(ALL_VIEWS);
                                dbg!(all_view_tests!());
                            })
                            .into(),
                        MenuEntry::new("panic").action(|| panic!("test panic")).into(),
                    ],
                ),
            ],
        )
        .open();

        self.root.retain(MenuEntry::is_enabled);
    }

    fn inspect(self: Weak<Self>) {
        dbg!(&self.frame());
    }
}

impl TableData for MenuView {
    fn cell_height(&self, _: usize) -> f32 {
        28.0
    }

    fn number_of_cells(&self) -> usize {
        self.root.length()
    }

    fn setup_cell(&mut self, index: usize, registry: &mut CellRegistry) -> Weak<dyn View> {
        let mut this = self.weak();

        let node = self.root.val_at_index(index);

        if node.value.label == "ui scale" {
            let cell = registry.cell_with_id::<ScaleCell>("ui scale");
            cell.set_node(node);
            cell
        } else if node.value.label == "lvl scale" {
            let cell = registry.cell_with_id::<ScaleCell>("lvl scale");
            let node = this.root.val_at_index(index);
            cell.weak().set_node(node);
            cell
        } else {
            let cell = registry.cell::<NodeCell>();
            cell.set_node(node);
            cell
        }
    }

    fn cell_selected(&mut self, index: usize) {
        let val = self.root.val_at_index(index);

        if val.is_leaf() {
            val.value.run();
            return;
        }

        val.open.toggle();
        self.root.update_indices(0, 0);
        self.table.reload_data();
    }
}

impl MenuView {
    async fn rest_pressed(self: Weak<Self>) -> anyhow::Result<()> {
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

async fn load_assets_test() -> Result<Weak<Image>> {
    let _spin = Spinner::lock();

    Image::download(
        "downloaded.jpg",
        "https://fastly.picsum.photos/id/299/1000/1000.jpg?hmac=DRpkgVaALpt0f0Y4kSTUOtLJ66_ULgUDZn2n6pbuafA",
    )
    .await
}
