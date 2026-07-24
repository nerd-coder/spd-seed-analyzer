# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-24

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- `AAA-AAA-AAA` preserves Java-oracle floors 6–9 with exact room bounds;
  floor 6 matches through pre-items.
- Floor-7 `LibraryRoom.paint` terrain and terrain-aware drop rejection are
  source-ported. Its persistent wall/bookshelf geometry matches the fixture.
- The Library prize roll exposes the first remaining divergence: Rust enters
  with the wrong RNG state and drops one Remove Curse scroll at cell 2121;
  Java drops Identify at 2071 and Upgrade at 2121. Status remains `partial`.

## Next phase

Trace the floor-7 painter RNG boundary immediately before `LibraryRoom`:

1. Add a pinned Java/Rust probe after `RegionDecoLineExitRoom` and before
   `LibraryRoom` (paint order places Library after that room).
2. Find the earliest preceding painter boundary that differs; port only that
   room's missing RNG/call-order behavior.
3. Re-enable exact Library prize parity, then re-check pre-mobs RNG and stop at
   the next first divergence.

## Known limits

- Uncovered room painters and deeper histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are outside the
  current regular-floor contract.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
