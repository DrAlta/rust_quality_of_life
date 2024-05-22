use std::{borrow::Borrow, collections::HashMap};

#[derive(Debug, Clone)]
pub struct RecurrentHashMap<T: std::hash::Hash> {
    map: HashMap<T, RecurrentHashMap<T>>
}

impl<T: std::hash::Hash> RecurrentHashMap<T>{
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }
}
impl<T: std::hash::Hash + Eq> RecurrentHashMap<T>{
    pub fn get<Q>(&self, k: &Q) -> Option<&RecurrentHashMap<T>>
    where
        Q: ?Sized,
        T: Borrow<Q>,
        Q: std::hash::Hash + Eq
    {
        self.map.get(k)
    }
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut RecurrentHashMap<T>>
    where
        Q: ?Sized,
        T: Borrow<Q>,
        Q: std::hash::Hash + Eq
    {
        self.map.get_mut(k)

    }
    pub fn insert(&mut self, key: T, value: RecurrentHashMap<T>) -> Option<RecurrentHashMap<T>> {
        self.map.insert(key, value)
    }
    pub fn merge(&mut self, k: T, v: RecurrentHashMap<T>) -> bool {
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
        let body = RecurrentHashMap::new();
        self.map.insert(k, body);
        return false
    }
}

impl<T: std::hash::Hash> IntoIterator for RecurrentHashMap<T>{
    type Item = (T, RecurrentHashMap<T>);

    type IntoIter=std::collections::hash_map::IntoIter::<T, RecurrentHashMap<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<'a, T: std::hash::Hash> IntoIterator for &'a RecurrentHashMap<T>{
    type Item = (&'a T, &'a RecurrentHashMap<T>);

    type IntoIter=std::collections::hash_map::Iter::<'a, T, RecurrentHashMap<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl<'a, T: std::hash::Hash> IntoIterator for &'a mut RecurrentHashMap<T>{
    type Item = (&'a T, &'a mut RecurrentHashMap<T>);

    type IntoIter=std::collections::hash_map::IterMut::<'a, T, RecurrentHashMap<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter_mut()
    }
}


impl<T> PartialEq for RecurrentHashMap<T>
where
    T: Eq + std::hash::Hash,
{
    fn eq(&self, other: &RecurrentHashMap<T>) -> bool {
        if self.map.len() != other.map.len() {
            return false;
        }

        self.map.iter().all(|(key, value)| other.map.get(key).map_or(false, |v| *value == *v))
    }
}

impl<T> Eq for RecurrentHashMap<T>
where
    T: Eq + std::hash::Hash,
{
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merge_test(){
        let mut aa = RecurrentHashMap::<i8>::new();
        aa.insert(11, RecurrentHashMap::<i8>::new());
 
        let mut a = RecurrentHashMap::<i8>::new();
        a.insert(1, aa);

        let mut bb = RecurrentHashMap::<i8>::new();
        bb.insert(12, RecurrentHashMap::<i8>::new());
 
        let mut b = RecurrentHashMap::<i8>::new();
        b.insert(1, bb);

        let mut test = RecurrentHashMap::<i8>::new();
        test.merge(0, a);
        test.merge(0, b);


        let mut one = RecurrentHashMap::<i8>::new();
        one.insert(11, RecurrentHashMap::<i8>::new());
        one.insert(12, RecurrentHashMap::<i8>::new());

        let mut zero = RecurrentHashMap::<i8>::new();
        zero.insert(1, one);
        
        let mut standared = RecurrentHashMap::<i8>::new();
        standared.insert(0, zero);
        
        assert_eq!(
            test,
            standared
        )

    }
}