pub trait UnwrapArray<T, const N: usize>{
    fn unwrap(self) -> Option<[T; N]>;
}

impl<T, const N: usize> UnwrapArray<T, N> for [Option<T>; N]{
    fn unwrap(mut self) -> Option<[T; N]> {
        if self.iter().any(|x| x.is_none()) {
            return None
        };
        Some(std::array::from_fn(|idx| {
            let mut x = None;
            std::mem::swap(&mut x, self.get_mut(idx).expect("We shouldn't be indexing outside of `self`"));
            x.expect("we already checked thay weren't None")
        }))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some() {

        let a = [
            Some('a'),
            Some('b'),
            Some('c'),
        ];
        assert_eq!(a.unwrap(), Some(['a', 'b', 'c']));
    }
    #[test]
    fn none() {

        let a = [
            Some('a'),
            Some('b'),
            Some('c'),
            None,
        ];
        assert_eq!(a.unwrap(), None);
    }
}