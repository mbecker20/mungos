use futures::stream::TryStreamExt;
use mongodb::{bson::doc, error::Result, options::FindOptions};
use serde::de::DeserializeOwned;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_most_recent(
        &self,
        num_items: i64,
    ) -> Result<Vec<T>> {
        let find_options = FindOptions::builder()
            .sort(doc! { "_id": -1 })
            .limit(num_items)
            .build();
        let mut cursor = self.collection.find(doc! {}, find_options).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        items.reverse();
        Ok(items)
    }
}
