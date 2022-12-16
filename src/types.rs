use mongodb::bson::Document;

pub struct Projection<'a>(pub &'a str);

impl<'a> Into<Option<mongodb::options::FindOptions>> for Projection<'a> {
    fn into(self) -> Option<mongodb::options::FindOptions> {
        let mut projection = Document::new();
        for field in self.0.split(" ") {
            projection.insert(field, 1);
        }
        mongodb::options::FindOptions::builder()
            .projection(projection)
            .build()
            .into()
    }
}

impl<'a> Into<Option<mongodb::options::FindOneOptions>> for Projection<'a> {
    fn into(self) -> Option<mongodb::options::FindOneOptions> {
        let mut projection = Document::new();
        for field in self.0.split(" ") {
            projection.insert(field, 1);
        }
        mongodb::options::FindOneOptions::builder()
            .projection(projection)
            .build()
            .into()
    }
}
