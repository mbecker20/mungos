use mongodb::{
    bson::Document,
    error::Result,
    options::{UpdateModifications, UpdateOptions},
    results::UpdateResult,
};

use crate::Collection;

impl<T> Collection<T> {
    pub async fn upsert(
        &self,
        query: Document,
        update: impl Into<UpdateModifications>,
    ) -> Result<UpdateResult> {
        let options = UpdateOptions::builder().upsert(true).build();
        self.collection.update_one(query, update, options).await
    }
}
