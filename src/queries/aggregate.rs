use mongodb::{bson::Document, error::Result, options::AggregateOptions, Cursor};
use serde::de::DeserializeOwned;
use futures::TryStreamExt;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn aggregate(
        &self,
        pipeline: impl IntoIterator<Item = Document>,
        options: impl Into<Option<AggregateOptions>>,
    ) -> Result<Cursor<Document>> {
        self.collection.aggregate(pipeline, options).await
    }

    pub async fn aggregate_collect(
        &self,
        pipeline: impl IntoIterator<Item = Document>,
        options: impl Into<Option<AggregateOptions>>,
    ) -> Result<Vec<Document>> {
        self.aggregate(pipeline, options).await?.try_collect().await
    }
}
