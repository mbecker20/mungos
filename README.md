# mungos

inspired by the ```mongoose``` npm package, this crate contains the ```Mungos``` struct, a wrapper around the [mongodb](https://crates.io/crates/mongodb) client containing some additional queries and functionality

# usage

```rust
#[derive(Serialize, Deserialize, Debug)]
struct TestDoc {
	timestamp: i64,
	name: String,
	#[serde(default)]
	description: String,
}

let mungos = mungos::Mungos::new(uri, app_name, Duration::from_secs(3), None).await.unwrap();
let coll = mungos.collection::<TestDoc>("test_db", "test_coll");

let items: Vec<TestDoc> = coll
	.get_most_recent(
		"timestamp", 
		10, 
		0, 
		None, 
		mungos::Projection("timestamp name")
	)
	.await
	.unwrap();

println!("{items:#?}"); // prints the 10 most recent docs by timestamp
```

## initializing from environment

```
# specify full uri directly
MONGO_URI=mongodb://username:password@localhost:27017

## or

# specify uri parts
MONGO_ADDRESS=localhost:27017
MONGO_USERNAME=username
MONGO_PASSWORD=password

# ---------------------

# specify other options
MONGO_APP_NAME=tester # optional. default is 'mungos'
MONGO_TIMEOUT_SECS=30 # optional. default is '3'
MONGO_COMPRESSORS=snappy,zstd(10),zlib(8) # optional. defaults to None
```

```rust
let mungos = mungos::Mungos::new_from_env().await.unwrap();
let coll = mungos.collection::<TestDoc>("test_db", "test_coll");

let items = coll.get_some(None, None).await.unwrap(); // Vec<TestDoc>
```