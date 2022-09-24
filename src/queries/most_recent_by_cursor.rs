use mongodb::{
    bson::{doc, Document},
    error::Result,
    options::FindOptions,
    Cursor,
};
use serde::de::DeserializeOwned;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn most_recent_by_cursor(
        &self,
        most_recent_by_field: &str,
        num_items: i64,
        offset: u64,
        filter: impl Into<Option<Document>>,
        projection: impl Into<Option<Document>>,
    ) -> Result<Cursor<T>> {
        let find_options = FindOptions::builder()
            .sort(doc! { most_recent_by_field: -1 })
            .skip(offset)
            .limit(num_items)
            .projection(projection)
            .build();
        let cursor = self.collection.find(filter, find_options).await?;
        Ok(cursor)
    }
}
