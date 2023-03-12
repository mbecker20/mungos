mod collection;
pub mod helpers;
mod queries;
pub mod types;
use std::{sync::Arc, time::Duration};

pub use crate::{
    collection::Collection,
    queries::{
        aggregate::AggStage, bulk_update::BulkUpdate, bulk_upsert::BulkUpsert,
        get_sorted_cursor::SortDirection, update_one::Update,
    },
};
use anyhow::{anyhow, Context};
use helpers::parse_comma_seperated_compressors;
pub use mongodb::{
    self,
    bson::{
        self, doc, oid::ObjectId, serde_helpers::deserialize_hex_string_from_object_id, to_bson,
        Document,
    },
    error::{self, Error, Result},
    options::{self, ClientOptions, Compressor, FindOptions},
    IndexModel,
};
pub use serde::{Deserialize, Serialize};

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
        let env: MungosEnv =
            envy::from_env().context("failed to parse env for mungos new_from_env")?;
        let uri = if let Some(uri) = env.mongo_uri {
            uri
        } else {
            let address = env.mongo_address.ok_or(anyhow!(
                "must specify either MONGO_URI or MONGO_ADDRESS in env. got neither"
            ))?;
            if let Some(username) = env.mongo_username {
                let password = env
                    .mongo_password
                    .ok_or(anyhow!("must specify MONGO_PASSWORD in env"))?;
                format!("mongodb://{username}:{password}@{address}")
            } else {
                format!("mongodb://{address}")
            }
        };
        let compressors = env
            .mongo_compressors
            .map(|c| {
                parse_comma_seperated_compressors(&c)
                    .context("failed to parse mongo compressors specified in env")
            })
            .transpose()?;
        let mungos = Mungos::new(
            &uri,
            &env.mongo_app_name.unwrap_or("mungos".to_string()),
            Duration::from_secs(env.mongo_timeout_secs.unwrap_or(3)),
            compressors,
        )
        .await
        .context("failed to initialize connection to mongo")?;
        Ok(mungos)
    }

    pub fn builder<'a>() -> MungosBuilder<'a> {
        MungosBuilder {
            app_name: "mungos",
            timeout: Duration::from_secs(3),
            ..Default::default()
        }
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

#[derive(Default)]
pub struct MungosBuilder<'a> {
    uri: Option<&'a str>,
    username: Option<&'a str>,
    password: Option<&'a str>,
    address: Option<&'a str>,

    app_name: &'a str,
    timeout: Duration,
    compressors: Option<Vec<Compressor>>,
}

impl<'a> MungosBuilder<'a> {
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

        let mungos = Mungos::new(&uri, self.app_name, self.timeout, self.compressors)
            .await
            .context("failed to initialize connection to mongo")?;

        Ok(mungos)
    }

    pub fn uri(mut self, uri: impl Into<Option<&'a str>>) -> MungosBuilder<'a> {
        self.uri = uri.into();
        self
    }

    pub fn username(mut self, username: impl Into<Option<&'a str>>) -> MungosBuilder<'a> {
        self.username = username.into();
        self
    }

    pub fn password(mut self, password: impl Into<Option<&'a str>>) -> MungosBuilder<'a> {
        self.password = password.into();
        self
    }

    pub fn address(mut self, address: impl Into<Option<&'a str>>) -> MungosBuilder<'a> {
        self.address = address.into();
        self
    }

    pub fn app_name(mut self, app_name: &'a str) -> MungosBuilder<'a> {
        self.app_name = app_name;
        self
    }

    pub fn timeout(mut self, duration: Duration) -> MungosBuilder<'a> {
        self.timeout = duration;
        self
    }

    pub fn compressors(
        mut self,
        compressors: impl Into<Option<Vec<Compressor>>>,
    ) -> MungosBuilder<'a> {
        self.compressors = compressors.into();
        self
    }
}
