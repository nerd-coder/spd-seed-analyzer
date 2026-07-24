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
- Floor 6 matches through `CrystalVaultRoom` and the mob/items boundaries.
  Floor 7 now matches its first divergent `GardenRoom` through exact terrain
  and planted cell; the next divergence is `ArmoryRoom` later in painter order.
- Status remains `partial`; fixtures beyond that boundary are evidence and next
  work, not a claim of full replay parity.

## Next phase

Continue `AAA-AAA-AAA` floor 7 from the verified Garden boundary:

1. Port pinned `ArmoryRoom.paint`, including carpet/statue geometry and exact
   equipment placement/RNG order.
2. Extend the replay assertion to the Armory bounds and re-check pre-mobs RNG.
3. Then continue to the next first divergence only.

## Known limits

- Uncovered special/secret geometry and deeper histories may diverge.
- ToxicGas vents/gas blobs are not exact exported additive facts for
  `AAA-AAA-ACB`.
- VaultLevel branches and player-dependent later-shop bag selection are out of
  the current regular-floor contract.
- The unseeded early Guidebook page is intentionally out of scope.

Before committing, run the complete CI `check` sequence from `AGENTS.md`.
