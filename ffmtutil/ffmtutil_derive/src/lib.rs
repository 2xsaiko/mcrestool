extern crate proc_macro;

use proc_macro::TokenStream;

use darling::{FromDeriveInput, FromField, FromVariant};
use quote::quote;
use syn::Member;

mod ser;
mod de;

#[proc_macro_derive(BinSerialize)]
pub fn bin_serialize_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("failed to parse token stream");
    let opts: ser::BinSerializeOpts = FromDeriveInput::from_derive_input(&ast).unwrap();
    eprintln!("{:#?}", opts);
    ser::impl_bin_serialize(&opts)
}

#[proc_macro_derive(BinDeserialize)]
pub fn bin_deserialize_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("failed to parse token stream");
    let opts: de::BinDeserializeOpts = FromDeriveInput::from_derive_input(&ast).unwrap();
    eprintln!("{:#?}", opts);
    de::impl_bin_deserialize(&opts)
}

#[proc_macro]
pub fn member_to_ident(input: TokenStream) -> TokenStream {
    let ast: Member = syn::parse(input).expect("failed to parse token stream");
    match ast {
        Member::Named(x) => quote!(#x).into(),
        Member::Unnamed(x) => format!("v{}", x.index).parse().unwrap(),
    }
}
