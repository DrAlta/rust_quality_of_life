use std::collections::BTreeMap;

use crate::UnwrapArray;

use super::GetManyMut;

impl<T, Q: std::hash::Hash + Ord, const N: usize, > GetManyMut<T, Q, N> for BTreeMap<Q, T>{
    fn get_many_mut(&mut self, keys: [Q; N]) -> Option<[&mut T; N]>{
        let mut ret: [Option<&mut T>; N] = std::array::from_fn(|_| None);
        for (key, entry) in self{
            for (idx, q) in keys.iter().enumerate() {
                if key == q {
                    std::mem::swap(
                        ret.get_mut(idx).expect("we should be indexing out of range as we re itering over an array of the same size"), 
                        &mut Some(entry)
                    );
                    break;
                }
            }
        }
        ret.unwrap()
    }

    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btreemap_get_many_mut_test() {
        let mut a = BTreeMap::new();
        a.insert(1, 'a');
        a.insert(2, 'b');
        a.insert(4, 'c');

        assert_eq!(
            a.get_many_mut([1,4]),
            Some([&mut 'a', &mut 'c'])
        );
    }
}