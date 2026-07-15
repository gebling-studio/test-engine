use anyhow::Result;
use test_engine::{
    dispatch::from_main,
    refs::Weak,
    ui::{GREEN, Label, Setup, TableData, TableView, View, ViewData, ViewFrame, ViewTest, view},
    ui_test::{check_colors, inject_scroll},
};

#[view]
struct TableViewResize {
    #[init]
    table: TableView,
}

impl Setup for TableViewResize {
    fn setup(self: Weak<Self>) {
        self.table.set_frame((20, 20, 200, 200));
        self.table.set_data_source(self).register_cell::<Label>();
    }
}

impl TableData for TableViewResize {
    fn cell_height(&self, _: usize) -> f32 {
        50.0
    }

    fn number_of_cells(&self) -> usize {
        1
    }

    fn setup_cell(&mut self, _index: usize, registry: &mut test_engine::ui::CellRegistry) -> Weak<dyn View> {
        let cell = registry.cell::<Label>();
        cell.set_color(GREEN);
        cell.set_text("alalalalal");
        cell
    }
}

fn check_initial_cell() -> Result<()> {
    check_colors(
        r"
            448    4 -  89 124 149
            24   24 -   0 255   0
            52   24 -   0 255   0
            80   24 -   0 255   0
            152   24 -   0 255   0
            184   24 -   0 255   0
            216   24 -   0 255   0
            592   36 -  89 124 149
            116   40 -   0 255   0
            200   44 -   0 255   0
            64   48 -   0 255   0
            92   48 -   0 255   0
            116   48 -   0 255   0
            140   48 -   0 255   0
            164   48 -   0 255   0
            24   68 -   0 255   0
            48   68 -   0 255   0
            128   68 -   0 255   0
            152   68 -   0 255   0
            180   68 -   0 255   0
            208   68 -   0 255   0
            592  184 -  89 124 149
            372  204 -  89 124 149
            200  248 -  89 124 149
            44  332 -  89 124 149
            516  388 -  89 124 149
            300  404 -  89 124 149
            148  448 -  89 124 149
            404  564 -  89 124 149
            4  592 -  89 124 149
            212  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )
}

fn check_cell_after_scroll() -> Result<()> {
    for i in 0..5 {
        inject_scroll(i);
    }

    check_colors(
        r"
            448    4 -  89 124 149
            24   24 -   0 255   0
            52   24 -   0 255   0
            80   24 -   0 255   0
            152   24 -   0 255   0
            184   24 -   0 255   0
            216   24 -   0 255   0
            592   36 -  89 124 149
            116   40 -   0 255   0
            200   44 -   0 255   0
            64   48 -   0 255   0
            92   48 -   0 255   0
            116   48 -   0 255   0
            140   48 -   0 255   0
            164   48 -   0 255   0
            24   68 -   0 255   0
            48   68 -   0 255   0
            128   68 -   0 255   0
            152   68 -   0 255   0
            180   68 -   0 255   0
            208   68 -   0 255   0
            592  184 -  89 124 149
            372  204 -  89 124 149
            200  248 -  89 124 149
            44  332 -  89 124 149
            516  388 -  89 124 149
            300  404 -  89 124 149
            148  448 -  89 124 149
            404  564 -  89 124 149
            4  592 -  89 124 149
            212  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )
}

fn check_resized_table(view: Weak<TableViewResize>) -> Result<()> {
    from_main(move || {
        view.table.set_size(400, 100);
    });

    check_colors(
        r"
            24   24 -   0 255   0
            116   24 -   0 255   0
            312   24 -   0 255   0
            364   24 -   0 255   0
            416   24 -   0 255   0
            68   28 -   0 255   0
            216   40 -   0 255   0
            164   48 -   0 255   0
            192   48 -   0 255   0
            216   48 -   0 255   0
            240   48 -   0 255   0
            264   48 -   0 255   0
            372   52 -   0 255   0
            40   68 -   0 255   0
            88   68 -   0 255   0
            128   68 -   0 255   0
            300   68 -   0 255   0
            344   68 -   0 255   0
            396   68 -   0 255   0
            592   92 -  89 124 149
            56  200 -  89 124 149
            184  240 -  89 124 149
            412  252 -  89 124 149
            592  280 -  89 124 149
            8  324 -  89 124 149
            300  396 -  89 124 149
            524  436 -  89 124 149
            132  444 -  89 124 149
            404  556 -  89 124 149
            4  592 -  89 124 149
            216  592 -  89 124 149
            592  592 -  89 124 149
        ",
    )
}

impl ViewTest for TableViewResize {
    fn perform_test(view: Weak<Self>) -> Result<()> {
        check_initial_cell()?;
        check_cell_after_scroll()?;
        check_resized_table(view)?;

        for i in 0..5 {
            inject_scroll(-i);
        }

        Ok(())
    }
}
