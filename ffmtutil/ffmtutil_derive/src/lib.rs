extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;

#[proc_macro_derive(BinSerialize)]
pub fn bin_serialize_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("failed to parse token stream");
    impl_bin_serialize(&ast)
}

fn impl_bin_serialize(ast: &syn::DeriveInput) -> TokenStream {
    // TODO
    let name = &ast.ident;
    let gen = quote! {
        impl ffmtutil::BinSerialize for #name {

        }
    };
    gen.into()
}
