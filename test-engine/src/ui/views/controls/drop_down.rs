use crate::gm::{LossyConvert, Toggle, color::WHITE, flat::Size};
use refs::Weak;
use crate::ui::{Button, Label, Setup, ToLabel, View, ViewData, ViewFrame, ViewSubviews, view};
use vents::Event;

use crate::{
    self as test_engine,
    ui::{CellRegistry, TableData, TableView},
};

#[view]
pub struct DropDown<T: 'static> {
    values:  Vec<T>,
    opened:  bool,
    changed: Event<T>,

    custom_format: Option<Box<dyn Fn(T) -> String>>,

    selected_index: usize,

    #[init]
    button: Button,
    label:  Label,
    table:  TableView,
}

impl<T: ToLabel + Clone + 'static> DropDown<T> {
    pub fn on_changed(&self, action: impl FnMut(T) + Send + 'static) {
        self.changed.val(action);
    }

    pub fn try_get_value(&self) -> Option<&T> {
        self.values.get(self.selected_index)
    }

    pub fn value(&self) -> &T {
        assert!(!self.values.is_empty());
        self.values.get(self.selected_index).unwrap()
    }

    pub fn set_values(&mut self, values: Vec<T>) {
        self.selected_index = 0;
        self.values = values;

        if self.values.is_empty() {
            self.label.set_text("");
            return;
        }

        let first = self.values.first().unwrap().clone();

        if let Some(format) = &self.custom_format {
            self.label.set_text(format(first));
        } else {
            self.label.set_text(first);
        }

        let size: Size = (
            self.width(),
            self.height() * self.number_of_cells().lossy_convert(),
        )
            .into();

        self.table.set_size(size.width, size.height);
    }

    pub fn custom_format(&mut self, format: impl Fn(T) -> String + 'static) {
        self.custom_format = Some(Box::new(format));
        self.set_values(self.values.clone());
    }

    fn tapped(&mut self) {
        if self.opened.toggle() {
            self.label.set_hidden(false);
            self.button.set_hidden(false);
            self.table.set_hidden(true);
        } else {
            self.label.set_hidden(true);
            self.button.set_hidden(true);
            self.table.set_hidden(false);
            let table_height = self.height() * self.number_of_cells().lossy_convert();
            let width = self.width();
            self.table.set_size(width, table_height);
            self.table.reload_data();

            if self.superview().height() - self.max_y() < table_height {
                let y = -table_height + self.height();
                self.table.set_y(y);
            } else {
                self.table.set_y(0);
            }
        }
    }
}

impl<T: ToLabel + Clone + 'static> Setup for DropDown<T> {
    fn setup(mut self: Weak<Self>) {
        self.button.place().back();
        self.button.on_tap(move || self.tapped());

        self.label.set_color(WHITE).place().back();

        self.table.set_data_source(self).register_cell::<Label>();
        self.table.set_hidden(true);
    }
}

impl<T: ToLabel + Clone + 'static> TableData for DropDown<T> {
    fn number_of_cells(&self) -> usize {
        self.values.len()
    }

    fn cell_height(&self, _: usize) -> f32 {
        self.height()
    }

    fn setup_cell(&mut self, index: usize, registry: &mut CellRegistry) -> Weak<dyn View> {
        let this = self.weak();
        let cell = registry.cell::<Label>();
        cell.__base_view().view_label += "DropDown cell: ";

        let val = this.values[index].clone();

        cell.set_color(WHITE);

        if let Some(format) = &this.custom_format {
            cell.set_text(format(val));
        } else {
            cell.set_text(val);
        }

        cell
    }

    fn cell_selected(&mut self, index: usize) {
        self.selected_index = index;
        let val = &self.values[index];
        if let Some(format) = &self.custom_format {
            self.label.set_text(format(val.clone()));
        } else {
            self.label.set_text(val.clone());
        }
        self.changed.trigger(val.clone());
        self.tapped();
    }
}
