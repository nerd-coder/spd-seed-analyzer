//! `RegularLevel.initRooms` + sewer standard/special counts.

use crate::level::Feeling;
use crate::random::Random;
use crate::rooms::secret;
use crate::rooms::special::SpecialFloorState;
use crate::rooms::standard;
use crate::rooms::types::RoomSpec;

/// Floor room list after `initRooms` (before builder placement).
#[derive(Debug, Clone)]
pub struct FloorRooms {
    pub rooms: Vec<RoomSpec>,
    pub builder_kind: BuilderKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuilderKind {
    Loop,
    FigureEight,
}

/// SewerLevel.standardRooms
fn sewer_standard_rooms(force_max: bool) -> i32 {
    if force_max {
        return 6;
    }
    // 4 to 6, average 5
    4 + Random::chances(&[1., 3., 1.])
}

/// SewerLevel.specialRooms
fn sewer_special_rooms(force_max: bool) -> i32 {
    if force_max {
        return 2;
    }
    // 1 to 2, average 1.8
    1 + Random::chances(&[1., 4.])
}

/// `RegularLevel.builder()` — consumes RNG for builder choice / shape.
pub fn select_builder() -> BuilderKind {
    if Random::int_max(2) == 0 {
        let _ = Random::float_range(0.0, 0.65);
        let _ = Random::float_range(0.0, 0.50);
        BuilderKind::Loop
    } else {
        let _ = Random::float_range(0.3, 0.8);
        // offset fixed 0f — no second random
        BuilderKind::FigureEight
    }
}

/// Full initRooms sequence for a regular (non-boss) sewer-style floor.
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
) -> FloorRooms {
    // builder chosen first in build()
    let builder_kind = select_builder();

    let mut rooms = Vec::new();
    rooms.push(standard::create_entrance(depth));
    rooms.push(standard::create_exit(depth));

    let force_max = feeling == Feeling::Large;
    let mut standards = sewer_standard_rooms(force_max);
    if feeling == Feeling::Large {
        standards = (standards as f32 * 1.5).ceil() as i32;
    }

    // Java: for (i=0; i<standards; i++) { create; setSizeCat(standards-i); i += sizeFactor()-1; }
    // net: i increases by sizeFactor each accepted room
    let mut i = 0;
    while i < standards {
        let (name, size_factor) = loop {
            let (name, _ctor_size) = standard::create_standard_room(depth);
            if let Some(sf) = standard::set_size_cat(&name, standards - i) {
                break (name, sf);
            }
        };
        rooms.push(RoomSpec::standard(name, size_factor));
        i += size_factor;
    }

    if shop_on_level {
        rooms.push(RoomSpec::shop());
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
        rooms.push(room);
        si += 1;
    }
    *pit_needed_depth = special_floor.pit_needed_depth;

    let mut secrets = secret::secrets_for_floor(depth, region_secrets);
    if feeling == Feeling::Secrets {
        secrets += 1;
    }
    for _ in 0..secrets {
        rooms.push(secret::create_room(run_secrets));
    }

    // Random.shuffle(initRooms)
    Random::shuffle_vec(&mut rooms);

    FloorRooms {
        rooms,
        builder_kind,
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
        );
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(42);
        let mut run_sp = crate::rooms::special::init_for_run();
        let (mut run_sec, mut reg) = crate::rooms::secret::init_for_run();
        let mut lab = 0;
        let mut pit = -1;
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
        );
        Random::pop_generator();

        let names_a: Vec<_> = a.rooms.iter().map(|r| r.name.as_str()).collect();
        let names_b: Vec<_> = b.rooms.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names_a, names_b);
        assert_eq!(a.builder_kind, b.builder_kind);
    }
}
