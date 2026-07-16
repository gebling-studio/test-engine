use proc_macro::TokenStream;
use quote::quote;

use crate::view::VIEWS;

pub fn all_views_impl() -> TokenStream {
    let views = VIEWS.lock();
    let views = views.iter();

    quote! {
        [#(#views),*]
    }
    .into()
}
