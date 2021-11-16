use futures::stream::TryStreamExt;
use mongodb::{bson::doc, error::Result, options::FindOptions};
use serde::de::DeserializeOwned;

use crate::Database;

impl Database {
    pub async fn get_most_recent<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        db_name: &str,
        collection_name: &str,
        num_items: i64,
    ) -> Result<Vec<T>> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        let find_options = FindOptions::builder()
            .sort(doc! { "_id": -1 })
            .limit(num_items)
            .build();
        let mut cursor = collection.find(doc! {}, find_options).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        items.reverse();
        Ok(items)
    }
}
