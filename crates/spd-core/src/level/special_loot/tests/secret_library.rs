use super::super::secret_rooms::secret_library;
use super::test_room;
use crate::items::model::ItemCategory;
use crate::level::terrain::{paint_minimal, EMPTY_SP};
use crate::random::Random;

#[test]
fn uses_private_distinct_scroll_pool_without_queue_access() {
    Random::reset_generators();
    Random::push_generator_seeded(0x051E_C1A8);
    let room = test_room("SecretLibraryRoom", 8, 8);
    let mut map = paint_minimal(std::slice::from_ref(&room)).expect("library map");
    map.map.fill(EMPTY_SP);
    let loot = secret_library(&room, &mut map);
    Random::pop_generator();

    assert!(matches!(loot.len(), 2 | 3));
    assert!(loot.iter().all(|drop| {
        drop.item.category == ItemCategory::Scroll
            && drop.item.source.as_deref() == Some("SecretLibraryRoom")
    }));
    let mut classes: Vec<_> = loot.iter().map(|drop| &drop.item.class_name).collect();
    classes.sort();
    classes.dedup();
    assert_eq!(classes.len(), loot.len());
    assert_eq!(map.known_heaps.iter().flatten().count(), loot.len());
}
