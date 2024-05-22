use std::hash::Hash;

pub trait PushOrInsert<K: Hash + Eq, V>{
    fn append_or_insert(&mut self, k: K, v: &mut Vec<V>);
    fn push_or_insert(&mut self, k: K, v: V);
}