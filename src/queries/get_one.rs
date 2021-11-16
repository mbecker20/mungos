//! Retrieves a single document from a collection.


use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Result,
};
use serde::de::DeserializeOwned;
use std::str::FromStr;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_one(&self, id: &str) -> Result<Option<T>> {
        let item = self
            .collection
            .find_one(Some(doc! { "_id": ObjectId::from_str(id).unwrap() }), None)
            .await?;
        Ok(item)
    }
}
