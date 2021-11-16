use std::time::Duration;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, to_bson, Document},
    error::Result,
    options::{ClientOptions, FindOptions},
    Client,
};
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
}
