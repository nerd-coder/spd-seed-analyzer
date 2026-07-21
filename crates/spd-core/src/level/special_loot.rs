//! Special / secret room prize generation (partial paint loot).
//!
//! Called after minimal geometry paint and **before** `createItems`, matching
//! RegularPainter room paint order (shuffle rooms → placeDoors RNG → room prizes).
//! Placement loops consume RNG; layout water/grass/traps still incomplete so
//! results remain approximate vs full game parity.

use crate::dungeon::DungeonState;
use crate::generator::Category;
use crate::geom::Point;
use crate::items::enchants;
use crate::items::model::{GeneratedItem, ItemCategory};
use crate::items::randomize::randomize_item;
use crate::level::create_items::PlacedLoot;
use crate::random::Random;
use crate::rooms::room::Room;
use crate::rooms::types::RoomKind;

/// Generate special/secret room prizes; may consume items from `items_to_spawn`
/// via `findPrizeItem` (TrinketCatalyst, forced potions, etc.).
pub fn special_room_loot(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    if rooms.is_empty() {
        return out;
    }

    let mut order: Vec<usize> = (0..rooms.len()).collect();
    Random::shuffle_vec(&mut order);

    // placeDoors-style RNG for each undirected connection (door cell pick).
    let mut doors_placed: Vec<(usize, usize)> = Vec::new();
    for &ri in &order {
        place_doors_rng(rooms, ri, &mut doors_placed);

        let room = &rooms[ri];
        if room.is_empty() {
            continue;
        }
        match room.kind {
            RoomKind::Special | RoomKind::Secret => {
                let mut loot = paint_special(dungeon, rooms, ri, items_to_spawn);
                out.append(&mut loot);
            }
            RoomKind::Standard
                if room.name == "RitualSiteRoom" || room.name == "BlacksmithRoom" =>
            {
                let mut loot = paint_special(dungeon, rooms, ri, items_to_spawn);
                out.append(&mut loot);
            }
            RoomKind::Shop => {
                // Shop contents deferred (FOR_SALE filter); still burn no RNG here.
            }
            _ => {}
        }
    }

    out
}

fn place_doors_rng(rooms: &[Room], ri: usize, doors_placed: &mut Vec<(usize, usize)>) {
    let room = &rooms[ri];
    for &ni in &room.connected {
        let a = ri.min(ni);
        let b = ri.max(ni);
        if doors_placed.contains(&(a, b)) {
            continue;
        }
        let other = &rooms[ni];
        if other.is_empty() {
            continue;
        }
        let spots = door_spots(room, other);
        if !spots.is_empty() {
            let _ = Random::element(&spots);
        }
        doors_placed.push((a, b));
    }
}

fn door_spots(a: &Room, b: &Room) -> Vec<Point> {
    let left = a.left.max(b.left);
    let right = a.right.min(b.right);
    let top = a.top.max(b.top);
    let bottom = a.bottom.min(b.bottom);
    let mut spots = Vec::new();
    for x in left..=right {
        for y in top..=bottom {
            let p = Point::new(x, y);
            if can_connect(a, p) && can_connect(b, p) {
                spots.push(p);
            }
        }
    }
    spots
}

fn can_connect(r: &Room, p: Point) -> bool {
    (p.x == r.left || p.x == r.right) != (p.y == r.top || p.y == r.bottom)
}

fn paint_special(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    ri: usize,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let room = &rooms[ri];
    let name = room.name.as_str();
    match name {
        "CryptRoom" => vec![crypt_prize(dungeon)],
        "ArmoryRoom" => armory_prizes(dungeon, room, items_to_spawn),
        "LibraryRoom" => library_prizes(dungeon, room, items_to_spawn),
        "TreasuryRoom" => treasury_prizes(dungeon, room, items_to_spawn),
        "PoolRoom" => vec![pool_prize(dungeon, room, items_to_spawn)],
        "StorageRoom" => storage_prizes(dungeon, room, items_to_spawn),
        "RunestoneRoom" => runestone_prizes(dungeon, room, items_to_spawn),
        "LaboratoryRoom" => laboratory_prizes(dungeon, room, items_to_spawn),
        "StatueRoom" => vec![statue_weapon(dungeon, room)],
        "SecretLibraryRoom" => secret_library(dungeon, room, items_to_spawn),
        "SecretRunestoneRoom" => secret_runestone(dungeon, room, items_to_spawn),
        "SecretArtilleryRoom" => secret_artillery(dungeon, room),
        "SecretLaboratoryRoom" => secret_laboratory(dungeon, room, items_to_spawn),
        "SecretLarderRoom" => secret_larder(dungeon, room),
        "SecretHoardRoom" => secret_hoard(dungeon, room),
        // Garden / well / pit / remaining secrets with portable prizes
        "GardenRoom" => garden_prizes(room, items_to_spawn),
        "SecretGardenRoom" => secret_garden_prizes(room),
        "MagicWellRoom" => magic_well(items_to_spawn),
        "SecretWellRoom" => secret_well(),
        "PitRoom" => pit_prizes(dungeon, items_to_spawn),
        "SecretMazeRoom" => vec![secret_maze_prize(dungeon)],
        "SecretSummoningRoom" => vec![secret_summoning_prize(dungeon)],
        "SecretChestChasmRoom" => secret_chest_chasm(dungeon, items_to_spawn),
        // Layout-only (no portable prize items)
        "WeakFloorRoom" | "DemonSpawnerRoom" => Vec::new(),
        "SentryRoom" => vec![sentry_prize(dungeon, items_to_spawn)],
        "TrapsRoom" => vec![traps_prize(dungeon, items_to_spawn)],
        "MagicalFireRoom" => magical_fire_prizes(dungeon, room, items_to_spawn),
        "SacrificeRoom" => vec![sacrifice_prize(dungeon, rooms, ri)],
        "ToxicGasRoom" => toxic_gas_prizes(dungeon, room, items_to_spawn),
        "SecretHoneypotRoom" => secret_honeypot(room),
        "CrystalVaultRoom" => crystal_vault(dungeon, rooms, ri, items_to_spawn),
        "CrystalChoiceRoom" => crystal_choice(dungeon, room, items_to_spawn),
        "CrystalPathRoom" => crystal_path(dungeon, items_to_spawn),
        // Wandmaker quest rooms
        "MassGraveRoom" => mass_grave_prizes(dungeon, room, items_to_spawn),
        "RitualSiteRoom" => ritual_site_setup(items_to_spawn),
        "RotGardenRoom" => rot_garden_setup(room, items_to_spawn),
        // Blacksmith quest room — two random equip drops + NPC / exit placement RNG
        "BlacksmithRoom" => blacksmith_room_prizes(dungeon, room),
        // Imp quest room — NPC offset burns IntRange(-1,1); no prize items
        "AmbitiousImpRoom" => {
            let _ = Random::int_range_inclusive(-1, 1);
            Vec::new()
        }
        _ => Vec::new(),
    }
}

/// `MassGraveRoom.paint` loot (approx placement; skeleton mobs skipped).
fn mass_grave_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Barricade entrance → PotionOfLiquidFlame into itemsToSpawn
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLiquidFlame",
        ItemCategory::Potion,
    ));

    // 50% 1 skeleton, 50% 2 — placement burns room.random() (terrain check approx)
    let n_skel = Random::int_max(2); // 0 or 1 → loop i=0..=n_skel → 1 or 2
    let mut occupied = Vec::new();
    for _ in 0..=n_skel {
        burn_drop_pos(room, &mut occupied);
    }

    let mut out = Vec::new();
    // 100% corpse dust, 2x gold(1), 2x30% gold, 1x60% random, 1x30% armor
    let mut items: Vec<GeneratedItem> = Vec::new();
    items.push(GeneratedItem::new("CorpseDust", ItemCategory::Other));
    {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        g.quantity = 1;
        items.push(g);
    }
    {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        g.quantity = 1;
        items.push(g);
    }
    if Random::float() <= 0.3 {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        randomize_item(&mut g, dungeon.depth);
        items.push(g);
    }
    if Random::float() <= 0.3 {
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        randomize_item(&mut g, dungeon.depth);
        items.push(g);
    }
    if Random::float() <= 0.6 {
        items.push(dungeon.generator.random(dungeon.depth));
    }
    if Random::float() <= 0.3 {
        items.push(
            dungeon
                .generator
                .random_armor(dungeon.depth / 5, dungeon.depth),
        );
    }

    for mut item in items {
        burn_drop_pos(room, &mut occupied);
        // Haunted-if-cursed: no extra RNG for analysis
        item.source = Some("MassGraveRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "skeleton",
        });
    }
    out
}

/// `RitualSiteRoom.paint` — four ceremonial candles into itemsToSpawn.
fn ritual_site_setup(items_to_spawn: &mut Vec<GeneratedItem>) -> Vec<PlacedLoot> {
    for _ in 0..4 {
        items_to_spawn.push(GeneratedItem::new("CeremonialCandle", ItemCategory::Other));
    }
    Vec::new()
}

/// `RotGardenRoom.paint` — locked door key + approximate wall-scatter RNG.
fn rot_garden_setup(room: &Room, items_to_spawn: &mut Vec<GeneratedItem>) -> Vec<PlacedLoot> {
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));

    // Chaotic wall placement: 12× random(1), 8× random(2), 4× random(3)
    for _ in 0..12 {
        let _ = room.random_margin(1);
    }
    for _ in 0..8 {
        let _ = room.random_margin(2);
    }
    for _ in 0..4 {
        let _ = room.random_margin(3);
    }
    // Full game retries until openCells threshold + heart/lasher placement —
    // not ported; further mob RNG deferred (may desync createItems slightly).
    let _ = room;
    Vec::new()
}

// --- crystal rooms ---

/// `CrystalVaultRoom.paint` — two crystal-chest prizes (WAND/RING/ARTIFACT rotate).
fn crystal_vault(
    dungeon: &mut DungeonState,
    rooms: &[Room],
    ri: usize,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let room = &rooms[ri];
    // prizeClasses rotate: shuffle then take/rotate twice
    let mut prize_classes = [Category::Wand, Category::Ring, Category::Artifact];
    Random::shuffle(&mut prize_classes);

    let mut take_prize = |classes: &mut [Category; 3]| -> GeneratedItem {
        let cat = classes[0];
        classes.rotate_left(1);
        // do { prize = Generator.random(cat) } while blocked — no challenges here
        dungeon.generator.random_category(cat, dungeon.depth)
    };

    let mut i1 = take_prize(&mut prize_classes);
    let mut i2 = take_prize(&mut prize_classes);
    i1.source = Some("CrystalVaultRoom".into());
    i2.source = Some("CrystalVaultRoom".into());

    // Pedestal placement: CIRCLE8 opposite pair, reject if adjacent to entrance door.
    burn_crystal_vault_positions(rooms, ri);

    // 10% crystal mimic on second chest (no RatSkull / MimicTooth trinkets).
    let second_heap = if Random::float() < 0.1 {
        "crystal_mimic"
    } else {
        "crystal_chest"
    };

    items_to_spawn.push(GeneratedItem::new("CrystalKey", ItemCategory::Other));
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));

    let _ = room; // keys/chests reported; geometry is approximate
    vec![
        PlacedLoot {
            item: i1,
            heap_type: "crystal_chest",
        },
        PlacedLoot {
            item: i2,
            heap_type: second_heap,
        },
    ]
}

/// PathFinder.CIRCLE8 clockwise offsets (dx, dy) for unit steps.
const CIRCLE8: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

fn burn_crystal_vault_positions(rooms: &[Room], ri: usize) {
    let room = &rooms[ri];
    if room.is_empty() {
        let _ = Random::int_max(8);
        return;
    }
    let cx = (room.left + room.right) / 2;
    let cy = (room.top + room.bottom) / 2;
    let door = vault_entrance_cell(rooms, ri).unwrap_or_else(|| Point::new(room.left, cy));

    // Match Java do-while: keep rolling until neither pedestal is adjacent to the door.
    for _ in 0..32 {
        let idx = Random::int_max(8) as usize;
        let (dx1, dy1) = CIRCLE8[idx];
        let (dx2, dy2) = CIRCLE8[(idx + 4) % 8];
        let i1 = Point::new(cx + dx1, cy + dy1);
        let i2 = Point::new(cx + dx2, cy + dy2);
        if !adjacent_4(i1, door) && !adjacent_4(i2, door) {
            return;
        }
    }
}

fn vault_entrance_cell(rooms: &[Room], ri: usize) -> Option<Point> {
    let room = &rooms[ri];
    let ni = *room.connected.first()?;
    let other = rooms.get(ni)?;
    if other.is_empty() {
        return None;
    }
    let spots = door_spots(room, other);
    // placeDoors already burned element(); door is geometric mid-edge — pick mid.
    if spots.is_empty() {
        None
    } else {
        Some(spots[spots.len() / 2])
    }
}

fn adjacent_4(a: Point, b: Point) -> bool {
    (a.x - b.x).abs() + (a.y - b.y).abs() == 1
}

/// `CrystalChoiceRoom.paint` — 3–4 potion/scroll piles + one chest (wand/ring/artifact).
fn crystal_choice(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // room1/room2 may swap
    let _ = Random::int_max(2);

    let n = Random::normal_int_range(3, 4);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        // room.random(1) or random(0) depending on square — burn margin-0 style
        burn_drop_pos_margin(room, 0, &mut occupied);
        let cat = *Random::one_of(&[Category::Potion, Category::Scroll]);
        let mut item = dungeon.generator.random_category(cat, dungeon.depth);
        item.source = Some("CrystalChoiceRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }

    let hidden_cat = *Random::one_of(&[Category::Wand, Category::Ring, Category::Artifact]);
    let mut hidden = dungeon.generator.random_category(hidden_cat, dungeon.depth);
    hidden.source = Some("CrystalChoiceRoom".into());
    out.push(PlacedLoot {
        item: hidden,
        heap_type: "chest",
    });

    items_to_spawn.push(GeneratedItem::new("CrystalKey", ItemCategory::Other));
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
    out
}

/// `CrystalPathRoom.paint` loot — 3 potions + 3 scrolls (deduped), XP/transmutation branch.
fn crystal_path(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut potions: Vec<GeneratedItem> = Vec::new();
    let mut scrolls: Vec<GeneratedItem> = Vec::new();
    let mut duplicates: Vec<GeneratedItem> = Vec::new();

    // ExoticCrystals trinket chance is 0 → always base XP / Transmutation.
    if Random::int_max(2) == 0 {
        add_reward_item(dungeon, Category::Potion, &mut potions, &mut duplicates);
        scrolls.push(GeneratedItem::new(
            "ScrollOfTransmutation",
            ItemCategory::Scroll,
        ));
    } else {
        potions.push(GeneratedItem::new(
            "PotionOfExperience",
            ItemCategory::Potion,
        ));
        add_reward_item(dungeon, Category::Scroll, &mut scrolls, &mut duplicates);
    }
    add_reward_item(dungeon, Category::Potion, &mut potions, &mut duplicates);
    add_reward_item(dungeon, Category::Scroll, &mut scrolls, &mut duplicates);
    add_reward_item(dungeon, Category::Potion, &mut potions, &mut duplicates);
    add_reward_item(dungeon, Category::Scroll, &mut scrolls, &mut duplicates);

    // Undo deck pulls from rejected duplicates (matches Generator.undoDrop).
    for d in &duplicates {
        dungeon.generator.undo_drop(&d.class_name);
    }

    // Sort by defaultProbsTotal descending (common first / rare last).
    sort_by_default_value(dungeon, Category::Potion, &mut potions);
    sort_by_default_value(dungeon, Category::Scroll, &mut scrolls);

    // Placement shuffle: rooms 2&3 / 0&1 / prizes — one Int(2).
    let _shuffle = Random::int_max(2);

    let mut out = Vec::new();
    // three potions + three scrolls in placement order (values still reported fully)
    for mut p in potions {
        p.source = Some("CrystalPathRoom".into());
        out.push(PlacedLoot {
            item: p,
            heap_type: "heap",
        });
    }
    for mut s in scrolls {
        s.source = Some("CrystalPathRoom".into());
        out.push(PlacedLoot {
            item: s,
            heap_type: "heap",
        });
    }

    for _ in 0..3 {
        items_to_spawn.push(GeneratedItem::new("CrystalKey", ItemCategory::Other));
    }
    out
}

fn add_reward_item(
    dungeon: &mut DungeonState,
    cat: Category,
    items: &mut Vec<GeneratedItem>,
    dupes: &mut Vec<GeneratedItem>,
) {
    loop {
        let reward = dungeon.generator.random_category(cat, dungeon.depth);
        let dupe = items.iter().any(|i| i.class_name == reward.class_name);
        if dupe {
            dupes.push(reward);
        } else {
            items.push(reward);
            return;
        }
    }
}

fn sort_by_default_value(dungeon: &DungeonState, cat: Category, items: &mut [GeneratedItem]) {
    items.sort_by(|a, b| {
        let av = dungeon.generator.default_prob_total(cat, &a.class_name);
        let bv = dungeon.generator.default_prob_total(cat, &b.class_name);
        // Java: return bVal - aVal → higher total first
        bv.partial_cmp(&av).unwrap_or(std::cmp::Ordering::Equal)
    });
}

/// `BlacksmithRoom.paint` — two random armor/weapon/missile drops + NPC/exit placement RNG.
fn blacksmith_room_prizes(dungeon: &mut DungeonState, room: &Room) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..2 {
        // drop on EMPTY_SP: room.random() until free
        burn_drop_pos(room, &mut occupied);
        let cat = *Random::one_of(&[Category::Armor, Category::Weapon, Category::Missile]);
        let mut item = dungeon.generator.random_category(cat, dungeon.depth);
        item.source = Some("BlacksmithRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    // NPC pos: random(2) until free of heaps
    burn_drop_pos_margin(room, 2, &mut occupied);
    // entrancePos: random(2) until free of heaps and NPC
    burn_drop_pos_margin(room, 2, &mut occupied);
    out
}

// --- prize helpers ---

fn crypt_prize(dungeon: &mut DungeonState) -> PlacedLoot {
    // Generator.randomArmor((depth/5)+1)
    let mut prize = dungeon
        .generator
        .random_armor((dungeon.depth / 5) + 1, dungeon.depth);
    // always roll a curse glyph (parchment scrap isolation)
    let curse = enchants::random_armor_curse(None).to_string();
    if !prize.cursed {
        prize.level += 1;
        if !is_good_glyph(&prize) {
            prize.enchantment = Some(curse);
        }
    }
    prize.cursed = true;
    prize.source = Some("CryptRoom".into());
    PlacedLoot {
        item: prize,
        heap_type: "tomb",
    }
}

fn is_good_glyph(item: &GeneratedItem) -> bool {
    match item.enchantment.as_deref() {
        Some(e) => !matches!(
            e,
            "AntiEntropy"
                | "Corrosion"
                | "Displacement"
                | "Metabolism"
                | "Multiplicity"
                | "Stench"
                | "Overgrowth"
                | "Bulk"
        ),
        None => false,
    }
}

fn is_curse_enchant(item: &GeneratedItem) -> bool {
    match item.enchantment.as_deref() {
        Some(e) => matches!(
            e,
            "Annoying"
                | "Displacing"
                | "Dazzling"
                | "Explosive"
                | "Sacrificial"
                | "Wayward"
                | "Polarized"
                | "Friendly"
                | "AntiEntropy"
                | "Corrosion"
                | "Displacement"
                | "Metabolism"
                | "Multiplicity"
                | "Stench"
                | "Overgrowth"
                | "Bulk"
        ),
        None => false,
    }
}

fn armory_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    // statue position Random.Int(2)
    let _ = Random::int_max(2);

    let n = Random::int_range_inclusive(2, 3);
    let mut prize_cats = [1.0f32, 1.0, 1.0, 1.0];
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let index = Random::chances(&prize_cats).max(0) as usize;
        prize_cats[index] = 0.0;
        let mut item = match index {
            0 => bomb_random(),
            1 => dungeon
                .generator
                .random_weapon(dungeon.depth / 5, false, dungeon.depth),
            2 => dungeon
                .generator
                .random_armor(dungeon.depth / 5, dungeon.depth),
            _ => dungeon
                .generator
                .random_missile(dungeon.depth / 5, false, dungeon.depth),
        };
        item.source = Some("ArmoryRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }

    if let Some(mut cata) = find_prize_item(items_to_spawn, Some("TrinketCatalyst")) {
        burn_drop_pos(room, &mut occupied);
        cata.source = Some("ArmoryRoom".into());
        out.push(PlacedLoot {
            item: cata,
            heap_type: "heap",
        });
    }
    out
}

fn bomb_random() -> GeneratedItem {
    // Bomb.random: 1/4 DoubleBomb else Bomb
    if Random::int_max(4) == 0 {
        let mut b = GeneratedItem::new("DoubleBomb", ItemCategory::Other);
        b.quantity = 2;
        b
    } else {
        GeneratedItem::new("Bomb", ItemCategory::Other)
    }
}

fn library_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    let n = Random::normal_int_range(1, 3);
    let mut occupied = Vec::new();
    for i in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = if i == 0 {
            if Random::int_max(2) == 0 {
                GeneratedItem::new("ScrollOfIdentify", ItemCategory::Scroll)
            } else {
                GeneratedItem::new("ScrollOfRemoveCurse", ItemCategory::Scroll)
            }
        } else {
            library_prize(dungeon, items_to_spawn)
        };
        item.source = Some("LibraryRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

fn library_prize(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> GeneratedItem {
    if let Some(cata) = find_prize_item(items_to_spawn, Some("TrinketCatalyst")) {
        return cata;
    }
    if let Some(scroll) = find_prize_item_category(items_to_spawn, ItemCategory::Scroll) {
        return scroll;
    }
    dungeon
        .generator
        .random_category(Category::Scroll, dungeon.depth)
}

fn treasury_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    // center statue — no RNG
    let heap_chest = Random::int_max(2) == 0;
    let n = Random::int_range_inclusive(2, 3);
    let mimic_chance = 0.2f32; // 1/5 without MimicTooth
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item =
            find_prize_item(items_to_spawn, Some("TrinketCatalyst")).unwrap_or_else(|| {
                let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
                randomize_item(&mut g, dungeon.depth);
                g
            });
        let heap_type = if heap_chest && dungeon.depth > 1 && Random::float() < mimic_chance {
            item.source = Some("TreasuryRoom:mimic".into());
            "mimic"
        } else {
            item.source = Some("TreasuryRoom".into());
            if heap_chest {
                "chest"
            } else {
                "heap"
            }
        };
        out.push(PlacedLoot { item, heap_type });
    }
    if !heap_chest {
        for _ in 0..6 {
            let _ = Random::int_range_inclusive(room.left + 1, room.right - 1);
            let _ = Random::int_range_inclusive(room.top + 1, room.bottom - 1);
            // small gold piles blacklisted from report — still burn quantity RNG
            let _qty = Random::int_range_inclusive(5, 12);
        }
    }
    out
}

fn pool_prize(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> PlacedLoot {
    // pedestal position is geometric; prize first
    let mut prize = if Random::int_max(3) == 0 {
        find_prize_item(items_to_spawn, None).unwrap_or_else(|| pool_equip(dungeon))
    } else {
        pool_equip(dungeon)
    };
    prize.cursed = false;
    if is_curse_enchant(&prize) {
        prize.enchantment = None;
    }
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize.source = Some("PoolRoom".into());

    // piranha placement burns RNG (3 piranhas)
    for _ in 0..3 {
        burn_terrain_pos(room, /*water-like*/ true);
    }

    PlacedLoot {
        item: prize,
        heap_type: "chest",
    }
}

fn pool_equip(dungeon: &mut DungeonState) -> GeneratedItem {
    let floor = (dungeon.depth / 5) + 1;
    match Random::int_max(5) {
        0 | 1 => dungeon.generator.random_weapon(floor, false, dungeon.depth),
        2 => dungeon
            .generator
            .random_missile(floor, false, dungeon.depth),
        _ => dungeon.generator.random_armor(floor, dungeon.depth),
    }
}

fn storage_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    let mut honey = Random::int_max(2) == 0;
    let n = Random::int_range_inclusive(3, 4);
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = if honey {
            honey = false;
            GeneratedItem::new("Honeypot", ItemCategory::Other)
        } else {
            storage_prize(dungeon, items_to_spawn)
        };
        item.source = Some("StorageRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    // addItemToSpawn PotionOfLiquidFlame — deferred to createItems path in full game;
    // we push into items_to_spawn so it can land as forced.
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLiquidFlame",
        ItemCategory::Potion,
    ));
    out
}

fn storage_prize(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> GeneratedItem {
    if Random::int_max(3) != 0 {
        if let Some(p) = find_prize_item(items_to_spawn, None) {
            return p;
        }
    }
    let cat = *Random::one_of(&[
        Category::Potion,
        Category::Scroll,
        Category::Food,
        Category::Gold,
    ]);
    dungeon.generator.random_category(cat, dungeon.depth)
}

fn runestone_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    let n = Random::normal_int_range(2, 3);
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = if let Some(c) = find_prize_item(items_to_spawn, Some("TrinketCatalyst")) {
            c
        } else if let Some(s) = find_prize_item_category(items_to_spawn, ItemCategory::Stone) {
            s
        } else {
            dungeon
                .generator
                .random_category(Category::Stone, dungeon.depth)
        };
        item.source = Some("RunestoneRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

fn laboratory_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    // pot position Random.Int(2)
    let _ = Random::int_max(2);

    let mut occupied = Vec::new();
    burn_drop_pos(room, &mut occupied);
    // EnergyCrystal x5 — blacklisted elsewhere? report as lab energy
    let mut crystal = GeneratedItem::new("EnergyCrystal", ItemCategory::Other);
    crystal.quantity = 5;
    crystal.source = Some("LaboratoryRoom".into());
    // blacklisted in is_blacklisted — skip reporting energy
    let _ = crystal;

    let n = Random::normal_int_range(1, 2);
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = if let Some(c) = find_prize_item(items_to_spawn, Some("TrinketCatalyst")) {
            c
        } else if let Some(p) = find_prize_item(items_to_spawn, Some("PotionOfStrength")) {
            p
        } else {
            let cat = *Random::one_of(&[Category::Potion, Category::Stone]);
            dungeon.generator.random_category(cat, dungeon.depth)
        };
        item.source = Some("LaboratoryRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    // alchemy guide pages — skip (document state not tracked)
    out
}

fn statue_weapon(dungeon: &mut DungeonState, _room: &Room) -> PlacedLoot {
    // Statue.random: 10% armored (rat skull default)
    let _armored = Random::float() < 0.1;
    let mut weapon = dungeon
        .generator
        .random_category(Category::Weapon, dungeon.depth);
    weapon.cursed = false;
    weapon.enchantment = Some(enchants::random_weapon_enchant(None).to_string());
    weapon.source = Some("StatueRoom".into());
    PlacedLoot {
        item: weapon,
        heap_type: "statue",
    }
}

fn secret_library(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Approximate: 2-3 scrolls
    let n = Random::int_range_inclusive(2, 3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item =
            find_prize_item(items_to_spawn, Some("TrinketCatalyst")).unwrap_or_else(|| {
                dungeon
                    .generator
                    .random_category(Category::Scroll, dungeon.depth)
            });
        item.source = Some("SecretLibraryRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

fn secret_runestone(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let n = Random::int_range_inclusive(2, 3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item =
            find_prize_item(items_to_spawn, Some("TrinketCatalyst")).unwrap_or_else(|| {
                dungeon
                    .generator
                    .random_category(Category::Stone, dungeon.depth)
            });
        item.source = Some("SecretRunestoneRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

fn secret_artillery(dungeon: &mut DungeonState, room: &Room) -> Vec<PlacedLoot> {
    let n = Random::int_range_inclusive(2, 3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = dungeon
            .generator
            .random_missile(dungeon.depth / 5, false, dungeon.depth);
        item.source = Some("SecretArtilleryRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

fn secret_laboratory(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    laboratory_prizes(dungeon, room, items_to_spawn)
        .into_iter()
        .map(|mut p| {
            p.item.source = Some("SecretLaboratoryRoom".into());
            p
        })
        .collect()
}

fn secret_larder(dungeon: &mut DungeonState, room: &Room) -> Vec<PlacedLoot> {
    let n = Random::int_range_inclusive(2, 3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut item = dungeon
            .generator
            .random_category(Category::Food, dungeon.depth);
        item.source = Some("SecretLarderRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    out
}

fn secret_hoard(dungeon: &mut DungeonState, room: &Room) -> Vec<PlacedLoot> {
    // Approximate: gold piles (blacklisted from report — still burn RNG)
    let n = Random::int_range_inclusive(3, 5);
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos(room, &mut occupied);
        let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
        randomize_item(&mut g, dungeon.depth);
        g.source = Some("SecretHoardRoom".into());
        let _ = g;
    }
    Vec::new()
}

// --- garden / well / pit / maze / summoning / chest-chasm ---

/// `GardenRoom.paint` — IronKey + 0–2 plant seeds (Sungrass / Blandfruit).
fn garden_prizes(room: &Room, items_to_spawn: &mut Vec<GeneratedItem>) -> Vec<PlacedLoot> {
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));

    let bushes = Random::int_max(3);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    match bushes {
        0 => {
            burn_drop_pos(room, &mut occupied);
            out.push(plant_loot("SungrassSeed", "GardenRoom"));
        }
        1 => {
            burn_drop_pos(room, &mut occupied);
            out.push(plant_loot("BlandfruitBushSeed", "GardenRoom"));
        }
        _ => {
            // 20% both seeds
            if Random::int_max(5) == 0 {
                burn_drop_pos(room, &mut occupied);
                out.push(plant_loot("SungrassSeed", "GardenRoom"));
                burn_drop_pos(room, &mut occupied);
                out.push(plant_loot("BlandfruitBushSeed", "GardenRoom"));
            }
        }
    }
    out
}

/// `SecretGardenRoom.paint` — Starflower + Seedpod + Dewcatcher + 50% extra.
/// `Patch.generate` grass layout is burned for RNG stream parity (approx plant cells).
fn secret_garden_prizes(room: &Room) -> Vec<PlacedLoot> {
    // Patch.generate(w-2, h-2, 0.5, clustering=0, forceFillRate=true)
    let pw = (room.width() - 2).max(0);
    let ph = (room.height() - 2).max(0);
    burn_patch_generate(pw, ph, 0.5, 0, true);

    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for class in ["StarflowerSeed", "SeedpodSeed", "DewcatcherSeed"] {
        burn_drop_pos(room, &mut occupied);
        out.push(plant_loot(class, "SecretGardenRoom"));
    }
    burn_drop_pos(room, &mut occupied);
    let extra = if Random::int_max(2) == 0 {
        "SeedpodSeed"
    } else {
        "DewcatcherSeed"
    };
    out.push(plant_loot(extra, "SecretGardenRoom"));
    out
}

fn plant_loot(class_name: &str, source: &str) -> PlacedLoot {
    let mut item = GeneratedItem::new(class_name, ItemCategory::Seed);
    item.source = Some(source.into());
    PlacedLoot {
        item,
        heap_type: "plant",
    }
}

/// Minimal port of `Patch.generate` RNG consumption (for secret-garden parity).
fn burn_patch_generate(w: i32, h: i32, fill: f32, clustering: i32, force_fill_rate: bool) {
    if w <= 0 || h <= 0 {
        return;
    }
    let length = (w * h) as usize;
    let mut fill = fill;
    let mut fill_diff = -((length as f32 * fill).round() as i32);

    if force_fill_rate && clustering > 0 {
        fill += (0.5 - fill) * 0.5;
    }

    let mut off = vec![false; length];
    let mut cur = vec![false; length];
    for cell in &mut off {
        *cell = Random::float() < fill;
        if *cell {
            fill_diff += 1;
        }
    }

    for _ in 0..clustering {
        for y in 0..h {
            for x in 0..w {
                let pos = (x + y * w) as usize;
                let mut count = 0i32;
                let mut neighbours = 0i32;
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && nx < w && ny >= 0 && ny < h {
                            neighbours += 1;
                            if off[(nx + ny * w) as usize] {
                                count += 1;
                            }
                        }
                    }
                }
                cur[pos] = 2 * count >= neighbours;
                if cur[pos] != off[pos] {
                    fill_diff += if cur[pos] { 1 } else { -1 };
                }
            }
        }
        std::mem::swap(&mut cur, &mut off);
    }

    // force fill-rate adjustment (uses Random while fillDiff != 0)
    if force_fill_rate && w.min(h) > 2 {
        let growing = fill_diff < 0;
        // Cap iterations so pathological rooms never hang the analyzer.
        let mut guard = length * 20 + 64;
        while fill_diff != 0 && guard > 0 {
            guard -= 1;
            let mut tries = 0;
            // random interior cell; try length/10 times to match growing state
            // Java: Random.Int(1, w-1) exclusive upper → [1, w-2]
            let cell = loop {
                let cx = Random::int_range_inclusive(1, w - 2);
                let cy = Random::int_range_inclusive(1, h - 2);
                let c = (cx + cy * w) as usize;
                tries += 1;
                if off[c] == growing || tries * 10 >= length as i32 {
                    break c;
                }
            };
            // 3×3 neighbourhood around cell (including self)
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    if fill_diff == 0 {
                        break;
                    }
                    let nx = (cell as i32 % w) + dx;
                    let ny = (cell as i32 / w) + dy;
                    if nx < 0 || nx >= w || ny < 0 || ny >= h {
                        continue;
                    }
                    let npos = (nx + ny * w) as usize;
                    if off[npos] != growing {
                        off[npos] = growing;
                        fill_diff += if growing { 1 } else { -1 };
                    }
                }
            }
        }
    }
}

/// `MagicWellRoom.paint` — locked IronKey + Awareness/Health well type.
fn magic_well(items_to_spawn: &mut Vec<GeneratedItem>) -> Vec<PlacedLoot> {
    items_to_spawn.push(GeneratedItem::new("IronKey", ItemCategory::Other));
    secret_well_type("MagicWellRoom")
}

/// `SecretWellRoom.paint` — Awareness/Health well (no key).
fn secret_well() -> Vec<PlacedLoot> {
    secret_well_type("SecretWellRoom")
}

fn secret_well_type(source: &str) -> Vec<PlacedLoot> {
    let waters = ["WaterOfAwareness", "WaterOfHealth"];
    let class = *Random::one_of(&waters);
    let mut item = GeneratedItem::new(class, ItemCategory::Other);
    item.source = Some(source.into());
    vec![PlacedLoot {
        item,
        heap_type: "well",
    }]
}

/// `PitRoom.paint` — skeleton main loot (ring/artifact/equip) + 1–2 consumables + CrystalKey.
fn pit_prizes(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Well corner: Random.Int(2) (door side is geometric from entrance).
    let _ = Random::int_max(2);

    // Main loot: ring / artifact / equip (weapon×2, missile, armor×2).
    // Challenges.isItemBlocked is always false without challenges — single draw.
    let mut main = match Random::int_max(3) {
        0 => dungeon
            .generator
            .random_category(Category::Ring, dungeon.depth),
        1 => dungeon
            .generator
            .random_category(Category::Artifact, dungeon.depth),
        _ => {
            let cats = [
                Category::Weapon,
                Category::Weapon,
                Category::Missile,
                Category::Armor,
                Category::Armor,
            ];
            let cat = *Random::one_of(&cats);
            dungeon.generator.random_category(cat, dungeon.depth)
        }
    };
    main.source = Some("PitRoom".into());
    let mut out = vec![PlacedLoot {
        item: main,
        heap_type: "skeleton",
    }];

    let n = Random::int_range_inclusive(1, 2);
    for _ in 0..n {
        let cats = [
            Category::Potion,
            Category::Scroll,
            Category::Food,
            Category::Gold,
        ];
        let cat = *Random::one_of(&cats);
        let mut prize = dungeon.generator.random_category(cat, dungeon.depth);
        prize.source = Some("PitRoom".into());
        out.push(PlacedLoot {
            item: prize,
            heap_type: "skeleton",
        });
    }

    items_to_spawn.push(GeneratedItem::new("CrystalKey", ItemCategory::Other));
    out
}

/// `SecretMazeRoom.paint` prize — +1 floor-set weapon/armor, never cursed, 33% upgrade.
/// Maze layout RNG is not fully ported (PathFinder distance pick skipped) — prize stream is approximate.
fn secret_maze_prize(dungeon: &mut DungeonState) -> PlacedLoot {
    let floor = (dungeon.depth / 5) + 1;
    let mut prize = if Random::int_max(2) == 0 {
        let mut w = dungeon.generator.random_weapon(floor, true, dungeon.depth);
        if is_curse_enchant(&w) {
            w.enchantment = None;
        }
        w
    } else {
        let mut a = dungeon.generator.random_armor(floor, dungeon.depth);
        if is_curse_enchant(&a) {
            a.enchantment = None;
        }
        a
    };
    prize.cursed = false;
    // cursedKnown = true is UI-only in full game
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize.source = Some("SecretMazeRoom".into());
    PlacedLoot {
        item: prize,
        heap_type: "chest",
    }
}

/// `SecretSummoningRoom.paint` — center skeleton with `Generator.random()`.
fn secret_summoning_prize(dungeon: &mut DungeonState) -> PlacedLoot {
    // Trap reveal chance is 0 without TrapMechanism trinket — no extra RNG in trap loop.
    let mut item = dungeon.generator.random(dungeon.depth);
    item.source = Some("SecretSummoningRoom".into());
    PlacedLoot {
        item,
        heap_type: "skeleton",
    }
}

/// `SecretChestChasmRoom.paint` — 4 locked chests (`randomUsingDefaults`) + golden keys + levitation.
fn secret_chest_chasm(
    dungeon: &mut DungeonState,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    let mut out = Vec::new();
    // Geometry is fixed; four locked chests always drop (heaps non-null without blocked items).
    for _ in 0..4 {
        let mut item = dungeon.generator.random_using_defaults_any(dungeon.depth);
        item.source = Some("SecretChestChasmRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "locked_chest",
        });
    }
    // Four golden keys for the four chests (reported so seed-finder can see key count)
    for _ in 0..4 {
        let mut key = GeneratedItem::new("GoldenKey", ItemCategory::Other);
        key.source = Some("SecretChestChasmRoom".into());
        out.push(PlacedLoot {
            item: key,
            heap_type: "heap",
        });
    }
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLevitation",
        ItemCategory::Potion,
    ));
    out
}

// --- sentry / traps / fire / sacrifice / toxic gas / honeypot ---

/// `SentryRoom.paint` prize — chest equip or findPrizeItem + PotionOfHaste.
fn sentry_prize(dungeon: &mut DungeonState, items_to_spawn: &mut Vec<GeneratedItem>) -> PlacedLoot {
    // Layout (center/sentry/treasure) is geometric from entrance — no RNG before prize.
    let mut prize = if Random::int_max(2) == 0 {
        find_prize_item(items_to_spawn, None).unwrap_or_else(|| sentry_equip(dungeon))
    } else {
        sentry_equip(dungeon)
    };
    prize.cursed = false;
    if is_curse_enchant(&prize) {
        prize.enchantment = None;
    }
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize.source = Some("SentryRoom".into());
    items_to_spawn.push(GeneratedItem::new("PotionOfHaste", ItemCategory::Potion));
    PlacedLoot {
        item: prize,
        heap_type: "chest",
    }
}

fn sentry_equip(dungeon: &mut DungeonState) -> GeneratedItem {
    let floor = (dungeon.depth / 5) + 1;
    // Random.Int(5): 0,1 weapon; 2 missile; 3,4 armor
    match Random::int_max(5) {
        0 | 1 => dungeon.generator.random_weapon(floor, false, dungeon.depth),
        2 => dungeon
            .generator
            .random_missile(floor, false, dungeon.depth),
        _ => dungeon.generator.random_armor(floor, dungeon.depth),
    }
}

/// `TrapsRoom.paint` — trap class RNG then chest prize + PotionOfLevitation.
fn traps_prize(dungeon: &mut DungeonState, items_to_spawn: &mut Vec<GeneratedItem>) -> PlacedLoot {
    // Trap class selection (layout only; no trap instances in report).
    // Int(4)==0 → chasm (null traps); else oneOf(levelTraps[depth/5]).
    if Random::int_max(4) != 0 {
        let region = (dungeon.depth / 5).clamp(0, 4) as usize;
        let n = TRAPS_ROOM_TRAP_COUNTS[region];
        let _ = Random::int_max(n);
    }

    // Pedestal vs free chest: Random.Int(3) == 0 skips pedestal (geometry only).
    let _ = Random::int_max(3);

    let mut prize = if Random::int_max(3) != 0 {
        find_prize_item(items_to_spawn, None).unwrap_or_else(|| traps_equip(dungeon))
    } else {
        traps_equip(dungeon)
    };
    prize.cursed = false;
    if is_curse_enchant(&prize) {
        prize.enchantment = None;
    }
    if Random::int_max(3) == 0 {
        prize.level += 1;
    }
    prize.source = Some("TrapsRoom".into());
    items_to_spawn.push(GeneratedItem::new(
        "PotionOfLevitation",
        ItemCategory::Potion,
    ));
    PlacedLoot {
        item: prize,
        heap_type: "chest",
    }
}

/// Counts for `TrapsRoom.levelTraps` oneOf per region (sewers…halls).
const TRAPS_ROOM_TRAP_COUNTS: [i32; 5] = [3, 3, 3, 3, 1];

fn traps_equip(dungeon: &mut DungeonState) -> GeneratedItem {
    let floor = (dungeon.depth / 5) + 1;
    if Random::int_max(2) == 0 {
        dungeon.generator.random_weapon(floor, false, dungeon.depth)
    } else {
        dungeon.generator.random_armor(floor, dungeon.depth)
    }
}

/// `MagicalFireRoom.paint` — 3–4 honeypot/consumable drops + PotionOfFrost.
fn magical_fire_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Fire wall geometry from entrance — no RNG. behindFire drops use random(0).
    let mut honey = Random::int_max(2) == 0;
    let n = Random::int_range_inclusive(3, 4);
    let mut out = Vec::new();
    let mut occupied = Vec::new();
    for _ in 0..n {
        burn_drop_pos_margin(room, 0, &mut occupied);
        let mut item = if honey {
            honey = false;
            GeneratedItem::new("Honeypot", ItemCategory::Other)
        } else {
            // Same prize table as StorageRoom / MagicalFireRoom.prize
            storage_prize(dungeon, items_to_spawn)
        };
        item.source = Some("MagicalFireRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "heap",
        });
    }
    items_to_spawn.push(GeneratedItem::new("PotionOfFrost", ItemCategory::Potion));
    out
}

/// `SacrificeRoom.paint` — cursed upgraded weapon on sacrificial fire.
fn sacrifice_prize(dungeon: &mut DungeonState, rooms: &[Room], ri: usize) -> PlacedLoot {
    // Center offset when door is mid-wall aligned with room center.
    burn_sacrifice_center_offset(rooms, ri);

    // 1 floor set higher than normal
    let mut prize = dungeon
        .generator
        .random_weapon((dungeon.depth / 5) + 1, false, dungeon.depth);

    // Always generate curse (parchment scrap isolation), matching CryptRoom pattern.
    let curse = enchants::random_weapon_curse(None).to_string();
    if !prize.cursed {
        prize.level += 1;
        if !is_good_weapon_enchant(&prize) {
            prize.enchantment = Some(curse);
        }
    }
    prize.cursed = true;
    prize.source = Some("SacrificeRoom".into());
    PlacedLoot {
        item: prize,
        heap_type: "sacrificial",
    }
}

fn is_good_weapon_enchant(item: &GeneratedItem) -> bool {
    match item.enchantment.as_deref() {
        Some(e) => !matches!(
            e,
            "Annoying"
                | "Displacing"
                | "Dazzling"
                | "Explosive"
                | "Sacrificial"
                | "Wayward"
                | "Polarized"
                | "Friendly"
        ),
        None => false,
    }
}

/// Burn `Random.Int(2)` center nudge when entrance is mid-edge (SacrificeRoom).
fn burn_sacrifice_center_offset(rooms: &[Room], ri: usize) {
    let room = &rooms[ri];
    if room.is_empty() {
        return;
    }
    let c = Point::new((room.left + room.right) / 2, (room.top + room.bottom) / 2);
    let Some(door) = vault_entrance_cell(rooms, ri) else {
        return;
    };
    let side_door = (door.x == room.left || door.x == room.right) && door.y == c.y;
    let end_door = (door.y == room.top || door.y == room.bottom) && door.x == c.x;
    if side_door || end_door {
        let _ = Random::int_max(2);
    }
}

/// `ToxicGasRoom.paint` — skeleton 2×gold + 2 chests (cata/gold) + trap placement RNG.
fn toxic_gas_prizes(
    dungeon: &mut DungeonState,
    room: &Room,
    items_to_spawn: &mut Vec<GeneratedItem>,
) -> Vec<PlacedLoot> {
    // Vent traps: min(w-2, h-2) placements at random(2) on EMPTY (approx unique).
    let traps = (room.width() - 2).min(room.height() - 2).max(0);
    let mut occupied = Vec::new();
    for _ in 0..traps {
        burn_drop_pos_margin(room, 2, &mut occupied);
    }

    // 8 candidate gold positions at random(2); furthest becomes skeleton (trueDistance
    // pick is pure geometry — no extra RNG).
    for _ in 0..8 {
        burn_drop_pos_margin(room, 2, &mut occupied);
    }

    let mut out = Vec::new();

    // Main gold ×2 on skeleton heap (blacklisted from report; still for RNG parity).
    let mut main = GeneratedItem::new("Gold", ItemCategory::Gold);
    randomize_item(&mut main, dungeon.depth);
    main.quantity = main.quantity.saturating_mul(2);
    main.source = Some("ToxicGasRoom".into());
    out.push(PlacedLoot {
        item: main,
        heap_type: "skeleton",
    });

    // Two chests: TrinketCatalyst prize item or random gold
    for _ in 0..2 {
        let mut item =
            find_prize_item(items_to_spawn, Some("TrinketCatalyst")).unwrap_or_else(|| {
                let mut g = GeneratedItem::new("Gold", ItemCategory::Gold);
                randomize_item(&mut g, dungeon.depth);
                g
            });
        item.source = Some("ToxicGasRoom".into());
        out.push(PlacedLoot {
            item,
            heap_type: "chest",
        });
    }

    items_to_spawn.push(GeneratedItem::new("PotionOfPurity", ItemCategory::Potion));
    out
}

/// `SecretHoneypotRoom.paint` — shattered pot (geom) + honeypot + Bomb.random().
fn secret_honeypot(room: &Room) -> Vec<PlacedLoot> {
    // brokenPotPos is geometric midpoint of center and entrance — no RNG.
    // Bee spawn does not consume loot RNG.
    let mut out = Vec::new();
    let mut occupied = Vec::new();

    // Shattered pot reported as Honeypot.ShatteredPot for identity
    let mut shattered = GeneratedItem::new("ShatteredPot", ItemCategory::Other);
    shattered.source = Some("SecretHoneypotRoom".into());
    out.push(PlacedLoot {
        item: shattered,
        heap_type: "heap",
    });

    burn_drop_pos(room, &mut occupied);
    let mut honey = GeneratedItem::new("Honeypot", ItemCategory::Other);
    honey.source = Some("SecretHoneypotRoom".into());
    out.push(PlacedLoot {
        item: honey,
        heap_type: "heap",
    });

    burn_drop_pos(room, &mut occupied);
    let mut bomb = bomb_random();
    bomb.source = Some("SecretHoneypotRoom".into());
    out.push(PlacedLoot {
        item: bomb,
        heap_type: "heap",
    });

    out
}

/// Consume Room.random()-style placement until unique cell (cap tries).
fn burn_drop_pos(room: &Room, occupied: &mut Vec<(i32, i32)>) {
    burn_drop_pos_margin(room, 1, occupied);
}

/// `Room.random(m)` placement until unique cell.
fn burn_drop_pos_margin(room: &Room, m: i32, occupied: &mut Vec<(i32, i32)>) {
    if room.width() <= 2 * m || room.height() <= 2 * m {
        return;
    }
    for _ in 0..50 {
        let x = Random::int_range_inclusive(room.left + m, room.right - m);
        let y = Random::int_range_inclusive(room.top + m, room.bottom - m);
        if !occupied.contains(&(x, y)) {
            occupied.push((x, y));
            return;
        }
    }
}

fn burn_terrain_pos(room: &Room, _water: bool) {
    // simplified: consume the same RNG shape as a single accepted roll
    // (full map terrain not painted yet)
    let _x = Random::int_range_inclusive(room.left + 1, room.right - 1);
    let _y = Random::int_range_inclusive(room.top + 1, room.bottom - 1);
}

fn find_prize_item(
    items_to_spawn: &mut Vec<GeneratedItem>,
    match_class: Option<&str>,
) -> Option<GeneratedItem> {
    if items_to_spawn.is_empty() {
        return None;
    }
    match match_class {
        None => {
            // prefer TrinketCatalyst
            if let Some(i) = items_to_spawn
                .iter()
                .position(|it| it.class_name == "TrinketCatalyst")
            {
                return Some(items_to_spawn.remove(i));
            }
            let idx = Random::int_max(items_to_spawn.len() as i32) as usize;
            Some(items_to_spawn.remove(idx))
        }
        Some(class) => items_to_spawn
            .iter()
            .position(|it| it.class_name == class)
            .map(|i| items_to_spawn.remove(i)),
    }
}

fn find_prize_item_category(
    items_to_spawn: &mut Vec<GeneratedItem>,
    cat: ItemCategory,
) -> Option<GeneratedItem> {
    items_to_spawn
        .iter()
        .position(|it| it.category == cat)
        .map(|i| items_to_spawn.remove(i))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rooms::types::RoomKind;
    use crate::run::{dungeon_from_run, init_run};

    fn test_room(name: &str, w: i32, h: i32) -> Room {
        let mut r = Room::new(0, name, RoomKind::Special, 1, 3, w, w + 2, h, h + 2);
        r.left = 0;
        r.top = 0;
        r.right = w;
        r.bottom = h;
        r
    }

    #[test]
    fn sacrifice_prize_is_cursed_weapon() {
        Random::reset_generators();
        let run = init_run(42);
        Random::push_generator_seeded(12345);
        let mut d = dungeon_from_run(run);
        d.depth = 6;
        let room = test_room("SacrificeRoom", 7, 7);
        let loot = sacrifice_prize(&mut d, &[room], 0);
        Random::pop_generator();

        assert_eq!(loot.item.source.as_deref(), Some("SacrificeRoom"));
        assert_eq!(loot.heap_type, "sacrificial");
        assert!(loot.item.cursed);
        assert_eq!(loot.item.category, ItemCategory::Weapon);
        // uncursed weapons get a free upgrade before the curse is forced
        assert!(loot.item.level >= 1 || loot.item.enchantment.is_some());
    }

    #[test]
    fn sentry_prize_deterministic() {
        Random::reset_generators();
        let run = init_run(7);
        Random::push_generator_seeded(777);
        let mut d = dungeon_from_run(run.clone());
        d.depth = 8;
        let mut spawn = Vec::new();
        let a = sentry_prize(&mut d, &mut spawn);
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(777);
        let mut d2 = dungeon_from_run(run);
        d2.depth = 8;
        let mut spawn2 = Vec::new();
        let b = sentry_prize(&mut d2, &mut spawn2);
        Random::pop_generator();

        assert_eq!(a.item.class_name, b.item.class_name);
        assert_eq!(a.item.level, b.item.level);
        assert!(!a.item.cursed);
        assert_eq!(a.item.source.as_deref(), Some("SentryRoom"));
        assert_eq!(spawn.len(), 1);
        assert_eq!(spawn[0].class_name, "PotionOfHaste");
    }

    #[test]
    fn traps_prize_adds_levitation() {
        Random::reset_generators();
        let run = init_run(99);
        Random::push_generator_seeded(55);
        let mut d = dungeon_from_run(run);
        d.depth = 11;
        let mut spawn = Vec::new();
        let loot = traps_prize(&mut d, &mut spawn);
        Random::pop_generator();

        assert_eq!(loot.item.source.as_deref(), Some("TrapsRoom"));
        assert_eq!(loot.heap_type, "chest");
        assert!(!loot.item.cursed);
        assert_eq!(spawn[0].class_name, "PotionOfLevitation");
    }

    #[test]
    fn magical_fire_drops_and_frost() {
        Random::reset_generators();
        let run = init_run(3);
        Random::push_generator_seeded(9);
        let mut d = dungeon_from_run(run);
        d.depth = 4;
        let room = test_room("MagicalFireRoom", 9, 9);
        let mut spawn = Vec::new();
        let loot = magical_fire_prizes(&mut d, &room, &mut spawn);
        Random::pop_generator();

        assert!((3..=4).contains(&loot.len()));
        assert!(loot
            .iter()
            .all(|p| p.item.source.as_deref() == Some("MagicalFireRoom")));
        assert_eq!(
            spawn.last().map(|i| i.class_name.as_str()),
            Some("PotionOfFrost")
        );
    }

    #[test]
    fn secret_honeypot_has_pot_honey_bomb() {
        Random::reset_generators();
        Random::push_generator_seeded(1);
        let room = test_room("SecretHoneypotRoom", 7, 7);
        let loot = secret_honeypot(&room);
        Random::pop_generator();

        assert_eq!(loot.len(), 3);
        assert_eq!(loot[0].item.class_name, "ShatteredPot");
        assert_eq!(loot[1].item.class_name, "Honeypot");
        assert!(
            loot[2].item.class_name == "Bomb" || loot[2].item.class_name == "DoubleBomb",
            "got {}",
            loot[2].item.class_name
        );
    }

    #[test]
    fn toxic_gas_burns_layout_and_adds_purity() {
        Random::reset_generators();
        let run = init_run(12);
        Random::push_generator_seeded(88);
        let mut d = dungeon_from_run(run);
        d.depth = 7;
        let room = test_room("ToxicGasRoom", 9, 9);
        let mut spawn = Vec::new();
        let loot = toxic_gas_prizes(&mut d, &room, &mut spawn);
        Random::pop_generator();

        // skeleton gold + 2 chests
        assert_eq!(loot.len(), 3);
        assert_eq!(loot[0].heap_type, "skeleton");
        assert_eq!(loot[1].heap_type, "chest");
        assert_eq!(loot[2].heap_type, "chest");
        assert_eq!(
            spawn.last().map(|i| i.class_name.as_str()),
            Some("PotionOfPurity")
        );
    }

    #[test]
    fn secret_chest_chasm_four_locked_plus_levitation() {
        Random::reset_generators();
        let run = init_run(21);
        Random::push_generator_seeded(404);
        let mut d = dungeon_from_run(run);
        d.depth = 9;
        let mut spawn = Vec::new();
        let loot = secret_chest_chasm(&mut d, &mut spawn);
        Random::pop_generator();

        let chests: Vec<_> = loot
            .iter()
            .filter(|p| p.heap_type == "locked_chest")
            .collect();
        assert_eq!(chests.len(), 4);
        assert!(chests
            .iter()
            .all(|p| p.item.source.as_deref() == Some("SecretChestChasmRoom")));
        assert_eq!(
            spawn.last().map(|i| i.class_name.as_str()),
            Some("PotionOfLevitation")
        );
    }

    #[test]
    fn secret_summoning_skeleton_random() {
        Random::reset_generators();
        let run = init_run(5);
        Random::push_generator_seeded(11);
        let mut d = dungeon_from_run(run);
        d.depth = 6;
        let loot = secret_summoning_prize(&mut d);
        Random::pop_generator();

        assert_eq!(loot.heap_type, "skeleton");
        assert_eq!(loot.item.source.as_deref(), Some("SecretSummoningRoom"));
        assert!(!loot.item.class_name.is_empty());
    }

    #[test]
    fn secret_maze_uncursed_equip() {
        Random::reset_generators();
        let run = init_run(8);
        Random::push_generator_seeded(33);
        let mut d = dungeon_from_run(run);
        d.depth = 10;
        let loot = secret_maze_prize(&mut d);
        Random::pop_generator();

        assert_eq!(loot.heap_type, "chest");
        assert!(!loot.item.cursed);
        assert_eq!(loot.item.source.as_deref(), Some("SecretMazeRoom"));
        assert!(matches!(
            loot.item.category,
            ItemCategory::Weapon | ItemCategory::Armor
        ));
    }

    #[test]
    fn pit_room_skeleton_and_crystal_key() {
        Random::reset_generators();
        let run = init_run(15);
        Random::push_generator_seeded(66);
        let mut d = dungeon_from_run(run);
        d.depth = 12;
        let mut spawn = Vec::new();
        let loot = pit_prizes(&mut d, &mut spawn);
        Random::pop_generator();

        assert!((2..=3).contains(&loot.len())); // main + 1–2 side prizes
        assert!(loot.iter().all(|p| p.heap_type == "skeleton"));
        assert!(loot
            .iter()
            .all(|p| p.item.source.as_deref() == Some("PitRoom")));
        assert_eq!(
            spawn.last().map(|i| i.class_name.as_str()),
            Some("CrystalKey")
        );
    }

    #[test]
    fn garden_and_well_prizes() {
        Random::reset_generators();
        Random::push_generator_seeded(2);
        let room = test_room("GardenRoom", 7, 7);
        let mut spawn = Vec::new();
        let garden = garden_prizes(&room, &mut spawn);
        Random::pop_generator();
        assert_eq!(spawn[0].class_name, "IronKey");
        // bushes roll may yield 0–2 plants
        assert!(garden.len() <= 2);
        assert!(garden
            .iter()
            .all(|p| p.heap_type == "plant" && p.item.source.as_deref() == Some("GardenRoom")));

        Random::reset_generators();
        Random::push_generator_seeded(3);
        let well = magic_well(&mut spawn);
        Random::pop_generator();
        assert_eq!(well.len(), 1);
        assert!(
            well[0].item.class_name == "WaterOfAwareness"
                || well[0].item.class_name == "WaterOfHealth"
        );
        assert_eq!(well[0].heap_type, "well");

        Random::reset_generators();
        Random::push_generator_seeded(4);
        let room = test_room("SecretGardenRoom", 8, 8);
        let secret = secret_garden_prizes(&room);
        Random::pop_generator();
        assert_eq!(secret.len(), 4);
        assert_eq!(secret[0].item.class_name, "StarflowerSeed");
        assert!(secret
            .iter()
            .all(|p| p.item.source.as_deref() == Some("SecretGardenRoom")));
    }
}
