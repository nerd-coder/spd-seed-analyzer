# v4 close-out progress

Index: [README.md](README.md). Facts: `specs/analysis/`. Decks: `specs/generator-decks.md`.

## Done

- **00–07 closed.** 07 is a deny-list; 00–06 shipped without porting it.
- `spd-core --lib` is green (405). Smoke/search/shop tests retargeted to the v4 public projection. `68bb00c`.
- Stale v3 oracle assertions are aligned with v4 fixtures for GFX floor 3 mobs, AAA-AAD floor 1 seed presence, AAA floor 6 weapons, and AAA floor 14 ToxicGas vents.

## Now

Fix reproducible v4 Java-oracle parity failures, one verified slice per commit.

## Next

1. Fix GFX floor 6 SecretLibrary weighted order against the pinned copied-HashMap fixture.
2. Fix AAA floor 6 CrystalVault replay and AAA-AAZ floor 1 SentryRoom placement.
3. Resolve city floors 16–19 and main-loop heap parity.
4. Resolve halls floor 21–24 mob, door, and paint RNG parity.

## Remaining

`java_oracle_goldens`: 33 pass, 12 real failures remain across the work above. Analyzer still `partial`.
