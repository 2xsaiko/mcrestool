use darling::ast::{Data, Fields, Style};
use quote::quote;
use syn::export::TokenStream2;
use syn::Index;

use crate::common::*;

pub fn impl_bin_deserialize(opts: &BinSerdeOpts) -> TokenStream2 {
    let name = &opts.ident;
    let deserialize_body = gen_deserialize_method_body(opts);

    let deserialize_in_place_m = match &opts.data {
        Data::Enum(_) => quote!(),
        Data::Struct(fields) => {
            let body = gen_deserialize_in_place_method_body(fields);
            quote! {
                fn deserialize_in_place<D: ::binserde::BinDeserializer<'de>>(&mut self, mut deserializer: D) -> ::binserde::Result<()> {
                    #body
                }
            }
        }
    };

    let generic_defs = generic_defs(opts).map_or_else(||quote!(<'de>), |el| quote!(<'de, #el>));
    let generic_params = generic_params_on_target(opts).map(|el| quote!(<#el>));
    let where_clause = add_trait_bounds(opts, &quote!(::binserde::BinDeserialize<'de>));

    let gen = quote! {
        impl #generic_defs ::binserde::BinDeserialize<'de> for #name #generic_params #where_clause {
            fn deserialize<D: ::binserde::BinDeserializer<'de>>(mut deserializer: D) -> ::binserde::Result<Self> {
                #deserialize_body
            }

            #deserialize_in_place_m
        }
    };

    gen
}

fn gen_deserialize_method_body(opts: &BinSerdeOpts) -> TokenStream2 {
    fn gen_struct_like(struct_like: TokenStream2, fields: &Fields<BinSerdeField>) -> TokenStream2 {
        let idents = to_idents(fields, false);

        let fields_list = quote! { #( #idents ),* };
        let struct_value = match fields.style {
            Style::Tuple => quote! { #struct_like ( #fields_list ) },
            Style::Struct => quote! { #struct_like { #fields_list } },
            Style::Unit => quote! { #struct_like },
        };

        let exprs = fields.iter().map(|el| {
            if el.skip {
                quote!(Default::default())
            } else {
                let mut expr = quote!(&mut deserializer);

                if el.no_dedup {
                    expr = quote!(::binserde::BinDeserializer::disable_dedup(#expr));
                }

                quote!( ::binserde::BinDeserialize::deserialize( #expr )? )
            }
        });

        quote! {
            #( let #idents = #exprs; )*
            Ok( #struct_value )
        }
    }

    fn gen_variant_impl(index: usize, variant: &BinSerdeVariant) -> TokenStream2 {
        let name = &variant.ident;
        let index = Index::from(index);
        let g = gen_struct_like(quote!(Self::#name), &variant.fields);
        quote! {
            #index => { #g }
        }
    }

    match &opts.data {
        Data::Enum(variants) if variants.is_empty() => {
            let ident = opts.ident.to_string();
            quote! {
                panic!("can't deserialize empty enum {}", #ident)
            }
        }
        Data::Enum(variants) => {
            let variants = variants
                .iter()
                .enumerate()
                .map(|(idx, el)| gen_variant_impl(idx, el));
            quote! {
                match usize::deserialize(&mut deserializer)? {
                    #( #variants )*
                    x @ _ => Err(::binserde::Error::custom(&format!("invalid variant {}", x))),
                }
            }
        }
        Data::Struct(fields) => gen_struct_like(quote!(Self), fields),
    }
}

fn gen_deserialize_in_place_method_body(fields: &Fields<BinSerdeField>) -> TokenStream2 {
    let idents = to_struct_fields(fields, false);

    let exprs = fields.iter().zip(idents.iter()).map(|(el, field)| {
        if el.skip {
            quote!(self.#field = Default::default();)
        } else {
            let mut expr = quote!(&mut deserializer);

            if el.no_dedup {
                expr = quote!(::binserde::BinDeserializer::disable_dedup(#expr));
            }

            quote!( ::binserde::BinDeserialize::deserialize_in_place( &mut self.#field, #expr )?; )
        }
    });

    quote! {
        #( #exprs )*
        Ok(())
    }
}
