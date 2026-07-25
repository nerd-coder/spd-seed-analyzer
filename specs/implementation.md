# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Instrument the pinned Java oracle and Rust FigureEightBuilder with matching
per-attempt traces: start/end RNG probes, failure stage, room insertion order,
and bounds. Use the first differing attempt boundary to make only a
source-backed retry, placement, or collection-order fix; then carry floor 21
parity through painting and final map facts.

## Checkpoint

- HKT-JZN-XQQ floor 12 now has a deterministic browser baseline matching the
  supplied regular-floor geometry and BlacksmithRoom placement.
- Its `_Q` image is the separate branch-1 `MiningLevel`; it is registered as a
  source-only quest-branch reference because branch maps are not analyzed.
- Floor 21 room selection matches Java, but Rust still emits seven TunnelRooms
  where Java emits six. Overall accuracy stays `partial`.
- Rust attempt 0 fails with 15 rooms and attempt 1 succeeds with 17. Java's
  pre-paint RNG state is 297 integer draws later; forcing attempt 1 to fail
  makes Rust attempt 2 succeed with 18 rooms and does not converge. Existing
  fixtures therefore cannot justify suppressing a retry or changing geometry.
