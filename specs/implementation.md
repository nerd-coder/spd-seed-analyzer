# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Resume floor 21 at its FigureEightBuilder divergence. Compare Java and Rust
connection placement from the exact post-room-selection boundary; fix only a
source-backed call-order or collection-order mismatch, then carry parity
through painting and final map facts.

## Checkpoint

- HKT-JZN-XQQ floor 12 now has a deterministic browser baseline matching the
  supplied regular-floor geometry and BlacksmithRoom placement.
- Its `_Q` image is the separate branch-1 `MiningLevel`; it is registered as a
  source-only quest-branch reference because branch maps are not analyzed.
- Floor 21 room selection matches Java, but Rust still emits seven TunnelRooms
  where Java emits six. Overall accuracy stays `partial`.
