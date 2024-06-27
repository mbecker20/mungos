use mongodb::bson::{Bson, Document};

/// Flattens a nested bson documents ONE LEVEL using the mongo '.' syntax. Useful for partial updates.
/// doc! { "f1": "yes", "f2": { "f3": "no", "f4": { "f5": "yes", "f6": "no" } } } -> doc! { "f1": "yes", "f2.f3": "no", "f2.f4": { "f5": "yes", "f6": "no" } }
pub fn flatten_document_once(doc: Document) -> Document {
  let mut target = Document::new();

  for (field, bson) in doc {
    if let Bson::Document(doc) = bson {
      for (inner_field, bson) in doc {
        target.insert(format!("{field}.{inner_field}"), bson);
      }
    } else {
      target.insert(field, bson);
    }
  }

  target
}

/// Recursively flattens a nested bson document using the mongo '.' syntax. Useful for partial updates.
/// doc! { "f1": "yes", "f2": { "f3": "no" } } -> doc! { "f1": "yes", "f2.f3": "no" }
pub fn flatten_document(doc: Document) -> Document {
  let mut target = Document::new();
  flatten_document_rec(&mut target, None, doc);
  target
}

fn flatten_document_rec(target: &mut Document, parent_field: Option<String>, doc: Document) {
  let pf = match parent_field {
    Some(parent_field) => format!("{parent_field}."),
    None => String::new(),
  };
  for (field, bson) in doc {
    let f = format!("{pf}{field}");
    if let Bson::Document(doc) = bson {
      flatten_document_rec(target, Some(f), doc)
    } else {
      target.insert(f, bson);
    }
  }
}
