use mongodb::bson::{doc, ser::Error, to_bson, Document};
use serde::Serialize;

use crate::helpers::flatten_document;

pub enum Update<T> {
  Full(T),
  Set(Document),
  FlattenSet(Document),
  Custom(Document),
}

impl<T: Serialize> TryFrom<Update<&T>> for Document {
  type Error = Error;
  fn try_from(update: Update<&T>) -> Result<Self, Self::Error> {
    let update = match update {
      Update::Full(update) => {
        doc! { "$set": to_bson(update)? }
      }
      Update::Set(doc) => doc! { "$set": doc },
      Update::FlattenSet(doc) => doc! { "$set": flatten_document(doc) },
      Update::Custom(doc) => doc,
    };
    Ok(update)
  }
}
