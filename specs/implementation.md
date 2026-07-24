# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-24

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- `AAA-AAA-AAA` preserves Java-oracle floors 6–9 with exact room bounds.
- Floor 6 matches through `CrystalVaultRoom`, including pre-mobs/pre-items RNG.
- Floor 7 is pinned through `ArmoryRoom`: boundary/statue geometry, locked
  entrance, exact prize cells/items, and `DoubleBomb` quantity match SPD.
- The floor-7 pre-mobs probe still diverges. `LibraryRoom` is the next painter
  gap. Status remains `partial`.

## Next phase

Continue `AAA-AAA-AAA` floor 7 at `LibraryRoom`:

1. Port pinned `LibraryRoom.paint` terrain and `drawInside` geometry.
2. Record exact prize heap placement and preserve RNG/call order.
3. Extend the replay assertion through Library, re-check pre-mobs RNG, and stop
   at the next first divergence.

## Known limits

- Uncovered room painters and deeper histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are outside the
  current regular-floor contract.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
