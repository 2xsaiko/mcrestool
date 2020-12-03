extern crate proc_macro;

use proc_macro::TokenStream;

use darling::FromDeriveInput;
use quote::quote;
use syn::Member;

mod common;
mod de;
mod ser;

#[proc_macro_derive(BinSerialize, attributes(binserde))]
pub fn bin_serialize_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("failed to parse token stream");
    let opts: common::BinSerdeOpts = FromDeriveInput::from_derive_input(&ast).unwrap();
    ser::impl_bin_serialize(&opts).into()
}

#[proc_macro_derive(BinDeserialize, attributes(binserde))]
pub fn bin_deserialize_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("failed to parse token stream");
    let opts: common::BinSerdeOpts = FromDeriveInput::from_derive_input(&ast).unwrap();
    de::impl_bin_deserialize(&opts).into()
}

#[proc_macro]
pub fn member_to_ident(input: TokenStream) -> TokenStream {
    let ast: Member = syn::parse(input).expect("failed to parse token stream");
    match ast {
        Member::Named(x) => quote!(#x).into(),
        Member::Unnamed(x) => format!("v{}", x.index).parse().unwrap(),
    }
}
