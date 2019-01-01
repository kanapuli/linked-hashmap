use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
const INITIAL_BUCKET_SIZE: usize = 1;
struct Bucket<K,V>{
   items: Vec<(K, V)>, 
}
pub struct HashMap<K,V > {
    buckets: Vec<Bucket<K,V>>,
}

impl <K, V> HashMap<K,V>
{
    pub fn new() -> Self{
       HashMap{
            buckets: Vec::new(),
        }
    }

}

impl <K, V> HashMap<K,V>
where: K: Hash 
    pub fn insert(&mut self, key: K, value: V) -> Option<V>{
        let mut hasher =  DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket = hasher.finish() % self.buckets.len();
        let bucket = &mut self.buckets[bucket];
        for &mut (ekey, evalue) in  bucket {
            //the given key exists already in the hashmap
            if ekey == key {
                use std::mem;
                //replace the existing value
                return Some(mem::replace(evalue, value));
            }
        }
        bucket.push((key,value));
        
    }

    fn resize(&mut self){
        let target_size = match self.buckets.len(){
            0 => INITIAL_BUCKET_SIZE,
            n => 2 * n,
        };
    }
}
