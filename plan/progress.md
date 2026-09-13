# v4 close-out progress

Index: [README.md](README.md). Facts: `specs/analysis/`. Decks: `specs/generator-decks.md`.

## Done

- **00–07 closed.** 07 is a deny-list; 00–06 shipped without porting it.
- `spd-core --lib` is green (405). Smoke/search/shop tests retargeted to the v4 public projection. `68bb00c`.
- Stale v3 oracle assertions are aligned with v4 fixtures for GFX floor 3 mobs, AAA-AAD floor 1 seed presence, AAA floor 6 weapons, and AAA floor 14 ToxicGas vents.
- SecretLibrary's private weighted scroll pool follows the pinned v4 JVM HashMap order and weights; its GFX floor-6 replay now matches.
- CrystalVault and SentryRoom golden checks now derive their floor-6/floor-1 facts from the current v4 fixtures; no generation code changed.
- Halls inherited-Ruins merging matches Java for `RuinsRoom`, `RuinsEntranceRoom`, and `RuinsExitRoom`; AAA floor 23 and GFX floor 22 post-door traces pass.
- AAA floor 24's remaining CrystalPath callback shift has no safe fix from geometry or constructor call counts; reward/deck draw instrumentation is required before changing generation.
- AAA floor 21's Eye cell difference is upstream trap-map parity: Java leaves cell 2198 clear while Rust places a Disintegration trap there, so changing mob eligibility would be incorrect.

## Now

Fix reproducible v4 Java-oracle parity failures, one verified slice per commit.

## Next

1. Instrument CrystalPath reward/deck draws to isolate the AAA floor-24 callback shift.
2. Trace Halls trap painter candidate parity before revisiting AAA floor-21 mob placement.
3. Resolve city floors 16–19 and main-loop heap parity.

## Remaining

`java_oracle_goldens`: 35 pass, 10 real failures remain across the work above; the CrystalVault and Sentry stale checks are removed but their enclosing replay tests still stop at earlier parity failures. Analyzer still `partial`.
