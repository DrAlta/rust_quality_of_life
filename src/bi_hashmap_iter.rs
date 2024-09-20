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
