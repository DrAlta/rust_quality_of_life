use super::BiHashMap;

impl<'a, 'b, O, I, V> FromIterator<((&'a O,&'b I), V)> for BiHashMap<O, I, V> where
    O: Eq + std::hash::Hash + Clone, 
    I: Eq + std::hash::Hash + Clone
{
    fn from_iter<T: IntoIterator<Item = ((&'a O,&'b I), V)>>(iter: T) -> Self {
        let mut ret = BiHashMap::new();
        for ((o,i), v) in iter{
            ret.insert((o.clone(), i.clone()), v);
        }
        ret
    }
}

impl<O, I, V> FromIterator<((O, I), V)> for BiHashMap<O, I, V> where
    O: Eq + std::hash::Hash + Clone, 
    I: Eq + std::hash::Hash + Clone
{
    fn from_iter<T: IntoIterator<Item = ((O, I), V)>>(iter: T) -> Self {
        let mut ret = BiHashMap::new();
        for ((o, i), v) in iter{
            ret.insert((o, i), v);
        }
        ret
    }
}
