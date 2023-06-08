#![recursion_limit = "1024"]
use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};
trait Interval<'a, K, V> {
    fn init(&mut self, _key_begin: K, _key_end: K);
    fn assign(&mut self, _key_begin: K, _key_end: K, _value: V);
    fn delete_keys(&mut self, _keys: &Vec<K>);
    fn delete_range(&mut self, _key_begin: K, _key_end: K);
    fn make_canonical(&mut self);
}

#[derive(Debug)]
struct Map<'a, K, V> {
    val_begin: V,
    mymap: &'a mut BTreeMap<K, V>,
}

impl<'a, K, V> Interval<'a, K, V> for Map<'a, K, V>
where
    K: Copy + std::cmp::Eq + std::hash::Hash + std::cmp::Ord + std::fmt::Debug + 'a,
    V: Copy + std::cmp::PartialEq + std::default::Default + std::fmt::Debug + 'a,
{
    fn init(&mut self, _key_begin: K, _key_end: K) {
        self.mymap.insert(_key_begin, self.val_begin);
        self.mymap.insert(_key_end, self.val_begin);
    }
    fn assign(&mut self, _key_begin: K, _key_end: K, _value: V) {
        if !(_key_begin < _key_end)
            || self.mymap.first_key_value().unwrap().0 >= &_key_begin
            || self.mymap.last_key_value().unwrap().0 < &_key_end
            || _value == self.val_begin
        {
            return;
        }
        let mut what_ends: V = Default::default();
        for(_, value) in self.mymap.range((Unbounded, Included(_key_begin))) {
            what_ends = *value;
        }
        self.delete_range(_key_begin, _key_end);
        self.mymap.insert(_key_begin, _value);
        self.mymap.insert(_key_end, what_ends);
    }
    fn delete_range(&mut self, _key_begin: K, _key_end: K) {
        self.mymap
            .retain(|&key, _| key >= _key_begin || key < _key_end);
    }
    fn make_canonical(&mut self) {
        let mut current: V = Default::default();
        let mut previous: V;
        let mut keys: Vec<K> = vec![];
        let last_key = self.mymap.last_key_value().unwrap().0;
        for (_key, _value) in &*self.mymap {
            previous = current;
            current = *_value;
            if current == previous && _key != last_key {
                keys.push(*_key);
            }
        }
        self.delete_keys(&keys);
    }
    fn delete_keys(&mut self, _keys: &Vec<K>) {
        for _key in _keys {
            let _ = &self.mymap.remove(&_key);
        }
    }
}

fn main() {
    let mut my_map: Map<i32, char> = Map::<i32, char> {
        val_begin: '_',
        mymap: &mut BTreeMap::<i32, char>::new(),
    };
    my_map.init(i32::MIN, i32::MAX);
    my_map.assign(1, 2, 'A');
    my_map.assign(2, 5, 'A');
    my_map.assign(10, 50, 'A');
    my_map.assign(20, 30, 'B');
    my_map.make_canonical();
    println!("Hello, world! {:?}", my_map);
}
