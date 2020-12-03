use darling::ast::{Data, Fields, Style};
use quote::quote;
use syn::export::TokenStream2;
use syn::Index;

use crate::common::{BinSerdeField, BinSerdeOpts, BinSerdeVariant, to_idents, to_struct_fields};

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
        let idents = to_idents(fields);

        let fields_list = quote! { #( #idents ),* };
        let struct_value = match fields.style {
            Style::Tuple => quote! { #struct_like ( #fields_list ) },
            Style::Struct => quote! { #struct_like { #fields_list } },
            Style::Unit => quote! { #struct_like },
        };

        quote! {
            #( let #idents = ::ffmtutil::BinDeserialize::deserialize(&mut deserializer)?; )*
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
    let idents = to_struct_fields(fields);

    quote! {
        #( self.#idents = ::ffmtutil::BinDeserialize::deserialize(&mut deserializer)?; )*
        Ok(())
    }
}
