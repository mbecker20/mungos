use mongodb::{
    bson::{doc, Document},
    error::Result,
    options::FindOptions,
    Cursor,
};
use serde::de::DeserializeOwned;

use crate::Collection;

pub enum SortDirection {
    Ascending,
    Descending,
}

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_sorted_cursor(
        &self,
        filter: impl Into<Option<Document>>,
        sort_field: &str,
        sort_direction: SortDirection,
    ) -> Result<Cursor<T>> {
        let direction = match sort_direction {
            SortDirection::Ascending => 1,
            SortDirection::Descending => -1,
        };
        let options = FindOptions::builder()
            .sort(doc! { sort_field: direction })
            .build();
        let cursor = self.collection.find(filter, options).await?;
        Ok(cursor)
    }
}
