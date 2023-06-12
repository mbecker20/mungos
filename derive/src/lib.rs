use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr};

#[proc_macro_derive(
    MungosColl,
    attributes(index, unique_index, doc_index, unique_doc_index)
)]
pub fn derive_collection(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, attrs, data, ..
    } = parse_macro_input!(input as DeriveInput);

    let mut doc_indexes = Vec::new();
    let mut unique_doc_indexes = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("unique_doc_index") {
            let doc = attr.parse_args::<Expr>().expect("expected doc! macro");
            unique_doc_indexes.push(doc);
        }
        if attr.path().is_ident("doc_index") {
            let doc = attr.parse_args::<Expr>().expect("expected doc! macro");
            doc_indexes.push(doc);
        }
    }

    let s = match data {
        Data::Struct(s) => s,
        _ => panic!("must derive on struct"),
    };

    let mut unique_indexes = Vec::new();
    let mut indexes = Vec::new();

    for field in s.fields {
        let is_unique = field
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("unique_index"));
        if is_unique {
            unique_indexes.push(field.ident.expect("field must have ident"));
            continue;
        }
        let is_index = field.attrs.iter().any(|attr| attr.path().is_ident("index"));
        if is_index {
            indexes.push(field.ident.expect("field must have ident"));
        }
    }

    quote! {
        impl #ident {
            pub async fn collection(mungos: &mungos::Mungos, db: &str, create_index: bool)
                -> anyhow::Result<mungos::Collection<#ident>>
            {
                let coll = mungos.collection(db, stringify!(#ident));

                if create_index {
                    #(coll.create_unique_index_from_doc(#unique_doc_indexes).await?;)*
                    #(coll.create_index_from_doc(#doc_indexes).await?;)*
                    #(coll.create_unique_index(stringify!(#unique_indexes)).await?;)*
                    #(coll.create_index(stringify!(#indexes)).await?;)*
                }

                Ok(coll)
            }
        }
    }
    .into()
}
