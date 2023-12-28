use std::time::Duration;

use anyhow::Context;
use mongodb::{
  error::Result,
  options::{ClientOptions, Compressor},
};
use serde::Deserialize;

use crate::helpers::parse_comma_seperated_compressors;

pub async fn with_args(
  uri: &str,
  app_name: impl Into<String>,
  timeout: Duration,
  compressors: impl Into<Option<Vec<Compressor>>>,
) -> Result<mongodb::Client> {
  let mut client_options = ClientOptions::parse(uri).await?;
  client_options.app_name = Some(app_name.into());
  client_options.connect_timeout = Some(timeout);
  client_options.compressors = compressors.into();
  mongodb::Client::with_options(client_options)
}

pub async fn from_env() -> anyhow::Result<mongodb::Client> {
  builder_from_env()?.build().await
}

pub fn builder_from_env() -> anyhow::Result<MongoBuilder> {
  let env: MongoEnv = envy::from_env().context("failed to parse mungos env")?;

  let compressors = env
    .mongo_compressors
    .map(|c| {
      parse_comma_seperated_compressors(&c)
        .context("failed to parse mongo compressors specified in env")
    })
    .transpose()?;

  let builder = MongoBuilder {
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

#[derive(Deserialize)]
struct MongoEnv {
  mongo_uri: Option<String>,
  mongo_username: Option<String>,
  mongo_password: Option<String>,
  mongo_address: Option<String>,
  mongo_app_name: Option<String>,
  mongo_timeout_secs: Option<u64>,
  mongo_compressors: Option<String>,
}

#[derive(Default, Debug)]
pub struct MongoBuilder {
  uri: Option<String>,
  username: Option<String>,
  password: Option<String>,
  address: Option<String>,

  app_name: String,
  timeout: Duration,
  compressors: Option<Vec<Compressor>>,
}

impl MongoBuilder {
  pub async fn build(self) -> anyhow::Result<mongodb::Client> {
    let uri = if let Some(uri) = self.uri {
      uri.to_string()
    } else {
      let address = self
        .address
        .context("must specify either full uri or address. got neither")?;
      if let Some(username) = self.username {
        let password = self
          .password
          .context("specified mongo username, but not password")?;
        format!("mongodb://{username}:{password}@{address}")
      } else {
        format!("mongodb://{address}")
      }
    };

    with_args(&uri, self.app_name, self.timeout, self.compressors)
      .await
      .context("failed to init mongo with given args")
  }

  pub fn uri(mut self, uri: impl Into<String>) -> MongoBuilder {
    self.uri = uri.into().into();
    self
  }

  pub fn username(mut self, username: impl Into<String>) -> MongoBuilder {
    self.username = username.into().into();
    self
  }

  pub fn password(mut self, password: impl Into<String>) -> MongoBuilder {
    self.password = password.into().into();
    self
  }

  pub fn address(mut self, address: impl Into<String>) -> MongoBuilder {
    self.address = address.into().into();
    self
  }

  pub fn app_name(mut self, app_name: impl Into<String>) -> MongoBuilder {
    self.app_name = app_name.into();
    self
  }

  pub fn timeout(mut self, duration: Duration) -> MongoBuilder {
    self.timeout = duration;
    self
  }

  pub fn compressors(mut self, compressors: impl Into<Option<Vec<Compressor>>>) -> MongoBuilder {
    self.compressors = compressors.into();
    self
  }
}
