use mongodb::{error::Result, bson::Document, results::UpdateResult, options::{UpdateModifications, UpdateOptions}};

use crate::Collection;

impl<T> Collection<T> {
    pub async fn upsert(&self, query: Document, update: impl Into<UpdateModifications>) -> Result<UpdateResult> {
		let options = UpdateOptions::builder()
			.upsert(true)
			.build();
        self.collection.update_one(query, update, options).await
    }
}
