# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Register and match the two supplied HKT-JZN-XQQ floor-12 visual fixtures:
the full floor and Troll Blacksmith quest-room reference. Extend the fixture
contract as needed, fix only source-backed generation/render divergences,
update the accuracy manifest, run CI parity, save, commit, and stop. Then
resume floor 21 at its FigureEightBuilder divergence.

## Checkpoint

- Floor 21 preserves Java's floor-20 `CityBossLevel` generation lifecycle.
- ChasmRoom now uses pinned `{4,2,1}` size-category probabilities; floor-21
  non-connection room selection matches Java exactly.
- Rust still emits seven TunnelRooms where Java emits six, so bounds and the
  pre-paint RNG boundary remain divergent. Overall accuracy stays `partial`.
