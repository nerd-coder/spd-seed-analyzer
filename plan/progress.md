# v4 close-out progress

Index: [README.md](README.md). Facts: `specs/analysis/`. Decks: `specs/generator-decks.md`.

## Done

- **00–07 closed.** 07 is a deny-list; 00–06 shipped without porting it.
- Smoke/search/shop tests are retargeted to the v4 public projection; `spd-core --lib` has 412 passing tests.
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
- Barren Land/NO_HERBALISM is pinned as a separate depth-4 `SecretGardenRoom` fixture; seed/position RNG and terrain conversion remain intact while plant objects, occupancy, and synthetic plant loot are suppressed exactly like Java.
- Rat Skull level-3 profile replay is pinned at AAA floors 6 and 7; its full 1/50 mob/elemental/Piranha multiplier and half-effective CrystalVault/Statue branch match the Java oracle without changing RNG probes.
- Forbidden Runes is pinned at AAA floor 3. The Java oracle carries the `NO_SCROLLS` challenge metadata, and the profiled Rust replay preserves room/map/RNG/heaps while omitting the second scheduled Upgrade Scroll (`[0, 1, 0]` across floors 1–3).
- Badder Bosses is pinned at AAA floor 15. `STRONGER_BOSSES` now drives the Caves boss inactive-trap roll (`1/4` instead of `1/8`); the profiled Rust replay matches the Java terrain, discoverability, transitions, and pre-items RNG boundary while keeping boss mobs runtime-only.

## Now

All checked-in v4 Java-oracle parity cases pass; the analyzer remains intentionally partial outside the covered generation profiles. The next bounded generation pass must start from a fresh fixture before changing another partial path.

## Next

1. Preserve the existing oracle-backed boundaries and `partial` status while extending coverage.
2. Add another challenge or trinket fixture only after its Java contract and RNG boundary are isolated.

## Remaining

`java_oracle_goldens`: 53 pass, 0 failures. `spd-core --lib`: 413 pass. Analyzer remains `partial`.
