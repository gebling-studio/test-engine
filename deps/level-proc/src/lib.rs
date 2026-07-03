use std::str::FromStr;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    __private::TokenStream2, Data, DeriveInput, Field, Fields, GenericParam, parse::Parser,
    parse_macro_input, spanned::Spanned,
};

#[proc_macro_attribute]
#[allow(clippy::too_many_lines)]
pub fn level(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let mut stream = parse_macro_input!(stream as DeriveInput);

    let Data::Struct(data) = &mut stream.data else {
        panic!("`level` macro has to be used with structs")
    };

    let name = &stream.ident;

    let _name_str =
        TokenStream2::from_str(&format!("\"{name}\"")).expect("Failed to extract level struct name");

    let generics = &stream.generics;

    let type_param_names: Vec<_> = generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Type(type_param) => Some(type_param.ident.clone()),
            GenericParam::Const(const_param) => Some(const_param.ident.clone()),
            GenericParam::Lifetime(_) => None,
        })
        .collect();

    let type_params = quote_spanned! {stream.generics.span()=>
        #(#type_param_names),*
    };

    let Fields::Named(fields) = &mut data.fields else {
        panic!("No named fields");
    };

    fields.named.insert(
        0,
        Field::parse_named
            .parse2(quote! { __level_base: test_engine::level::LevelBase })
            .expect("parse2(quote! { __level_base: test_engine::level::LevelBase })"),
    );

    quote! {
        #stream

        impl #generics test_engine::level::Level for #name <#type_params> { }

        impl #generics test_engine::level::LevelInternal for #name <#type_params> {
            fn __internal_setup(&self) {
                use test_engine::level::LevelSetup;
                let mut level = test_engine::refs::weak_from_ref(self);
                if level.needs_physics() {
                    level.init_physics();
                }
                level.setup();
            }

            fn __internal_update(&self, frame_time: f32) {
                use test_engine::level::Level;
                use test_engine::level::LevelSetup;
                let mut level = test_engine::refs::weak_from_ref(self);
                level.update_physics(frame_time);
                level.update();
            }
        }

        impl #generics test_engine::refs::AsAny for #name <#type_params> {
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

        impl #generics std::ops::Deref for #name <#type_params> {
            type Target = test_engine::level::LevelBase;
            fn deref(&self) -> &test_engine::level::LevelBase {
                &self.__level_base
            }
        }
        impl #generics std::ops::DerefMut for #name <#type_params>  {
            fn deref_mut(&mut self) -> &mut test_engine::level::LevelBase {
                &mut self.__level_base
            }
        }
    }
    .into()
}
