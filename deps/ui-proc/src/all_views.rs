use proc_macro::TokenStream;
use quote::quote;

use crate::view::{VIEW_TESTS, VIEWS};

pub fn all_views_impl() -> TokenStream {
    let views = VIEWS.lock();
    let views = views.iter();

    quote! {
        [#(#views),*]
    }
    .into()
}

pub fn all_view_tests_impl() -> TokenStream {
    let tests = VIEW_TESTS.lock();
    let tests = tests.iter();

    quote! {
        [#(#tests),*]
    }
    .into()
}
