use std::borrow::Borrow;

use mongodb::{
    bson::{doc, Document},
    error::Result,
};

use crate::Collection;

#[derive(Debug)]
pub struct BulkUpdate {
    pub query: Document,
    pub update: Document,
}

impl BulkUpdate {
    pub fn new(query: Document, update: Document) -> BulkUpdate {
        BulkUpdate { query, update }
    }
}

impl<T> Collection<T> {
    pub async fn bulk_update(&self, updates: impl Borrow<Vec<BulkUpdate>>) -> Result<Document> {
        let update_docs: Vec<Document> = updates
            .borrow()
            .into_iter()
            .map(|update| {
                doc! {
                    "q": &update.query,
                    "u": &update.update,
                }
            })
            .collect();
        let command = doc! {
            "update": self.collection.name(),
            "updates": update_docs,
        };
        self.db.run_command(command, None).await
    }
}
