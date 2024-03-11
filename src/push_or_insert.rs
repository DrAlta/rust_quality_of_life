use std::{collections::HashMap, hash::Hash};

pub trait PushOrInsert<K: Hash + Eq, V>{
    fn append_or_insert(&mut self, k: K, v: &mut Vec<V>);
    fn push_or_insert(&mut self, k: K, v: V);
}
impl<K: Hash + Eq, V> PushOrInsert<K, V> for HashMap<K, Vec<V>> {
    fn append_or_insert(&mut self, k: K, v: &mut Vec<V>){
        let Some(thing)= self.get_mut(&k) else {
            let mut thing = Vec::new();
            thing.append(v);
            if let Some(_) = self.insert(k, thing) {
                panic!("We shouldn't be replacing a entry in the HashMap.")
            };
            return;
        };
        thing.append(v);
    }
    fn push_or_insert(&mut self, k: K, v: V){
        let Some(thing)= self.get_mut(&k) else {
            let thing = vec![v];
            if let Some(_) = self.insert(k, thing) {
                panic!("We shouldn't be replacing a entry in the HashMap.")
            };
            return;
        };
        thing.push(v);
    }
}