mod all_views;
mod async_link_button;
mod cast_cell;
mod launch_app;
mod view;

use proc_macro::TokenStream;

use crate::{
    all_views::all_views_impl, async_link_button::async_link_button_impl, cast_cell::cast_cell_impl,
    launch_app::launch_app_impl, view::view_impl,
};

/// Turns a struct into a view, and registers it as a UI test if it implements
/// `ViewTest`.
#[proc_macro_attribute]
pub fn view(attr: TokenStream, stream: TokenStream) -> TokenStream {
    view_impl(attr, stream)
}

#[proc_macro]
pub fn all_views(_: TokenStream) -> TokenStream {
    all_views_impl()
}

#[proc_macro]
pub fn launch_app(_: TokenStream) -> TokenStream {
    launch_app_impl()
}

#[proc_macro]
pub fn async_link_button(input: TokenStream) -> TokenStream {
    async_link_button_impl(input)
}

#[proc_macro]
pub fn cast_cell(input: TokenStream) -> TokenStream {
    cast_cell_impl(input)
}
