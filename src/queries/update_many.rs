//! Updates multiple documents in a collection by their id field.


use mongodb::{bson::Document, error::Result};

use crate::Collection;

impl<Any> Collection<Any> {
    pub async fn update_many(&self, query: Document, update: Document) -> Result<()> {
        self.collection.update_many(query, update, None).await?;
        Ok(())
    }
}
