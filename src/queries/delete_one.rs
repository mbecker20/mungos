use crate::Collection;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Result,
};
use std::str::FromStr;

impl<T> Collection<T> {
    pub async fn delete_one(&self, id: &str) -> Result<String> {
        let filter = doc! { "_id": ObjectId::from_str(id).unwrap() };
        self.collection.delete_one(filter, None).await?;
        Ok(id.to_string())
    }
}
