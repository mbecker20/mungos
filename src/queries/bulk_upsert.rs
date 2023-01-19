use std::borrow::Borrow;

use mongodb::{
    bson::{doc, Document},
    error::Result,
};

use crate::Collection;

#[derive(Debug)]
pub struct BulkUpsert {
    pub query: Document,
    pub update: Document,
}

impl BulkUpsert {
    pub fn new(query: Document, update: Document) -> BulkUpsert {
        BulkUpsert { query, update }
    }
}

impl<T> Collection<T> {
    pub async fn bulk_upsert(&self, updates: impl Borrow<Vec<BulkUpsert>>) -> Result<Document> {
        let update_docs: Vec<Document> = updates
            .borrow()
            .into_iter()
            .map(|update| {
                doc! {
                    "q": &update.query,
                    "u": &update.update,
                    "upsert": true
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
