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

    pub async fn get_full_collection<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        db_name: &str,
        collection_name: &str,
    ) -> Result<Vec<T>> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        let mut cursor = collection.find(None, None).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        Ok(items)
    }

    pub async fn get_many_by_id<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        db_name: &str,
        collection_name: &str,
        ids: &Vec<String>,
    ) -> Result<Vec<T>> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        let ids: Vec<ObjectId> = ids
            .iter()
            .map(|id| ObjectId::from_str(id).unwrap())
            .collect();
        let mut cursor = collection
            .find(
                doc! {
                    "_id": {
                        "$in": ids
                    }
                },
                None,
            )
            .await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        Ok(items)
    }

    pub async fn get_most_recent<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        db_name: &str,
        collection_name: &str,
        num_items: i64,
    ) -> Result<Vec<T>> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        let find_options = FindOptions::builder()
            .sort(doc! { "_id": -1 })
            .limit(num_items)
            .build();
        let mut cursor = collection.find(doc! {}, find_options).await?;
        let mut items = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            items.push(item);
        }
        items.reverse();
        Ok(items)
    }

    pub async fn get_one<T: DeserializeOwned + Unpin + Send + Sync>(
        &self,
        db_name: &str,
        collection_name: &str,
        id: &str,
    ) -> Result<T> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        let item = collection
            .find_one(Some(doc! { "_id": ObjectId::from_str(id).unwrap() }), None)
            .await?
            .unwrap();
        Ok(item)
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

    pub async fn update_one<T: Serialize>(
        &self,
        db_name: &str,
        collection_name: &str,
        id: &str,
        item: T,
    ) -> Result<T> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        let filter = doc! { "_id": ObjectId::from_str(id).unwrap() };
        let update = doc! { "$set": to_bson(&item)? };
        collection.update_one(filter, update, None).await?;
        Ok(item)
    }

    pub async fn update_many<T: Serialize>(
        &self,
        db_name: &str,
        collection_name: &str,
        query: Document,
        update: Document,
    ) -> Result<()> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        collection.update_many(query, update, None).await?;
        Ok(())
    }

    pub async fn delete_one<T>(
        &self,
        db_name: &str,
        collection_name: &str,
        id: &str,
    ) -> Result<String> {
        let collection = self
            .client
            .database(db_name)
            .collection::<T>(collection_name);
        let filter = doc! { "_id": ObjectId::from_str(id).unwrap() };
        collection.delete_one(filter, None).await?;
        Ok(id.to_string())
    }
}
