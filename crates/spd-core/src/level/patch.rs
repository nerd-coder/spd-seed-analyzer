//! Port of SPD `levels.Patch.generate` (cellular patches for water/grass).

use crate::java_random::JavaRandom;
use crate::random::Random;

/// `Patch.generate(w, h, fill, clustering, forceFillRate)`.
///
/// Returns a length-`w*h` boolean mask of filled cells (row-major).
pub fn generate(w: i32, h: i32, fill: f32, clustering: i32, force_fill_rate: bool) -> Vec<bool> {
    if w <= 0 || h <= 0 {
        return Vec::new();
    }
    let length = (w * h) as usize;
    let mut fill = fill;
    let mut fill_diff = -((length as f32 * fill).round() as i32);

    if force_fill_rate && clustering > 0 {
        fill += (0.5 - fill) * 0.5;
    }

    let mut off: Vec<bool> = Random::with_active(|generator| {
        (0..length).map(|_| generator.next_float() < fill).collect()
    });
    fill_diff += off.iter().filter(|&&cell| cell).count() as i32;

    let mut cur = vec![false; length];
    for _ in 0..clustering {
        fill_diff += smooth(&off, &mut cur, w as usize, h as usize);
        std::mem::swap(&mut cur, &mut off);
    }

    // Even with force fill rate, only adjust when there is an interior border.
    if force_fill_rate && w.min(h) > 2 {
        Random::with_active(|generator| force_fill(generator, &mut off, w, fill_diff));
    }

    off
}

/// Grow or shrink 3×3 blobs at random interior cells until the fill count hits
/// its target.
fn force_fill(generator: &mut JavaRandom, off: &mut [bool], w: i32, mut fill_diff: i32) {
    let length = off.len();
    let h = length as i32 / w;
    // `Random.Int(min, max)` on the active generator.
    let mut int_range = |min: i32, max: i32| {
        let span = max - min;
        min + if span > 0 {
            generator.next_int_bound(span)
        } else {
            0
        }
    };
    let neighbours: [i32; 9] = [-w - 1, -w, -w + 1, -1, 0, 1, w - 1, w, w + 1];
    let growing = fill_diff < 0;
    // Cap so a pathological float edge-case never hangs the analyzer.
    let mut guard = length * 20 + 64;
    while fill_diff != 0 && guard > 0 {
        guard -= 1;
        let mut tries = 0;
        // random cell, not in map borders: Random.Int(1, w-1) + Random.Int(1, h-1)*w
        let cell = loop {
            let cx = int_range(1, w - 1);
            let cy = int_range(1, h - 1);
            let c = (cx + cy * w) as usize;
            tries += 1;
            if off[c] == growing || tries * 10 >= length as i32 {
                break c;
            }
        };
        for &di in &neighbours {
            if fill_diff == 0 {
                break;
            }
            let npos = (cell as i32 + di) as usize;
            if npos >= length {
                continue;
            }
            if off[npos] != growing {
                off[npos] = growing;
                fill_diff += if growing { 1 } else { -1 };
            }
        }
    }
}

/// One clustering pass: a cell fills when at least half of its in-bounds 3×3
/// neighbourhood (itself included) is filled. Returns the fill-count change.
///
/// Per row: vertical 3-sums (out-of-map rows read as empty), then a 3-wide
/// window. Same counts as Java's per-cell neighbour checks, but branch-free
/// so the inner loops vectorize.
fn smooth(off: &[bool], cur: &mut [bool], w: usize, h: usize) -> i32 {
    let empty = vec![false; w];
    let mut columns = vec![0u8; w];
    let mut diff = 0;
    for y in 0..h {
        let row = &off[y * w..(y + 1) * w];
        let up = if y > 0 {
            &off[(y - 1) * w..y * w]
        } else {
            &empty
        };
        let down = if y + 1 < h {
            &off[(y + 1) * w..(y + 2) * w]
        } else {
            &empty
        };
        for (sum, ((&a, &b), &c)) in columns.iter_mut().zip(up.iter().zip(row).zip(down)) {
            *sum = a as u8 + b as u8 + c as u8;
        }
        let rows = 1 + (y > 0) as u8 + (y + 1 < h) as u8;
        let out = &mut cur[y * w..(y + 1) * w];
        let mut settle = |x: usize, count: u8, width: u8| {
            let filled = 2 * count >= rows * width;
            out[x] = filled;
            diff += filled as i32 - row[x] as i32;
        };
        if w == 1 {
            settle(0, columns[0], 1);
            continue;
        }
        settle(0, columns[0] + columns[1], 2);
        for (x, window) in columns.windows(3).enumerate() {
            settle(x + 1, window[0] + window[1] + window[2], 3);
        }
        settle(w - 1, columns[w - 2] + columns[w - 1], 2);
    }
    diff
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct CavesBossPatchFixture {
        depth_seed: i64,
        mask: Vec<bool>,
        post_patch_rng: Vec<i32>,
        trap_rolls: usize,
        entrance_variant: i32,
        corner_variant: i32,
        post_entrance_rng: Vec<i32>,
    }

    #[test]
    fn patch_generate_size_and_rng_stable() {
        Random::push_generator_seeded(0xC0FFEE_u64 as i64);
        let a = generate(8, 6, 0.3, 3, true);
        Random::pop_generator();
        assert_eq!(a.len(), 48);

        Random::push_generator_seeded(0xC0FFEE_u64 as i64);
        let b = generate(8, 6, 0.3, 3, true);
        Random::pop_generator();
        assert_eq!(a, b);
    }

    #[test]
    fn patch_empty_dims() {
        assert!(generate(0, 5, 0.5, 0, true).is_empty());
        assert!(generate(5, 0, 0.5, 0, true).is_empty());
    }

    #[test]
    fn caves_boss_patch_matches_java_oracle() {
        let fixture: CavesBossPatchFixture = serde_json::from_str(include_str!(
            "../../../../tools/java-oracle/fixtures/aaa-aaa-aaa-caves-boss-patch.json"
        ))
        .unwrap();
        Random::push_generator_seeded(fixture.depth_seed);
        let actual = generate(33, 28, 0.15, 2, true);
        let post_patch_rng = Random::peek_ints(8);
        Random::pop_generator();

        assert_eq!(actual, fixture.mask);
        assert_eq!(post_patch_rng, fixture.post_patch_rng);

        Random::push_generator_seeded(fixture.depth_seed);
        let mask = generate(33, 28, 0.15, 2, true);
        let mut trap_rolls = 0;
        for row in 0..23 {
            let radius = 23.0 / 2.0;
            let row_y = -radius + 0.5 + f64::from(row);
            let row_width = (2.0
                * ((radius * radius) * (1.0 - row_y * row_y / (radius * radius))).sqrt()
                / 2.0)
                .floor() as i32
                * 2
                + 1;
            let left = 5 + (23 - row_width) / 2;
            for x in left..left + row_width {
                if !mask[(x + row * 33) as usize] {
                    Random::int_max(8);
                    trap_rolls += 1;
                }
            }
        }
        let entrance_variant = Random::int_max(4);
        let corner_variant = Random::int_max(4);
        let post_entrance_rng = Random::peek_ints(8);
        Random::pop_generator();
        assert_eq!(trap_rolls, fixture.trap_rolls);
        assert_eq!(entrance_variant, fixture.entrance_variant);
        assert_eq!(corner_variant, fixture.corner_variant);
        assert_eq!(post_entrance_rng, fixture.post_entrance_rng);
    }

    /// Java's per-cell neighbour walk, kept as the reference for `smooth`.
    fn smooth_reference(off: &[bool], w: i32, h: i32) -> (Vec<bool>, i32) {
        let mut cur = vec![false; off.len()];
        let mut diff = 0;
        for y in 0..h {
            for x in 0..w {
                let (mut count, mut neighbours) = (0, 0);
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let (nx, ny) = (x + dx, y + dy);
                        if (0..w).contains(&nx) && (0..h).contains(&ny) {
                            neighbours += 1;
                            count += off[(nx + ny * w) as usize] as i32;
                        }
                    }
                }
                let pos = (x + y * w) as usize;
                cur[pos] = 2 * count >= neighbours;
                if cur[pos] != off[pos] {
                    diff += if cur[pos] { 1 } else { -1 };
                }
            }
        }
        (cur, diff)
    }

    #[test]
    fn smooth_matches_java_neighbour_walk() {
        Random::push_generator_seeded(0x5_1007);
        for (w, h) in [(1, 1), (1, 7), (7, 1), (2, 2), (3, 5), (33, 28), (48, 36)] {
            for fill in [0.1, 0.45, 0.8] {
                let off: Vec<bool> = (0..w * h).map(|_| Random::float() < fill).collect();
                let mut cur = vec![false; off.len()];
                let diff = smooth(&off, &mut cur, w as usize, h as usize);
                assert_eq!(
                    (cur, diff),
                    smooth_reference(&off, w, h),
                    "{w}x{h} @ {fill}"
                );
            }
        }
        Random::pop_generator();
    }
}
