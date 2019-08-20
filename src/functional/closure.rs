use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T, K, V>
{
    func: T,
    value: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
    // which Fn should use? Fn, FnOnce, FnMut
    where T: Fn(K) -> V,
          K: Hash + Eq + Copy,
          V: Clone
{
    pub fn new(fun: T) -> Cacher<T, K, V> {
        Cacher {
            func: fun,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: K) -> V
    {
        match self.value.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = &(self.func)(arg);
                self.value.insert(arg, v.clone());
                v.clone()
            }
        }
    }
}

