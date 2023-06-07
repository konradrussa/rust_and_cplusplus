#![recursion_limit = "1024"]
use std::collections::BTreeMap;
trait Interval<'a, K, V> {
    fn init(&mut self, _key_begin: K);
    fn assign(&mut self, _key_begin: K, _key_end: K, _value: V);
    fn delete_keys(&mut self, _keys: Vec<K>);
    fn make_canonical(&mut self);
}

#[derive(Debug)]
struct Map<'a, K, V> {
    val_begin: V,
    mymap: &'a mut BTreeMap<K, V>,
}

impl<'a, K, V> Interval<'a, K, V> for Map<'a, K, V>
where
    K: Copy + std::cmp::Eq + std::hash::Hash + std::cmp::Ord + 'a,
    V: Copy + std::cmp::PartialEq + std::default::Default + 'a,
{
    fn init(&mut self, _key_begin: K) {
        self.mymap.insert(_key_begin, self.val_begin);
    }
    fn assign(&mut self, _key_begin: K, _key_end: K, _value: V) {
        self.mymap.insert(_key_begin, _value);
        self.mymap.insert(_key_end, self.val_begin);
    }
    fn delete_keys(&mut self, _keys: Vec<K>) {
        for _key in _keys {
            let _ = &self.mymap.remove(&_key);
        }
    }

    fn make_canonical(&mut self) {
        let mut current: V = Default::default();
        let mut previous: V;
        let mut keys = vec![];
        for (_key, _value) in &*self.mymap {
            previous = current;
            current = *_value;
            if current == previous {
                keys.push(*_key);
            }
        }
        self.delete_keys(keys);
    }
}

fn main() {
    let mut my_map: Map<i32, char> = Map::<i32, char> {
        val_begin: '_',
        mymap: &mut BTreeMap::<i32, char>::new(),
    };
    my_map.init(i32::MIN);
    my_map.assign(1, 2, 'A');
    my_map.assign(2, 5, 'A');
    my_map.assign(10, 50, 'A');
    my_map.make_canonical();
    println!("Hello, world! {:?}", my_map);
}
