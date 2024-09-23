use std::collections::HashMap;

use super::AddOrInsert;

impl<K: std::hash::Hash + Eq, V: std::ops::AddAssign<V>> AddOrInsert<K, V> for HashMap<K, V> {
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add(){
        let mut specimen = HashMap::from([("a" ,1)]);
        specimen.add_or_insert("a", 3);
        assert_eq!(
            specimen,
            HashMap::from([("a", 4)])
        )
    }
    #[test]
    fn insert(){
        let mut specimen = HashMap::new();
        specimen.add_or_insert("a", 2);
        assert_eq!(
            specimen,
            HashMap::from([("a", 2)])
        )
    }
}