use std::{fmt::Debug};

#[derive(Default)]
struct TestMap<K, V> {
    containers: u32,
    taken: u32,
    map: Vec<Option<(K, V)>>,
}

impl<K: Default + Debug + Copy, V: Default + Copy> TestMap<K, V> {
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
            map: vec![Default::default(); 10]
        }
    }

    fn insert(&mut self, key: K, val: V ) {
        let hash = Self::hash(&format!("{:?}", key));
        let index: u32 = hash % self.containers;
        
        self.map[index as usize] = Some((key, val));
    }

    fn get(&self, key: K) -> Option<V> {
        let hash = Self::hash(&format!("{:?}", key));
        let index: u32 = hash % self.containers;
        println!("hash {hash}, index {index}");
        if let Some(k) = self.map[index as usize] {
            return Some(k.1)
        } 
        None
    }
}

fn main() {
    let mut m = TestMap::new();
    m.insert("hello", "world");
    println!("{:?}", m.get("hello"));
    println!("{:?}", m.get("hell1"));
}
