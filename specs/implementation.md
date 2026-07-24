# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-24

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- `AAA-AAA-AAA` preserves exact room bounds on floors 6–9 and exact floor-7
  pre-mobs/pre-items RNG plus mobs.
- Floor 7's PitRoom now matches its pinned central skeleton stack exactly:
  `CrystalKey`, `Gold(162)`, `UnstableSpellbook`. The key is dropped into that
  heap, not deferred through `itemsToSpawn`.
- Overall accuracy remains `partial`.

## Next phase

Continue `AAA-AAA-AAA` floor-7 item parity from the exact pre-items boundary:

1. Reconcile Generator category/deck state entering `createItems`; Rust's five
   main drops begin with scroll categories where pinned SPD begins with a
   missile.
2. Match the five main item classes, heap types, and cells without regressing
   floor-6 fixtures, the PitRoom stack, or floor-7 mobs.

## Known limits

- Uncovered room painters and deeper histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts.
- VaultLevel branches and player-dependent later-shop bags are outside the
  current regular-floor contract.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
