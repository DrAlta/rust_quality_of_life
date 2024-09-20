use std::collections::HashMap;

#[derive(Debug)]
pub struct BiHashMap<O, I, V>(HashMap<O, HashMap<I, V>>);
impl<O, I, V> BiHashMap<O, I, V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
impl<O, I, V> BiHashMap<O, I, V> {
    pub fn get_inner(&self) -> &HashMap<O, HashMap<I, V>> {
        &self.0
    }
}

#[allow(dead_code)]
impl<O: Eq + std::hash::Hash + Clone, I: Eq + std::hash::Hash, V> BiHashMap<O, I, V> {
    pub fn get(&self, (o, i): (&O, &I)) -> Option<&V> {
        self.0.get(o)?.get(i)
    }
    pub fn get_mut(&mut self, (o, i): (&O, &I)) -> Option<&mut V> {
        self.0.get_mut(o)?.get_mut(i)
    }
    pub fn insert(&mut self, (o, i): (O, I), v: V) -> Option<V> {
        if !self.0.contains_key(&o) {
            self.0.insert(o.clone(), HashMap::new());
        }
        let inner = self
            .0
            .get_mut(&o)
            .expect("we added the outer key if it didn't already exist");
        inner.insert(i, v)
    }
}
impl<O: Eq + std::hash::Hash + Clone, I: Eq + std::hash::Hash, V> BiHashMap<O, I, Vec<V>> {
    pub fn push_or_insert(&mut self, (o, i): (O, I), v: V) {
        if !self.0.contains_key(&o) {
            self.0.insert(o.clone(), HashMap::new());
        }
        let inner = self
            .0
            .get_mut(&o)
            .expect("we added the outer key if it didn't already exist");

        if inner.contains_key(&i) {
            inner
                .get_mut(&i)
                .expect("we allreadycheck is the key was in it")
                .push(v);
        } else {
            inner.insert(i, vec![v]);
        }
    }
}

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

impl<'a, O, I, V> IntoIterator for &'a BiHashMap<O, I, V> {
    type Item = ((&'a O, &'a I), &'a V);

    type IntoIter = BiHashMapIter<'a ,O, I, V>;

    fn into_iter(self) -> Self::IntoIter {
        BiHashMapIter::new(self.0.iter())
        
    }
}