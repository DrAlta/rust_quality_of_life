use std::{collections::HashMap, ops::AddAssign};

use crate::AddOrInsert;

use super::BiHashMap;

impl<O: Eq + std::hash::Hash + Clone, I: Eq + std::hash::Hash, V: AddAssign> AddOrInsert<(O, I), V> for BiHashMap<O, I, V> {
    fn add_or_insert(&mut self, (o, i): (O, I), value:V) {
        if !self.0.contains_key(&o) {
            self.0.insert(o.clone(), HashMap::new());
        }
        let inner = self
            .0
            .get_mut(&o)
            .expect("we added the outer key if it didn't already exist");
        inner.add_or_insert(i, value)
    }
}