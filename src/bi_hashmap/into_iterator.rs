use crate::BiHashMapIter;

use super::BiHashMap;

impl<'a, O, I, V> IntoIterator for &'a BiHashMap<O, I, V> where
    O: Eq + std::hash::Hash, 
    I: Eq + std::hash::Hash
{
    type Item = ((&'a O, &'a I), &'a V);

    type IntoIter = BiHashMapIter<'a ,O, I, V>;

    fn into_iter(self) -> Self::IntoIter {
        BiHashMapIter::new(self.0.iter())
        
    }
}
