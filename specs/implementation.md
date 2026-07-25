# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Accuracy:** `partial`; `specs/accuracy.json` is authoritative.

## Goal

Expose only seed-invariant facts. Keep runtime/history-sensitive sampled values
internal for Java parity, and publish only proven constraints plus explicitly
conditional player-state effects. Apply this to reports, maps, WASM, search,
and UI.

## Next phase

1. Audit the initial forced-item queue: identity, existence, quantity, and later
   room consumption must each be proven seed-only before public exposure.
2. Resolve createItems grass flattening with either pre-items terrain provenance
   or conservative cell/map suppression.
3. Audit remaining uncovered room/lifecycle loot and the secret-room queue's
   main-path assumption.

Add altered-history regressions for every retained field. Do not restore
partial rendered maps from painter snapshots: later doors, decoration, mobs,
and items can rewrite earlier cells. Per-cell finality provenance is required.

## Checkpoint

- Sacrifice, shop, quest, standard, special, crystal, and secret-room rewards
  retain exact internal parity but use seed-safe public constraints.
- Runtime-sensitive painter callbacks suppress the affected item/quest tail and
  whole rendered map; sampled heap cells and concrete markers are sanitized.
- Rare Artifact shop generation occurs before layout completion, so its public
  floor omits builder, room list, static room facts, map, and downstream facts.
- Depths 1–2 omit public maps because intro/guidebook history changes entrance
  door visibility. Exact internal maps remain available to parity tests.
- Fixed-shape sensitive callbacks may retain sanitized maps; exact searches
  ignore constrained predictions.
- Overall status remains `partial`; do not claim full accuracy.
