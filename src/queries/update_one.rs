//! Retrieves multiple documents by their id field.


use crate::Collection;
use mongodb::{
    bson::{doc, oid::ObjectId, to_bson},
    error::Result,
};
use serde::Serialize;
use std::str::FromStr;

impl<T: Serialize> Collection<T> {
    pub async fn update_one(&self, id: &str, item: T) -> Result<T> {
        let filter = doc! { "_id": ObjectId::from_str(id).unwrap() };
        let update = doc! { "$set": to_bson(&item)? };
        self.collection.update_one(filter, update, None).await?;
        Ok(item)
    }
}
