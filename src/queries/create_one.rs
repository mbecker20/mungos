use mongodb::error::Result;
use serde::Serialize;

use crate::Database;

impl Database {
	pub async fn create_one<T: Serialize>(
        &self,
        db_name: &str,
        collection_name: &str,
        item: T,
    ) -> Result<String> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        let res = collection.insert_one(item, None).await?;
        Ok(res.inserted_id.as_object_id().unwrap().to_string())
    }
}