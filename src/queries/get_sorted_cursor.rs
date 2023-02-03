use mongodb::{
    bson::{doc, Document},
    error::Result,
    options::{Collation, FindOptions},
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
        batch_size: impl Into<Option<u32>>,
        numeric_ordering: impl Into<Option<bool>>,
    ) -> Result<Cursor<T>> {
        let direction = match sort_direction {
            SortDirection::Ascending => 1,
            SortDirection::Descending => -1,
        };
        let options = FindOptions::builder()
            .sort(doc! { sort_field: direction })
            .batch_size(batch_size)
            .collation(
                Collation::builder()
                    .locale("simple")
                    .numeric_ordering(numeric_ordering)
                    .build(),
            )
            .build();
        let cursor = self.collection.find(filter, options).await?;
        Ok(cursor)
    }
}
