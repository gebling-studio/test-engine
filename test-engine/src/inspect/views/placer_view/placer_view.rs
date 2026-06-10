use inspect::ui::ViewRepr;
use refs::Weak;
use ui::{Setup, UIEvent, View, ViewData};
use ui_proc::view;

use crate::{
    inspect::views::LayoutRuleCell,
    ui::{CellRegistry, TableData, TableView},
};

mod test_engine {
    pub(crate) use educe;
    pub(crate) use refs;

    pub(crate) use crate::ui;
}

#[view]
pub struct PlacerView {
    pub rule_changed: UIEvent,

    view: Weak<ViewRepr>,

    #[init]
    table: TableView,
}

impl Setup for PlacerView {
    fn setup(self: Weak<Self>) {
        self.place().all_ver();
        self.table.set_data_source(self).register_cell::<LayoutRuleCell>();
    }
}

impl PlacerView {
    pub fn set_view(mut self: Weak<Self>, view: Weak<ViewRepr>) {
        self.view = view;
        self.table.reload_data();
    }
}

impl TableData for PlacerView {
    fn cell_height(&self, _: usize) -> f32 {
        50.0
    }

    fn number_of_cells(&self) -> usize {
        let Some(view) = self.view.get() else {
            return 0;
        };
        view.placer.get_rules().len()
    }

    fn setup_cell(&mut self, index: usize, registry: &mut CellRegistry) -> Weak<dyn View> {
        let cell = registry.cell::<LayoutRuleCell>();

        if !self.view.is_ok() {
            return cell;
        }

        cell.set_data(self.view, index);
        let this = self.weak();
        cell.editing_ended.sub(this, move || {
            this.rule_changed.trigger(());
        });

        cell
    }
}
