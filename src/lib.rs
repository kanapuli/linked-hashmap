const INITIAL_NBUCKETS: usize = 1;
//Bucket is the list of tuples of Key and Values
struct Bucket<K,V> {
    items: Vec<(K,V)>,

}
//HashMap is the list of Buckets
pub struct HashMap<K,V> {
    buckets: Vec<Bucket<K,V>>
}

impl<K,V> HashMap<K,V> {
    pub fn new() -> Self {
        HashMap{
            buckets: Vec::new(),
        }
    }

    fn resize(&mut self){
        let target_size = match self.buckets.len(){
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };        
        //TODO:
    }
}