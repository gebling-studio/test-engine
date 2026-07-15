use test_engine::{
    App, Window,
    refs::Own,
    ui::{Button, Setup, Size, View},
    ui_test::{TestRunReport, register_test_runner, run_test_map},
};
#[cfg(not_wasm)]
use test_engine::{PinnedFuture, net::SecretsManager};

#[cfg(feature = "bench")]
use crate::interface::dev::UIBenchmarkView;
use crate::interface::{BUTTON, loading_view::LoadingView};

#[cfg(not_wasm)]
async fn secrets() -> anyhow::Result<&'static SecretsManager> {
    use std::env::var;

    use anyhow::Context;
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

/// Every test test-game carries: its own registered views plus the engine's.
/// Called off the main thread by the inspect server.
fn run_ui_tests() -> TestRunReport {
    let mut tests = crate::UI_TESTS.lock().clone();
    tests.append(&mut test_engine::UI_TESTS.lock().clone());
    run_test_map(&tests)
}

#[derive(Default)]
pub struct TestGameApp;

impl App for TestGameApp {
    fn before_launch(&self) {
        BUTTON.apply_globally::<Button>();
        register_test_runner(run_ui_tests);
    }

    fn after_launch(&self) {
        Window::set_quit_on_escape(true);
    }

    fn make_root_view(&self) -> Own<dyn View> {
        #[cfg(feature = "bench")]
        {
            use std::env::{args, var};

            use crate::interface::dev::guard_benchmark;

            if var("UI_BENCHMARK").is_ok() {
                let force = args().any(|arg| arg == "--no-guard");
                guard_benchmark(force);
                return UIBenchmarkView::new();
            }
        }
        LoadingView::new()
    }

    fn initial_size(&self) -> Size {
        (1500, 1200).into()
    }

    #[cfg(not_wasm)]
    fn sentry_url(&self) -> PinnedFuture<Option<String>> {
        Box::pin(async {
            dotenvy::dotenv()?;
            let url = secrets().await?.get("SENTRY_URL").await?;
            Ok(Some(url))
        })
    }
}
