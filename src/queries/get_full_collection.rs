use mongodb::error::Result;
use serde::de::DeserializeOwned;

use crate::Database;

impl Database {
	pub async fn get_full_collection<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        db_name: &str,
        collection_name: &str,
    ) -> Result<Vec<T>> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        let mut cursor = collection.find(None, None).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        Ok(items)
    }
}