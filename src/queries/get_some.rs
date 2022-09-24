//! Retrieves some documents from a collection based on options.
//!
//! Retrieves multiple documents from a collection based on the FindOptions provided to the function,
//! and returns a Vec of the type that the collection is assigned to.
//!
//! FindOptions are created using the mongoDB FindOptions
//!
//! # Examples
//!
//! ```
//!     use mungos::{Mungos, FindOptions, doc, Serialize, Deserialize};
//!     
//!     #[derive(Debug, Serialize, Deserialize)]
//!     struct Item {  
//!         field: String
//!     }
//!
//!     let db = Mungos::new("uri", "app name", timeout, None).await;
//!     let collection = db.connection::<Item>("db name", "collection name");
//!     
//!     //gets the 10 most recent documents
//!     let filter = None;
//!     let find_options = FindOptions::builder().sort(doc! { _id: -1 }).limit(10).build();
//!     let items = collection.get_some(filter, find_options).await.unwrap();    
//!
//!     //gets all the documents in the collection, but skips every fifth document.
//!     let find_options_2 = FindOptions::builder().skip(5).build()
//!     let items_2 = collection.get_some(None, find_options_2).await.unwrap()
//! ```
//!

use futures::stream::TryStreamExt;
use mongodb::{bson::Document, error::Result, options::FindOptions};
use serde::de::DeserializeOwned;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_some(
        &self,
        filter: impl Into<Option<Document>>,
        find_options: impl Into<Option<FindOptions>>,
    ) -> Result<Vec<T>> {
        self.collection
            .find(filter, find_options)
            .await?
            .try_collect()
            .await
    }
}
