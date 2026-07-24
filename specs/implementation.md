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
- Floor 3 matches room classes, bounds, and pre-paint RNG. Durable pre/post-door
  probes show Rust consumes four extra base-stream draws during room painting;
  door painting preserves the offset and the isolated painter tail is exact.
- Floor 7 still has exact local paint/mob boundaries, but its five main drops
  diverge because the earlier overall Generator category history differs.

## Next phase

Trace the four extra floor-3 room-paint draws room by room. Start with the final
two `TunnelRoom` painters, which consume two draws each in Rust, and compare
their `getDoorCenter` call lifecycle and connection iteration against pinned
Java. Identify the exact source-backed mismatch, remove it generally, then
advance floor 3 from exact pre-paint parity through pre-mobs and pre-items.

After floor 3 is exact, continue floor 4 and replay through floor 7. The target
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
