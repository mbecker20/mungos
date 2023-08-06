use std::sync::Arc;

use crate::{BulkUpsert, Collection};
use anyhow::{anyhow, Context};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{Bson, Document},
    error,
    options::{Compressor, FindOptions},
    Cursor,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Default)]
pub struct MigrateCollectionParams {
    pub batch_size: usize,
}

pub async fn migrate_collection<
    S: DeserializeOwned + Unpin + Send + Sync,
    T: Serialize + Unpin + Send + Sync + 'static,
>(
    source: &Collection<S>,
    target: Arc<Collection<T>>,
    map: fn(S) -> anyhow::Result<BulkUpsert>,
    MigrateCollectionParams { batch_size }: MigrateCollectionParams,
) -> anyhow::Result<()> {
    let mut cursor = source
        .collection
        .find(
            None,
            FindOptions::builder().batch_size(batch_size as u32).build(),
        )
        .await?;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Vec<BulkUpsert>>();

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();

    let handle = tokio::spawn(async move {
        loop {
            if cancel_clone.is_cancelled() {
                break;
            }
            if let Some(batch) = rx.recv().await {
                let res = target
                    .bulk_upsert(batch)
                    .await
                    .context("failed at bulk upsert");
                if let Err(e) = res {
                    cancel_clone.cancel();
                    return Err(e);
                }
            }
        }
        anyhow::Ok(())
    });

    loop {
        if cancel.is_cancelled() {
            // the upsert thread has error upserting
            return handle.await.context("context")?.context("context");
        }
        let batch = batch_load_cursor_map(&mut cursor, batch_size, map)
            .await?
            .into_iter()
            .collect::<anyhow::Result<Vec<_>>>()?;
        if batch.is_empty() {
            cancel.cancel();
            break;
        }
        tx.send(batch)?;
    }

    // wait for upsert thread to finish before function finishes
    handle.await??;

    Ok(())
}

pub async fn batch_load_cursor<T: DeserializeOwned + Unpin + Send + Sync>(
    cursor: &mut Cursor<T>,
    batch_size: usize,
) -> error::Result<Vec<T>> {
    let mut res = Vec::with_capacity(batch_size);
    loop {
        let doc = cursor.try_next().await?;
        if doc.is_none() {
            break;
        }
        res.push(doc.unwrap());
        if res.len() >= batch_size {
            break;
        }
    }
    Ok(res)
}

pub async fn batch_load_cursor_map<S: DeserializeOwned + Unpin + Send + Sync, T>(
    cursor: &mut Cursor<S>,
    batch_size: usize,
    map: impl Fn(S) -> anyhow::Result<T>,
) -> anyhow::Result<Vec<anyhow::Result<T>>> {
    let mut res = Vec::with_capacity(batch_size);
    loop {
        let doc = cursor.try_next().await?;
        if doc.is_none() {
            break;
        }
        res.push(map(doc.unwrap()));
        if res.len() >= batch_size {
            break;
        }
    }
    Ok(res)
}

pub fn parse_comma_seperated_compressors(compressors: &str) -> anyhow::Result<Vec<Compressor>> {
    compressors.split(',').map(parse_compressor).collect()
}

fn parse_compressor(compressor: &str) -> anyhow::Result<Compressor> {
    if compressor.contains("snappy") {
        Ok(Compressor::Snappy)
    } else if compressor.contains("zstd") {
        let level = compressor
            .split('(')
            .collect::<Vec<_>>()
            .get(1)
            .map(|l| l.replace(')', ""))
            .map(|l| {
                let l = l
                    .parse::<i32>()
                    .context("zstd compression level must be i32")?;
                if !(1..=22).contains(&l) {
                    Err(anyhow!(
                        "ztd compression level must be between 1 and 22. got {l}"
                    ))
                } else {
                    Ok(l)
                }
            })
            .transpose()?;
        Ok(Compressor::Zstd { level })
    } else if compressor.contains("zlib") {
        let level = compressor
            .split('(')
            .collect::<Vec<_>>()
            .get(1)
            .map(|l| l.replace(')', ""))
            .map(|l| {
                let l = l
                    .parse::<i32>()
                    .context("zlib compression level must be i32")?;
                if !(0..=22).contains(&l) {
                    Err(anyhow!(
                        "ztd compression level must be between 0 and 9. got {l}"
                    ))
                } else {
                    Ok(l)
                }
            })
            .transpose()?;
        Ok(Compressor::Zlib { level })
    } else {
        Err(anyhow!("unrecognized compressor: {compressor}"))
    }
}

pub fn flatten_document(doc: Document) -> Document {
    let mut target = Document::new();
    flatten_document_rec(&mut target, None, doc);
    target
}

fn flatten_document_rec(target: &mut Document, parent_field: Option<String>, doc: Document) {
    let pf = match parent_field {
        Some(parent_field) => format!("{parent_field}."),
        None => String::new(),
    };
    for (field, bson) in doc {
        let f = format!("{pf}{field}");
        if let Bson::Document(doc) = bson {
            flatten_document_rec(target, Some(f), doc)
        } else {
            target.insert(f, bson);
        }
    }
}
