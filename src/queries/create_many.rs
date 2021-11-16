use mongodb::error::Result;
use serde::Serialize;

use crate::Collection;

impl<T: Serialize> Collection<T> {
    pub async fn create_many(&self, items: Vec<T>) -> Result<()> {
        self.collection.insert_many(items, None).await?;
        Ok(())
    }
}
