//! Deletes entries matching the filter in the specified collection.
//!
//! # Examples
//!
//! ```
//!     use mungos::{Mungos, doc}
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
//! 		// deletes any items that match field: apple
//!     collection.delete_many(doc! { "field": "apple" }).await.unwrap();
//! ```
//!

use crate::Collection;
use mongodb::{bson::Document, error::Result};

impl<T> Collection<T> {
    pub async fn delete_many(&self, filter: Document) -> Result<()> {
        self.collection.delete_many(filter, None).await?;
        Ok(())
    }
}
