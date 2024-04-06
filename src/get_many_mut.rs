mod btreemap;
mod vec;

pub trait GetManyMut<T, Q, const N: usize>{
    fn get_many_mut(&mut self, keys: [Q; N]) -> Option<[&mut T; N]>;  
}


