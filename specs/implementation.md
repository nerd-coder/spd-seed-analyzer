# SPD Seed Analyzer — Handoff

**Updated:** 2026-07-25

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Continue the AAA-AAA-AAA floor-12 FigureEightBuilder trace immediately after
the now-aligned incremental collision-list behavior. Compare each remaining
`placeRoom` result, closing stitch, and branch retry on the first build attempt;
fix only the next pinned-source semantic difference. Promote floor 12 only
after the 19-room Java layout matches, then verify pre-paint, pre-mobs, and
pre-items RNG boundaries.

## Current checkpoint

- Floor-12 room selection, builder choice/intensity, and post-`initRooms` RNG
  match. Figure-eight loop placement now uses Java's incremental collision
  order, including first-loop stitches before second-loop tunnels.
- The next builder difference is unlocated: Rust still has 22 rooms, Java 19.
  Painter and lifecycle parity are not claimed for floor 12.
- Floor 11 remains exact at its documented deterministic boundaries.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
