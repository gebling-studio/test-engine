use std::{
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

use test_engine::{
    Window,
    gm::LossyConvert,
    refs::Weak,
    ui::{
        Alert,
        Anchor::{Right, Top, Width, X},
        Button, CheckBox, CircleView, Color, Container, DrawingView, ImageView, Label, NumberView,
        ProgressView, ScrollView, Setup, Slider, Switch, TextAlignment, View, ViewCallbacks, ViewData,
        ViewFrame, ViewSubviews, view,
    },
};

const PANEL_COLS: usize = 10;
const PANEL_ROWS: usize = 8;
const PANEL_SIZE: f32 = 150.0;

/// One panel is added every frame until the rolling average of frame work
/// time crosses this. 16.6 ms is the 60 fps budget.
const LAG_THRESHOLD_MS: f32 = 16.0;
const ROLLING_WINDOW: usize = 10;

/// Safety bound so the run always terminates.
const MAX_PANELS: usize = 10_000;

/// Step between rows in the final report. Every frame is recorded, printing
/// all of them would flood the console.
const REPORT_EVERY: usize = 50;

static PANEL_INDEX: AtomicUsize = AtomicUsize::new(0);

const TEXTS: &[&str] = &["alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf"];

fn shade(i: usize, salt: usize) -> Color {
    let channel = |m: usize| -> f32 {
        let v: f32 = ((i * m + salt * 31) % 100).lossy_convert();
        0.15 + 0.7 * (v / 100.0)
    };
    Color::rgb(channel(37), channel(53), channel(71))
}

fn count_views(view: &dyn View) -> usize {
    1 + view.subviews().iter().map(|sub| count_views(sub.deref())).sum::<usize>()
}

struct StepResult {
    panels: usize,
    views:  usize,
    avg_ms: f32,
}

#[view]
pub struct UIBenchmarkView {
    panels:          usize,
    views_per_panel: usize,
    reported:        bool,
    results:         Vec<StepResult>,
}

impl Setup for UIBenchmarkView {
    fn setup(self: Weak<Self>) {
        Window::set_vsync(false);
        PANEL_INDEX.store(0, Ordering::Relaxed);
    }
}

impl ViewCallbacks for UIBenchmarkView {
    fn update(&mut self) {
        if self.reported {
            return;
        }

        self.add_panel();

        if self.views_per_panel == 0 {
            self.views_per_panel = count_views(self) - 1;
        }

        self.results.push(StepResult {
            panels: self.panels,
            views:  self.panels * self.views_per_panel,
            avg_ms: Window::current().frame_work_time() * 1000.0,
        });

        if (self.rolling_ms() >= LAG_THRESHOLD_MS && self.results.len() >= ROLLING_WINDOW)
            || self.panels >= MAX_PANELS
        {
            self.reported = true;
            self.report();
        }
    }
}

impl UIBenchmarkView {
    fn rolling_ms(&self) -> f32 {
        let window = self.results.len().min(ROLLING_WINDOW);
        if window == 0 {
            return 0.0;
        }
        let sum: f32 = self.results[self.results.len() - window..].iter().map(|r| r.avg_ms).sum();
        let len: f32 = window.lossy_convert();
        sum / len
    }

    // The grid covers the window, then panels stack on top of it in layers
    // shifted by a deterministic offset. Off-screen views are culled from
    // rendering, so panels must stay inside the window to keep costing.
    fn add_panel(&mut self) {
        let per_layer = PANEL_COLS * PANEL_ROWS;

        let col: f32 = (self.panels % PANEL_COLS).lossy_convert();
        let row: f32 = (self.panels / PANEL_COLS % PANEL_ROWS).lossy_convert();
        let layer = self.panels / per_layer;

        let shift_x: f32 = (layer * 13 % 120).lossy_convert();
        let shift_y: f32 = (layer * 29 % 120).lossy_convert();

        let panel = self.add_view::<BenchPanel>();
        panel.set_frame((
            col * PANEL_SIZE + shift_x,
            row * PANEL_SIZE + shift_y,
            PANEL_SIZE,
            PANEL_SIZE,
        ));

        self.panels += 1;
    }

    fn sampled(&self) -> impl Iterator<Item = &StepResult> {
        let last_index = self.results.len() - 1;
        self.results
            .iter()
            .enumerate()
            .filter(move |(index, _)| index % REPORT_EVERY == 0 || *index == last_index)
            .map(|(_, result)| result)
    }

    fn report(&self) {
        println!();
        println!("UI benchmark:");
        println!("{:<7} {:>7} {:>8} {:>9}", "panels", "views", "ms", "fps");

        for result in self.sampled() {
            let fps = if result.avg_ms > 0.0 { 1000.0 / result.avg_ms } else { 0.0 };
            println!(
                "{:<7} {:>7} {:>8.3} {:>9.1}",
                result.panels, result.views, result.avg_ms, fps
            );
        }

        self.write_json();

        let last = self.results.last().expect("benchmark finished with no results");
        let rolling = self.rolling_ms();
        Alert::show(format!(
            "{} panels, {} views: {rolling:.2} ms per frame ({:.0} fps)",
            last.panels,
            last.views,
            if rolling > 0.0 { 1000.0 / rolling } else { 0.0 }
        ));

        // Scripted run: UI_BENCHMARK=1 cargo run -p test-game. Exit so the
        // caller's pipe gets the report. From the menu the app stays alive.
        if std::env::var("UI_BENCHMARK").is_ok() {
            test_engine::AppRunner::stop();
        }
    }

    fn write_json(&self) {
        let Ok(path) = std::env::var("UI_BENCHMARK_JSON") else {
            return;
        };

        let last = self.results.last().expect("benchmark finished with no results");

        let json = serde_json::json!({
            "panels": last.panels,
            "views": last.views,
            "ms": (self.rolling_ms() * 100.0).round() / 100.0,
        });

        let json = serde_json::to_string_pretty(&json).expect("failed to serialize benchmark results");

        if let Err(err) = std::fs::write(&path, json) {
            eprintln!("Failed to write benchmark JSON to {path}: {err}");
        }
    }
}

#[view]
struct BenchPanel {
    #[init]
    title:      Label,
    body:       Label,
    button:     Button,
    image:      ImageView,
    switch:     Switch,
    check:      CheckBox,
    slider:     Slider,
    progress:   ProgressView,
    circle:     CircleView,
    number:     NumberView,
    drawing:    DrawingView,
    badge:      Container,
    rel_box:    Container,
    halves:     Container,
    footer:     Container,
    custom_box: Container,
    scroll:     ScrollView,
}

impl Setup for BenchPanel {
    fn setup(mut self: Weak<Self>) {
        let i = PANEL_INDEX.fetch_add(1, Ordering::Relaxed);

        self.set_gradient(shade(i, 0), shade(i, 1));
        self.set_corner_radius(4);
        self.set_border_color(shade(i, 2));
        self.set_border_width(1);

        self.title
            .set_text(TEXTS[i % TEXTS.len()])
            .set_text_size(9)
            .set_alignment(TextAlignment::Left);
        self.title.place().lrt(2).h(14).max_width(400).min_height(10);

        self.body.set_text("multi\nline\ntext").set_multiline(true).set_text_size(8);
        self.body.place().same([Width, X], self.title).anchor(Top, self.title, 2).h(30);

        self.button.set_text("tap").set_text_size(8);
        self.button.place().bl(2).size(46, 14);

        self.image.set_image("round.png");
        self.image.place().br(2).size(18, 18);

        self.switch.place().tr(2).size(34, 16);
        if i.is_multiple_of(2) {
            self.switch.set_on(true);
        }

        self.check.place().same([Width, X], self.switch).anchor(Top, self.switch, 2).h(16);
        if i.is_multiple_of(3) {
            self.check.set_on(true);
        }

        self.slider.place().r(2).center_y().size(12, 44);

        self.progress.place().lr(2).b(18).h(4);
        let progress: f32 = (i % 10).lossy_convert();
        self.progress.set_progress(progress / 10.0);

        self.circle.set_radius(6);
        self.circle.set_color(shade(i, 3));
        self.circle.place().between(self.button, self.image);

        self.number.place().center().size(24, 28);
        let value: f32 = (i % 50).lossy_convert();
        self.number.set_value(value);

        self.drawing.place().l(2).center_y().size(26, 26);
        self.drawing.add_path([(0, 0), (13, 22), (26, 0)], shade(i, 4));

        self.badge.set_color(shade(i, 5)).set_corner_radius(3);
        self.badge.place().size(7, 7).between_super(self.button, Right);

        self.rel_box.set_color(shade(i, 6));
        self.rel_box.place().bl(20).relative(Width, self.title, 0.4).h(6);

        self.halves.place().lr(2).t(16).h(8);
        let left = self.halves.add_view::<Container>();
        left.set_color(shade(i, 7));
        left.place().left_half();
        let right = self.halves.add_view::<Container>();
        right.set_color(shade(i, 8));
        right.place().right_half();

        self.footer.place().lr(2).b(8).h(8).distribute_ratio([1.0, 2.0, 3.0]);
        for salt in 9..12 {
            self.footer.add_view::<Container>().set_color(shade(i, salt));
        }

        self.custom_box.set_color(shade(i, 12));
        let width: f32 = (10 + (i * 7) % 20).lossy_convert();
        self.custom_box.place().custom(move |rect| *rect = (2.0, 30.0, width, 6.0).into());

        self.scroll.place().t(2).r(40).size(30, 40);
        self.scroll.set_content_size((30, 120));
        let scroll_label = self.scroll.add_view::<Label>();
        scroll_label.set_text("scroll").set_text_size(8);
        scroll_label.place().tl(1).size(28, 20);
    }
}
