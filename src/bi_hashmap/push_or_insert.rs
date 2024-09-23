use std::collections::HashMap;

use super::BiHashMap;

impl<O: Eq + std::hash::Hash + Clone, I: Eq + std::hash::Hash, V> crate::PushOrInsert<(O, I), V> for BiHashMap<O, I, Vec<V>> {
    fn push_or_insert(&mut self, (o, i): (O, I), v: V) {
        if !self.0.contains_key(&o) {
            self.0.insert(o.clone(), HashMap::new());
        }
        let inner = self
            .0
            .get_mut(&o)
            .expect("we added the outer key if it didn't already exist");

        if inner.contains_key(&i) {
            inner
                .get_mut(&i)
                .expect("we already check is the key was in it")
                .push(v);
        } else {
            inner.insert(i, vec![v]);
        }
    }
    
    fn append_or_insert(&mut self, (o, i): (O, I), v: &mut Vec<V>) {
        if !self.0.contains_key(&o) {
            self.0.insert(o.clone(), HashMap::new());
        }
        let inner = self
            .0
            .get_mut(&o)
            .expect("we added the outer key if it didn't already exist");

        if inner.contains_key(&i) {
            inner
                .get_mut(&i)
                .expect("we already check is the key was in it")
                .append(v);
        } else {
            let mut new = Vec::new();
            new.append(v);
            inner.insert(i, new);
        }
    }
}
