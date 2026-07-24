# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-24

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- `AAA-AAA-AAA` floors 1–2 match through pre-items RNG; floor 2 also matches
  exact seeded heaps after porting GrassyGrave tombs and SecretChestChasm
  chests, keys, and terrain.
- Floor 3 matches through exact pre-mobs/pre-items RNG, including its regular
  exit at cell 237. `SacrificeRoom` now preserves `Room.center()` odd-span
  burns before its entrance-alignment nudge.
- Floor 7 still has exact local paint/mob boundaries, but its five main drops
  diverge because the earlier overall Generator category history differs.

## Next phase

Continue exact floor 4 and replay through floor 7. Compare durable boundaries
first, then port the earliest pinned source lifecycle mismatch. Do not remove
`TunnelRoom.getDoorCenter` burns: pinned Java unconditionally evaluates both
`Random.Float()` calls, and RingTunnel caches its second lookup. The target
floor-7 main drops remain:

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
