//! `VaultLevel` equipment, consumable, prize, and wandering-mob streams.

use std::collections::HashSet;

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::items::enchants::{random_armor_glyph, random_weapon_enchant};
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::random::Random;

const BANNED_WANDS: &[&str] = &["WandOfRegrowth", "WandOfTransfusion", "WandOfCorruption"];
const BANNED_RINGS: &[&str] = &["RingOfWealth", "RingOfMight", "RingOfForce"];

const T1_MOBS: [VaultMob; 2] = [VaultMob::Skeleton, VaultMob::Dm100];
const T2_MOBS: [VaultMob; 3] = [VaultMob::Shaman, VaultMob::Dm200, VaultMob::Ghoul];
const T3_MOBS: [VaultMob; 2] = [VaultMob::Elemental, VaultMob::Golem];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum VaultMob {
    Skeleton,
    Dm100,
    Shaman,
    Dm200,
    Ghoul,
    /// Queued `VaultElemental.class` before `VaultElemental.random()`.
    Elemental,
    FireElemental,
    FrostElemental,
    ShockElemental,
    Golem,
}

impl VaultMob {
    pub(super) fn large(self) -> bool {
        matches!(self, Self::Golem | Self::Dm200)
    }

    pub(super) fn tier(self) -> usize {
        match self {
            Self::Skeleton | Self::Dm100 => 1,
            Self::Shaman | Self::Dm200 | Self::Ghoul => 2,
            Self::Elemental
            | Self::FireElemental
            | Self::FrostElemental
            | Self::ShockElemental
            | Self::Golem => 3,
        }
    }

    fn ctor_rng(self) {
        match self {
            Self::Golem | Self::Dm200 => {
                let _ = Random::int_max(2);
            }
            Self::Shaman => {
                let _ = Random::int_max(5);
            }
            Self::FireElemental | Self::FrostElemental | Self::ShockElemental => {
                let _ = Random::normal_int_range(3, 5);
            }
            _ => {}
        }
    }

    fn resolve(self) -> Self {
        let resolved = if self == Self::Elemental {
            let roll = Random::float();
            if roll < 0.4 {
                Self::FireElemental
            } else if roll < 0.8 {
                Self::FrostElemental
            } else {
                Self::ShockElemental
            }
        } else {
            self
        };
        resolved.ctor_rng();
        resolved
    }
}

struct EquipmentPool {
    by_tier: [Vec<Option<GeneratedItem>>; 4],
    generated: HashSet<String>,
    higher_idx: usize,
    lower_idx: usize,
}

pub(super) struct VaultGen {
    pool: EquipmentPool,
    consumables: [Vec<GeneratedItem>; 4],
    mobs: Vec<VaultMob>,
    t2_solve: [&'static str; 2],
    t3_solve: [&'static str; 2],
}

impl VaultGen {
    fn new() -> Self {
        Self {
            pool: EquipmentPool {
                by_tier: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
                generated: HashSet::new(),
                higher_idx: 0,
                lower_idx: 0,
            },
            consumables: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            mobs: Vec::new(),
            t2_solve: ["StoneOfBlink", "PotionOfInvisibility"],
            t3_solve: ["StoneOfBlink", "PotionOfInvisibility"],
        }
    }

    pub(super) fn create_equipment(
        &mut self,
        dungeon: &mut DungeonState,
        loot_tier: usize,
    ) -> GeneratedItem {
        setup_equipment(&mut self.pool, dungeon);
        take_equipment(&mut self.pool, loot_tier)
    }

    pub(super) fn create_consumable(&mut self, tier: usize) -> GeneratedItem {
        if self.consumables[tier].is_empty() {
            setup_consumables(&mut self.consumables);
        }
        self.consumables[tier].remove(0)
    }

    pub(super) fn create_mob(&mut self) -> VaultMob {
        if self.mobs.is_empty() {
            self.mobs.extend_from_slice(&T1_MOBS);
            self.mobs.push(*Random::one_of(&T1_MOBS));
            self.mobs.extend_from_slice(&T2_MOBS);
            self.mobs.extend_from_slice(&T3_MOBS);
            self.mobs.push(*Random::one_of(&T3_MOBS));
            Random::shuffle_list(&mut self.mobs);
        }
        let mob = self.mobs.remove(0);
        mob.resolve()
    }

    pub(super) fn return_mob(&mut self, mob: VaultMob) {
        self.mobs.insert(0, mob);
    }

    /// `VaultSingleEnemyTreasureRoom` uses `Random.oneOf(T2Mobs)`, not `createMob`.
    pub(super) fn create_t2_mob(&mut self) -> VaultMob {
        Random::one_of(&T2_MOBS).resolve()
    }

    pub(super) fn find_t3_solve(
        &mut self,
        items: &mut Vec<GeneratedItem>,
    ) -> Option<GeneratedItem> {
        Random::shuffle(&mut self.t3_solve);
        for class in self.t3_solve {
            if let Some(item) = find_prize_class(items, class) {
                return Some(item);
            }
        }
        None
    }

    pub(super) fn find_t2_solve(
        &mut self,
        items: &mut Vec<GeneratedItem>,
    ) -> Option<GeneratedItem> {
        Random::shuffle(&mut self.t2_solve);
        for class in self.t2_solve {
            if let Some(item) = find_prize_class(items, class) {
                return Some(item);
            }
        }
        self.find_t3_solve(items)
    }
}

pub(super) fn queue_floor_loot(dungeon: &mut DungeonState) -> VaultGen {
    dungeon.items_to_spawn.clear();
    let mut gen = VaultGen::new();
    for _ in 0..4 {
        let loot = gen.create_equipment(dungeon, 0);
        dungeon.items_to_spawn.push(loot);
    }
    dungeon
        .items_to_spawn
        .push(GeneratedItem::new("Dart", ItemCategory::Missile));
    setup_consumables(&mut gen.consumables);
    for _ in 0..5 {
        dungeon.items_to_spawn.push(gen.create_consumable(0));
    }
    for _ in 0..3 {
        dungeon.items_to_spawn.push(
            dungeon
                .generator
                .random_using_defaults(Category::Food, dungeon.depth),
        );
    }
    gen
}

pub(super) fn find_prize_any(items: &mut Vec<GeneratedItem>) -> Option<GeneratedItem> {
    if items.is_empty() {
        return None;
    }
    if let Some(i) = items
        .iter()
        .position(|it| it.class_name == "TrinketCatalyst")
    {
        return Some(items.remove(i));
    }
    let idx = Random::int_max(items.len() as i32) as usize;
    Some(items.remove(idx))
}

pub(super) fn find_prize_class(
    items: &mut Vec<GeneratedItem>,
    class_name: &str,
) -> Option<GeneratedItem> {
    items
        .iter()
        .position(|it| it.class_name == class_name)
        .map(|i| items.remove(i))
}

pub(super) fn find_prize_equipable(items: &mut Vec<GeneratedItem>) -> Option<GeneratedItem> {
    items
        .iter()
        .position(|it| {
            matches!(
                it.category,
                ItemCategory::Weapon
                    | ItemCategory::Armor
                    | ItemCategory::Missile
                    | ItemCategory::Ring
                    | ItemCategory::Artifact
            )
        })
        .map(|i| items.remove(i))
}

fn take_equipment(pool: &mut EquipmentPool, loot_tier: usize) -> GeneratedItem {
    let list = &mut pool.by_tier[loot_tier];
    let idx = if loot_tier >= 2 {
        let idx = pool.higher_idx;
        if idx >= list.len() {
            pool.higher_idx = 0;
            0
        } else {
            pool.higher_idx += 1;
            idx
        }
    } else {
        let idx = pool.lower_idx;
        if idx >= list.len() {
            pool.lower_idx = 0;
            0
        } else {
            pool.lower_idx += 1;
            idx
        }
    };
    let mut slot = idx;
    while list[slot].is_none() {
        slot += 1;
        if slot >= list.len() {
            slot = 0;
        }
    }
    let mut loot = list[slot].take().expect("vault equipment slot");
    loot.cursed = false;
    loot
}

fn setup_equipment(pool: &mut EquipmentPool, dungeon: &mut DungeonState) {
    for tier in 0..4 {
        if pool.by_tier[tier].iter().all(Option::is_none) {
            pool.by_tier[tier] = setup_equipment_at_tier(pool, dungeon, tier)
                .into_iter()
                .map(Some)
                .collect();
        }
    }
}

fn setup_equipment_at_tier(
    pool: &mut EquipmentPool,
    dungeon: &mut DungeonState,
    loot_tier: usize,
) -> Vec<GeneratedItem> {
    let mut loot_list = Vec::new();
    let first_wep = match loot_tier {
        2 => Category::WepT3,
        3 => Category::WepT4,
        _ => Category::WepT2,
    };
    let loot = draw_unique(
        dungeon,
        first_wep,
        loot_tier > 1,
        &mut pool.generated,
        |_| false,
    );
    loot_list.push(finish_weapon(
        loot,
        if loot_tier == 0 {
            loot_tier as i32
        } else {
            loot_tier as i32 + 1
        },
        loot_tier,
    ));

    let second_wep = match loot_tier {
        0 => Category::WepT2,
        1 => Category::WepT3,
        2 => Category::WepT4,
        _ => Category::WepT5,
    };
    let loot = draw_unique(
        dungeon,
        second_wep,
        loot_tier > 0,
        &mut pool.generated,
        |_| false,
    );
    loot_list.push(finish_weapon(loot, loot_tier as i32, loot_tier));

    let missile = match loot_tier {
        0 => Category::MisT2,
        1 => Category::MisT3,
        2 => Category::MisT4,
        _ => Category::MisT5,
    };
    let loot = draw_unique(dungeon, missile, true, &mut pool.generated, |_| false);
    loot_list.push(finish_weapon(loot, loot_tier as i32, loot_tier));

    let armor_name = match loot_tier {
        0 => "LeatherArmor",
        1 => "MailArmor",
        2 => "ScaleArmor",
        _ => "PlateArmor",
    };
    pool.generated.insert(armor_name.to_string());
    let mut armor = GeneratedItem::new(armor_name, ItemCategory::Armor);
    armor.level = loot_tier as i32;
    if Random::int_max(3) >= loot_tier as i32 {
        armor.enchantment = None;
    } else {
        armor.enchantment = Some(random_armor_glyph(armor.enchantment.as_deref()).to_string());
    }
    loot_list.push(armor);

    let wand = draw_unique(dungeon, Category::Wand, true, &mut pool.generated, |name| {
        BANNED_WANDS.contains(&name)
    });
    let mut wand = wand;
    wand.level = loot_tier as i32;
    loot_list.push(wand);

    let ring = draw_unique(dungeon, Category::Ring, true, &mut pool.generated, |name| {
        BANNED_RINGS.contains(&name)
    });
    let mut ring = ring;
    ring.level = loot_tier as i32;
    loot_list.push(ring);

    loot_list
}

fn draw_unique(
    dungeon: &mut DungeonState,
    cat: Category,
    reject_duplicate: bool,
    generated: &mut HashSet<String>,
    banned: impl Fn(&str) -> bool,
) -> GeneratedItem {
    loop {
        let item = dungeon.generator.random_using_defaults(cat, dungeon.depth);
        if (!reject_duplicate || !generated.contains(&item.class_name)) && !banned(&item.class_name)
        {
            generated.insert(item.class_name.clone());
            return item;
        }
    }
}

fn finish_weapon(mut loot: GeneratedItem, level: i32, loot_tier: usize) -> GeneratedItem {
    loot.level = level;
    if Random::int_max(3) >= loot_tier as i32 {
        loot.enchantment = None;
    } else {
        loot.enchantment = Some(random_weapon_enchant(loot.enchantment.as_deref()).to_string());
    }
    loot
}

fn setup_consumables(consumables: &mut [Vec<GeneratedItem>; 4]) {
    if consumables.iter().all(Vec::is_empty) {
        // lists exist; fill empty tiers below
    }
    if consumables[0].is_empty() {
        consumables[0] = vec![
            named(
                one_of(&["PotionOfFrost", "PotionOfLevitation"]),
                ItemCategory::Potion,
            ),
            named(
                one_of(&["Mageroyal$Seed", "Icecap$Seed", "Stormvine$Seed"]),
                ItemCategory::Seed,
            ),
            named(
                one_of(&["ScrollOfMirrorImage", "ScrollOfTeleportation"]),
                ItemCategory::Scroll,
            ),
            named(
                one_of(&["StoneOfFlock", "StoneOfShock", "StoneOfFear"]),
                ItemCategory::Stone,
            ),
        ];
        // JDK `Collections.shuffle` is unseeded; do not touch the watabou stream.
        consumables[0].insert(0, named("PotionOfHealing", ItemCategory::Potion));
    }
    if consumables[1].is_empty() {
        consumables[1] = vec![
            named(
                one_of(&["PotionOfToxicGas", "PotionOfParalyticGas"]),
                ItemCategory::Potion,
            ),
            named(
                one_of(&["Firebloom$Seed", "Sorrowmoss$Seed", "Blindweed$Seed"]),
                ItemCategory::Seed,
            ),
            named(
                one_of(&["ScrollOfRecharging", "ScrollOfTerror"]),
                ItemCategory::Scroll,
            ),
            named(
                one_of(&[
                    "StoneOfDeepSleep",
                    "StoneOfClairvoyance",
                    "StoneOfAggression",
                ]),
                ItemCategory::Stone,
            ),
        ];
        consumables[1].insert(0, named("PotionOfHealing", ItemCategory::Potion));
    }
    if consumables[2].is_empty() {
        consumables[2] = vec![
            named(
                one_of(&["PotionOfMindVision", "PotionOfLiquidFlame"]),
                ItemCategory::Potion,
            ),
            named(
                one_of(&["Swiftthistle$Seed", "Sungrass$Seed"]),
                ItemCategory::Seed,
            ),
            named(
                one_of(&["ScrollOfLullaby", "ScrollOfMagicMapping"]),
                ItemCategory::Scroll,
            ),
            named(
                one_of(&["StoneOfBlast", "StoneOfBlink"]),
                ItemCategory::Stone,
            ),
        ];
        consumables[2].insert(0, named("PotionOfHealing", ItemCategory::Potion));
    }
    if consumables[3].is_empty() {
        consumables[3] = vec![
            named(
                one_of(&["PotionOfExperience", "PotionOfInvisibility"]),
                ItemCategory::Potion,
            ),
            named(
                one_of(&["Earthroot$Seed", "Starflower$Seed"]),
                ItemCategory::Seed,
            ),
            named(
                one_of(&["ScrollOfRetribution", "ScrollOfTransmutation"]),
                ItemCategory::Scroll,
            ),
            named(
                one_of(&["StoneOfEnchantment", "StoneOfAugmentation"]),
                ItemCategory::Stone,
            ),
            named("PotionOfHealing", ItemCategory::Potion),
        ];
        // JDK shuffle of T3; watabou stream already consumed by oneOf.
    }
}

fn one_of(names: &[&str]) -> String {
    Random::one_of(names).to_string()
}

fn named(class_name: impl Into<String>, category: ItemCategory) -> GeneratedItem {
    GeneratedItem::new(class_name, category)
}
