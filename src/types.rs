use mongodb::bson::Document;

pub struct Projection<'a>(pub &'a str);

impl<'a> Into<Document> for Projection<'a> {
    fn into(self) -> Document {
        let mut projection = Document::new();
        for field in self.0.split(" ") {
            projection.insert(field, 1);
        }
        projection
    }
}

impl<'a> Into<Option<Document>> for Projection<'a> {
    fn into(self) -> Option<Document> {
        let mut projection = Document::new();
        for field in self.0.split(" ") {
            projection.insert(field, 1);
        }
        projection.into()
    }
}
