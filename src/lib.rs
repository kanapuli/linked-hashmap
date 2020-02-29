use std::collections::hash_map::DefaultHasher;
const INITIAL_NBUCKETS: usize = 1;
//Bucket is the list of tuples of Key and Values
struct Bucket<K,V> {
    items: Vec<(K,V)>,

}
//HashMap is the list of Buckets
pub struct HashMap<K,V> {
    buckets: Vec<Bucket<K,V>>
}

impl<K,V> HashMap<K,V> 
where: K: Hash
{

    //new creates a  new hashMap
    pub fn new() -> Self {
        HashMap{
            buckets: Vec::new(),
        }
    }
    //insert inserts a bucket into Linked HashMap
    pub fn insert(&mut self, key: K, value: V){
        let hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let bucket  = hasher.finish() % self.buckets.len();
        let bucket = &mut self.buckets[bucket]

    }
    //resize resizes the HashMaps's bucket length
    fn resize(&mut self){
        let target_size = match self.buckets.len(){
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };        
        //TODO:
    }
}