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
