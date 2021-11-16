//! Retrieves multiple documents by their id field.
//! 
//! Updates a single document in the collection, replacing it with the document provided into this function.
//! 
//! The input must implement the Serializable trait in order for mongoDB to store it. 
//! 
//! # Examples
//! 
//! ``` 
//!     use mungos::{Mungos};
//!     use serde::{Serialize, Deserialize};
//!     
//!     #[derive(Debug, Serialize, Deserialize)]
//!     struct Item {  
//!         field: String
//!     }
//! 
//!     let db = Mungos.new("uri", "app name", timeout).await;
//!     let collection = db.connection::<Item>("db name", "collection name");
//!     
//!     let id = "..."
//!     let new_item = Item { field: "..." }
//!     let items = collection.update_one(id, item).await.unwrap();    
//! ```
//! 

use crate::Collection;
use mongodb::{
    bson::{doc, oid::ObjectId, to_bson},
    error::Result,
};
use serde::Serialize;
use std::str::FromStr;

impl<T: Serialize> Collection<T> {
    pub async fn update_one(&self, id: &str, item: T) -> Result<T> {
        let filter = doc! { "_id": ObjectId::from_str(id).unwrap() };
        let update = doc! { "$set": to_bson(&item)? };
        self.collection.update_one(filter, update, None).await?;
        Ok(item)
    }
}
