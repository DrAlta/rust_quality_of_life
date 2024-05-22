
pub trait AddOrInsert<K, V> {
    fn add_or_insert(&mut self, key: K, value:V);
    
}
