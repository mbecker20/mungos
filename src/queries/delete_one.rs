//! Deletes a single entry in the specified collection.
//!
//! Uses the ObjectID (_id) field of the object and removes it from the collection.
//!
//! # Examples
//!
//! ```
//!     use mungos::{Mungos, ObjectId}
//!     use serde::{Serialize, Deserialize}
//!     
//!     #[derive(Debug, Serialize, Deserialize)]
//!     struct Item {
//!         _id: ObjectId
//!         field: String
//!     }
//!
//!     let db = Mungos::new("uri", "app name", timeout).await;
//!     let collection = db.connection::<Item>("db name", "collection name");
//!     
//!     let id = "...";
//!     collection.delete_one(id).await.unwrap();
//!
//! ```
//!

use crate::Collection;
use anyhow::Context;
use mongodb::bson::{doc, oid::ObjectId};
use std::str::FromStr;

impl<T> Collection<T> {
    pub async fn delete_one(&self, id: &str) -> anyhow::Result<String> {
        let filter =
            doc! { "_id": ObjectId::from_str(id).context("failed to convert id into ObjectId")? };
        self.collection.delete_one(filter, None).await?;
        Ok(id.to_string())
    }
}
