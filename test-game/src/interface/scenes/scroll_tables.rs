use test_engine::{
    gm::LossyConvert,
    refs::Weak,
    ui::{
        Anchor, CellRegistry, Container, Label, ScrollView, Setup, TableData, TableView, TextAlignment, View,
        ViewData, ViewFrame, ViewSubviews, view,
    },
};

use crate::interface::{
    palette::{BG, SURFACE, SURFACE_ALT, TEXT, TEXT_DIM},
    scenes::{HEADER_HEIGHT, add_header},
};

const SCROLL_ROWS: usize = 26;
const TABLE_ROWS: usize = 60;

/// A manual `ScrollView` with a tall stack of rows and a recycling
/// `TableView` driven by a data source. Side by side on wide screens,
/// stacked vertically on narrow ones.
#[view]
pub struct ScrollTables {
    #[init]
    scroll: ScrollView,
    table:  TableView,
}

impl Setup for ScrollTables {
    fn setup(self: Weak<Self>) {
        self.set_color(BG);

        self.scroll.set_color(SURFACE_ALT).set_corner_radius(12);
        self.fill_scroll();

        self.table.set_color(SURFACE).set_corner_radius(12);
        self.table.set_data_source(self).register_cell::<Label>();

        add_header(self, "Scroll and Tables");

        self.size_changed().sub(move || self.arrange());
        self.arrange();
    }
}

impl ScrollTables {
    /// Wide screens show the panels side by side, narrow ones stack them
    /// vertically so both stay usable.
    fn arrange(self: Weak<Self>) {
        let top = HEADER_HEIGHT + 12.0;
        if self.width() < 560.0 {
            self.scroll.place().clear().t(top).lr(12).relative_height(self, 0.4);
            self.table.place().clear().anchor(Anchor::Top, self.scroll, 12).lr(12).b(12);
        } else {
            self.scroll.place().clear().t(top).b(16).l(16).w(330);
            self.table.place().clear().t(top).b(16).r(16).relative_width(self, 0.44);
        }
    }

    fn fill_scroll(self: Weak<Self>) {
        for i in 0..SCROLL_ROWS {
            let row = self.scroll.add_view::<Container>();
            row.set_color(if i.is_multiple_of(2) { SURFACE } else { SURFACE_ALT });
            row.place().t(46.0 * i.lossy_convert()).lr(0).h(46);

            let label = row.add_view::<Label>();
            label.set_text(format!("Scroll row {i}")).set_text_color(TEXT).set_text_size(15);
            label.place().l(14).center_y().w(220).h(22);
        }
    }
}

impl TableData for ScrollTables {
    fn number_of_cells(&self) -> usize {
        TABLE_ROWS
    }

    fn cell_height(&self, _: usize) -> f32 {
        40.0
    }

    fn setup_cell(&mut self, index: usize, registry: &mut CellRegistry) -> Weak<dyn View> {
        let cell = registry.cell::<Label>();
        cell.set_text(format!("   Table cell {index}"))
            .set_text_color(if index.is_multiple_of(2) { TEXT } else { TEXT_DIM })
            .set_text_size(15)
            .set_alignment(TextAlignment::Left);
        cell
    }

    fn cell_selected(&mut self, _: usize) {}
}
