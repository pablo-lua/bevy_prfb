use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Index, GenericParam, token::Comma, TypeParam, ConstParam, punctuated::Punctuated, parse_quote, parse::{Parse, ParseBuffer, ParseStream}, DeriveInput, spanned::Spanned};



#[proc_macro_derive(PrefabData)]
pub fn derive_system_param(input: TokenStream) -> TokenStream {
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

    let mut fields = Vec::new();
    for (i, field) in field_definitions.iter().enumerate() {
        let i = Index::from(i);
        fields.push(
            field
                .ident
                .as_ref()
                .map(|f| quote! { #f })
                .unwrap_or_else(|| quote! { #i }),
        );
    }
    let struct_name = &ast.ident;
    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    TokenStream::from(quote! {
        const _: () = {
            impl #impl_generics bevy_prfb::prefab::PrefabData for #struct_name #ty_generics #where_clause {
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