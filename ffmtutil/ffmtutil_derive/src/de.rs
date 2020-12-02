use proc_macro::TokenStream;
use std::borrow::Cow;

use darling::{FromDeriveInput, FromField, FromVariant};
use darling::ast::{Data, Fields, Style};
use quote::{quote, ToTokens};
use syn::{Generics, Ident, Type};
use syn::export::Span;
use syn::export::TokenStream2;

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(binserde), supports(struct_any, enum_any))]
pub struct BinDeserializeOpts {
    ident: Ident,
    generics: Generics,
    data: darling::ast::Data<BinSerdeVariant, BinSerdeField>,
}

#[derive(FromVariant, Debug)]
#[darling(attributes(binserde))]
struct BinSerdeVariant {
    ident: Ident,
    fields: Fields<BinSerdeField>,
}

#[derive(FromField, Debug)]
#[darling(attributes(binserde))]
struct BinSerdeField {
    ident: Option<syn::Ident>,
    ty: Type,
}

pub fn impl_bin_deserialize(opts: &BinDeserializeOpts) -> TokenStream {
    let name = &opts.ident;
    let body = match &opts.data {
        Data::Enum(variants) => gen_variants(&variants),
        Data::Struct(s) => {
            gen_deserialize_fields(s)
        }
    };

    let gen = quote! {
        impl<'de> ffmtutil::BinDeserialize<'de> for #name {
            fn serialize<D: ffmtutil::BinDeserializer<'de>>(&self, mut deserializer: D) -> ffmtutil::Result<()> {
                #body
            }
        }
    };

    let out = gen.into();
    eprintln!("{}", out);
    out
}

enum StructField<'a> {
    Tuple(syn::Index),
    Struct(&'a syn::Ident),
}

impl ToTokens for StructField<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            StructField::Tuple(v) => v.to_tokens(tokens),
            StructField::Struct(v) => v.to_tokens(tokens),
        }
    }
}

fn to_struct_fields<'a>(fields: &'a Fields<BinSerdeField>) -> Vec<StructField<'a>> {
    match fields.style {
        Style::Tuple => fields
            .fields
            .iter()
            .enumerate()
            .map(|(idx, _el)| StructField::Tuple(syn::Index::from(idx)))
            .collect(),
        Style::Struct => fields
            .fields
            .iter()
            .map(|el| StructField::Struct(el.ident.as_ref().unwrap()))
            .collect(),
        Style::Unit => vec![],
    }
}

fn gen_deserialize_fields(fields: &Fields<BinSerdeField>) -> TokenStream2 {
    let idents = to_idents(fields);
    let struct_fields = to_struct_fields(fields);
    quote! {
        #( let #idents = ffmtutil::BinDeserialize::deserialize(&mut deserializer)?; )*
        Ok(Self { #( #idents )* })
    }
}

fn gen_deserialize_in_place_fields(fields: &Fields<BinSerdeField>) -> TokenStream2 {
    quote! {
        unimplemented!()
    }
}

struct EnumVariant<'a> {
    ident: &'a Ident,
    fields: Vec<StructField<'a>>,
}

fn gen_variants(variants: &[BinSerdeVariant]) -> TokenStream2 {
    let variants = variants.iter().map(|el| gen_variant_impl(el));
    quote! {
        match self {
            #( #variants )*
        }
    }
}

fn to_idents(fields: &Fields<BinSerdeField>) -> Vec<Cow<Ident>> {
    match fields.style {
        Style::Tuple => {
            fields
                .iter()
                .enumerate()
                .map(|(idx, _el)| Ident::new(&format!("v{}", idx), Span::call_site()).into())
                .collect()
        }
        Style::Struct => {
            fields.iter().map(|el| el.ident.as_ref().unwrap().into()).collect()
        }
        Style::Unit => Vec::new(),
    }
}

fn gen_variant_impl(variant: &BinSerdeVariant) -> TokenStream2 {
    let name = &variant.ident;
    let fs = &variant.fields.fields;
    let fields = to_idents(&variant.fields);
    let args = match variant.fields.style {
        Style::Tuple => quote! { ( #( #fields ),* ) },
        Style::Struct => quote! { { #( #fields ),* } },
        Style::Unit => quote!(),
    };
    quote! {
        Self::#name #args => {
            #( #fields.serialize(&mut serializer)?; )*
            Ok(())
        }
    }
}
