use futures::stream::TryStreamExt;
use mongodb::{error::Result, options::FindOptions};
use serde::de::DeserializeOwned;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_some(&self, find_options: Option<FindOptions>) -> Result<Vec<T>> {
        let mut cursor = self.collection.find(None, find_options).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        Ok(items)
    }
}
