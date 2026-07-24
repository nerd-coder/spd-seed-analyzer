# SPD Seed Analyzer — Handoff

**Updated:** 2026-07-25

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port generation only
from the pinned checkout at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Next phase

Resolve floor-11 GuidePage, CrystalKey, and main/forced-drop placement while
preserving the exact pre-paint, pre-mobs, and pre-items RNG boundaries. Current
example: Rust GuidePage cell 1687 vs Java cell 870. Compare the nested
`randomDropCell` room-order mutation and occupied-cell state first.

## Checkpoint

- Accuracy details now use a responsive, accessible modal with a bounded
  two-axis table scroller; mobile overflow, Escape close, and focus return are
  covered by Playwright.
- Floor 11 has exact RNG boundaries, mobs, and SecretHoardRoom heaps, but not
  all additive heaps.

## Constraints discovered

- Do not globally reorder equal-score shop bags. Pinned `ChooseBag` uses
  identity-hash `HashMap` iteration: equivalent fresh inventories choose
  `MagicalHolster` in the HKT oracle and `PotionBandolier` in the AAA oracle.
  This is non-seeded JVM behavior, not a source-faithful accuracy fix.
- Player inventory, Hourglass sandbags, VaultLevel branches, and the unseeded
  early Guidebook page remain outside general exact prediction.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
