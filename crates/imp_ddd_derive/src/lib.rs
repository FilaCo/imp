use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Attribute, Data, DeriveInput, Fields};

#[proc_macro_derive(Entity, attributes(entity))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let data = input.data;
    let target_attr: Attribute = parse_quote!(#[entity(id)]);
    let (field_id_name, field_id_ty) = match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let field_id = fields
                    .named
                    .iter()
                    .filter(|f| f.attrs.contains(&target_attr))
                    .take(1)
                    .next()
                    .expect("id field not found");

                (&field_id.ident, &field_id.ty)
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let expanded = quote! {
        impl #impl_generics imp_ddd::prelude::Entity for #name #ty_generics #where_clause {
            type Id = #field_id_ty;

            fn id(&self) -> &Self::Id {
                &self.#field_id_name
            }
        }

        impl #impl_generics PartialEq for #name #ty_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                self.id() == other.id()
            }
        }

        impl #impl_generics Eq for #name #ty_generics #where_clause {}
    };

    expanded.into()
}

#[proc_macro_derive(VO, attributes(vo))]
pub fn derive_vo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = input.generics;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics imp_ddd::prelude::VO for #name #ty_generics #where_clause {}
    };

    expanded.into()
}
