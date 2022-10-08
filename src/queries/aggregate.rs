use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    error::Result,
    options::AggregateOptions,
    Cursor,
};
use serde::de::DeserializeOwned;

use crate::Collection;

impl<T: DeserializeOwned + Unpin + Send + Sync> Collection<T> {
    pub async fn aggregate(
        &self,
        pipeline: impl IntoIterator<Item = impl Into<Document>>,
        options: impl Into<Option<AggregateOptions>>,
    ) -> Result<Cursor<Document>> {
        self.collection
            .aggregate(pipeline.into_iter().map(|d| d.into()), options)
            .await
    }

    pub async fn aggregate_collect(
        &self,
        pipeline: impl IntoIterator<Item = impl Into<Document>>,
        options: impl Into<Option<AggregateOptions>>,
    ) -> Result<Vec<Document>> {
        self.aggregate(pipeline.into_iter().map(|d| d.into()), options)
            .await?
            .try_collect()
            .await
    }
}

pub enum AggStage {
    AddFields(Document),
    Bucket(Document),
    BucketAuto(Document),
    CollStats(Document),
    Count(String),
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
    Out(String),
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
    Unset(String),
    Unwind(Document),
    Doc(Document),
}

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
        },
        Doc(doc) => doc,
    }
}

impl Into<Document> for AggStage {
    fn into(self) -> Document {
        get_agg_stage_as_doc(self)
    }
}
