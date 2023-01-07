//! Retrieves the n most recent documents in a collection.
//!
//! Retrieves the n most recent items from the collection, pushed back by offset,
//! and returns a Vec of the type that the collection is assigned to.
//!
//! If n is greater than the number of documents in the collection, the 0th index of the returned Vec
//! will be the first document in the collection ..
//!
//! # Examples
//!
//! ```
//!     use mungos::{Mungos}
//!     use serde::{Serialize, Deserialize}
//!     
//!     #[derive(Debug, Serialize, Deserialize)]
//!     struct Item {  
//!         field: String
//!     }
//!
//!     let db = Mungos::new("uri", "app name", timeout).await;
//!     let collection = db.connection::<Item>("db name", "collection name");
//!
//!     let n = 10
//!     //returns the 10 most recent docs in the collection,
//!     //in order of oldest to latest
//!     //can pass a filter / projection doc
//!     let items = collection.get_most_recent(n, 0, None, None).await.unwrap();
//!
//! ```
//!

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    error::Result,
    options::FindOptions,
};
use serde::de::DeserializeOwned;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_most_recent(
        &self,
        index: &str,
        limit: i64,
        offset: u64,
        filter: impl Into<Option<Document>>,
        projection: impl Into<Option<Document>>,
    ) -> Result<Vec<T>> {
        let find_options = FindOptions::builder()
            .sort(doc! { index: -1 })
            .skip(offset)
            .limit(limit)
            .projection(projection)
            .build();
        let mut cursor = self.collection.find(filter, find_options).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        items.reverse();
        Ok(items)
    }
}
