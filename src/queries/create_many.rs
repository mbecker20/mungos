//! Creates many documents in a collection.
//! 
//! Takes a Vec of structs of type T which implement the Serializable trait, and adds them sequentially to the collection,
//! in that the 0th index of the Vec is added first into the collection.
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
//!     let items = Vec::from([ { Item{...} }, { Item{...} }, { Item{...} }, ...])
//!     db.collection.create_many(items).await.unwrap()
//! ```
//! 

use mongodb::error::Result;
use serde::Serialize;

use crate::Collection;

impl<T: Serialize> Collection<T> {
    pub async fn create_many(&self, items: Vec<T>) -> Result<()> {
        self.collection.insert_many(items, None).await?;
        Ok(())
    }
}
