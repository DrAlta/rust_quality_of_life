use std::{collections::HashMap, hash::Hash};

use super::PushOrInsert;

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add(){
        let mut specimen = HashMap::from([("a" ,vec![1])]);
        specimen.push_or_insert("a", 2);
        assert_eq!(
            specimen,
            HashMap::from([("a", vec![1,2])])
        )
    }
    #[test]
    fn insert(){
        let mut specimen = HashMap::new();
        specimen.push_or_insert("a", 3);
        assert_eq!(
            specimen,
            HashMap::from([("a", vec![3])])
        )
    }
}