mod inner_iter_iterator;
pub use inner_iter_iterator::InnerIterIterator;


pub trait InnerIter<T>{
    fn inner_iter(self)->InnerIterIterator<T>;
}


impl<I, T: IntoIterator<IntoIter=I>> InnerIter<I> for Option<T> {
    fn inner_iter(self)-> InnerIterIterator<I>{
        if self.is_some(){
            InnerIterIterator::new(Some(self.unwrap().into_iter()))
    }   else {
            InnerIterIterator::new(None)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn some_inner_iter() {
        let a: [i32; 4] = [1,2,3,4];
        let b = Some(a.clone());
        let mut iter = b.inner_iter();
        for x in a{
            assert_eq!(Some(x), iter.next());

        }
    }
    #[test]
    fn none_inner_iter() {

        let b = Option::<[i32; 4]>::None;
        let mut iter = b.inner_iter();
            
        assert_eq!(None, iter.next());
    }
}