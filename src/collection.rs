use mongodb::{
    bson::doc, error::Result, options::IndexOptions, results::CreateIndexResult, Database,
    IndexModel,
};

#[derive(Clone, Debug)]
pub struct Collection<T> {
    pub db: Database,
    pub collection: mongodb::Collection<T>,
}

impl<T> Collection<T> {
    pub async fn create_index(&self, field: &str) -> Result<CreateIndexResult> {
        let index = IndexModel::builder().keys(doc! { field: 1 }).build();
        self.collection.create_index(index, None).await
    }

    pub async fn create_unique_index(&self, field: &str) -> Result<CreateIndexResult> {
        let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder()
            .keys(doc! { field: 1 })
            .options(options)
            .build();
        self.collection.create_index(index, None).await
    }
}
