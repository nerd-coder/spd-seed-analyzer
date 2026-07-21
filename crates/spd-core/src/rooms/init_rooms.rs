//! `RegularLevel.initRooms` + builder selection.

use crate::generator::GeneratorState;
use crate::level::Feeling;
use crate::quests::{self, BlacksmithQuestState, ImpQuestState, WandmakerQuestState};
use crate::random::Random;
use crate::rooms::room::{dims_for_kind, Room};
use crate::rooms::secret;
use crate::rooms::special::SpecialFloorState;
use crate::rooms::standard;
use crate::rooms::types::RoomSpec;

#[derive(Debug, Clone)]
pub struct FloorRooms {
    pub rooms: Vec<Room>,
    pub builder_kind: BuilderKind,
    pub curve_intensity: f32,
    pub curve_offset: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuilderKind {
    Loop,
    FigureEight,
}

fn sewer_standard_rooms(force_max: bool) -> i32 {
    if force_max {
        return 6;
    }
    4 + Random::chances(&[1., 3., 1.])
}

fn sewer_special_rooms(force_max: bool) -> i32 {
    if force_max {
        return 2;
    }
    1 + Random::chances(&[1., 4.])
}

/// `RegularLevel.builder()` — returns kind + loop shape params.
pub fn select_builder() -> (BuilderKind, f32, f32) {
    if Random::int_max(2) == 0 {
        let intensity = Random::float_range(0.0, 0.65);
        let offset = Random::float_range(0.0, 0.50);
        (BuilderKind::Loop, intensity, offset)
    } else {
        let intensity = Random::float_range(0.3, 0.8);
        (BuilderKind::FigureEight, intensity, 0.0)
    }
}

fn room_from_spec(id: usize, spec: RoomSpec) -> Room {
    let (mw, xw, mh, xh) = dims_for_kind(spec.kind, spec.size_factor, &spec.name);
    Room::new(
        id,
        spec.name,
        spec.kind,
        spec.size_factor,
        spec.max_connections,
        mw,
        xw,
        mh,
        xh,
    )
}

/// Full initRooms sequence for a regular (non-boss) floor.
///
/// Region quest rooms are added after base rooms and **before** shuffle:
/// - Prison: `Wandmaker.Quest.spawnRoom`
/// - Caves: `Blacksmith.Quest.spawn` (also generates smithRewards immediately)
/// - City: `Imp.Quest.spawn` (also generates the ring reward immediately)
#[allow(clippy::too_many_arguments)] // mirrors SPD RegularLevel.initRooms parameter surface
pub fn init_rooms_regular(
    depth: i32,
    feeling: Feeling,
    shop_on_level: bool,
    lab_needed: bool,
    lab_room_count: &mut i32,
    run_specials: &mut Vec<&'static str>,
    run_secrets: &mut Vec<&'static str>,
    region_secrets: &mut [i32; 5],
    pit_needed_depth: &mut i32,
    wandmaker: &mut WandmakerQuestState,
    blacksmith: &mut BlacksmithQuestState,
    imp: &mut ImpQuestState,
    generator: &mut GeneratorState,
) -> FloorRooms {
    let (builder_kind, curve_intensity, curve_offset) = select_builder();

    let mut specs: Vec<RoomSpec> = Vec::new();
    specs.push(standard::create_entrance(depth));
    specs.push(standard::create_exit(depth));

    let force_max = feeling == Feeling::Large;
    let mut standards = sewer_standard_rooms(force_max);
    if feeling == Feeling::Large {
        standards = (standards as f32 * 1.5).ceil() as i32;
    }

    let mut i = 0;
    while i < standards {
        let (name, size_factor) = loop {
            let (name, _ctor_size) = standard::create_standard_room(depth);
            if let Some(sf) = standard::set_size_cat(&name, standards - i) {
                break (name, sf);
            }
        };
        specs.push(RoomSpec::standard(name, size_factor));
        i += size_factor;
    }

    if shop_on_level {
        specs.push(RoomSpec::shop());
    }

    let mut specials = sewer_special_rooms(force_max);
    if feeling == Feeling::Large {
        specials += 1;
    }

    let mut special_floor =
        SpecialFloorState::init_for_floor(run_specials, lab_needed, lab_room_count);
    special_floor.pit_needed_depth = *pit_needed_depth;

    let boss_next = matches!(depth + 1, 5 | 10 | 15 | 20 | 25);
    let mut si = 0;
    while si < specials {
        let room = special_floor.create_room(run_specials, depth, boss_next);
        if room.name == "PitRoom" {
            specials += 1;
        }
        specs.push(room);
        si += 1;
    }
    *pit_needed_depth = special_floor.pit_needed_depth;

    let mut secrets = secret::secrets_for_floor(depth, region_secrets);
    if feeling == Feeling::Secrets {
        secrets += 1;
    }
    for _ in 0..secrets {
        specs.push(secret::create_room(run_secrets));
    }

    // PrisonLevel.initRooms: Wandmaker.Quest.spawnRoom(super.initRooms())
    let _ = quests::try_spawn_wandmaker_room(wandmaker, depth, &mut specs);
    // CavesLevel.initRooms: Blacksmith.Quest.spawn — generates smithRewards now
    let _ = quests::try_spawn_blacksmith(blacksmith, generator, depth, &mut specs);
    // CityLevel.initRooms: Imp.Quest.spawn(super.initRooms()) — generates ring now
    let _ = quests::try_spawn_imp(imp, generator, depth, &mut specs);
    // HallsLevel.initRooms appends one mandatory demon spawner before build shuffle.
    if (21..=24).contains(&depth) {
        specs.push(RoomSpec::special("DemonSpawnerRoom"));
    }

    // RegularLevel.build passes an ArrayList to Random.shuffle, which delegates
    // to `Collections.shuffle` (backwards Fisher-Yates), not watabou's array helper.
    Random::shuffle_list(&mut specs);

    let rooms: Vec<Room> = specs
        .into_iter()
        .enumerate()
        .map(|(id, s)| room_from_spec(id, s))
        .collect();

    FloorRooms {
        rooms,
        builder_kind,
        curve_intensity,
        curve_offset,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn init_rooms_deterministic() {
        Random::reset_generators();
        Random::push_generator_seeded(42);
        let mut run_sp = crate::rooms::special::init_for_run();
        let (mut run_sec, mut reg) = crate::rooms::secret::init_for_run();
        let mut lab = 0;
        let mut pit = -1;
        let mut wm = WandmakerQuestState::default();
        let mut bs = BlacksmithQuestState::default();
        let mut imp = ImpQuestState::default();
        let mut gen = crate::generator::full_reset();
        let a = init_rooms_regular(
            2,
            Feeling::None,
            false,
            false,
            &mut lab,
            &mut run_sp,
            &mut run_sec,
            &mut reg,
            &mut pit,
            &mut wm,
            &mut bs,
            &mut imp,
            &mut gen,
        );
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(42);
        let mut run_sp = crate::rooms::special::init_for_run();
        let (mut run_sec, mut reg) = crate::rooms::secret::init_for_run();
        let mut lab = 0;
        let mut pit = -1;
        let mut wm = WandmakerQuestState::default();
        let mut bs = BlacksmithQuestState::default();
        let mut imp = ImpQuestState::default();
        let mut gen = crate::generator::full_reset();
        let b = init_rooms_regular(
            2,
            Feeling::None,
            false,
            false,
            &mut lab,
            &mut run_sp,
            &mut run_sec,
            &mut reg,
            &mut pit,
            &mut wm,
            &mut bs,
            &mut imp,
            &mut gen,
        );
        Random::pop_generator();

        let names_a: Vec<_> = a.rooms.iter().map(|r| r.name.as_str()).collect();
        let names_b: Vec<_> = b.rooms.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(
            names_a,
            [
                "SecretArtilleryRoom",
                "SewerPipeRoom",
                "WaterBridgeEntranceRoom",
                "MagicWellRoom",
                "RingExitRoom",
                "WaterBridgeRoom",
                "CircleBasinRoom",
            ]
        );
        assert_eq!(names_a, names_b);
        assert_eq!(a.builder_kind, b.builder_kind);
    }

    #[test]
    fn halls_append_the_mandatory_demon_spawner() {
        Random::reset_generators();
        Random::push_generator_seeded(0xD3E0);
        let mut run_sp = crate::rooms::special::init_for_run();
        let (mut run_sec, mut reg) = crate::rooms::secret::init_for_run();
        let mut lab = 0;
        let mut pit = -1;
        let mut wm = WandmakerQuestState::default();
        let mut bs = BlacksmithQuestState::default();
        let mut imp = ImpQuestState::default();
        let mut generator = crate::generator::full_reset();
        let floor = init_rooms_regular(
            23,
            Feeling::None,
            false,
            false,
            &mut lab,
            &mut run_sp,
            &mut run_sec,
            &mut reg,
            &mut pit,
            &mut wm,
            &mut bs,
            &mut imp,
            &mut generator,
        );
        Random::pop_generator();

        assert_eq!(
            floor
                .rooms
                .iter()
                .filter(|room| room.name == "DemonSpawnerRoom")
                .count(),
            1
        );
    }
}
