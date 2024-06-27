use anyhow::Context;
use futures::TryStreamExt;
use mongodb::{
  bson::{doc, from_document, Document},
  error::Error,
  options::AggregateOptions,
  Collection, Cursor,
};

pub async fn aggregate<T: Send + Sync>(
  collection: &Collection<T>,
  pipeline: impl IntoIterator<Item = impl Into<Document>>,
  options: impl Into<Option<AggregateOptions>>,
) -> Result<Cursor<Document>, Error> {
  collection
    .aggregate(pipeline.into_iter().map(|d| d.into()))
    .with_options(options)
    .await
}

pub async fn aggregate_collect<T: Send + Sync>(
  collection: &Collection<T>,
  pipeline: impl IntoIterator<Item = impl Into<Document>>,
  options: impl Into<Option<AggregateOptions>>,
) -> Result<Vec<Document>, Error> {
  aggregate(collection, pipeline, options)
    .await?
    .try_collect()
    .await
}

pub async fn aggregate_collect_parse<T: Send + Sync, D: DeserializeOwned>(
  collection: &Collection<T>,
  pipeline: impl IntoIterator<Item = impl Into<Document>>,
  options: impl Into<Option<AggregateOptions>>,
) -> anyhow::Result<Vec<D>> {
  aggregate_collect(collection, pipeline, options)
    .await?
    .into_iter()
    .map(|d| from_document(d).context("failed to parse aggregation result"))
    .collect()
}

pub enum AggStage {
  AddFields(Document),
  Bucket(Document),
  BucketAuto(Document),
  CollStats(Document),
  Count(&'static str),
  Densify(Document),
  Facet(Document),
  Fill(Document),
  GeoNear(Document),
  GraphLookup(Document),
  Group(Document),
  IndexStats(Document),
  Limit(i64),
  Lookup(Document),
  Match(Document),
  Merge(Document),
  Out(&'static str),
  Project(Document),
  Redact(Document),
  ReplaceWith(Document),
  ReplaceRoot(Document),
  Sample(i64),
  Search(Document),
  SearchMeta(Document),
  Set(Document),
  SetWindowFields(Document),
  Skip(i64),
  Sort(Document),
  SortByCount(Document),
  UnionWith(Document),
  Unset(&'static str),
  Unwind(Document),
  Doc(Document),
}

use serde::de::DeserializeOwned;
use AggStage::*;

fn get_agg_stage_as_doc(stage: AggStage) -> Document {
  match stage {
    AddFields(doc) => {
      doc! {
        "$addFields": doc
      }
    }
    Bucket(doc) => {
      doc! {
        "$bucket": doc
      }
    }
    BucketAuto(doc) => {
      doc! {
        "$bucketAuto": doc
      }
    }
    CollStats(doc) => {
      doc! {
        "$collStats": doc
      }
    }
    Count(count) => {
      doc! {
      "$count": count
      }
    }
    Densify(doc) => {
      doc! {
        "$densify": doc
      }
    }
    Facet(doc) => {
      doc! {
        "$facet": doc
      }
    }
    Fill(doc) => {
      doc! {
        "$fill": doc
      }
    }
    GeoNear(doc) => {
      doc! {
        "$geoNear": doc
      }
    }
    GraphLookup(doc) => {
      doc! {
        "$graphLookup": doc
      }
    }
    Group(doc) => {
      doc! {
        "$group": doc
      }
    }
    IndexStats(doc) => {
      doc! {
        "$indexStats": doc
      }
    }
    Limit(limit) => {
      doc! {
        "$limit": limit
      }
    }
    Lookup(doc) => {
      doc! {
        "$lookup": doc
      }
    }
    Match(doc) => {
      doc! {
        "$match": doc
      }
    }
    Merge(doc) => {
      doc! {
        "$merge": doc
      }
    }
    Out(out) => {
      doc! {
        "$out": out
      }
    }
    Project(doc) => {
      doc! {
        "$project": doc
      }
    }
    Redact(doc) => {
      doc! {
        "$redact": doc
      }
    }
    ReplaceWith(doc) => {
      doc! {
        "$replaceWith": doc
      }
    }
    ReplaceRoot(doc) => {
      doc! {
        "$replaceRoot": doc
      }
    }
    Sample(sample) => {
      doc! {
        "$sample": sample
      }
    }
    Search(doc) => {
      doc! {
        "$search": doc
      }
    }
    SearchMeta(doc) => {
      doc! {
        "$searchMeta": doc
      }
    }
    Set(doc) => {
      doc! {
        "$set": doc
      }
    }
    SetWindowFields(doc) => {
      doc! {
        "$setWindowFields": doc
      }
    }
    Skip(skip) => {
      doc! {
        "$skip": skip
      }
    }
    Sort(doc) => {
      doc! {
        "$sort": doc
      }
    }
    SortByCount(doc) => {
      doc! {
        "$sortByCount": doc
      }
    }
    UnionWith(doc) => {
      doc! {
        "$unionWith": doc
      }
    }
    Unset(unset) => {
      doc! {
        "$unset": unset
      }
    }
    Unwind(doc) => {
      doc! {
        "$unwind": doc
      }
    }
    Doc(doc) => doc,
  }
}

impl From<AggStage> for Document {
  fn from(value: AggStage) -> Document {
    get_agg_stage_as_doc(value)
  }
}
