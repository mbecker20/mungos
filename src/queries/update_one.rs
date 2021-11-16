use serde::Serialize;
use mongodb::{error::Result, bson::{doc, oid::ObjectId, to_bson}};
use std::str::FromStr;
use crate::Database;

impl Database {
  pub async fn update_one<T: Serialize>(
        &self,
        db_name: &str,
        collection_name: &str,
        id: &str,
        item: T,
    ) -> Result<T> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        let filter = doc! { "_id": ObjectId::from_str(id).unwrap() };
        let update = doc! { "$set": to_bson(&item)? };
        collection.update_one(filter, update, None).await?;
        Ok(item)
    }
}