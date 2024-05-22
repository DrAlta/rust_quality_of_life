use std::{borrow::Borrow, collections::BTreeMap};

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RecurrentBTreeMap<T: std::hash::Hash> {
    map: BTreeMap<T, RecurrentBTreeMap<T>>
}

impl<T: std::hash::Hash> RecurrentBTreeMap<T>{
    pub fn new() -> Self {
        Self { map: BTreeMap::new() }
    }
}
impl<T: std::hash::Hash + Eq + Ord> RecurrentBTreeMap<T>{
    pub fn get<Q>(&self, k: &Q) -> Option<&RecurrentBTreeMap<T>>
    where
        Q: ?Sized,
        T: Borrow<Q>,
        Q: std::hash::Hash + Eq + Ord
    {
        self.map.get(k)
    }
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut RecurrentBTreeMap<T>>
    where
        Q: ?Sized,
        T: Borrow<Q>,
        Q: std::hash::Hash + Eq + Ord
    {
        self.map.get_mut(k)

    }
    pub fn insert(&mut self, key: T, value: RecurrentBTreeMap<T>) -> Option<RecurrentBTreeMap<T>> {
        self.map.insert(key, value)
    }
    pub fn merge(&mut self, k: T, v: RecurrentBTreeMap<T>) -> bool {
        let Some(outer) = self.map.get_mut(&k) else {
            self.map.insert(k, v);
            return false
        };

        for item in v {
            outer.merge(item.0, item.1);
        }
        true
    }
    pub fn push(&mut self, k: T) -> bool {
        if self.map.contains_key(&k) {
            return true
        }
        let body = RecurrentBTreeMap::new();
        self.map.insert(k, body);
        return false
    }
}

impl<T: std::hash::Hash> IntoIterator for RecurrentBTreeMap<T>{
    type Item = (T, RecurrentBTreeMap<T>);

    type IntoIter=std::collections::btree_map::IntoIter::<T, RecurrentBTreeMap<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<'a, T: std::hash::Hash> IntoIterator for &'a RecurrentBTreeMap<T>{
    type Item = (&'a T, &'a RecurrentBTreeMap<T>);

    type IntoIter=std::collections::btree_map::Iter::<'a, T, RecurrentBTreeMap<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl<'a, T: std::hash::Hash> IntoIterator for &'a mut RecurrentBTreeMap<T>{
    type Item = (&'a T, &'a mut RecurrentBTreeMap<T>);

    type IntoIter=std::collections::btree_map::IterMut::<'a, T, RecurrentBTreeMap<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter_mut()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merge_test(){
        let mut aa = RecurrentBTreeMap::<i8>::new();
        aa.insert(11, RecurrentBTreeMap::<i8>::new());
 
        let mut a = RecurrentBTreeMap::<i8>::new();
        a.insert(1, aa);

        let mut bb = RecurrentBTreeMap::<i8>::new();
        bb.insert(12, RecurrentBTreeMap::<i8>::new());
 
        let mut b = RecurrentBTreeMap::<i8>::new();
        b.insert(1, bb);

        let mut test = RecurrentBTreeMap::<i8>::new();
        test.merge(0, a);
        test.merge(0, b);


        let mut one = RecurrentBTreeMap::<i8>::new();
        one.insert(11, RecurrentBTreeMap::<i8>::new());
        one.insert(12, RecurrentBTreeMap::<i8>::new());

        let mut zero = RecurrentBTreeMap::<i8>::new();
        zero.insert(1, one);
        
        let mut standared = RecurrentBTreeMap::<i8>::new();
        standared.insert(0, zero);
        
        assert_eq!(
            test,
            standared
        )

    }
}