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
- AAA floor 21's Eye cell difference is upstream trap-map parity: Java leaves cell 2198 clear while Rust places a Disintegration trap there, so changing mob eligibility would be incorrect; no safe painter fix is identified.
- Halls trap selection is fully diagnosed for AAA floor 21: class order, RNG boundaries, candidate counts, and predicates match Java; only upstream painted-map candidate ordering differs, so no safe trap or mob edit is justified.
- The AAA-AFU floor-1 Runestone heap check now derives its cells and classes from the pinned v4 fixture instead of stale coordinates.
- The floor-one MagicalFire and hello TrapsRoom checks are also fixture-gated: the pinned v4 fixtures no longer contain the old hard-coded prize facts, so those assertions no longer report stale v3 failures.
- AAA floor 7 matches the pinned mob/item boundary and exact ambient-mob fixture. Its `StatueRoom.paint` canvas uses Java's wall, inset-empty, and statue-strip terrain; all 343 ambient-trap candidates, trap placement, and the downstream item count match without an RNG compensation.
- City floor 16 and floor 18 fixture checks pass. CrystalPath depth-one lifecycle and AAA/GFX/AFU floor-24 halls traces also pass; the ABC floor-24 RuinsExit callback remains a separate RNG drift.
- Visual browser suite passes all 27 cases, including the corrected GFX floor-16 StatueRoom snapshot.
- SecretLaboratory's weighted potion pool follows the pinned JVM `HashMap` order captured after the canonical floor-17 lifecycle, so observing it does not perturb `Class` identity hashes. AAA floor 17 now matches its exact Frost/Haste heaps.
- AAA floor 19's Laboratory Alchemy blob now derives its cell and visibility from the pinned fixture; the prior hard-coded coordinates were stale.

## Now

Trace the ABC floor-24 RuinsExit room callback RNG drift.

## Next

1. Keep the AAA floor-21 trap-map mismatch bounded to the painted-map difference; do not change Eye eligibility without an upstream map proof.
2. Trace the ABC floor-24 RuinsExit room callback RNG drift.

## Remaining

`java_oracle_goldens`: 44 pass, 2 real failures remain. The remaining failures are AAA floor 21 trap-map parity and the ABC floor-24 RuinsExit callback. `spd-core --lib`: 406 pass, 1 unrelated compact-report expectation failure. Analyzer remains `partial`.
