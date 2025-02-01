use std::collections::{HashMap, BTreeSet};

use super::InsertOrInsert;

impl<K: std::hash::Hash + Eq , V:Ord> InsertOrInsert<K, V> for HashMap<K, BTreeSet<V>> {
    /*fn append_or_insert(&mut self, k: K, v: &mut Vec<V>){
        let Some(thing)= self.get_mut(&k) else {
            let mut thing = Vec::new();
            thing.append(v);
            if let Some(_) = self.insert(k, thing) {
                panic!("We shouldn't be replacing a entry in the BTreeMap.")
            };
            return;
        };
        thing.append(v);
    }*/
    fn insert_or_insert(&mut self, k: K, v: V) -> bool {
        let Some(thing)= self.get_mut(&k) else {
            let thing = BTreeSet::from([v]);
            if let Some(_) = self.insert(k, thing) {
                panic!("We shouldn't be replacing a entry in the HashMap.")
            };
            return true;
        };
        thing.insert(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add(){
        let mut specimen = HashMap::from(
            [
                (
                    "a",
                    BTreeSet::from(
                        [1]
                    )
                )
            ]
        );
        specimen.insert_or_insert("a", 2);
        assert_eq!(
            specimen,
            HashMap::from([("a", BTreeSet::from([1,2]))])
        )
    }
    #[test]
    fn insert(){
        let mut specimen = HashMap::new();
        specimen.insert_or_insert("a", 3);
        assert_eq!(
            specimen,
            HashMap::from([("a", BTreeSet::from([3]))])
        )
    }
}