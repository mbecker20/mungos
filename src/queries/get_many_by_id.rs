use std::str::FromStr;
use futures::stream::TryStreamExt;
use mongodb::{bson::{oid::ObjectId, doc}, error::Result};
use serde::de::DeserializeOwned;
use crate::Database;

impl Database {
  	pub async fn get_many_by_id<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        db_name: &str,
        collection_name: &str,
        ids: &Vec<String>,
    ) -> Result<Vec<T>> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        let ids: Vec<ObjectId> = ids
            .iter()
            .map(|id| ObjectId::from_str(id).unwrap())
            .collect();
        let mut cursor = collection
            .find(
                doc! {
                    "_id": {
                        "$in": ids
                    }
                },
                None,
            )
            .await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        Ok(items)
    }
}