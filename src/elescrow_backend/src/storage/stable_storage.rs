use ic_stable_structures::{StableBTreeMap, Storable};
use std::cell::RefCell;
use crate::storage::memory::{Memory, MemoryRegion, get_memory};
use crate::types::errors::ApiError;
pub struct StableStorage<K, V> 
where 
    K: Storable + Ord + Clone,
    V: Storable + Clone,
{
    map: RefCell<StableBTreeMap<K, V, Memory>>,
    region: MemoryRegion,
}

impl<K, V> StableStorage<K, V>
where
    K: Storable + Ord + Clone,
    V: Storable + Clone,
{
    pub fn new(region: MemoryRegion) -> Self {
        Self {
            map: RefCell::new(StableBTreeMap::init(get_memory(region))),
            region,
        }
    }
    
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.map.borrow_mut().insert(key, value)
    }
    
    pub fn get(&self, key: &K) -> Option<V> {
        self.map.borrow().get(key)
    }
    
    pub fn get_or_error(&self, key: &K, resource: &str) -> Result<V, ApiError> {
        self.get(key).ok_or_else(|| ApiError::NotFound {
            resource: resource.to_string(),
        })
    }
    
    pub fn remove(&self, key: &K) -> Option<V> {
        self.map.borrow_mut().remove(key)
    }
    
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.borrow().contains_key(key)
    }
    
    pub fn len(&self) -> u64 {
        self.map.borrow().len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    pub fn clear(&self) {
        let keys: Vec<K> = self.map.borrow().iter().map(|(k, _)| k).collect();
        let mut map = self.map.borrow_mut();
        for key in keys {
            map.remove(&key);
        }
    }
    
    pub fn keys(&self) -> Vec<K> {
        self.map.borrow().iter().map(|(k, _)| k).collect()
    }
    
    pub fn values(&self) -> Vec<V> {
        self.map.borrow().iter().map(|(_, v)| v).collect()
    }
    
    pub fn entries(&self) -> Vec<(K, V)> {
        self.map.borrow().iter().collect()
    }
    
    pub fn filter<F>(&self, predicate: F) -> Vec<(K, V)>
    where
        F: Fn(&K, &V) -> bool,
    {
        self.map.borrow()
            .iter()
            .filter(|(k, v)| predicate(k, v))
            .collect()
    }
    
    pub fn update<F>(&self, key: &K, updater: F) -> Result<V, ApiError>
    where
        F: FnOnce(&mut V),
    {
        let mut value = self.get_or_error(key, "Resource")?;
        updater(&mut value);
        self.insert(key.clone(), value.clone());
        Ok(value)
    }
    
    pub fn get_or_insert_with<F>(&self, key: K, default: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(value) = self.get(&key) {
            value
        } else {
            let value = default();
            self.insert(key, value.clone());
            value
        }
    }
    
    pub fn batch_insert(&self, items: Vec<(K, V)>) {
        let mut map = self.map.borrow_mut();
        for (key, value) in items {
            map.insert(key, value);
        }
    }
    
    pub fn batch_remove(&self, keys: Vec<K>) -> Vec<Option<V>> {
        let mut map = self.map.borrow_mut();
        keys.into_iter().map(|key| map.remove(&key)).collect()
    }
    
    pub fn paginate(&self, offset: u64, limit: u64) -> Vec<(K, V)> {
        self.map.borrow()
            .iter()
            .skip(offset as usize)
            .take(limit as usize)
            .collect()
    }
    
    pub fn memory_region(&self) -> MemoryRegion {
        self.region
    }
}
pub struct IndexedStorage<K, V, I>
where
    K: Storable + Ord + Clone,
    V: Storable + Clone,
    I: Storable + Ord + Clone,
{
    primary: StableStorage<K, V>,
    index: StableStorage<I, K>,
}

impl<K, V, I> IndexedStorage<K, V, I>
where
    K: Storable + Ord + Clone,
    V: Storable + Clone,
    I: Storable + Ord + Clone,
{
    pub fn new(primary_region: MemoryRegion, index_region: MemoryRegion) -> Self {
        Self {
            primary: StableStorage::new(primary_region),
            index: StableStorage::new(index_region),
        }
    }
    
    pub fn insert_indexed(&self, key: K, value: V, index_key: I) -> Option<V> {
        self.index.insert(index_key, key.clone());
        self.primary.insert(key, value)
    }
    
    pub fn get_by_index(&self, index_key: &I) -> Option<V> {
        self.index.get(index_key)
            .and_then(|key| self.primary.get(&key))
    }
    
    pub fn remove_by_index(&self, index_key: &I) -> Option<V> {
        self.index.remove(index_key)
            .and_then(|key| self.primary.remove(&key))
    }
    
    pub fn update_index(&self, old_index: &I, new_index: I, key: &K) -> Result<(), ApiError> {
        self.index.remove(old_index)
            .ok_or_else(|| ApiError::NotFound {
                resource: "Index entry".to_string(),
            })?;
        self.index.insert(new_index, key.clone());
        Ok(())
    }
}

pub struct TimeSeriesStorage<V>
where
    V: Storable + Clone,
{
    storage: StableStorage<u64, V>, 
}

impl<V> TimeSeriesStorage<V>
where
    V: Storable + Clone,
{
    pub fn new(region: MemoryRegion) -> Self {
        Self {
            storage: StableStorage::new(region),
        }
    }
    
    pub fn add(&self, timestamp: u64, value: V) {
        self.storage.insert(timestamp, value);
    }
    
    pub fn range(&self, start: u64, end: u64) -> Vec<(u64, V)> {
        self.storage.filter(|timestamp, _| *timestamp >= start && *timestamp <= end)
    }
    
    pub fn latest(&self) -> Option<(u64, V)> {
        self.storage.entries()
            .into_iter()
            .max_by_key(|(timestamp, _)| *timestamp)
    }
    
    pub fn cleanup_before(&self, timestamp: u64) -> u64 {
        let to_remove: Vec<u64> = self.storage
            .keys()
            .into_iter()
            .filter(|t| *t < timestamp)
            .collect();
        
        let count = to_remove.len() as u64;
        self.storage.batch_remove(to_remove);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_stable_structures::storable::{Bound, Storable};
    use std::borrow::Cow;
    
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct TestKey(u64);
    
    #[derive(Clone, Debug, PartialEq)]
    struct TestValue(String);
    
    impl Storable for TestKey {
        const BOUND: Bound = Bound::Bounded {
            max_size: 8,
            is_fixed_size: true,
        };
        
        fn to_bytes(&self) -> Cow<[u8]> {
            Cow::Owned(self.0.to_le_bytes().to_vec())
        }
        
        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            let arr: [u8; 8] = bytes.as_ref().try_into().unwrap();
            TestKey(u64::from_le_bytes(arr))
        }
    }
    
    impl Storable for TestValue {
        const BOUND: Bound = Bound::Bounded {
            max_size: 100,
            is_fixed_size: false,
        };
        
        fn to_bytes(&self) -> Cow<[u8]> {
            Cow::Owned(self.0.as_bytes().to_vec())
        }
        
        fn from_bytes(bytes: Cow<[u8]>) -> Self {
            TestValue(String::from_utf8(bytes.to_vec()).unwrap())
        }
    }
    
    #[test]
    fn test_stable_storage_operations() {
        let storage: StableStorage<TestKey, TestValue> = StableStorage::new(MemoryRegion::Reserved1);
        
        let key = TestKey(1);
        let value = TestValue("test".to_string());
        assert!(storage.insert(key.clone(), value.clone()).is_none());
        
        assert_eq!(storage.get(&key), Some(value.clone()));
        
        let new_value = TestValue("updated".to_string());
        assert_eq!(storage.insert(key.clone(), new_value.clone()), Some(value));
        assert_eq!(storage.get(&key), Some(new_value.clone()));
        
        assert_eq!(storage.remove(&key), Some(new_value));
        assert_eq!(storage.get(&key), None);
    }
}