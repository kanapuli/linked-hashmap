use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash, Hasher};
use std::mem;
const INITIAL_BUCKET_SIZE: usize = 1;
pub struct HashMap<K,V > {
    buckets: Vec<Vec<(K,V)>>,
    items: usize,
}

impl <K, V> HashMap<K,V>
{
    pub fn new() -> Self{
       HashMap{
            buckets: Vec::new(),
            items: 0,
        }
    }

}

impl <K, V> HashMap<K,V>
where K: Hash  + Eq {
    fn bucket(&self, key: &K) -> usize{
        let mut hasher =  DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V>{
    
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len()/4 {
            //if bucket is empty or the item count is three fourth of bucket length
            //call resize
            self.resize();
        }
        let bucket = self.bucket(&key);
        let bucket = &mut self.buckets[bucket];
        for &mut (ref ekey,ref mut evalue) in  bucket.iter_mut() {
            //the given key exists already in the hashmap
            if ekey == &key {
                //replace the existing value
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key,value));
        None 
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket = self.bucket(key);
        self.buckets[bucket]
            .iter()
            .find(|&(ref ekey, _)| ekey == key)
            .map(|&( _, ref evalue)| evalue)
    }
    fn resize(&mut self){
        let target_size = match self.buckets.len(){
            0 => INITIAL_BUCKET_SIZE,
            n => 2 * n,
        };
        let mut new_buckets =  Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));
        for (key, value) in  self
            .buckets
            .iter_mut()
            .flat_map(|bucket| bucket.drain(..)){
        let mut hasher =  DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket = (hasher.finish() % new_buckets.len() as u64) as usize ;
        new_buckets[bucket].push((key,value));
        }
        mem::replace(&mut self.buckets, new_buckets);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  insert(){
        let mut map = HashMap::new();
        map.insert("bar", 96);
        //call get to cross verify
        let value = map.get(&"bar");
        assert_eq!(value, Some(&96));
    }
}
