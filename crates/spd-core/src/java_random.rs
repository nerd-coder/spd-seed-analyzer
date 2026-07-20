//! Faithful port of `java.util.Random` (48-bit LCG).
//!
//! Algorithm matches OpenJDK / Android libcore used by libGDX SPD builds.

/// Multiplier for the linear congruential generator.
const MULTIPLIER: u64 = 0x5DEECE66D;
/// Addend for the linear congruential generator.
const ADDEND: u64 = 0xB;
/// Mask to keep 48 bits of state.
const MASK: u64 = (1 << 48) - 1;

/// Java-compatible pseudorandom number generator.
#[derive(Clone, Debug)]
pub struct JavaRandom {
    /// Internal 48-bit seed state.
    seed: u64,
}

impl JavaRandom {
    /// Create a generator with the given seed (same as `new java.util.Random(seed)`).
    pub fn new(seed: i64) -> Self {
        let mut r = Self { seed: 0 };
        r.set_seed(seed);
        r
    }

    /// Unseeded constructor matching `new java.util.Random()` is not used for analysis;
    /// provided for stack base generator compatibility.
    pub fn new_unseeded() -> Self {
        // Java uses a unique seed from nanoTime; for deterministic analysis we only
        // use explicitly seeded generators. Base stack entry is never consulted
        // during seeded runs after pushGenerator(seed).
        Self::new(0)
    }

    /// `Random.setSeed(long)`.
    pub fn set_seed(&mut self, seed: i64) {
        self.seed = (seed as u64 ^ MULTIPLIER) & MASK;
    }

    /// Protected `next(bits)` — returns the next pseudorandom bits.
    pub fn next(&mut self, bits: i32) -> i32 {
        self.seed = self.seed.wrapping_mul(MULTIPLIER).wrapping_add(ADDEND) & MASK;
        (self.seed >> (48 - bits as u32)) as i32
    }

    /// `nextInt()` — full 32-bit signed int.
    pub fn next_int(&mut self) -> i32 {
        self.next(32)
    }

    /// `nextInt(bound)` — uniform in `[0, bound)`.
    ///
    /// Uses OpenJDK rejection sampling for non-power-of-two bounds.
    pub fn next_int_bound(&mut self, bound: i32) -> i32 {
        if bound <= 0 {
            // Java throws IllegalArgumentException; watabou Random.Int(max) returns 0 for max <= 0
            // before calling nextInt. Callers that need Java semantics should check bound first.
            panic!("bound must be positive");
        }

        let bound_u = bound as u32;

        // Power of two: (bound * next(31)) >> 31
        if (bound_u & (bound_u - 1)) == 0 {
            return ((bound as i64 * self.next(31) as i64) >> 31) as i32;
        }

        // Rejection sampling
        loop {
            let bits = self.next(31);
            let val = bits % bound;
            if bits - val + (bound - 1) >= 0 {
                return val;
            }
        }
    }

    /// `nextLong()`.
    pub fn next_long(&mut self) -> i64 {
        // ((long)next(32) << 32) + next(32)
        ((self.next(32) as i64) << 32).wrapping_add(self.next(32) as i64)
    }

    /// `nextFloat()` — value in `[0.0, 1.0)`.
    pub fn next_float(&mut self) -> f32 {
        self.next(24) as f32 / (1i32 << 24) as f32
    }

    /// `nextDouble()` — value in `[0.0, 1.0)`.
    pub fn next_double(&mut self) -> f64 {
        let high = self.next(26) as i64;
        let low = self.next(27) as i64;
        ((high << 27) + low) as f64 / (1i64 << 53) as f64
    }

    /// Current internal seed state (for tests).
    #[cfg(test)]
    pub fn internal_seed(&self) -> u64 {
        self.seed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_sequence_seed_1() {
        // Verified against JVM java.util.Random(1)
        let mut r = JavaRandom::new(1);
        assert_eq!(r.next_int(), -1155869325);
        assert_eq!(r.next_int(), 431529176);
        assert_eq!(r.next_int(), 1761283695);
        assert_eq!(r.next_int(), 1749940626);
        assert_eq!(r.next_int(), 892128508);
    }

    #[test]
    fn next_int_bound_power_of_two() {
        let mut r = JavaRandom::new(42);
        for _ in 0..100 {
            let v = r.next_int_bound(16);
            assert!((0..16).contains(&v));
        }
    }

    #[test]
    fn next_int_bound_non_power_of_two() {
        let mut r = JavaRandom::new(12345);
        for _ in 0..200 {
            let v = r.next_int_bound(10);
            assert!((0..10).contains(&v));
        }
    }

    #[test]
    fn next_float_range() {
        let mut r = JavaRandom::new(7);
        for _ in 0..100 {
            let f = r.next_float();
            assert!((0.0..1.0).contains(&f));
        }
    }

    #[test]
    fn next_long_sequence() {
        let mut r = JavaRandom::new(1);
        // First nextLong from seed 1 on JVM
        assert_eq!(r.next_long(), -4964420948893066024);
    }
}
