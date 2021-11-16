use mongodb::error::Result;
use serde::Serialize;

use crate::Collection;

impl<T: Serialize> Collection<T> {
    pub async fn create_one(&self, item: T) -> Result<String> {
        let res = self.collection.insert_one(item, None).await?;
        Ok(res.inserted_id.as_object_id().unwrap().to_string())
    }
}
