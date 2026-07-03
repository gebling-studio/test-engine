use std::any::type_name;

use refs::{Own, Weak};

use crate::{
    gm::flat::Size,
    ui::{View, view::view_frame::ViewFrame},
    window::RenderPass,
};

pub trait ViewCallbacks {
    fn update(&mut self);
    fn before_render(&self, pass: &mut RenderPass);
    fn content_size(&self) -> &Size;
    fn clips_to_bounds(&self) -> bool;
    fn theme_changed(&mut self);
}

impl<T: ?Sized + View> ViewCallbacks for T {
    default fn update(&mut self) {}
    default fn before_render(&self, _pass: &mut RenderPass) {}
    default fn content_size(&self) -> &Size {
        &self.frame().size
    }
    default fn clips_to_bounds(&self) -> bool {
        false
    }
    default fn theme_changed(&mut self) {}
}

pub trait __ViewInternalSetup {
    fn __internal_before_setup(&mut self);
    fn __internal_setup(&mut self);
    fn __internal_inspect(&mut self);
    fn __internal_on_selection_changed(&mut self, selected: bool);
}

pub trait Setup {
    fn new() -> Own<Self>
    where Self: Default;
    fn setup(self: Weak<Self>);
    fn on_selection_changed(self: Weak<Self>, selected: bool);
    fn before_setup(self: Weak<Self>);
    fn inspect(self: Weak<Self>);
}

impl<T: View + 'static> Setup for T {
    fn new() -> Own<Self>
    where Self: Default {
        let own = Own::<Self>::default();
        own.__base_view().view_label = type_name::<Self>().to_string();
        own
    }

    default fn setup(self: Weak<Self>) {}

    default fn on_selection_changed(self: Weak<Self>, _selected: bool) {}

    default fn before_setup(self: Weak<Self>) {}
    default fn inspect(self: Weak<Self>) {}
}
