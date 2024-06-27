use futures::TryStreamExt;
use mongodb::{bson::Document, options::FindOptions, Collection};
use serde::de::DeserializeOwned;

pub async fn find_collect<T: Send + Sync + DeserializeOwned>(
  coll: &Collection<T>,
  filter: impl Into<Option<Document>>,
  options: impl Into<Option<FindOptions>>,
) -> Result<Vec<T>, mongodb::error::Error> {
  coll
    .find(filter.into().unwrap_or_default())
    .with_options(options)
    .await?
    .try_collect()
    .await
}
