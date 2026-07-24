# SPD Seed Analyzer — Accuracy Handoff

**Updated:** 2026-07-24

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — do not claim full seed-finder accuracy

`specs/accuracy.json` is the coverage source of truth. Port only from the
pinned checkout, normally at
`/Users/toan/code/repos/00-Evan/shattered-pixel-dungeon`.

## Checkpoint

- `AAA-AAA-AAA` is the second preserved-state replay, with complete Java facts
  and exact room bounds pinned for floors 6–9. It broadens coverage with Vault,
  Larder, Sentry, Treasury, WeakFloor, Armory, Garden, Library, Pit, Aquarium,
  and Burned room families.
- Its first divergence, `CrystalVaultRoom`, is source-ported through geometry,
  locked entrance, pedestals, chest cells, and exact item association. Floor 6
  now reaches the mob/items lifecycle boundaries; floor 7 diverges during paint.
- Status remains `partial`; fixtures beyond that boundary are evidence and next
  work, not a claim of full replay parity.

## Next phase

Continue `AAA-AAA-AAA` floor 7 from its pinned pre-paint boundary:

1. Identify the first painter/RNG divergence without changing later families.
2. Port only that room path from pinned SPD and strengthen the replay assertion.
3. Then convert another fixture-backed family that still emits `Room loot`.

## Known limits

- Uncovered special/secret geometry and deeper histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts for
  `AAA-AAA-ACB`.
- VaultLevel branches and player-dependent later-shop bag selection are out of
  the current regular-floor contract.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
