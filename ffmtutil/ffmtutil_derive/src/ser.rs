use darling::ast::{Data, Style};
use quote::quote;
use syn::export::Span;
use syn::export::TokenStream2;
use syn::Ident;

use crate::common::{BinSerdeOpts, BinSerdeVariant, StructField, to_struct_fields};

pub fn impl_bin_serialize(opts: &BinSerdeOpts) -> TokenStream2 {
    let name = &opts.ident;
    let body = match &opts.data {
        Data::Enum(variants) => gen_variants(&variants),
        Data::Struct(s) => {
            let fields = to_struct_fields(s);

            gen_serialize_fields(&fields)
        }
    };

    let gen = quote! {
        impl ::ffmtutil::BinSerialize for #name {
            fn serialize<S: ::ffmtutil::BinSerializer>(&self, mut serializer: S) -> ::ffmtutil::Result<()> {
                #body
            }
        }
    };

    eprintln!("{}", gen);
    gen
}

fn gen_serialize_fields(fields: &[StructField]) -> TokenStream2 {
    quote! {
        #( self.#fields.serialize(&mut serializer)?; )*
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
    let fs = &variant.fields.fields;
    let (args, fields) = match variant.fields.style {
        Style::Tuple => {
            let fields: Vec<_> = fs
                .iter()
                .enumerate()
                .map(|(idx, _el)| Ident::new(&format!("v{}", idx), Span::call_site()))
                .collect();
            (quote! { ( #( #fields ),* ) }, fields)
        }
        Style::Struct => {
            let fields: Vec<_> = fs.iter().map(|el| el.ident.clone().unwrap()).collect();
            (quote! { { #( #fields ),* } }, fields)
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
