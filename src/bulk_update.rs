use anyhow::Context;
use futures::future::join;
use mongodb::{
  bson::{self, doc, Document},
  error::ErrorKind,
  Database,
};
use serde::Serialize;

use crate::update::Update;

pub struct BulkUpdate {
  pub query: Document,
  pub update: Document,
}

impl BulkUpdate {
  pub fn new(query: Document, update: Document) -> BulkUpdate {
    BulkUpdate { query, update }
  }

  pub fn new_from_update<T: Serialize>(
    query: Document,
    update: Update<&T>,
  ) -> Result<BulkUpdate, bson::ser::Error> {
    Ok(BulkUpdate {
      query,
      update: update.try_into()?,
    })
  }
}

pub async fn bulk_update(
  db: &Database,
  collection_name: &str,
  updates: &[BulkUpdate],
  upsert: bool,
) -> Result<Document, mongodb::error::Error> {
  let updates: Vec<_> = updates
    .iter()
    .map(|update| {
      doc! {
        "q": &update.query,
        "u": &update.update,
        "upsert": upsert
      }
    })
    .collect();
  let command = doc! {
    "update": collection_name,
    "updates": updates
  };
  db.run_command(command, None).await
}

pub async fn bulk_update_retry_too_big(
  db: &Database,
  collection_name: &str,
  updates: &[BulkUpdate],
  upsert: bool,
) -> anyhow::Result<Document> {
  let updates: Vec<_> = updates
    .iter()
    .map(|update| {
      doc! {
        "q": &update.query,
        "u": &update.update,
        "upsert": upsert
      }
    })
    .collect();
  let command = doc! {
    "update": collection_name,
    "updates": &updates
  };
  let res = db.run_command(command, None).await;
  let error_kind = match &res {
    Ok(_) => return res.context(""),
    Err(e) => &*e.kind,
  };
  if let ErrorKind::Command(c) = error_kind {
    // code 10_334 has code_name "BSONObjectTooLarge"
    if c.code == 10_334 && updates.len() > 1 {
      let (u1, u2) = updates.split_at(updates.len() / 2);
      let c1 = doc! {
        "update": collection_name,
        "updates": u1,
      };
      let c2 = doc! {
        "update": collection_name,
        "updates": u2,
      };
      let (res1, res2) = join(db.run_command(c1, None), db.run_command(c2, None)).await;
      let doc1 = res1.context("failed again on 1st half of update batch")?;
      let doc2 = res2.context("failed again on 2nd half of update batch")?;
      return Ok(doc! {
        "res1": doc1,
        "res2": doc2,
      });
    }
  }
  res.context("failed to bulk update (not too big)")
}
