pub trait InsertOrInsert<K, V> {
    fn insert_or_insert(&mut self, key: K, value:V) -> bool;
}