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
        // Rooms without portable prize items (or not yet ported): still burn layout RNG where needed
        "GardenRoom" | "SecretGardenRoom" => {
            garden_rng(room);
            Vec::new()
        }
        "WeakFloorRoom" | "MagicWellRoom" | "PitRoom" => Vec::new(),
        "SentryRoom" | "ToxicGasRoom" | "MagicalFireRoom" | "TrapsRoom" | "SacrificeRoom" => {
            // Complex trap/mob rooms — prize generation deferred; no RNG burn (may desync).
            Vec::new()
        }
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

fn garden_rng(room: &Room) {
    // plants / dew — burn a couple of position rolls
    let mut occupied = Vec::new();
    for _ in 0..2 {
        burn_drop_pos(room, &mut occupied);
    }
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
