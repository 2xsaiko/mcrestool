use std::borrow::Cow;

use darling::ast::{Data, Fields, Style};
use quote::quote;
use syn::export::TokenStream2;

use crate::common::*;

pub fn impl_bin_serialize(opts: &BinSerdeOpts) -> TokenStream2 {
    let name = &opts.ident;
    let body = match &opts.data {
        Data::Enum(variants) => gen_variants(&variants),
        Data::Struct(s) => gen_serialize_fields(s),
    };

    let generic_defs = generic_defs(opts).map(|el| quote!(<#el>));
    let generic_params = generic_params_on_target(opts).map(|el| quote!(<#el>));
    let where_clause = add_trait_bounds(opts, &quote!(::binserde::BinSerialize));

    let gen = quote! {
        impl #generic_defs ::binserde::BinSerialize for #name #generic_params #where_clause {
            fn serialize<S: ::binserde::BinSerializer>(&self, mut serializer: S) -> ::binserde::Result<()> {
                #body
            }
        }
    };

    gen
}

fn gen_serialize_fields(fields: &Fields<BinSerdeField>) -> TokenStream2 {
    let idents = to_struct_fields(fields, true);

    let serializers = fields.iter().filter(|el| !el.skip).map(|el| {
        let mut expr = quote!(&mut serializer);

        if el.no_dedup {
            expr = quote!(::binserde::BinSerializer::disable_dedup(#expr));
        }

        expr
    });

    quote! {
        #( self.#idents.serialize( #serializers )?; )*
        Ok(())
    }
}

fn gen_variants(variants: &[BinSerdeVariant]) -> TokenStream2 {
    if !variants.is_empty() {
        let variants = variants
            .iter()
            .enumerate()
            .map(|(idx, el)| gen_variant_impl(idx, el));
        quote! {
            match self {
                #( #variants )*
            }
        }
    } else {
        quote! {
            unreachable!()
        }
    }
}

fn gen_variant_impl(idx: usize, variant: &BinSerdeVariant) -> TokenStream2 {
    let name = &variant.ident;
    let fs = &variant.fields;
    let (args, fields) = match variant.fields.style {
        Style::Tuple => {
            let idents = to_idents(fs, true);
            (quote! { ( #( #idents ),* ) }, idents)
        }
        Style::Struct => {
            let idents: Vec<_> = fs
                .iter()
                .filter(|el| !el.skip)
                .map(|el| Cow::Borrowed(el.ident.as_ref().unwrap()))
                .collect();
            (quote! { { #( #idents ),* } }, idents)
        }
        Style::Unit => (quote!(), vec![]),
    };
    quote! {
        Self::#name #args => {
            ::binserde::BinSerialize::serialize(&#idx, &mut serializer)?;
            #( ::binserde::BinSerialize::serialize(&#fields, &mut serializer)?; )*
            Ok(())
        }
    }
}
