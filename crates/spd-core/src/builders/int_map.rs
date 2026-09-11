//! libGDX 1.14 `IntMap` used by watabou `SparseArray.keyArray()`.
//!
//! GridBuilder picks the next neighbour with `keys[Random.Int(keys.length)]`,
//! so iteration order must match Fibonacci hashing + linear probing, not
//! insertion order.

#![allow(dead_code)] // used by GridBuilder; analyze wiring is PR 4

const FIBONACCI: u64 = 0x9E3779B97F4A7C15;
const LOAD_FACTOR: f32 = 0.8;

pub(super) struct IntMap<V> {
    key_table: Vec<i32>,
    value_table: Vec<Option<V>>,
    zero_value: Option<V>,
    has_zero_value: bool,
    size: i32,
    mask: i32,
    shift: u32,
    threshold: i32,
}

impl<V> IntMap<V> {
    pub(super) fn new() -> Self {
        Self::with_capacity(51)
    }

    fn with_capacity(initial_capacity: i32) -> Self {
        let table_size = table_size(initial_capacity, LOAD_FACTOR);
        let mask = table_size - 1;
        Self {
            key_table: vec![0; table_size as usize],
            value_table: (0..table_size as usize).map(|_| None).collect(),
            zero_value: None,
            has_zero_value: false,
            size: 0,
            mask,
            shift: (mask as u64).leading_zeros(),
            threshold: (table_size as f32 * LOAD_FACTOR) as i32,
        }
    }

    fn place(&self, item: i32) -> usize {
        ((item as i64 as u64).wrapping_mul(FIBONACCI) >> self.shift) as usize
    }

    fn locate_key(&self, key: i32) -> i32 {
        let mask = self.mask as usize;
        let mut i = self.place(key);
        loop {
            let other = self.key_table[i];
            if other == 0 {
                return -(i as i32 + 1);
            }
            if other == key {
                return i as i32;
            }
            i = (i + 1) & mask;
        }
    }

    pub(super) fn put(&mut self, key: i32, value: V) {
        if key == 0 {
            self.zero_value = Some(value);
            if !self.has_zero_value {
                self.has_zero_value = true;
                self.size += 1;
            }
            return;
        }
        let mut i = self.locate_key(key);
        if i >= 0 {
            self.value_table[i as usize] = Some(value);
            return;
        }
        i = -(i + 1);
        self.key_table[i as usize] = key;
        self.value_table[i as usize] = Some(value);
        self.size += 1;
        if self.size >= self.threshold {
            self.resize((self.key_table.len() as i32) << 1);
        }
    }

    pub(super) fn get(&self, key: i32) -> Option<&V> {
        if key == 0 {
            return self.zero_value.as_ref().filter(|_| self.has_zero_value);
        }
        let i = self.locate_key(key);
        if i >= 0 {
            self.value_table[i as usize].as_ref()
        } else {
            None
        }
    }

    pub(super) fn contains_key(&self, key: i32) -> bool {
        if key == 0 {
            self.has_zero_value
        } else {
            self.locate_key(key) >= 0
        }
    }

    /// `SparseArray.keyArray()` — table order, zero-key first when present.
    pub(super) fn key_array(&self) -> Vec<i32> {
        let mut keys = Vec::with_capacity(self.size as usize);
        if self.has_zero_value {
            keys.push(0);
        }
        for &key in &self.key_table {
            if key != 0 {
                keys.push(key);
            }
        }
        keys
    }

    fn resize(&mut self, new_size: i32) {
        let old_keys = std::mem::take(&mut self.key_table);
        let old_values = std::mem::take(&mut self.value_table);
        self.threshold = (new_size as f32 * LOAD_FACTOR) as i32;
        self.mask = new_size - 1;
        self.shift = (self.mask as u64).leading_zeros();
        self.key_table = vec![0; new_size as usize];
        self.value_table = (0..new_size as usize).map(|_| None).collect();
        for (key, value) in old_keys.into_iter().zip(old_values) {
            if key != 0 {
                if let Some(value) = value {
                    self.put_resize(key, value);
                }
            }
        }
    }

    fn put_resize(&mut self, key: i32, value: V) {
        let mask = self.mask as usize;
        let mut i = self.place(key);
        loop {
            if self.key_table[i] == 0 {
                self.key_table[i] = key;
                self.value_table[i] = Some(value);
                return;
            }
            i = (i + 1) & mask;
        }
    }
}

fn table_size(capacity: i32, load_factor: f32) -> i32 {
    let needed = ((capacity as f32 / load_factor).ceil() as i32).max(2);
    next_power_of_two(needed)
}

fn next_power_of_two(mut value: i32) -> i32 {
    if value == 0 {
        return 1;
    }
    value -= 1;
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    value + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_table_is_64_wide_like_libgdx() {
        let map = IntMap::<u8>::new();
        assert_eq!(map.key_table.len(), 64);
        assert_eq!(map.mask, 63);
        assert_eq!(map.shift, 58);
        assert_eq!(map.threshold, 51);
    }

    #[test]
    fn key_array_follows_table_order_not_insertion() {
        let mut map = IntMap::new();
        map.put(100_100, 0usize);
        map.put(101_100, 1usize);
        map.put(100_101, 2usize);
        let keys = map.key_array();
        assert_eq!(keys.len(), 3);
        let positions: Vec<_> = keys
            .iter()
            .map(|key| map.key_table.iter().position(|&k| k == *key).unwrap())
            .collect();
        let mut sorted = positions.clone();
        sorted.sort_unstable();
        assert_eq!(positions, sorted);
    }
}
