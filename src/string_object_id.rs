use std::str::FromStr;

use anyhow::Context;
use mongodb::bson::{oid::ObjectId, DateTime};

pub trait StringObjectId {
    fn id(&self) -> &str;
    fn object_id(&self) -> anyhow::Result<ObjectId> {
        ObjectId::from_str(self.id()).context("failed to convert id to ObjectId")
    }
    fn created_at(&self) -> anyhow::Result<DateTime> {
        let date_time = self.object_id()?.timestamp();
        Ok(date_time)
    }
    fn created_at_millis(&self) -> anyhow::Result<i64> {
        let ts = self.created_at()?.timestamp_millis();
        Ok(ts)
    }
}
