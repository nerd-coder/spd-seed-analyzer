# v4 close-out progress

Index: [README.md](README.md). Facts: `specs/analysis/`. Decks: `specs/generator-decks.md`.

## Done

- **00–07 closed.** 07 is a deny-list; 00–06 shipped without porting it.
- `spd-core --lib` is green (405). Smoke/search/shop tests retargeted to the v4 public projection. `68bb00c`.
- Stale v3 oracle assertions are aligned with v4 fixtures for GFX floor 3 mobs, AAA-AAD floor 1 seed presence, AAA floor 6 weapons, and AAA floor 14 ToxicGas vents.
- SecretLibrary's private weighted scroll pool follows the pinned v4 JVM HashMap order and weights; its GFX floor-6 replay now matches.
- CrystalVault and SentryRoom golden checks now derive their floor-6/floor-1 facts from the current v4 fixtures; no generation code changed.
- Halls inherited-Ruins merging matches Java for `RuinsRoom`, `RuinsEntranceRoom`, and `RuinsExitRoom`; AAA floor 23 and GFX floor 22 post-door traces pass.
- AAA floor 24 first diverges inside the CrystalPath callback. Its incoming POTION/SCROLL deck state already differs from Java, causing different retry/conversion consumption; a local compensating draw would be incorrect.
- AAA floor 21's Eye cell difference is upstream trap-map parity: Java leaves cell 2198 clear while Rust places a Disintegration trap there, so changing mob eligibility would be incorrect.
- Halls trap selection is fully diagnosed for AAA floor 21: class order, RNG boundaries, candidate counts, and predicates match Java; only upstream painted-map candidate ordering differs, so no safe trap or mob edit is justified.
- The AAA-AFU floor-1 Runestone heap check now derives its cells and classes from the pinned v4 fixture instead of stale coordinates.
- The floor-one MagicalFire and hello TrapsRoom checks are also fixture-gated: the pinned v4 fixtures no longer contain the old hard-coded prize facts, so those assertions no longer report stale v3 failures.
- AAA floor-7 replay's planted-cell, Armory, and Library checks now derive from the pinned v4 terrain/heaps. The one-draw `pre_items_rng` shift occurs during ambient mob creation after an identical `pre_mobs_rng` boundary; Rust places the same ten ambient mobs but diverges in candidate/placement consumption, with no safe compensating draw identified. SegmentedRoom geometry/RNG matches Java, and its remaining tile differences are decoration variants.
- City replay's first persistent overall-category drift is floor 7. Java's six main-loop `Generator.random()` calls consume POTION, SCROLL, and four GOLD weights; Rust's shifted `pre_items_rng` rolls five items and consumes two POTION and three GOLD weights. The missing sixth `RegularLevel.createItems` iteration explains the one-GOLD deficit through floor 15; the STONE deck still matches before floor 16.

## Now

Locate the first candidate/placement RNG divergence in AAA floor-7 ambient mobs; the downstream GOLD/category and city drift are now accounted for.

## Next

1. Trace AAA floor-7 ambient mob candidate acceptance against Java after the matching `pre_mobs_rng` boundary; do not compensate for decoration-only tile differences.
2. Recheck the floor-7 item count, city floors 16–19, and CrystalPath after the mob boundary matches Java.

## Remaining

`java_oracle_goldens`: 36 pass, 9 real failures remain across the work above; the CrystalVault, Sentry, MagicalFire, TrapsRoom, Runestone, and AAA floor-7 planted/Armory/Library stale checks are removed or retargeted, while their enclosing replay tests still stop at earlier parity failures. Analyzer still `partial`.
