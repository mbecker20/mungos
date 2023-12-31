use std::str::FromStr;

use anyhow::Context;
use futures::TryStreamExt;
use mongodb::{
  bson::{doc, oid::ObjectId, Document},
  options::FindOptions,
  Collection,
};
use serde::de::DeserializeOwned;

pub async fn find_collect<T: DeserializeOwned>(
  coll: &Collection<T>,
  filter: impl Into<Option<Document>>,
  options: impl Into<Option<FindOptions>>,
) -> Result<Vec<T>, mongodb::error::Error> {
  coll.find(filter, options).await?.try_collect().await
}

pub async fn find_one_by_id<T: DeserializeOwned + Unpin + Send + Sync>(
  coll: &Collection<T>,
  id: &str,
) -> anyhow::Result<Option<T>> {
  let filter = doc! { "_id": ObjectId::from_str(id).context("id is not ObjectId")? };
  coll
    .find_one(filter, None)
    .await
    .context("failed at mongo query")
}

