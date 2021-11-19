use crate::Collection;
use mongodb::error::Result;
use serde::{de::DeserializeOwned, Serialize};

pub async fn move_to_new_collection<T: Serialize + DeserializeOwned + Unpin + Send + Sync>(
	source_collection: Collection<T>,
	target_collection: Collection<T>,
) -> Result<()> {
	let items = source_collection.get_full_collection().await?;
	target_collection.create_many(items).await?;
	Ok(())
}