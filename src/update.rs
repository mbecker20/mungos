use mongodb::{
  bson::{doc, Document},
  options::UpdateModifications,
};

use crate::helpers::{flatten_document, flatten_document_once};

pub enum Update {
  /// { $set: Document }
  Set(Document),
  /// { $set: flatten_document_once(Document) }
  FlattenOnceSet(Document),
  /// { $set: flatten_document(Document) } (recurses to flatten deeploy nested fields)
  FlattenSet(Document),
  /// Document
  Custom(Document),
}

impl From<Update> for Document {
  fn from(update: Update) -> Self {
    match update {
      Update::Set(doc) => doc! { "$set": doc },
      Update::FlattenOnceSet(doc) => doc! { "$set": flatten_document_once(doc) },
      Update::FlattenSet(doc) => doc! { "$set": flatten_document(doc) },
      Update::Custom(doc) => doc,
    }
  }
}

impl From<Update> for UpdateModifications {
  fn from(update: Update) -> Self {
    let update: Document = update.into();
    update.into()
  }
}
