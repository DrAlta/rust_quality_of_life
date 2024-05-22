pub trait VecStuff<T> {
    fn swap_remove_first_item(&mut self, item: &T) -> bool;
    fn find_first(&self, item: &T) -> Option<usize>;
}
impl<T: PartialEq> VecStuff<T> for Vec<T> {
    fn swap_remove_first_item(&mut self, item: &T) -> bool {
        let Some(index) = self.find_first(item) else {
            return false
        };
        self.swap_remove(index);
        true
    }
    fn find_first(&self, item: &T) -> Option<usize> {
        for (idx, other_item) in self.iter().enumerate() {
            if item == other_item {
                return Some(idx);
            };
        };
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_first(){
        let vecna = vec![0,1,9,3,9,5,9,7,9];
        assert_eq!(
            vecna.find_first(&9),
            Some(2)
        )
    }
    #[test]
    fn swap_remove_first_item(){
        let mut vecna = vec![0,1,9,3,9,5,9,7,9,10];
        vecna.swap_remove_first_item(&9);
        assert_eq!(
            vecna,
            vec![0,1,10,3,9,5,9,7,9]
        )
    }
}