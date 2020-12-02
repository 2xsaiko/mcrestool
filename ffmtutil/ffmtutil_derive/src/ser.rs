use proc_macro::TokenStream;

use darling::ast::{Data, Fields, Style};
use darling::{FromDeriveInput, FromField, FromVariant};
use quote::{quote, ToTokens};
use syn::export::Span;
use syn::export::TokenStream2;
use syn::{Generics, Ident, Type};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(binserde), supports(struct_any, enum_any))]
pub struct BinSerializeOpts {
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

pub fn impl_bin_serialize(opts: &BinSerializeOpts) -> TokenStream {
    let name = &opts.ident;
    let body = match &opts.data {
        Data::Enum(variants) => gen_variants(&variants),
        Data::Struct(s) => {
            let fields = from_fields(s);

            gen_serialize_fields(&fields)
        }
    };

    let gen = quote! {
        impl ffmtutil::BinSerialize for #name {
            fn serialize<S: ffmtutil::BinSerializer>(&self, mut serializer: S) -> ffmtutil::Result<()> {
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

fn from_fields<'a>(fields: &'a Fields<BinSerdeField>) -> Vec<StructField<'a>> {
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

fn gen_serialize_fields(fields: &[StructField]) -> TokenStream2 {
    quote! {
        #( self.#fields.serialize(&mut serializer)?; )*
        Ok(())
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

fn gen_variant_impl(variant: &BinSerdeVariant) -> TokenStream2 {
    let name = &variant.ident;
    let fs = &variant.fields.fields;
    let (args, fields) = match variant.fields.style {
        Style::Tuple => {
            let fields: Vec<_> = fs
                .iter()
                .enumerate()
                .map(|(idx, _el)| Ident::new(&format!("v{}", idx), Span::call_site()))
                .collect();
            (
                quote! {
                    ( #( #fields ),* )
                },
                fields,
            )
        }
        Style::Struct => {
            let fields: Vec<_> = fs.iter().map(|el| el.ident.clone().unwrap()).collect();
            (
                quote! {
                    { #( #fields ),* }
                },
                fields,
            )
        }
        Style::Unit => (quote!(), vec![]),
    };
    quote! {
        Self::#name #args => {
            #( #fields.serialize(&mut serializer)?; )*
            Ok(())
        }
    }
}
