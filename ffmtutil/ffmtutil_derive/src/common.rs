use std::borrow::Cow;

use darling::ast::{Fields, Style};
use darling::{FromDeriveInput, FromField, FromVariant};
use syn::export::{Span, ToTokens, TokenStream2};
use syn::{Generics, Ident, Type};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(binserde), supports(struct_any, enum_any))]
pub struct BinSerdeOpts {
    pub ident: Ident,
    pub generics: Generics,
    pub data: darling::ast::Data<BinSerdeVariant, BinSerdeField>,
}

#[derive(FromVariant, Debug)]
#[darling(attributes(binserde))]
pub struct BinSerdeVariant {
    pub ident: Ident,
    pub fields: Fields<BinSerdeField>,
    #[darling(default)]
    pub index: Option<usize>,
}

#[derive(FromField, Debug)]
#[darling(attributes(binserde))]
pub struct BinSerdeField {
    pub ident: Option<syn::Ident>,
    pub ty: Type,
    #[darling(default)]
    pub no_dedup: bool,
    #[darling(default)]
    pub skip: bool,
    #[darling(default)]
    pub index: Option<usize>,
}

pub enum StructField<'a> {
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

pub fn to_struct_fields(fields: &Fields<BinSerdeField>, skip: bool) -> Vec<StructField> {
    let iter = fields
        .iter()
        .enumerate()
        .filter(|(_, el)| !skip || !el.skip);

    match fields.style {
        Style::Tuple => iter
            .map(|(idx, _el)| StructField::Tuple(syn::Index::from(idx)))
            .collect(),
        Style::Struct => iter
            .map(|(_, el)| StructField::Struct(el.ident.as_ref().unwrap()))
            .collect(),
        Style::Unit => vec![],
    }
}

pub fn to_idents(fields: &Fields<BinSerdeField>, skip: bool) -> Vec<Cow<Ident>> {
    let iter = fields
        .iter()
        .enumerate()
        .filter(|(_, el)| !skip || !el.skip);

    match fields.style {
        Style::Tuple => iter
            .map(|(idx, _el)| Cow::Owned(Ident::new(&format!("v{}", idx), Span::call_site())))
            .collect(),
        Style::Struct => iter
            .map(|(_, el)| Cow::Borrowed(el.ident.as_ref().unwrap()))
            .collect(),
        Style::Unit => Vec::new(),
    }
}
