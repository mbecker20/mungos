use mongodb::Collection as MongoCollection;

pub struct Collection<T> {
    pub collection: MongoCollection<T>,
}