use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub struct Cacher<F, K, V> {
    func: F,
    value: HashMap<K, V>,
}


// cache the key
impl<F, K, V> Cacher<F, K, V>
// which Fn should use ???  Fn, FnOnce, FnMut
    where F: Fn(K) -> V,
          K: Hash + Eq + Copy,
          V: Clone
{
    pub fn new(func: F) -> Cacher<F, K, V> {
        Cacher {
            func: func,
            value: HashMap::new(),
        }
    }

    pub fn get(&mut self, arg: K, mut vstore: V) -> V {
        match self.value.get(&arg) {
            Some(v) => v.clone(),
            None => {
                vstore = (self.func)(arg);
                self.value.insert(arg, vstore.clone());
                vstore
            }
        }
    }

    pub fn get_boxptr(&mut self, arg: K) -> Box<V> {
        match self.value.get(&arg) {
            Some(v) => Box::new(v.clone()),
            None => {
                let v = (self.func)(arg);
                self.value.insert(arg, v.clone());
                Box::new(v)
            }
        }
    }

    pub fn get_rcptr(&mut self, arg: K) -> Rc<V> {
        match self.value.get(&arg) {
            Some(v) => Rc::new(v.clone()),
            None => {
                let v = (self.func)(arg);
                self.value.insert(arg, v.clone());
                Rc::new(v)
            }
        }
    }
}
