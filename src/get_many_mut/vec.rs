use crate::UnwrapArray;

use super::GetManyMut;

impl<T, const N: usize, > GetManyMut<T, usize, N> for Vec<T>{
    fn get_many_mut(&mut self, keys: [usize; N]) -> Option<[&mut T; N]>{
        let mut ret: [Option<&mut T>; N] = std::array::from_fn(|_| None);
        for (key, entry) in self.iter_mut().enumerate() {
            for (idx, q) in keys.iter().enumerate() {
                if &key == q {
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
    use crate::GetManyMut;
    #[test]
    fn vec_get_many_mut_test() {
        let mut a = vec!['a', 'b', 'c', 'd'];
 
        assert_eq!(
            a.get_many_mut([1,3]),
            Some([&mut 'b', &mut 'd'])
        );
    }
}