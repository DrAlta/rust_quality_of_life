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
    fn inner_iter() {
        let a = vec![1,2,3];
        let mut b = Some(&a).inner_iter();
        assert_eq!(b.next().unwrap(), &1);
        assert_eq!(b.next().unwrap(), &2);
        assert_eq!(b.next().unwrap(), &3);
        assert_eq!(b.next(), None);
        }
}