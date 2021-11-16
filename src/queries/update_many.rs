use mongodb::{bson::Document, error::Result};
use serde::Serialize;

use crate::Database;

impl Database {
    pub async fn update_many<T: Serialize>(
        &self,
        db_name: &str,
        collection_name: &str,
        query: Document,
        update: Document,
    ) -> Result<()> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        collection.update_many(query, update, None).await?;
        Ok(())
    }
}
