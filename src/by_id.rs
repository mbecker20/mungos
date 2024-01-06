use std::str::FromStr;

use anyhow::Context;
use mongodb::{
  bson::{doc, oid::ObjectId},
  options::{DeleteOptions, UpdateModifications, UpdateOptions},
  results::{DeleteResult, UpdateResult},
  Collection,
};
use serde::de::DeserializeOwned;

pub async fn find_one_by_id<T: DeserializeOwned + Unpin + Send + Sync>(
  coll: &Collection<T>,
  id: &str,
) -> anyhow::Result<Option<T>> {
  let filter = doc! { "_id": ObjectId::from_str(id).context("id is not ObjectId")? };
  coll
    .find_one(filter, None)
    .await
    .context("failed at mongo find_one")
}

pub async fn update_one_by_id<T>(
  coll: &Collection<T>,
  id: &str,
  update: impl Into<UpdateModifications>,
  options: impl Into<Option<UpdateOptions>>,
) -> anyhow::Result<UpdateResult> {
  coll
    .update_one(
      doc! { "_id": ObjectId::from_str(id).context("id is not valid ObjectId")? },
      update,
      options,
    )
    .await
    .context("failed at mongo update_one")
}

pub async fn delete_one_by_id<T>(
  coll: &Collection<T>,
  id: &str,
  options: impl Into<Option<DeleteOptions>>,
) -> anyhow::Result<DeleteResult> {
  coll
    .delete_one(
      doc! { "_id": ObjectId::from_str(id).context("id is not valid ObjectId")? },
      options,
    )
    .await
    .context("failed at mongo delete_one")
}
