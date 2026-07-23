//! Runtime `GeneratorState` — decks, draws, and category RNG.

use super::categories::CategoryDef;
use super::categories::{FLOOR_SET_TIER_PROBS, MIS_TIERS, WEP_TIERS};
use super::Category;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::random::Random;

#[derive(Debug, Clone)]
struct CatRuntime {
    def: CategoryDef,
    probs: Vec<f32>,
    using_2nd_probs: bool,
    seed: Option<i64>,
    dropped: i32,
    default_probs_total: Option<Vec<f32>>,
}

#[derive(Debug, Clone)]
pub struct GeneratorState {
    using_first_deck: bool,
    /// Parallel to Category::ALL — overall category deck weights.
    category_probs: Vec<f32>,
    default_cat_probs: Vec<f32>,
    cats: Vec<CatRuntime>,
}

impl GeneratorState {
    /// `Generator.fullReset()` matching Java call order.
    pub fn full_reset_ordered() -> Self {
        let using_first_deck = Random::int_max(2) == 0;

        let mut category_probs = Vec::with_capacity(Category::ALL.len());
        let mut default_cat_probs = Vec::with_capacity(Category::ALL.len());
        for &cat in Category::ALL {
            let def = cat.def();
            category_probs.push(if using_first_deck {
                def.first_prob
            } else {
                def.second_prob
            });
            default_cat_probs.push(def.first_prob + def.second_prob);
        }

        let mut cats = Vec::with_capacity(Category::ALL.len());
        for &cat in Category::ALL {
            let def = cat.def();
            let mut using_2nd = def.default_probs2.is_some() && Random::int_max(2) == 0;
            if def.default_probs2.is_some() {
                using_2nd = !using_2nd;
            }
            let probs = if let Some(p2) = def.default_probs2 {
                if using_2nd {
                    p2.to_vec()
                } else {
                    def.default_probs.unwrap().to_vec()
                }
            } else if let Some(p) = def.default_probs {
                p.to_vec()
            } else if cat == Category::Gold {
                // GOLD.probs = {1} with no defaultProbs / seed
                vec![1.0]
            } else if cat == Category::Armor {
                // ARMOR.probs used only for non-randomArmor paths; keep Java static
                vec![1., 1., 1., 1., 1., 0., 0., 0., 0., 0., 0.]
            } else {
                Vec::new()
            };
            let default_probs_total = match (def.default_probs, def.default_probs2) {
                (Some(a), Some(b)) => Some(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()),
                _ => None,
            };
            let seed = if def.default_probs.is_some() {
                Some(Random::long())
            } else {
                None
            };
            cats.push(CatRuntime {
                def,
                probs,
                using_2nd_probs: using_2nd,
                seed,
                dropped: 0,
                default_probs_total,
            });
        }

        Self {
            using_first_deck,
            category_probs,
            default_cat_probs,
            cats,
        }
    }

    fn general_reset(&mut self) {
        for (i, &cat) in Category::ALL.iter().enumerate() {
            let def = cat.def();
            self.category_probs[i] = if self.using_first_deck {
                def.first_prob
            } else {
                def.second_prob
            };
            self.default_cat_probs[i] = def.first_prob + def.second_prob;
        }
    }

    fn reset_cat(&mut self, cat: Category) {
        let rt = &mut self.cats[cat.index()];
        if let Some(p2) = rt.def.default_probs2 {
            rt.using_2nd_probs = !rt.using_2nd_probs;
            rt.probs = if rt.using_2nd_probs {
                p2.to_vec()
            } else {
                rt.def.default_probs.unwrap().to_vec()
            };
        } else if let Some(p) = rt.def.default_probs {
            rt.probs = p.to_vec();
        }
    }

    /// `Generator.random()` — pick category then item.
    pub fn random(&mut self, depth: i32) -> GeneratedItem {
        let mut idx = Random::chances(&self.category_probs);
        if idx < 0 {
            self.using_first_deck = !self.using_first_deck;
            self.general_reset();
            idx = Random::chances(&self.category_probs);
        }
        let cat = Category::ALL[idx as usize];
        self.category_probs[idx as usize] -= 1.0;

        if cat == Category::Seed {
            self.random_using_defaults(cat, depth)
        } else {
            self.random_category(cat, depth)
        }
    }

    pub fn random_category(&mut self, cat: Category, depth: i32) -> GeneratedItem {
        match cat {
            Category::Armor => self.random_armor(depth / 5, depth),
            Category::Weapon => self.random_weapon(depth / 5, false, depth),
            Category::Missile => self.random_missile(depth / 5, false, depth),
            Category::Artifact => {
                if let Some(item) = self.random_artifact(depth) {
                    item
                } else {
                    self.random_category(Category::Ring, depth)
                }
            }
            _ => self.random_deck_item(cat, depth),
        }
    }

    /// `Generator.randomUsingDefaults()` — pick category via `defaultCatProbs`, then item.
    pub fn random_using_defaults_any(&mut self, depth: i32) -> GeneratedItem {
        let idx = Random::chances(&self.default_cat_probs);
        let cat = if idx < 0 {
            // should not happen while defaultCatProbs has weight
            Category::Gold
        } else {
            Category::ALL[idx as usize]
        };
        self.random_using_defaults(cat, depth)
    }

    /// `Generator.randomUsingDefaults(cat)`.
    pub fn random_using_defaults(&mut self, cat: Category, depth: i32) -> GeneratedItem {
        match cat {
            Category::Weapon => self.random_weapon(depth / 5, true, depth),
            Category::Missile => self.random_missile(depth / 5, true, depth),
            _ => {
                let rt = &self.cats[cat.index()];
                if rt.def.default_probs.is_none() || cat == Category::Artifact {
                    return self.random_category(cat, depth);
                }
                let (class_name, consume_exotic_roll) =
                    if let Some(ref total) = rt.default_probs_total {
                        let i = Random::chances(total);
                        assert!(i >= 0, "defaultProbsTotal chances empty");
                        (rt.def.classes[i as usize], false)
                    } else {
                        let i = Random::chances(rt.def.default_probs.unwrap());
                        assert!(i >= 0, "defaultProbs chances empty");
                        (rt.def.classes[i as usize], true)
                    };
                // Java returns directly from the cumulative
                // `defaultProbsTotal` branch. Only the plain default-probability
                // path reaches the exotic consumable conversion roll.
                if consume_exotic_roll {
                    consume_exotic_conversion_roll(cat);
                }
                let mut item = GeneratedItem::new(class_name, rt.def.item_category);
                randomize_item(&mut item, depth);
                item
            }
        }
    }

    fn random_deck_item(&mut self, cat: Category, depth: i32) -> GeneratedItem {
        let idx = cat.index();
        // Push category seed deck if present
        let (seed, dropped) = {
            let rt = &self.cats[idx];
            (rt.seed, rt.dropped)
        };
        if let Some(s) = seed {
            Random::push_generator_seeded(s);
            for _ in 0..dropped {
                Random::long();
            }
        }

        let mut i = Random::chances(&self.cats[idx].probs);
        if i < 0 {
            // Java keeps this category generator pushed while it resets the
            // exhausted probabilities and makes the second selection.
            self.reset_cat(cat);
            i = Random::chances(&self.cats[idx].probs);
        }

        if i < 0 {
            if seed.is_some() {
                Random::pop_generator();
            }
            // Fallback (should not happen with valid tables)
            let mut item = GeneratedItem::new("Gold", ItemCategory::Gold);
            randomize_item(&mut item, depth);
            return item;
        }

        let i = i as usize;
        if self.cats[idx].def.default_probs.is_some() {
            self.cats[idx].probs[i] -= 1.0;
        }
        let class_name = self.cats[idx].def.classes[i];
        let item_cat = self.cats[idx].def.item_category;

        if seed.is_some() {
            Random::pop_generator();
            self.cats[idx].dropped += 1;
        }

        consume_exotic_conversion_roll(cat);
        let mut item = if cat == Category::Gold {
            GeneratedItem::new("Gold", ItemCategory::Gold)
        } else {
            GeneratedItem::new(class_name, item_cat)
        };
        randomize_item(&mut item, depth);
        item
    }

    pub fn random_armor(&mut self, floor_set: i32, depth: i32) -> GeneratedItem {
        let floor_set = floor_set.clamp(0, FLOOR_SET_TIER_PROBS.len() as i32 - 1) as usize;
        let tier = Random::chances(&FLOOR_SET_TIER_PROBS[floor_set]) as usize;
        let class_name = self.cats[Category::Armor.index()].def.classes[tier];
        let mut item = GeneratedItem::new(class_name, ItemCategory::Armor);
        randomize_item(&mut item, depth);
        item
    }

    pub fn random_weapon(
        &mut self,
        floor_set: i32,
        use_defaults: bool,
        depth: i32,
    ) -> GeneratedItem {
        let floor_set = floor_set.clamp(0, FLOOR_SET_TIER_PROBS.len() as i32 - 1) as usize;
        let tier = Random::chances(&FLOOR_SET_TIER_PROBS[floor_set]) as usize;
        let tier_cat = WEP_TIERS[tier];
        if use_defaults {
            self.random_using_defaults(tier_cat, depth)
        } else {
            self.random_deck_item(tier_cat, depth)
        }
    }

    pub fn random_missile(
        &mut self,
        floor_set: i32,
        use_defaults: bool,
        depth: i32,
    ) -> GeneratedItem {
        let floor_set = floor_set.clamp(0, FLOOR_SET_TIER_PROBS.len() as i32 - 1) as usize;
        let tier = Random::chances(&FLOOR_SET_TIER_PROBS[floor_set]) as usize;
        let tier_cat = MIS_TIERS[tier];
        if use_defaults {
            self.random_using_defaults(tier_cat, depth)
        } else {
            self.random_deck_item(tier_cat, depth)
        }
    }

    pub fn random_artifact(&mut self, depth: i32) -> Option<GeneratedItem> {
        let cat = Category::Artifact;
        let idx = cat.index();
        let (seed, dropped) = {
            let rt = &self.cats[idx];
            (rt.seed, rt.dropped)
        };
        if let Some(s) = seed {
            Random::push_generator_seeded(s);
            for _ in 0..dropped {
                Random::long();
            }
        }

        let i = Random::chances(&self.cats[idx].probs);

        if seed.is_some() {
            Random::pop_generator();
            self.cats[idx].dropped += 1;
        }

        if i < 0 {
            return None;
        }

        self.cats[idx].probs[i as usize] -= 1.0;
        let class_name = self.cats[idx].def.classes[i as usize];
        let mut item = GeneratedItem::new(class_name, ItemCategory::Artifact);
        randomize_item(&mut item, depth);
        Some(item)
    }

    pub fn using_first_deck(&self) -> bool {
        self.using_first_deck
    }

    #[cfg(test)]
    pub(super) fn deck_state(&self, cat: Category) -> (i64, i32, f32) {
        let runtime = &self.cats[cat.index()];
        (
            runtime.seed.expect("deck category seed"),
            runtime.dropped,
            runtime.probs.iter().map(|prob| prob.max(0.0)).sum(),
        )
    }

    /// `Generator.undoDrop(Class)` — put a drawn class back into its category deck.
    ///
    /// Does not reverse the category-seed `dropped` counter (matches Java).
    /// Java checks `cls.isAssignableFrom(cat.superClass)`, which never holds for
    /// concrete item classes; we instead match by class name so re-rolls (e.g.
    /// Wandmaker unique wands) restore the intended deck weight.
    pub fn undo_drop(&mut self, class_name: &str) {
        for cat in Category::ALL {
            let idx = cat.index();
            let rt = &mut self.cats[idx];
            if rt.def.default_probs.is_none() {
                continue;
            }
            if let Some(i) = rt.def.classes.iter().position(|&c| c == class_name) {
                if i < rt.probs.len() {
                    rt.probs[i] += 1.0;
                }
            }
        }
    }

    /// `Category.defaultProbsTotal[i]` for a class (used by CrystalPath rarity sort).
    pub fn default_prob_total(&self, cat: Category, class_name: &str) -> f32 {
        let rt = &self.cats[cat.index()];
        let Some(i) = rt.def.classes.iter().position(|&c| c == class_name) else {
            return 0.0;
        };
        if let Some(ref total) = rt.default_probs_total {
            return total.get(i).copied().unwrap_or(0.0);
        }
        if let Some(p) = rt.def.default_probs {
            return p.get(i).copied().unwrap_or(0.0);
        }
        0.0
    }
}

/// Java checks every regular potion and scroll against its exotic counterpart
/// after leaving the category's seeded deck. The current engine models no
/// ExoticCrystals trinket, so conversion is impossible, but `Random.Float()`
/// is still evaluated and must advance the active level-generation stream.
fn consume_exotic_conversion_roll(cat: Category) {
    if matches!(cat, Category::Potion | Category::Scroll) {
        let _ = Random::float();
    }
}

/// `Generator.fullReset()` entry used by run init.
pub fn full_reset() -> GeneratorState {
    GeneratorState::full_reset_ordered()
}
