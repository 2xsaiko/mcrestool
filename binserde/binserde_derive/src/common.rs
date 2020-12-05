use std::borrow::Cow;

use darling::{FromDeriveInput, FromField, FromVariant};
use darling::ast::{Fields, Style};
use syn::{Generics, Ident, Type};
use syn::export::{Span, TokenStream2, ToTokens};

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

fn move_sort<T, F>(slice: &mut [T], op: F)
    where
        F: Fn(&T) -> Option<usize>,
{
    let more = slice.len();
    let mut idx = 0;

    while more > 0 {
        let d = &slice[idx];
        let new_idx = op(d);

        if let Some(new_idx) = new_idx {
            if new_idx < idx {}
            idx = new_idx;
        }

        idx += 1;
    }
}

fn swap_at<T>(slice: &mut [T], idx: usize) {
    if idx == 0 || idx == slice.len() {
        return;
    }

    let (left, right) = slice.split_at_mut(idx);

    if left.len() == right.len() {
        left.swap_with_slice(right);
    } else if left.len() < right.len() {
        swap_outer(slice, idx);
        let len = slice.len() - idx;
        swap_at(&mut slice[..len], idx);
    } else if right.len() < left.len() {
        let count = slice.len() - idx;
        swap_outer(slice, count);
        swap_at(&mut slice[idx..], count);
    }
}

fn swap_outer<T>(slice: &mut [T], count: usize) {
    let (left, right) = slice.split_at_mut(count);
    let i = right.len() - count;
    let right = &mut right[i..];
    left.swap_with_slice(right);
}

#[test]
fn test_swap_at() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    swap_at(&mut arr, 3);
    assert_eq!([4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 1, 2, 3], arr);
}
