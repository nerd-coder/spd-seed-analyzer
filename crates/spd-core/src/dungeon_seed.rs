//! Port of `com.shatteredpixel.shatteredpixeldungeon.utils.DungeonSeed`.

use thiserror::Error;

/// Largest possible seed has a value of 26^9.
pub const TOTAL_SEEDS: i64 = 5_429_503_678_976; // 26^9

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SeedError {
    #[error("seed input is empty")]
    Empty,
    #[error("codes must be 9 A-Z characters")]
    InvalidCode,
    #[error("seeds must be within the range [0, TOTAL_SEEDS)")]
    OutOfRange,
}

/// Seed encode/decode helpers.
pub struct DungeonSeed;

impl DungeonSeed {
    /// Takes a seed code (`ABC-DEF-GHI` or `ABCDEFGHI`) and converts to long value.
    pub fn convert_from_code(code: &str) -> Result<i64, SeedError> {
        let mut code = code.to_string();

        // If formatted properly, force uppercase
        if code.len() == 11
            && code.as_bytes().get(3) == Some(&b'-')
            && code.as_bytes().get(7) == Some(&b'-')
        {
            code = code.to_ascii_uppercase();
        }

        // Ignore whitespace and dashes
        let code: String = code
            .chars()
            .filter(|c| *c != '-' && !c.is_whitespace())
            .collect();

        if code.len() != 9 {
            return Err(SeedError::InvalidCode);
        }

        let mut result: i64 = 0;
        for (idx, ch) in code.chars().enumerate() {
            let c = ch as u8;
            if !c.is_ascii_uppercase() {
                // allow lowercase by normalizing only when full format matched above;
                // Java requires A-Z after strip — lowercase without dashes fails.
                return Err(SeedError::InvalidCode);
            }
            let power = 8 - idx;
            // Java uses Math.pow(26, power) as double then adds — we use integer pow for exactness
            // for exponents 0..8 this matches exactly.
            result += (c as i64 - 65) * 26i64.pow(power as u32);
        }
        Ok(result)
    }

    /// Takes a long value and converts to `ABC-DEF-GHI`.
    pub fn convert_to_code(seed: i64) -> Result<String, SeedError> {
        if !(0..TOTAL_SEEDS).contains(&seed) {
            return Err(SeedError::OutOfRange);
        }

        // Long.toString(seed, 26) → 0-9 then a-p
        let interim = to_string_radix_26(seed as u64);
        let mut result = String::new();

        for i in 0..9 {
            if i < interim.len() {
                let c = interim.as_bytes()[i] as char;
                let c = if c.is_ascii_digit() {
                    // convert 0-9 to A-J
                    (c as u8 + 17) as char
                } else {
                    // convert a-p to K-Z
                    (c as u8 - 22) as char
                };
                result.push(c);
            } else {
                // pad with A at front until length 9
                result.insert(0, 'A');
            }
        }

        // Insert dashes for readability at indices 3 and 7
        result.insert(3, '-');
        result.insert(7, '-');
        Ok(result)
    }

    /// Creates a seed from arbitrary user text input.
    pub fn convert_from_text(input_text: &str) -> Result<i64, SeedError> {
        if input_text.is_empty() {
            return Err(SeedError::Empty);
        }

        // First: try as seed code
        if let Ok(v) = Self::convert_from_code(input_text) {
            return Ok(v);
        }

        // Second: number (ignoring spaces), with overflow wrap via % TOTAL_SEEDS
        let no_spaces: String = input_text.chars().filter(|c| !c.is_whitespace()).collect();
        if let Ok(n) = no_spaces.parse::<i64>() {
            // Java Long.parseLong then % TOTAL_SEEDS; negative mods in Java toward zero for %
            // but Long.parseLong rejects leading + in older? Standard parse.
            let mut v = n % TOTAL_SEEDS;
            if v < 0 {
                v += TOTAL_SEEDS;
            }
            return Ok(v);
        }

        // Finally: string hash style (31 * total + c), then normalize
        let mut total: i64 = 0;
        for c in input_text.chars() {
            // Java char is UTF-16 code unit; for BMP this matches
            total = total.wrapping_mul(31).wrapping_add(c as u32 as i64);
        }
        if total < 0 {
            total = total.wrapping_add(i64::MAX);
        }
        total %= TOTAL_SEEDS;
        Ok(total)
    }

    /// Format input as code if it is a valid code; otherwise return as-is.
    pub fn format_text(input_text: &str) -> String {
        match Self::convert_from_code(input_text) {
            Ok(v) => Self::convert_to_code(v).unwrap_or_else(|_| input_text.to_string()),
            Err(_) => input_text.to_string(),
        }
    }
}

/// `Long.toString(n, 26)` equivalent for non-negative n.
fn to_string_radix_26(mut n: u64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    const DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut buf = Vec::new();
    while n > 0 {
        buf.push(DIGITS[(n % 26) as usize]);
        n /= 26;
    }
    buf.reverse();
    String::from_utf8(buf).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_zero() {
        let code = DungeonSeed::convert_to_code(0).unwrap();
        assert_eq!(code, "AAA-AAA-AAA");
        assert_eq!(DungeonSeed::convert_from_code(&code).unwrap(), 0);
    }

    #[test]
    fn round_trip_sample() {
        let seed = 123456789i64;
        let code = DungeonSeed::convert_to_code(seed).unwrap();
        assert_eq!(DungeonSeed::convert_from_code(&code).unwrap(), seed);
    }

    #[test]
    fn round_trip_max_minus_one() {
        let seed = TOTAL_SEEDS - 1;
        let code = DungeonSeed::convert_to_code(seed).unwrap();
        assert_eq!(code, "ZZZ-ZZZ-ZZZ");
        assert_eq!(DungeonSeed::convert_from_code(&code).unwrap(), seed);
    }

    #[test]
    fn from_code_strips_spaces() {
        assert_eq!(
            DungeonSeed::convert_from_code("ABC DEF GHI").unwrap(),
            DungeonSeed::convert_from_code("ABCDEFGHI").unwrap()
        );
    }

    #[test]
    fn from_text_numeric() {
        assert_eq!(DungeonSeed::convert_from_text("42").unwrap(), 42);
        assert_eq!(DungeonSeed::convert_from_text("4 2").unwrap(), 42);
    }

    #[test]
    fn from_text_fun_seed_deterministic() {
        let a = DungeonSeed::convert_from_text("hello").unwrap();
        let b = DungeonSeed::convert_from_text("hello").unwrap();
        assert_eq!(a, b);
        assert!((0..TOTAL_SEEDS).contains(&a));
    }

    #[test]
    fn convert_from_code_rejects_short() {
        assert!(DungeonSeed::convert_from_code("ABC").is_err());
    }

    #[test]
    fn many_round_trips() {
        for seed in [1, 26, 26 * 26, 999_999, 1_000_000_000, TOTAL_SEEDS / 2] {
            let code = DungeonSeed::convert_to_code(seed).unwrap();
            assert_eq!(
                DungeonSeed::convert_from_code(&code).unwrap(),
                seed,
                "seed {seed} code {code}"
            );
        }
    }
}
