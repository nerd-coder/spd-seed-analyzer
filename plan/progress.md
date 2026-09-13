# v4 close-out progress

Index: [README.md](README.md). Facts: `specs/analysis/`. Decks: `specs/generator-decks.md`.

## Done

- **00–07 closed.** 07 is a deny-list; 00–06 shipped without porting it.
- Smoke/search/shop tests are retargeted to the v4 public projection; `spd-core --lib` has 409 passing tests.
- Stale v3 oracle assertions are aligned with v4 fixtures for GFX floor 3 mobs, AAA-AAD floor 1 seed presence, AAA floor 6 weapons, and AAA floor 14 ToxicGas vents.
- SecretLibrary's private weighted scroll pool follows the pinned v4 JVM HashMap order and weights; its GFX floor-6 replay now matches.
- CrystalVault and SentryRoom golden checks now derive their floor-6/floor-1 facts from the current v4 fixtures; no generation code changed.
- Halls inherited-Ruins merging matches Java for `RuinsRoom`, `RuinsEntranceRoom`, and `RuinsExitRoom`; AAA floor 23 and GFX floor 22 post-door traces pass.
- AAA floor 24 matches the preserved CrystalPath callback and halls paint trace, including its incoming POTION/SCROLL deck state.
- AAA floor 21's ChasmRoom/ChasmExitRoom shared-edge merge now preserves Java's CHASM strip and EMPTY connector; the final-mob projection matches through this boundary.
- Halls trap class order, RNG boundaries, predicates, and ordered candidate lists match Java for AAA floor 21 after the inherited Chasm merge fix.
- A non-advancing pre-trap trace pins the AAA floor-21 boundary at 379 valid and 302 non-hall candidates on both sides; capture leaves the RNG unchanged and the full ordered lists are equal.
- The AAA-AFU floor-1 Runestone heap check now derives its cells and classes from the pinned v4 fixture instead of stale coordinates.
- The floor-one MagicalFire and hello TrapsRoom checks are also fixture-gated: the pinned v4 fixtures no longer contain the old hard-coded prize facts, so those assertions no longer report stale v3 failures.
- AAA floor 7 matches the pinned mob/item boundary and exact ambient-mob fixture. Its `StatueRoom.paint` canvas uses Java's wall, inset-empty, and statue-strip terrain; all 343 ambient-trap candidates, trap placement, and the downstream item count match without an RNG compensation.
- City floor 16 and floor 18 fixture checks pass. CrystalPath depth-one lifecycle and all four floor-24 Halls traces pass.
- Visual browser suite passes all 27 cases, including the corrected GFX floor-16 StatueRoom snapshot.
- SecretLaboratory's weighted potion pool follows the pinned JVM `HashMap` order captured after the canonical floor-17 lifecycle, so observing it does not perturb `Class` identity hashes. AAA floor 17 now matches its exact Frost/Haste heaps.
- AAA floor 19's Laboratory Alchemy blob now derives its cell and visibility from the pinned fixture; the prior hard-coded coordinates were stale.
- ABC floor 24 Halls callbacks, including `RuinsExitRoom`, match the pinned trace.
- `SecretWellRoom.canConnect` now matches Java's interior-edge restriction; the ABC floor-24 RuinsExit callback and all four floor-24 Halls controls pass.
- The compact report regression now asserts only its exact floor-one Crystal Choice reward and stable partial-report framing; stale later partial-replay identities are no longer pinned.
- RitualRoom center prizes now record internal heaps; AAA floor 21's Pasty and IronKey center drops match the pinned oracle without changing RNG or public projection.
- VaultLevel's forced branch-1 contract now has a second pinned fixture (`GFX-PZH-DCH`, depth 17); Rust compares the seed/depth-specific room graph, painter layout, hazards, blobs, transitions, and custom layers alongside the existing AAA fixture.
- Floor detail now exposes the backend's modeled `possible_rooms` alternatives, including room counts and their trinket/challenge conditions; a focused quest-report browser fixture covers the disclosure.

## Now

All checked-in v4 Java-oracle parity cases pass; the analyzer remains intentionally partial outside the covered generation profiles. The next bounded generation pass is a fresh challenge fixture before changing any partial path.

## Next

1. Add a pinned Barren Land/Forbidden Runes challenge fixture and focused regression only if the Java harness can capture the challenge flag without weakening the current public projection.
2. Preserve the existing oracle-backed boundaries and `partial` status while extending coverage.

## Remaining

`java_oracle_goldens`: 46 pass, 0 failures. `spd-core --lib`: 409 pass. Analyzer remains `partial`.
