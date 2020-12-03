use darling::ast::{Data, Fields, Style};
use quote::quote;
use syn::export::TokenStream2;
use syn::Index;

use crate::common::{to_idents, to_struct_fields, BinSerdeField, BinSerdeOpts, BinSerdeVariant};

pub fn impl_bin_deserialize(opts: &BinSerdeOpts) -> TokenStream2 {
    let name = &opts.ident;
    let deserialize_body = gen_deserialize_method_body(opts);

    let deserialize_in_place_m = match &opts.data {
        Data::Enum(_) => quote!(),
        Data::Struct(fields) => {
            let body = gen_deserialize_in_place_method_body(fields);
            quote! {
                fn deserialize_in_place<D: ::ffmtutil::BinDeserializer<'de>>(&mut self, mut deserializer: D) -> ::ffmtutil::Result<()> {
                    #body
                }
            }
        }
    };

    let gen = quote! {
        impl<'de> ::ffmtutil::BinDeserialize<'de> for #name {
            fn deserialize<D: ::ffmtutil::BinDeserializer<'de>>(mut deserializer: D) -> ::ffmtutil::Result<Self> {
                #deserialize_body
            }

            #deserialize_in_place_m
        }
    };

    eprintln!("{}", gen);
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
                    expr = quote!(::ffmtutil::BinDeserializer::disable_dedup(#expr));
                }

                quote!( ::ffmtutil::BinDeserialize::deserialize( #expr )? )
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
                    x @ _ => Err(::ffmtutil::Error::custom(&format!("invalid variant {}", x))),
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
                expr = quote!(::ffmtutil::BinDeserializer::disable_dedup(#expr));
            }

            quote!( ::ffmtutil::BinDeserialize::deserialize_in_place( &mut self.#field, #expr )?; )
        }
    });

    quote! {
        #( #exprs )*
        Ok(())
    }
}
