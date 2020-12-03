use darling::ast::{Data, Fields, Style};
use quote::quote;
use syn::export::Span;
use syn::export::TokenStream2;
use syn::Ident;

use crate::common::{to_struct_fields, BinSerdeField, BinSerdeOpts, BinSerdeVariant};

pub fn impl_bin_serialize(opts: &BinSerdeOpts) -> TokenStream2 {
    let name = &opts.ident;
    let body = match &opts.data {
        Data::Enum(variants) => gen_variants(&variants),
        Data::Struct(s) => gen_serialize_fields(s),
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

fn gen_serialize_fields(fields: &Fields<BinSerdeField>) -> TokenStream2 {
    let idents = to_struct_fields(fields);

    let serializers = fields.iter().map(|el| {
        let mut expr = quote!(&mut serializer);

        if el.no_dedup {
            expr = quote!(::ffmtutil::BinSerializer::disable_dedup(#expr));
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
