#![allow(dead_code)]

use test_engine::{
    App, Window,
    refs::Own,
    ui::{Button, Label, Setup, Size, View},
};

#[cfg(feature = "bench")]
use crate::interface::test_game_view::UIBenchmarkView;
use crate::interface::{loading_view::LoadingView, test_game_view::BUTTON};

#[cfg(not_wasm)]
async fn secrets() -> anyhow::Result<&'static test_engine::net::SecretsManager> {
    use std::env::var;

    use anyhow::Context;
    use test_engine::net::SecretsManager;
    use tokio::sync::OnceCell;

    static SECRETS: OnceCell<SecretsManager> = OnceCell::const_new();

    SECRETS
        .get_or_try_init(|| async {
            let client_secret = var("INFISICAL_TE").context("INFISICAL_TE")?;

            let manager = SecretsManager::new(
                "49d67108-3678-45de-b28c-912519d5d3a0",
                client_secret,
                "d8a0c826-859b-406f-b876-ddf98cb5a6f6",
                "dev",
            )
            .await
            .context("Secrets Manager init")?;

            Ok(manager)
        })
        .await
}

#[derive(Default)]
pub struct TestGameApp;

impl App for TestGameApp {
    fn before_launch(&self) {
        BUTTON.apply_globally::<Button>();
        BUTTON.apply_globally::<Label>();
    }

    fn after_launch(&self) {
        Window::set_quit_on_escape(true);
    }

    fn make_root_view(&self) -> Own<dyn View> {
        #[cfg(feature = "bench")]
        if std::env::var("UI_BENCHMARK").is_ok() {
            let force = std::env::args().any(|arg| arg == "--no-guard");
            crate::interface::test_game_view::guard_benchmark(force);
            return UIBenchmarkView::new();
        }
        LoadingView::new()
    }

    fn initial_size(&self) -> Size {
        (1500, 1200).into()
    }

    #[cfg(not_wasm)]
    fn sentry_url(&self) -> test_engine::PinnedFuture<String> {
        Box::pin(async {
            dotenvy::dotenv()?;
            let url = secrets().await?.get("SENTRY_URL").await?;
            Ok(url)
        })
    }
}
