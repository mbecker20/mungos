use mongodb::{
  bson::{doc, Document},
  options::UpdateModifications,
};

use crate::helpers::flatten_document;

pub enum Update {
  Set(Document),
  FlattenSet(Document),
  Custom(Document),
}

impl From<Update> for Document {
  fn from(update: Update) -> Self {
    match update {
      Update::Set(doc) => doc! { "$set": doc },
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
