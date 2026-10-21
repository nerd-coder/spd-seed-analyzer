//! Pinned `mechanics.Ballistica` and `mechanics.ConeAOE` over a painted map.

use std::collections::BTreeSet;

use crate::level::terrain::{TerrainMap, CHASM, TRAP, WELL};

pub(crate) const STOP_TARGET: u32 = 1;
pub(crate) const STOP_SOLID: u32 = 4;
pub(crate) const WONT_STOP: u32 = 0;

/// Watabou `PointF.PI` and `G2R`. `3.1415926f` is one ulp below
/// `f32::consts::PI`, which shifts cone edge cells.
#[allow(clippy::approx_constant)]
const PI: f32 = 3.141_592_6;
const G2R: f32 = PI / 180.0;

pub(crate) struct Ballistica {
    pub path: Vec<usize>,
    pub collision: usize,
    pub dist: usize,
}

impl Ballistica {
    /// `subPath(start, end)`: inclusive, clamped to the path.
    pub fn sub_path(&self, start: usize, end: usize) -> &[usize] {
        let end = end.min(self.path.len().saturating_sub(1));
        if start > end {
            return &[];
        }
        &self.path[start..=end]
    }
}

fn inside_map(map: &TerrainMap, cell: i64) -> bool {
    let w = i64::from(map.width);
    let len = w * i64::from(map.height);
    !(cell < w || cell >= len - w || cell % w == 0 || cell % w == w - 1)
}

/// `Terrain.AVOID`.
fn avoid(tile: i32) -> bool {
    matches!(tile, CHASM | TRAP | WELL)
}

/// `new Ballistica(from, to, params)`. Level-start chars are `mob_occupied`.
pub(crate) fn ballistica(map: &TerrainMap, from: usize, to: usize, params: u32) -> Ballistica {
    let stop_target = params & STOP_TARGET != 0;
    let stop_terrain = params & STOP_SOLID != 0;
    let w = i64::from(map.width);
    let (from_i, to_i) = (from as i64, to as i64);
    let (x0, y0, x1, y1) = (from_i % w, from_i / w, to_i % w, to_i / w);
    let (mut dx, mut dy) = (x1 - x0, y1 - y0);
    let step_x = if dx > 0 { 1 } else { -1 };
    let step_y = if dy > 0 { 1 } else { -1 };
    dx = dx.abs();
    dy = dy.abs();
    let (step_a, step_b, d_a, d_b) = if dx > dy {
        (step_x, step_y * w, dx, dy)
    } else {
        (step_y * w, step_x, dy, dx)
    };

    let mut path: Vec<usize> = Vec::new();
    let mut collision: Option<usize> = None;
    let mut cell = from_i;
    let mut err = d_a / 2;
    while inside_map(map, cell) {
        let c = cell as usize;
        if collision.is_none()
            && stop_terrain
            && c != from
            && !map.passable[c]
            && !avoid(map.map[c])
            && !map.mob_occupied[c]
        {
            collision = path.last().copied();
        }
        path.push(c);
        if collision.is_none() && stop_terrain && c != from && map.is_solid(c) {
            collision = Some(c);
        }
        if collision.is_none() && c == to && stop_target {
            collision = Some(c);
        }
        cell += step_a;
        err += d_b;
        if err >= d_a {
            err -= d_a;
            cell += step_b;
        }
    }

    match collision {
        Some(collision) => Ballistica {
            dist: path.iter().position(|&c| c == collision).unwrap_or(0),
            path,
            collision,
        },
        None if !path.is_empty() => Ballistica {
            collision: *path.last().unwrap(),
            dist: path.len() - 1,
            path,
        },
        None => Ballistica {
            path: vec![from],
            collision: from,
            dist: 0,
        },
    }
}

/// `new ConeAOE(core, maxDist, degrees, params).cells`.
pub(crate) fn cone_aoe(
    map: &TerrainMap,
    core: &Ballistica,
    max_dist: f32,
    degrees: f32,
    params: u32,
) -> BTreeSet<usize> {
    let w = map.width as usize;
    let source = core.path[0];
    let from = ((source % w) as f32 + 0.5, (source / w) as f32 + 0.5);
    let mut to = (
        (core.collision % w) as f32 + 0.5,
        (core.collision / w) as f32 + 0.5,
    );
    let distance = |a: (f32, f32), b: (f32, f32)| {
        let (dx, dy) = (a.0 - b.0, a.1 - b.1);
        ((dx * dx + dy * dy) as f64).sqrt() as f32
    };
    if distance(from, to) > max_dist {
        let d = max_dist / distance(from, to);
        to = (from.0 + (to.0 - from.0) * d, from.1 + (to.1 - from.1) * d);
    }
    let radius = distance(from, to) + 0.5;

    let arc_cell = |angle: f32, length: f32| {
        let mut x = length * (f64::from(angle * G2R).cos() as f32) + from.0;
        let mut y = length * (f64::from(angle * G2R).sin() as f32) + from.1;
        x += if from.0 > x { 0.5 } else { -0.5 };
        y += if from.1 > y { 0.5 } else { -0.5 };
        let cx = (x.floor() as i32).clamp(0, map.width - 1);
        let cy = (y.floor() as i32).clamp(0, map.height - 1);
        (cx + cy * map.width) as usize
    };

    let initial = (f64::from(to.1 - from.1).atan2(f64::from(to.0 - from.0)) as f32) / G2R;
    let mut targets: Vec<usize> = Vec::new();
    let mut a = initial + degrees / 2.0;
    while a >= initial - degrees / 2.0 {
        for length in [Some(radius), (radius >= 4.0).then_some(radius - 1.0)]
            .into_iter()
            .flatten()
        {
            let cell = arc_cell(a, length);
            if !targets.contains(&cell) {
                targets.push(cell);
            }
        }
        a -= 0.5;
    }

    let mut cells = BTreeSet::new();
    for target in targets {
        let ray = ballistica(map, source, target, params);
        cells.extend(ray.sub_path(1, ray.dist).iter().copied());
    }
    cells
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::terrain::{EMPTY, WALL};

    fn open_room(width: i32, height: i32) -> TerrainMap {
        let mut map = TerrainMap::filled(width, height, WALL);
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                map.map[(x + y * width) as usize] = EMPTY;
            }
        }
        map.recompute_passable();
        map
    }

    #[test]
    fn wont_stop_runs_to_the_map_edge_and_solid_stops_before_walls() {
        let mut map = open_room(9, 9);
        let from = 4 + 4 * 9;
        let free = ballistica(&map, from, from + 1, WONT_STOP);
        assert_eq!(free.collision, 7 + 4 * 9);
        map.map[6 + 4 * 9] = WALL;
        map.recompute_passable();
        let blocked = ballistica(&map, from, from + 1, STOP_SOLID);
        assert_eq!(blocked.collision, 5 + 4 * 9);
        assert_eq!(blocked.sub_path(1, blocked.dist), &[5 + 4 * 9]);
    }

    #[test]
    fn narrow_cone_follows_its_axis() {
        let map = open_room(11, 11);
        let from = 5 + 5 * 11;
        let aim = ballistica(&map, from, from + 1, WONT_STOP);
        let cells = cone_aoe(&map, &aim, 3.0, 10.0, STOP_SOLID | STOP_TARGET);
        assert!(cells.contains(&(6 + 5 * 11)));
        assert!(cells.contains(&(8 + 5 * 11)));
        assert!(!cells.contains(&(4 + 5 * 11)));
        assert!(cells.iter().all(|&c| c % 11 > 5));
    }
}
