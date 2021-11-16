//! Retrieves the n most recent documents in a collection.
//! 
//! Retrieves multiple documents from a collection by their id field.
//! 
//! Retriees the n most recent items from the collection, and returns a Vec of the type that the collection is assigned to.
//! 
//! If latest_first is set to true, the 0th index of the Vec will be the most recent item that was added to the collection, 
//! and if set to false, the 0th index of the Vec will be the nth item that was added to the collection.
//! 
//! If n is greater than the number of documents in the collection, the first document added to the collection 
//! will either be the 0th or the last index, depending on whether latest_first is set to true or false. 
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
//!     //returns the 10 most recent docs in the collection, with the latest doc at the 0th index of the Vec
//!     let latest_first = collection.get_most_recent(n, true).await.unwrap();
//!     
//!     //returns the 10 most recent docs in the colelction, with the latest doc at the last index of the Vec
//!     let oldest_first = collection.get_most_recent(n, false).await.unwrap();
//! 
//! ```
//! 


use futures::stream::TryStreamExt;
use mongodb::{bson::doc, error::Result, options::FindOptions};
use serde::de::DeserializeOwned;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_most_recent(&self, num_items: i64, latest_first: bool) -> Result<Vec<T>> {
        let find_options = FindOptions::builder()
            .sort(doc! { "_id": -1 })
            .limit(num_items)
            .build();
        let mut cursor = self.collection.find(doc! {}, find_options).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        if !latest_first {items.reverse()};
        Ok(items)
    }
}
