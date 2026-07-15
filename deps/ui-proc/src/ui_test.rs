use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    ItemFn, LitStr, Path, ReturnType, Token,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
};

/// `#[ui_test(crate = some::path)]` - the path providing `ui_test`. Defaults to
/// `test_engine`.
struct UITestArgs {
    root: Path,
}

impl Parse for UITestArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                root: parse_quote!(test_engine),
            });
        }

        input.parse::<Token![crate]>()?;
        input.parse::<Token![=]>()?;

        Ok(Self { root: input.parse()? })
    }
}

pub fn ui_test_impl(attr: TokenStream, stream: TokenStream) -> TokenStream {
    let root = parse_macro_input!(attr as UITestArgs).root;
    let func = parse_macro_input!(stream as ItemFn);

    if func.sig.asyncness.is_some() {
        return syn::Error::new(
            func.sig.ident.span(),
            "`ui_test` only works on a plain fn. A UI test drives the main thread through \
             `from_main` and never awaits, so it has no reason to be async.",
        )
        .to_compile_error()
        .into();
    }

    let name = &func.sig.ident;
    let name_str = LitStr::new(&name.to_string(), name.span());
    let store = format_ident!("__store_ui_test_{name}");
    let entry = format_ident!("__ui_test_entry_{name}");

    // A test that cannot fail says so in its signature. The registry holds one
    // fn type, so wrap that one rather than make every test return a `Result`
    // it never uses.
    let entry_fn = match func.sig.output {
        ReturnType::Default => quote! {
            fn #entry() -> anyhow::Result<()> {
                #name();
                Ok(())
            }
        },
        ReturnType::Type(..) => quote! {
            fn #entry() -> anyhow::Result<()> {
                #name()
            }
        },
    };

    quote! {
        #func

        #[#root::__internal_macro_deps::ctor::ctor(unsafe, crate_path = #root::__internal_macro_deps::ctor)]
        fn #store() {
            #entry_fn

            // The same engine owned map `#[view_test]` registers into. A test
            // written as a fn and one written as a view are both just UI tests,
            // so one registry holds them and one count covers them.
            assert!(
                #root::UI_TESTS
                    .lock()
                    .insert(#name_str.to_string(), #entry as fn() -> anyhow::Result<()>)
                    .is_none(),
                "Duplicate ui test: {}", #name_str,
            );
        }
    }
    .into()
}
