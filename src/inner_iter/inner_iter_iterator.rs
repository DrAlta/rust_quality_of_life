pub struct InnerIterIterator<T>{
    inner: Option<T>
}


impl<T> InnerIterIterator<T> {
    pub fn new(inner: Option<T>)-> Self{
        Self{inner}
    }
}


impl<I, T: Iterator<Item = I>> Iterator for InnerIterIterator<T> {
    type Item = I;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
    self.inner.as_mut()?.next()
    }
}

