use std::collections::BTreeMap;

use anyhow::Result;
use hreads::from_main;
use parking_lot::Mutex;

use super::{TestFailure, clear_failures, run_test_sync, take_failures};
use crate::ui::{Label, Style, UIManager};

pub struct TestRunReport {
    pub total:    usize,
    pub failures: Vec<TestFailure>,
}

/// Tests expect scale 1 and 32 point text. Any host that runs them must match,
/// or every layout and color check drifts.
fn prepare_harness() {
    Label::set_default_text_size(32);

    from_main(|| {
        UIManager::override_scale(1.0);
    });
}

/// Run a whole map of registered tests through the failure collector. Must not
/// run on the main thread, the tests drive the main thread through `from_main`.
pub fn run_test_map(tests: &BTreeMap<String, fn() -> Result<()>>) -> TestRunReport {
    let app_styles = from_main(Style::take_globals);

    prepare_harness();
    clear_failures();

    for (name, test) in tests {
        run_test_sync(name, *test);
    }

    let report = TestRunReport {
        total:    tests.len(),
        failures: take_failures(),
    };

    from_main(move || Style::restore_globals(app_styles));

    report
}

type TestRunner = fn() -> TestRunReport;

static TEST_RUNNER: Mutex<Option<TestRunner>> = Mutex::new(None);

/// An app registers how to run its own suite. The engine cannot reach an app's
/// `UI_TESTS` map on its own, the map is a static of the app crate.
pub fn register_test_runner(runner: TestRunner) {
    *TEST_RUNNER.lock() = Some(runner);
}

#[cfg(feature = "inspect")]
pub(crate) fn test_runner() -> Option<TestRunner> {
    *TEST_RUNNER.lock()
}
