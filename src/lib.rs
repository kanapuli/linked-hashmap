use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

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
    fn bucket(&mut self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        //hasher.finish() returns the hash value written so far
        (hasher.finish() % self.buckets.len() as u64) as usize
    }
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
        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];
        self.items += 1;
        //ekey - existing key
        //evalue - existing value
        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                // if the key exists already, replace the existing value
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
        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));
        //get rid of all the things in  current vec
        for (key, val) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket: usize = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, val));
        }
        mem::replace(&mut self.buckets, new_buckets);
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let bucket = self.bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|&(ref ekey, _)| ekey == key)
            .map(|&(_, ref evalue)| evalue)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn insert() {
        let mut map = HashMap::new();
        map.insert("foo", "bar");
        //        assert!((map.get(&"foo")), Some(&"bar"));
    }
}
