//! Retrieves all the documents in a collection.

use futures::stream::TryStreamExt;
use mongodb::error::Result;
use serde::de::DeserializeOwned;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_full_collection(&self) -> Result<Vec<T>> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        Ok(items)
    }
}
