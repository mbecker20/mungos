use std::borrow::Borrow;

use anyhow::Context;
use mongodb::{
    bson::{doc, Document},
    error::{ErrorKind, Result},
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
            .iter()
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

    pub async fn bulk_update_retry_too_big(
        &self,
        updates: impl Borrow<Vec<BulkUpdate>>,
    ) -> anyhow::Result<Document> {
        let update_docs: Vec<Document> = updates
            .borrow()
            .iter()
            .map(|update| {
                doc! {
                    "q": &update.query,
                    "u": &update.update,
                }
            })
            .collect();
        let command = doc! {
            "update": self.collection.name(),
            "updates": &update_docs,
        };
        let res = self.db.run_command(command, None).await;
        if res.is_ok() {
            return res.context("");
        }
        let error_kind = &*res.as_ref().err().unwrap().kind;
        if let ErrorKind::Command(c) = error_kind {
            // code 10334 has code_name "BSONObjectTooLarge"
            if c.code == 10334 && update_docs.len() > 1 {
                let (u1, u2) = update_docs.split_at(update_docs.len() / 2);
                let c1 = doc! {
                    "update": self.collection.name(),
                    "updates": u1,
                };
                let c2 = doc! {
                    "update": self.collection.name(),
                    "updates": u2,
                };
                let (res1, res2) =
                    tokio::join!(self.db.run_command(c1, None), self.db.run_command(c2, None));
                let doc1 = res1.context("failed again on 1st half of update batch")?;
                let doc2 = res2.context("failed again on 2nd half of update batch")?;
                return Ok(doc! {
                    "res1": doc1,
                    "res2": doc2
                });
            }
        }
        res.context("failed to bulk update")
    }
}
