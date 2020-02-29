use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const INITIAL_NBUCKETS: usize = 1;
//HashMap is the list of Buckets
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    //new creates a  new hashMap
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items: 0,
        }
    }
    //insert inserts a bucket into Linked HashMap
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {

        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize()
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        //hasher.finish() returns the hash value written so far
        let bucket: usize = (hasher.finish() % self.buckets.len() as u64) as usize;
        let bucket = &mut self.buckets[bucket];
        self.items += 1;
        //ekey - existing key
        //evalue - existing value
        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                // if the key exists already, replace the existing value
                use std::mem;
                //return the existing value
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key, value));
        //return None since there is no value exists before
        None
    }
    //resize resizes the HashMaps's bucket length
    fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };
        //TODO:
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn insert_test() {
        let mut map = HashMap::new();
        map.insert("foo", "bar");
    }
}
