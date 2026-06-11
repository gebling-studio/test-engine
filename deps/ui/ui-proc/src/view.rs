use parking_lot::Mutex;
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{
    __private::TokenStream2,
    Attribute, Data, DeriveInput, Field, Fields, FieldsNamed, Ident, LitStr, Meta, Path, Token, Type,
    parse::{Parse, ParseStream, Parser},
    parse_macro_input, parse_quote,
    token::{Bracket, Pound},
};

pub(crate) static VIEWS: Mutex<Vec<String>> = Mutex::new(Vec::new());
pub(crate) static VIEW_TESTS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// `#[view(crate = some::path)]` - the path providing `ui`, `refs` and
/// `educe`. Defaults to `test_engine`.
struct ViewArgs {
    root: Path,
}

impl Parse for ViewArgs {
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

#[allow(clippy::too_many_lines)]
pub fn view_impl(attr: TokenStream, stream: TokenStream, test: bool) -> TokenStream {
    let root = parse_macro_input!(attr as ViewArgs).root;
    let mut stream = parse_macro_input!(stream as DeriveInput);

    let Data::Struct(data) = &mut stream.data else {
        return syn::Error::new(stream.ident.span(), "`view` macro can only be used on structs")
            .to_compile_error()
            .into();
    };

    let name = &stream.ident;

    let name_str = LitStr::new(&name.to_string(), name.span());

    VIEWS.lock().push(name.to_string());

    if test {
        VIEW_TESTS.lock().push(format!("{} {:#?}", name, Span::call_site().file()));
    }

    let (impl_generics, ty_generics, where_clause) = stream.generics.split_for_impl();

    let Fields::Named(fields) = &mut data.fields else {
        return syn::Error::new(
            stream.ident.span(),
            "`view` struct must have named fields: `struct Name { ... }`",
        )
        .to_compile_error()
        .into();
    };

    let inits = add_inits(name, fields, &root);

    fields.named.insert(
        0,
        Field::parse_named
            .parse2(quote! { __view_base: #root::ui::ViewBase })
            .expect("parse2(quote! { __view_base: #root::ui::ViewBase })"),
    );

    let ui_test_related_stuff = if test {
        quote! {
            #[#root::__internal_macro_deps::ctor::ctor(crate_path = #root::__internal_macro_deps::ctor)]
            fn store_test() {
                crate::UI_TESTS
                    .lock()
                    .insert(#name_str.to_string(), run_ui_test);
            }

            #[test]
            fn ui_test() -> anyhow::Result<()> {
                #root::ui_test::run_test_app(env!("CARGO_MANIFEST_DIR"), #name_str)
            }

            pub fn run_ui_test() -> anyhow::Result<()> {
                use #root::ui::ViewTest;
                #name::perform_test(#root::ui_test::UITest::start::<#name>())
            }
        }
    } else {
        quote!()
    };

    quote! {


        #[derive(#root::educe::Educe)]
        #[educe(Default)]
        #stream

        impl #impl_generics #root::ui::View for #name #ty_generics #where_clause {
            fn weak_view(&self) -> #root::refs::Weak<dyn #root::ui::View> {
                #root::refs::weak_from_ref(self as &dyn #root::ui::View)
            }
            fn __base_view(&self) -> &mut #root::ui::ViewBase {
                #![allow(clippy::transmute_ptr_to_ptr)]
                unsafe { std::mem::transmute(&self.__view_base) }
            }
            fn __init_views(&mut self) {
                use #root::ui::ViewSubviews;
                #inits
            }
            fn as_cell(&mut self) -> &mut dyn #root::ui::CellCallbacks {
                self as &mut dyn #root::ui::CellCallbacks
            }
        }

        impl #impl_generics #root::refs::AsAny for #name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn std::any::Any {
               self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
               self
            }

            fn into_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
        }

        impl #impl_generics #root::ui::__ViewInternalSetup for #name #ty_generics #where_clause {
            fn __internal_before_setup(&mut self) {
                use #root::ui::Setup;
                let mut weak = #root::refs::weak_from_ref(self);
                weak.before_setup();
            }

            fn __internal_setup(&mut self) {
                use #root::ui::Setup;
                use #root::ui::WithHeader;
                use #root::ui::ViewData;
                self.__view_base.view_label = #name_str.to_string();
                self.layout_header();
                let mut weak = #root::refs::weak_from_ref(self);
                weak.setup();
                self.__view_base.events.setup.trigger(());
            }

            fn __internal_inspect(&mut self) {
                use #root::ui::Setup;
                let mut weak = #root::refs::weak_from_ref(self);
                weak.inspect();
            }

            fn __internal_on_selection_changed(&mut self, selected: bool) {
                use #root::ui::Setup;
                let mut weak = #root::refs::weak_from_ref(self);
                weak.on_selection_changed(selected);
            }
        }

        impl #impl_generics #root::ui::__ViewIntoUnsizedOwn for #name #ty_generics #where_clause {
            unsafe fn __into_unsized_own<V: ?Sized + #root::ui::View + 'static>(own: #root::refs::Own<V>) -> #root::refs::Own<dyn #root::ui::View> {
                use #root::refs::Own;
                use #root::ui::View;

                assert!(own.sized());
                assert_eq!(size_of::<Own<Self>>(), size_of::<Own<V>>());

                let unsized_own: Own<Self> = unsafe { std::mem::transmute_copy(&own) };
                std::mem::forget(own);
                unsized_own
            }
        }

        #ui_test_related_stuff

    }
    .into()
}

fn add_inits(root_name: &Ident, fields: &mut FieldsNamed, root: &Path) -> TokenStream2 {
    let mut res = quote!();

    let init_attr = Attribute {
        pound_token:   Pound::default(),
        style:         syn::AttrStyle::Outer,
        bracket_token: Bracket::default(),
        meta:          Meta::Path(parse_quote!(init)),
    };

    let mut inits_started = false;

    for field in &mut fields.named {
        if let Some(idx) = field.attrs.iter().position(|a| *a == init_attr) {
            field.attrs.remove(idx);
            inits_started = true;
        }

        if !inits_started {
            continue;
        }

        let name = field.ident.as_ref().expect("let name = field.ident.as_ref()");

        let ty = &field.ty;

        let weak_wrapped_type = Type::without_plus
            .parse2(quote! { #root::refs::Weak<#ty> })
            .expect("Type::without_plus.parse2(quote! { Weak<#ty> })");

        field.ty = weak_wrapped_type;

        let label = LitStr::new(&format!("{root_name}.{name}"), name.span());

        res = quote! {
            #res
            self.#name = self.__add_view_internal();
            self.#name.__base_view().view_label = format!("{}: {}", #label, self.#name.__base_view().view_label);
        }
    }

    res
}
