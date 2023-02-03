use crate::Collection;
use mongodb::{
    error::Result,
    options::{ClientOptions, Compressor},
    Client,
};
use std::{sync::Arc, time::Duration};

#[derive(Clone, Debug)]
pub struct Mungos {
    pub client: Client,
}

impl Mungos {
    pub async fn new(
        uri: &str,
        app_name: &str,
        timeout: Duration,
        compressors: impl Into<Option<Vec<Compressor>>>,
    ) -> Result<Mungos> {
        let mut client_options = ClientOptions::parse(uri).await?;
        client_options.app_name = Some(app_name.to_string());
        client_options.connect_timeout = Some(timeout);
        client_options.compressors = compressors.into();
        let client = Client::with_options(client_options)?;
        let mungos = Mungos { client };
        Ok(mungos)
    }

    pub fn collection<T>(&self, db_name: &str, collection_name: &str) -> Collection<T> {
        let db = self.client.database(db_name);
        let collection = db.collection(collection_name);
        Collection { db, collection }
    }

    pub fn arc_collection<T>(&self, db_name: &str, collection_name: &str) -> Arc<Collection<T>> {
        let db = self.client.database(db_name);
        let collection = db.collection(collection_name);
        Arc::new(Collection { db, collection })
    }

    pub async fn list_collections(&self, db_name: &str) -> Result<Vec<String>> {
        self.client
            .database(db_name)
            .list_collection_names(None)
            .await
    }
}
