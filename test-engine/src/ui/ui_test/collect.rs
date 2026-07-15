use std::{
    any::Any,
    future::Future,
    panic::{AssertUnwindSafe, catch_unwind},
    pin::Pin,
    task::{Context, Poll},
};

use anyhow::Result;
use log::error;
use parking_lot::Mutex;

use super::failure_report;

/// One failed test unit. `detail` holds the returned error or the panic
/// message together with the failure report.
pub struct TestFailure {
    pub name:   String,
    pub detail: String,
}

static FAILURES: Mutex<Vec<TestFailure>> = Mutex::new(Vec::new());

/// Drop every recorded failure. The full suite calls this before a run so a
/// second run in the same process starts clean.
pub fn clear_failures() {
    FAILURES.lock().clear();
}

/// Take and clear the failures collected so far.
pub fn take_failures() -> Vec<TestFailure> {
    std::mem::take(&mut FAILURES.lock())
}

pub fn any_failed() -> bool {
    !FAILURES.lock().is_empty()
}

fn record(name: &str, detail: String) {
    error!("{name}: FAILED");
    FAILURES.lock().push(TestFailure {
        name: name.to_string(),
        detail,
    });
}

/// Record a failure from outside the runner, used by the panic hook for a main
/// thread panic that the async `CatchUnwind` cannot reach.
pub fn push_failure(name: &str, detail: String) {
    record(name, detail);
}

/// Catch panics raised while polling a future. Same idea as
/// `futures::FutureExt::catch_unwind`, kept local to avoid pulling the whole
/// `futures` crate in for one combinator.
struct CatchUnwind<F>(F);

impl<F: Future> Future for CatchUnwind<F> {
    type Output = std::thread::Result<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: standard pin projection to the single field. Nothing is moved
        // out of the pinned future.
        let inner = unsafe { self.map_unchecked_mut(|s| &mut s.0) };
        match catch_unwind(AssertUnwindSafe(move || inner.poll(cx))) {
            Ok(Poll::Ready(out)) => Poll::Ready(Ok(out)),
            Ok(Poll::Pending) => Poll::Pending,
            Err(panic) => Poll::Ready(Err(panic)),
        }
    }
}

fn panic_message(panic: &(dyn Any + Send)) -> String {
    if let Some(s) = panic.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = panic.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown panic".to_string()
    }
}

/// Run one async test unit. Catches a returned `Err` and a panic, records the
/// failure, and returns without propagating, so the run keeps going and every
/// failure is reported at the end.
pub async fn run_test(name: &str, fut: impl Future<Output = Result<()>>) {
    match CatchUnwind(fut).await {
        Ok(Ok(())) => {}
        Ok(Err(err)) => record(name, format!("{err:?}")),
        Err(panic) => {
            let report = failure_report().unwrap_or_else(|e| format!("failed to collect report: {e}"));
            record(name, format!("panic: {}\n{report}", panic_message(&*panic)));
        }
    }
}

/// Run one synchronous test unit, used for the registered `#[view_test]` map.
pub fn run_test_sync(name: &str, test: impl FnOnce() -> Result<()>) {
    match catch_unwind(AssertUnwindSafe(test)) {
        Ok(Ok(())) => {}
        Ok(Err(err)) => record(name, format!("{err:?}")),
        Err(panic) => {
            let report = failure_report().unwrap_or_else(|e| format!("failed to collect report: {e}"));
            record(name, format!("panic: {}\n{report}", panic_message(&*panic)));
        }
    }
}
