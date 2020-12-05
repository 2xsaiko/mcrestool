use std::borrow::Cow;

use darling::ast::{Data, Fields, Style};
use quote::quote;
use syn::export::TokenStream2;

use crate::common::{to_idents, to_struct_fields, BinSerdeField, BinSerdeOpts, BinSerdeVariant};

pub fn impl_bin_serialize(opts: &BinSerdeOpts) -> TokenStream2 {
    let name = &opts.ident;
    let body = match &opts.data {
        Data::Enum(variants) => gen_variants(&variants),
        Data::Struct(s) => gen_serialize_fields(s),
    };

    let gen = quote! {
        impl ::binserde::BinSerialize for #name {
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
            #idx.serialize(&mut serializer)?;
            #( #fields.serialize(&mut serializer)?; )*
            Ok(())
        }
    }
}