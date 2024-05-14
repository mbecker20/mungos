use mongodb::bson::Document;

pub struct Projection<'a>(pub &'a str);

impl<'a> From<Projection<'a>> for Document {
  fn from(value: Projection<'a>) -> Document {
    let mut projection = Document::new();
    for field in value.0.split(' ') {
      projection.insert(field, 1);
    }
    projection
  }
}

impl<'a> From<Projection<'a>> for Option<Document> {
  fn from(value: Projection<'a>) -> Self {
    let doc: Document = value.into();
    doc.into()
  }
}
