use test_engine::{
    gm::LossyConvert,
    refs::{Weak, manage::DataManager},
    ui::{
        CellRegistry, Container, Font, Label, ScrollView, Setup, TableData, TableView, TextAlignment, View,
        ViewData, ViewSubviews, view,
    },
};

use crate::interface::{
    palette::{BG, SURFACE, SURFACE_ALT, TEXT, TEXT_DIM},
    scenes::add_back_button,
};

const SCROLL_ROWS: usize = 26;
const TABLE_ROWS: usize = 60;
const SCROLL_WIDTH: f32 = 330.0;

/// A manual `ScrollView` with a tall stack of rows on the left and a
/// recycling `TableView` driven by a data source on the right.
#[view]
pub struct ScrollTables {
    #[init]
    scroll: ScrollView,
    table:  TableView,
}

impl Setup for ScrollTables {
    fn setup(self: Weak<Self>) {
        self.set_color(BG);

        let title = self.add_view::<Label>();
        title
            .set_text("Scroll and Tables")
            .set_text_color(TEXT)
            .set_text_size(22)
            .set_font(Font::get("RussoOne-Regular.ttf"))
            .set_alignment(TextAlignment::Center);
        title.place().t(18).center_x().w(360).h(34);

        self.scroll.set_color(SURFACE_ALT).set_corner_radius(12);
        self.scroll.place().t(66).b(16).l(16).w(SCROLL_WIDTH);
        self.fill_scroll();

        self.table.set_color(SURFACE).set_corner_radius(12);
        self.table.place().t(66).b(16).r(16).relative_width(self, 0.44);
        self.table.set_data_source(self).register_cell::<Label>();

        add_back_button(self);
    }
}

impl ScrollTables {
    fn fill_scroll(mut self: Weak<Self>) {
        for i in 0..SCROLL_ROWS {
            let row = self.scroll.add_view::<Container>();
            row.set_color(if i.is_multiple_of(2) { SURFACE } else { SURFACE_ALT });
            row.place().t(46.0 * i.lossy_convert()).lr(0).h(46);

            let label = row.add_view::<Label>();
            label.set_text(format!("Scroll row {i}")).set_text_color(TEXT).set_text_size(15);
            label.place().l(14).center_y().w(220).h(22);
        }

        // Direct children of a scroll size against its content size, which
        // starts at zero width, so set it to the left panel width for the
        // rows to fill.
        self.scroll.set_content_width(SCROLL_WIDTH);
        self.scroll.set_content_height(46.0 * SCROLL_ROWS.lossy_convert());
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
