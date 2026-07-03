use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub(crate) fn launch_app_impl() -> TokenStream {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let use_statement = crate_name.replace('-', "_");

    let crate_ident = Ident::new(&use_statement, Span::call_site().into());

    let output = quote! {
        pub use #crate_ident::test_engine_create_app;
        test_engine::test_engine_start_app();
    };

    output.into()
}
