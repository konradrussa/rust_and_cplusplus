#![recursion_limit = "1024"]
use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};
trait Interval<'a, K, V> {
    fn init(&mut self, _key_begin: K, _key_end: K);
    fn assign(&mut self, _key_begin: K, _key_end: K, _value: V);
    fn delete_keys(&mut self, _keys: &Vec<K>);
    fn delete_range(&mut self, _key_begin: K, _key_end: K);
    fn make_canonical(&mut self);
    fn get_closest(&self, _key: K) -> Option<(&K, &V)>;
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
            || _key_begin <= *self.mymap.first_key_value().unwrap().0
            || _key_end > *self.mymap.last_key_value().unwrap().0
            || _value == self.val_begin
        {
            return;
        }
        let what_ends: V = *self
            .mymap
            .range((Unbounded, Included(_key_end)))
            .last()
            .unwrap()
            .1;
        self.delete_range(_key_begin, _key_end);
        self.mymap.insert(_key_begin, _value);
        self.mymap.insert(_key_end, what_ends);
    }
    fn delete_range(&mut self, _key_begin: K, _key_end: K) {
        self.mymap
            .retain(|&key, _| key <= _key_begin || key > _key_end);
    }
    fn make_canonical(&mut self) {
        let mut current: V = Default::default();
        let mut previous: V;
        let mut keys: Vec<K> = vec![];
        let last_key = *self.mymap.last_key_value().unwrap().0;
        for (_key, _value) in &*self.mymap {
            previous = current;
            current = *_value;
            if current == previous && *_key != last_key {
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
    fn get_closest(&self, _key: K) ->  Option<(&K, &V)> {
        self
            .mymap
            .range((Unbounded, Included(_key)))
            .last()
    }
}

fn main() {
    let mut my_map: Map<i32, char> = Map::<i32, char> {
        val_begin: '_',
        mymap: &mut BTreeMap::<i32, char>::new(),
    };
    my_map.init(i32::MIN, i32::MAX);
    my_map.assign(0, 2, 'A');
    println!("{:?}", my_map);
    my_map.assign(2, 5, 'A');
    println!("{:?}", my_map);
    my_map.assign(10, 50, 'A');
    println!("{:?}", my_map);
    my_map.assign(20, 30, 'B');
    println!("{:?}", my_map);
    println!("Closest for 10: {:?}", my_map.get_closest(10));
    println!("Closest for 25: {:?}", my_map.get_closest(25));
    my_map.make_canonical();
    println!("Canonical {:?}", my_map);
    my_map.assign(-20, 80, 'V');
    println!("Cleares interval {:?}", my_map);
}
/*
Map { val_begin: '_', mymap: {-2147483648: '_', 0: 'A', 2: '_', 2147483647: '_'} }
Map { val_begin: '_', mymap: {-2147483648: '_', 0: 'A', 2: 'A', 5: '_', 2147483647: '_'} }
Map { val_begin: '_', mymap: {-2147483648: '_', 0: 'A', 2: 'A', 5: '_', 10: 'A', 50: '_', 2147483647: '_'} }
Map { val_begin: '_', mymap: {-2147483648: '_', 0: 'A', 2: 'A', 5: '_', 10: 'A', 20: 'B', 30: 'A', 50: '_', 2147483647: '_'} }  
Closest for 10: Some((10, 'A'))
Closest for 25: Some((20, 'B'))
Canonical Map { val_begin: '_', mymap: {-2147483648: '_', 0: 'A', 5: '_', 10: 'A', 20: 'B', 30: 'A', 50: '_', 2147483647: '_'} }
Cleares interval Map { val_begin: '_', mymap: {-2147483648: '_', -20: 'V', 80: '_', 2147483647: '_'} }
 */