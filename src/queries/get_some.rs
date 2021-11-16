//! Retrieves some documents from a collection based on options.
//! 
//! Retrieves multiple documents from a collection based on the FindOptions provided to the function.
//! 
//! FindOptions are created using the mongoDB FindOptions struct, found in mongodb::options::FindOptions;
//! 
//! # Examples
//! 
//! ``` 
//!     use mungos::{Mungos};
//!     use mungos::mongodb::{options::FindOptions, bson::doc};
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
//!     //gets the 10 most recent documents 
//!     let find_options = FindOptions::builder().sort(doc! { _id: -1 }).limit(10).build();
//!     let items = collection.get_some(find_options).await.unwrap();    
//! 
//!     //gets all the documents in the collection, but skips every fifth document.
//!     let find_options_2 = FindOptions::builder().skip(5).build()
//!     let items_2 = collection.get_some(find_options).await.unwpra()
//! ```
//! 


use futures::stream::TryStreamExt;
use mongodb::{error::Result, options::FindOptions};
use serde::de::DeserializeOwned;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn get_some(&self, find_options: Option<FindOptions>) -> Result<Vec<T>> {
        let mut cursor = self.collection.find(None, find_options).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        Ok(items)
    }
}
