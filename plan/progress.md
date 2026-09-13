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
- AAA floor-7 replay's planted-cell, Armory, and Library checks now derive from the pinned v4 terrain/heaps. Its first real divergence is SegmentedRoom terrain, which changes ambient mob placements/count before `pre_items_rng`.
- City replay enters floor 16 with the general category deck one GOLD weight behind Java while the STONE deck matches. Floor 16 consequently consumes StoneOfAggression and later heap generation drifts.

## Now

Fix the AAA floor-7 SegmentedRoom painter and locate the first pre-floor-16 GOLD category divergence.

## Next

1. Align SegmentedRoom painted terrain, then rerun the AAA floor-7 replay through mobs and heaps.
2. Trace the general category deck across prior floors to find the first missing GOLD consumption before city floor 16.
3. Recheck city floors 16–19 and CrystalPath after their incoming Generator state matches Java.

## Remaining

`java_oracle_goldens`: 36 pass, 9 real failures remain across the work above; the CrystalVault, Sentry, MagicalFire, TrapsRoom, Runestone, and AAA floor-7 planted/Armory/Library stale checks are removed or retargeted, while their enclosing replay tests still stop at earlier parity failures. Analyzer still `partial`.
