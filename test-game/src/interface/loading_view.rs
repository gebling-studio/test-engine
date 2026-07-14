use anyhow::{Ok, Result};
use log::error;
use test_engine::{
    Platform,
    dispatch::{from_main, on_main, spawn},
    gm::LossyConvert,
    refs::{Weak, manage::DataManager},
    ui::{
        AlertErr, CLEAR, Container, Image, LIGHT_BLUE, Label, ProgressView, Setup, Spinner, UIManager,
        ViewData, ViewSubviews, ViewTest, view,
    },
};

use crate::interface::HomeView;

#[view]
pub struct LoadingView {
    #[init]
    spinner:  Container,
    label:    Label,
    progress: ProgressView,
}

impl Setup for LoadingView {
    fn setup(self: Weak<Self>) {
        self.spinner.place().center().size(200, 200);

        self.label
            .set_text("Loading...")
            .set_color(CLEAR)
            .place()
            .above(self.spinner, 20)
            .h(40);

        self.progress.place().lrb(0).h(20);

        let mut spinner = self.spinner.add_view::<Spinner>();
        spinner.place().back();
        spinner.dot_color = LIGHT_BLUE;

        spawn(async move {
            self.load(vec![
                "frisk.png",
                "board.png",
                "shop.png",
                "stone_floor.png",
                "triangle.png",
                "sky.png",
                "square.png",
                "bullet.png",
                "cat.png",
                "crate_box.png",
            ])
            .await
            .alert_err();
        });
    }
}

impl LoadingView {
    async fn load(self: Weak<Self>, assets: Vec<&str>) -> Result<()> {
        let part = 1.0 / assets.len().lossy_convert();

        for asset in assets {
            if Platform::WASM {
                if let Err(err) = Self::download_asset(asset).await {
                    error!("{err}");
                }
            } else {
                Self::load_asset(asset.to_owned());
            }

            on_main(move || {
                self.progress.inc_progress(part);
            });
        }

        UIManager::set_view(HomeView::new());

        Ok(())
    }

    fn load_asset(path: String) {
        from_main(move || {
            Image::get(path);
        });
    }

    async fn download_asset(path: &str) -> Result<()> {
        Image::download(&path, &format!("http://192.168.0.14:44800/assets/images/{path}")).await?;

        Ok(())
    }
}

impl ViewTest for LoadingView {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        // record_ui_test();

        Ok(())
    }
}
