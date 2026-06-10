use std::{env::temp_dir, fmt::Write, ops::Deref};

use gm::flat::Rect;
use hreads::{from_main, is_main_thread, wait_for_next_frame};
use ui::{UIManager, View, ViewData, ViewFrame, ViewSubviews};

use crate::{AppRunner, ui_test::TEST_NAME};

const MAX_CHILDREN: usize = 30;

/// Everything an agent needs to debug a failed UI test: window info,
/// a screenshot of the actual screen and the view tree with frames.
pub fn failure_report() -> String {
    if is_main_thread() {
        return "No failure report: cannot collect it from the main thread.".to_string();
    }

    let test_name = TEST_NAME.lock().clone();

    let mut report = String::new();

    let (resolution, scale) = from_main(|| (UIManager::window_resolution(), UIManager::scale()));

    _ = writeln!(report, "Window resolution: {resolution:?}, scale: {scale}");
    _ = writeln!(report, "{}", save_failure_screenshot(&test_name));
    _ = writeln!(report, "View tree (label - frame - absolute frame):");
    report.push_str(&from_main(dump_view_tree));

    report
}

fn save_failure_screenshot(test_name: &str) -> String {
    wait_for_next_frame();

    let screenshot = match AppRunner::take_screenshot() {
        Ok(screenshot) => screenshot,
        Err(e) => return format!("Failed to take failure screenshot: {e}"),
    };

    let path = temp_dir().join(format!("ui_test_{}.png", test_name.replace(' ', "_")));

    match screenshot.save(&path) {
        Ok(()) => format!("Failure screenshot: {}", path.display()),
        Err(e) => format!("Failed to save failure screenshot: {e}"),
    }
}

fn dump_view_tree() -> String {
    let mut out = String::new();
    dump_view(UIManager::root_view().deref(), 0, &mut out);
    out
}

fn dump_view(view: &dyn View, depth: usize, out: &mut String) {
    let indent = "  ".repeat(depth);
    let hidden = if view.is_hidden() { " [hidden]" } else { "" };

    _ = writeln!(
        out,
        "{indent}{} - {} - {}{hidden}",
        view.label(),
        rect_str(view.frame()),
        rect_str(view.absolute_frame()),
    );

    let subviews = view.subviews();

    for sub in subviews.iter().take(MAX_CHILDREN) {
        dump_view(sub.deref(), depth + 1, out);
    }

    if subviews.len() > MAX_CHILDREN {
        _ = writeln!(out, "{indent}  ... and {} more", subviews.len() - MAX_CHILDREN);
    }
}

fn rect_str(rect: &Rect) -> String {
    format!(
        "[{}, {}, {}, {}]",
        rect.origin.x, rect.origin.y, rect.size.width, rect.size.height
    )
}
