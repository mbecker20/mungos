use crate::Collection;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Result,
};
use serde::de::DeserializeOwned;
use std::str::FromStr;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_many_by_id(&self, ids: &Vec<String>) -> Result<Vec<T>> {
        let ids: Vec<ObjectId> = ids
            .iter()
            .map(|id| ObjectId::from_str(id).unwrap())
            .collect();
        let mut cursor = self
            .collection
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
