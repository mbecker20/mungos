#[macro_export]
macro_rules! move_to_new_collection {
	(
		$source_mungos:expr, 
		$source_db:expr, 
		$source_col:expr,
		$target_mungos:expr,
		$target_db:expr, 
		$target_col:expr,
		$type_name:ty
	) => {
		{
			let data = $source_mungos.collection::<$type_name>($source_db, $source_col)
				.get_full_collection().await.unwrap();

			$target_mungos.collection($target_db, $target_col)
				.create_many(data).await.unwrap();
		}
	};
}