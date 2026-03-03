/// src/index/cache.rs
/// Implements an LRU cache to store recently accessed HNSW nodes or query vectors.
/// Reduces IndexedDB round-trips to keep the search operations blazing fast.
use lru::LruCache;
use std::num::NonZeroUsize;

pub struct VectorCache {
    cache: LruCache<String, Vec<f32>>,
}

impl VectorCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&Vec<f32>> {
        self.cache.get(key)
    }

    pub fn put(&mut self, key: String, vector: Vec<f32>) {
        self.cache.put(key, vector);
    }
}
