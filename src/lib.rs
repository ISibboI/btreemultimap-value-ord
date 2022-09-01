use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BTreeMultiMap<Key, Value> {
    map: BTreeMap<Key, BTreeMap<Value, usize>>,
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

    pub fn insert(&mut self, key: Key, value: Value)
    where
        Key: Ord,
        Value: Ord,
    {
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

    pub fn get(&self, key: &Key) -> Option<&BTreeMap<Value, usize>>
    where
        Key: Ord,
    {
        self.map.get(key)
    }

    pub fn get_value_count(&self, key: &Key, value: &Value) -> usize
    where
        Key: Ord,
        Value: Ord,
    {
        if let Some(map) = self.map.get(key) {
            map.get(value).copied().unwrap_or(0)
        } else {
            0
        }
    }

    pub fn remove(&mut self, key: &Key) -> Option<BTreeMap<Value, usize>>
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

impl<Key, Value> Default for BTreeMultiMap<Key, Value> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<Key, Value> FromIterator<(Key, Value)> for BTreeMultiMap<Key, Value>
where
    Key: Ord,
    Value: Ord,
{
    fn from_iter<T: IntoIterator<Item = (Key, Value)>>(iter: T) -> Self {
        let mut result = Self::default();
        for (key, value) in iter {
            result.insert(key, value);
        }
        result
    }
}
