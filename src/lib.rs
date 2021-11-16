use mongodb::{Collection as MongoCollection};

pub mod queries;

pub struct Mungos;

pub struct Collection<T> {
    pub collection: MongoCollection<T>,
}

pub use mongodb::{options::FindOptions, bson::doc};

