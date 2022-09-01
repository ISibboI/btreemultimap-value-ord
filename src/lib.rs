use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct BTreeMultiMap<Key, Value> {
    map: BTreeMap<Key, BTreeSet<Value>>,
}

impl<Key, Value> BTreeMultiMap<Key, Value> {
    pub fn new() -> Self
    where
        Key: Default,
        Value: Default,
    {
        Default::default()
    }

    pub fn clear(&mut self) {
        self.map.clear()
    }

    pub fn insert(&mut self, key: Key, value: Value) -> bool
    where
        Key: Ord,
        Value: Ord,
    {
        if let Some(set) = self.map.get_mut(&key) {
            set.insert(value)
        } else {
            self.map.insert(key, BTreeSet::from([value]));
            true
        }
    }

    pub fn get(&self, key: &Key) -> Option<&BTreeSet<Value>>
    where
        Key: Ord,
    {
        self.map.get(key)
    }

    pub fn remove(&mut self, key: &Key) -> Option<BTreeSet<Value>>
    where
        Key: Ord,
    {
        self.map.remove(key)
    }

    pub fn remove_key_value(&mut self, key: &Key, value: &Value) -> bool
    where
        Key: Ord,
        Value: Ord,
    {
        if let Some(set) = self.map.get_mut(key) {
            set.remove(value)
        } else {
            false
        }
    }
}
