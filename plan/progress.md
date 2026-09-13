# v4 close-out progress

Index: [README.md](README.md). Facts: `specs/analysis/`. Decks: `specs/generator-decks.md`.

## Done

- **00–07 closed.** 07 is a deny-list; 00–06 shipped without porting it.
- Smoke/search/shop tests are retargeted to the v4 public projection. `spd-core --lib` currently has one unrelated compact-report expectation failure (`compact_report_promotes_exact_floor_one_room_rewards`); the remaining 406 tests pass. `68bb00c`.
- Stale v3 oracle assertions are aligned with v4 fixtures for GFX floor 3 mobs, AAA-AAD floor 1 seed presence, AAA floor 6 weapons, and AAA floor 14 ToxicGas vents.
- SecretLibrary's private weighted scroll pool follows the pinned v4 JVM HashMap order and weights; its GFX floor-6 replay now matches.
- CrystalVault and SentryRoom golden checks now derive their floor-6/floor-1 facts from the current v4 fixtures; no generation code changed.
- Halls inherited-Ruins merging matches Java for `RuinsRoom`, `RuinsEntranceRoom`, and `RuinsExitRoom`; AAA floor 23 and GFX floor 22 post-door traces pass.
- AAA floor 24 matches the preserved CrystalPath callback and halls paint trace, including its incoming POTION/SCROLL deck state.
- AAA floor 21's Eye cell difference is upstream trap-map parity: Java leaves cell 2198 clear while Rust places a Disintegration trap there, so changing mob eligibility would be incorrect.
- Halls trap selection is fully diagnosed for AAA floor 21: class order, RNG boundaries, candidate counts, and predicates match Java; only upstream painted-map candidate ordering differs, so no safe trap or mob edit is justified.
- The AAA-AFU floor-1 Runestone heap check now derives its cells and classes from the pinned v4 fixture instead of stale coordinates.
- The floor-one MagicalFire and hello TrapsRoom checks are also fixture-gated: the pinned v4 fixtures no longer contain the old hard-coded prize facts, so those assertions no longer report stale v3 failures.
- AAA floor 7 matches the pinned mob/item boundary and exact ambient-mob fixture. Its `StatueRoom.paint` canvas uses Java's wall, inset-empty, and statue-strip terrain; all 343 ambient-trap candidates, trap placement, and the downstream item count match without an RNG compensation.
- City floor 16 and floor 18 fixture checks pass. CrystalPath depth-one lifecycle and AAA/GFX/AFU floor-24 halls traces also pass; the ABC floor-24 RuinsExit callback remains a separate RNG drift.

## Now

Resolve the remaining deterministic identity and callback drifts after the repaired floor-7 population boundary.

## Next

1. Trace the AAA floor-17 POTION deck identities (Purity/Levitation versus Frost/Haste) with the matching RNG boundaries.
2. Reconcile the AAA floor-19 Laboratory Alchemy blob placement and visibility with Java.
3. Revisit the AAA floor-21 trap-map difference before changing Eye eligibility.
4. Trace the ABC floor-24 RuinsExit room callback RNG drift.

## Remaining

`java_oracle_goldens`: 41 pass, 4 real failures remain. The remaining failures are AAA floor 17 potion identities, AAA floor 19 Laboratory Alchemy blob placement, AAA floor 21 trap-map parity, and the ABC floor-24 RuinsExit callback. `spd-core --lib`: 406 pass, 1 unrelated compact-report expectation failure. Analyzer remains `partial`.
