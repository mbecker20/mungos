use futures::TryStreamExt;
use mongodb::{bson::Document, options::FindOptions, Collection};
use serde::de::DeserializeOwned;

pub async fn find_collect<T: DeserializeOwned>(
  coll: &Collection<T>,
  filter: impl Into<Option<Document>>,
  options: impl Into<Option<FindOptions>>,
) -> Result<Vec<T>, mongodb::error::Error> {
  coll.find(filter, options).await?.try_collect().await
}
