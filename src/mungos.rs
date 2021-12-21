use std::time::Duration;
use mongodb::{options::ClientOptions, Client};
use crate::Collection;

#[derive(Clone)]
pub struct Mungos {
    pub client: Client,
}

impl Mungos {
    pub async fn new(uri: &str, app_name: &str, timeout: Duration) -> Mungos {
        // Parse your connection string into an options struct
        let mut client_options = ClientOptions::parse(uri).await.unwrap();
        // Manually set an option
        client_options.app_name = Some(app_name.to_string());
        client_options.connect_timeout = Some(timeout);
        // Get a handle to the cluster
        let client = Client::with_options(client_options).unwrap();
        // println!();
        // println!("============================");
        // println!("==== connected to mongo ====");
        // println!("============================");
        // println!();
        Mungos { client }
    }

    pub fn collection<T>(&self, db_name: &str, collection_name: &str) -> Collection<T> {
        Collection {
            collection: self.client.database(db_name).collection(collection_name),
        }
    }
}
