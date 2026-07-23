//! Port of `com.watabou.utils.Random` — stack of `java.util.Random` generators.

use std::cell::RefCell;
use std::collections::VecDeque;

use crate::java_random::JavaRandom;

thread_local! {
    static GENERATORS: RefCell<VecDeque<JavaRandom>> = RefCell::new({
        let mut d = VecDeque::new();
        d.push_back(JavaRandom::new_unseeded());
        d
    });

    #[cfg(test)]
    static STACK_MUTATIONS: RefCell<(usize, usize)> = const { RefCell::new((0, 0)) };
}

/// Static RNG façade matching watabou `Random`.
pub struct Random;

impl Random {
    /// Reset to a single unseeded base generator.
    pub fn reset_generators() {
        GENERATORS.with(|g| {
            let mut g = g.borrow_mut();
            g.clear();
            g.push_back(JavaRandom::new_unseeded());
        });
    }

    /// Push an unseeded generator.
    pub fn push_generator() {
        GENERATORS.with(|g| {
            g.borrow_mut().push_front(JavaRandom::new_unseeded());
        });
        #[cfg(test)]
        STACK_MUTATIONS.with(|mutations| mutations.borrow_mut().0 += 1);
    }

    /// Push a generator seeded with `scramble_seed(seed)`.
    pub fn push_generator_seeded(seed: i64) {
        let scrambled = Self::scramble_seed(seed);
        GENERATORS.with(|g| {
            g.borrow_mut().push_front(JavaRandom::new(scrambled));
        });
        #[cfg(test)]
        STACK_MUTATIONS.with(|mutations| mutations.borrow_mut().0 += 1);
    }

    /// MX3 scramble (Jon Maiga), matching SPD `Random.scrambleSeed`.
    pub fn scramble_seed(seed: i64) -> i64 {
        let mut seed = seed as u64;
        seed ^= seed >> 32;
        seed = seed.wrapping_mul(0xbea225f9eb34556d);
        seed ^= seed >> 29;
        seed = seed.wrapping_mul(0xbea225f9eb34556d);
        seed ^= seed >> 32;
        seed = seed.wrapping_mul(0xbea225f9eb34556d);
        seed ^= seed >> 29;
        seed as i64
    }

    /// Pop the top generator (must not pop the last one).
    pub fn pop_generator() {
        GENERATORS.with(|g| {
            let mut g = g.borrow_mut();
            if g.len() <= 1 {
                // Java reports exception but does not pop; we no-op.
                return;
            }
            g.pop_front();
            #[cfg(test)]
            STACK_MUTATIONS.with(|mutations| mutations.borrow_mut().1 += 1);
        });
    }

    fn with_top<F, R>(use_generator_stack: bool, f: F) -> R
    where
        F: FnOnce(&mut JavaRandom) -> R,
    {
        GENERATORS.with(|g| {
            let mut g = g.borrow_mut();
            // peekFirst = front (top of stack), peekLast = back (base)
            if use_generator_stack {
                f(g.front_mut().expect("generator stack empty"))
            } else {
                f(g.back_mut().expect("generator stack empty"))
            }
        })
    }

    /// Uniform float in `[0, 1)`.
    pub fn float() -> f32 {
        Self::float_stack(true)
    }

    pub fn float_stack(use_generator_stack: bool) -> f32 {
        Self::with_top(use_generator_stack, |r| r.next_float())
    }

    /// Uniform float in `[0, max)`.
    pub fn float_max(max: f32) -> f32 {
        Self::float() * max
    }

    /// Uniform float in `[min, max)`.
    pub fn float_range(min: f32, max: f32) -> f32 {
        min + Self::float_max(max - min)
    }

    /// Full-range int.
    pub fn int() -> i32 {
        Self::int_stack(true)
    }

    pub fn int_stack(use_generator_stack: bool) -> i32 {
        Self::with_top(use_generator_stack, |r| r.next_int())
    }

    /// Clone the active generator and inspect upcoming full-range ints without
    /// advancing the real stream. Used by the Java parity harness at lifecycle
    /// boundaries where comparing outputs alone would hide the first desync.
    pub(crate) fn peek_ints(count: usize) -> Vec<i32> {
        Self::with_top(true, |generator| {
            let mut snapshot = generator.clone();
            (0..count).map(|_| snapshot.next_int()).collect()
        })
    }

    /// Uniform int in `[0, max)`. Returns 0 if `max <= 0` (watabou behavior).
    pub fn int_max(max: i32) -> i32 {
        Self::int_max_stack(max, true)
    }

    pub fn int_max_stack(max: i32, use_generator_stack: bool) -> i32 {
        if max <= 0 {
            return 0;
        }
        Self::with_top(use_generator_stack, |r| r.next_int_bound(max))
    }

    /// Uniform int in `[min, max)`.
    pub fn int_range(min: i32, max: i32) -> i32 {
        min + Self::int_max(max - min)
    }

    /// Uniform int in `[min, max]` inclusive.
    pub fn int_range_inclusive(min: i32, max: i32) -> i32 {
        min + Self::int_max(max - min + 1)
    }

    /// Triangularly distributed int in `[min, max]` inclusive (`NormalIntRange`).
    pub fn normal_int_range(min: i32, max: i32) -> i32 {
        min + ((Self::float() + Self::float()) * (max - min + 1) as f32 / 2.0) as i32
    }

    /// Full-range long.
    pub fn long() -> i64 {
        Self::long_stack(true)
    }

    pub fn long_stack(use_generator_stack: bool) -> i64 {
        Self::with_top(use_generator_stack, |r| r.next_long())
    }

    /// Mostly uniform long in `[0, max)`.
    pub fn long_max(max: i64) -> i64 {
        let mut result = Self::long();
        if result < 0 {
            result += i64::MAX;
        }
        result % max
    }

    /// `Random.oneOf(T...)` / pick uniform element (panics if empty).
    pub fn one_of<T>(items: &[T]) -> &T {
        assert!(!items.is_empty(), "Random::one_of on empty slice");
        &items[Self::int_max(items.len() as i32) as usize]
    }

    /// `Random.element(Collection)` — returns None if empty.
    pub fn element<T>(items: &[T]) -> Option<&T> {
        if items.is_empty() {
            None
        } else {
            Some(&items[Self::int_max(items.len() as i32) as usize])
        }
    }

    /// Weighted index from chances array. Negative weights treated as 0. Returns -1 if sum <= 0.
    pub fn chances(chances: &[f32]) -> i32 {
        let mut sum = 0.0f32;
        for &c in chances {
            sum += c.max(0.0);
        }
        if sum <= 0.0 {
            return -1;
        }
        let value = Self::float_max(sum);
        let mut acc = 0.0f32;
        for (i, &c) in chances.iter().enumerate() {
            acc += c.max(0.0);
            if value < acc {
                return i as i32;
            }
        }
        -1
    }

    /// Fisher-Yates shuffle matching watabou `shuffle(T[] array)`.
    pub fn shuffle<T>(array: &mut [T]) {
        let len = array.len();
        if len <= 1 {
            return;
        }
        for i in 0..len - 1 {
            let j = Self::int_range(i as i32, len as i32) as usize;
            if j != i {
                array.swap(i, j);
            }
        }
    }

    /// Shuffle a `Vec` in place (same algorithm as array shuffle).
    pub fn shuffle_vec<T>(vec: &mut Vec<T>) {
        Self::shuffle(vec.as_mut_slice());
    }

    /// Shuffle a Java `List` using `Collections.shuffle(list, generator)`.
    ///
    /// SPD uses both its own forward Fisher–Yates helper for arrays and the
    /// JDK's backwards list shuffle.  Keeping this separate is important for
    /// room painters which pass an `ArrayList` to `Random.shuffle`.
    pub fn shuffle_list<T>(list: &mut [T]) {
        for i in (2..=list.len()).rev() {
            let j = Self::int_max(i as i32) as usize;
            list.swap(i - 1, j);
        }
    }

    /// Number of generators on the stack (for tests).
    #[cfg(test)]
    pub fn stack_len() -> usize {
        GENERATORS.with(|g| g.borrow().len())
    }

    /// Total successful pushes and pops on this thread (for draw-shape tests).
    #[cfg(test)]
    pub fn stack_mutations() -> (usize, usize) {
        STACK_MUTATIONS.with(|mutations| *mutations.borrow())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scramble_seed_smoke() {
        // Just ensure it's deterministic and non-identity for small seeds
        let a = Random::scramble_seed(1);
        let b = Random::scramble_seed(1);
        assert_eq!(a, b);
        assert_ne!(a, 1);
    }

    #[test]
    fn push_pop_stack() {
        Random::reset_generators();
        assert_eq!(Random::stack_len(), 1);
        Random::push_generator_seeded(42);
        assert_eq!(Random::stack_len(), 2);
        let _ = Random::int();
        Random::pop_generator();
        assert_eq!(Random::stack_len(), 1);
    }

    #[test]
    fn seeded_sequence_deterministic() {
        Random::reset_generators();
        Random::push_generator_seeded(123456);
        let a: Vec<i32> = (0..10).map(|_| Random::int()).collect();
        Random::pop_generator();

        Random::reset_generators();
        Random::push_generator_seeded(123456);
        let b: Vec<i32> = (0..10).map(|_| Random::int()).collect();
        Random::pop_generator();

        assert_eq!(a, b);
    }

    #[test]
    fn java_list_shuffle_uses_backwards_collections_order() {
        Random::reset_generators();
        Random::push_generator_seeded(0x1157);
        let mut list = vec![0, 1, 2, 3, 4, 5];
        Random::shuffle_list(&mut list);
        Random::pop_generator();
        assert_eq!(list, [2, 5, 1, 4, 3, 0]);
    }

    #[test]
    fn chances_picks_only_nonzero() {
        Random::reset_generators();
        Random::push_generator_seeded(1);
        for _ in 0..50 {
            let i = Random::chances(&[0.0, 1.0, 0.0]);
            assert_eq!(i, 1);
        }
        Random::pop_generator();
    }
}
