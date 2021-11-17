//! Retrieves the n most recent documents in a collection.
//! 
//! Retrieves the n most recent items from the collection, and returns a Vec of the type that the collection is assigned to.
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
//!     let db = Mungos.new("uri", "app name", timeout).await;
//!     let collection = db.connection::<Item>("db name", "collection name");
//! 
//!     let n = 10
//!     let skip = 2
//!     //returns the n / skip = 5 most recent docs in the collection, 
//!     //in order of oldest to latest
//!     let items = collection.get_most_recent_with_skip(n, skip).await.unwrap();
//! 
//! ```
//! 


use futures::stream::TryStreamExt;
use mongodb::{bson::doc, error::Result, options::FindOptions};
use serde::de::DeserializeOwned;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_most_recent_with_skip(&self, num_items: i64, skip: u64) -> Result<Vec<T>> {
        let find_options = FindOptions::builder()
            .sort(doc! { "_id": -1 })
            .limit(num_items)
            .skip(skip)
            .build();
        let mut cursor = self.collection.find(doc! {}, find_options).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        items.reverse();
        Ok(items)
    }
}