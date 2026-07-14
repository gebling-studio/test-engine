use std::ops::Deref;

use anyhow::Result;
use parking_lot::Mutex;
use refs::Weak;

use crate::{
    self as test_engine,
    gm::color::{BLUE, Color, GREEN, PURPLE, RED, YELLOW},
    ui::{CellRegistry, Container, Setup, TableData, TableView, View, ViewData, ViewTest, view_test},
    ui_test::{check_colors, inject_scroll, inject_touches, set_record_probe_count},
};

static SELECTED: Mutex<String> = Mutex::new(String::new());

const PALETTE: [Color; 4] = [GREEN, BLUE, YELLOW, RED];

// The backdrop behind the table has a color no cell uses, so the
// gaps between cells expose it and the recorder pins probes there.
#[view_test]
struct TableSpacingTest {
    #[init]
    under: Container,
    table: TableView,
}

impl Setup for TableSpacingTest {
    fn setup(mut self: Weak<Self>) {
        self.under.set_color(PURPLE);
        self.under.place().tl(0).size(400, 600);

        self.table.place().tl(0).size(400, 600);
        self.table.set_data_source(self).register_cell::<Container>();
        self.table.set_cell_spacing(16);
        self.table.set_columns(2);
        self.table.reload_data();
    }
}

impl TableData for TableSpacingTest {
    fn cell_height(&self, _: usize) -> f32 {
        90.0
    }

    fn number_of_cells(&self) -> usize {
        12
    }

    fn setup_cell(&mut self, index: usize, registry: &mut CellRegistry) -> Weak<dyn View> {
        let cell = registry.cell::<Container>();
        cell.set_color(PALETTE[index % PALETTE.len()]);
        cell
    }

    fn cell_selected(&mut self, index: usize) {
        *SELECTED.lock() += &format!("|{index}|");
    }
}

// Cells are 192x90 on a 106 pitch, the 16px gaps show the backdrop
// between rows and columns.
fn check_unscrolled() -> Result<()> {
    check_colors(
        r"
           4    4 -   0 255   0
          64    4 -   0 255   0
         128    4 -   0 255   0
         228    4 -   0   0 231
         316    4 -   0   0 231
         592    4 -  89 124 149
         456   16 -  89 124 149
         272   52 -   0   0 231
         384   56 -   0   0 231
          72   60 -   0 255   0
         148   76 -   0 255   0
         220   88 -   0   0 231
         312  100 - 255   0 255
           8  104 - 255   0 255
         516  120 -  89 124 149
         104  136 - 255 255   0
         396  140 - 255   0   0
         260  148 - 255   0   0
         180  152 - 255 255   0
          44  156 - 255 255   0
           4  208 - 255   0 255
          80  208 - 255   0 255
         592  212 -  89 124 149
         252  216 -   0   0 231
         348  216 -   0   0 231
         152  220 -   0 255   0
         484  224 -  89 124 149
         204  264 - 255   0 255
          40  268 -   0 255   0
         308  296 -   0   0 231
         404  300 -  89 124 149
         100  304 - 255   0 255
         488  308 -  89 124 149
           4  332 - 255 255   0
         220  332 - 255   0   0
         152  344 - 255 255   0
          60  356 - 255 255   0
         340  356 - 255   0   0
         276  360 - 255   0   0
         548  368 -  89 124 149
         396  396 - 255   0   0
         100  404 - 255 255   0
         184  404 - 255 255   0
         304  420 - 255   0 255
           4  432 -   0 255   0
         360  448 -   0   0 231
         232  460 -   0   0 231
         100  464 -   0 255   0
         500  464 -  89 124 149
         160  484 -   0 255   0
         396  500 -   0   0 231
          64  512 -   0 255   0
         280  516 - 255   0 255
         592  520 -  89 124 149
           4  528 - 255   0 255
         120  528 - 255   0 255
         344  536 - 255   0   0
         216  568 - 255   0   0
          64  572 - 255 255   0
         396  588 - 255   0   0
           4  592 - 255 255   0
         120  592 - 255 255   0
         312  592 - 255   0   0
         508  592 -  89 124 149
        ",
    )
}

// Content height is 6 rows of 90 plus 5 gaps of 16, no gap after the
// last row, so it lands flush with the table bottom after scrolling
// all the way down.
fn check_scrolled_to_bottom() -> Result<()> {
    check_colors(
        r"
           4    4 -   0 255   0
         124    4 -   0 255   0
         316    4 -   0   0 231
         488    4 -  89 124 149
         396   16 -   0   0 231
          64   20 -   0 255   0
         220   36 -   0   0 231
         156   56 -   0 255   0
         348   60 -   0   0 231
          64   84 - 255   0 255
         592   84 -  89 124 149
         292   92 - 255   0   0
           4  100 - 255 255   0
         396  100 - 255   0   0
         496  104 -  89 124 149
         160  116 - 255 255   0
          96  136 - 255 255   0
         236  144 - 255   0   0
         360  152 - 255   0   0
           4  164 - 255 255   0
         304  180 - 255   0 255
         100  196 -   0 255   0
         180  196 -   0 255   0
         396  204 -   0   0 231
         512  204 -  89 124 149
          36  224 -   0 255   0
         276  240 -   0   0 231
         340  244 -   0   0 231
         148  256 -   0 255   0
         216  264 -   0   0 231
           4  284 - 255   0 255
          96  296 - 255   0 255
         404  300 -  89 124 149
         304  304 - 255   0   0
         496  328 -  89 124 149
         592  328 -  89 124 149
         204  332 - 255   0 255
          28  336 - 255 255   0
         356  336 - 255   0   0
          76  368 - 255 255   0
         152  376 - 255 255   0
         248  384 - 255   0   0
           4  392 - 255   0 255
         336  392 - 255   0 255
         396  396 - 255   0 255
         512  424 -  89 124 149
         188  444 -   0 255   0
         392  456 -   0   0 231
         100  460 -   0 255   0
         296  468 -   0   0 231
           4  492 -   0 255   0
         348  496 - 255   0 255
         468  500 -  89 124 149
         212  516 - 255   0   0
         140  520 - 255 255   0
         592  520 -  89 124 149
          68  536 - 255 255   0
         276  536 - 255   0   0
         384  544 - 255   0   0
         460  588 -  89 124 149
           4  592 - 255 255   0
         124  592 - 255 255   0
         204  592 - 255   0 255
         312  592 - 255   0   0
        ",
    )
}

impl ViewTest for TableSpacingTest {
    fn perform_test(_view: Weak<Self>) -> Result<()> {
        set_record_probe_count(64);

        SELECTED.lock().clear();

        check_unscrolled()?;

        // Gaps are purely visual for touch: a tap in a gap selects
        // the nearest cell, each gap side goes to its closer cell.
        inject_touches(
            "
                 50   50   b
                 50   50   e
                300  150   b
                300  150   e
                196   50   b
                196   50   e
                204   50   b
                204   50   e
                100   94   b
                100   94   e
                100  102   b
                100  102   e
            ",
        );

        assert_eq!(SELECTED.lock().deref(), "|0||3||0||1||0||2|");
        SELECTED.lock().clear();

        inject_scroll(-1000);

        check_scrolled_to_bottom()?;

        inject_touches(
            "
                300  550   b
                300  550   e
            ",
        );

        assert_eq!(SELECTED.lock().deref(), "|11|");

        Ok(())
    }
}
