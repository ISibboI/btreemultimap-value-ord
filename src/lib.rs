#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BTreeMultiMap<Key: Ord, Value: Ord> {
    map: BTreeMap<Key, BTreeMap<Value, usize>>,
}

impl<Key: Ord, Value: Ord> BTreeMultiMap<Key, Value> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clear(&mut self) {
        self.map.clear()
    }

    pub fn insert(&mut self, key: Key, value: Value) {
        if let Some(map) = self.map.get_mut(&key) {
            if let Some(count) = map.get_mut(&value) {
                *count += 1;
            } else {
                map.insert(value, 1);
            }
        } else {
            self.map.insert(key, BTreeMap::from([(value, 1)]));
        }
    }

    pub fn get(&self, key: &Key) -> Option<&BTreeMap<Value, usize>> {
        self.map.get(key)
    }

    pub fn get_value_count(&self, key: &Key, value: &Value) -> usize {
        if let Some(map) = self.map.get(key) {
            map.get(value).copied().unwrap_or(0)
        } else {
            0
        }
    }

    pub fn remove(&mut self, key: &Key) -> Option<BTreeMap<Value, usize>> {
        self.map.remove(key)
    }

    pub fn remove_key_value(&mut self, key: &Key, value: &Value) -> bool {
        if let Some(map) = self.map.get_mut(key) {
            if let Some(count) = map.get_mut(value) {
                *count -= 1;
                if *count == 0 {
                    map.remove(value);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<Key: Ord, Value: Ord> Default for BTreeMultiMap<Key, Value> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<Key: Ord, Value: Ord> FromIterator<(Key, Value)> for BTreeMultiMap<Key, Value> {
    fn from_iter<T: IntoIterator<Item = (Key, Value)>>(iter: T) -> Self {
        let mut result = Self::default();
        for (key, value) in iter {
            result.insert(key, value);
        }
        result
    }
}
