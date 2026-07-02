use gm::{color::CLEAR, flat::Size};
use hreads::{from_main, on_main};
use refs::{Own, Weak};
use vents::OnceEvent;

use crate::{
    ScrimView, Setup, TouchStack, UIColor, UIManager, View, ViewData, ViewFrame, view::ViewSubviews,
};

pub trait ModalView<In = (), Out: 'static = ()>: 'static + View + Default {
    fn show_modally(view: Self) -> Weak<Self> {
        let mut view = Own::new(view);
        view.set_z_position(UIManager::MODAL_Z_OFFSET);
        let size = Self::modal_size();
        let weak = view.weak();
        TouchStack::push_layer(weak.weak_view());

        // The scrim owns the modal, so hiding removes both at once.
        // It sits one subview step behind the modal and in front of
        // everything else. It is not a touch layer, the modal already
        // blocks touches under it.
        let mut scrim = ScrimView::new();
        scrim.set_z_position(UIManager::MODAL_Z_OFFSET + UIManager::subview_z_offset());
        let scrim = UIManager::root_view().add_subview_to_root(scrim);
        scrim.set_color(Self::modal_scrim_color());
        scrim.place().back();
        scrim.add_subview(view);

        weak.place().size(size.width, size.height).center();
        weak
    }

    fn prepare_modally() -> Weak<Self> {
        Self::show_modally(Self::default())
    }

    fn prepare_modally_with_input(input: In) -> Weak<Self> {
        let view = Self::prepare_modally();
        view.setup_input(input);
        view
    }

    fn show_modally_with_input(input: In, callback: impl FnOnce(Out) + 'static + Send)
    where
        In: 'static + Send,
        Out: Send, {
        on_main(move || {
            let weak = Self::prepare_modally_with_input(input);
            weak.modal_event().val(callback);
        });
    }

    #[allow(async_fn_in_trait)]
    async fn show_modally_async(input: In) -> Out
    where
        In: 'static + Send,
        Out: Send, {
        from_main(|| Self::prepare_modally_with_input(input).modal_event().receiver().recv().unwrap())
    }

    fn hide_modal(self: Weak<Self>, result: Out)
    where Out: Send {
        on_main(move || {
            let mut scrim = *self.superview();
            scrim.remove_from_superview();
            TouchStack::pop_layer(self.weak_view());
            self.modal_event().trigger(result);
        });
    }

    fn modal_event(&self) -> &OnceEvent<Out>;

    fn modal_size() -> Size;

    /// The color of the fullscreen backdrop behind the modal.
    /// Transparent by default, override to dim the background.
    fn modal_scrim_color() -> UIColor {
        CLEAR.into()
    }

    fn setup_input(self: Weak<Self>, _: In) {}
}
