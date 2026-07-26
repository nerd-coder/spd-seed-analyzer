//! Port of SPD `levels.Patch.generate` (cellular patches for water/grass).

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

                if y > 0 {
                    if x > 0 {
                        if off[pos - w as usize - 1] {
                            count += 1;
                        }
                        neighbours += 1;
                    }
                    if off[pos - w as usize] {
                        count += 1;
                    }
                    neighbours += 1;
                    if x < w - 1 {
                        if off[pos - w as usize + 1] {
                            count += 1;
                        }
                        neighbours += 1;
                    }
                }

                if x > 0 {
                    if off[pos - 1] {
                        count += 1;
                    }
                    neighbours += 1;
                }
                if off[pos] {
                    count += 1;
                }
                neighbours += 1;
                if x < w - 1 {
                    if off[pos + 1] {
                        count += 1;
                    }
                    neighbours += 1;
                }

                if y < h - 1 {
                    if x > 0 {
                        if off[pos + w as usize - 1] {
                            count += 1;
                        }
                        neighbours += 1;
                    }
                    if off[pos + w as usize] {
                        count += 1;
                    }
                    neighbours += 1;
                    if x < w - 1 {
                        if off[pos + w as usize + 1] {
                            count += 1;
                        }
                        neighbours += 1;
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

    // Even with force fill rate, only adjust when there is an interior border.
    if force_fill_rate && w.min(h) > 2 {
        let neighbours: [i32; 9] = [-w - 1, -w, -w + 1, -1, 0, 1, w - 1, w, w + 1];
        let growing = fill_diff < 0;
        // Cap so a pathological float edge-case never hangs the analyzer.
        let mut guard = length * 20 + 64;
        while fill_diff != 0 && guard > 0 {
            guard -= 1;
            let mut tries = 0;
            // random cell, not in map borders: Random.Int(1, w-1) + Random.Int(1, h-1)*w
            let cell = loop {
                let cx = Random::int_range(1, w - 1);
                let cy = Random::int_range(1, h - 1);
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

    off
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
}
