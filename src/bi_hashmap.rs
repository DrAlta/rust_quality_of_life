use std::collections::HashMap;

use crate::BiHashMapIter;

pub mod add_or_insert;
pub mod from_iterator;
pub mod into_iterator;
pub mod push_or_insert;

#[derive(Debug, PartialEq)]
pub struct BiHashMap<O, I, V>(HashMap<O, HashMap<I, V>>) where
O: Eq + std::hash::Hash, 
I: Eq + std::hash::Hash;


impl<O, I, V> BiHashMap<O, I, V> where
    O: Eq + std::hash::Hash, 
    I: Eq + std::hash::Hash
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn from_inner(inner:HashMap<O, HashMap<I, V>>)->Self{
        Self(inner)
    }
    pub fn get_inner(&self) -> &HashMap<O, HashMap<I, V>> {
        &self.0
    }
    pub fn iter<'a>(&'a self) ->  BiHashMapIter<'a ,O, I, V> {
        self.into_iter()
    }
}


impl<O, I, V> BiHashMap<O, I, V> where
O: Eq + std::hash::Hash + Clone, 
I: Eq + std::hash::Hash
 {
    pub fn get(&self, (o, i): (&O, &I)) -> Option<&V> {
        self.0.get(o)?.get(i)
    }
    pub fn get_mut(&mut self, (o, i): (&O, &I)) -> Option<&mut V> {
        self.0.get_mut(o)?.get_mut(i)
    }
    pub fn insert(&mut self, (o, i): (O, I), v: V) -> Option<V> {
        if !self.0.contains_key(&o) {
            self.0.insert(o.clone(), HashMap::new());
        }
        let inner = self
            .0
            .get_mut(&o)
            .expect("we added the outer key if it didn't already exist");
        inner.insert(i, v)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_test() {
        let outer: HashMap<u8, HashMap<i8, i16>> = HashMap::new();
        let standard = BiHashMap::<u8, i8, i16>(outer);
        assert_eq!(
            BiHashMap::<u8, i8, i16>::new(),
            standard
        );
    }
    #[test]
    fn insert_test() {
        let inner: HashMap<i8, i16> = HashMap::from([(2,3)]);
        let outer: HashMap<u8, HashMap<i8, i16>> = HashMap::from([(1, inner)]);
        let standard = BiHashMap::<u8, i8, i16>(outer);
        let mut sample = BiHashMap::<u8, i8, i16>::new();
        sample.insert((1_u8,2_i8), 3_i16);
        assert_eq!(
            sample,
            standard
        )
    }
    #[test]
    fn from_iter_test() {
        let inner: HashMap<i8, i16> = HashMap::from([(2,3)]);
        let outer: HashMap<u8, HashMap<i8, i16>> = HashMap::from([(1, inner)]);
        let standard = BiHashMap::<u8, i8, i16>(outer);
        let mut sample = BiHashMap::<u8, i8, i16>::from_iter([((&1_u8,&2_i8), 3_i16)].into_iter());
        sample.insert((1_u8,2_i8), 3_i16);
        assert_eq!(
            sample,
            standard
        )
    }
    #[test]
    fn get_test() {
        let inner: HashMap<i8, i16> = HashMap::from([(2,3)]);
        let outer: HashMap<u8, HashMap<i8, i16>> = HashMap::from([(1, inner)]);
        let sample = BiHashMap::<u8, i8, i16>(outer);
        assert_eq!(
            sample.get((&1_u8, &2_i8)),
            Some(&3_i16)
        )
    }
    #[test]
    fn get_mut_test() {
        let inner: HashMap<i8, i16> = HashMap::from([(2,3)]);
        let outer: HashMap<u8, HashMap<i8, i16>> = HashMap::from([(1, inner)]);
        let mut sample = BiHashMap::<u8, i8, i16>(outer);
        assert_eq!(
            sample.get_mut((&1_u8, &2_i8)),
            Some(&mut 3_i16)
        )
    }
}

