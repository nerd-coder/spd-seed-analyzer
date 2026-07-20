//! Port of `ItemStatusHandler` label assignment (run-init path only).

use crate::random::Random;

/// Assigns each item a random remaining label, matching:
/// `new ItemStatusHandler(items, labelImages)`.
///
/// `labels` must be in the same order as the Java `LinkedHashMap` key insertion order.
/// `items` must match `Generator.Category.*.classes` order.
pub fn assign_labels(items: &[&str], labels: &[&str]) -> Vec<(String, String)> {
    let mut labels_left: Vec<String> = labels.iter().map(|s| (*s).to_string()).collect();
    let mut out = Vec::with_capacity(items.len());

    for item in items {
        let index = Random::int_max(labels_left.len() as i32) as usize;
        let label = labels_left.remove(index);
        out.push(((*item).to_string(), label));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn assigns_each_label_once() {
        Random::reset_generators();
        Random::push_generator_seeded(99);
        let items = ["A", "B", "C"];
        let labels = ["x", "y", "z"];
        let map = assign_labels(&items, &labels);
        assert_eq!(map.len(), 3);
        let mut used: Vec<_> = map.iter().map(|(_, l)| l.clone()).collect();
        used.sort();
        assert_eq!(used, vec!["x", "y", "z"]);
        Random::pop_generator();
    }
}
