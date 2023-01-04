use crate::Collection;
use futures::stream::TryStreamExt;
use mongodb::{error, Cursor};
use serde::{de::DeserializeOwned, Serialize};

pub async fn move_to_new_collection<T: Serialize + DeserializeOwned + Unpin + Send + Sync>(
    source_collection: Collection<T>,
    target_collection: Collection<T>,
) -> error::Result<()> {
    let items = source_collection.get_some(None, None).await?;
    target_collection.create_many(items).await?;
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
