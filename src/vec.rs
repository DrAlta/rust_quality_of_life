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
