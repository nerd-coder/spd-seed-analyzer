# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-25

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- `AAA-AAA-AAA` weapon-deck state now matches pinned Java from floor 3's
  SacrificeRoom through floor 6 `createItems`.
- SacrificeRoom uses the actual painted entrance door, restoring Java's center
  nudge, WEP_T2 Spear prize, and downstream floor-6 Quarterstaff, Crossbow, and
  Katana classes.
- Floor 7's five main Generator drops remain exact.

## Next phase

Find the earliest remaining `AAA-AAA-AAA` heap or item mismatch after floor 7.
Compare floor 8, then floor 9, at room-paint, shop, mobs, and `createItems`
boundaries. Add a pinned Java fixture for the first divergence and port only the
earliest proven RNG or generation mismatch.

## Known limits

- Uncovered room painters and Generator histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are out of scope.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
