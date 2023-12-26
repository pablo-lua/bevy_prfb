use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Index, GenericParam, token::Comma, TypeParam, ConstParam, punctuated::Punctuated, parse_quote, parse::{Parse, ParseBuffer, ParseStream}, DeriveInput, spanned::Spanned};



#[proc_macro_derive(PrefabData)]
pub fn derive_system_param(input: TokenStream) -> TokenStream {
    let token_stream = input.clone();
    let ast = parse_macro_input!(input as DeriveInput);
    let syn::Data::Struct(syn::DataStruct {
        fields: field_definitions,
        ..
    }) = ast.data
    else {
        return syn::Error::new(
            ast.span(),
            "Invalid `PrefabData` type: expected a `struct`",
        )
        .into_compile_error()
        .into();
    };

    let mut field_locals = Vec::new();
    let mut fields = Vec::new();
    let mut field_types = Vec::new();
    for (i, field) in field_definitions.iter().enumerate() {
        field_locals.push(format_ident!("f{i}"));
        let i = Index::from(i);
        fields.push(
            field
                .ident
                .as_ref()
                .map(|f| quote! { #f })
                .unwrap_or_else(|| quote! { #i }),
        );
        field_types.push(&field.ty);
    }
    let struct_name = &ast.ident;

    TokenStream::from(quote! {
        // We define the FetchState struct in an anonymous scope to avoid polluting the user namespace.
        // The struct can still be accessed via SystemParam::State, e.g. EventReaderState can be accessed via
        // <EventReader<'static, 'static, T> as SystemParam>::State
        const _: () = {
            impl bevy_prfb::prefab::PrefabData for #struct_name {
                fn insert_into_entity(self, entity: &mut bevy::ecs::world::EntityWorldMut) {
                    #(
                        self.#fields.insert_into_entity(entity);
                    )*
                }
                fn load_sub_assets(&mut self, world: &mut bevy::ecs::world::World) -> bool {
                    let mut loaded = false;
                    #(
                        loaded |= self.#fields.load_sub_assets(world);
                    )*
                    loaded
                }
            }
        };
    })
}