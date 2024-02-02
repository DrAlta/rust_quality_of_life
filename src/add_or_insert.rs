use std::collections::{BTreeMap, HashMap};

pub trait AddOrInsert<K, V> {
    fn add_or_insert(&mut self, key: K, value:V);
    
}

impl<K: std::hash::Hash + Eq + Ord, V: std::ops::AddAssign<V>> AddOrInsert<K, V> for BTreeMap<K, V> {
    fn add_or_insert(&mut self, key:K, mut value: V) {
        match self.remove(&key) {
            Some(entry) => {
                value += entry;
            }
            None => ()
        }
        self.insert(key, value);
    }
}

impl<K: std::hash::Hash + Eq + Ord, V: std::ops::AddAssign<V>> AddOrInsert<K, V> for HashMap<K, V> {
    fn add_or_insert(&mut self, key:K, mut value: V) {
        match self.remove(&key) {
            Some(entry) => {
                value += entry;
            }
            None => ()
        }
        self.insert(key, value);
    }
}