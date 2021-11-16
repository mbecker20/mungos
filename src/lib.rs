use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, to_bson, Document},
    error::Result,
    options::{ClientOptions, FindOptions},
    Client,
};
use serde::{de::DeserializeOwned, Serialize};
use std::str::FromStr;
use std::{marker::Send, time::Duration};

pub mod queries;

pub struct Database {
    pub client: Client,
}
impl Database {
    pub async fn new(uri: &str, app_name: &str, timeout: Duration) -> Database {
        // Parse your connection string into an options struct
        let mut client_options = ClientOptions::parse(uri).await.unwrap();
        // Manually set an option
        client_options.app_name = Some(app_name.to_string());
        client_options.connect_timeout = Some(timeout);
        // Get a handle to the cluster
        let client = Client::with_options(client_options).unwrap();
        println!();
        println!("============================");
        println!("==== connected to mongo ====");
        println!("============================");
        println!();
        Database { client }
    }

    pub async fn create_one<T: Serialize>(
        &self,
        db_name: &str,
        collection_name: &str,
        item: T,
    ) -> Result<String> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        let res = collection.insert_one(item, None).await?;
        Ok(res.inserted_id.as_object_id().unwrap().to_string())
    }

    
}
