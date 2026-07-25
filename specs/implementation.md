# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Accuracy:** `partial`; `specs/accuracy.json` is authoritative.

## Goal

Expose only seed-invariant facts. Keep runtime/history-sensitive sampled values
internal for Java parity, and publish only proven constraints plus explicitly
conditional player-state effects. Apply this to reports, maps, WASM, search,
and UI.

## Next phase

1. Resolve createItems grass flattening with either pre-items terrain provenance
   or conservative cell/map suppression.
2. Audit direct lifecycle drops after the queue (Darkness torches, Bones,
   Dried Rose petals, Cached Rations, guide/meta items).
3. Audit remaining uncovered room/lifecycle loot and the secret-room queue's
   main-path assumption.

Add altered-history regressions for every retained field. Do not restore
partial rendered maps from painter snapshots: later doors, decoration, mobs,
and items can rewrite earlier cells. Per-cell finality provenance is required.

## Checkpoint

- Sacrifice, shop, quest, standard, special, crystal, and secret-room rewards
  retain exact internal parity but use seed-safe public constraints.
- Initial `itemsToSpawn` state remains exact internally. Public output reports
  only initial-queue contracts: constrained food categories, invariant limited
  schedules, and Forbidden Runes conditionality; it never claims survival or a
  final heap/cell.
- Runtime-sensitive painter callbacks suppress the affected item/quest tail and
  whole rendered map; sampled heap cells and concrete markers are sanitized.
- Trinket-sensitive default feelings, even Scroll schedules, artifact history,
  divergent room callbacks, and Wandmaker tails permanently taint later sampled
  public facts while leaving independently proven queue contracts visible.
- Rare Artifact shop generation occurs before layout completion, so its public
  floor omits builder, room list, static room facts, map, and downstream facts.
- Depths 1–2 omit public maps because intro/guidebook history changes entrance
  door visibility. Exact internal maps remain available to parity tests.
- Sacrifice exposes only a cursed weapon from the one-higher floor-set tier
  distribution; sampled tier/class/level/enchantment remain internal.
- Exact searches ignore constrained queue/reward contracts.
- Overall status remains `partial`; do not claim full accuracy.
