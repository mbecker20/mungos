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
    pub async fn bulk_upsert(&self, updates: Vec<BulkUpsert>) -> Result<Document> {
        let mut update_docs: Vec<Document> = Vec::with_capacity(updates.len());
        for update in updates {
            let document = doc! {
                "q": update.query,
                "u": update.update,
                "upsert": true
            };
            update_docs.push(document);
        }
        let command = doc! {
            "update": self.collection.name(),
            "updates": update_docs,
        };
        self.db.run_command(command, None).await
    }
}
