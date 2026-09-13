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
- AAA floor 21's Eye cell difference is upstream trap-map parity: Java leaves cell 2198 clear while Rust places a Disintegration trap there, so changing mob eligibility would be incorrect; no safe painter fix is identified.
- Halls trap class order, RNG boundaries, and predicates match Java for AAA floor 21; the upstream painted-map candidate counts and ordering do not, so no trap or mob edit is justified.
- A non-advancing pre-trap trace pins the AAA floor-21 boundary: Java has 379 valid/302 non-hall candidates and Rust has 389/316; both RNG probes match, and the valid lists first differ at index 229 where Rust contributes ChasmRoom cell 891 while Java next contributes cell 1131.
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

## Now

One upstream parity boundary remains: AAA floor 21's ChasmRoom terrain contributes a different pre-trap candidate list, which changes the final Eye cell.

## Next

1. Isolate why Rust leaves ChasmRoom cell 891 trap-eligible while Java excludes it before trap selection.
2. Keep the mismatch bounded to ChasmRoom terrain/painting; do not change trap selection or Eye eligibility without an upstream map proof.

## Remaining

`java_oracle_goldens`: 45 pass, 1 real failure remains: AAA floor-21 trap-map/final-mob parity. `spd-core --lib`: 409 pass. Analyzer remains `partial`.
