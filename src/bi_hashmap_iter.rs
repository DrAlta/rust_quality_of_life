use std::collections::HashMap;

pub struct BiHashMapIter<'a, O, I, V>
{
    o: <&'a HashMap<O, HashMap<I, V>> as IntoIterator>::IntoIter,
    i_maybe: Option<(&'a O, <&'a HashMap<I, V> as IntoIterator>::IntoIter)>,
}

impl<'a, O, I, V> Iterator for BiHashMapIter<'a, O, I, V> {
    type Item = ((&'a O, &'a I), &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(i) = &mut self.i_maybe {
                let Some(next) = i.1.next() else {
                    // we've reached the end of the inner iter
                    self.i_maybe = None;
                    continue;
                };
                return Some(((i.0, next.0), next.1)) 
            } else {
                let Some(new_i) = self.o.next() else {
                    return None;
                };
                self.i_maybe = Some((new_i.0, new_i.1.iter()));
                continue;                
            };
        }
    }
}


impl<'a, O, I, V> BiHashMapIter<'a, O, I, V> {
    pub fn new(o:<&'a HashMap<O,HashMap<I, V>> as IntoIterator>::IntoIter) -> Self {
        Self { o, i_maybe: None }
    }
}
#[cfg(test)]
mod tests {
    use crate::BiHashMap;

    use super::*;

    
    #[test]
    fn bi_hashmap_iter_test() {  
        let a = BiHashMap::from_inner(
            HashMap::from([
                (1_u8, HashMap::from([
                    (2_i8, 3_i16)
                ]))
            ])
        );

        let mut b = a.iter();
        assert_eq!(
            Some(((&1,&2), &3)),
            b.next()
        );
        assert_eq!(
            None,
            b.next()
        );
        assert_eq!(
            None,
            b.next()
        );
    }
}