use mongodb::bson::Document;

pub enum FindOptions<'a> {
    StringProjection(&'a str),
}

impl<'a> Into<Option<mongodb::options::FindOptions>> for FindOptions<'a> {
    fn into(self) -> Option<mongodb::options::FindOptions> {
        match self {
            FindOptions::StringProjection(string_projection) => {
                let mut projection = Document::new();
                for field in string_projection.split(" ") {
					projection.insert(field, 1);
				}
                mongodb::options::FindOptions::builder()
                    .projection(projection)
                    .build()
                    .into()
            }
        }
    }
}

impl<'a> Into<Option<mongodb::options::FindOneOptions>> for FindOptions<'a> {
    fn into(self) -> Option<mongodb::options::FindOneOptions> {
        match self {
            FindOptions::StringProjection(string_projection) => {
                let mut projection = Document::new();
                for field in string_projection.split(" ") {
					projection.insert(field, 1);
				}
                mongodb::options::FindOneOptions::builder()
                    .projection(projection)
                    .build()
                    .into()
            }
        }
    }
}
