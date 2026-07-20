//! Port of `Generator.fullReset` (deck seeds for later item rolls).

use crate::random::Random;

/// Categories in `Generator.Category.values()` declaration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Trinket,
    Weapon,
    WepT1,
    WepT2,
    WepT3,
    WepT4,
    WepT5,
    Armor,
    Missile,
    MisT1,
    MisT2,
    MisT3,
    MisT4,
    MisT5,
    Wand,
    Ring,
    Artifact,
    Food,
    Potion,
    Seed,
    Scroll,
    Stone,
    Gold,
}

impl Category {
    pub const ALL: &'static [Category] = &[
        Category::Trinket,
        Category::Weapon,
        Category::WepT1,
        Category::WepT2,
        Category::WepT3,
        Category::WepT4,
        Category::WepT5,
        Category::Armor,
        Category::Missile,
        Category::MisT1,
        Category::MisT2,
        Category::MisT3,
        Category::MisT4,
        Category::MisT5,
        Category::Wand,
        Category::Ring,
        Category::Artifact,
        Category::Food,
        Category::Potion,
        Category::Seed,
        Category::Scroll,
        Category::Stone,
        Category::Gold,
    ];

    fn has_default_probs(self) -> bool {
        !matches!(
            self,
            Category::Weapon | Category::Armor | Category::Missile | Category::Gold
        )
    }

    fn has_default_probs2(self) -> bool {
        matches!(self, Category::Potion | Category::Scroll)
    }
}

#[derive(Debug, Clone)]
pub struct CategoryState {
    pub category: Category,
    pub using_2nd_probs: bool,
    pub seed: Option<i64>,
    pub dropped: i32,
}

#[derive(Debug, Clone)]
pub struct GeneratorState {
    pub using_first_deck: bool,
    pub categories: Vec<CategoryState>,
}

/// `Generator.fullReset()`.
pub fn full_reset() -> GeneratorState {
    let using_first_deck = Random::int_max(2) == 0;
    // generalReset() only fills HashMaps — no RNG.

    let mut categories = Vec::new();
    for &cat in Category::ALL {
        // using2ndProbs = defaultProbs2 != null && Random.Int(2) == 0
        let mut using_2nd = cat.has_default_probs2() && Random::int_max(2) == 0;
        // reset(cat) flips using2ndProbs when defaultProbs2 is present
        if cat.has_default_probs2() {
            using_2nd = !using_2nd;
        }

        let seed = if cat.has_default_probs() {
            Some(Random::long())
        } else {
            None
        };

        categories.push(CategoryState {
            category: cat,
            using_2nd_probs: using_2nd,
            seed,
            dropped: 0,
        });
    }

    GeneratorState {
        using_first_deck,
        categories,
    }
}
