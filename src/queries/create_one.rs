//! Creates a single entry in the specified collection.
//! 
//! Takes in a struct which implements the Serialize trait, and adds it to the collection. 
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
//!     let item = Item { field: "".to_string() };
//!     collection.create_one(item).await.unwrap();
//! 
//! ```
//! 


use mongodb::error::Result;
use serde::Serialize;

use crate::Collection;

impl<T: Serialize> Collection<T> {
    pub async fn create_one(&self, item: T) -> Result<String> {
        let res = self.collection.insert_one(item, None).await?;
        Ok(res.inserted_id.as_object_id().unwrap().to_string())
    }
}
