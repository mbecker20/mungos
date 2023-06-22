use std::{collections::HashMap, hash::Hash, path::PathBuf};

use mongodb::bson::{oid::ObjectId, Document};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Collection, Mungos};

#[async_trait::async_trait]
pub trait Indexed: Serialize + DeserializeOwned + Sync {
    fn name() -> &'static str {
        ""
    }
    fn indexes() -> Vec<String> {
        Vec::new()
    }
    fn unique_indexes() -> Vec<String> {
        Vec::new()
    }
    fn sparse_indexes() -> Vec<String> {
        Vec::new()
    }
    fn doc_indexes() -> Vec<Document> {
        Vec::new()
    }
    fn unique_doc_indexes() -> Vec<Document> {
        Vec::new()
    }
    fn sparse_doc_indexes() -> Vec<Document> {
        Vec::new()
    }
    async fn collection(
        mungos: &Mungos,
        db_name: &str,
        create_index: bool,
    ) -> anyhow::Result<Collection<Self>> {
        let coll = mungos.collection(db_name, Self::name());

        if create_index {
            for index in Self::indexes() {
                coll.create_index(&index).await?;
            }
            for unique_index in Self::unique_indexes() {
                coll.create_unique_index(&unique_index).await?;
            }
            for sparse_index in Self::sparse_indexes() {
                coll.create_sparse_index(&sparse_index).await?;
            }
            for doc_index in Self::doc_indexes() {
                coll.create_index_from_doc(doc_index).await?;
            }
            for unique_doc_index in Self::unique_doc_indexes() {
                coll.create_unique_index_from_doc(unique_doc_index).await?;
            }
            for sparse_doc_index in Self::sparse_doc_indexes() {
                coll.create_sparse_index_from_doc(sparse_doc_index).await?;
            }
        }

        Ok(coll)
    }
    async fn document_collection(mungos: &Mungos, db_name: &str) -> Collection<Document> {
        mungos.collection(db_name, Self::name())
    }
}

macro_rules! impl_indexed_basic {
    ($($ty:ty),*) => {
        $(impl Indexed for $ty {})*
        $(impl Indexed for Option<$ty> {})*
    };
}

macro_rules! impl_indexed_nested {
    ($($ty:ty),*) => {
        $(impl<T: Serialize + DeserializeOwned + Sync> Indexed for $ty {})*
        $(impl<T: Serialize + DeserializeOwned + Sync> Indexed for Option<$ty> {})*
    };
}

impl_indexed_basic!(
    String, PathBuf, bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, f32, f64,
    isize, ObjectId, Document
);
impl_indexed_nested!(Vec<T>);

impl<
        K: Serialize + DeserializeOwned + Sync + Eq + Hash,
        V: Serialize + DeserializeOwned + Sync,
    > Indexed for HashMap<K, V>
{
}
