use anyhow::Context;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::Result,
    options::FindOneOptions,
};
use serde::de::DeserializeOwned;
use std::str::FromStr;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn find_one(
        &self,
        filter: impl Into<Option<Document>>,
        options: impl Into<Option<FindOneOptions>>,
    ) -> Result<Option<T>> {
        self.collection.find_one(filter, options).await
    }

    pub async fn find_one_by_id(&self, id: &str) -> anyhow::Result<Option<T>> {
        let res = self.collection
            .find_one(Some(doc! { "_id": ObjectId::from_str(id).context("failed to parse id into ObjectId")? }), None)
            .await?;
        Ok(res)
    }
}
