# v4 close-out progress

Index: [README.md](README.md). Facts: `specs/analysis/`. Decks: `specs/generator-decks.md`.

## Done

- **00–07 closed.** 07 is a deny-list; 00–06 shipped without porting it.
- `spd-core --lib` is green (405). Smoke/search/shop tests retargeted to the v4 public projection. `68bb00c`.
- Stale v3 oracle assertions are aligned with v4 fixtures for GFX floor 3 mobs, AAA-AAD floor 1 seed presence, AAA floor 6 weapons, and AAA floor 14 ToxicGas vents.
- SecretLibrary's private weighted scroll pool follows the pinned v4 JVM HashMap order and weights; its GFX floor-6 replay now matches.

## Now

Fix reproducible v4 Java-oracle parity failures, one verified slice per commit.

## Next

1. Fix the AAA floor-24 Halls callback offset exposed after source-correct Ruins merging.
2. Isolate the AAA floor-21 mob-cell eligibility difference.
3. Fix AAA floor 6 CrystalVault replay and AAA-AAZ floor 1 SentryRoom placement.
4. Resolve city floors 16–19 and main-loop heap parity.

## Remaining

`java_oracle_goldens`: 35 pass, 10 real failures remain across the work above. Analyzer still `partial`.
