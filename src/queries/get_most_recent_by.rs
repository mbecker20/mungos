//! Retrieves the n most recent documents in a collection, ordered by a numerical field (such as timestamp).
//!
//! Retrieves the n / skip most recent items from the collection, pushed back by offset,
//! and returns a Vec of the type that the collection is assigned to.
//!
//! note a skip of 1 doesn't skip any documents.
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
//! 	let field = "timestamp";
//!     let n = 10;
//!     let skip = 2;
//!     //returns the n / skip = 5 most recent docs in the collection,
//!     //in order of oldest to latest
//!     let items = collection.get_most_recent_with_skip(field, n, skip, 0, None, None).await.unwrap();
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
    pub async fn get_most_recent_by(
        &self,
        field: &str,
        num_items: i64,
        skip: i64,
        offset: u64,
        filter: impl Into<Option<Document>>,
        projection: impl Into<Option<Document>>,
    ) -> Result<Vec<T>> {
        let find_options = FindOptions::builder()
            .sort(doc! { field: -1 })
            .skip(offset)
            .limit(num_items * skip)
            .projection(projection)
            .build();
        let mut cursor = self.collection.find(filter, find_options).await?;
        let mut items = Vec::new();
        let mut step = 0;
        while let Some(item) = cursor.try_next().await? {
            if step % skip == 0 {
                items.push(item);
            }
            step += 1;
        }
        items.reverse();
        Ok(items)
    }
}
