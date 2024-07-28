use std::{fmt::Debug};

#[derive(Default)]
struct TestMap<K, V> {
    containers: u32,
    taken: u32,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Default + Debug + Copy + Eq, V: Default + Copy + Eq> TestMap<K, V> {
    fn hash(str: &str) -> u32 {
        let mut hash: u32 = 2166136261;

        for byte in str.as_bytes() {
            hash = hash ^ hash.wrapping_mul(*byte as u32);
            hash = hash.wrapping_add(16777219);
        }

        hash
    }

    fn new() -> Self {
        TestMap {
            containers: 10,
            taken: 0,
            buckets: vec![Default::default(); 10]
        }
    }

    fn insert(&mut self, key: K, val: V ) {
        let hash = Self::hash(&format!("{:?}", key));
        let index: u32 = hash % self.containers;

        let bucket =&mut self.buckets[index as usize];
        for (k, v) in bucket.iter_mut() {
            if k == &key {
                *v = val;
                return;
            }
        }
        bucket.push((key, val));
    }

    fn get(&self, key: K) -> Option<V> {
        let hash = Self::hash(&format!("{:?}", key));
        let index: u32 = hash % self.containers;

        let bucket = &self.buckets[index as usize];

        for (k, v) in bucket.iter() {
            if *k == key {
                return Some(*v);
            }
        }
        
        None
    }
}

fn main() {
    let mut m: TestMap<&str, &str> = TestMap::new();
    m.insert("hello", "world");
    m.insert("hello1", "world");
    println!("{:?}, ", m.get("hello"));
    println!("{:?}, ", m.get("hel2lo"));
}
