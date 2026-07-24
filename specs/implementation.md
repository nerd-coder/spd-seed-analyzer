# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-24

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- `AAA-AAA-AAA` floor 7 matches all five main Generator drops exactly, including
  cells, heap types, classes, quantities, levels, and curse state.
- The earliest known preserved-run item mismatch is floor 6's private weapon
  decks: overall categories and placement cells match, but weapon classes do not.
- Isolated WEP_T4 draws match pinned Java, so the floor-6 replay discrepancy must
  be localized with lifecycle-boundary deck snapshots before changing behavior.

## Next phase

Add pinned Java and Rust Generator snapshots spanning floor 3's SacrificeRoom
through floor 6 `createItems`. Compare WEP_T2/WEP_T4 seeds, dropped counts, and
probability vectors at each lifecycle boundary; port the earliest proven mismatch.
Pin the corrected floor-6 main and shop weapon classes in the replay fixture.

## Known limits

- Uncovered room painters and Generator histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are out of scope.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
