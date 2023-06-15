use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::{anyhow, Context};
use mongodb::{
    bson::Document,
    error::Result,
    options::{ClientOptions, Compressor},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod helpers;
pub mod types;

mod collection;
mod queries;

pub use crate::{
    collection::Collection,
    helpers::parse_comma_seperated_compressors,
    queries::{
        aggregate::AggStage, bulk_update::BulkUpdate, bulk_upsert::BulkUpsert,
        get_sorted_cursor::SortDirection, update_one::Update,
    },
    types::Projection,
};

// pub use mungos_derive::{MungosIndexed};

pub use mongodb;

#[derive(Clone, Debug)]
pub struct Mungos {
    pub client: mongodb::Client,
}

#[derive(Deserialize)]
struct MungosEnv {
    mongo_uri: Option<String>,
    mongo_username: Option<String>,
    mongo_password: Option<String>,
    mongo_address: Option<String>,
    mongo_app_name: Option<String>,
    mongo_timeout_secs: Option<u64>,
    mongo_compressors: Option<String>,
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
        let client = mongodb::Client::with_options(client_options)?;
        let mungos = Mungos { client };
        Ok(mungos)
    }

    pub async fn new_from_env() -> anyhow::Result<Mungos> {
        Mungos::builder_from_env()?.build().await
    }

    pub fn builder() -> MungosBuilder {
        MungosBuilder {
            app_name: "mungos".to_string(),
            timeout: Duration::from_secs(3),
            ..Default::default()
        }
    }

    pub fn builder_from_env() -> anyhow::Result<MungosBuilder> {
        let env: MungosEnv = envy::from_env().context("failed to parse mungos env")?;

        let compressors = env
            .mongo_compressors
            .map(|c| {
                parse_comma_seperated_compressors(&c)
                    .context("failed to parse mongo compressors specified in env")
            })
            .transpose()?;

        let builder = MungosBuilder {
            uri: env.mongo_uri,
            username: env.mongo_username,
            password: env.mongo_password,
            address: env.mongo_address,
            app_name: env.mongo_app_name.unwrap_or("mungos".to_string()),
            timeout: Duration::from_secs(env.mongo_timeout_secs.unwrap_or(3)),
            compressors,
        };

        Ok(builder)
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

#[derive(Default, Debug)]
pub struct MungosBuilder {
    uri: Option<String>,
    username: Option<String>,
    password: Option<String>,
    address: Option<String>,

    app_name: String,
    timeout: Duration,
    compressors: Option<Vec<Compressor>>,
}

impl MungosBuilder {
    pub async fn build(self) -> anyhow::Result<Mungos> {
        let uri = if let Some(uri) = self.uri {
            uri.to_string()
        } else {
            let address = self.address.ok_or(anyhow!(
                "must specify either full uri or address. got neither"
            ))?;
            if let Some(username) = self.username {
                let password = self
                    .password
                    .ok_or(anyhow!("specified mongo username, but not password"))?;
                format!("mongodb://{username}:{password}@{address}")
            } else {
                format!("mongodb://{address}")
            }
        };

        let mungos = Mungos::new(&uri, &self.app_name, self.timeout, self.compressors)
            .await
            .context("failed to initialize connection to mongo")?;

        Ok(mungos)
    }

    pub fn uri(mut self, uri: impl Into<String>) -> MungosBuilder {
        self.uri = uri.into().into();
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> MungosBuilder {
        self.username = username.into().into();
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> MungosBuilder {
        self.password = password.into().into();
        self
    }

    pub fn address(mut self, address: impl Into<String>) -> MungosBuilder {
        self.address = address.into().into();
        self
    }

    pub fn app_name(mut self, app_name: impl Into<String>) -> MungosBuilder {
        self.app_name = app_name.into();
        self
    }

    pub fn timeout(mut self, duration: Duration) -> MungosBuilder {
        self.timeout = duration;
        self
    }

    pub fn compressors(mut self, compressors: impl Into<Option<Vec<Compressor>>>) -> MungosBuilder {
        self.compressors = compressors.into();
        self
    }
}

#[async_trait::async_trait]
pub trait Indexed: Serialize + DeserializeOwned + Sync {
    fn name() -> &'static str {
        ""
    }
    fn indexes() -> Vec<String> {
        Vec::new()
    }
    fn unique_indexes() -> Vec<String> {
        Vec::new()
    }
    fn doc_indexes() -> Vec<Document> {
        Vec::new()
    }
    fn unique_doc_indexes() -> Vec<Document> {
        Vec::new()
    }
    async fn collection(
        mungos: &Mungos,
        db_name: &str,
        create_index: bool,
    ) -> anyhow::Result<Collection<Self>> {
        let coll = mungos.collection(db_name, Self::name());

        if create_index {
            for index in Self::indexes() {
                coll.create_index(&index).await?;
            }
            for unique_index in Self::unique_indexes() {
                coll.create_unique_index(&unique_index).await?;
            }
            for doc_index in Self::doc_indexes() {
                coll.create_index_from_doc(doc_index).await?;
            }
            for unique_doc_index in Self::unique_doc_indexes() {
                coll.create_unique_index_from_doc(unique_doc_index).await?;
            }
        }

        Ok(coll)
    }
}

macro_rules! impl_indexed_basic {
    ($($ty:ty),*) => {
        $(impl Indexed for $ty {})*
        $(impl Indexed for Option<$ty> {})*
    };
}

macro_rules! impl_indexed_nested {
    ($($ty:ty),*) => {
        $(impl<T: Serialize + DeserializeOwned + Sync> Indexed for $ty {})*
        $(impl<T: Serialize + DeserializeOwned + Sync> Indexed for Option<$ty> {})*
    };
}

impl_indexed_basic!(
    String, PathBuf, bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);
impl_indexed_nested!(Vec<T>);
