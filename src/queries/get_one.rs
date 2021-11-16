use serde::de::DeserializeOwned;
use mongodb::{error::Result, bson::{doc, oid::ObjectId}};
use std::str::FromStr;

use crate::Database;

impl Database {
  pub async fn get_one<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        db_name: &str,
        collection_name: &str,
        id: &str,
      ) -> Result<T> {
          let collection = self
              .client
              .database(db_name)
              .collection::<T>(collection_name);
          let item = collection
              .find_one(Some(doc! { "_id": ObjectId::from_str(id).unwrap() }), None)
              .await?
              .unwrap();
          Ok(item)
      }
}

 