# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Accuracy:** `partial`; `specs/accuracy.json` is authoritative.

## Goal

Expose only seed-invariant facts. Keep runtime/history-sensitive sampled values
internal for Java parity, and publish only proven constraints plus explicitly
conditional player-state effects. Apply this to reports, maps, WASM, search,
and UI.

## Next phase

1. Audit remaining lifecycle branches, including direct drops, Spyglass taint,
   and boss-floor Bones behavior.
2. Audit Cached Rations, Dried Rose petals, Darkness torches, and guide/meta
   items for runtime-sensitive existence and RNG effects.

Add altered-history regressions for every retained field. Do not restore
partial rendered maps from painter snapshots: later doors, decoration, mobs,
and items can rewrite earlier cells. Per-cell finality provenance is required.

## Checkpoint

- Sacrifice, shop, quest, standard, special, crystal, and secret-room rewards
  retain exact internal parity but use seed-safe public constraints.
- Initial `itemsToSpawn` state remains exact internally. Public output reports
  only initial-queue contracts: constrained food categories, invariant limited
  schedules, Forbidden Runes conditionality, and one aggregate contract for the
  two Halls Torches queued before food; it never claims survival or a final
  heap/cell.
- Secret Library uses its pinned private weighted scroll pool and exact
  placement retries without touching `itemsToSpawn` or Generator; sampled base
  identity remains internal and Exotic Crystals is public only as a condition.
- Runtime-sensitive painter callbacks suppress the affected item/quest tail and
  sampled facts; every regular public map is now omitted pending per-cell
  finality proof. Exact internal maps remain available to oracle fixtures.
- Trinket-sensitive default feelings, even Scroll schedules, artifact history,
  divergent room callbacks, and Wandmaker tails permanently taint later sampled
  public facts while leaving independently proven queue contracts visible.
- Rare Artifact shop generation occurs before layout completion, so its public
  floor omits builder, room list, static room facts, map, and downstream facts.
- Sacrifice exposes only a cursed weapon from the one-higher floor-set tier
  distribution; sampled tier/class/level/enchantment remain internal.
- Exact searches ignore constrained queue/reward contracts.
- Overall status remains `partial`; do not claim full accuracy.
