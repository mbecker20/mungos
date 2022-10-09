//! Retrieves multiple documents by their id field.
//!
//! Updates a single document in the collection, replacing it with the document provided into this function.
//!
//! The input must implement the Serializable trait in order for mongoDB to store it.
//!
//! # Examples
//!
//! ```
//!     use mungos::{Mungos, Update, doc, to_bson};
//!     use serde::{Serialize, Deserialize};
//!     
//!     #[derive(Debug, Serialize, Deserialize)]
//!     struct Item {
//!         field0: String,
//!         field1: String,
//!     }
//!
//!     #[derive(Debug, Serialize, Deserialize, Clone)]
//!     struct ItemUpdate {
//!         field1: String // update can push a different serializable type to perform fine grained updates
//!     }
//!
//!     let db = Mungos::new("uri", "app name", timeout).await;
//!     let collection = db.connection::<Item>("db name", "collection name");
//!     
//!     let id = "..."
//!     let update = ItemUpdate { field1: "..." }
//!     collection.update_one(id, Update::Regular(update.clone())).await.unwrap();
//!     collection.update_one(id, Update::Custom(doc! { "$set": to_bson(update) }))
//! ```
//!

use crate::Collection;
use anyhow::Context;
use mongodb::bson::{doc, oid::ObjectId, to_bson, Document};
use serde::Serialize;
use std::str::FromStr;

pub enum Update<T> {
    Regular(T),
    Set(Document),
    Custom(Document),
}

impl<Any> Collection<Any> {
    pub async fn update_one<T: Serialize>(
        &self,
        id: &str,
        update: Update<T>,
    ) -> anyhow::Result<()> {
        let filter =
            doc! { "_id": ObjectId::from_str(id).context("failed to parse id into ObjectId")? };
        let update = match update {
            Update::Regular(update) => {
                doc! { "$set": to_bson(&update)? }
            }
            Update::Set(doc) => doc! { "$set": doc },
            Update::Custom(doc) => doc,
        };
        self.collection.update_one(filter, update, None).await?;
        Ok(())
    }
}
