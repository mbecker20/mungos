//! Retrieves a single document from a collection.
//!
//! Retrieves one document from the collection if it's ObjectID (or "_id") field is the same as the one passed to the function.
//! If no document in the collection has a matching ObjectID, the function returns None.
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
//!     let id = "...";
//!     let item = collection.get_one(id).await.unwrap();
//! ```
//!

use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Result,
};
use serde::de::DeserializeOwned;
use std::str::FromStr;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_one(&self, id: &str) -> Result<T> {
        let item = self
            .collection
            .find_one(Some(doc! { "_id": ObjectId::from_str(id).unwrap() }), None)
            .await?
            .unwrap();
        Ok(item)
    }
}
