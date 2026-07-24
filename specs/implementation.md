# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-24

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- `AAA-AAA-AAA` floors 6–9 preserve exact room bounds; floor 7 preserves the
  exact pre-items RNG boundary, mobs, and PitRoom skeleton heap.
- Concrete `Generator.undoDrop` calls now match pinned Java's effective no-op.
- Floor-7 main drops still diverge because earlier floors consume a different
  sequence of overall Generator categories.

## Next phase

Trace `AAA-AAA-AAA` floors 2–4 from Java-oracle fixtures and reconcile the first
missing room-paint or loot lifecycle that changes `Generator.random()` category
consumption. Add a boundary fixture at the earliest divergence, then restore
the five floor-7 main drops without seed-specific state injection.

Pinned floor-7 target:

- `ThrowingSpear(3)` at 281, heap
- `ScrollOfIdentify` at 662, heap
- `ScrollOfLullaby` at 812, heap
- `Gold(156)` at 997, heap
- `PotionOfPurity` at 2131, chest

## Known limits

- Uncovered room painters and earlier generator histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are out of scope.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
